use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("加密失败: {0}")]
    EncryptionError(String),
    #[error("解密失败: {0}")]
    DecryptionError(String),
    #[error("密钥错误: {0}")]
    KeyError(String),
    #[error("完整性检查失败: {0}")]
    IntegrityError(String),
}

// 简化版加密器 - 使用简单的XOR加密
pub struct DataEncryptor {
    key: Arc<[u8]>,
}

impl DataEncryptor {
    pub fn new(key: &[u8]) -> Result<Self, SecurityError> {
        if key.len() < 16 {
            return Err(SecurityError::KeyError("密钥长度至少16字节".to_string()));
        }

        Ok(Self {
            key: Arc::from(key),
        })
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, SecurityError> {
        let mut result = Vec::with_capacity(data.len() + 4);

        // 添加数据长度前缀
        let len = data.len() as u32;
        result.extend_from_slice(&len.to_le_bytes());

        // XOR加密
        for (i, &byte) in data.iter().enumerate() {
            let key_byte = self.key[i % self.key.len()];
            result.push(byte ^ key_byte);
        }

        Ok(result)
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, SecurityError> {
        if encrypted_data.len() < 4 {
            return Err(SecurityError::DecryptionError("数据太短".to_string()));
        }

        // 解析长度
        let len_bytes = &encrypted_data[0..4];
        let expected_len = u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;

        let data = &encrypted_data[4..];
        if data.len() != expected_len {
            return Err(SecurityError::DecryptionError("数据长度不匹配".to_string()));
        }

        // XOR解密
        let mut result = Vec::with_capacity(expected_len);
        for (i, &byte) in data.iter().enumerate() {
            let key_byte = self.key[i % self.key.len()];
            result.push(byte ^ key_byte);
        }

        Ok(result)
    }
}

// 简化版完整性检查器
pub struct DataIntegrityChecker {
    salt: [u8; 8],
}

impl DataIntegrityChecker {
    pub fn new(salt: [u8; 8]) -> Self {
        Self { salt }
    }

    pub fn calculate_checksum(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        self.salt.hash(&mut hasher);

        let hash = hasher.finish();
        hash.to_le_bytes().to_vec()
    }

    pub fn verify_checksum(&self, data: &[u8], expected_checksum: &[u8]) -> Result<(), SecurityError> {
        let calculated = self.calculate_checksum(data);
        if calculated != expected_checksum {
            return Err(SecurityError::IntegrityError("数据完整性检查失败".to_string()));
        }
        Ok(())
    }
}

// 安全配置
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub enable_encryption: bool,
    pub enable_integrity_check: bool,
    pub encryption_key: Option<Vec<u8>>,
    pub integrity_salt: Option<[u8; 8]>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_encryption: false,
            enable_integrity_check: true,
            encryption_key: None,
            integrity_salt: Some([0; 8]),
        }
    }
}