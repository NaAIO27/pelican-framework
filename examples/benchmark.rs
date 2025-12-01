//! 基准测试示例

use pelican_framework::{
    Pipeline,
    sources::MockPacketSource,
    processors::{PacketStatsProcessor, UpperCaseProcessor},
    sinks::MetricsSink
};
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("🧪 鹈鹕框架 - 基准测试");
    println!("======================");

    let packet_counts = [100, 500, 1000];

    for &count in &packet_counts {
        println!("\n测试 {} 个数据包的处理性能...", count);

        let start_time = Instant::now();

        let pipeline = Pipeline::builder()
            .add_source(Box::new(MockPacketSource::new(count).with_delay(0)))
            .add_processor(Box::new(PacketStatsProcessor::new()))
            .add_processor(Box::new(UpperCaseProcessor::new()))
            .add_sink(Box::new(MetricsSink::new()))
            .build();

        pipeline.run().await?;

        let elapsed = start_time.elapsed();
        let packets_per_second = count as f64 / elapsed.as_secs_f64();

        println!("结果:");
        println!("  • 处理数量: {} 数据包", count);
        println!("  • 总耗时: {:.2} 秒", elapsed.as_secs_f64());
        println!("  • 吞吐量: {:.2} 数据包/秒", packets_per_second);
    }

    println!("\n✅ 基准测试完成!");
    Ok(())
}