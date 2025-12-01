use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    let example_name = args.get(1).map(|s| s.as_str()).unwrap_or("basic");

    match example_name {
        "basic" => run_basic_example().await,
        "network" => run_network_analyzer().await,
        "performance" => run_performance_demo().await,
        "benchmark" => run_benchmark().await,
        _ => {
            println!("未知示例: {}", example_name);
            print_usage();
            Ok(())
        }
    }
}

fn print_usage() {
    println!();
    println!("🐦 鹈鹕框架 (Pelican Framework) - 示例运行器");
    println!("=============================================");
    println!("用法: cargo run [示例名称]");
    println!();
    println!("可用示例:");
    println!("  basic        - 基础数据处理示例");
    println!("  network      - 高级网络数据包分析器");
    println!("  performance  - 性能演示");
    println!("  benchmark    - 基准测试");
    println!();
    println!("例如:");
    println!("  cargo run basic");
    println!("  cargo run network");
    println!("  cargo run performance");
}

async fn run_basic_example() -> anyhow::Result<()> {
    use pelican_framework::{
        Pipeline,
        sources::MemorySource,
        processors::{FilterProcessor, UpperCaseProcessor},
        sinks::ConsoleSink
    };

    println!("🚀 运行基础示例...");

    let test_data = vec![
        b"hello world".to_vec(),
        b"pelican framework".to_vec(),
        b"rust streaming".to_vec(),
        b"hello pelican".to_vec(),
    ];

    let pipeline = Pipeline::builder()
        .add_source(Box::new(MemorySource::new(test_data)))
        .add_processor(Box::new(FilterProcessor::new(b"hello".to_vec())))
        .add_processor(Box::new(UpperCaseProcessor::new()))
        .add_sink(Box::new(ConsoleSink::new()))
        .build();

    pipeline.run().await?;

    println!("✅ 基础示例完成!");
    Ok(())
}

async fn run_network_analyzer() -> anyhow::Result<()> {
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

    println!("🌐 运行网络数据包分析器...");

    let pipeline = Pipeline::builder()
        .add_source(Box::new(MockPacketSource::new(500).with_delay(0)))
        .add_processor(Box::new(FilterProcessor::new(b"packet".to_vec())))
        .add_processor(Box::new(PacketStatsProcessor::new()))
        .add_processor(Box::new(IPAnalysisProcessor::new()))
        .add_processor(Box::new(ThreatDetectionProcessor::new()))
        .add_sink(Box::new(MetricsSink::new()))
        .add_sink(Box::new(StatsSink::new()))
        .build();

    pipeline.run().await?;

    println!("✅ 网络分析完成!");
    Ok(())
}

async fn run_performance_demo() -> anyhow::Result<()> {
    use pelican_framework::{
        Pipeline,
        sources::MockPacketSource,
        processors::{
            FilterProcessor,
            PacketStatsProcessor,
            UpperCaseProcessor,
            ThreatDetectionProcessor
        },
        sinks::MetricsSink
    };
    use std::time::Instant;

    println!("⚡ 运行性能演示...");

    let packet_count = 1000;
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

    Ok(())
}

async fn run_benchmark() -> anyhow::Result<()> {
    use pelican_framework::{
        Pipeline,
        sources::MockPacketSource,
        processors::{PacketStatsProcessor, UpperCaseProcessor},
        sinks::MetricsSink
    };
    use std::time::Instant;

    println!("🧪 运行基准测试...");

    let packet_counts = [100, 500, 1000];

    for &count in &packet_counts {
        println!("\n测试 {} 个数据包...", count);

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
        println!("  • 耗时: {:.2} 秒", elapsed.as_secs_f64());
        println!("  • 吞吐量: {:.2} 数据包/秒", packets_per_second);
    }

    println!("\n✅ 基准测试完成!");
    Ok(())
}