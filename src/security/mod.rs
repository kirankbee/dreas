//! Security module for DREAS
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: 2024-12-19
//! 
//! This module provides comprehensive security functionality including KMS integration,
//! key escrow, identity management, and audit logging for the DREAS framework.

pub mod kms;
pub mod escrow;
pub mod identity;
pub mod audit;

pub use kms::KmsClient;
pub use escrow::KeyEscrow;
pub use identity::IdentityManager;
pub use audit::AuditLogger;
