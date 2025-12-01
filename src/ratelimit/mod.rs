use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::warn;

/// 简化的速率限制器 - 使用延迟而不是信号量
#[derive(Debug, Clone)]
pub struct RateLimiter {
    max_per_second: u32,
}

impl RateLimiter {
    pub fn new(max_per_second: u32) -> Self {
        Self { max_per_second }
    }

    pub async fn wait(&self) {
        if self.max_per_second > 0 {
            let delay = Duration::from_secs(1) / self.max_per_second;
            time::sleep(delay).await;
        }
    }

    pub fn should_wait(&self) -> bool {
        self.max_per_second > 0
    }
}

/// 简化的背压控制器
#[derive(Debug, Clone)]
pub struct BackpressureController {
    max_queue_size: usize,
    current_size: Arc<std::sync::atomic::AtomicUsize>,
    warning_threshold: f64,
}

impl BackpressureController {
    pub fn new(max_queue_size: usize, warning_threshold: f64) -> Self {
        let threshold = warning_threshold.clamp(0.0, 1.0);
        Self {
            max_queue_size,
            current_size: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            warning_threshold: threshold,
        }
    }

    pub fn can_accept(&self) -> bool {
        let current = self.current_size.load(std::sync::atomic::Ordering::Relaxed);
        current < self.max_queue_size
    }

    pub fn increment(&self) -> bool {
        let current = self.current_size.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let new_size = current + 1;

        let warning_level = (self.max_queue_size as f64) * self.warning_threshold;
        if new_size as f64 > warning_level {
            warn!(
                "Backpressure warning: queue at {:.1}% capacity",
                (new_size as f64 / self.max_queue_size as f64) * 100.0
            );
        }

        new_size <= self.max_queue_size
    }

    pub fn decrement(&self) {
        self.current_size.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get_utilization(&self) -> f64 {
        let current = self.current_size.load(std::sync::atomic::Ordering::Relaxed);
        current as f64 / self.max_queue_size as f64
    }
}

// 不再需要 RateLimitPermit，因为简化版本不需要它