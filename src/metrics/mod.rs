// 声明 advanced 模块为公共
pub mod advanced;

// 重新导出 advanced 模块中的公共类型
pub use advanced::{AdvancedMetricsCollector, MetricsSummary};

// 原有的简单指标收集器
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
pub struct MetricsCollector {
    processed_count: Arc<AtomicU64>,
    error_count: Arc<AtomicU64>,
    start_time: Instant,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            processed_count: Arc::new(AtomicU64::new(0)),
            error_count: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
        }
    }

    pub fn record_processed(&self) {
        self.processed_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_error(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_metrics(&self) -> HashMap<String, u64> {
        let mut metrics = HashMap::new();
        metrics.insert(
            "processed_count".to_string(),
            self.processed_count.load(Ordering::Relaxed),
        );
        metrics.insert(
            "error_count".to_string(),
            self.error_count.load(Ordering::Relaxed)
        );
        metrics.insert(
            "uptime_seconds".to_string(),
            self.start_time.elapsed().as_secs(),
        );

        metrics
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}