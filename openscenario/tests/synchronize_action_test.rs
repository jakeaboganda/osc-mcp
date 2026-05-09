use openscenario::storyboard::{Action, SynchronizeAction, TargetPosition, TargetPositionMaster, TargetPositionRelative};
use openscenario::{OpenScenarioVersion, Scenario};

#[test]
fn test_synchronize_action_with_master_position() {
    let action = SynchronizeAction {
        entity_ref: "follower".to_string(),
        master_entity_ref: "leader".to_string(),
        target_position_master: TargetPositionMaster {
            position: openscenario::Position::world(100.0, 0.0, 0.0, 0.0),
        },
        target_position: TargetPosition::Relative(TargetPositionRelative {
            entity_ref: "leader".to_string(),
            dx: -10.0,
            dy: 0.0,
            dz: 0.0,
        }),
        final_speed: Some(30.0),
    };

    assert_eq!(action.entity_ref, "follower");
    assert_eq!(action.master_entity_ref, "leader");
    assert_eq!(action.final_speed, Some(30.0));
}

#[test]
fn test_synchronize_action_with_relative_position() {
    let action = SynchronizeAction {
        entity_ref: "car2".to_string(),
        master_entity_ref: "car1".to_string(),
        target_position_master: TargetPositionMaster {
            position: openscenario::Position::world(50.0, 10.0, 0.0, 0.0),
        },
        target_position: TargetPosition::Relative(TargetPositionRelative {
            entity_ref: "car1".to_string(),
            dx: -5.0,
            dy: 2.0,
            dz: 0.0,
        }),
        final_speed: None,
    };

    assert_eq!(action.entity_ref, "car2");
    match action.target_position {
        TargetPosition::Relative(rel) => {
            assert_eq!(rel.dx, -5.0);
            assert_eq!(rel.dy, 2.0);
        }
        _ => panic!("Expected Relative target position"),
    }
}

#[test]
fn test_synchronize_action_with_world_position() {
    let action = SynchronizeAction {
        entity_ref: "ego".to_string(),
        master_entity_ref: "target".to_string(),
        target_position_master: TargetPositionMaster {
            position: openscenario::Position::world(200.0, 50.0, 0.0, 0.0),
        },
        target_position: TargetPosition::World(openscenario::Position::world(180.0, 45.0, 0.0, 0.0)),
        final_speed: Some(25.0),
    };

    match action.target_position {
        TargetPosition::World(_) => {
            // Success
        }
        _ => panic!("Expected World target position"),
    }
}

#[test]
fn test_synchronize_action_in_action_enum() {
    let sync_action = SynchronizeAction {
        entity_ref: "follower".to_string(),
        master_entity_ref: "leader".to_string(),
        target_position_master: TargetPositionMaster {
            position: openscenario::Position::world(100.0, 0.0, 0.0, 0.0),
        },
        target_position: TargetPosition::Relative(TargetPositionRelative {
            entity_ref: "leader".to_string(),
            dx: -10.0,
            dy: 0.0,
            dz: 0.0,
        }),
        final_speed: Some(30.0),
    };

    let action = Action::Synchronize(sync_action);

    match action {
        Action::Synchronize(a) => {
            assert_eq!(a.entity_ref, "follower");
            assert_eq!(a.master_entity_ref, "leader");
        }
        _ => panic!("Expected Synchronize action"),
    }
}

#[test]
fn test_add_synchronize_action_to_scenario() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add entities
    scenario
        .add_vehicle(
            "leader",
            openscenario::entities::VehicleParams {
                catalog: None,
                vehicle_category: openscenario::entities::VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    scenario
        .add_vehicle(
            "follower",
            openscenario::entities::VehicleParams {
                catalog: None,
                vehicle_category: openscenario::entities::VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    scenario
        .set_initial_position("leader", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario
        .set_initial_position("follower", openscenario::Position::world(-10.0, 0.0, 0.0, 0.0))
        .unwrap();

    // Add story structure
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "follower")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    let result = scenario.add_synchronize_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "follower",
        "leader",
        TargetPositionMaster {
            position: openscenario::Position::world(100.0, 0.0, 0.0, 0.0),
        },
        TargetPosition::Relative(TargetPositionRelative {
            entity_ref: "leader".to_string(),
            dx: -10.0,
            dy: 0.0,
            dz: 0.0,
        }),
        Some(30.0),
    );

    assert!(result.is_ok(), "Failed to add synchronize action");
}

#[test]
fn test_synchronize_action_xml_export() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    scenario
        .add_vehicle(
            "leader",
            openscenario::entities::VehicleParams {
                catalog: None,
                vehicle_category: openscenario::entities::VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    scenario
        .add_vehicle(
            "follower",
            openscenario::entities::VehicleParams {
                catalog: None,
                vehicle_category: openscenario::entities::VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    scenario
        .set_initial_position("leader", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario
        .set_initial_position("follower", openscenario::Position::world(-10.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "follower")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    scenario
        .add_synchronize_action(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "event1",
            "follower",
            "leader",
            TargetPositionMaster {
                position: openscenario::Position::world(100.0, 0.0, 0.0, 0.5),
            },
            TargetPosition::Relative(TargetPositionRelative {
                entity_ref: "leader".to_string(),
                dx: -10.0,
                dy: 2.0,
                dz: 0.0,
            }),
            Some(25.0),
        )
        .unwrap();

    let xml = scenario.to_xml().unwrap();

    // Verify XML contains synchronize elements
    assert!(xml.contains("SynchronizeAction"));
    assert!(xml.contains("entityRef=\"follower\""));
    assert!(xml.contains("entityRef=\"leader\""));
    assert!(xml.contains("TargetPositionMaster"));
    assert!(xml.contains("TargetPosition"));
    assert!(xml.contains("RelativeWorldPosition"));
    assert!(xml.contains("dx=\"-10\""));
    assert!(xml.contains("dy=\"2\""));
    assert!(xml.contains("FinalSpeed"));
    assert!(xml.contains("value=\"25\""));
}

#[test]
fn test_synchronize_action_validation_entity_refs() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    scenario
        .add_vehicle(
            "car1",
            openscenario::entities::VehicleParams {
                catalog: None,
                vehicle_category: openscenario::entities::VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    scenario
        .set_initial_position("car1", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("story").unwrap();
    scenario.add_act("story", "act").unwrap();
    scenario.add_maneuver_group("story", "act", "mg").unwrap();
    scenario.add_actor("story", "act", "mg", "car1").unwrap();
    scenario.add_maneuver("story", "act", "mg", "maneuver").unwrap();

    // Try to synchronize with non-existent master entity
    let result = scenario.add_synchronize_action(
        "story",
        "act",
        "mg",
        "maneuver",
        "event",
        "car1",
        "nonexistent_master",
        TargetPositionMaster {
            position: openscenario::Position::world(50.0, 0.0, 0.0, 0.0),
        },
        TargetPosition::Relative(TargetPositionRelative {
            entity_ref: "nonexistent_master".to_string(),
            dx: -5.0,
            dy: 0.0,
            dz: 0.0,
        }),
        None,
    );

    assert!(result.is_err(), "Should reject non-existent master entity");
}
