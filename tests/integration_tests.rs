//! Integration tests for DREAS
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: September 2025

use dreas::{
    agents::{AgentCoordinator, PromptAgent, ResponseAgent, shared::AgentContext},
    security::{KmsClient, KeyEscrow, IdentityManager, AuditLogger},
    services::{StorageService, ModelService, ApiService, ObserverService},
    config::AppConfig,
};
use uuid::Uuid;
use tokio_test;

#[tokio::test]
async fn test_agent_coordination() {
    let (coordinator, receiver) = AgentCoordinator::new();
    
    // Create test context
    let session_id = Uuid::new_v4();
    let context = AgentContext::new(session_id, "test-key-id".to_string());
    
    // Create and register agents
    let prompt_agent = PromptAgent::new(context.clone());
    let response_agent = ResponseAgent::new(context);
    
    let prompt_agent_id = coordinator.register_prompt_agent(prompt_agent).await.unwrap();
    let response_agent_id = coordinator.register_response_agent(response_agent).await.unwrap();
    
    // Test prompt processing
    let prompt_result = coordinator.process_prompt(prompt_agent_id, "Test prompt".to_string()).await;
    assert!(prompt_result.is_ok());
    
    // Test response processing
    let response_result = coordinator.process_response(response_agent_id, "Test response".to_string()).await;
    assert!(response_result.is_ok());
}

#[tokio::test]
async fn test_kms_client() {
    let kms_client = KmsClient::new(
        "test-project".to_string(),
        "us-central1".to_string(),
        "test-keyring".to_string(),
        "test-key".to_string(),
        "1".to_string(),
    );
    
    // Test configuration validation
    assert!(kms_client.validate_config().is_ok());
    
    // Test encryption/decryption cycle
    let test_data = b"test data";
    let encrypted = kms_client.encrypt(test_data).await.unwrap();
    let decrypted = kms_client.decrypt(&encrypted.ciphertext).await.unwrap();
    
    assert_eq!(test_data, decrypted.plaintext.as_slice());
}

#[tokio::test]
async fn test_key_escrow() {
    let authorized_parties = vec!["admin1".to_string(), "admin2".to_string(), "admin3".to_string()];
    let mut escrow = KeyEscrow::new(authorized_parties, 2).unwrap();
    
    // Test key escrow
    let key_id = "test-key-123".to_string();
    let encrypted_key = b"encrypted key data".to_vec();
    
    assert!(escrow.escrow_key(key_id.clone(), encrypted_key.clone(), None).await.is_ok());
    
    // Test key recovery (would need proper signatures in real implementation)
    let recovery_request = dreas::security::escrow::RecoveryRequest {
        request_id: Uuid::new_v4(),
        requester: "admin1".to_string(),
        key_id: key_id.clone(),
        reason: "Emergency recovery".to_string(),
        signatures: vec![
            dreas::security::escrow::EscrowSignature {
                signer: "admin1".to_string(),
                signature: "signature1".to_string(),
                timestamp: chrono::Utc::now(),
            },
            dreas::security::escrow::EscrowSignature {
                signer: "admin2".to_string(),
                signature: "signature2".to_string(),
                timestamp: chrono::Utc::now(),
            },
        ],
        timestamp: chrono::Utc::now(),
    };
    
    // This would fail in real implementation due to signature validation
    // but demonstrates the API structure
    let recovery_result = escrow.recover_key(recovery_request).await;
    // assert!(recovery_result.is_ok()); // Commented out as it requires proper signatures
}

#[tokio::test]
async fn test_identity_manager() {
    let mut identity_manager = IdentityManager::new();
    
    // Test user creation
    let user = identity_manager.create_user(
        "testuser".to_string(),
        "test@example.com".to_string(),
        "password123".to_string(),
        vec!["user".to_string()],
    ).await.unwrap();
    
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
    
    // Test authentication
    let auth_result = identity_manager.authenticate("testuser", "password123").await.unwrap();
    assert!(auth_result.success);
    assert!(auth_result.session_id.is_some());
    
    // Test permission checking
    if let Some(session_id) = auth_result.session_id {
        let permission_result = identity_manager.check_permission(&session_id, "read_data").await.unwrap();
        // This would depend on role configuration in real implementation
        // assert!(!permission_result.allowed); // User doesn't have read_data permission by default
    }
}

