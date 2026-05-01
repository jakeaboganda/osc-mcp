use openscenario_mcp::server::ServerState;
use openscenario_mcp::handlers::{handle_create_scenario, handle_add_vehicle, handle_set_position};
use openscenario::Position;
use std::sync::{Arc, Mutex};

#[test]
fn test_create_scenario_handler() {
    let state = Arc::new(Mutex::new(ServerState::new()));
    
    // Create a scenario
    let result = handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string()
    );
    
    assert!(result.is_ok());
    let scenario_id = result.unwrap();
    
    // Verify scenario exists in state
    let state_lock = state.lock().unwrap();
    assert!(state_lock.scenarios.contains_key(&scenario_id));
    
    // Verify scenario has correct version
    let scenario = state_lock.scenarios.get(&scenario_id).unwrap();
    assert_eq!(scenario.version().to_string(), "1.2");
}

#[test]
fn test_add_vehicle_handler() {
    let state = Arc::new(Mutex::new(ServerState::new()));
    
    // First create a scenario
    let scenario_id = handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string()
    ).unwrap();
    
    // Add a vehicle
    let result = handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "Car".to_string(),
        None,
    );
    
    assert!(result.is_ok());
    let vehicle_id = result.unwrap();
    assert_eq!(vehicle_id, "ego_vehicle");
    
    // Verify vehicle exists in scenario
    let state_lock = state.lock().unwrap();
    let scenario = state_lock.scenarios.get(&scenario_id).unwrap();
    assert!(scenario.get_entity("ego_vehicle").is_some());
}

#[test]
fn test_set_position_handler() {
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
    
    // Set initial position
    let result = handle_set_position(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        10.0,
        20.0,
        0.5,
        1.57,
    );
    
    assert!(result.is_ok());
    
    // Verify position was set
    let state_lock = state.lock().unwrap();
    let scenario = state_lock.scenarios.get(&scenario_id).unwrap();
    let position = scenario.get_initial_position("ego_vehicle");
    assert!(position.is_some());
    
    // Verify position values
    if let Some(Position::World { x, y, z, h, .. }) = position {
        assert_eq!(x, &10.0);
        assert_eq!(y, &20.0);
        assert_eq!(z, &0.5);
        assert_eq!(h, &1.57);
    } else {
        panic!("Expected World position");
    }
}
