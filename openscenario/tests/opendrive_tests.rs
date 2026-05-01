use openscenario::opendrive_validator::OpenDriveValidator;
use std::path::PathBuf;

#[test]
fn test_load_opendrive() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/test_road.xodr");
    
    let validator = OpenDriveValidator::load(&path).expect("Failed to load OpenDRIVE file");
    assert!(validator.road_exists("road1"));
}

#[test]
fn test_validate_lane_position() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/test_road.xodr");
    
    let validator = OpenDriveValidator::load(&path).expect("Failed to load OpenDRIVE file");
    
    // Valid lane positions
    assert!(validator.validate_lane_position("road1", 1_i64).is_ok());
    assert!(validator.validate_lane_position("road1", -1_i64).is_ok());
    
    // Invalid lane positions
    assert!(validator.validate_lane_position("road1", 5_i64).is_err());
    assert!(validator.validate_lane_position("invalid_road", 1_i64).is_err());
}

#[test]
fn test_validate_road_position() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/test_road.xodr");
    
    let validator = OpenDriveValidator::load(&path).expect("Failed to load OpenDRIVE file");
    
    // Valid road positions (within 0-100 range)
    assert!(validator.validate_road_position("road1", 0.0).is_ok());
    assert!(validator.validate_road_position("road1", 50.0).is_ok());
    assert!(validator.validate_road_position("road1", 100.0).is_ok());
    
    // Invalid road positions
    assert!(validator.validate_road_position("road1", 150.0).is_err());
    assert!(validator.validate_road_position("road1", -10.0).is_err());
    assert!(validator.validate_road_position("invalid_road", 50.0).is_err());
}
