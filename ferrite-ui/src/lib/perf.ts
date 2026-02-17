/**
 * Full-stack performance tracker for Ferrite video playback.
 *
 * Records timing spans (start/end) and point events across the frontend
 * and backend. Backend timings are pulled from Server-Timing headers and
 * JSON `timing_ms` fields returned by the API.
 *
 * Toggle the overlay with the `P` key during playback.
 */

export interface PerfSpan {
  label: string;
  category: 'frontend' | 'backend' | 'network';
  startMs: number;
  endMs: number | null;
  durationMs: number | null;
  meta?: Record<string, string | number>;
}

export interface PerfEvent {
  label: string;
  category: 'frontend' | 'backend' | 'network';
  timestampMs: number;
  meta?: Record<string, string | number>;
}

export type PerfEntry = PerfSpan | PerfEvent;

function isSpan(e: PerfEntry): e is PerfSpan {
  return 'startMs' in e;
}

const MAX_ENTRIES = 200;

export class PerfTracker {
  private entries: PerfEntry[] = [];
  private openSpans: Map<string, PerfSpan> = new Map();
  private origin: number = performance.now();
  private listeners: Set<() => void> = new Set();

  /** Reset all data and set a new time origin. */
  reset(): void {
    this.entries = [];
    this.openSpans.clear();
    this.origin = performance.now();
    this.notify();
  }

  /** Subscribe to changes. Returns unsubscribe function. */
  subscribe(fn: () => void): () => void {
    this.listeners.add(fn);
    return () => this.listeners.delete(fn);
  }

  private notify(): void {
    for (const fn of this.listeners) fn();
  }

  private now(): number {
    return performance.now() - this.origin;
  }

  /** Start a named timing span. */
  startSpan(label: string, category: PerfSpan['category'] = 'frontend', meta?: Record<string, string | number>): void {
    const span: PerfSpan = {
      label,
      category,
      startMs: this.now(),
      endMs: null,
      durationMs: null,
      meta,
    };
    this.openSpans.set(label, span);
    this.push(span);
  }

  /** End a previously started span. */
  endSpan(label: string, meta?: Record<string, string | number>): number {
    const span = this.openSpans.get(label);
    if (!span) return 0;
    span.endMs = this.now();
    span.durationMs = span.endMs - span.startMs;
    if (meta) span.meta = { ...span.meta, ...meta };
    this.openSpans.delete(label);
    this.notify();
    return span.durationMs;
  }

  /** Record a point-in-time event. */
  event(label: string, category: PerfEvent['category'] = 'frontend', meta?: Record<string, string | number>): void {
    const evt: PerfEvent = {
      label,
      category,
      timestampMs: this.now(),
      meta,
    };
    this.push(evt);
    this.notify();
  }

  /** Ingest backend timing from a JSON `timing_ms` object (returned by API). */
  ingestBackendTiming(operation: string, timingMs: Record<string, number>): void {
    for (const [key, value] of Object.entries(timingMs)) {
      this.event(`${operation}/${key}`, 'backend', { ms: Math.round(value) });
    }
  }

  /** Get all entries (most recent first). */
  getEntries(): PerfEntry[] {
    return [...this.entries].reverse();
  }

  /** Get only completed spans, most recent first. */
  getSpans(): PerfSpan[] {
    return this.entries
      .filter((e): e is PerfSpan => isSpan(e) && e.durationMs !== null)
      .reverse();
  }

  /** Get a summary of the last N operations grouped by label. */
  getSummary(): { label: string; category: string; avgMs: number; count: number; lastMs: number }[] {
    const groups = new Map<string, { total: number; count: number; last: number; category: string }>();
    for (const entry of this.entries) {
      if (isSpan(entry) && entry.durationMs !== null) {
        const existing = groups.get(entry.label);
        if (existing) {
          existing.total += entry.durationMs;
          existing.count++;
          existing.last = Math.max(existing.last, entry.durationMs);
        } else {
          groups.set(entry.label, {
            total: entry.durationMs,
            count: 1,
            last: entry.durationMs,
            category: entry.category,
          });
        }
      }
    }
    return Array.from(groups.entries()).map(([label, g]) => ({
      label,
      category: g.category,
      avgMs: Math.round(g.total / g.count),
      count: g.count,
      lastMs: Math.round(g.last),
    }));
  }

  private push(entry: PerfEntry): void {
    this.entries.push(entry);
    if (this.entries.length > MAX_ENTRIES) {
      this.entries = this.entries.slice(-MAX_ENTRIES);
    }
  }
}

/** Singleton tracker instance shared across the app. */
export const perf = new PerfTracker();
