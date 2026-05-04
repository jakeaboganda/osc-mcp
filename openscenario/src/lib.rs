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

pub use catalog::{Catalog, CatalogEntry, CatalogType};
pub use entities::{Entity, MiscObjectParams, PedestrianParams, VehicleParams};
pub use error::{Result, ScenarioError};
pub use position::Position;
pub use scenario::{ParameterDeclaration, ParameterType, Scenario};
pub use storyboard::{
    Act, Action, ByValueCondition, ComparisonRule, Condition, ConditionEdge, ConditionGroup,
    ConditionKind, Event, LaneChangeAction, SpeedAction, Storyboard, TransitionShape, Trigger,
};
pub use validation::{ValidationReport, XsdValidator};
pub use version::OpenScenarioVersion;
