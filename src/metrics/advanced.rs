use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct AdvancedMetricsCollector {
    // 基本指标
    processed_total: Arc<AtomicU64>,
    processed_bytes: Arc<AtomicU64>,
    errors_total: Arc<AtomicU64>,

    // 延迟指标
    total_latency: Arc<AtomicU64>,
    count_latency: Arc<AtomicU64>,

    // 吞吐量指标
    window_start: Arc<RwLock<Instant>>,
    window_count: Arc<AtomicU64>,
    window_bytes: Arc<AtomicU64>,

    // 组件级指标
    component_stats: Arc<RwLock<HashMap<String, ComponentStats>>>,
}

#[derive(Debug, Clone)]
pub struct ComponentStats {
    pub processed: u64,
    pub errors: u64,
    pub avg_latency: f64,
}

impl AdvancedMetricsCollector {
    pub fn new() -> Self {
        Self {
            processed_total: Arc::new(AtomicU64::new(0)),
            processed_bytes: Arc::new(AtomicU64::new(0)),
            errors_total: Arc::new(AtomicU64::new(0)),
            total_latency: Arc::new(AtomicU64::new(0)),
            count_latency: Arc::new(AtomicU64::new(0)),
            window_start: Arc::new(RwLock::new(Instant::now())),
            window_count: Arc::new(AtomicU64::new(0)),
            window_bytes: Arc::new(AtomicU64::new(0)),
            component_stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn record_processed(&self, bytes: usize, latency: Option<Duration>) {
        self.processed_total.fetch_add(1, Ordering::Relaxed);
        self.processed_bytes.fetch_add(bytes as u64, Ordering::Relaxed);
        self.window_count.fetch_add(1, Ordering::Relaxed);
        self.window_bytes.fetch_add(bytes as u64, Ordering::Relaxed);

        if let Some(latency) = latency {
            let latency_us = latency.as_micros() as u64;
            self.total_latency.fetch_add(latency_us, Ordering::Relaxed);
            self.count_latency.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_error(&self, component: &str) {
        self.errors_total.fetch_add(1, Ordering::Relaxed);

        if let Ok(mut stats) = self.component_stats.write() {
            let component_stat = stats.entry(component.to_string()).or_insert(ComponentStats {
                processed: 0,
                errors: 0,
                avg_latency: 0.0,
            });
            component_stat.errors += 1;
        }
    }

    pub fn record_component_processed(&self, component: &str, latency: Duration) {
        if let Ok(mut stats) = self.component_stats.write() {
            let component_stat = stats.entry(component.to_string()).or_insert(ComponentStats {
                processed: 0,
                errors: 0,
                avg_latency: 0.0,
            });

            let latency_us = latency.as_micros() as u64;
            let new_processed = component_stat.processed + 1;
            let new_avg = (component_stat.avg_latency * component_stat.processed as f64
                + latency_us as f64) / new_processed as f64;

            component_stat.processed = new_processed;
            component_stat.avg_latency = new_avg;
        }
    }

    pub fn get_throughput(&self) -> (f64, f64) {
        let window_start = self.window_start.read().unwrap();
        let elapsed = window_start.elapsed();

        if elapsed.as_secs() > 0 {
            let count = self.window_count.load(Ordering::Relaxed);
            let bytes = self.window_bytes.load(Ordering::Relaxed);

            let items_per_sec = count as f64 / elapsed.as_secs_f64();
            let bytes_per_sec = bytes as f64 / elapsed.as_secs_f64();

            (items_per_sec, bytes_per_sec)
        } else {
            (0.0, 0.0)
        }
    }

    pub fn reset_window(&self) {
        let mut window_start = self.window_start.write().unwrap();
        *window_start = Instant::now();
        self.window_count.store(0, Ordering::Relaxed);
        self.window_bytes.store(0, Ordering::Relaxed);
    }

    pub fn get_summary(&self) -> MetricsSummary {
        let processed = self.processed_total.load(Ordering::Relaxed);
        let errors = self.errors_total.load(Ordering::Relaxed);
        let total_latency = self.total_latency.load(Ordering::Relaxed);
        let count_latency = self.count_latency.load(Ordering::Relaxed);

        let avg_latency = if count_latency > 0 {
            total_latency as f64 / count_latency as f64
        } else {
            0.0
        };

        let (throughput_items, throughput_bytes) = self.get_throughput();

        MetricsSummary {
            processed_total: processed,
            processed_bytes: self.processed_bytes.load(Ordering::Relaxed),
            errors_total: errors,
            avg_latency_us: avg_latency,
            throughput_items_per_sec: throughput_items,
            throughput_bytes_per_sec: throughput_bytes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetricsSummary {
    pub processed_total: u64,
    pub processed_bytes: u64,
    pub errors_total: u64,
    pub avg_latency_us: f64,
    pub throughput_items_per_sec: f64,
    pub throughput_bytes_per_sec: f64,
}