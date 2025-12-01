pub mod pipeline;
pub mod processors;
pub mod sources;
pub mod sinks;
pub mod metrics;
pub mod ratelimit;
pub mod security;
pub mod config;

// 导出核心接口
pub use pipeline::{Pipeline, PipelineBuilder};
pub use processors::Processor;
pub use sources::Source;
pub use sinks::Sink;

// 重新导出常用组件
pub use processors::{
    FilterProcessor, UpperCaseProcessor,
    PacketStatsProcessor, PacketStatsSnapshot,
    IPAnalysisProcessor, ThreatDetectionProcessor,
    EncryptionProcessor, DecryptionProcessor
};
pub use sources::{MemorySource, MockPacketSource};
pub use sinks::{ConsoleSink, StatsSink, MetricsSink, Metrics, SecureFileSink};
// 根据您选择的版本调整导出
pub use ratelimit::{RateLimiter, BackpressureController}; // 简化版本不需要 RateLimitPermit
pub use metrics::{MetricsCollector, AdvancedMetricsCollector, MetricsSummary};
pub use security::{SecurityConfig, DataEncryptor, DataIntegrityChecker, SecurityError};
pub use config::PipelineConfig;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct DataChunk {
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub sequence: u64,
}

impl DataChunk {
    pub fn new(data: Vec<u8>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            data,
            timestamp,
            sequence: 0,
        }
    }

    pub fn with_sequence(mut self, sequence: u64) -> Self {
        self.sequence = sequence;
        self
    }
}