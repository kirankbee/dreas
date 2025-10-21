//! Audit logging and compliance tracking
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: September 2025
//! 
//! This module provides comprehensive audit logging for compliance and security
//! monitoring, tracking all operations within the DREAS framework.

use crate::{DreasResult, DreasError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Audit logger for tracking all system operations
#[derive(Debug, Clone)]
pub struct AuditLogger {
    log_id: Uuid,
    retention_days: u32,
    audit_entries: Vec<AuditEntry>,
    sensitive_operations: Vec<String>,
}

/// Individual audit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub entry_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub action: String,
    pub resource: String,
    pub result: AuditResult,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Audit result enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditResult {
    Success,
    Failure,
    Partial,
}

/// Audit query parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQuery {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub resource: Option<String>,
    pub result: Option<AuditResult>,
    pub limit: Option<usize>,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(retention_days: u32) -> Self {
        Self {
            log_id: Uuid::new_v4(),
            retention_days,
            audit_entries: Vec::new(),
            sensitive_operations: vec![
                "key_escrow".to_string(),
                "key_recovery".to_string(),
                "user_authentication".to_string(),
                "permission_change".to_string(),
                "data_encryption".to_string(),
                "data_decryption".to_string(),
            ],
        }
    }
    
    /// Log an audit entry
    pub async fn log_operation(
        &mut self,
        user_id: Option<String>,
        session_id: Option<String>,
        action: String,
        resource: String,
        result: AuditResult,
        metadata: Option<HashMap<String, String>>,
    ) -> DreasResult<Uuid> {
        let entry_id = Uuid::new_v4();
        
        let entry = AuditEntry {
            entry_id,
            timestamp: Utc::now(),
            user_id,
            session_id,
            action: action.clone(),
            resource,
            result: result.clone(),
            ip_address: None, // TODO: Extract from request context
            user_agent: None, // TODO: Extract from request context
            metadata: metadata.unwrap_or_default(),
        };
        
        // Store the audit entry
        self.audit_entries.push(entry.clone());
        
        // Log to tracing for immediate visibility
        let log_level = match result {
            AuditResult::Success => tracing::Level::INFO,
            AuditResult::Failure => tracing::Level::ERROR,
            AuditResult::Partial => tracing::Level::WARN,
        };
        
        let log_message = serde_json::json!({
            "audit_id": self.log_id,
            "entry_id": entry_id,
            "timestamp": entry.timestamp,
            "user_id": entry.user_id,
            "session_id": entry.session_id,
            "action": entry.action,
            "resource": entry.resource,
            "result": entry.result,
            "metadata": entry.metadata
        });
        
        tracing::event!(log_level, "{}", log_message);
        
        // If this is a sensitive operation, log additional details
        if self.sensitive_operations.contains(&action) {
            tracing::warn!("Sensitive operation detected: {}", action);
        }
        
        Ok(entry_id)
    }
    
    /// Query audit entries
    pub fn query_audit_entries(&self, query: AuditQuery) -> DreasResult<Vec<AuditEntry>> {
        let mut results = self.audit_entries.clone();
        
        // Apply filters
        if let Some(start_date) = query.start_date {
            results.retain(|entry| entry.timestamp >= start_date);
        }
        
        if let Some(end_date) = query.end_date {
            results.retain(|entry| entry.timestamp <= end_date);
        }
        
        if let Some(user_id) = query.user_id {
            results.retain(|entry| entry.user_id.as_ref() == Some(&user_id));
        }
        
        if let Some(action) = query.action {
            results.retain(|entry| entry.action == action);
        }
        
        if let Some(resource) = query.resource {
            results.retain(|entry| entry.resource == resource);
        }
        
        if let Some(result) = query.result {
            results.retain(|entry| entry.result == result);
        }
        
        // Sort by timestamp (newest first)
        results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }
        
        Ok(results)
    }
    
    /// Generate audit report
    pub fn generate_report(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> DreasResult<serde_json::Value> {
        let entries = self.query_audit_entries(AuditQuery {
            start_date: Some(start_date),
            end_date: Some(end_date),
            user_id: None,
            action: None,
            resource: None,
            result: None,
            limit: None,
        })?;
        
        let total_operations = entries.len();
        let successful_operations = entries.iter().filter(|e| e.result == AuditResult::Success).count();
        let failed_operations = entries.iter().filter(|e| e.result == AuditResult::Failure).count();
        let partial_operations = entries.iter().filter(|e| e.result == AuditResult::Partial).count();
        
        let mut action_counts = HashMap::new();
        let mut user_counts = HashMap::new();
        
        for entry in &entries {
            *action_counts.entry(entry.action.clone()).or_insert(0) += 1;
            if let Some(user_id) = &entry.user_id {
                *user_counts.entry(user_id.clone()).or_insert(0) += 1;
            }
        }
        
        Ok(serde_json::json!({
            "report_id": Uuid::new_v4(),
            "generated_at": Utc::now(),
            "period": {
                "start_date": start_date,
                "end_date": end_date
            },
            "summary": {
                "total_operations": total_operations,
                "successful_operations": successful_operations,
                "failed_operations": failed_operations,
                "partial_operations": partial_operations,
                "success_rate": if total_operations > 0 { 
                    (successful_operations as f64 / total_operations as f64) * 100.0 
                } else { 0.0 }
            },
            "action_breakdown": action_counts,
            "user_activity": user_counts,
            "audit_log_id": self.log_id
        }))
    }
    
    /// Clean up old audit entries based on retention policy
    pub fn cleanup_old_entries(&mut self) -> DreasResult<usize> {
        let cutoff_date = Utc::now() - chrono::Duration::days(self.retention_days as i64);
        let initial_count = self.audit_entries.len();
        
        self.audit_entries.retain(|entry| entry.timestamp > cutoff_date);
        
        let removed_count = initial_count - self.audit_entries.len();
        
        if removed_count > 0 {
            tracing::info!("Cleaned up {} old audit entries", removed_count);
        }
        
        Ok(removed_count)
    }
    
    /// Get audit statistics
    pub fn get_audit_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "audit_log_id": self.log_id,
            "total_entries": self.audit_entries.len(),
            "retention_days": self.retention_days,
            "sensitive_operations_tracked": self.sensitive_operations.len(),
            "oldest_entry": self.audit_entries.iter().map(|e| e.timestamp).min(),
            "newest_entry": self.audit_entries.iter().map(|e| e.timestamp).max()
        })
    }
}
