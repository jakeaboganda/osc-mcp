use openscenario::{Scenario, OpenScenarioVersion, Position};
use openscenario::entities::{VehicleParams, VehicleCategory};

#[test]
fn test_add_position_action() {
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
    
    let position = Position::world(100.0, 200.0, 0.0, 1.57);
    let result = scenario.add_position_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        position,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_add_distance_action() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params.clone()).unwrap();
    scenario.add_vehicle("target", params).unwrap();
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    let result = scenario.add_distance_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "target",
        30.0,
        true,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_position_action_in_xml() {
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
    
    let position = Position::world(100.0, 200.0, 0.0, 1.57);
    scenario.add_position_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        position,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    assert!(xml.contains("<TeleportAction>"));
    assert!(xml.contains("<Position>"));
    assert!(xml.contains("<WorldPosition"));
    assert!(xml.contains("x=\"100\""));
    assert!(xml.contains("y=\"200\""));
}
