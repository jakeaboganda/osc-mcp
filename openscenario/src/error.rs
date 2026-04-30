//! Error types for OpenSCENARIO library

use thiserror::Error;

/// Result type for OpenSCENARIO operations
pub type Result<T> = std::result::Result<T, ScenarioError>;

/// Errors that can occur during scenario operations
#[derive(Debug, Error)]
pub enum ScenarioError {
    #[error("Not implemented yet")]
    NotImplemented,
}
