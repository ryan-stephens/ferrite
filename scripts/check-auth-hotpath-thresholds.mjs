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
  console.log(`Auth hot-path benchmark threshold gate

Usage:
  node scripts/check-auth-hotpath-thresholds.mjs \
    --benchmark docs/benchmarks/auth-hotpath-baseline-2026-02-20.json \
    --thresholds docs/benchmarks/auth-hotpath-thresholds.json

Constraint keys supported:
  *_ms_lte, *_ms_gte, count_lte, count_gte, value_lte, value_gte
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

  for (const [constraint, expected] of Object.entries(thresholds ?? {})) {
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

function findTimingByFragments(timings, fragments) {
  return (timings ?? []).find((row) => {
    const name = row?.name;
    return typeof name === 'string' && fragments.every((fragment) => name.includes(fragment));
  });
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

  evaluateObjectThresholds(
    benchmark.auth_hotpath,
    thresholdSpec.auth_hotpath ?? {},
    'auth_hotpath',
    violations,
  );

  const backendTimings = Array.isArray(benchmark.backend_metrics?.auth_hotpath_timings)
    ? benchmark.backend_metrics.auth_hotpath_timings
    : benchmark.backend_metrics?.timings;

  for (const requirement of thresholdSpec.backend_metrics?.required_timings ?? []) {
    const label = requirement.label ?? 'unnamed_timing_requirement';
    const fragments = Array.isArray(requirement.name_contains)
      ? requirement.name_contains.filter((v) => typeof v === 'string' && v.length > 0)
      : [];

    if (fragments.length === 0) {
      violations.push(`backend_timing.${label} missing name_contains fragments in threshold spec`);
      continue;
    }

    const row = findTimingByFragments(backendTimings, fragments);
    if (!row) {
      violations.push(`backend_timing.${label} missing (name contains: ${fragments.join(', ')})`);
      continue;
    }

    evaluateObjectThresholds(
      row,
      requirement.constraints ?? {},
      `backend_timing.${label}`,
      violations,
    );
  }

  if (violations.length > 0) {
    console.error('Auth hot-path threshold gate failed:');
    for (const v of violations) {
      console.error(`  - ${v}`);
    }
    process.exit(1);
  }

  console.log(`Auth hot-path threshold gate passed (${benchmarkPath})`);
}

main().catch((err) => {
  console.error(`Threshold check failed: ${err.message}`);
  process.exit(1);
});
