use openscenario::{Scenario, OpenScenarioVersion};
use openscenario::entities::{VehicleParams, VehicleCategory};

#[test]
fn test_add_story() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let result = scenario.add_story("main_story");
    assert!(result.is_ok());
}

#[test]
fn test_add_act() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    scenario.add_story("main_story").unwrap();
    let result = scenario.add_act("main_story", "act1");
    assert!(result.is_ok());
}

#[test]
fn test_add_act_story_not_found() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let result = scenario.add_act("nonexistent", "act1");
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("nonexistent"));
}

#[test]
fn test_add_maneuver_group() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    
    let result = scenario.add_maneuver_group("main_story", "act1", "mg1");
    assert!(result.is_ok());
}

#[test]
fn test_add_actor() {
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
    
    let result = scenario.add_actor("main_story", "act1", "mg1", "ego");
    assert!(result.is_ok());
}

#[test]
fn test_add_actor_entity_not_found() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    
    let result = scenario.add_actor("main_story", "act1", "mg1", "nonexistent");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("nonexistent"));
}
