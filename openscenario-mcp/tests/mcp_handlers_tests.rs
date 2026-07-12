use openscenario::Position;
use openscenario_mcp::handlers::{
    handle_add_vehicle, handle_create_scenario, handle_load_road_network, handle_set_position,
};
use openscenario_mcp::server::ServerState;
use std::fs;
use std::sync::{Arc, Mutex};

// Minimal OpenDRIVE XML for testing
const MINIMAL_XODR: &str = r###"<?xml version="1.0" encoding="UTF-8"?>
<OpenDRIVE>
    <header revMajor="1" revMinor="6" name="test_road" version="1.0" date="2026-05-31T00:00:00"/>
    <road name="test_road" length="1000.0" id="1" junction="-1">
        <link/>
        <planView>
            <geometry s="0.0" x="0.0" y="0.0" hdg="0.0" length="1000.0">
                <line/>
            </geometry>
        </planView>
        <lanes>
            <laneSection s="0.0">
                <center>
                    <lane id="0" type="none" level="false">
                        <link/>
                    </lane>
                </center>
                <right>
                    <lane id="-1" type="driving" level="false">
                        <link/>
                        <width sOffset="0.0" a="3.5" b="0.0" c="0.0" d="0.0"/>
                    </lane>
                </right>
            </laneSection>
        </lanes>
    </road>
</OpenDRIVE>
"###;

fn setup_state() -> Arc<Mutex<ServerState>> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let state = Arc::new(Mutex::new(ServerState::new()));

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let xodr_path = format!("/tmp/test_handlers_road_{}.xodr", timestamp);
    fs::write(&xodr_path, MINIMAL_XODR).expect("Failed to write test XODR");

    let _ = handle_load_road_network(state.clone(), xodr_path.clone());
    let _ = fs::remove_file(&xodr_path);

    state
}

#[test]
fn test_create_scenario_handler() {
    let state = setup_state();

    // Create a scenario
    let result = handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string(),
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
fn test_create_scenario_handler_v1_3() {
    let state = setup_state();

    let scenario_id = handle_create_scenario(
        state.clone(),
        "test_scenario_v13".to_string(),
        "1.3".to_string(),
    )
    .unwrap();

    let state_lock = state.lock().unwrap();
    let scenario = state_lock.scenarios.get(&scenario_id).unwrap();
    assert_eq!(scenario.version().to_string(), "1.3");
}

#[test]
fn test_create_scenario_handler_rejects_unknown_version() {
    let state = setup_state();

    let result = handle_create_scenario(state.clone(), "bad".to_string(), "1.4".to_string());

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid version"));
}

#[test]
fn test_add_vehicle_handler() {
    let state = setup_state();

    // First create a scenario
    let scenario_id = handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string(),
    )
    .unwrap();

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
    let state = setup_state();

    // Create scenario and add vehicle
    let scenario_id = handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string(),
    )
    .unwrap();

    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "ego_vehicle".to_string(),
        "Car".to_string(),
        None,
    )
    .unwrap();

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
