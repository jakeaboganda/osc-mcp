use openscenario::{
    OpenScenarioVersion, Position, Scenario, ScenarioError,
};
use openscenario::storyboard::{ConditionEdge, Rule};

#[test]
fn test_time_headway_condition_basic() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    // Add two vehicles (ego following target)
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("lead_vehicle", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("lead_vehicle", Position::world(30.0, 0.0, 0.0, 0.0)).unwrap();
    
    // Add story structure
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Add event with time headway condition (maintain 2 seconds)
    let result = scenario.add_event_with_time_headway_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",               // entity being monitored
        "lead_vehicle",      // reference entity (vehicle ahead)
        2.0,                 // time headway value in seconds
        Rule::GreaterThan,   // trigger when headway > 2s (too far)
        true,                // freespace: true = measure to nearest point on bounding box
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_time_headway_condition_close_following() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("lead_vehicle", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("lead_vehicle", Position::world(10.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Trigger when following too closely (< 1.5 seconds)
    let result = scenario.add_event_with_time_headway_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "lead_vehicle", 1.5, Rule::LessThan, true,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_time_headway_condition_reference_point() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("lead_vehicle", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("lead_vehicle", Position::world(20.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Freespace: false = measure to reference point (center)
    let result = scenario.add_event_with_time_headway_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "lead_vehicle", 2.0, Rule::EqualTo, false,  // freespace = false
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_time_headway_condition_negative_value_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("lead_vehicle", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("lead_vehicle", Position::world(20.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Negative time headway should fail
    let result = scenario.add_event_with_time_headway_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "lead_vehicle", -2.0, Rule::LessThan, true,
    );
    
    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidValue { field, .. }) => {
            assert!(field.contains("time_headway"));
        }
        _ => panic!("Expected InvalidValue error"),
    }
}

#[test]
fn test_time_headway_condition_zero_value_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("lead_vehicle", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("lead_vehicle", Position::world(20.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Zero time headway should fail (must be positive)
    let result = scenario.add_event_with_time_headway_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "lead_vehicle", 0.0, Rule::LessThan, true,
    );
    
    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidValue { field, .. }) => {
            assert!(field.contains("time_headway"));
        }
        _ => panic!("Expected InvalidValue error"),
    }
}

#[test]
fn test_time_headway_condition_nonexistent_entity_fails() {
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
    
    // Reference non-existent lead vehicle
    let result = scenario.add_event_with_time_headway_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "nonexistent", 2.0, Rule::LessThan, true,
    );
    
    assert!(result.is_err());
}

#[test]
fn test_time_headway_condition_nonexistent_source_entity_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("lead_vehicle", vehicle_params).unwrap();
    scenario.set_initial_position("lead_vehicle", Position::world(20.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Source entity doesn't exist
    let result = scenario.add_event_with_time_headway_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "nonexistent", "lead_vehicle", 2.0, Rule::LessThan, true,
    );
    
    assert!(result.is_err());
}

#[test]
fn test_time_headway_condition_all_rules() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("lead_vehicle", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("lead_vehicle", Position::world(30.0, 0.0, 0.0, 0.0)).unwrap();
    
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
        let result = scenario.add_event_with_time_headway_condition(
            "main_story", "act1", "mg1", "maneuver1",
            &format!("event_{:?}", rule),
            "ego", "lead_vehicle", 2.0, *rule, true,
        );
        
        assert!(result.is_ok(), "Failed for rule: {:?}", rule);
    }
}

#[test]
fn test_time_headway_condition_with_edge_and_delay() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("lead_vehicle", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("lead_vehicle", Position::world(30.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Test with edge and delay
    let result = scenario.add_event_with_time_headway_condition_advanced(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "lead_vehicle", 2.0, Rule::LessThan, true,
        ConditionEdge::Rising, 0.3,  // trigger on rising edge with 0.3s delay
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_time_headway_condition_all_edges() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("lead_vehicle", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("lead_vehicle", Position::world(30.0, 0.0, 0.0, 0.0)).unwrap();
    
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
        let result = scenario.add_event_with_time_headway_condition_advanced(
            "main_story", "act1", "mg1", "maneuver1",
            &format!("event_{:?}", edge),
            "ego", "lead_vehicle", 2.0, Rule::LessThan, true, *edge, 0.0,
        );
        
        assert!(result.is_ok(), "Failed for edge: {:?}", edge);
    }
}

#[test]
fn test_time_headway_condition_xml_export() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("lead_vehicle", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("lead_vehicle", Position::world(30.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    scenario.add_event_with_time_headway_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "lead_vehicle", 2.0, Rule::LessThan, true,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Check for time headway condition presence
    assert!(xml.contains("<ByEntityCondition>"));
    assert!(xml.contains("<TimeHeadwayCondition"));
    assert!(xml.contains("value=\"2\""));
    assert!(xml.contains("rule=\"lessThan\""));
    assert!(xml.contains("freespace=\"true\""));
    assert!(xml.contains("entityRef=\"lead_vehicle\""));
}

#[test]
fn test_time_headway_condition_platooning_scenario() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    
    // Create a platoon: lead -> follower1 -> follower2
    scenario.add_vehicle("lead", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("follower1", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("follower2", vehicle_params).unwrap();
    scenario.set_initial_position("lead", Position::world(100.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("follower1", Position::world(80.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("follower2", Position::world(60.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "follower1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "follower2").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Follower1 maintains 1 second gap to lead
    scenario.add_event_with_time_headway_condition(
        "main_story", "act1", "mg1", "maneuver1", "follower1_maintain_gap",
        "follower1", "lead", 1.0, Rule::LessThan, true,
    ).unwrap();
    
    // Follower2 maintains 1 second gap to follower1
    scenario.add_event_with_time_headway_condition(
        "main_story", "act1", "mg1", "maneuver1", "follower2_maintain_gap",
        "follower2", "follower1", 1.0, Rule::LessThan, true,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Should contain both conditions
    assert!(xml.contains("follower1_maintain_gap"));
    assert!(xml.contains("follower2_maintain_gap"));
}
