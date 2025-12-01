pub mod advanced_sinks;
pub mod security_sink;  // 添加安全输出端模块

// 重新导出公共类型
pub use advanced_sinks::{MetricsSink, Metrics};
pub use security_sink::SecureFileSink;  // 导出安全输出端

use async_trait::async_trait;
use crate::DataChunk;
use tracing::info;

#[async_trait]
pub trait Sink: Send + Sync {
    async fn send(&mut self, chunk: DataChunk) -> anyhow::Result<()>;
    fn name(&self) -> &str;
}

// 控制台输出
pub struct ConsoleSink {
    name: String,
}

impl ConsoleSink {
    pub fn new() -> Self {
        Self {
            name: "ConsoleSink".to_string(),
        }
    }
}

#[async_trait]
impl Sink for ConsoleSink {
    async fn send(&mut self, chunk: DataChunk) -> anyhow::Result<()> {
        let data_str = String::from_utf8_lossy(&chunk.data);
        info!(
            "[{}] Seq: {}, Time: {}, Data: '{}'",
            self.name, chunk.sequence, chunk.timestamp, data_str
        );
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// 统计输出 - 只记录统计信息
pub struct StatsSink {
    count: u64,
    total_bytes: u64,
    name: String,
}

impl StatsSink {
    pub fn new() -> Self {
        Self {
            count: 0,
            total_bytes: 0,
            name: "StatsSink".to_string(),
        }
    }
}

#[async_trait]
impl Sink for StatsSink {
    async fn send(&mut self, chunk: DataChunk) -> anyhow::Result<()> {
        self.count += 1;
        self.total_bytes += chunk.data.len() as u64;

        if self.count % 10 == 0 {
            info!(
                "[{}] Processed {} chunks, total bytes: {}",
                self.name, self.count, self.total_bytes
            );
        }

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}