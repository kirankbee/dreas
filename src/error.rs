//! Error handling for DREAS framework
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: 2024-12-19

use thiserror::Error;

/// Main error type for DREAS operations
#[derive(Error, Debug)]
pub enum DreasError {
    #[error("KMS encryption error: {0}")]
    KmsEncryption(String),
    
    #[error("KMS decryption error: {0}")]
    KmsDecryption(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Agent coordination error: {0}")]
    AgentCoordination(String),
    
    #[error("Audit logging error: {0}")]
    AuditLogging(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Generic error: {0}")]
    Generic(String),
}

/// Result type alias for DREAS operations
pub type DreasResult<T> = Result<T, DreasError>;
