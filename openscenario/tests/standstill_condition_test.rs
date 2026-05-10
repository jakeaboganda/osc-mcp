use openscenario::{
    OpenScenarioVersion, Position, Scenario, ScenarioError,
};
use openscenario::storyboard::ConditionEdge;

#[test]
fn test_standstill_condition_basic() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    // Add a vehicle
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    
    // Add story structure
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Add event with standstill condition (stopped for 5 seconds)
    let result = scenario.add_event_with_standstill_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        5.0,  // duration in seconds
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_standstill_condition_traffic_light() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Trigger after stopped at light for 30 seconds
    let result = scenario.add_event_with_standstill_condition(
        "main_story", "act1", "mg1", "maneuver1", "light_timeout",
        "ego", 30.0,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_standstill_condition_short_duration() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Very short standstill (0.5 seconds)
    let result = scenario.add_event_with_standstill_condition(
        "main_story", "act1", "mg1", "maneuver1", "brief_stop",
        "ego", 0.5,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_standstill_condition_zero_duration_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Zero duration should fail
    let result = scenario.add_event_with_standstill_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", 0.0,
    );
    
    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidValue { field, .. }) => {
            assert!(field.contains("duration"));
        }
        _ => panic!("Expected InvalidValue error"),
    }
}

#[test]
fn test_standstill_condition_negative_duration_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Negative duration should fail
    let result = scenario.add_event_with_standstill_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", -5.0,
    );
    
    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidValue { field, .. }) => {
            assert!(field.contains("duration"));
        }
        _ => panic!("Expected InvalidValue error"),
    }
}

#[test]
fn test_standstill_condition_nonexistent_entity_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Reference non-existent entity
    let result = scenario.add_event_with_standstill_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "nonexistent", 5.0,
    );
    
    assert!(result.is_err());
}

#[test]
fn test_standstill_condition_with_edge() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Test with rising edge (trigger when condition becomes true)
    let result = scenario.add_event_with_standstill_condition_advanced(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", 5.0, ConditionEdge::Rising, 0.0,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_standstill_condition_with_delay() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Add 1.5 second delay after standstill detected
    let result = scenario.add_event_with_standstill_condition_advanced(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", 10.0, ConditionEdge::None, 1.5,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_standstill_condition_all_edges() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Test all edge types
    let edges = vec![
        ConditionEdge::None,
        ConditionEdge::Rising,
        ConditionEdge::Falling,
        ConditionEdge::RisingOrFalling,
    ];
    
    for edge in edges.iter() {
        let result = scenario.add_event_with_standstill_condition_advanced(
            "main_story", "act1", "mg1", "maneuver1",
            &format!("event_{:?}", edge),
            "ego", 5.0, *edge, 0.0,
        );
        
        assert!(result.is_ok(), "Failed for edge: {:?}", edge);
    }
}

#[test]
fn test_standstill_condition_xml_export() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    scenario.add_event_with_standstill_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", 10.0,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Check for standstill condition presence
    assert!(xml.contains("<ByEntityCondition>"));
    assert!(xml.contains("<StandStillCondition"));
    assert!(xml.contains("duration=\"10\""));
}

#[test]
fn test_standstill_condition_multiple_vehicles() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    
    // Create multiple vehicles
    scenario.add_vehicle("car1", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("car2", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("car3", vehicle_params).unwrap();
    scenario.set_initial_position("car1", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("car2", Position::world(10.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("car3", Position::world(20.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "car1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "car2").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "car3").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Different standstill durations for different vehicles
    scenario.add_event_with_standstill_condition(
        "main_story", "act1", "mg1", "maneuver1", "car1_stopped",
        "car1", 5.0,
    ).unwrap();
    
    scenario.add_event_with_standstill_condition(
        "main_story", "act1", "mg1", "maneuver1", "car2_stopped",
        "car2", 10.0,
    ).unwrap();
    
    scenario.add_event_with_standstill_condition(
        "main_story", "act1", "mg1", "maneuver1", "car3_stopped",
        "car3", 15.0,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Should contain all three conditions
    assert!(xml.contains("car1_stopped"));
    assert!(xml.contains("car2_stopped"));
    assert!(xml.contains("car3_stopped"));
}

#[test]
fn test_standstill_condition_parking_scenario() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Parking complete: stopped for 2 seconds
    let result = scenario.add_event_with_standstill_condition(
        "main_story", "act1", "mg1", "maneuver1", "parking_complete",
        "ego", 2.0,
    );
    
    assert!(result.is_ok());
    
    let xml = scenario.to_xml().unwrap();
    assert!(xml.contains("parking_complete"));
    assert!(xml.contains("duration=\"2\""));
}
