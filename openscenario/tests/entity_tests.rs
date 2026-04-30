use openscenario::{Scenario, OpenScenarioVersion};
use openscenario::entities::{VehicleParams, VehicleCategory, PedestrianParams, MiscObjectParams};
use openscenario::Position;

#[test]
fn test_add_vehicle() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    let result = scenario.add_vehicle("ego", params);
    assert!(result.is_ok());
}

#[test]
fn test_add_vehicle_conflict() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car1", params.clone()).unwrap();
    let result = scenario.add_vehicle("car1", params);
    
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("car1"));
    assert!(err_msg.contains("already exists"));
}

#[test]
fn test_set_initial_position() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("ego", params).unwrap();
    let pos = Position::world(0.0, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_position("ego", pos);
    
    assert!(result.is_ok());
}

#[test]
fn test_set_initial_position_entity_not_found() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let pos = Position::world(0.0, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_position("nonexistent", pos);
    
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("nonexistent"));
}

#[test]
fn test_add_pedestrian() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = PedestrianParams {
        catalog: None,
        model: None,
        mass: Some(75.0),
    };
    
    let result = scenario.add_pedestrian("ped1", params);
    assert!(result.is_ok());
}

#[test]
fn test_add_misc_object() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = MiscObjectParams {
        catalog: None,
        category: Some("Barrier".to_string()),
        mass: Some(100.0),
    };
    
    let result = scenario.add_misc_object("barrier1", params);
    assert!(result.is_ok());
}

#[test]
fn test_get_entity() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("ego", params).unwrap();
    
    let entity = scenario.get_entity("ego");
    assert!(entity.is_some());
    assert_eq!(entity.unwrap().name(), "ego");
    
    let missing = scenario.get_entity("nonexistent");
    assert!(missing.is_none());
}
