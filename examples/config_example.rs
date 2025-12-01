//! 配置驱动示例

use pelican_framework::{
    Pipeline,
    config::PipelineConfig,
    sources::MockPacketSource,
    processors::{PacketStatsProcessor, UpperCaseProcessor},
    sinks::ConsoleSink
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("⚙️  鹈鹕框架 - 配置驱动示例");
    println!("==============================");

    // 创建配置
    let config = r#"
        name: "测试管道"
        description: "从配置创建的测试管道"
        max_queue_size: 1000

        sources:
          - name: "模拟数据源"
            type: "MockPacketSource"
            params:
              packet_count: 100
              delay_ms: 10

        processors:
          - name: "数据包统计"
            type: "PacketStatsProcessor"
            params: {}

          - name: "大写转换"
            type: "UpperCaseProcessor"
            params: {}

        sinks:
          - name: "控制台输出"
            type: "ConsoleSink"
            params: {}
    "#;

    // 从YAML解析配置
    let pipeline_config: PipelineConfig = serde_yaml::from_str(config)?;

    println!("从配置创建管道: {}", pipeline_config.name);

    // 手动创建管道（在实际实现中应该根据配置自动创建）
    let pipeline = Pipeline::builder()
        .add_source(Box::new(MockPacketSource::new(100).with_delay(10)))
        .add_processor(Box::new(PacketStatsProcessor::new()))
        .add_processor(Box::new(UpperCaseProcessor::new()))
        .add_sink(Box::new(ConsoleSink::new()))
        .build();

    pipeline.run().await?;

    println!("✅ 配置驱动示例完成!");
    Ok(())
}