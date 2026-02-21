#!/usr/bin/env node
import { readFile } from 'node:fs/promises';

function parseArgs(argv) {
  const args = {};
  for (let i = 0; i < argv.length; i += 1) {
    const key = argv[i];
    if (!key.startsWith('--')) continue;
    const name = key.slice(2);
    const value = argv[i + 1];
    if (!value || value.startsWith('--')) {
      args[name] = 'true';
      continue;
    }
    args[name] = value;
    i += 1;
  }
  return args;
}

function usage() {
  console.log(`Playback benchmark threshold gate

Usage:
  node scripts/check-playback-thresholds.mjs \
    --benchmark docs/benchmarks/playback-baseline-2026-02-20.json \
    --thresholds docs/benchmarks/perf-thresholds.json

Constraint keys supported:
  any numeric field ending in _lte or _gte
  examples: p95_ms_lte, count_gte, switch_rate_gte, value_lte
`);
}

function assertFiniteNumber(value, label) {
  if (typeof value !== 'number' || Number.isNaN(value) || !Number.isFinite(value)) {
    throw new Error(`${label} must be a finite number`);
  }
}

function evaluateConstraint(actual, key, expected, label) {
  assertFiniteNumber(actual, `${label}.${key}`);
  assertFiniteNumber(expected, `${label}.expected.${key}`);

  if (key.endsWith('_lte')) {
    return {
      ok: actual <= expected,
      detail: `${label}.${key.replace(/_lte$/, '')}=${actual} <= ${expected}`,
    };
  }
  if (key.endsWith('_gte')) {
    return {
      ok: actual >= expected,
      detail: `${label}.${key.replace(/_gte$/, '')}=${actual} >= ${expected}`,
    };
  }
  throw new Error(`Unsupported constraint '${key}' for ${label}`);
}

function evaluateObjectThresholds(actualObject, thresholds, label, violations) {
  if (!actualObject || typeof actualObject !== 'object') {
    violations.push(`${label} missing`);
    return;
  }

  for (const [constraint, expected] of Object.entries(thresholds)) {
    const field = constraint.replace(/_(lte|gte)$/, '');
    const actual = actualObject[field];
    if (actual === undefined) {
      violations.push(`${label}.${field} missing`);
      continue;
    }

    const result = evaluateConstraint(actual, constraint, expected, label);
    if (!result.ok) {
      violations.push(result.detail);
    }
  }
}

function timingMap(timings) {
  const map = new Map();
  for (const row of timings ?? []) {
    if (row?.name) map.set(row.name, row);
  }
  return map;
}

function counterMap(counters) {
  const map = new Map();
  for (const row of counters ?? []) {
    if (row?.name) map.set(row.name, row);
  }
  return map;
}

async function readJson(path) {
  const raw = await readFile(path, 'utf8');
  return JSON.parse(raw);
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  if (args.help === 'true' || args.h === 'true') {
    usage();
    process.exit(0);
  }

  const benchmarkPath = args.benchmark;
  const thresholdPath = args.thresholds;

  if (!benchmarkPath || !thresholdPath) {
    usage();
    throw new Error('--benchmark and --thresholds are required');
  }

  const benchmark = await readJson(benchmarkPath);
  const thresholdSpec = await readJson(thresholdPath);
  const violations = [];

  for (const [scenario, constraints] of Object.entries(thresholdSpec.scenarios ?? {})) {
    const sample = benchmark.scenarios?.[scenario];
    evaluateObjectThresholds(sample, constraints, `scenario.${scenario}`, violations);
  }

  const timings = timingMap(benchmark.backend_metrics?.timings);
  for (const [metricName, constraints] of Object.entries(thresholdSpec.backend_metrics?.timings ?? {})) {
    evaluateObjectThresholds(
      timings.get(metricName),
      constraints,
      `backend.timing.${metricName}`,
      violations,
    );
  }

  const counters = counterMap(benchmark.backend_metrics?.counters);
  for (const [metricName, constraints] of Object.entries(thresholdSpec.backend_metrics?.counters ?? {})) {
    evaluateObjectThresholds(
      counters.get(metricName),
      constraints,
      `backend.counter.${metricName}`,
      violations,
    );
  }

  if (violations.length > 0) {
    console.error('Playback perf threshold gate failed:');
    for (const v of violations) {
      console.error(`  - ${v}`);
    }
    process.exit(1);
  }

  console.log(`Playback perf threshold gate passed (${benchmarkPath})`);
}

main().catch((err) => {
  console.error(`Threshold check failed: ${err.message}`);
  process.exit(1);
});
