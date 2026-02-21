#!/usr/bin/env node
import { randomUUID } from 'node:crypto';
import { mkdir, writeFile } from 'node:fs/promises';
import { performance } from 'node:perf_hooks';
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
  console.log(`Auth hot-path load harness

Usage:
  node scripts/auth-hotpath-load.mjs \
    --base-url http://127.0.0.1:8080 \
    --token <ADMIN_JWT> \
    --media-id <MEDIA_ID> \
    --auth-mode bearer \
    --iterations 20 \
    --concurrency 16 \
    --out docs/benchmarks/auth-hotpath-<date>.json

Auth modes:
  bearer         Use Authorization header
  token-query    Use ?token=... query parameter
  api-key-header Use X-API-Key header (requires --api-key)
  api-key-query  Use ?api_key=... query parameter (requires --api-key)

Notes:
  - --token is always used for /api/system/metrics reset/read and cleanup calls.
  - Stream requests use credentials based on --auth-mode.
`);
}

function percentile(sorted, p) {
  if (sorted.length === 0) return 0;
  const idx = Math.max(0, Math.min(sorted.length - 1, Math.ceil(sorted.length * p) - 1));
  return sorted[idx];
}

function summarize(samples) {
  const sorted = [...samples].sort((a, b) => a - b);
  const total = sorted.reduce((acc, v) => acc + v, 0);
  return {
    count: sorted.length,
    min_ms: sorted[0] ?? 0,
    avg_ms: sorted.length ? total / sorted.length : 0,
    p50_ms: percentile(sorted, 0.5),
    p95_ms: percentile(sorted, 0.95),
    max_ms: sorted[sorted.length - 1] ?? 0,
    samples_ms: sorted,
  };
}

async function fetchChecked(url, init, readMode = 'text') {
  const started = performance.now();
  const res = await fetch(url, init);
  const elapsedMs = performance.now() - started;

  if (!res.ok) {
    const body = await res.text().catch(() => '');
    throw new Error(`HTTP ${res.status} ${res.statusText} for ${url}: ${body}`);
  }

  if (readMode === 'first-byte') {
    if (res.body) {
      const reader = res.body.getReader();
      await reader.read();
      await reader.cancel().catch(() => {});
    }
    return { elapsedMs, data: null };
  }

  if (readMode === 'json') {
    const data = await res.json();
    return { elapsedMs, data };
  }

  const data = await res.text();
  return { elapsedMs, data };
}

function firstMediaLine(playlist, extension) {
  const lines = playlist
    .split('\n')
    .map((line) => line.trim())
    .filter((line) => line.length > 0 && !line.startsWith('#'));

  if (!extension) return lines[0] ?? null;
  return lines.find((line) => line.includes(extension)) ?? null;
}

function extractMapUri(playlist) {
  const mapLine = playlist
    .split('\n')
    .map((line) => line.trim())
    .find((line) => line.startsWith('#EXT-X-MAP:'));
  if (!mapLine) return null;
  const match = mapLine.match(/URI="([^"]+)"/);
  return match ? match[1] : null;
}

function resolveUrl(baseUrl, relativeOrAbsolute) {
  return new URL(relativeOrAbsolute, baseUrl).toString();
}

function appendQueryParams(url, params) {
  const u = new URL(url);
  for (const [k, v] of Object.entries(params)) {
    if (v !== undefined && v !== null && v !== '') {
      u.searchParams.set(k, v);
    }
  }
  return u.toString();
}

function buildStreamAuth(mode, token, apiKey) {
  const headers = {};
  const query = {};

  switch (mode) {
    case 'bearer':
      headers.Authorization = `Bearer ${token}`;
      break;
    case 'token-query':
      query.token = token;
      break;
    case 'api-key-header':
      headers['X-API-Key'] = apiKey;
      break;
    case 'api-key-query':
      query.api_key = apiKey;
      break;
    default:
      throw new Error(`Unsupported --auth-mode '${mode}'`);
  }

  return { headers, query };
}

async function resetMetrics(baseUrl, controlHeaders) {
  await fetchChecked(
    `${baseUrl}/api/system/metrics`,
    { method: 'DELETE', headers: controlHeaders },
    'json',
  );
}

async function readMetrics(baseUrl, controlHeaders) {
  const { data } = await fetchChecked(
    `${baseUrl}/api/system/metrics`,
    { method: 'GET', headers: controlHeaders },
    'json',
  );
  return data;
}

async function stopHls(baseUrl, mediaId, playbackSessionId, controlHeaders) {
  await fetch(
    `${baseUrl}/api/stream/${mediaId}/hls?playback_session_id=${encodeURIComponent(playbackSessionId)}`,
    { method: 'DELETE', headers: controlHeaders },
  ).catch(() => {});
}

