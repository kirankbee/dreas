//! API Service Binary
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: 2024-12-19
//! 
//! Main entry point for the DREAS API service

use dreas::{config::AppConfig, services::ApiService};
use std::env;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting DREAS API Service");
    
    // Load configuration
    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "config/config.toml".to_string());
    
    let config = match AppConfig::from_file(&config_path) {
        Ok(config) => {
            info!("Configuration loaded from: {}", config_path);
            config
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            info!("Using default configuration");
            AppConfig::default()
        }
    };
    
    // Validate configuration
    if let Err(e) = config.validate() {
        error!("Configuration validation failed: {}", e);
        return Err(e.into());
    }
    
    // Create API service
    let mut api_service = ApiService::new(config.api_port);
    
    // Register default endpoints
    register_default_endpoints(&mut api_service).await?;
    
    // Start the API server
    info!("Starting API server on port {}", config.api_port);
    
    if let Err(e) = api_service.start_server().await {
        error!("Failed to start API server: {}", e);
        return Err(e.into());
    }
    
    info!("DREAS API Service started successfully");
    
    // Keep the service running
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received");
    
    // Stop the API server
    if let Err(e) = api_service.stop_server().await {
        error!("Failed to stop API server: {}", e);
    }
    
    info!("DREAS API Service stopped");
    Ok(())
}

async fn register_default_endpoints(api_service: &mut ApiService) -> Result<(), Box<dyn std::error::Error>> {
    use dreas::services::api::{ApiEndpoint, HttpMethod};
    
    // Health check endpoint
    let health_endpoint = ApiEndpoint {
        path: "/health".to_string(),
        method: HttpMethod::GET,
        handler: "health_check".to_string(),
        requires_auth: false,
        rate_limit: Some(100),
        timeout_seconds: Some(5),
    };
    
    api_service.register_endpoint(health_endpoint).await?;
    
    // Stats endpoint
    let stats_endpoint = ApiEndpoint {
        path: "/stats".to_string(),
        method: HttpMethod::GET,
        handler: "get_stats".to_string(),
        requires_auth: true,
        rate_limit: Some(10),
        timeout_seconds: Some(30),
    };
    
    api_service.register_endpoint(stats_endpoint).await?;
    
    Ok(())
}
