use openscenario_mcp::server::ServerState;
use openscenario_mcp::handlers::{
    handle_create_scenario, handle_add_vehicle, handle_set_position,
    handle_add_speed_action, handle_add_lane_change_action, handle_export_xml
};
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::Path;

#[test]
fn test_add_speed_action_handler() {
    let state = Arc::new(Mutex::new(ServerState::new()));
    
    // Create scenario and add vehicle
    let scenario_id = handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string()
    ).unwrap();
    
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "Car".to_string(),
        None,
    ).unwrap();
    
    // Add speed action
    let result = handle_add_speed_action(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "test_story".to_string(),
        50.0,
        5.0,
    );
    
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Speed action added"));
}

#[test]
fn test_add_lane_change_action_handler() {
    let state = Arc::new(Mutex::new(ServerState::new()));
    
    // Create scenario and add vehicle
    let scenario_id = handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string()
    ).unwrap();
    
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "Car".to_string(),
        None,
    ).unwrap();
    
    // Add lane change action
    let result = handle_add_lane_change_action(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "test_story".to_string(),
        -3.5,
        4.0,
    );
    
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Lane change action added"));
}

#[test]
fn test_export_xml_handler() {
    let state = Arc::new(Mutex::new(ServerState::new()));
    
    // Create scenario and add vehicle
    let scenario_id = handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string()
    ).unwrap();
    
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "Car".to_string(),
        None,
    ).unwrap();
    
    handle_set_position(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        10.0,
        20.0,
        0.0,
        0.0,
    ).unwrap();
    
    // Export to XML
    let output_path = "/tmp/test_scenario_export.xosc";
    let result = handle_export_xml(
        state.clone(),
        scenario_id.clone(),
        output_path.to_string(),
    );
    
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Exported scenario to"));
    
    // Verify file exists and contains valid XML
    assert!(Path::new(output_path).exists());
    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains("<?xml"));
    assert!(content.contains("OpenSCENARIO"));
    assert!(content.contains("ego_vehicle"));
    
    // Cleanup
    fs::remove_file(output_path).ok();
}
