use async_trait::async_trait;
use crate::DataChunk;
use crate::processors::{Processor, ProcessResult};
use crate::security::DataEncryptor;
use tracing::error;

pub struct EncryptionProcessor {
    encryptor: DataEncryptor,
    name: String,
}

impl EncryptionProcessor {
    pub fn new(key: &[u8], name: &str) -> Result<Self, crate::security::SecurityError> {
        let encryptor = DataEncryptor::new(key)?;
        Ok(Self {
            encryptor,
            name: name.to_string(),
        })
    }
}

#[async_trait]
impl Processor for EncryptionProcessor {
    async fn process(&mut self, chunk: &mut DataChunk) -> anyhow::Result<ProcessResult> {
        match self.encryptor.encrypt(&chunk.data) {
            Ok(encrypted_data) => {
                chunk.data = encrypted_data;
                Ok(ProcessResult::Continue)
            }
            Err(e) => {
                error!("加密失败: {}", e);
                Err(anyhow::anyhow!("加密失败: {}", e))
            }
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub struct DecryptionProcessor {
    encryptor: DataEncryptor,
    name: String,
}

impl DecryptionProcessor {
    pub fn new(key: &[u8], name: &str) -> Result<Self, crate::security::SecurityError> {
        let encryptor = DataEncryptor::new(key)?;
        Ok(Self {
            encryptor,
            name: name.to_string(),
        })
    }
}

#[async_trait]
impl Processor for DecryptionProcessor {
    async fn process(&mut self, chunk: &mut DataChunk) -> anyhow::Result<ProcessResult> {
        match self.encryptor.decrypt(&chunk.data) {
            Ok(decrypted_data) => {
                chunk.data = decrypted_data;
                Ok(ProcessResult::Continue)
            }
            Err(e) => {
                error!("解密失败: {}", e);
                Err(anyhow::anyhow!("解密失败: {}", e))
            }
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}