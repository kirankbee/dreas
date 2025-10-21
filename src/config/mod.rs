//! Configuration management for DREAS
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: August 2025

use serde::{Deserialize, Serialize};
use std::path::Path;

mod config_impl;

pub use config_impl::Config;

/// GCP configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpConfig {
    pub project_id: String,
    pub kms_key_uri: String,
    pub location: String,
    pub service_account_key_path: Option<String>,
}

/// Security configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_audit_logging: bool,
    pub enable_key_escrow: bool,
    pub audit_log_retention_days: u32,
}

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub gcp: GcpConfig,
    pub security: SecurityConfig,
    pub api_port: u16,
    pub log_level: String,
}
