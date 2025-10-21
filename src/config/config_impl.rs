//! Configuration implementation for DREAS
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: August 2025 

use super::AppConfig;
use crate::{DreasError, DreasResult};
use config::{Config, ConfigError, File, FileFormat};

impl AppConfig {
    /// Load configuration from TOML file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> DreasResult<Self> {
        let path = path.as_ref();
        
        let config = Config::builder()
            .add_source(File::new(path.to_str().unwrap(), FileFormat::Toml))
            .build()
            .map_err(|e| DreasError::Configuration(format!("Failed to load config: {}", e)))?;
        
        config
            .try_deserialize()
            .map_err(|e| DreasError::Configuration(format!("Failed to deserialize config: {}", e)))
    }
    
    /// Create default configuration
    pub fn default() -> Self {
        Self {
            gcp: super::GcpConfig {
                project_id: "your-project-id".to_string(),
                kms_key_uri: "projects/../cryptoKeys/../cryptoKeyVersions/1".to_string(),
                location: "us-central1".to_string(),
                service_account_key_path: None,
            },
            security: super::SecurityConfig {
                enable_audit_logging: true,
                enable_key_escrow: true,
                audit_log_retention_days: 365,
            },
            api_port: 8080,
            log_level: "info".to_string(),
        }
    }
    
    /// Validate configuration
    pub fn validate(&self) -> DreasResult<()> {
        if self.gcp.project_id.is_empty() {
            return Err(DreasError::Configuration("GCP project ID cannot be empty".to_string()));
        }
        
        if self.gcp.kms_key_uri.is_empty() {
            return Err(DreasError::Configuration("KMS key URI cannot be empty".to_string()));
        }
        
        if self.api_port == 0 {
            return Err(DreasError::Configuration("API port must be greater than 0".to_string()));
        }
        
        Ok(())
    }
}
