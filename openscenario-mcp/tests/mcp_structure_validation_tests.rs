use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::{DynamicsDimension, DynamicsShape, TransitionDynamics};
use openscenario::Position;
use openscenario_mcp::handlers::{
    handle_create_scenario, handle_load_road_network, handle_validate_scenario_structure,
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
    let xodr_path = format!("/tmp/test_structure_validation_road_{}.xodr", timestamp);
    fs::write(&xodr_path, MINIMAL_XODR).expect("Failed to write test XODR");

    let _ = handle_load_road_network(state.clone(), xodr_path.clone());
    let _ = fs::remove_file(&xodr_path);

    state
}

fn create_scenario(state: &Arc<Mutex<ServerState>>) -> String {
    handle_create_scenario(
        state.clone(),
        "test_scenario".to_string(),
        "1.2".to_string(),
    )
    .unwrap()
}

fn linear_dynamics() -> TransitionDynamics {
    TransitionDynamics {
        shape: DynamicsShape::Linear,
        dimension: DynamicsDimension::Time,
        value: 5.0,
    }
}

#[test]
fn test_validate_structure_scenario_not_found() {
    let state = setup_state();
    let result = handle_validate_scenario_structure(state, "nonexistent".to_string(), false);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}

#[test]
fn test_validate_structure_empty_scenario_passes() {
    // No stories at all: nothing to check, should report no issues.
    let state = setup_state();
    let scenario_id = create_scenario(&state);

    let report = handle_validate_scenario_structure(state, scenario_id, false).unwrap();
    assert!(report.contains("No issues found"));
}

#[test]
fn test_validate_structure_fully_built_scenario_passes() {
    let state = setup_state();
    let scenario_id = create_scenario(&state);

    {
        let mut state_lock = state.lock().unwrap();
        let scenario = state_lock.scenarios.get_mut(&scenario_id).unwrap();
        scenario
            .add_vehicle(
                "ego",
                VehicleParams {
                    catalog: None,
                    vehicle_category: VehicleCategory::Car,
                    properties: None,
                },
            )
            .unwrap();
        scenario
            .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
            .unwrap();
        scenario.add_story("story1").unwrap();
        scenario.add_act("story1", "act1").unwrap();
        scenario
            .add_maneuver_group("story1", "act1", "mg1")
            .unwrap();
        scenario.add_actor("story1", "act1", "mg1", "ego").unwrap();
        scenario
            .add_maneuver("story1", "act1", "mg1", "maneuver1")
            .unwrap();
        scenario
            .add_speed_action(
                "story1",
                "act1",
                "mg1",
                "maneuver1",
                "event1",
                10.0,
                linear_dynamics(),
            )
            .unwrap();
    }

    let report = handle_validate_scenario_structure(state, scenario_id, false).unwrap();
    assert!(
        report.contains("No issues found"),
        "well-formed scenario should report clean: {report}"
    );
}

#[test]
fn test_validate_structure_story_with_no_acts_is_error() {
    // Story requires >= 1 Act per the OpenSCENARIO XSD; zero Acts means the
    // exported document would fail real XSD validation.
    let state = setup_state();
    let scenario_id = create_scenario(&state);

    {
        let mut state_lock = state.lock().unwrap();
        let scenario = state_lock.scenarios.get_mut(&scenario_id).unwrap();
        scenario.add_story("empty_story").unwrap();
    }

    let report = handle_validate_scenario_structure(state, scenario_id, false).unwrap();
    assert!(report.contains("Errors"), "report: {report}");
    assert!(report.contains("empty_story"), "report: {report}");
    assert!(
        report.contains("no Acts") || report.contains("no acts"),
        "report: {report}"
    );
}

#[test]
fn test_validate_structure_act_with_no_maneuver_groups_is_error() {
    // Act requires >= 1 ManeuverGroup per the XSD.
    let state = setup_state();
    let scenario_id = create_scenario(&state);

    {
        let mut state_lock = state.lock().unwrap();
        let scenario = state_lock.scenarios.get_mut(&scenario_id).unwrap();
        scenario.add_story("story1").unwrap();
        scenario.add_act("story1", "empty_act").unwrap();
    }

    let report = handle_validate_scenario_structure(state, scenario_id, false).unwrap();
    assert!(report.contains("Errors"), "report: {report}");
    assert!(report.contains("empty_act"), "report: {report}");
}

