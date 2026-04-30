use thiserror::Error;

pub type Result<T> = std::result::Result<T, ScenarioError>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ScenarioError {
    #[error("Entity '{name}' already exists{}", .existing_location.as_ref().map(|l| format!(" (defined at {})", l)).unwrap_or_default())]
    EntityConflict {
        name: String,
        existing_location: Option<String>,
    },
    
    #[error("Story '{name}' not found. Available stories: {available:?}")]
    StoryNotFound {
        name: String,
        available: Vec<String>,
    },
    
    #[error("Entity '{entity}' not found (referenced by {context})")]
    EntityNotFound {
        entity: String,
        context: String,
    },
    
    #[error("Feature '{feature}' requires OpenSCENARIO {required_version}+, but scenario is version {current_version}")]
    VersionMismatch {
        feature: String,
        required_version: String,
        current_version: String,
    },
    
    #[error("XSD validation failed: {message}")]
    XsdViolation {
        message: String,
    },
    
    #[error("Road '{road_id}' not found in loaded OpenDRIVE network. Load road network first with load_road_network()")]
    RoadNotFound {
        road_id: String,
    },
    
    #[error("Catalog '{path}' not found or could not be loaded: {reason}")]
    CatalogLoadError {
        path: String,
        reason: String,
    },
    
    #[error("Invalid catalog reference: catalog '{catalog}' entry '{entry}' not found")]
    CatalogEntryNotFound {
        catalog: String,
        entry: String,
    },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),
}
