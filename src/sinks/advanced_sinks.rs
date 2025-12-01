use async_trait::async_trait;
use crate::DataChunk;
use super::Sink;
use std::fs::OpenOptions;
use std::io::Write;
use tokio::sync::mpsc;
use tracing::{info, error};
use std::time::{SystemTime, UNIX_EPOCH};

// 性能指标输出端
pub struct MetricsSink {
    start_time: std::time::Instant,
    packet_count: u64,
    total_bytes: u64,
    name: String,
}

impl MetricsSink {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            packet_count: 0,
            total_bytes: 0,
            name: "MetricsSink".to_string(),
        }
    }

    pub fn get_metrics(&self) -> Metrics {
        let elapsed = self.start_time.elapsed();
        let packets_per_second = if elapsed.as_secs() > 0 {
            self.packet_count as f64 / elapsed.as_secs() as f64
        } else {
            0.0
        };

        let bytes_per_second = if elapsed.as_secs() > 0 {
            self.total_bytes as f64 / elapsed.as_secs() as f64
        } else {
            0.0
        };

        Metrics {
            packet_count: self.packet_count,
            total_bytes: self.total_bytes,
            packets_per_second,
            bytes_per_second,
            elapsed_time: elapsed,
        }
    }
}

#[derive(Debug)]
pub struct Metrics {
    pub packet_count: u64,
    pub total_bytes: u64,
    pub packets_per_second: f64,
    pub bytes_per_second: f64,
    pub elapsed_time: std::time::Duration,
}

#[async_trait]
impl Sink for MetricsSink {
    async fn send(&mut self, chunk: DataChunk) -> anyhow::Result<()> {
        self.packet_count += 1;
        self.total_bytes += chunk.data.len() as u64;

        // 每50个包打印一次性能指标
        if self.packet_count % 50 == 0 {
            let metrics = self.get_metrics();
            info!(
                "📊 Performance Metrics - Packets: {}/s, Throughput: {:.2} KB/s, Total: {} packets",
                metrics.packets_per_second as u64,
                metrics.bytes_per_second / 1024.0,
                metrics.packet_count
            );
        }

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}