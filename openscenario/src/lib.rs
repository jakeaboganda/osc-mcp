//! OpenSCENARIO library for scenario generation and validation

pub mod catalog;
pub mod entities;
pub mod error;
pub mod opendrive_validator;
pub mod position;
pub mod scenario;
pub mod storyboard;
pub mod validation;
pub mod version;
pub mod xml;

pub use catalog::{Catalog, CatalogType, CatalogEntry};
pub use entities::{Entity, VehicleParams, PedestrianParams, MiscObjectParams};
pub use error::{ScenarioError, Result};
pub use position::Position;
pub use scenario::Scenario;
pub use storyboard::{Storyboard, TransitionShape, Action, SpeedAction, LaneChangeAction};
pub use validation::{ValidationReport, XsdValidator};
pub use version::OpenScenarioVersion;
