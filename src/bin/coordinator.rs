//! Coordinator Service Binary
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: September 2025
//! 
//! Main entry point for the DREAS Agent Coordinator service

use dreas::{
    config::AppConfig,
    agents::{AgentCoordinator, PromptAgent, ResponseAgent, shared::AgentContext},
};
use std::env;
use tracing::{info, error};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting DREAS Agent Coordinator");
    
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
    
    // Create agent coordinator
    let (coordinator, receiver) = AgentCoordinator::new();
    
    // Create sample agents
    let session_id = Uuid::new_v4();
    let context = AgentContext::new(session_id, config.gcp.kms_key_uri.clone());
    
    let prompt_agent = PromptAgent::new(context.clone());
    let response_agent = ResponseAgent::new(context);
    
    // Register agents
    let prompt_agent_id = coordinator.register_prompt_agent(prompt_agent).await?;
    let response_agent_id = coordinator.register_response_agent(response_agent).await?;
    
    info!("Registered agents - Prompt: {}, Response: {}", prompt_agent_id, response_agent_id);
    
    // Start the coordinator event loop in a separate task
    let coordinator_clone = coordinator.clone();
    tokio::spawn(async move {
        coordinator_clone.start_event_loop(receiver).await;
    });
    
    info!("DREAS Agent Coordinator started successfully");
    
    // Example: Process some sample data
    let sample_prompt = "Hello, this is a test prompt for the DREAS system.";
    let sample_response = "This is a sample response from the LLM.";
    
    // Process prompt
    match coordinator.process_prompt(prompt_agent_id, sample_prompt.to_string()).await {
        Ok(result) => info!("Prompt processing result: {}", result),
        Err(e) => error!("Failed to process prompt: {}", e),
    }
    
    // Process response
    match coordinator.process_response(response_agent_id, sample_response.to_string()).await {
        Ok(result) => info!("Response processing result: {}", result),
        Err(e) => error!("Failed to process response: {}", e),
    }
    
    // Keep the coordinator running
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received");
    
    info!("DREAS Agent Coordinator stopped");
    Ok(())
}
