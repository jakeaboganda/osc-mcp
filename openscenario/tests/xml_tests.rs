use openscenario::{Scenario, OpenScenarioVersion, Position};
use openscenario::entities::{VehicleParams, VehicleCategory};
use openscenario::storyboard::TransitionShape;

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
