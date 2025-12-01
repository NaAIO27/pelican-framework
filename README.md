# 鹈鹕框架 (Pelican Framework) 技术文档
## 技术栈

编程语言与框架：
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-000000?style=for-the-badge&logo=tokio&logoColor=white)
![Async-Await](https://img.shields.io/badge/Async/Await-000000?style=for-the-badge&logo=asynchronous&logoColor=white)

架构模式：
![Pipeline-Filter](https://img.shields.io/badge/Pipeline_Filter_Architecture-0052CC?style=for-the-badge&logo=apachekafka&logoColor=white)
![EDA](https://img.shields.io/badge/Event_Driven-FF6F61?style=for-the-badge&logo=eventbrite&logoColor=white)
![Microservices](https://img.shields.io/badge/Microservices-FF9900?style=for-the-badge&logo=microservices&logoColor=white)

数据处理：
![Data-Streaming](https://img.shields.io/badge/Data_Streaming-2496ED?style=for-the-badge&logo=apacheflink&logoColor=white)
![Real-Time](https://img.shields.io/badge/Real_Time-00A98F?style=for-the-badge&logo=clock&logoColor=white)
![Zero-Copy](https://img.shields.io/badge/Zero_Copy-8B0000?style=for-the-badge&logo=memory&logoColor=white)

安全特性：
![Integrity](https://img.shields.io/badge/Data_Integrity-32CD32?style=for-the-badge&logo=shield-check&logoColor=white)

监控与可观测性：
![Metrics](https://img.shields.io/badge/Metrics_&_Monitoring-E95420?style=for-the-badge&logo=prometheus&logoColor=white)
![Observability](https://img.shields.io/badge/Observability-FF6B6B?style=for-the-badge&logo=datadog&logoColor=white)

配置与部署：
![YAML](https://img.shields.io/badge/YAML_Config-000080?style=for-the-badge&logo=yaml&logoColor=white)
![JSON](https://img.shields.io/badge/JSON_Config-000000?style=for-the-badge&logo=json&logoColor=white)
![TOML](https://img.shields.io/badge/TOML_Config-000000?style=for-the-badge&logo=toml&logoColor=white)
## 项目概述
鹈鹕框架是一个用Rust编写的高性能数据流处理框架，专注于实时数据流处理任务。其核心设计理念是"管道-过滤器"架构，通过可插拔的组件系统实现灵活的数据处理流水线。

## 项目核心价值
高性能：基于Tokio的异步运行时，零拷贝设计

模块化：松耦合的组件设计，易于扩展和维护

安全性：内置数据加密和完整性检查机制

可观测性：完整的指标收集和监控系统

## 架构设计
核心架构模式：管道-过滤器

```
text
数据源(Source) → 处理器(Processor) → 输出端(Sink)
组件层级关系
```


```
text
lib.rs (顶层导出)
├── pipeline.rs (管道协调器)
├── sources/ (数据生产者)
├── processors/ (数据转换器) 
├── sinks/ (数据消费者)
├── metrics/ (指标收集)
├── ratelimit/ (流量控制)
├── security/ (安全模块)
└── config/ (配置管理)
## 核心数据结构
DataChunk - 数据流的基本单元
rust
pub struct DataChunk {
    pub data: Vec<u8>,      // 原始字节数据
    pub timestamp: u64,     // 创建时间戳(UNIX秒)
    pub sequence: u64,      // 全局序列号
}
```

## 组件系统详解
### 1. 数据源 (Source) - 数据生产者
核心Trait

```
rust
#[async_trait]
pub trait Source: Send + Sync {
    async fn stream_data(&mut self, tx: mpsc::Sender<DataChunk>) -> anyhow::Result<()>;
    fn name(&self) -> &str;
}
```

内置实现
MemorySource: 从内存数组生成数据流

MockPacketSource: 模拟网络数据包生成器

RepeatingSource: 重复模式数据生成器

### 2. 处理器 (Processor) - 数据转换器
核心Trait

```
rust
#[async_trait]
pub trait Processor: Send + Sync {
    async fn process(&mut self, chunk: &mut DataChunk) -> anyhow::Result<ProcessResult>;
    fn name(&self) -> &str;
}
处理结果枚举
rust
pub enum ProcessResult {
    Continue,  // 继续处理链
    Skip,      // 跳过后续处理器
    Stop,      // 停止整个管道
}
```

## 处理器分类
基础处理器：

FilterProcessor: 模式匹配过滤器

UpperCaseProcessor: 大小写转换器

LogProcessor: 日志记录处理器

## 网络分析处理器：

PacketStatsProcessor: 数据包统计分析

IPAnalysisProcessor: IP地址分析器

ThreatDetectionProcessor: 威胁检测器

## 安全处理器：

EncryptionProcessor: 数据加密处理器

DecryptionProcessor: 数据解密处理器

### 3. 输出端 (Sink) - 数据消费者
核心Trait

```
rust
#[async_trait]
pub trait Sink: Send + Sync {
    async fn send(&mut self, chunk: DataChunk) -> anyhow::Result<()>;
    fn name(&self) -> &str;
}
```

#### 输出端类型
基础输出端：

ConsoleSink: 控制台输出

StatsSink: 统计信息输出

#### 高级输出端：

MetricsSink: 性能指标输出

SecureFileSink: 安全文件存储（支持完整性检查）

### 4. 管道 (Pipeline) - 协调器
构建器模式

```
rust
let pipeline = Pipeline::builder()
    .add_source(Box::new(MemorySource::new(data)))
    .add_processor(Box::new(FilterProcessor::new(pattern)))
    .add_processor(Box::new(UpperCaseProcessor::new()))
    .add_sink(Box::new(ConsoleSink::new()))
    .build();
```

### 执行流程
初始化所有组件

创建异步通道连接组件

启动数据源任务

主循环接收并处理数据

发送处理结果到输出端

清理和关闭

## 核心模块详解
### 安全模块 (security)
设计目的：为数据流提供安全保障

核心组件
DataEncryptor: 简化版XOR加密器（生产环境应替换为AES）

DataIntegrityChecker: 基于哈希的完整性检查器

SecurityConfig: 安全配置容器

 安全特性
端到端数据加密

数据完整性验证

可配置的安全策略

### 流量控制模块 (ratelimit)
设计目的：防止系统过载，保证稳定运行

 核心组件
RateLimiter: 基于令牌桶的速率限制器

BackpressureController: 背压控制器

 控制策略
令牌桶算法实现速率限制

队列监控实现背压控制

可配置的警告阈值

#### 指标收集模块 (metrics)
设计目的：系统可观测性和性能监控

组件层次
MetricsCollector: 基础指标收集器

AdvancedMetricsCollector: 高级指标收集器

收集指标
处理吞吐量（数据包/秒）

数据量统计（字节/秒）

处理延迟分布

组件级性能指标

错误率和失败统计

### 配置模块 (config)
设计目的：支持声明式管道配置

核心结构
PipelineConfig: 完整管道配置

ComponentConfig: 组件配置基类

## 配置格式支持
YAML（主要格式）

JSON

TOML

### 异步设计模式
基于Tokio的并发模型

```
rust
// 管道主循环
while let Some(mut chunk) = processor_rx.recv().await {
    // 异步处理每个数据块
    for processor in &mut self.processors {
        processor.process(&mut chunk).await?;
    }
    // 异步发送到输出端
    for sink in &mut self.sinks {
        sink.send(chunk.clone()).await?;
    }
}
```

### 通道通信模式
mpsc通道: 连接Source到Processor

广播通道: 支持多个Sink并行接收

容量限制: 防止内存泄漏

错误处理策略
分层错误处理

```
rust
// 组件级错误 - 继续处理其他数据
match processor.process(&mut chunk).await {
    Ok(ProcessResult::Continue) => { /* 正常继续 */ }
    Ok(ProcessResult::Skip) => { /* 跳过此数据 */ }
    Ok(ProcessResult::Stop) => { /* 优雅停止 */ }
    Err(e) => {
        // 记录错误但继续运行
        error!("Processor error: {}", e);
    }
}

// 管道级错误 - 整体失败
pub async fn run(mut self) -> anyhow::Result<()> {
    // 管道运行逻辑
}
```

### 性能优化特性
1. 零拷贝设计
DataChunk在组件间传递时使用引用或克隆

处理器直接在原数据上操作

2. 异步流水线
各阶段并行执行

非阻塞I/O操作

3. 内存高效
使用Vec<u8>存储二进制数据

及时释放已处理数据

4. 并发安全
所有组件实现Send + Sync

使用Arc进行安全共享

## 扩展指南
### 添加新数据源
在sources/目录创建新文件

实现Source trait

在sources/mod.rs中导出

在lib.rs中重新导出

### 添加新处理器
在processors/目录创建新文件

实现Processor trait

返回适当的ProcessResult

在processors/mod.rs中导出

### 添加新输出端
在sinks/目录创建新文件

实现Sink trait

在sinks/mod.rs中导出

## 配置示例
YAML配置示例

```
yaml
name: "网络监控管道"
description: "实时网络数据包分析和威胁检测"

sources:
  - name: "packet_source"
    type: "MockPacketSource"
    params:
      packet_count: 1000
      delay_ms: 0

processors:
  - name: "packet_stats"
    type: "PacketStatsProcessor"
  - name: "threat_detection"
    type: "ThreatDetectionProcessor"

sinks:
  - name: "console_output"
    type: "ConsoleSink"
  - name: "metrics_output"
    type: "MetricsSink"
```

运行时特性
## 启动流程

```
text
1. 解析配置 -> 2. 构建组件 -> 3. 连接管道 -> 
4. 启动异步任务 -> 5. 监控运行 -> 6. 优雅关闭
```

## 监控指标

```
text
- 系统级: CPU/内存使用率
- 管道级: 吞吐量/延迟
- 组件级: 处理量/错误率
```

## 最佳实践
### 1. 组件设计原则
单一职责：每个组件只做一件事

无状态设计：尽量设计为无状态处理器

明确接口：输入输出类型清晰

### 2. 错误处理原则
组件级错误不影响整体管道

关键错误触发优雅关闭

详细日志记录便于排查

### 3. 性能优化原则
避免数据复制，使用引用传递

使用异步I/O操作

合理设置通道缓冲区大小

## 故障排查
常见问题及解决
内存泄漏

检查DataChunk是否正确释放

监控通道队列长度

## 性能瓶颈

使用MetricsSink查看性能指标

检查处理器链的长度和复杂度

## 死锁风险

避免在处理器中进行阻塞操作

使用tokio::time::timeout设置超时

# 项目结构映射
## 物理结构

```
text
src/
├── lib.rs                    # 公共接口和核心类型
├── pipeline.rs              # 管道实现
├── processors/              # 处理器模块
│   ├── mod.rs              # 处理器基类和导出
│   ├── packet_processor.rs # 网络处理器
│   └── security_processor.rs # 安全处理器
├── sources/                 # 数据源模块
│   ├── mod.rs              # 数据源基类和导出
│   └── packet_source.rs    # 网络数据源
├── sinks/                  # 输出端模块
│   ├── mod.rs              # 输出端基类和导出
│   ├── advanced_sinks.rs   # 高级输出端
│   └── security_sink.rs    # 安全输出端
├── metrics/                # 指标模块
│   ├── mod.rs              # 基础指标收集器
│   └── advanced.rs         # 高级指标收集器
├── ratelimit/              # 流量控制模块
│   └── mod.rs
├── security/               # 安全模块
│   └── mod.rs
└── config/                 # 配置模块
    └── mod.rs
```

## 逻辑依赖关系

```
text
          lib.rs (入口点)
            │
            ├── pipeline (管道协调)
            │     ├── sources (数据输入)
            │     ├── processors (数据处理)
            │     └── sinks (数据输出)
            │
            ├── metrics (监控)
            ├── ratelimit (流量控制)
            ├── security (安全)
            └── config (配置)
```

# 设计模式总结
构建者模式：PipelineBuilder

策略模式：可插拔的Processor实现

观察者模式：多个Sink同时接收数据

管道-过滤器模式：整体架构模式

装饰器模式：处理器链式组合

## 安全注意事项
生产环境建议
加密模块：替换XOR加密为AES-256-GCM

密钥管理：使用安全的密钥管理系统

完整性验证：使用HMAC替代简单哈希

访问控制：添加组件级别的权限控制

## 性能调优建议
缓冲区大小：根据数据量调整通道缓冲区

并发度：根据CPU核心数调整任务数量

批处理：对大量小数据包进行批处理

内存池：为DataChunk实现对象池

## 未来扩展方向
短期目标
添加更多内置组件

完善配置系统

添加性能基准测试套件

长期目标
分布式处理支持

WebAssembly集成

可视化监控界面

插件系统支持

项目维护者：确保所有新组件遵循现有设计模式，保持API一致性，在添加新功能时更新此文档。