#[test]
fn test_validate_structure_maneuver_with_no_events_is_error() {
    // Maneuver requires >= 1 Event per the XSD.
    let state = setup_state();
    let scenario_id = create_scenario(&state);

    {
        let mut state_lock = state.lock().unwrap();
        let scenario = state_lock.scenarios.get_mut(&scenario_id).unwrap();
        scenario.add_story("story1").unwrap();
        scenario.add_act("story1", "act1").unwrap();
        scenario
            .add_maneuver_group("story1", "act1", "mg1")
            .unwrap();
        scenario
            .add_maneuver("story1", "act1", "mg1", "empty_maneuver")
            .unwrap();
    }

    let report = handle_validate_scenario_structure(state, scenario_id, false).unwrap();
    assert!(report.contains("Errors"), "report: {report}");
    assert!(report.contains("empty_maneuver"), "report: {report}");
}

#[test]
fn test_validate_structure_entity_without_initial_state_is_warning() {
    // An entity with neither an initial position nor speed is silently omitted
    // from <Init><Actions> by the XML writer -- it never gets placed in the sim.
    let state = setup_state();
    let scenario_id = create_scenario(&state);

    {
        let mut state_lock = state.lock().unwrap();
        let scenario = state_lock.scenarios.get_mut(&scenario_id).unwrap();
        scenario
            .add_vehicle(
                "ghost",
                VehicleParams {
                    catalog: None,
                    vehicle_category: VehicleCategory::Car,
                    properties: None,
                },
            )
            .unwrap();
    }

    let report = handle_validate_scenario_structure(state, scenario_id, false).unwrap();
    assert!(report.contains("Warnings"), "report: {report}");
    assert!(report.contains("ghost"), "report: {report}");
}

#[test]
fn test_validate_structure_maneuver_group_with_no_actors_is_warning() {
    // Schema-valid (Actors' EntityRef children are optional) but semantically a
    // no-op: the maneuvers in this group will never act on anyone.
    let state = setup_state();
    let scenario_id = create_scenario(&state);

    {
        let mut state_lock = state.lock().unwrap();
        let scenario = state_lock.scenarios.get_mut(&scenario_id).unwrap();
        scenario.add_story("story1").unwrap();
        scenario.add_act("story1", "act1").unwrap();
        scenario
            .add_maneuver_group("story1", "act1", "actorless_mg")
            .unwrap();
        scenario
            .add_maneuver("story1", "act1", "actorless_mg", "maneuver1")
            .unwrap();
        scenario
            .add_speed_action(
                "story1",
                "act1",
                "actorless_mg",
                "maneuver1",
                "event1",
                10.0,
                linear_dynamics(),
            )
            .unwrap();
    }

    let report = handle_validate_scenario_structure(state, scenario_id, false).unwrap();
    assert!(report.contains("Warnings"), "report: {report}");
    assert!(report.contains("actorless_mg"), "report: {report}");
}

#[test]
fn test_validate_structure_multiple_issues_all_reported() {
    let state = setup_state();
    let scenario_id = create_scenario(&state);

    {
        let mut state_lock = state.lock().unwrap();
        let scenario = state_lock.scenarios.get_mut(&scenario_id).unwrap();
        scenario
            .add_vehicle(
                "ghost",
                VehicleParams {
                    catalog: None,
                    vehicle_category: VehicleCategory::Car,
                    properties: None,
                },
            )
            .unwrap();
        scenario.add_story("empty_story").unwrap();
        scenario.add_story("story2").unwrap();
        scenario.add_act("story2", "empty_act").unwrap();
    }

    let report = handle_validate_scenario_structure(state, scenario_id, false).unwrap();
    assert!(report.contains("empty_story"), "report: {report}");
    assert!(report.contains("empty_act"), "report: {report}");
    assert!(report.contains("ghost"), "report: {report}");
}

#[test]
fn test_validate_structure_auto_fix_does_not_hide_non_fixable_issues() {
    // None of the new checks have a safe auto-fix (there's no sensible default
    // position/actor to guess), so auto_fix=true must still surface them.
    let state = setup_state();
    let scenario_id = create_scenario(&state);

    {
        let mut state_lock = state.lock().unwrap();
        let scenario = state_lock.scenarios.get_mut(&scenario_id).unwrap();
        scenario.add_story("empty_story").unwrap();
    }

    let report = handle_validate_scenario_structure(state, scenario_id, true).unwrap();
    assert!(report.contains("empty_story"), "report: {report}");
    assert!(report.contains("Errors"), "report: {report}");
}
