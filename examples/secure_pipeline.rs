//! 安全管道示例 - 展示加密、完整性检查和限流功能

use pelican_framework::{
    Pipeline,
    sources::MockPacketSource,
    processors::{
        PacketStatsProcessor,
        security_processor::{EncryptionProcessor, DecryptionProcessor}
    },
    sinks::{ConsoleSink, SecureFileSink},
    security::SecurityConfig,
    ratelimit::RateLimiter,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("🔒 鹈鹕框架 - 安全管道示例");
    println!("==============================");

    // 创建安全配置
    let security_config = SecurityConfig {
        enable_encryption: true,
        enable_integrity_check: true,
        encryption_key: Some(vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
        ]),
        integrity_salt: Some([1, 2, 3, 4, 5, 6, 7, 8]),
    };

    // 创建加密密钥
    let encryption_key = security_config.encryption_key.as_ref().unwrap();

    println!("🚀 启动安全管道...");
    let start_time = std::time::Instant::now();

    // 构建安全处理管道
    let pipeline = Pipeline::builder()
        .add_source(Box::new(MockPacketSource::new(50).with_delay(10)))

        // 添加统计处理器
        .add_processor(Box::new(PacketStatsProcessor::new()))

        // 添加加密处理器
        .add_processor(Box::new(
            EncryptionProcessor::new(encryption_key, "DataEncryptor").unwrap()
        ))

        // 添加解密处理器（演示用）
        .add_processor(Box::new(
            DecryptionProcessor::new(encryption_key, "DataDecryptor").unwrap()
        ))

        // 添加输出端
        .add_sink(Box::new(ConsoleSink::new()))
        .add_sink(Box::new(
            SecureFileSink::new(
                "secure_output.bin".to_string(),
                &security_config,
                "SecureFileSink"
            ).unwrap()
        ))

        .build();

    // 运行管道
    pipeline.run().await?;

    let elapsed = start_time.elapsed();
    println!("\n✅ 安全管道运行完成!");
    println!("总耗时: {:.2}秒", elapsed.as_secs_f64());

    println!("\n📊 安全功能已启用:");
    println!("  • 数据加密: ✅");
    println!("  • 完整性检查: ✅");
    println!("  • 输出文件: secure_output.bin");

    Ok(())
}