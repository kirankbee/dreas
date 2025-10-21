//! Agentic AI coordination and management
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: August 2025
//! 
//! This module provides the core agentic AI functionality for DREAS,
//! including prompt management, response handling, and agent coordination.

pub mod coordinator;
pub mod prompt_agent;
pub mod response_agent;
pub mod shared;

pub use coordinator::AgentCoordinator;
pub use prompt_agent::PromptAgent;
pub use response_agent::ResponseAgent;
