use openscenario::{
    OpenScenarioVersion, Position, Scenario, ScenarioError,
};
use openscenario::storyboard::{ConditionEdge, Rule};

#[test]
fn test_ttc_condition_basic() {
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
    scenario.set_initial_position("target", Position::world(100.0, 0.0, 0.0, 0.0)).unwrap();
    
    // Add story structure
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Add event with TTC condition
    let result = scenario.add_event_with_ttc_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",          // entity being monitored
        "target",       // entity to check collision with
        2.0,            // TTC value in seconds
        Rule::LessThan, // trigger when TTC < 2.0s
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_ttc_condition_with_edge() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(100.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Test with rising edge and delay
    let result = scenario.add_event_with_ttc_condition_advanced(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        "target",
        1.5,
        Rule::LessThan,
        ConditionEdge::Rising,
        0.5,  // 0.5 second delay
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_ttc_condition_negative_value_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(100.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Negative TTC value should fail
    let result = scenario.add_event_with_ttc_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        "target",
        -1.0,  // invalid
        Rule::LessThan,
    );
    
    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidValue { field, .. }) => {
            assert!(field.contains("ttc"));
        }
        _ => panic!("Expected InvalidValue error"),
    }
}

#[test]
fn test_ttc_condition_zero_value() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(100.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Zero TTC should be valid (immediate collision risk)
    let result = scenario.add_event_with_ttc_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        "target",
        0.0,
        Rule::EqualTo,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_ttc_condition_nonexistent_entity_fails() {
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
    let result = scenario.add_event_with_ttc_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        "nonexistent",  // Not in scenario
        2.0,
        Rule::LessThan,
    );
    
    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidEntityRef { .. }) => (),
        _ => panic!("Expected InvalidEntityRef error"),
    }
}

#[test]
fn test_ttc_condition_nonexistent_source_entity_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("target", Position::world(100.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "target").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Reference non-existent source entity
    let result = scenario.add_event_with_ttc_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "nonexistent",  // Not in scenario
        "target",
        2.0,
        Rule::LessThan,
    );
    
    assert!(result.is_err());
}

#[test]
fn test_ttc_condition_all_rules() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(100.0, 0.0, 0.0, 0.0)).unwrap();
    
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
        let result = scenario.add_event_with_ttc_condition(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            &format!("event_{:?}", rule),
            "ego",
            "target",
            2.0,
            *rule,
        );
        
        assert!(result.is_ok(), "Failed for rule: {:?}", rule);
    }
}

#[test]
fn test_ttc_condition_all_edges() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(100.0, 0.0, 0.0, 0.0)).unwrap();
    
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
        let result = scenario.add_event_with_ttc_condition_advanced(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            &format!("event_{:?}", edge),
            "ego",
            "target",
            2.0,
            Rule::LessThan,
            *edge,
            0.0,
        );
        
        assert!(result.is_ok(), "Failed for edge: {:?}", edge);
    }
}

#[test]
fn test_ttc_condition_xml_export() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(100.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    scenario.add_event_with_ttc_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "target", 2.0, Rule::LessThan,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Check for TTC condition presence
    assert!(xml.contains("<ByEntityCondition>"));
    assert!(xml.contains("<TimeToCollisionCondition"));
    assert!(xml.contains("value=\"2\""));
    assert!(xml.contains("rule=\"lessThan\""));
    assert!(xml.contains("<TimeToCollisionConditionTarget>"));
    assert!(xml.contains("<EntityRef entityRef=\"target\""));
}

#[test]
fn test_ttc_condition_multiple_targets() {
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
    scenario.set_initial_position("target1", Position::world(50.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target2", Position::world(100.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Add multiple TTC conditions for different targets
    scenario.add_event_with_ttc_condition(
        "main_story", "act1", "mg1", "maneuver1", "ttc_target1",
        "ego", "target1", 2.0, Rule::LessThan,
    ).unwrap();
    
    scenario.add_event_with_ttc_condition(
        "main_story", "act1", "mg1", "maneuver1", "ttc_target2",
        "ego", "target2", 3.0, Rule::LessThan,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Should contain both conditions
    assert!(xml.contains("ttc_target1"));
    assert!(xml.contains("ttc_target2"));
    assert!(xml.contains("entityRef=\"target1\""));
    assert!(xml.contains("entityRef=\"target2\""));
}

#[test]
fn test_ttc_condition_with_delay() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(100.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Add condition with 1.5 second delay
    let result = scenario.add_event_with_ttc_condition_advanced(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        "target",
        2.0,
        Rule::LessThan,
        ConditionEdge::Rising,
        1.5,  // delay in seconds
    );
    
    assert!(result.is_ok());
    
    let xml = scenario.to_xml().unwrap();
    assert!(xml.contains("delay=\"1.5\""));
}
