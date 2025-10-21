//! DREAS - DeepRockEncryptionAsService
//! 
//! Enterprise-Grade Agentic AI Security Framework
//! Built with Rust, Secured by Google Cloud KMS & HSM, Multi-Language Ready
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: August 2025
//! 
//! This library provides enterprise-grade encryption and governance for agentic AI systems,
//! leveraging Google Cloud KMS with HSM-backed keys to ensure all agent prompts, user responses,
//! and sensitive artifacts are encrypted at all stages.

pub mod agents;
pub mod security;
pub mod services;
pub mod config;
pub mod error;

pub use error::{DreasError, DreasResult};

/// Version information for the DREAS framework
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
