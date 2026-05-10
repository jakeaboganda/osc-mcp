use openscenario::{
    OpenScenarioVersion, Position, Scenario, ScenarioError,
};
use openscenario::storyboard::{ConditionEdge, CoordinateSystem, RelativeDistanceType, Rule};

#[test]
fn test_relative_distance_condition_basic() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    // Add two vehicles
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(50.0, 0.0, 0.0, 0.0)).unwrap();
    
    // Add story structure
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Add event with relative distance condition (Euclidean)
    let result = scenario.add_event_with_relative_distance_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",                              // entity being monitored
        "target",                           // reference entity
        10.0,                               // distance value in meters
        Rule::LessThan,                     // trigger when distance < 10m
        RelativeDistanceType::Euclidean,    // Euclidean distance
        false,                              // freespace: false = reference point
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_relative_distance_condition_longitudinal() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(50.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Longitudinal distance (along road direction)
    let result = scenario.add_event_with_relative_distance_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "target", 20.0, Rule::LessThan,
        RelativeDistanceType::Longitudinal, false,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_relative_distance_condition_lateral() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(0.0, 3.5, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Lateral distance (perpendicular to road direction)
    let result = scenario.add_event_with_relative_distance_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "target", 4.0, Rule::LessThan,
        RelativeDistanceType::Lateral, false,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_relative_distance_condition_freespace() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(10.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Freespace: true = distance between bounding boxes
    let result = scenario.add_event_with_relative_distance_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "target", 5.0, Rule::LessThan,
        RelativeDistanceType::Euclidean, true,  // freespace = true
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_relative_distance_condition_negative_value_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(50.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Negative distance should fail
    let result = scenario.add_event_with_relative_distance_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "target", -10.0, Rule::LessThan,
        RelativeDistanceType::Euclidean, false,
    );
    
    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidValue { field, .. }) => {
            assert!(field.contains("distance"));
        }
        _ => panic!("Expected InvalidValue error"),
    }
}

#[test]
fn test_relative_distance_condition_nonexistent_entity_fails() {
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
    
    // Reference non-existent target entity
    let result = scenario.add_event_with_relative_distance_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "nonexistent", 10.0, Rule::LessThan,
        RelativeDistanceType::Euclidean, false,
    );
    
    assert!(result.is_err());
}

#[test]
fn test_relative_distance_condition_all_rules() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(50.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Test all rule types
    let rules = vec![
        Rule::LessThan,
        Rule::GreaterThan,
        Rule::EqualTo,
    ];
    
    for rule in rules.iter() {
        let result = scenario.add_event_with_relative_distance_condition(
            "main_story", "act1", "mg1", "maneuver1",
            &format!("event_{:?}", rule),
            "ego", "target", 20.0, *rule,
            RelativeDistanceType::Euclidean, false,
        );
        
        assert!(result.is_ok(), "Failed for rule: {:?}", rule);
    }
}

#[test]
fn test_relative_distance_condition_all_distance_types() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(50.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Test all distance types
    let distance_types = vec![
        RelativeDistanceType::Longitudinal,
        RelativeDistanceType::Lateral,
        RelativeDistanceType::Euclidean,
    ];
    
    for dist_type in distance_types.iter() {
        let result = scenario.add_event_with_relative_distance_condition(
            "main_story", "act1", "mg1", "maneuver1",
            &format!("event_{:?}", dist_type),
            "ego", "target", 20.0, Rule::LessThan, *dist_type, false,
        );
        
        assert!(result.is_ok(), "Failed for distance type: {:?}", dist_type);
    }
}

#[test]
fn test_relative_distance_condition_with_edge_and_delay() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(50.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Test with edge and delay
    let result = scenario.add_event_with_relative_distance_condition_advanced(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "target", 10.0, Rule::LessThan,
        RelativeDistanceType::Euclidean, false,
        ConditionEdge::Rising, 0.5,  // edge + delay
        CoordinateSystem::Entity,    // coordinate system
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_relative_distance_condition_xml_export() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(50.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    scenario.add_event_with_relative_distance_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "target", 10.0, Rule::LessThan,
        RelativeDistanceType::Euclidean, false,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Check for relative distance condition presence
    assert!(xml.contains("<ByEntityCondition>"));
    assert!(xml.contains("<RelativeDistanceCondition"));
    assert!(xml.contains("value=\"10\""));
    assert!(xml.contains("rule=\"lessThan\""));
    assert!(xml.contains("relativeDistanceType=\"euclideanDistance\""));
    assert!(xml.contains("freespace=\"false\""));
    assert!(xml.contains("entityRef=\"target\""));
}

#[test]
fn test_relative_distance_condition_multiple_targets() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target1", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target2", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target1", Position::world(20.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target2", Position::world(40.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Add multiple distance conditions for different targets
    scenario.add_event_with_relative_distance_condition(
        "main_story", "act1", "mg1", "maneuver1", "close_to_target1",
        "ego", "target1", 15.0, Rule::LessThan,
        RelativeDistanceType::Longitudinal, false,
    ).unwrap();
    
    scenario.add_event_with_relative_distance_condition(
        "main_story", "act1", "mg1", "maneuver1", "close_to_target2",
        "ego", "target2", 25.0, Rule::LessThan,
        RelativeDistanceType::Longitudinal, false,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Should contain both conditions
    assert!(xml.contains("close_to_target1"));
    assert!(xml.contains("close_to_target2"));
    assert!(xml.contains("entityRef=\"target1\""));
    assert!(xml.contains("entityRef=\"target2\""));
}
