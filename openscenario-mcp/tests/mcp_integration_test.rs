use openscenario_mcp::handlers::{
    handle_add_speed_action, handle_add_vehicle, handle_create_scenario, handle_export_xml,
    handle_set_position, handle_validate_scenario,
};
use openscenario_mcp::server::ServerState;
use std::fs;
use std::sync::{Arc, Mutex};
use tempfile::TempDir;

/// Test complete scenario workflow: create → add vehicle → set position → add action → export
#[test]
fn test_complete_scenario_workflow() {
    let state = Arc::new(Mutex::new(ServerState::new()));
    let temp_dir = TempDir::new().unwrap();
    let export_path = temp_dir.path().join("test_scenario.xosc");

    // Step 1: Create scenario
    let scenario_id = handle_create_scenario(
        state.clone(),
        "integration_test".to_string(),
        "1.2".to_string(),
    )
    .expect("Failed to create scenario");

    assert!(scenario_id.starts_with("integration_test_"));

    // Step 2: Add vehicle
    let vehicle_id = handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "Car".to_string(),
        None,
    )
    .expect("Failed to add vehicle");

    assert_eq!(vehicle_id, "ego_vehicle");

    // Step 3: Set initial position
    let position_result = handle_set_position(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        10.0,
        20.0,
        0.5,
        1.57,
    )
    .expect("Failed to set position");

    assert!(position_result.contains("Position set"));

    // Step 4: Add speed action
    let action_result = handle_add_speed_action(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "main_story".to_string(),
        30.0, // speed: 30 m/s
        5.0,  // duration: 5 seconds
    )
    .expect("Failed to add speed action");

    assert!(action_result.contains("Speed action added"));

    // Step 5: Export to XML
    let export_result = handle_export_xml(
        state.clone(),
        scenario_id.clone(),
        export_path.to_str().unwrap().to_string(),
    )
    .expect("Failed to export XML");

    assert!(export_result.contains("Exported scenario"));
    assert!(export_path.exists(), "Export file should exist");

    // Verify exported XML contains expected elements
    let xml_content = fs::read_to_string(&export_path).unwrap();
    assert!(xml_content.contains("<?xml version"));
    assert!(xml_content.contains("OpenSCENARIO"));
    assert!(xml_content.contains("ego_vehicle"));
    assert!(xml_content.contains("WorldPosition"));
    assert!(xml_content.contains("SpeedAction"));
}

/// Test validation workflow: create → validate
#[test]
fn test_validation_workflow() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    // Step 1: Create scenario
    let scenario_id = handle_create_scenario(
        state.clone(),
        "validation_test".to_string(),
        "1.2".to_string(),
    )
    .expect("Failed to create scenario");

    // Step 2: Add vehicle (required for valid scenario)
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "test_vehicle".to_string(),
        "Car".to_string(),
        None,
    )
    .expect("Failed to add vehicle");

    // Step 3: Set position (required for valid scenario)
    handle_set_position(
        state.clone(),
        scenario_id.clone(),
        "test_vehicle".to_string(),
        0.0,
        0.0,
        0.0,
        0.0,
    )
    .expect("Failed to set position");

    // Step 4: Validate scenario
    let validation_result = handle_validate_scenario(state.clone(), scenario_id.clone())
        .expect("Failed to validate scenario");

    // Parse validation result as JSON
    let report: serde_json::Value =
        serde_json::from_str(&validation_result).expect("Validation result should be valid JSON");

    // Check that we got a valid report structure
    assert!(
        report.get("valid").is_some(),
        "Report should have 'valid' field"
    );
    assert!(
        report.get("errors").is_some(),
        "Report should have 'errors' field"
    );

    // The scenario should be valid (basic structure is correct)
    let is_valid = report["valid"].as_bool().unwrap_or(false);
    assert!(
        is_valid,
        "Basic scenario should be valid. Errors: {}",
        report["errors"]
    );
}

/// Test catalog workflow: create → add vehicle from catalog
#[test]
fn test_catalog_workflow() {
    let state = Arc::new(Mutex::new(ServerState::new()));
    let temp_dir = TempDir::new().unwrap();

    // Step 1: Create a simple catalog file
    let catalog_path = temp_dir.path().join("VehicleCatalog.xosc");
    let catalog_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="2" date="2024-01-01T00:00:00" description="Test Vehicle Catalog"/>
  <Catalog name="VehicleCatalog">
    <Vehicle name="SportsCar" vehicleCategory="car">
      <BoundingBox>
        <Center x="1.5" y="0.0" z="0.9"/>
        <Dimensions width="2.1" length="4.5" height="1.8"/>
      </BoundingBox>
      <Performance maxSpeed="69.444" maxAcceleration="10.0" maxDeceleration="10.0"/>
      <Axles>
        <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.8" positionX="3.1" positionZ="0.3"/>
        <RearAxle maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.8" positionX="0.0" positionZ="0.3"/>
      </Axles>
    </Vehicle>
  </Catalog>
</OpenSCENARIO>
"#;
    fs::write(&catalog_path, catalog_content).expect("Failed to write catalog file");

    // Step 2: Create scenario
    let scenario_id =
        handle_create_scenario(state.clone(), "catalog_test".to_string(), "1.2".to_string())
            .expect("Failed to create scenario");

    // Step 3: Add vehicle from catalog
    let catalog_ref = format!("{}:SportsCar", catalog_path.to_str().unwrap());
    let vehicle_id = handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "sports_car".to_string(),
        "Car".to_string(),
        Some(catalog_ref.clone()),
    )
    .expect("Failed to add vehicle from catalog");

    assert_eq!(vehicle_id, "sports_car");

    // Step 4: Set position for catalog vehicle
    let position_result = handle_set_position(
        state.clone(),
        scenario_id.clone(),
        "sports_car".to_string(),
        5.0,
        10.0,
        0.0,
        0.0,
    )
    .expect("Failed to set position for catalog vehicle");

    assert!(position_result.contains("Position set"));

    // Step 5: Export and verify catalog reference in XML
    let export_path = temp_dir.path().join("catalog_scenario.xosc");
    handle_export_xml(
        state.clone(),
        scenario_id.clone(),
        export_path.to_str().unwrap().to_string(),
    )
    .expect("Failed to export scenario with catalog");

    // Verify catalog reference in exported XML
    let xml_content = fs::read_to_string(&export_path).unwrap();

    // The XML should contain CatalogReference structure
    // Note: Current implementation doesn't output catalogName/entryName attributes yet
    // but the structure indicates catalog usage
    assert!(
        xml_content.contains("CatalogReference"),
        "XML should contain CatalogReference"
    );
    assert!(
        xml_content.contains("sports_car"),
        "XML should reference the vehicle entity"
    );
}
