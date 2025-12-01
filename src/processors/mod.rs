pub mod packet_processor;
pub mod security_processor;  // 添加安全处理器模块

// 重新导出 packet_processor 中的公共类型
pub use packet_processor::{
    PacketStatsProcessor, PacketStatsSnapshot,
    IPAnalysisProcessor, ThreatDetectionProcessor
};

// 重新导出 security_processor 中的公共类型
pub use security_processor::{EncryptionProcessor, DecryptionProcessor};

use async_trait::async_trait;
use crate::DataChunk;

#[async_trait]
pub trait Processor: Send + Sync {
    async fn process(&mut self, chunk: &mut DataChunk) -> anyhow::Result<ProcessResult>;
    fn name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub enum ProcessResult {
    Continue,  // 继续处理
    Skip,      // 跳过后续处理
    Stop,      // 停止处理
}

// 数据过滤处理器
#[derive(Clone)]
pub struct FilterProcessor {
    pattern: Vec<u8>,
    name: String,
}

impl FilterProcessor {
    pub fn new(pattern: Vec<u8>) -> Self {
        let name = format!("FilterProcessor({})", String::from_utf8_lossy(&pattern));
        Self { pattern, name }
    }
}

#[async_trait]
impl Processor for FilterProcessor {
    async fn process(&mut self, chunk: &mut DataChunk) -> anyhow::Result<ProcessResult> {
        if self.pattern.is_empty() {
            return Ok(ProcessResult::Continue);
        }

        // 如果数据包含指定模式，则继续处理；否则跳过
        if chunk.data.windows(self.pattern.len()).any(|window| window == self.pattern) {
            Ok(ProcessResult::Continue)
        } else {
            Ok(ProcessResult::Skip)
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// 大写转换处理器
#[derive(Clone)]
pub struct UpperCaseProcessor;

impl UpperCaseProcessor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Processor for UpperCaseProcessor {
    async fn process(&mut self, chunk: &mut DataChunk) -> anyhow::Result<ProcessResult> {
        chunk.data = chunk.data.iter().map(|&b| b.to_ascii_uppercase()).collect();
        Ok(ProcessResult::Continue)
    }

    fn name(&self) -> &str {
        "UpperCaseProcessor"
    }
}