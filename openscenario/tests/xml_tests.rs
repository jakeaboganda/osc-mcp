use openscenario::{Scenario, OpenScenarioVersion, Position};
use openscenario::entities::{VehicleParams, VehicleCategory};
use openscenario::storyboard::TransitionShape;
use openscenario::position::Orientation;

#[test]
fn test_xml_export_minimal() {
    let scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let xml = scenario.to_xml().unwrap();
    
    // Should have basic structure
    assert!(xml.contains("<?xml"));
    assert!(xml.contains("<OpenSCENARIO"));
    assert!(xml.contains("</OpenSCENARIO>"));
    assert!(xml.contains("FileHeader"));
}

#[test]
fn test_xml_export_with_vehicle() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();
    
    let pos = Position::world(0.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("ego", pos).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    assert!(xml.contains("<ScenarioObject name=\"ego\""));
    assert!(xml.contains("<Vehicle"));
    assert!(xml.contains("<Init>"));
    assert!(xml.contains("<WorldPosition"));
}

#[test]
fn test_xml_export_with_story() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    scenario.add_speed_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        50.0,
        5.0,
        TransitionShape::Linear,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    assert!(xml.contains("<Storyboard"));
    assert!(xml.contains("<Story"));
    assert!(xml.contains("<Act"));
    assert!(xml.contains("<ManeuverGroup"));
    assert!(xml.contains("<Maneuver"));
    assert!(xml.contains("<Event"));
    assert!(xml.contains("<Action"));
    assert!(xml.contains("<SpeedAction"));
}

#[test]
fn test_xml_export_with_relative_positions() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params.clone()).unwrap();
    scenario.add_vehicle("target", params).unwrap();
    
    // Test RelativeWorld position
    let pos_relative_world = Position::relative_world(
        "target",
        10.0,
        5.0,
        0.0,
        Orientation { h: 0.0, p: 0.0, r: 0.0 },
    );
    scenario.set_initial_position("ego", pos_relative_world).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    assert!(xml.contains("<RelativeWorldPosition"));
    assert!(xml.contains("entityRef=\"target\""));
    assert!(xml.contains("dx=\"10\""));
    assert!(xml.contains("dy=\"5\""));
    assert!(xml.contains("<Orientation"));
    assert!(xml.contains("type=\"relative\""));
}

#[test]
fn test_xml_export_lane_position_with_orientation() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();
    
    // Test Lane position with orientation
    let pos = Position::lane(
        "road1",
        -1,
        100.0,
        0.5,
        Some(Orientation { h: 1.57, p: 0.0, r: 0.0 }),
    );
    scenario.set_initial_position("ego", pos).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    assert!(xml.contains("<LanePosition"));
    assert!(xml.contains("roadId=\"road1\""));
    assert!(xml.contains("laneId=\"-1\""));
    assert!(xml.contains("s=\"100\""));
    assert!(xml.contains("offset=\"0.5\""));
    assert!(xml.contains("h=\"1.57\""));
}

#[test]
fn test_xml_export_road_position() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();
    
    // Test Road position
    let pos = Position::road(
        "road1",
        50.0,
        2.5,
        Some(Orientation { h: 0.0, p: 0.0, r: 0.0 }),
    );
    scenario.set_initial_position("ego", pos).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    assert!(xml.contains("<RoadPosition"));
    assert!(xml.contains("roadId=\"road1\""));
    assert!(xml.contains("s=\"50\""));
    assert!(xml.contains("t=\"2.5\""));
}

#[test]
fn test_xml_export_lane_change_with_actor() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    scenario.add_lane_change_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        1.0,
        3.0,
        TransitionShape::Linear,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    assert!(xml.contains("<LaneChangeAction"));
    assert!(xml.contains("<RelativeTargetLane"));
    // Verify entityRef is set to the actor, not empty
    assert!(xml.contains("entityRef=\"ego\""));
    assert!(xml.contains("value=\"1\""));
}
