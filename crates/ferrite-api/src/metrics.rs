use dashmap::DashMap;
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug)]
struct TimingSeries {
    count: AtomicU64,
    total_micros: AtomicU64,
    min_micros: AtomicU64,
    max_micros: AtomicU64,
    last_micros: AtomicU64,
}

impl Default for TimingSeries {
    fn default() -> Self {
        Self {
            count: AtomicU64::new(0),
            total_micros: AtomicU64::new(0),
            min_micros: AtomicU64::new(u64::MAX),
            max_micros: AtomicU64::new(0),
            last_micros: AtomicU64::new(0),
        }
    }
}

impl TimingSeries {
    fn record_ms(&self, ms: f64) {
        if !ms.is_finite() || ms < 0.0 {
            return;
        }

        let micros = (ms * 1000.0).round() as u64;
        self.count.fetch_add(1, Ordering::Relaxed);
        self.total_micros.fetch_add(micros, Ordering::Relaxed);
        self.last_micros.store(micros, Ordering::Relaxed);
        atomic_min(&self.min_micros, micros);
        atomic_max(&self.max_micros, micros);
    }

    fn snapshot(&self, name: String) -> TimingSnapshot {
        let count = self.count.load(Ordering::Relaxed);
        let total = self.total_micros.load(Ordering::Relaxed);
        let min = self.min_micros.load(Ordering::Relaxed);
        let max = self.max_micros.load(Ordering::Relaxed);
        let last = self.last_micros.load(Ordering::Relaxed);

        let avg_ms = if count == 0 {
            0.0
        } else {
            micros_to_ms(total) / count as f64
        };

        TimingSnapshot {
            name,
            count,
            avg_ms,
            min_ms: if count == 0 { 0.0 } else { micros_to_ms(min) },
            max_ms: if count == 0 { 0.0 } else { micros_to_ms(max) },
            last_ms: micros_to_ms(last),
        }
    }
}

#[derive(Debug, Default)]
pub struct PlaybackMetrics {
    timings: DashMap<String, Arc<TimingSeries>>,
    counters: DashMap<String, Arc<AtomicU64>>,
}

#[derive(Debug, Serialize)]
pub struct TimingSnapshot {
    pub name: String,
    pub count: u64,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub last_ms: f64,
}

#[derive(Debug, Serialize)]
pub struct CounterSnapshot {
    pub name: String,
    pub value: u64,
}

#[derive(Debug, Serialize)]
pub struct PlaybackMetricsSnapshot {
    pub timings: Vec<TimingSnapshot>,
    pub counters: Vec<CounterSnapshot>,
}

impl PlaybackMetrics {
    pub fn record_timing(&self, metric: &str, labels: &[(&str, &str)], ms: f64) {
        let key = metric_key(metric, labels);
        let series = self
            .timings
            .entry(key)
            .or_insert_with(|| Arc::new(TimingSeries::default()))
            .clone();
        series.record_ms(ms);
    }

    pub fn increment_counter(&self, metric: &str, labels: &[(&str, &str)], value: u64) {
        let key = metric_key(metric, labels);
        let counter = self
            .counters
            .entry(key)
            .or_insert_with(|| Arc::new(AtomicU64::new(0)))
            .clone();
        counter.fetch_add(value, Ordering::Relaxed);
    }

    pub fn reset(&self) {
        self.timings.clear();
        self.counters.clear();
    }

    pub fn snapshot(&self) -> PlaybackMetricsSnapshot {
        let mut timings: Vec<TimingSnapshot> = self
            .timings
            .iter()
            .map(|entry| entry.value().snapshot(entry.key().clone()))
            .collect();
        timings.sort_by(|a, b| a.name.cmp(&b.name));

        let mut counters: Vec<CounterSnapshot> = self
            .counters
            .iter()
            .map(|entry| CounterSnapshot {
                name: entry.key().clone(),
                value: entry.value().load(Ordering::Relaxed),
            })
            .collect();
        counters.sort_by(|a, b| a.name.cmp(&b.name));

        PlaybackMetricsSnapshot { timings, counters }
    }
}

fn metric_key(metric: &str, labels: &[(&str, &str)]) -> String {
    if labels.is_empty() {
        return metric.to_string();
    }

    let labels = labels
        .iter()
        .map(|(k, v)| format!("{}={}", k, sanitize_label_value(v)))
        .collect::<Vec<_>>()
        .join(",");
    format!("{}{{{}}}", metric, labels)
}

fn sanitize_label_value(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn micros_to_ms(micros: u64) -> f64 {
    micros as f64 / 1000.0
}

fn atomic_min(target: &AtomicU64, value: u64) {
    let mut current = target.load(Ordering::Relaxed);
    while value < current {
        match target.compare_exchange_weak(current, value, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(next) => current = next,
        }
    }
}

fn atomic_max(target: &AtomicU64, value: u64) {
    let mut current = target.load(Ordering::Relaxed);
    while value > current {
        match target.compare_exchange_weak(current, value, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(next) => current = next,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timing_and_counter_snapshot_aggregates() {
        let metrics = PlaybackMetrics::default();

        metrics.record_timing(
            "playback_ttff_ms",
            &[("stream", "direct"), ("path", "stream")],
            100.0,
        );
        metrics.record_timing(
            "playback_ttff_ms",
            &[("stream", "direct"), ("path", "stream")],
            300.0,
        );
        metrics.increment_counter("rebuffer_count", &[("stream", "hls")], 2);

        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.timings.len(), 1);
        assert_eq!(snapshot.counters.len(), 1);

        let ttff = &snapshot.timings[0];
        assert_eq!(ttff.count, 2);
        assert_eq!(ttff.min_ms, 100.0);
        assert_eq!(ttff.max_ms, 300.0);
        assert_eq!(ttff.avg_ms, 200.0);

        assert_eq!(snapshot.counters[0].value, 2);
    }

    #[test]
    fn reset_clears_all_series() {
        let metrics = PlaybackMetrics::default();

        metrics.record_timing("seek_latency_ms", &[("mode", "direct")], 42.0);
        metrics.increment_counter("rebuffer_count", &[("stream", "hls")], 1);
        metrics.reset();

        let snapshot = metrics.snapshot();
        assert!(snapshot.timings.is_empty());
        assert!(snapshot.counters.is_empty());
    }
}
