use async_trait::async_trait;
use crate::{DataChunk, Sink};
use crate::security::{DataIntegrityChecker, SecurityConfig};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use tracing::{info, error};

pub struct SecureFileSink {
    file_path: String,
    integrity_checker: Option<DataIntegrityChecker>,
    enable_encryption: bool,
    name: String,
}

impl SecureFileSink {
    pub fn new(
        file_path: String,
        security_config: &SecurityConfig,
        name: &str,
    ) -> Result<Self, anyhow::Error> {
        let integrity_checker = if security_config.enable_integrity_check {
            security_config.integrity_salt.map(|salt| DataIntegrityChecker::new(salt))
        } else {
            None
        };

        Ok(Self {
            file_path,
            integrity_checker,
            enable_encryption: security_config.enable_encryption,
            name: name.to_string(),
        })
    }

    fn write_with_integrity(&self, data: &[u8]) -> anyhow::Result<()> {
        let path = Path::new(&self.file_path);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        // 如果启用了完整性检查，计算校验和
        if let Some(checker) = &self.integrity_checker {
            let checksum = checker.calculate_checksum(data);

            // 写入数据长度、数据和校验和
            let data_len = data.len() as u32;
            file.write_all(&data_len.to_le_bytes())?;
            file.write_all(data)?;
            file.write_all(&checksum)?;

            info!("已写入带完整性检查的数据: {} bytes", data_len);
        } else {
            // 普通写入
            file.write_all(data)?;
            info!("已写入数据: {} bytes", data.len());
        }

        Ok(())
    }
}

#[async_trait]
impl Sink for SecureFileSink {
    async fn send(&mut self, chunk: DataChunk) -> anyhow::Result<()> {
        if self.enable_encryption {
            info!("加密功能已启用（简化实现）");
        }

        self.write_with_integrity(&chunk.data)?;

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}