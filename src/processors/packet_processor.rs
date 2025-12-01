use async_trait::async_trait;
use crate::DataChunk;
use crate::processors::{Processor, ProcessResult};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tracing::info;  // 移除 debug

// ... 其余代码保持不变 ...

// 数据包统计分析器
pub struct PacketStatsProcessor {
    stats: Arc<PacketStats>,
    name: String,
}

#[derive(Default)]
struct PacketStats {
    total_packets: AtomicU64,
    tcp_packets: AtomicU64,
    udp_packets: AtomicU64,
    http_packets: AtomicU64,
    dns_packets: AtomicU64,
    other_packets: AtomicU64,
    total_bytes: AtomicU64,
}

impl PacketStatsProcessor {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(PacketStats::default()),
            name: "PacketStatsProcessor".to_string(),
        }
    }

    pub fn get_stats(&self) -> PacketStatsSnapshot {
        PacketStatsSnapshot {
            total_packets: self.stats.total_packets.load(Ordering::Relaxed),
            tcp_packets: self.stats.tcp_packets.load(Ordering::Relaxed),
            udp_packets: self.stats.udp_packets.load(Ordering::Relaxed),
            http_packets: self.stats.http_packets.load(Ordering::Relaxed),
            dns_packets: self.stats.dns_packets.load(Ordering::Relaxed),
            other_packets: self.stats.other_packets.load(Ordering::Relaxed),
            total_bytes: self.stats.total_bytes.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PacketStatsSnapshot {
    pub total_packets: u64,
    pub tcp_packets: u64,
    pub udp_packets: u64,
    pub http_packets: u64,
    pub dns_packets: u64,
    pub other_packets: u64,
    pub total_bytes: u64,
}

#[async_trait]
impl Processor for PacketStatsProcessor {
    async fn process(&mut self, chunk: &mut DataChunk) -> anyhow::Result<ProcessResult> {
        self.stats.total_packets.fetch_add(1, Ordering::Relaxed);
        self.stats.total_bytes.fetch_add(chunk.data.len() as u64, Ordering::Relaxed);

        let data_str = String::from_utf8_lossy(&chunk.data);

        if data_str.contains("TCP") {
            self.stats.tcp_packets.fetch_add(1, Ordering::Relaxed);
        } else if data_str.contains("UDP") {
            self.stats.udp_packets.fetch_add(1, Ordering::Relaxed);
        } else if data_str.contains("HTTP") {
            self.stats.http_packets.fetch_add(1, Ordering::Relaxed);
        } else if data_str.contains("DNS") {
            self.stats.dns_packets.fetch_add(1, Ordering::Relaxed);
        } else {
            self.stats.other_packets.fetch_add(1, Ordering::Relaxed);
        }

        // 每100个包打印一次统计
        let total = self.stats.total_packets.load(Ordering::Relaxed);
        if total % 100 == 0 {
            let stats = self.get_stats();
            info!(
                "Packet Stats - Total: {}, TCP: {}, UDP: {}, HTTP: {}, DNS: {}, Other: {}, Bytes: {}",
                stats.total_packets, stats.tcp_packets, stats.udp_packets,
                stats.http_packets, stats.dns_packets, stats.other_packets,
                stats.total_bytes
            );
        }

        Ok(ProcessResult::Continue)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// IP地址分析器
pub struct IPAnalysisProcessor {
    ip_counter: HashMap<String, u64>,
    name: String,
}

impl IPAnalysisProcessor {
    pub fn new() -> Self {
        Self {
            ip_counter: HashMap::new(),
            name: "IPAnalysisProcessor".to_string(),
        }
    }

    pub fn get_top_ips(&self, limit: usize) -> Vec<(String, u64)> {
        let mut ips: Vec<_> = self.ip_counter.iter().collect();
        ips.sort_by(|a, b| b.1.cmp(a.1));
        ips.into_iter()
            .take(limit)
            .map(|(ip, &count)| (ip.clone(), count))
            .collect()
    }
}

#[async_trait]
impl Processor for IPAnalysisProcessor {
    async fn process(&mut self, chunk: &mut DataChunk) -> anyhow::Result<ProcessResult> {
        let data_str = String::from_utf8_lossy(&chunk.data);

        // 简单的IP地址提取（实际应用中应该使用更复杂的解析）
        for part in data_str.split_whitespace() {
            if part.contains('.') && part.chars().all(|c| c.is_ascii_digit() || c == '.') {
                let ip = part.to_string();
                *self.ip_counter.entry(ip).or_insert(0) += 1;
            }
        }

        // 每200个包打印一次IP统计
        if chunk.sequence % 200 == 0 && chunk.sequence > 0 {
            let top_ips = self.get_top_ips(5);
            info!("Top IPs by packet count: {:?}", top_ips);
        }

        Ok(ProcessResult::Continue)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// 实时威胁检测处理器
pub struct ThreatDetectionProcessor {
    suspicious_patterns: Vec<Vec<u8>>,
    threat_count: u64,
    name: String,
}

impl ThreatDetectionProcessor {
    pub fn new() -> Self {
        let suspicious_patterns = vec![
            b"malware".to_vec(),
            b"exploit".to_vec(),
            b"injection".to_vec(),
            b"SELECT * FROM".to_vec(), // SQL注入特征
            b"<script>".to_vec(),      // XSS特征
        ];

        Self {
            suspicious_patterns,
            threat_count: 0,
            name: "ThreatDetectionProcessor".to_string(),
        }
    }

    pub fn get_threat_count(&self) -> u64 {
        self.threat_count
    }
}

#[async_trait]
impl Processor for ThreatDetectionProcessor {
    async fn process(&mut self, chunk: &mut DataChunk) -> anyhow::Result<ProcessResult> {
        for pattern in &self.suspicious_patterns {
            if chunk.data.windows(pattern.len()).any(|window| window == pattern) {
                self.threat_count += 1;
                info!(
                    "🚨 THREAT DETECTED! Pattern: '{}' in packet {}",
                    String::from_utf8_lossy(pattern),
                    chunk.sequence
                );
                break;
            }
        }

        Ok(ProcessResult::Continue)
    }

    fn name(&self) -> &str {
        &self.name
    }
}