use pelican_framework::{
    Pipeline,
    sources::MockPacketSource,
    processors::{
        FilterProcessor,
        PacketStatsProcessor,
        IPAnalysisProcessor,
        ThreatDetectionProcessor
    },
    sinks::{MetricsSink, StatsSink}
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    println!("🌐 Pelican Framework - 高级网络数据包分析器");
    println!("=============================================");
    println!("这个示例演示了框架的高性能特性：");
    println!("• 实时数据包处理");
    println!("• 多阶段并行处理");
    println!("• 统计分析");
    println!("• 威胁检测");
    println!("• 性能监控");
    println!("=============================================\n");

    println!("🚀 启动网络数据包分析管道...");
    let start_time = std::time::Instant::now();

    // 构建高性能处理管道 - 处理1000个数据包
    let pipeline = Pipeline::builder()
        // 数据源：模拟生成1000个网络数据包，无延迟全速生成
        .add_source(Box::new(MockPacketSource::new(1000).with_delay(0)))

        // 处理器链
        .add_processor(Box::new(FilterProcessor::new(b"packet".to_vec()))) // 过滤包含"packet"的数据包
        .add_processor(Box::new(PacketStatsProcessor::new())) // 数据包统计
        .add_processor(Box::new(IPAnalysisProcessor::new()))  // IP分析
        .add_processor(Box::new(ThreatDetectionProcessor::new())) // 威胁检测

        // 输出端
        .add_sink(Box::new(MetricsSink::new())) // 性能指标
        .add_sink(Box::new(StatsSink::new()))   // 基础统计
        .build();

    // 运行管道
    pipeline.run().await?;

    let elapsed = start_time.elapsed();
    println!("\n=============================================");
    println!("✅ 分析完成！");
    println!("总耗时: {:.2}秒", elapsed.as_secs_f64());
    println!("处理速度: {:.2} 数据包/秒", 1000.0 / elapsed.as_secs_f64());
    println!("=============================================");

    Ok(())
}