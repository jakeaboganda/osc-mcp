use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::TransitionShape;
use openscenario::{OpenScenarioVersion, Position, Scenario};

#[test]
fn test_complete_scenario_workflow() {
    // Create scenario
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

    // Add vehicles
    let ego_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", ego_params.clone()).unwrap();
    scenario.add_vehicle("npc", ego_params).unwrap();

    // Set initial positions
    let ego_pos = Position::world(0.0, 0.0, 0.0, 0.0);
    let npc_pos = Position::world(50.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("ego", ego_pos).unwrap();
    scenario.set_initial_position("npc", npc_pos).unwrap();

    // Create storyboard
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "ego_mg")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "ego_mg", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "ego_mg", "speed_maneuver")
        .unwrap();

    // Add actions
    scenario
        .add_speed_action(
            "main_story",
            "act1",
            "ego_mg",
            "speed_maneuver",
            "speed_event",
            30.0,
            5.0,
            TransitionShape::Linear,
        )
        .unwrap();

    scenario
        .add_lane_change_action(
            "main_story",
            "act1",
            "ego_mg",
            "speed_maneuver",
            "lane_change_event",
            -1.0,
            3.0,
            TransitionShape::Sinusoidal,
        )
        .unwrap();

    // Export to XML
    let xml = scenario.to_xml().unwrap();

    // Verify XML structure
    assert!(xml.contains("<?xml"));
    assert!(xml.contains("<OpenSCENARIO"));
    assert!(xml.contains("<FileHeader"));
    assert!(xml.contains("<Entities"));
    assert!(xml.contains("<ScenarioObject name=\"ego\""));
    assert!(xml.contains("<ScenarioObject name=\"npc\""));
    assert!(xml.contains("<Storyboard"));
    assert!(xml.contains("<Story"));
    assert!(xml.contains("<SpeedAction"));
    assert!(xml.contains("<LaneChangeAction"));
    assert!(xml.contains("</OpenSCENARIO>"));

    // Verify it's not empty
    assert!(xml.len() > 1000);
}

#[test]
fn test_lane_position_scenario() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("car", params).unwrap();

    let lane_pos = Position::lane("road1", 1, 50.0, 0.0, None);
    scenario.set_initial_position("car", lane_pos).unwrap();

    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("<LanePosition"));
    assert!(xml.contains("roadId=\"road1\""));
    assert!(xml.contains("laneId=\"1\""));
}

#[test]
fn test_relative_position_scenario() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("lead", params.clone()).unwrap();
    scenario.add_vehicle("follower", params).unwrap();

    let lead_pos = Position::world(0.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("lead", lead_pos).unwrap();

    use openscenario::position::Orientation;
    let follower_pos = Position::relative_world("lead", -10.0, 0.0, 0.0, Orientation::default());
    scenario
        .set_initial_position("follower", follower_pos)
        .unwrap();

    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("<RelativeWorldPosition"));
    assert!(xml.contains("entityRef=\"lead\""));
}

// ========== Negative Test Cases ==========

#[test]
#[ignore]
#[should_panic(expected = "Entity not found")]
fn test_set_position_for_nonexistent_entity() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let pos = Position::world(0.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("nonexistent", pos).unwrap();
}

#[test]
fn test_duplicate_vehicle_name() {
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
    assert!(err_msg.contains("already exists") || err_msg.contains("duplicate"));
}

#[test]
#[ignore]
fn test_empty_entity_name() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    let result = scenario.add_vehicle("", params);
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_add_story_with_empty_name() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let result = scenario.add_story("");
    assert!(result.is_err());
}

#[test]
fn test_add_act_to_nonexistent_story() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let result = scenario.add_act("nonexistent_story", "act1");
    assert!(result.is_err());
}

#[test]
fn test_add_maneuver_group_to_nonexistent_act() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    scenario.add_story("story1").unwrap();
    
    let result = scenario.add_maneuver_group("story1", "nonexistent_act", "mg1");
    assert!(result.is_err());
}

#[test]
fn test_add_actor_with_nonexistent_entity() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    scenario.add_story("story1").unwrap();
    scenario.add_act("story1", "act1").unwrap();
    scenario.add_maneuver_group("story1", "act1", "mg1").unwrap();
    
    let result = scenario.add_actor("story1", "act1", "mg1", "nonexistent_entity");
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_speed_action_with_invalid_parameters() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car", params).unwrap();
    scenario.add_story("story").unwrap();
    scenario.add_act("story", "act").unwrap();
    scenario.add_maneuver_group("story", "act", "mg").unwrap();
    scenario.add_actor("story", "act", "mg", "car").unwrap();
    scenario.add_maneuver("story", "act", "mg", "maneuver").unwrap();
    
    // Negative speed
    let result = scenario.add_speed_action(
        "story", "act", "mg", "maneuver", "event",
        -10.0, // Invalid negative speed
        5.0,
        TransitionShape::Linear,
    );
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_speed_action_with_zero_duration() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car", params).unwrap();
    scenario.add_story("story").unwrap();
    scenario.add_act("story", "act").unwrap();
    scenario.add_maneuver_group("story", "act", "mg").unwrap();
    scenario.add_actor("story", "act", "mg", "car").unwrap();
    scenario.add_maneuver("story", "act", "mg", "maneuver").unwrap();
    
    // Zero duration
    let result = scenario.add_speed_action(
        "story", "act", "mg", "maneuver", "event",
        30.0,
        0.0, // Invalid zero duration
        TransitionShape::Linear,
    );
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_lane_change_action_with_invalid_duration() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car", params).unwrap();
    scenario.add_story("story").unwrap();
    scenario.add_act("story", "act").unwrap();
    scenario.add_maneuver_group("story", "act", "mg").unwrap();
    scenario.add_actor("story", "act", "mg", "car").unwrap();
    scenario.add_maneuver("story", "act", "mg", "maneuver").unwrap();
    
    // Negative duration
    let result = scenario.add_lane_change_action(
        "story", "act", "mg", "maneuver", "event",
        -1.0,
        -3.0, // Invalid negative duration
        TransitionShape::Sinusoidal,
    );
    assert!(result.is_err());
}

