use openscenario::{Scenario, OpenScenarioVersion, Position};
use openscenario::entities::{VehicleParams, VehicleCategory};
use openscenario::storyboard::TransitionShape;

#[test]
fn test_complete_scenario_workflow() {
    // Create scenario
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    
    // Add vehicles
    let ego_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", ego_params.clone()).unwrap();
    scenario.add_vehicle("npc", ego_params).unwrap();
    
    // Set initial positions
    let ego_pos = Position::world(0.0, 0.0, 0.0, 0.0);
    let npc_pos = Position::world(50.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("ego", ego_pos).unwrap();
    scenario.set_initial_position("npc", npc_pos).unwrap();
    
    // Create storyboard
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "ego_mg").unwrap();
    scenario.add_actor("main_story", "act1", "ego_mg", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "ego_mg", "speed_maneuver").unwrap();
    
    // Add actions
    scenario.add_speed_action(
        "main_story",
        "act1",
        "ego_mg",
        "speed_maneuver",
        "speed_event",
        30.0,
        5.0,
        TransitionShape::Linear,
    ).unwrap();
    
    scenario.add_lane_change_action(
        "main_story",
        "act1",
        "ego_mg",
        "speed_maneuver",
        "lane_change_event",
        -1.0,
        3.0,
        TransitionShape::Sinusoidal,
    ).unwrap();
    
    // Export to XML
    let xml = scenario.to_xml().unwrap();
    
    // Verify XML structure
    assert!(xml.contains("<?xml"));
    assert!(xml.contains("<OpenSCENARIO"));
    assert!(xml.contains("<FileHeader"));
    assert!(xml.contains("<Entities"));
    assert!(xml.contains("<ScenarioObject name=\"ego\""));
    assert!(xml.contains("<ScenarioObject name=\"npc\""));
    assert!(xml.contains("<Storyboard"));
    assert!(xml.contains("<Story"));
    assert!(xml.contains("<SpeedAction"));
    assert!(xml.contains("<LaneChangeAction"));
    assert!(xml.contains("</OpenSCENARIO>"));
    
    // Verify it's not empty
    assert!(xml.len() > 1000);
}

#[test]
fn test_lane_position_scenario() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("car", params).unwrap();
    
    let lane_pos = Position::lane("road1", 1, 50.0, 0.0, None);
    scenario.set_initial_position("car", lane_pos).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    assert!(xml.contains("<LanePosition"));
    assert!(xml.contains("roadId=\"road1\""));
    assert!(xml.contains("laneId=\"1\""));
}

#[test]
fn test_relative_position_scenario() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("lead", params.clone()).unwrap();
    scenario.add_vehicle("follower", params).unwrap();
    
    let lead_pos = Position::world(0.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("lead", lead_pos).unwrap();
    
    use openscenario::position::Orientation;
    let follower_pos = Position::relative_world("lead", -10.0, 0.0, 0.0, Orientation::default());
    scenario.set_initial_position("follower", follower_pos).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    assert!(xml.contains("<RelativeWorldPosition"));
    assert!(xml.contains("entityRef=\"lead\""));
}
