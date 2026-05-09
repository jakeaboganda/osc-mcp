use openscenario::storyboard::{Action, FollowTrajectoryAction, Trajectory, Vertex, TimingMode};
use openscenario::{OpenScenarioVersion, Scenario};

#[test]
fn test_trajectory_vertex_creation() {
    let vertex = Vertex {
        time: 0.0,
        position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
    };

    assert_eq!(vertex.time, 0.0);
}

#[test]
fn test_trajectory_creation() {
    let trajectory = Trajectory {
        name: "test_trajectory".to_string(),
        closed: false,
        vertices: vec![
            Vertex {
                time: 0.0,
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                time: 1.0,
                position: openscenario::Position::world(10.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                time: 2.0,
                position: openscenario::Position::world(20.0, 10.0, 0.0, 0.0),
            },
        ],
    };

    assert_eq!(trajectory.name, "test_trajectory");
    assert_eq!(trajectory.closed, false);
    assert_eq!(trajectory.vertices.len(), 3);
}

#[test]
fn test_follow_trajectory_action_creation() {
    let trajectory = Trajectory {
        name: "path1".to_string(),
        closed: false,
        vertices: vec![
            Vertex {
                time: 0.0,
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                time: 5.0,
                position: openscenario::Position::world(50.0, 0.0, 0.0, 0.0),
            },
        ],
    };

    let action = FollowTrajectoryAction {
        trajectory,
        timing_mode: TimingMode::Timing,
        initial_distance_offset: None,
    };

    assert_eq!(action.trajectory.name, "path1");
    assert_eq!(action.timing_mode, TimingMode::Timing);
    assert!(action.initial_distance_offset.is_none());
}

#[test]
fn test_follow_trajectory_action_with_offset() {
    let trajectory = Trajectory {
        name: "path2".to_string(),
        closed: false,
        vertices: vec![
            Vertex {
                time: 0.0,
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                time: 3.0,
                position: openscenario::Position::world(30.0, 0.0, 0.0, 0.0),
            },
        ],
    };

    let action = FollowTrajectoryAction {
        trajectory,
        timing_mode: TimingMode::None,
        initial_distance_offset: Some(5.0),
    };

    assert_eq!(action.timing_mode, TimingMode::None);
    assert_eq!(action.initial_distance_offset, Some(5.0));
}

#[test]
fn test_timing_mode_variants() {
    assert_eq!(TimingMode::Timing, TimingMode::Timing);
    assert_eq!(TimingMode::None, TimingMode::None);
}

#[test]
fn test_follow_trajectory_action_in_action_enum() {
    let trajectory = Trajectory {
        name: "test".to_string(),
        closed: false,
        vertices: vec![
            Vertex {
                time: 0.0,
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
            },
        ],
    };

    let follow_trajectory = FollowTrajectoryAction {
        trajectory,
        timing_mode: TimingMode::Timing,
        initial_distance_offset: None,
    };

    let action = Action::FollowTrajectory(follow_trajectory);

    match action {
        Action::FollowTrajectory(a) => {
            assert_eq!(a.trajectory.name, "test");
            assert_eq!(a.timing_mode, TimingMode::Timing);
        }
        _ => panic!("Expected FollowTrajectory action"),
    }
}

#[test]
fn test_add_follow_trajectory_action_to_scenario() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add entity
    scenario
        .add_vehicle(
            "ego",
            openscenario::entities::VehicleParams {
                catalog: None,
                vehicle_category: openscenario::entities::VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    // Set initial position
    scenario
        .set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    // Add story structure
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    // Create trajectory
    let trajectory = Trajectory {
        name: "highway_path".to_string(),
        closed: false,
        vertices: vec![
            Vertex {
                time: 0.0,
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                time: 5.0,
                position: openscenario::Position::world(50.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                time: 10.0,
                position: openscenario::Position::world(100.0, 20.0, 0.0, 0.0),
            },
        ],
    };

    // Add follow trajectory action
    let result = scenario.add_follow_trajectory_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        trajectory,
        TimingMode::Timing,
        None,
    );

    assert!(result.is_ok(), "Failed to add follow trajectory action");
}

#[test]
fn test_follow_trajectory_action_xml_export() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add entity
    scenario
        .add_vehicle(
            "ego",
            openscenario::entities::VehicleParams {
                catalog: None,
                vehicle_category: openscenario::entities::VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    scenario
        .set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();

    let trajectory = Trajectory {
        name: "test_path".to_string(),
        closed: false,
        vertices: vec![
            Vertex {
                time: 0.0,
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                time: 2.0,
                position: openscenario::Position::world(20.0, 5.0, 0.0, 0.0),
            },
            Vertex {
                time: 4.0,
                position: openscenario::Position::world(40.0, 10.0, 0.0, 0.5),
            },
        ],
    };

    scenario
        .add_follow_trajectory_action(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "event1",
            trajectory,
            TimingMode::Timing,
            Some(2.5),
        )
        .unwrap();

    let xml = scenario.to_xml().unwrap();

    // Verify XML contains trajectory elements
    assert!(xml.contains("RoutingAction"));
    assert!(xml.contains("FollowTrajectoryAction"));
    assert!(xml.contains("Trajectory"));
    assert!(xml.contains("name=\"test_path\""));
    assert!(xml.contains("closed=\"false\""));
    assert!(xml.contains("Polyline"));
    assert!(xml.contains("Vertex"));
    assert!(xml.contains("time=\"0\""));
    assert!(xml.contains("time=\"2\""));
    assert!(xml.contains("time=\"4\""));
    assert!(xml.contains("WorldPosition"));
    assert!(xml.contains("initialDistanceOffset=\"2.5\""));
}

#[test]
fn test_closed_trajectory() {
    let trajectory = Trajectory {
        name: "loop".to_string(),
        closed: true,
        vertices: vec![
            Vertex {
                time: 0.0,
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                time: 1.0,
                position: openscenario::Position::world(10.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                time: 2.0,
                position: openscenario::Position::world(10.0, 10.0, 0.0, 0.0),
            },
            Vertex {
                time: 3.0,
                position: openscenario::Position::world(0.0, 10.0, 0.0, 0.0),
            },
        ],
    };

    assert_eq!(trajectory.closed, true);
    assert_eq!(trajectory.vertices.len(), 4);
}

#[test]
fn test_trajectory_validation_minimum_vertices() {
    // A trajectory needs at least 2 vertices
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    scenario
        .add_vehicle(
            "ego",
            openscenario::entities::VehicleParams {
                catalog: None,
                vehicle_category: openscenario::entities::VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    scenario
        .set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();

    scenario.add_story("story").unwrap();
    scenario.add_act("story", "act").unwrap();
    scenario.add_maneuver_group("story", "act", "mg").unwrap();
    scenario.add_actor("story", "act", "mg", "ego").unwrap();
    scenario.add_maneuver("story", "act", "mg", "maneuver").unwrap();

    // Single vertex trajectory (invalid)
    let invalid_trajectory = Trajectory {
        name: "invalid".to_string(),
        closed: false,
        vertices: vec![Vertex {
            time: 0.0,
            position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
        }],
    };

    let result = scenario.add_follow_trajectory_action(
        "story",
        "act",
        "mg",
        "maneuver",
        "event",
        invalid_trajectory,
        TimingMode::Timing,
        None,
    );

    assert!(result.is_err(), "Should reject trajectory with < 2 vertices");
}
