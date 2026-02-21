#!/usr/bin/env node
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

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
  console.log(`Playback-critical integration suite (WS7)

Usage:
  node tests/integration/run-playback-critical.mjs \
    --cases same-media-isolation,abr-switch-under-throttle \
    --include-live-auth false

Options:
  --cases <csv>             Comma-separated case IDs. Defaults to all static cases.
  --include-live-auth true  Also run live auth hot-path scenarios.

Live auth options (required when --include-live-auth true):
  --base-url http://127.0.0.1:8080
  --token <ADMIN_JWT>
  --media-id <MEDIA_ID>

Optional live auth options:
  --api-key <API_KEY>       Enables api-key-header and api-key-query runs.
  --iterations 20
  --concurrency 16
  --auth-out-dir docs/benchmarks
  --auth-thresholds docs/benchmarks/auth-hotpath-thresholds.json
`);
}

function toBool(value) {
  if (typeof value !== 'string') return false;
  const normalized = value.trim().toLowerCase();
  return normalized === 'true' || normalized === '1' || normalized === 'yes';
}

function parsePositiveInt(value, label) {
  const parsed = Number.parseInt(value, 10);
  if (!Number.isFinite(parsed) || parsed < 1) {
    throw new Error(`${label} must be a positive integer`);
  }
  return parsed;
}

function nowStamp() {
  return new Date().toISOString().replace(/[:.]/g, '-');
}

function runCommand(label, command, args, cwd) {
  return new Promise((resolve, reject) => {
    console.log(`\n==> ${label}`);
    console.log(`    ${command} ${args.join(' ')}`);

    const child = spawn(command, args, {
      cwd,
      stdio: 'inherit',
      shell: false,
      env: process.env,
    });

    child.on('error', (err) => {
      reject(new Error(`${label} failed to start: ${err.message}`));
    });

    child.on('close', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${label} failed with exit code ${code}`));
    });
  });
}

const STATIC_CASES = [
  {
    id: 'same-media-isolation',
    description: 'Concurrent same-media playback isolation',
    command: 'cargo',
    args: [
      'test',
      '-p',
      'ferrite-stream',
      '--test',
      'hls_session_integration',
      'concurrent_media_sessions_are_isolated',
      '--',
      '--exact',
    ],
  },
  {
    id: 'abr-switch-under-throttle',
    description: 'ABR variant downgrade under constrained throughput',
    command: 'cargo',
    args: [
      'test',
      '-p',
      'ferrite-stream',
      '--test',
      'hls_session_integration',
      'abr_master_playlist_supports_variant_downgrade_under_throttle',
      '--',
      '--exact',
    ],
  },
  {
    id: 'seek-reuse-recreate',
    description: 'Seek within-buffer reuse and far-seek recreate',
    command: 'cargo',
    args: [
      'test',
      '-p',
      'ferrite-stream',
      '--test',
      'hls_session_integration',
      'get_or_create_session_reuses_nearby_start_and_recreates_far_start',
      '--',
      '--exact',
    ],
  },
  {
    id: 'api-contract-session-capability',
    description: 'Stream API contract tests for session/capability behavior',
    command: 'cargo',
    args: [
      'test',
      '-p',
      'ferrite-api',
      'handlers::stream::tests',
    ],
  },
  {
    id: 'progress-isolation-by-user',
    description: 'Playback progress remains isolated by user',
    command: 'cargo',
    args: [
      'test',
      '-p',
      'ferrite-db',
      '--test',
      'playback_progress_user_isolation',
    ],
  },
];

function selectedStaticCases(casesCsv) {
  if (!casesCsv) {
    return STATIC_CASES;
  }

  const requested = casesCsv
    .split(',')
    .map((value) => value.trim())
    .filter((value) => value.length > 0);

  if (requested.length === 0) {
    throw new Error('--cases was provided but no valid case IDs were parsed');
  }

  const byId = new Map(STATIC_CASES.map((testCase) => [testCase.id, testCase]));
  const selected = [];
  for (const id of requested) {
    const testCase = byId.get(id);
    if (!testCase) {
      throw new Error(`Unknown case '${id}'. Known cases: ${STATIC_CASES.map((c) => c.id).join(', ')}`);
    }
    selected.push(testCase);
  }
  return selected;
}

function authModes(apiKey) {
  if (apiKey) {
    return ['bearer', 'token-query', 'api-key-header', 'api-key-query'];
  }
  return ['bearer', 'token-query'];
}

async function runStaticSuite(repoRoot, casesCsv) {
  const cases = selectedStaticCases(casesCsv);
  for (const testCase of cases) {
    await runCommand(`[${testCase.id}] ${testCase.description}`, testCase.command, testCase.args, repoRoot);
  }
}

async function runLiveAuthSuite(repoRoot, args) {
  const baseUrl = (args['base-url'] || '').replace(/\/$/, '');
  const token = args.token || '';
  const mediaId = args['media-id'] || '';
  const apiKey = args['api-key'] || '';

  if (!baseUrl || !token || !mediaId) {
    throw new Error('Live auth suite requires --base-url, --token, and --media-id');
  }

  const iterations = parsePositiveInt(args.iterations ?? '20', '--iterations');
  const concurrency = parsePositiveInt(args.concurrency ?? '16', '--concurrency');
  const authOutDir = args['auth-out-dir'] || path.join('docs', 'benchmarks');
  const thresholdsPath = args['auth-thresholds'] || path.join('docs', 'benchmarks', 'auth-hotpath-thresholds.json');
  const stamp = nowStamp();

  for (const mode of authModes(apiKey)) {
    const outPath = path.join(authOutDir, `auth-hotpath-${mode}-${stamp}.json`);

    const loadArgs = [
      'scripts/auth-hotpath-load.mjs',
      '--base-url', baseUrl,
      '--token', token,
      '--media-id', mediaId,
      '--auth-mode', mode,
      '--iterations', String(iterations),
      '--concurrency', String(concurrency),
      '--out', outPath,
    ];

    if (mode.startsWith('api-key-')) {
      if (!apiKey) {
        throw new Error(`--api-key is required for auth mode '${mode}'`);
      }
      loadArgs.push('--api-key', apiKey);
    }

    await runCommand(`[auth:${mode}] load`, process.execPath, loadArgs, repoRoot);
    await runCommand(
      `[auth:${mode}] threshold-check`,
      process.execPath,
      [
        'scripts/check-auth-hotpath-thresholds.mjs',
        '--benchmark',
        outPath,
        '--thresholds',
        thresholdsPath,
      ],
      repoRoot,
    );
  }
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  if (args.help === 'true' || args.h === 'true') {
    usage();
    process.exit(0);
  }

  const thisDir = path.dirname(fileURLToPath(import.meta.url));
  const repoRoot = path.resolve(thisDir, '..', '..');

  await runStaticSuite(repoRoot, args.cases);

  if (toBool(args['include-live-auth'])) {
    await runLiveAuthSuite(repoRoot, args);
  }

  console.log('\nPlayback-critical integration suite completed successfully.');
}

main().catch((err) => {
  console.error(`Integration suite failed: ${err.message}`);
  process.exit(1);
});
