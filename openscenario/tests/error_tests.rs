use openscenario::ScenarioError;

#[test]
fn test_entity_conflict_error() {
    let err = ScenarioError::EntityConflict {
        name: "car1".to_string(),
        existing_location: None,
    };
    
    assert!(err.to_string().contains("car1"));
    assert!(err.to_string().contains("already exists"));
}

#[test]
fn test_version_mismatch_error() {
    let err = ScenarioError::VersionMismatch {
        feature: "AppearanceAction".to_string(),
        required_version: "1.2".to_string(),
        current_version: "1.0".to_string(),
    };
    
    assert!(err.to_string().contains("AppearanceAction"));
    assert!(err.to_string().contains("1.2"));
}