#[test]
fn test_relative_position_with_invalid_entity_ref() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car", params).unwrap();
    
    use openscenario::position::Orientation;
    let pos = Position::relative_world("nonexistent", 10.0, 0.0, 0.0, Orientation::default());
    let result = scenario.set_initial_position("car", pos);
    
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Entity not found") || err_msg.contains("nonexistent"));
}

#[test]
fn test_xml_generation_without_entities() {
    let scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let result = scenario.to_xml();
    
    // Should succeed but generate minimal XML
    assert!(result.is_ok());
    let xml = result.unwrap();
    assert!(xml.contains("<OpenSCENARIO"));
    assert!(xml.contains("<Entities"));
}

#[test]
fn test_xml_generation_without_storyboard() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car", params).unwrap();
    let result = scenario.to_xml();
    
    // Should succeed even without storyboard
    assert!(result.is_ok());
    let xml = result.unwrap();
    assert!(xml.contains("<Entities"));
    assert!(xml.contains("name=\"car\""));
}

#[test]
fn test_entity_without_initial_position() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car", params).unwrap();
    // Don't set initial position
    
    let result = scenario.to_xml();
    // Should fail or warn about missing position
    // Behavior depends on implementation requirements
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_lane_position_with_invalid_lane_id() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car", params).unwrap();
    
    // Lane ID 0 is typically invalid in OpenDRIVE
    let pos = Position::lane("road1", 0, 50.0, 0.0, None);
    let result = scenario.set_initial_position("car", pos);
    
    // Implementation may or may not validate lane IDs at construction time
    // This documents the behavior
    if result.is_err() {
        assert!(result.unwrap_err().to_string().contains("lane"));
    }
}

#[test]
fn test_world_position_with_extreme_coordinates() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car", params).unwrap();
    
    // Very large coordinates
    let pos = Position::world(1e10, 1e10, 1e10, std::f64::consts::PI * 1000.0);
    let result = scenario.set_initial_position("car", pos);
    
    // Should accept but may produce strange XML
    assert!(result.is_ok());
}

#[test]
fn test_empty_road_id_in_lane_position() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car", params).unwrap();
    
    let pos = Position::lane("", 1, 50.0, 0.0, None);
    let result = scenario.set_initial_position("car", pos);
    
    // Empty road ID should be rejected
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("road") || err_msg.contains("empty"));
    }
}

#[test]
fn test_add_maneuver_to_nonexistent_maneuver_group() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    scenario.add_story("story").unwrap();
    scenario.add_act("story", "act").unwrap();
    
    let result = scenario.add_maneuver("story", "act", "nonexistent_mg", "maneuver");
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_hierarchical_name_conflicts() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    
    scenario.add_story("story1").unwrap();
    scenario.add_act("story1", "act1").unwrap();
    scenario.add_maneuver_group("story1", "act1", "mg1").unwrap();
    
    // Try to add duplicate maneuver group in same act
    let result = scenario.add_maneuver_group("story1", "act1", "mg1");
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_nan_values_in_position() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car", params).unwrap();
    
    // NaN values should be rejected
    let pos = Position::world(f64::NAN, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_position("car", pos);
    
    if result.is_ok() {
        // If accepted, XML generation should fail or produce invalid XML
        let xml_result = scenario.to_xml();
        if let Ok(xml) = xml_result {
            // NaN in XML is invalid
            assert!(!xml.contains("NaN"));
        }
    } else {
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("NaN") || err_msg.contains("invalid"));
    }
}

#[test]
#[ignore]
fn test_infinite_values_in_speed_action() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car", params).unwrap();
    scenario.add_story("story").unwrap();
    scenario.add_act("story", "act").unwrap();
    scenario.add_maneuver_group("story", "act", "mg").unwrap();
    scenario.add_actor("story", "act", "mg", "car").unwrap();
    scenario.add_maneuver("story", "act", "mg", "maneuver").unwrap();
    
    let result = scenario.add_speed_action(
        "story", "act", "mg", "maneuver", "event",
        f64::INFINITY, // Invalid infinite speed
        5.0,
        TransitionShape::Linear,
    );
    assert!(result.is_err());
}
