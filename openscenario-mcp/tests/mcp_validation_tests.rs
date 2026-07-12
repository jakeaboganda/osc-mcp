use openscenario_mcp::handlers::{
    handle_create_scenario, handle_load_road_network, handle_validate_scenario,
};
use openscenario_mcp::server::ServerState;
use std::fs;
use std::sync::{Arc, Mutex};

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
    let xodr_path = format!("/tmp/test_validation_road_{}.xodr", timestamp);
    fs::write(&xodr_path, MINIMAL_XODR).expect("Failed to write test XODR");

    let _ = handle_load_road_network(state.clone(), xodr_path.clone());
    let _ = fs::remove_file(&xodr_path);

    state
}

#[test]
fn test_validate_scenario_handler() {
    let state = setup_state();

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

    // v1.2.0's real ASAM schema is checked into openscenario/schemas/, so this must
    // exercise actual XSD validation, not fall back to the "schema not available" path.
    assert!(
        !report.contains("XSD schema not available"),
        "v1.2.0 schema should be loaded: {report}"
    );
    assert!(report.contains("valid"));
    assert!(
        report.contains("true"),
        "bare scenario should pass real XSD validation: {report}"
    );
}

#[test]
fn test_validate_nonexistent_scenario() {
    let state = setup_state();

    // Try to validate a scenario that doesn't exist
    let result = handle_validate_scenario(state.clone(), "nonexistent_id".to_string());

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Scenario not found"));
}
