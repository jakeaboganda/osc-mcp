use openscenario::{
    OpenScenarioVersion, Position, Scenario, ScenarioError,
};
use openscenario::storyboard::{DynamicsDimension, DynamicsShape, TransitionDynamics};

#[test]
fn test_acceleration_action_basic() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    // Add vehicle
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    
    // Set position
    let pos = Position::world(0.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("ego", pos).unwrap();
    
    // Add story structure
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Add acceleration action with simple parameters
    let result = scenario.add_acceleration_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        5.0,  // acceleration (m/s²)
        3.0,  // duration (seconds)
        None, // use default linear dynamics
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_acceleration_action_with_dynamics() {
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
    
    // Custom dynamics
    let dynamics = TransitionDynamics {
        shape: DynamicsShape::Cubic,
        dimension: DynamicsDimension::Time,
        value: 5.0,
    };
    
    let result = scenario.add_acceleration_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        -9.81,  // emergency braking
        5.0,
        Some(dynamics),
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_acceleration_action_negative_deceleration() {
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
    
    // Negative acceleration (deceleration) should be valid
    let result = scenario.add_acceleration_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        -5.0,  // deceleration
        3.0,
        None,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_acceleration_action_zero_duration_fails() {
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
    
    // Zero or negative duration should fail
    let result = scenario.add_acceleration_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        5.0,
        0.0,  // invalid duration
        None,
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
fn test_acceleration_action_negative_duration_fails() {
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
    
    let result = scenario.add_acceleration_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        5.0,
        -1.0,  // invalid duration
        None,
    );
    
    assert!(result.is_err());
}

#[test]
fn test_acceleration_action_extreme_values() {
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
    
    // Very high acceleration (edge case, but valid)
    let result = scenario.add_acceleration_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        100.0,  // extreme acceleration
        0.1,    // very short duration
        None,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_acceleration_action_missing_story_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    
    // Try to add action without story structure
    let result = scenario.add_acceleration_action(
        "nonexistent_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        5.0,
        3.0,
        None,
    );
    
    assert!(result.is_err());
    match result {
        Err(ScenarioError::StoryNotFound { .. }) => (),
        _ => panic!("Expected StoryNotFound error"),
    }
}

#[test]
fn test_acceleration_action_xml_export() {
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
    
    scenario.add_acceleration_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        5.0,
        3.0,
        None,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Debug: print XML to see actual output
    println!("Generated XML:\n{}", xml);
    
    // Check for AccelerationAction presence (exported as SpeedAction with rate)
    assert!(xml.contains("<LongitudinalAction>"));
    assert!(xml.contains("<SpeedAction>"));
    assert!(xml.contains("<SpeedActionDynamics"));
    assert!(xml.contains("dynamicsDimension=\"rate\""));  // Acceleration uses rate dimension
    assert!(xml.contains("value=\"5"));  // acceleration value
    
    // Check for valid SpeedActionTarget (must not be empty per spec)
    assert!(xml.contains("<SpeedActionTarget>"));
    assert!(xml.contains("<RelativeTargetSpeed"));
    assert!(xml.contains("continuous=\"true\""));  // Continuous acceleration
}

#[test]
fn test_acceleration_action_multiple_actions_same_entity() {
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
    
    // Add multiple acceleration actions
    scenario.add_acceleration_action(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        5.0, 3.0, None,
    ).unwrap();
    
    scenario.add_acceleration_action(
        "main_story", "act1", "mg1", "maneuver1", "event2",
        -3.0, 2.0, None,
    ).unwrap();
    
    scenario.add_acceleration_action(
        "main_story", "act1", "mg1", "maneuver1", "event3",
        2.0, 5.0, None,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Should contain all three events
    assert!(xml.contains("event1"));
    assert!(xml.contains("event2"));
    assert!(xml.contains("event3"));
}

#[test]
fn test_acceleration_action_with_different_dynamics_shapes() {
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
    
    // Test all dynamics shapes
    let shapes = vec![
        DynamicsShape::Linear,
        DynamicsShape::Cubic,
        DynamicsShape::Sinusoidal,
        DynamicsShape::Step,
    ];
    
    for (i, shape) in shapes.iter().enumerate() {
        let dynamics = TransitionDynamics {
            shape: *shape,
            dimension: DynamicsDimension::Time,
            value: 3.0,
        };
        
        let result = scenario.add_acceleration_action(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            &format!("event_{}", i),
            5.0,
            3.0,
            Some(dynamics),
        );
        
        assert!(result.is_ok(), "Failed for shape: {:?}", shape);
    }
}

#[test]
fn test_acceleration_action_with_different_dimensions() {
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
    
    // Test all dimensions
    let dimensions = vec![
        DynamicsDimension::Time,
        DynamicsDimension::Distance,
        DynamicsDimension::Rate,
    ];
    
    for (i, dimension) in dimensions.iter().enumerate() {
        let dynamics = TransitionDynamics {
            shape: DynamicsShape::Linear,
            dimension: *dimension,
            value: 3.0,
        };
        
        let result = scenario.add_acceleration_action(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            &format!("event_{}", i),
            5.0,
            3.0,
            Some(dynamics),
        );
        
        assert!(result.is_ok(), "Failed for dimension: {:?}", dimension);
    }
}

#[test]
fn test_acceleration_action_invalid_dynamics_value_fails() {
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
    
    // Custom dynamics with invalid (negative) value
    let dynamics = TransitionDynamics {
        shape: DynamicsShape::Linear,
        dimension: DynamicsDimension::Time,
        value: -1.0,  // Invalid
    };
    
    let result = scenario.add_acceleration_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        5.0,
        3.0,
        Some(dynamics),
    );
    
    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidValue { field, .. }) => {
            assert!(field.contains("dynamics"));
        }
        _ => panic!("Expected InvalidValue error for dynamics.value"),
    }
}

#[test]
fn test_acceleration_action_zero_dynamics_value_fails() {
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
    
    // Custom dynamics with zero value
    let dynamics = TransitionDynamics {
        shape: DynamicsShape::Linear,
        dimension: DynamicsDimension::Time,
        value: 0.0,  // Invalid
    };
    
    let result = scenario.add_acceleration_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        5.0,
        3.0,
        Some(dynamics),
    );
    
    assert!(result.is_err());
}