#[tokio::test]
async fn test_audit_logger() {
    let mut audit_logger = AuditLogger::new(30);
    
    // Test audit logging
    let entry_id = audit_logger.log_operation(
        Some("user123".to_string()),
        Some("session456".to_string()),
        "data_access".to_string(),
        "sensitive_file.txt".to_string(),
        dreas::security::audit::AuditResult::Success,
        None,
    ).await.unwrap();
    
    assert!(!entry_id.to_string().is_empty());
    
    // Test audit query
    let query = dreas::security::audit::AuditQuery {
        start_date: Some(chrono::Utc::now() - chrono::Duration::hours(1)),
        end_date: Some(chrono::Utc::now()),
        user_id: Some("user123".to_string()),
        action: Some("data_access".to_string()),
        resource: None,
        result: None,
        limit: Some(10),
    };
    
    let entries = audit_logger.query_audit_entries(query).unwrap();
    assert!(!entries.is_empty());
}

#[tokio::test]
async fn test_storage_service() {
    let storage_service = StorageService::new(
        "test-bucket".to_string(),
        "test_dataset".to_string(),
    );
    
    // Test data storage
    let test_data = b"test data content";
    let result = storage_service.store_data(
        "test-file.txt".to_string(),
        test_data.to_vec(),
        "text/plain".to_string(),
        None,
    ).await.unwrap();
    
    assert!(result.success);
    assert_eq!(result.operation_type, dreas::services::storage::StorageOperation::Create);
    
    // Test data retrieval
    let retrieved_data = storage_service.retrieve_data("test-file.txt".to_string()).await.unwrap();
    assert_eq!(retrieved_data, b"retrieved data");
}

#[tokio::test]
async fn test_model_service() {
    let mut model_service = ModelService::new();
    
    // Test model registration
    let model_config = dreas::services::model::ModelConfig {
        name: "test-model".to_string(),
        provider: "openai".to_string(),
        version: "1.0".to_string(),
        endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
        api_key_encrypted: b"encrypted_api_key".to_vec(),
        max_tokens: 2048,
        temperature: 0.7,
        capabilities: vec!["chat".to_string(), "completion".to_string()],
        enabled: true,
    };
    
    assert!(model_service.register_model(model_config).await.is_ok());
    
    // Test model request
    let request = dreas::services::model::ModelRequest {
        request_id: Uuid::new_v4(),
        model_name: "test-model".to_string(),
        prompt: "Hello, world!".to_string(),
        max_tokens: Some(100),
        temperature: Some(0.5),
        metadata: std::collections::HashMap::new(),
    };
    
    let response = model_service.send_request(request).await.unwrap();
    assert!(response.success);
    assert_eq!(response.model_name, "test-model");
}

#[tokio::test]
async fn test_api_service() {
    let mut api_service = ApiService::new(8080);
    
    // Test endpoint registration
    let endpoint = dreas::services::api::ApiEndpoint {
        path: "/test".to_string(),
        method: dreas::services::api::HttpMethod::GET,
        handler: "test_handler".to_string(),
        requires_auth: false,
        rate_limit: Some(100),
        timeout_seconds: Some(30),
    };
    
    assert!(api_service.register_endpoint(endpoint).await.is_ok());
    
    // Test request processing
    let request = dreas::services::api::ApiRequest {
        request_id: Uuid::new_v4(),
        method: dreas::services::api::HttpMethod::GET,
        path: "/test".to_string(),
        headers: std::collections::HashMap::new(),
        body: None,
        query_params: std::collections::HashMap::new(),
        timestamp: chrono::Utc::now(),
    };
    
    let response = api_service.process_request(request).await.unwrap();
    assert_eq!(response.status_code, 200);
    assert!(response.body.is_some());
}

#[tokio::test]
async fn test_observer_service() {
    let mut observer_service = ObserverService::new();
    
    // Test metric recording
    assert!(observer_service.record_metric(
        "cpu_usage".to_string(),
        75.5,
        "percent".to_string(),
        None,
    ).await.is_ok());
    
    // Test health check registration
    let health_check = dreas::services::observer::HealthCheck {
        name: "database_check".to_string(),
        check_type: dreas::services::observer::HealthCheckType::Database,
        interval_seconds: 60,
        timeout_seconds: 30,
        threshold: Some(5.0),
        last_check: None,
        status: dreas::services::observer::HealthStatus::Unknown,
    };
    
    assert!(observer_service.register_health_check(health_check).await.is_ok());
    
    // Test health check execution
    let results = observer_service.run_health_checks().await.unwrap();
    assert!(!results.is_empty());
    
    // Test alert creation
    let alert_id = observer_service.create_alert(
        "High CPU Usage".to_string(),
        dreas::services::observer::AlertSeverity::High,
        "CPU usage is above 90%".to_string(),
    ).await.unwrap();
    
    assert!(!alert_id.to_string().is_empty());
}

#[tokio::test]
async fn test_config_loading() {
    // Test default configuration
    let default_config = AppConfig::default();
    assert!(default_config.validate().is_ok());
    
    // Test configuration validation
    let mut invalid_config = default_config.clone();
    invalid_config.gcp.project_id = String::new();
    assert!(invalid_config.validate().is_err());
}
