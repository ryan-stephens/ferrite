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
  console.log(`Ferrite playback benchmark harness

Usage:
  node scripts/playback-benchmark.mjs \\
    --base-url http://127.0.0.1:8080 \\
    --token <jwt-token> \\
    --direct-id <media-id> \\
    --remux-id <media-id> \\
    --audio-id <media-id> \\
    --full-id <media-id> \\
    --hls-id <media-id> \\
    --iterations 3 \\
    --concurrency 1 \\
    --abr-throttle-kbps 300 \\
    --seek-secs 600 \\
    --out docs/benchmarks/playback-baseline.json

Notes:
  - Token must have access to /api/system/metrics endpoints (admin recommended).
  - Provide media IDs per mode for clean baselines.
  - HLS scenarios auto-generate a unique playback_session_id per run.
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

function benchmarkPlaybackSessionId() {
  return `bench-${randomUUID()}`;
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function parseMasterVariants(masterPlaylist, baseUrl) {
  const lines = masterPlaylist
    .split('\n')
    .map((line) => line.trim())
    .filter((line) => line.length > 0);

  const variants = [];
  for (let i = 0; i < lines.length; i += 1) {
    const line = lines[i];
    if (!line.startsWith('#EXT-X-STREAM-INF:')) continue;

    const attrs = line.replace('#EXT-X-STREAM-INF:', '');
    const bandwidthBps = attrs
      .split(',')
      .map((part) => part.trim())
      .find((part) => part.startsWith('BANDWIDTH='))
      ?.replace('BANDWIDTH=', '');
    const parsedBandwidthBps = Number.parseInt(bandwidthBps ?? '', 10);
    if (!Number.isFinite(parsedBandwidthBps)) continue;

    let uri = null;
    for (let j = i + 1; j < lines.length; j += 1) {
      const candidate = lines[j];
      if (candidate.startsWith('#')) continue;
      uri = candidate;
      i = j;
      break;
    }

    if (!uri) continue;
    variants.push({
      bandwidth_bps: parsedBandwidthBps,
      uri,
      url: resolveUrl(baseUrl, uri),
    });
  }

  return variants.sort((a, b) => a.bandwidth_bps - b.bandwidth_bps);
}

function pickVariantForThroughput(variants, throughputBps) {
  const fitting = variants
    .filter((variant) => variant.bandwidth_bps <= throughputBps)
    .sort((a, b) => b.bandwidth_bps - a.bandwidth_bps);

  if (fitting.length > 0) {
    return fitting[0];
  }

  return variants[0] ?? null;
}

async function readBodyWithThrottle(res, throttleBytesPerSec) {
  const started = performance.now();

  if (!res.body) {
    const data = await res.arrayBuffer();
    return {
      bytes: data.byteLength,
      duration_ms: Math.max(performance.now() - started, 1),
    };
  }

  const reader = res.body.getReader();
  let bytes = 0;

  while (true) {
    const { done, value } = await reader.read();
    if (done) break;

    bytes += value.byteLength;

    if (throttleBytesPerSec > 0) {
      const targetElapsedMs = (bytes / throttleBytesPerSec) * 1000;
      const elapsedMs = performance.now() - started;
      if (targetElapsedMs > elapsedMs) {
        await sleep(targetElapsedMs - elapsedMs);
      }
    }
  }

  return {
    bytes,
    duration_ms: Math.max(performance.now() - started, 1),
  };
}

async function measureSegmentThroughput(url, headers, throttleBitsPerSec) {
  const res = await fetch(url, {
    method: 'GET',
    headers,
  });

  if (!res.ok) {
    const body = await res.text().catch(() => '');
    throw new Error(`HTTP ${res.status} ${res.statusText} for ${url}: ${body}`);
  }

  const throttleBytesPerSec = throttleBitsPerSec > 0 ? throttleBitsPerSec / 8 : 0;
  const { bytes, duration_ms: durationMs } = await readBodyWithThrottle(res, throttleBytesPerSec);
  if (!bytes) {
    return 0;
  }

  const seconds = Math.max(durationMs / 1000, 0.001);
  return (bytes * 8) / seconds;
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
    return { elapsedMs, res, data: null };
  }

  if (readMode === 'json') {
    const data = await res.json();
    return { elapsedMs, res, data };
  }

  const data = await res.text();
  return { elapsedMs, res, data };
}

async function runScenario(name, runOne, iterations, concurrency) {
  const samples = [];
  for (let i = 0; i < iterations; i += 1) {
    const batch = await Promise.all(
      Array.from({ length: concurrency }, async () => runOne()),
    );
    samples.push(...batch);
    console.log(`[${name}] iteration ${i + 1}/${iterations} complete`);
  }
  return summarize(samples);
}

async function runSwitchScenario(name, runOne, iterations, concurrency) {
  const decisionSamples = [];
  const switchFlags = [];
  let minSelectedBandwidthBps = Number.POSITIVE_INFINITY;
  let maxSelectedBandwidthBps = 0;

  for (let i = 0; i < iterations; i += 1) {
    const batch = await Promise.all(
      Array.from({ length: concurrency }, async () => runOne()),
    );

    for (const result of batch) {
      decisionSamples.push(result.decision_ms);
      switchFlags.push(result.switched ? 1 : 0);
      minSelectedBandwidthBps = Math.min(minSelectedBandwidthBps, result.selected_bandwidth_bps);
      maxSelectedBandwidthBps = Math.max(maxSelectedBandwidthBps, result.selected_bandwidth_bps);
    }

    console.log(`[${name}] iteration ${i + 1}/${iterations} complete`);
  }

  const summary = summarize(decisionSamples);
  const switchCount = switchFlags.reduce((sum, flag) => sum + flag, 0);
  return {
    ...summary,
    switch_count: switchCount,
    switch_rate: switchFlags.length > 0 ? switchCount / switchFlags.length : 0,
    min_selected_bandwidth_bps: Number.isFinite(minSelectedBandwidthBps)
      ? minSelectedBandwidthBps
      : 0,
    max_selected_bandwidth_bps: maxSelectedBandwidthBps,
  };
}

async function resetMetrics(ctx) {
  await fetchChecked(
    `${ctx.baseUrl}/api/system/metrics`,
    {
      method: 'DELETE',
      headers: ctx.authHeaders,
    },
    'json',
  );
}

async function readMetrics(ctx) {
  const result = await fetchChecked(
    `${ctx.baseUrl}/api/system/metrics`,
    {
      method: 'GET',
      headers: ctx.authHeaders,
    },
    'json',
  );
  return result.data;
}

async function stopHls(ctx, mediaId, playbackSessionId) {
  const playbackSuffix = playbackSessionId
    ? `?playback_session_id=${encodeURIComponent(playbackSessionId)}`
    : '';

  await fetch(`${ctx.baseUrl}/api/stream/${mediaId}/hls${playbackSuffix}`, {
    method: 'DELETE',
    headers: ctx.authHeaders,
  }).catch(() => {});
}

async function resolveVariantProbeTarget(ctx, variantUrl) {
  const variant = await fetchChecked(
    variantUrl,
    {
      method: 'GET',
      headers: ctx.authHeaders,
    },
    'text',
  );

  const firstSeg = firstMediaLine(variant.data, '.m4s') ?? firstMediaLine(variant.data, '.ts');
  if (firstSeg) {
    return resolveUrl(ctx.baseUrl, firstSeg);
  }

  const mapUri = extractMapUri(variant.data);
  if (mapUri) {
    return resolveUrl(ctx.baseUrl, mapUri);
  }

  throw new Error(`No media segment found in playlist ${variantUrl}`);
}

async function loadHlsStartupPath(ctx, masterUrl) {
  const started = performance.now();

  const master = await fetchChecked(masterUrl, {
    method: 'GET',
    headers: ctx.authHeaders,
  });

  let variantUrl = firstMediaLine(master.data, '.m3u8');
  let variantBody = master.data;
  if (variantUrl) {
    variantUrl = resolveUrl(ctx.baseUrl, variantUrl);
    const variant = await fetchChecked(variantUrl, {
      method: 'GET',
      headers: ctx.authHeaders,
    });
    variantBody = variant.data;
  }

  const mapUri = extractMapUri(variantBody);
  if (mapUri) {
    await fetchChecked(resolveUrl(ctx.baseUrl, mapUri), {
      method: 'GET',
      headers: ctx.authHeaders,
    }, 'first-byte');
  }

  const firstSeg = firstMediaLine(variantBody, '.m4s') ?? firstMediaLine(variantBody, '.ts');
  if (!firstSeg) {
    throw new Error(`No media segment found in playlist ${variantUrl ?? masterUrl}`);
  }

  await fetchChecked(resolveUrl(ctx.baseUrl, firstSeg), {
    method: 'GET',
    headers: ctx.authHeaders,
  }, 'first-byte');

  return performance.now() - started;
}

async function benchmarkProgressive(ctx, mediaId) {
  const result = await fetchChecked(
    `${ctx.baseUrl}/api/stream/${mediaId}`,
    {
      method: 'GET',
      headers: ctx.authHeaders,
    },
    'first-byte',
  );
  return result.elapsedMs;
}

async function benchmarkHlsStartup(ctx, mediaId) {
  const playbackSessionId = benchmarkPlaybackSessionId();
  const playbackParam = `playback_session_id=${encodeURIComponent(playbackSessionId)}`;

  try {
    return await loadHlsStartupPath(
      ctx,
      `${ctx.baseUrl}/api/stream/${mediaId}/hls/master.m3u8?${playbackParam}`,
    );
  } finally {
    await stopHls(ctx, mediaId, playbackSessionId);
  }
}

async function benchmarkHlsAbrSwitch(ctx, mediaId, abrThrottleKbps) {
  const playbackSessionId = benchmarkPlaybackSessionId();
  const playbackParam = `playback_session_id=${encodeURIComponent(playbackSessionId)}`;

  try {
    const decisionStarted = performance.now();

    const master = await fetchChecked(
      `${ctx.baseUrl}/api/stream/${mediaId}/hls/master.m3u8?${playbackParam}`,
      {
        method: 'GET',
        headers: ctx.authHeaders,
      },
      'text',
    );

    const variants = parseMasterVariants(master.data, ctx.baseUrl);
    if (variants.length < 2) {
      throw new Error('ABR switch scenario requires at least 2 variants in master playlist');
    }

    const highest = variants[variants.length - 1];
    const probeTarget = await resolveVariantProbeTarget(ctx, highest.url);
    const observedThroughputBps = await measureSegmentThroughput(
      probeTarget,
      ctx.authHeaders,
      abrThrottleKbps * 1000,
    );

    // Reserve some headroom so selected level fits under induced link capacity.
    const selected = pickVariantForThroughput(variants, observedThroughputBps * 0.85);
    if (!selected) {
      throw new Error('ABR switch scenario could not select a target variant');
    }

    const selectedProbeTarget = await resolveVariantProbeTarget(ctx, selected.url);
    await fetchChecked(
      selectedProbeTarget,
      {
        method: 'GET',
        headers: ctx.authHeaders,
      },
      'first-byte',
    );

    return {
      decision_ms: performance.now() - decisionStarted,
      switched: selected.bandwidth_bps < highest.bandwidth_bps,
      selected_bandwidth_bps: selected.bandwidth_bps,
    };
  } finally {
    await stopHls(ctx, mediaId, playbackSessionId);
  }
}

async function benchmarkHlsSeek(ctx, mediaId, seekSecs) {
  const playbackSessionId = benchmarkPlaybackSessionId();
  const playbackParam = `playback_session_id=${encodeURIComponent(playbackSessionId)}`;

  try {
    await fetchChecked(
      `${ctx.baseUrl}/api/stream/${mediaId}/hls/master.m3u8?${playbackParam}`,
      {
        method: 'GET',
        headers: ctx.authHeaders,
      },
      'text',
    );

    const seekStarted = performance.now();
    const seek = await fetchChecked(
      `${ctx.baseUrl}/api/stream/${mediaId}/hls/seek?start=${seekSecs.toFixed(3)}&${playbackParam}`,
      {
        method: 'POST',
        headers: ctx.authHeaders,
      },
      'json',
    );

    if (!seek.data?.master_url) {
      throw new Error('HLS seek response missing master_url');
    }

    await loadHlsStartupPath(ctx, resolveUrl(ctx.baseUrl, seek.data.master_url));
    return performance.now() - seekStarted;
  } finally {
    await stopHls(ctx, mediaId, playbackSessionId);
  }
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  if (args.help === 'true' || args.h === 'true') {
    usage();
    process.exit(0);
  }

  const baseUrl = (args['base-url'] || '').replace(/\/$/, '');
  const token = args.token || '';

  if (!baseUrl || !token) {
    usage();
    throw new Error('--base-url and --token are required');
  }

  const iterations = Number.parseInt(args.iterations ?? '3', 10);
  const concurrency = Number.parseInt(args.concurrency ?? '1', 10);
  const abrThrottleKbps = Number.parseFloat(args['abr-throttle-kbps'] ?? '300');
  const seekSecs = Number.parseFloat(args['seek-secs'] ?? '600');

  if (!Number.isFinite(iterations) || iterations < 1) {
    throw new Error('--iterations must be a positive integer');
  }
  if (!Number.isFinite(concurrency) || concurrency < 1) {
    throw new Error('--concurrency must be a positive integer');
  }
  if (!Number.isFinite(abrThrottleKbps) || abrThrottleKbps <= 0) {
    throw new Error('--abr-throttle-kbps must be a positive number');
  }

  const ids = {
    direct: args['direct-id'],
    remux: args['remux-id'],
    audio: args['audio-id'],
    full: args['full-id'],
    hls: args['hls-id'],
  };

  const hasAny = Object.values(ids).some(Boolean);
  if (!hasAny) {
    throw new Error('Provide at least one media id (--direct-id/--remux-id/--audio-id/--full-id/--hls-id)');
  }

  const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
  const defaultOut = `docs/benchmarks/playback-baseline-${timestamp}.json`;
  const outPath = args.out || defaultOut;

  const ctx = {
    baseUrl,
    authHeaders: {
      Authorization: `Bearer ${token}`,
    },
  };

  await resetMetrics(ctx);

  const scenarios = {};
  if (ids.direct) {
    scenarios.direct = await runScenario(
      'direct',
      () => benchmarkProgressive(ctx, ids.direct),
      iterations,
      concurrency,
    );
  }
  if (ids.remux) {
    scenarios.remux = await runScenario(
      'remux',
      () => benchmarkProgressive(ctx, ids.remux),
      iterations,
      concurrency,
    );
  }
  if (ids.audio) {
    scenarios.audio_transcode = await runScenario(
      'audio_transcode',
      () => benchmarkProgressive(ctx, ids.audio),
      iterations,
      concurrency,
    );
  }
  if (ids.full) {
    scenarios.full_transcode = await runScenario(
      'full_transcode',
      () => benchmarkProgressive(ctx, ids.full),
      iterations,
      concurrency,
    );
  }
  if (ids.hls) {
    scenarios.hls_startup = await runScenario(
      'hls_startup',
      () => benchmarkHlsStartup(ctx, ids.hls),
      iterations,
      concurrency,
    );
    scenarios.hls_seek = await runScenario(
      'hls_seek',
      () => benchmarkHlsSeek(ctx, ids.hls, seekSecs),
      iterations,
      concurrency,
    );
    scenarios.hls_abr_switch = await runSwitchScenario(
      'hls_abr_switch',
      () => benchmarkHlsAbrSwitch(ctx, ids.hls, abrThrottleKbps),
      iterations,
      concurrency,
    );
  }

  const backendMetrics = await readMetrics(ctx);

  const output = {
    generated_at: new Date().toISOString(),
    base_url: baseUrl,
    benchmark: {
      iterations,
      concurrency,
      abr_throttle_kbps: abrThrottleKbps,
      seek_secs: seekSecs,
      media_ids: ids,
    },
    scenarios,
    backend_metrics: backendMetrics,
  };

  await mkdir(path.dirname(outPath), { recursive: true });
  await writeFile(outPath, `${JSON.stringify(output, null, 2)}\n`, 'utf8');

  console.log('\nBenchmark complete.');
  console.log(`Output written to: ${outPath}`);
}

main().catch((err) => {
  console.error(`Benchmark failed: ${err.message}`);
  process.exit(1);
});
