//! Service layer for DREAS
//! 
//! Author: Kiran Kumar Balijepalli
//! Date: 
//! 
//! This module provides the service layer functionality including storage,
//! model management, API services, and system observation for the DREAS framework.

pub mod storage;
pub mod model;
pub mod api;
pub mod observer;

pub use storage::StorageService;
pub use model::ModelService;
pub use api::ApiService;
pub use observer::ObserverService;
