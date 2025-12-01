use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // 获取参数 - cargo run 会把参数放在第二个位置
    let args: Vec<String> = env::args().collect();

    // 如果是 cargo run，第一个参数是程序名，第二个是 cargo run 的参数，第三个才是我们的参数
    // 如果是直接运行二进制，第一个是程序名，第二个是我们的参数
    let example = if args.len() > 2 {
        &args[2]  // cargo run -- example_name
    } else if args.len() > 1 {
        &args[1]  // ./pelican-framework example_name
    } else {
        "basic"
    };

    match example {
        "network" => run_network_analyzer().await,
        "performance" => run_performance_demo().await,
        "benchmark" => run_benchmark().await,
        _ => run_basic_example().await,
    }
}

async fn run_basic_example() -> anyhow::Result<()> {
    println!("🚀 运行基础示例...");
    // ... 基础示例代码
    Ok(())
}

async fn run_network_analyzer() -> anyhow::Result<()> {
    println!("🌐 运行网络分析器...");
    // ... 网络分析器代码
    Ok(())
}

async fn run_performance_demo() -> anyhow::Result<()> {
    println!("⚡ 运行性能演示...");
    // ... 性能演示代码
    Ok(())
}

async fn run_benchmark() -> anyhow::Result<()> {
    println!("🧪 运行基准测试...");
    // ... 基准测试代码
    Ok(())
}