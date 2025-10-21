//! Agent coordination and orchestration
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: August 2025
//! 
//! The AgentCoordinator manages the lifecycle and coordination of multiple agents
//! within the DREAS framework, ensuring secure communication and proper encryption.

use crate::{DreasResult, DreasError};
use super::{PromptAgent, ResponseAgent};
use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

/// Agent coordination manager
#[derive(Debug)]
pub struct AgentCoordinator {
    prompt_agents: RwLock<HashMap<Uuid, PromptAgent>>,
    response_agents: RwLock<HashMap<Uuid, ResponseAgent>>,
    command_channel: mpsc::UnboundedSender<CoordinatorCommand>,
}

/// Commands that can be sent to the coordinator
#[derive(Debug, Clone)]
pub enum CoordinatorCommand {
    RegisterPromptAgent { id: Uuid, agent: PromptAgent },
    RegisterResponseAgent { id: Uuid, agent: ResponseAgent },
    ProcessPrompt { agent_id: Uuid, prompt: String },
    ProcessResponse { agent_id: Uuid, response: String },
    Shutdown,
}

impl AgentCoordinator {
    /// Create a new agent coordinator
    pub fn new() -> (Self, mpsc::UnboundedReceiver<CoordinatorCommand>) {
        let (tx, rx) = mpsc::unbounded_channel();
        
        let coordinator = Self {
            prompt_agents: RwLock::new(HashMap::new()),
            response_agents: RwLock::new(HashMap::new()),
            command_channel: tx,
        };
        
        (coordinator, rx)
    }
    
    /// Register a prompt agent
    pub async fn register_prompt_agent(&self, agent: PromptAgent) -> DreasResult<Uuid> {
        let id = Uuid::new_v4();
        
        self.command_channel
            .send(CoordinatorCommand::RegisterPromptAgent { id, agent })
            .map_err(|_| DreasError::AgentCoordination("Failed to send registration command".to_string()))?;
        
        Ok(id)
    }
    
    /// Register a response agent
    pub async fn register_response_agent(&self, agent: ResponseAgent) -> DreasResult<Uuid> {
        let id = Uuid::new_v4();
        
        self.command_channel
            .send(CoordinatorCommand::RegisterResponseAgent { id, agent })
            .map_err(|_| DreasError::AgentCoordination("Failed to send registration command".to_string()))?;
        
        Ok(id)
    }
    
    /// Process a prompt through the appropriate agent
    pub async fn process_prompt(&self, agent_id: Uuid, prompt: String) -> DreasResult<String> {
        let prompt_agents = self.prompt_agents.read().await;
        
        if let Some(agent) = prompt_agents.get(&agent_id) {
            agent.process_prompt(prompt).await
        } else {
            Err(DreasError::AgentCoordination(format!("Prompt agent {} not found", agent_id)))
        }
    }
    
    /// Process a response through the appropriate agent
    pub async fn process_response(&self, agent_id: Uuid, response: String) -> DreasResult<String> {
        let response_agents = self.response_agents.read().await;
        
        if let Some(agent) = response_agents.get(&agent_id) {
            agent.process_response(response).await
        } else {
            Err(DreasError::AgentCoordination(format!("Response agent {} not found", agent_id)))
        }
    }
    
    /// Start the coordinator's event loop
    pub async fn start_event_loop(&self, mut receiver: mpsc::UnboundedReceiver<CoordinatorCommand>) {
        while let Some(command) = receiver.recv().await {
            match command {
                CoordinatorCommand::RegisterPromptAgent { id, agent } => {
                    let mut prompt_agents = self.prompt_agents.write().await;
                    prompt_agents.insert(id, agent);
                }
                CoordinatorCommand::RegisterResponseAgent { id, agent } => {
                    let mut response_agents = self.response_agents.write().await;
                    response_agents.insert(id, agent);
                }
                CoordinatorCommand::ProcessPrompt { agent_id, prompt } => {
                    if let Err(e) = self.process_prompt(agent_id, prompt).await {
                        tracing::error!("Failed to process prompt: {}", e);
                    }
                }
                CoordinatorCommand::ProcessResponse { agent_id, response } => {
                    if let Err(e) = self.process_response(agent_id, response).await {
                        tracing::error!("Failed to process response: {}", e);
                    }
                }
                CoordinatorCommand::Shutdown => {
                    tracing::info!("Shutting down agent coordinator");
                    break;
                }
            }
        }
    }
}
