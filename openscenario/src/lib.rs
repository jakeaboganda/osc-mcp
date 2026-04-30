//! OpenSCENARIO library for scenario generation and validation

pub mod catalog;
pub mod entities;
pub mod error;
pub mod position;
pub mod scenario;
pub mod storyboard;
pub mod version;
pub mod xml;

pub use catalog::{Catalog, CatalogType, CatalogEntry};
pub use entities::{Entity, VehicleParams, PedestrianParams, MiscObjectParams};
pub use error::{ScenarioError, Result};
pub use position::Position;
pub use scenario::Scenario;
pub use storyboard::{Storyboard, TransitionShape, Action, SpeedAction, LaneChangeAction};
pub use version::OpenScenarioVersion;
