use pelican_framework::{
    Pipeline,
    sources::MockPacketSource,
    processors::{
        FilterProcessor,
        PacketStatsProcessor,
        UpperCaseProcessor,
        ThreatDetectionProcessor
    },
    sinks::{MetricsSink, ConsoleSink}
};
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    println!("⚡ Pelican Framework - 性能演示");
    println!("================================");

    // 测试不同规模的数据处理
    let test_cases = vec![
        (100, "小型数据流"),
        (1000, "中型数据流"),
        (5000, "大型数据流"),
    ];

    for (packet_count, description) in test_cases {
        println!("\n🧪 测试: {} ({}个数据包)", description, packet_count);
        println!("{}", "-".repeat(40));

        let start_time = Instant::now();

        let pipeline = Pipeline::builder()
            .add_source(Box::new(MockPacketSource::new(packet_count).with_delay(0)))
            .add_processor(Box::new(FilterProcessor::new(b"TCP".to_vec())))
            .add_processor(Box::new(PacketStatsProcessor::new()))
            .add_processor(Box::new(UpperCaseProcessor::new()))
            .add_processor(Box::new(ThreatDetectionProcessor::new()))
            .add_sink(Box::new(MetricsSink::new()))
            .build();

        pipeline.run().await?;

        let elapsed = start_time.elapsed();
        let throughput = packet_count as f64 / elapsed.as_secs_f64();

        println!("📊 性能结果:");
        println!("  • 处理数量: {} 数据包", packet_count);
        println!("  • 总耗时: {:.3} 秒", elapsed.as_secs_f64());
        println!("  • 吞吐量: {:.2} 数据包/秒", throughput);
        println!("  • 平均延迟: {:.3} 毫秒/数据包",
                 elapsed.as_millis() as f64 / packet_count as f64);
    }

    println!("\n================================");
    println!("✅ 性能演示完成！");
    println!("鹈鹕框架展示了优秀的数据处理能力！");

    Ok(())
}