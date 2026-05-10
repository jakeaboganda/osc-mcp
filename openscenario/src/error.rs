use thiserror::Error;

/// Result type alias for operations that may return a ScenarioError.
///
/// Provides a convenient shorthand for Result<T, ScenarioError>.
///
/// # Examples
/// ```
/// use openscenario::error::Result;
///
/// fn create_scenario() -> Result<()> {
///     // scenario creation logic
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, ScenarioError>;

/// Errors that can occur during OpenSCENARIO scenario creation and validation.
///
/// This enum covers all error conditions that can arise when building, validating,
/// or processing OpenSCENARIO scenarios. Errors include entity conflicts, missing
/// references, version mismatches, and validation failures.
///
/// # Examples
/// ```
/// use openscenario::error::ScenarioError;
///
/// # fn main() {
/// let error = ScenarioError::EntityNotFound {
///     entity: "Vehicle1".to_string(),
///     context: "speed condition".to_string(),
/// };
/// assert!(error.to_string().contains("not found"));
/// # }
/// ```
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ScenarioError {
    /// Raised when attempting to add an entity with a name that already exists.
    ///
    /// Entity names must be unique within a scenario. This error includes the
    /// conflicting name and optionally the location where it was first defined.
    #[error("Entity '{name}' already exists{}", .existing_location.as_ref().map(|l| format!(" (defined at {})", l)).unwrap_or_default())]
    EntityConflict {
        name: String,
        existing_location: Option<String>,
    },

    /// Raised when referencing a Story that doesn't exist in the Storyboard.
    ///
    /// Includes the requested story name and a list of available stories for debugging.
    #[error("Story '{name}' not found. Available stories: {available:?}")]
    StoryNotFound {
        name: String,
        available: Vec<String>,
    },

    /// Raised when an entity reference in a condition or action is invalid.
    ///
    /// This occurs when referring to an entity that hasn't been added to the scenario.
    /// The error includes the context where the reference occurred.
    #[error("Entity '{entity}' not found (referenced by {context})")]
    EntityNotFound { entity: String, context: String },

    /// Raised when using a feature incompatible with the scenario's OpenSCENARIO version.
    ///
    /// OpenSCENARIO features are version-specific. This error indicates that a feature
    /// requires a newer version than the scenario's declared version.
    #[error("Feature '{feature}' requires OpenSCENARIO {required_version}+, but scenario is version {current_version}")]
    VersionMismatch {
        feature: String,
        required_version: String,
        current_version: String,
    },

    /// Raised when the scenario violates OpenSCENARIO XSD schema rules.
    ///
    /// XSD validation ensures conformance to the OpenSCENARIO standard.
    #[error("XSD validation failed: {message}")]
    XsdViolation { message: String },

    /// Raised when referencing an OpenDRIVE road that doesn't exist in the loaded network.
    ///
    /// This typically occurs when using lane or road positions before loading the road network.
    #[error("Road '{road_id}' not found in loaded OpenDRIVE network. Load road network first with load_road_network()")]
    RoadNotFound { road_id: String },

    /// Raised when a catalog file cannot be loaded.
    ///
    /// Catalog errors include the file path and a description of why loading failed.
    #[error("Catalog '{path}' not found or could not be loaded: {reason}")]
    CatalogLoadError { path: String, reason: String },

    /// Raised when a catalog entry doesn't exist in the specified catalog.
    ///
    /// This occurs when referencing a catalog entry by name that isn't defined in that catalog.
    #[error("Invalid catalog reference: catalog '{catalog}' entry '{entry}' not found")]
    CatalogEntryNotFound { catalog: String, entry: String },

    /// Raised when a catalog file is malformed or invalid.
    #[error("Invalid catalog: {0}")]
    InvalidCatalog(String),

    /// Raised when attempting to add a parameter with a name that already exists.
    ///
    /// Parameter names must be unique within a scenario.
    #[error("Parameter '{name}' already exists")]
    ParameterConflict { name: String },

    /// Raised when a parameter reference is invalid.
    ///
    /// This occurs when using a parameter that hasn't been declared.
    #[error("Invalid parameter reference: {0}")]
    InvalidParameterRef(String),

    /// Raised when an entity reference in a condition doesn't exist.
    ///
    /// Includes the invalid entity name and a list of available entities for debugging.
    #[error("Invalid entity reference '{entity}': entity not found in scenario. Available entities: {available:?}")]
    InvalidEntityRef {
        entity: String,
        available: Vec<String>,
    },

    /// Raised when a field has an invalid value.
    ///
    /// This is a general validation error for field values that don't meet requirements.
    #[error("Invalid value for {field}: {reason}")]
    InvalidValue { field: String, reason: String },

    /// Raised when a name conflict occurs within a specific context.
    ///
    /// Names must be unique within their scope (e.g., Act names within a Story).
    #[error("Name conflict: {item_type} '{name}' already exists in {context}")]
    NameConflict {
        item_type: String,
        name: String,
        context: String,
    },

    /// Wraps standard I/O errors.
    ///
    /// Raised during file operations (reading, writing scenario files).
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Wraps XML parsing/serialization errors.
    ///
    /// Raised when reading or writing XML scenario files.
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),
}
