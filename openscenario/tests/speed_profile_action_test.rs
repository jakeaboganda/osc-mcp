use openscenario::{
    OpenScenarioVersion, Position, Scenario, ScenarioError,
};

#[test]
fn test_speed_profile_action_basic() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    // Add vehicle
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
    
    // Add speed profile with waypoints
    let speed_waypoints = vec![
        (0.0, 10.0),    // (time/distance, speed)
        (5.0, 15.0),
        (10.0, 20.0),
    ];
    
    let result = scenario.add_speed_profile_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        speed_waypoints,
        true,  // following_mode: true = time-based, false = distance-based
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_speed_profile_action_distance_based() {
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
    
    // Distance-based speed profile
    let speed_waypoints = vec![
        (0.0, 20.0),      // At 0m: 20 m/s
        (100.0, 25.0),    // At 100m: 25 m/s
        (200.0, 15.0),    // At 200m: 15 m/s (deceleration)
    ];
    
    let result = scenario.add_speed_profile_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        speed_waypoints,
        false,  // distance-based
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_speed_profile_action_empty_waypoints_fails() {
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
    
    // Empty waypoints should fail
    let result = scenario.add_speed_profile_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        vec![],  // empty
        true,
    );
    
    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidValue { field, .. }) => {
            assert!(field.contains("waypoints") || field.contains("profile"));
        }
        _ => panic!("Expected InvalidValue error"),
    }
}

#[test]
fn test_speed_profile_action_single_waypoint() {
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
    
    // Single waypoint should be valid (constant speed)
    let result = scenario.add_speed_profile_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        vec![(0.0, 15.0)],
        true,
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_speed_profile_action_negative_speed_fails() {
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
    
    // Negative speed should fail
    let speed_waypoints = vec![
        (0.0, 10.0),
        (5.0, -5.0),  // negative!
    ];
    
    let result = scenario.add_speed_profile_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        speed_waypoints,
        true,
    );
    
    assert!(result.is_err());
}

#[test]
fn test_speed_profile_action_negative_position_fails() {
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
    
    // Negative time/distance should fail
    let speed_waypoints = vec![
        (-1.0, 10.0),  // negative position!
        (5.0, 15.0),
    ];
    
    let result = scenario.add_speed_profile_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        speed_waypoints,
        true,
    );
    
    assert!(result.is_err());
}

#[test]
fn test_speed_profile_action_xml_export() {
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
    
    let speed_waypoints = vec![
        (0.0, 10.0),
        (5.0, 20.0),
    ];
    
    scenario.add_speed_profile_action(
        "main_story", "act1", "mg1", "maneuver1", "event1",
        speed_waypoints, true,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Check for speed profile action presence
    assert!(xml.contains("<LongitudinalAction>"));
    assert!(xml.contains("<SpeedProfileAction"));
}

#[test]
fn test_speed_profile_action_multiple_profiles() {
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
    
    // Add multiple speed profiles to different events
    scenario.add_speed_profile_action(
        "main_story", "act1", "mg1", "maneuver1", "profile1",
        vec![(0.0, 10.0), (5.0, 20.0)], true,
    ).unwrap();
    
    scenario.add_speed_profile_action(
        "main_story", "act1", "mg1", "maneuver1", "profile2",
        vec![(0.0, 15.0), (10.0, 25.0)], false,
    ).unwrap();
    
    let xml = scenario.to_xml().unwrap();
    
    // Should contain both profiles
    assert!(xml.contains("profile1"));
    assert!(xml.contains("profile2"));
}