async function resolveAuthTarget(baseUrl, mediaId, playbackSessionId, streamAuth) {
  const masterBase = `${baseUrl}/api/stream/${mediaId}/hls/master.m3u8`;
  const masterUrl = appendQueryParams(masterBase, {
    playback_session_id: playbackSessionId,
    ...streamAuth.query,
  });

  const master = await fetchChecked(
    masterUrl,
    { method: 'GET', headers: streamAuth.headers },
    'text',
  );

  const variantRel = firstMediaLine(master.data, '.m3u8');
  if (!variantRel) {
    throw new Error('No variant playlist line found in HLS master playlist');
  }

  const variantUrl = appendQueryParams(resolveUrl(baseUrl, variantRel), streamAuth.query);
  const variant = await fetchChecked(
    variantUrl,
    { method: 'GET', headers: streamAuth.headers },
    'text',
  );

  const mapUri = extractMapUri(variant.data);
  if (mapUri) {
    return appendQueryParams(resolveUrl(baseUrl, mapUri), streamAuth.query);
  }

  const segRel = firstMediaLine(variant.data, '.m4s') ?? firstMediaLine(variant.data, '.ts');
  if (!segRel) {
    throw new Error('No map URI or media segment found in variant playlist');
  }

  return appendQueryParams(resolveUrl(baseUrl, segRel), streamAuth.query);
}

async function runLoad(url, streamAuth, iterations, concurrency) {
  const samples = [];
  for (let i = 0; i < iterations; i += 1) {
    const batch = await Promise.all(
      Array.from({ length: concurrency }, async () => {
        const result = await fetchChecked(
          url,
          { method: 'GET', headers: streamAuth.headers },
          'first-byte',
        );
        return result.elapsedMs;
      }),
    );
    samples.push(...batch);
    console.log(`[auth-hotpath] iteration ${i + 1}/${iterations} complete`);
  }
  return summarize(samples);
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  if (args.help === 'true' || args.h === 'true') {
    usage();
    process.exit(0);
  }

  const baseUrl = (args['base-url'] || '').replace(/\/$/, '');
  const token = args.token || '';
  const mediaId = args['media-id'] || '';
  const authMode = (args['auth-mode'] || 'bearer').toLowerCase();
  const apiKey = args['api-key'] || '';
  const iterations = Number.parseInt(args.iterations ?? '20', 10);
  const concurrency = Number.parseInt(args.concurrency ?? '16', 10);

  if (!baseUrl || !token || !mediaId) {
    usage();
    throw new Error('--base-url, --token, and --media-id are required');
  }
  if ((authMode === 'api-key-header' || authMode === 'api-key-query') && !apiKey) {
    throw new Error(`--api-key is required for auth mode '${authMode}'`);
  }
  if (!Number.isFinite(iterations) || iterations < 1) {
    throw new Error('--iterations must be a positive integer');
  }
  if (!Number.isFinite(concurrency) || concurrency < 1) {
    throw new Error('--concurrency must be a positive integer');
  }

  const playbackSessionId = args['playback-session-id'] || `auth-hotpath-${randomUUID()}`;
  const controlHeaders = { Authorization: `Bearer ${token}` };
  const streamAuth = buildStreamAuth(authMode, token, apiKey);

  const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
  const defaultOut = `docs/benchmarks/auth-hotpath-${authMode}-${timestamp}.json`;
  const outPath = args.out || defaultOut;

  await resetMetrics(baseUrl, controlHeaders);

  try {
    const targetUrl = await resolveAuthTarget(baseUrl, mediaId, playbackSessionId, streamAuth);
    const summary = await runLoad(targetUrl, streamAuth, iterations, concurrency);
    const backendMetrics = await readMetrics(baseUrl, controlHeaders);

    const authTimingSeries = (backendMetrics.timings ?? []).filter(
      (row) => typeof row?.name === 'string' && row.name.startsWith('auth_hotpath_ms'),
    );

    const output = {
      generated_at: new Date().toISOString(),
      base_url: baseUrl,
      media_id: mediaId,
      auth_mode: authMode,
      benchmark: {
        iterations,
        concurrency,
        playback_session_id: playbackSessionId,
        target_url: targetUrl,
      },
      auth_hotpath: summary,
      backend_metrics: {
        auth_hotpath_timings: authTimingSeries,
        raw: backendMetrics,
      },
    };

    await mkdir(path.dirname(outPath), { recursive: true });
    await writeFile(outPath, `${JSON.stringify(output, null, 2)}\n`, 'utf8');

    console.log('\nAuth hot-path load complete.');
    console.log(`Output written to: ${outPath}`);
  } finally {
    await stopHls(baseUrl, mediaId, playbackSessionId, controlHeaders);
  }
}

main().catch((err) => {
  console.error(`Auth hot-path load failed: ${err.message}`);
  process.exit(1);
});
