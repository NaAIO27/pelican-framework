// 只声明一次模块
pub mod packet_source;

// 重新导出公共类型
pub use packet_source::{PacketCaptureSource, MockPacketSource};

use async_trait::async_trait;
use tokio::sync::mpsc;
use crate::DataChunk;

#[async_trait]
pub trait Source: Send + Sync {
    async fn stream_data(&mut self, tx: mpsc::Sender<DataChunk>) -> anyhow::Result<()>;
    fn name(&self) -> &str;
}

// 内存数据源
pub struct MemorySource {
    data: Vec<Vec<u8>>,
    sequence: u64,
    name: String,
}

impl MemorySource {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        Self {
            data,
            sequence: 0,
            name: "MemorySource".to_string(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

#[async_trait]
impl Source for MemorySource {
    async fn stream_data(&mut self, tx: mpsc::Sender<DataChunk>) -> anyhow::Result<()> {
        for chunk_data in &self.data {
            let chunk = DataChunk::new(chunk_data.clone())
                .with_sequence(self.sequence);

            self.sequence += 1;

            if tx.send(chunk).await.is_err() {
                break; // 接收端已关闭
            }

            // 模拟一些延迟
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// 重复数据源 - 持续生成数据
pub struct RepeatingSource {
    template: Vec<u8>,
    count: usize,
    delay_ms: u64,
    name: String,
}

impl RepeatingSource {
    pub fn new(template: Vec<u8>, count: usize, delay_ms: u64) -> Self {
        Self {
            template,
            count,
            delay_ms,
            name: "RepeatingSource".to_string(),
        }
    }
}

#[async_trait]
impl Source for RepeatingSource {
    async fn stream_data(&mut self, tx: mpsc::Sender<DataChunk>) -> anyhow::Result<()> {
        for i in 0..self.count {
            let mut data = self.template.clone();
            data.extend_from_slice(format!("-{}", i).as_bytes());

            let chunk = DataChunk::new(data).with_sequence(i as u64);

            if tx.send(chunk).await.is_err() {
                break;
            }

            if self.delay_ms > 0 {
                tokio::time::sleep(tokio::time::Duration::from_millis(self.delay_ms)).await;
            }
        }
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}