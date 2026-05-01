use openscenario_mcp::handlers::{handle_create_scenario, handle_validate_scenario};
use openscenario_mcp::server::ServerState;
use std::sync::{Arc, Mutex};

#[test]
fn test_validate_scenario_handler() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    // Create a scenario
    let scenario_id = handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string(),
    )
    .unwrap();

    // Validate the scenario
    let result = handle_validate_scenario(state.clone(), scenario_id.clone());

    assert!(result.is_ok());
    let report = result.unwrap();

    // Should contain validation report with valid field
    assert!(report.contains("valid"));
    assert!(report.contains("true") || report.contains("false"));
}

#[test]
fn test_validate_nonexistent_scenario() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    // Try to validate a scenario that doesn't exist
    let result = handle_validate_scenario(state.clone(), "nonexistent_id".to_string());

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Scenario not found"));
}
