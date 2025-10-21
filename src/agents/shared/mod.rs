//! Shared types and utilities for agents
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: September 2025
//! 
//! This module provides shared types, utilities, and common functionality
//! used across different agent types in the DREAS framework.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Context information shared between agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    pub session_id: Uuid,
    pub user_id: Option<String>,
    pub metadata: HashMap<String, String>,
    pub encryption_key_id: String,
}

/// Agent status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentStatus {
    Initializing,
    Ready,
    Processing,
    Error,
    Shutdown,
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

impl AgentContext {
    /// Create a new agent context
    pub fn new(session_id: Uuid, encryption_key_id: String) -> Self {
        Self {
            session_id,
            user_id: None,
            metadata: HashMap::new(),
            encryption_key_id,
        }
    }
    
    /// Set user ID
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            name: "default_agent".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["encryption".to_string(), "decryption".to_string()],
            timeout_seconds: 30,
            max_retries: 3,
        }
    }
}
