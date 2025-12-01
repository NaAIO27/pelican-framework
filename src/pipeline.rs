// 确保这些结构体是pub的
pub struct PipelineBuilder {
    sources: Vec<Box<dyn crate::Source>>,
    processors: Vec<Box<dyn crate::Processor>>,
    sinks: Vec<Box<dyn crate::Sink>>,
}

pub struct Pipeline {
    sources: Vec<Box<dyn crate::Source>>,
    processors: Vec<Box<dyn crate::Processor>>,
    sinks: Vec<Box<dyn crate::Sink>>,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            processors: Vec::new(),
            sinks: Vec::new(),
        }
    }

    pub fn add_source(mut self, source: Box<dyn crate::Source>) -> Self {
        self.sources.push(source);
        self
    }

    pub fn add_processor(mut self, processor: Box<dyn crate::Processor>) -> Self {
        self.processors.push(processor);
        self
    }

    pub fn add_sink(mut self, sink: Box<dyn crate::Sink>) -> Self {
        self.sinks.push(sink);
        self
    }

    pub fn build(self) -> Pipeline {
        Pipeline {
            sources: self.sources,
            processors: self.processors,
            sinks: self.sinks,
        }
    }
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder::new()
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        use tokio::sync::mpsc;
        use tracing::{info, error, debug};

        info!("Starting Pelican Pipeline with {} sources, {} processors, {} sinks",
              self.sources.len(), self.processors.len(), self.sinks.len());

        if self.sources.is_empty() {
            return Err(anyhow::anyhow!("No sources configured"));
        }

        // 创建通道连接各个处理阶段
        let (source_tx, mut processor_rx) = mpsc::channel::<crate::DataChunk>(1000);

        // 启动数据源 - 转移所有权到异步任务中
        let mut source_handles = vec![];
        for mut source in self.sources {
            let tx = source_tx.clone();
            let source_name = source.name().to_string();

            let handle = tokio::spawn(async move {
                info!("Starting source: {}", source_name);
                if let Err(e) = source.stream_data(tx).await {
                    error!("Source {} error: {}", source_name, e);
                }
                info!("Source {} finished", source_name);
            });
            source_handles.push(handle);
        }

        // 释放source_tx，这样当所有源完成时，processor_rx.recv()会返回None
        drop(source_tx);

        // 处理数据流
        let mut processed_count = 0;
        let mut skipped_count = 0;

        while let Some(mut chunk) = processor_rx.recv().await {
            processed_count += 1;
            let mut should_process = true;

            // 应用所有处理器
            for processor in &mut self.processors {
                if !should_process {
                    break;
                }

                match processor.process(&mut chunk).await {
                    Ok(crate::processors::ProcessResult::Continue) => {
                        // 继续处理
                    }
                    Ok(crate::processors::ProcessResult::Skip) => {
                        should_process = false;
                        skipped_count += 1;
                        debug!("Skipping chunk {} due to processor {}", chunk.sequence, processor.name());
                    }
                    Ok(crate::processors::ProcessResult::Stop) => {
                        info!("Processing stopped by processor {}", processor.name());
                        break;
                    }
                    Err(e) => {
                        error!("Processor {} error: {}", processor.name(), e);
                        should_process = false;
                    }
                }
            }

            // 只有需要处理的数据才发送到输出端
            if should_process {
                for sink in &mut self.sinks {
                    if let Err(e) = sink.send(chunk.clone()).await {
                        error!("Sink {} error: {}", sink.name(), e);
                    }
                }
            }

            if processed_count % 10 == 0 {
                debug!("Processed {} chunks, skipped {}", processed_count, skipped_count);
            }
        }

        // 等待所有源完成
        for handle in source_handles {
            let _ = handle.await;
        }

        info!("Pelican Pipeline stopped. Total: {} processed, {} skipped", processed_count, skipped_count);
        Ok(())
    }
}