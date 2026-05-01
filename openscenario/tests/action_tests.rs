use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::TransitionShape;
use openscenario::{OpenScenarioVersion, Scenario};

#[test]
fn test_add_maneuver() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();

    let result = scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1");
    assert!(result.is_ok());
}

#[test]
fn test_add_speed_action() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    let result = scenario.add_speed_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        50.0,
        5.0,
        TransitionShape::Linear,
    );

    assert!(result.is_ok());
}

#[test]
fn test_add_lane_change_action() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    let result = scenario.add_lane_change_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        -1.0,
        3.0,
        TransitionShape::Sinusoidal,
    );

    assert!(result.is_ok());
}
