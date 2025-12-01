use async_trait::async_trait;
use tokio::sync::mpsc;
use crate::{DataChunk, Source};
use tracing::info;  // 移除 error

// ... 其余代码保持不变 ...
pub struct PacketCaptureSource {
    device_name: String,
    filter: Option<String>,
    packet_count: usize,
}

impl PacketCaptureSource {
    pub fn new(device_name: &str) -> Self {
        Self {
            device_name: device_name.to_string(),
            filter: None,
            packet_count: 1000,
        }
    }

    pub fn with_filter(mut self, filter: &str) -> Self {
        self.filter = Some(filter.to_string());
        self
    }

    pub fn with_packet_count(mut self, count: usize) -> Self {
        self.packet_count = count;
        self
    }
}

#[async_trait]
impl Source for PacketCaptureSource {
    async fn stream_data(&mut self, tx: mpsc::Sender<DataChunk>) -> anyhow::Result<()> {
        info!("Starting packet capture on device: {}", self.device_name);

        // 暂时使用模拟数据
        info!("Real packet capture not implemented, using mock data instead");
        let mut mock_source = MockPacketSource::new(self.packet_count);
        mock_source.stream_data(tx).await?;

        info!("Packet capture completed: {} packets", self.packet_count);
        Ok(())
    }

    fn name(&self) -> &str {
        "PacketCaptureSource"
    }
}

// 模拟数据包源，用于测试（不需要pcap权限）
pub struct MockPacketSource {
    packet_count: usize,
    delay_ms: u64,
}

impl MockPacketSource {
    pub fn new(packet_count: usize) -> Self {
        Self {
            packet_count,
            delay_ms: 1,
        }
    }

    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }
}

#[async_trait]
impl Source for MockPacketSource {
    async fn stream_data(&mut self, tx: mpsc::Sender<DataChunk>) -> anyhow::Result<()> {
        info!("Starting mock packet source with {} packets", self.packet_count);

        for i in 0..self.packet_count {
            // 模拟不同类型的网络数据包
            let packet_data = match i % 5 {
                0 => format!("TCP packet from 192.168.1.{} to 10.0.0.1", i % 255).into_bytes(),
                1 => format!("UDP packet port {} size {}", 8000 + i % 1000, 512 + i % 1024).into_bytes(),
                2 => format!("HTTP GET /api/v1/data?id={}", i).into_bytes(),
                3 => format!("DNS query for example{}.com", i % 10).into_bytes(),
                _ => format!("ICMP packet seq {} ttl 64", i).into_bytes(),
            };

            let chunk = DataChunk::new(packet_data).with_sequence(i as u64);

            if tx.send(chunk).await.is_err() {
                break;
            }

            if self.delay_ms > 0 {
                tokio::time::sleep(tokio::time::Duration::from_millis(self.delay_ms)).await;
            }

            if i % 500 == 0 && i > 0 {
                info!("Generated {} mock packets", i);
            }
        }

        info!("Mock packet generation completed");
        Ok(())
    }

    fn name(&self) -> &str {
        "MockPacketSource"
    }
}