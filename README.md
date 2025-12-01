# Pelican Framework Technical Documentation
## Tech Stack

Programming Languages & Frameworks:
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-000000?style=for-the-badge&logo=tokio&logoColor=white)
![Async-Await](https://img.shields.io/badge/Async/Await-000000?style=for-the-badge&logo=asynchronous&logoColor=white)

Architecture Patterns:
![Pipeline-Filter](https://img.shields.io/badge/Pipeline_Filter_Architecture-0052CC?style=for-the-badge&logo=apachekafka&logoColor=white)
![EDA](https://img.shields.io/badge/Event_Driven-FF6F61?style=for-the-badge&logo=eventbrite&logoColor=white)
![Microservices](https://img.shields.io/badge/Microservices-FF9900?style=for-the-badge&logo=microservices&logoColor=white)

Data Processing:
![Data-Streaming](https://img.shields.io/badge/Data_Streaming-2496ED?style=for-the-badge&logo=apacheflink&logoColor=white)
![Real-Time](https://img.shields.io/badge/Real_Time-00A98F?style=for-the-badge&logo=clock&logoColor=white)
![Zero-Copy](https://img.shields.io/badge/Zero_Copy-8B0000?style=for-the-badge&logo=memory&logoColor=white)

Security Features:
![Encryption](https://img.shields.io/badge/End--to--End_Encryption-4A90E2?style=for-the-badge&logo=lock&logoColor=white)
![Integrity](https://img.shields.io/badge/Data_Integrity-32CD32?style=for-the-badge&logo=shield-check&logoColor=white)

Monitoring & Observability:
![Metrics](https://img.shields.io/badge/Metrics_&_Monitoring-E95420?style=for-the-badge&logo=prometheus&logoColor=white)
![Observability](https://img.shields.io/badge/Observability-FF6B6B?style=for-the-badge&logo=datadog&logoColor=white)

Configuration & Deployment:
![YAML](https://img.shields.io/badge/YAML_Config-000080?style=for-the-badge&logo=yaml&logoColor=white)
![JSON](https://img.shields.io/badge/JSON_Config-000000?style=for-the-badge&logo=json&logoColor=white)
![TOML](https://img.shields.io/badge/TOML_Config-000000?style=for-the-badge&logo=toml&logoColor=white)

Performance Features:
![High-Performance](https://img.shields.io/badge/High_Performance-00D8FF?style=for-the-badge&logo=speedtest&logoColor=white)
![Concurrent](https://img.shields.io/badge/Concurrent_Safe-FF6F00?style=for-the-badge&logo=concurrency&logoColor=white)
![Memory-Safe](https://img.shields.io/badge/Memory_Safe-8A2BE2?style=for-the-badge&logo=memory-safe&logoColor=white)

Modularity Features:
![Modular](https://img.shields.io/badge/Modular_Design-FF69B4?style=for-the-badge&logo=module&logoColor=white)
![Plug-and-Play](https://img.shields.io/badge/Plug_and_Play-00C853?style=for-the-badge&logo=plugin&logoColor=white)
![Extensible](https://img.shields.io/badge/Extensible-6200EA?style=for-the-badge&logo=extension&logoColor=white)

Networking & Communication:
![Network-Analysis](https://img.shields.io/badge/Network_Analysis-1E88E5?style=for-the-badge&logo=network&logoColor=white)
![Packet-Processing](https://img.shields.io/badge/Packet_Processing-FF9800?style=for-the-badge&logo=network&logoColor=white)
![Async-Channels](https://img.shields.io/badge/Async_Channels-795548?style=for-the-badge&logo=channel&logoColor=white)

Traffic Control:
![Rate-Limiting](https://img.shields.io/badge/Rate_Limiting-3F51B5?style=for-the-badge&logo=speedometer&logoColor=white)
![Backpressure](https://img.shields.io/badge/Backpressure_Control-FF5252?style=for-the-badge&logo=pressure&logoColor=white)

## Project Overview
The Pelican Framework is a high-performance data stream processing framework written in Rust, focusing on real-time data stream processing tasks. Its core design philosophy is the "Pipeline-Filter" architecture, achieving flexible data processing pipelines through a pluggable component system.

## Project Core Values
High Performance: Based on Tokio's async runtime, zero-copy design

Modular: Loosely coupled component design, easy to extend and maintain

Security: Built-in data encryption and integrity checking mechanisms

Observability: Complete metrics collection and monitoring system

## Architecture Design
Core Architecture Pattern: Pipeline-Filter

```
Data Source(Source) → Processor(Processor) → Output(Sink)
Component Hierarchy
```

```
lib.rs (top-level exports)
├── pipeline.rs (pipeline coordinator)
├── sources/ (data producers)
├── processors/ (data transformers) 
├── sinks/ (data consumers)
├── metrics/ (metrics collection)
├── ratelimit/ (traffic control)
├── security/ (security module)
└── config/ (configuration management)
```

## Core Data Structures
DataChunk - Basic unit of data stream
````rust
pub struct DataChunk {
    pub data: Vec<u8>,      // Raw byte data
    pub timestamp: u64,     // Creation timestamp (UNIX seconds)
    pub sequence: u64,      // Global sequence number
}
```

## Component System Details
### 1. Source - Data Producer
Core Trait

```
rust
#[async_trait]
pub trait Source: Send + Sync {
    async fn stream_data(&mut self, tx: mpsc::Sender<DataChunk>) -> anyhow::Result<()>;
    fn name(&self) -> &str;
}
```

Built-in Implementations
MemorySource: Generates data streams from memory arrays

MockPacketSource: Simulated network packet generator

RepeatingSource: Repeating pattern data generator

### 2. Processor - Data Transformer
Core Trait

```
rust
#[async_trait]
pub trait Processor: Send + Sync {
    async fn process(&mut self, chunk: &mut DataChunk) -> anyhow::Result<ProcessResult>;
    fn name(&self) -> &str;
}
```
Processing Result Enum
```
rust
pub enum ProcessResult {
    Continue,  // Continue processing chain
    Skip,      // Skip subsequent processors
    Stop,      // Stop entire pipeline
}
```

## Processor Categories
Basic Processors:

FilterProcessor: Pattern matching filter

UpperCaseProcessor: Case converter

LogProcessor: Logging processor
## Network Analysis Processors:

PacketStatsProcessor: Packet statistical analysis

IPAnalysisProcessor: IP address analyzer

ThreatDetectionProcessor: Threat detector

## Security Processors:

EncryptionProcessor: Data encryption processor

DecryptionProcessor: Data decryption processor
### 3. Sink - Data Consumer
Core Trait

```
rust
#[async_trait]
pub trait Sink: Send + Sync {
    async fn send(&mut self, chunk: DataChunk) -> anyhow::Result<()>;
    fn name(&self) -> &str;
}
```

#### Sink Types
Basic Sinks:

ConsoleSink: Console output

StatsSink: Statistical information output

#### Advanced Sinks:

MetricsSink: Performance metrics output

SecureFileSink: Secure file storage (supports integrity checking)
### 4. Pipeline - Coordinator
Builder Pattern

```
rust
let pipeline = Pipeline::builder()
    .add_source(Box::new(MemorySource::new(data)))
    .add_processor(Box::new(FilterProcessor::new(pattern)))
    .add_processor(Box::new(UpperCaseProcessor::new()))
    .add_sink(Box::new(ConsoleSink::new()))
    .build();
```

### Execution Flow
Initialize all components

Create async channels to connect components

Start source tasks

Main loop receives and processes data

Send processed results to sinks

Cleanup and shutdown

Core Module Details

## Core Module Details
### Security Module (security)
Design Purpose: Provide security for data streams

Core Components
DataEncryptor: Simplified XOR encryptor (should be replaced with AES in production)

DataIntegrityChecker: Hash-based integrity checker

SecurityConfig: Security configuration container

Security Features
End-to-end data encryption

Data integrity verification

Configurable security policies

### Rate Limiting Module (ratelimit)
Design Purpose: Prevent system overload, ensure stable operation

Core Components
RateLimiter: Token bucket-based rate limiter

BackpressureController: Backpressure controller

Control Strategies
Token bucket algorithm for rate limiting

Queue monitoring for backpressure control

Configurable warning thresholds

#### Metrics Collection Module (metrics)
Design Purpose: System observability and performance monitoring

Component Hierarchy
MetricsCollector: Basic metrics collector

AdvancedMetricsCollector: Advanced metrics collector

Collected Metrics
Processing throughput (packets/sec)

Data volume statistics (bytes/sec)

Processing latency distribution

Component-level performance metrics

Error rates and failure statistics

### Configuration Module (config)
Design Purpose: Support declarative pipeline configuration

Core Structures
PipelineConfig: Complete pipeline configuration

ComponentConfig: Base component configuration class
## Supported Configuration Formats
YAML (primary format)

JSON

TOML

### Asynchronous Design Patterns
Tokio-based concurrency model
```
rust
// Pipeline main loop
while let Some(mut chunk) = processor_rx.recv().await {
    // Asynchronously process each data chunk
    for processor in &mut self.processors {
        processor.process(&mut chunk).await?;
    }
    // Asynchronously send to sinks
    for sink in &mut self.sinks {
        sink.send(chunk.clone()).await?;
    }
}
```

### Channel Communication Patterns
mpsc channels: Connect Source to Processor

Broadcast channels: Support multiple Sinks receiving in parallel

Capacity limits: Prevent memory leaks

Error Handling Strategies
Layered error handling
```
rust
/// Component-level errors - continue processing other data
match processor.process(&mut chunk).await {
    Ok(ProcessResult::Continue) => { /* Continue normally */ }
    Ok(ProcessResult::Skip) => { /* Skip this data */ }
    Ok(ProcessResult::Stop) => { /* Graceful stop */ }
    Err(e) => {
        // Log error but continue running
        error!("Processor error: {}", e);
    }
}

// Pipeline-level errors - overall failure
pub async fn run(mut self) -> anyhow::Result<()> {
    // Pipeline running logic
}
```

### Performance Optimization Features
Zero-copy Design
DataChunk passed between components using references or clones

Processors operate directly on original data

Asynchronous Pipeline
Parallel execution of stages

Non-blocking I/O operations

Memory Efficiency
Using Vec<u8> for binary data storage

Timely release of processed data

Concurrency Safety
All components implement Send + Sync

Using Arc for safe sharing

## Extension Guide
### Adding New Sources
Create new file in sources/ directory

Implement Source trait

Export in sources/mod.rs

Re-export in lib.rs

### Adding New Processors
Create new file in processors/ directory

Implement Processor trait

Return appropriate ProcessResult

Export in processors/mod.rs
### Adding New Sinks
Create new file in sinks/ directory

Implement Sink trait

Export in sinks/mod.rs

## 配置示例
YAML配置示例

```
yaml
name: "Network Monitoring Pipeline"
description: "Real-time network packet analysis and threat detection"

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

# Runtime Features
## Startup Flow
```
text
1. Parse configuration -> 2. Build components -> 3. Connect pipeline -> 
4. Start async tasks -> 5. Monitor operation -> 6. Graceful shutdown
```

## Monitoring Metrics

```
text
- System level: CPU/Memory usage
- Pipeline level: Throughput/Latency
- Component level: Processing volume/Error rate
```

## Best Practices
### 1. Component Design Principles
Single responsibility: Each component does one thing

Stateless design: Design as stateless processors whenever possible

Clear interfaces: Clear input/output types

### 2. Error Handling Principles
Component-level errors don't affect overall pipeline

Critical errors trigger graceful shutdown

Detailed logging for troubleshooting

### 3. Performance Optimization Principles
Avoid data copying, use reference passing

Use async I/O operations

Set appropriate channel buffer sizes
## Troubleshooting
Common Issues and Solutions
Memory Leaks

Check if DataChunk is properly released

Monitor channel queue lengths
## Performance Bottlenecks

Use MetricsSink to view performance metrics

Check processor chain length and complexity

## Deadlock Risks

Avoid blocking operations in processors

Use tokio::time::timeout for timeouts

# Project Structure Mapping
## Physical Structure

```
text
src/
├── lib.rs                    # Public interface and core types
├── pipeline.rs              # Pipeline implementation
├── processors/              # Processor module
│   ├── mod.rs              # Processor base classes and exports
│   ├── packet_processor.rs # Network processors
│   └── security_processor.rs # Security processors
├── sources/                 # Source module
│   ├── mod.rs              # Source base classes and exports
│   └── packet_source.rs    # Network sources
├── sinks/                  # Sink module
│   ├── mod.rs              # Sink base classes and exports
│   ├── advanced_sinks.rs   # Advanced sinks
│   └── security_sink.rs    # Security sinks
├── metrics/                # Metrics module
│   ├── mod.rs              # Basic metrics collector
│   └── advanced.rs         # Advanced metrics collector
├── ratelimit/              # Rate limiting module
│   └── mod.rs
├── security/               # Security module
│   └── mod.rs
└── config/                 # Configuration module
    └── mod.rs
```

## Logical Dependencies

```
text
          lib.rs (entry point)
            │
            ├── pipeline (pipeline coordination)
            │     ├── sources (data input)
            │     ├── processors (data processing)
            │     └── sinks (data output)
            │
            ├── metrics (monitoring)
            ├── ratelimit (traffic control)
            ├── security (security)
            └── config (configuration)
```

# Design Pattern Summary
Builder Pattern: PipelineBuilder

Strategy Pattern: Pluggable Processor implementations

Observer Pattern: Multiple Sinks receiving data simultaneously

Pipeline-Filter Pattern: Overall architecture pattern

Decorator Pattern: Processor chain composition

## Security Considerations
Production Environment Recommendations
Encryption module: Replace XOR encryption with AES-256-GCM

Key management: Use secure key management systems

Integrity verification: Use HMAC instead of simple hashing

Access control: Add component-level permission control
## Performance Tuning Recommendations
Buffer sizes: Adjust channel buffers based on data volume

Concurrency: Adjust task count based on CPU cores

Batch processing: Batch process large numbers of small packets

Memory pools: Implement object pools for DataChunk

## Future Expansion Directions
Short-term Goals
Add more built-in components

Complete configuration system

Add performance benchmark suite

Long-term Goals
Distributed processing support

WebAssembly integration

Visual monitoring interface

Plugin system support


Project Maintainers: Ensure all new components follow existing design patterns, maintain API consistency, and update this documentation when adding new features.

