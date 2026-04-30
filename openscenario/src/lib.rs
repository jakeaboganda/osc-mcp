//! OpenSCENARIO library for scenario generation and validation

pub mod error;
pub mod position;
pub mod scenario;
pub mod version;

pub use error::{ScenarioError, Result};
pub use position::Position;
pub use scenario::Scenario;
pub use version::OpenScenarioVersion;
