use openscenario::{
    OpenScenarioVersion, Position, Scenario, ScenarioError,
};
use openscenario::storyboard::ConditionEdge;

#[test]
fn test_collision_condition_basic() {
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
    scenario.set_initial_position("target", Position::world(5.0, 0.0, 0.0, 0.0)).unwrap();
    
    // Add story structure
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Add event with collision condition
    let result = scenario.add_event_with_collision_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",          // entity being monitored
        "target",       // entity to detect collision with
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_collision_condition_with_edge() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(5.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Test with rising edge and delay
    let result = scenario.add_event_with_collision_condition_advanced(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        "target",
        ConditionEdge::Rising,
        0.5,  // 0.5 second delay
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_collision_condition_nonexistent_entity_fails() {
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
    let result = scenario.add_event_with_collision_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        "nonexistent",  // Not in scenario
    );
    
    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidEntityRef { .. }) => (),
        _ => panic!("Expected InvalidEntityRef error"),
    }
}

#[test]
fn test_collision_condition_nonexistent_source_entity_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("target", Position::world(5.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "target").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Reference non-existent source entity
    let result = scenario.add_event_with_collision_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "nonexistent",  // Not in scenario
        "target",
    );
    
    assert!(result.is_err());
}

#[test]
fn test_collision_condition_all_edges() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(5.0, 0.0, 0.0, 0.0)).unwrap();
    
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
        let result = scenario.add_event_with_collision_condition_advanced(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            &format!("event_{:?}", edge),
            "ego",
            "target",
            *edge,
            0.0,
        );
        
        assert!(result.is_ok(), "Failed for edge: {:?}", edge);
    }
}

#[test]
fn test_collision_condition_xml_export() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(5.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    scenario.add_event_with_collision_condition(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        "ego", "target",
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Check for collision condition presence
    assert!(xml.contains("<ByEntityCondition>"));
    assert!(xml.contains("<CollisionCondition"));
    assert!(xml.contains("<EntityRef entityRef=\"target\""));
}

#[test]
fn test_collision_condition_multiple_targets() {
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
    scenario.set_initial_position("target1", Position::world(5.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target2", Position::world(10.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Add multiple collision conditions for different targets
    scenario.add_event_with_collision_condition(
        "main_story", "act1", "mg1", "maneuver1", "collision_target1",
        "ego", "target1",
    ).unwrap();
    
    scenario.add_event_with_collision_condition(
        "main_story", "act1", "mg1", "maneuver1", "collision_target2",
        "ego", "target2",
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Should contain both conditions
    assert!(xml.contains("collision_target1"));
    assert!(xml.contains("collision_target2"));
    assert!(xml.contains("entityRef=\"target1\""));
    assert!(xml.contains("entityRef=\"target2\""));
}

#[test]
fn test_collision_condition_with_delay() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let vehicle_params = openscenario::entities::VehicleParams {
        catalog: None,
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params.clone()).unwrap();
    scenario.add_vehicle("target", vehicle_params).unwrap();
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0)).unwrap();
    scenario.set_initial_position("target", Position::world(5.0, 0.0, 0.0, 0.0)).unwrap();
    
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario.add_maneuver_group("main_story", "act1", "mg1").unwrap();
    scenario.add_actor("main_story", "act1", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1").unwrap();
    
    // Add condition with 1.0 second delay
    let result = scenario.add_event_with_collision_condition_advanced(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        "target",
        ConditionEdge::Rising,
        1.0,  // delay in seconds
    );
    
    assert!(result.is_ok());
    
    let xml = scenario.to_xml().unwrap();
    assert!(xml.contains("delay=\"1\""));
}

#[test]
fn test_collision_condition_same_entity_collision() {
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
    
    // Collision with self should be allowed (simulator may handle as no-op)
    let result = scenario.add_event_with_collision_condition(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "ego",
        "ego",  // same entity
    );
    
    // Some implementations might reject self-collision, others allow it
    // For now, allow it and let simulator handle semantics
    assert!(result.is_ok());
}
