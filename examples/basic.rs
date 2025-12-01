use pelican_framework::{
    Pipeline,
    sources::MemorySource,
    processors::{FilterProcessor, UpperCaseProcessor},
    sinks::ConsoleSink
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    println!("Starting simple Pelican Framework demo...");

    // 创建测试数据
    let test_data = vec![
        b"hello world".to_vec(),
        b"pelican framework".to_vec(),
        b"rust streaming".to_vec(),
        b"hello pelican".to_vec(),
    ];

    // 构建处理管道
    let pipeline = Pipeline::builder()
        .add_source(Box::new(MemorySource::new(test_data)))
        .add_processor(Box::new(FilterProcessor::new(b"hello".to_vec())))
        .add_processor(Box::new(UpperCaseProcessor::new()))
        .add_sink(Box::new(ConsoleSink::new()))
        .build();

    // 运行管道 - 注意现在需要转移所有权
    pipeline.run().await?;

    println!("Demo completed successfully!");
    Ok(())
}