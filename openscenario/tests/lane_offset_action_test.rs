use openscenario::storyboard::{Action, LaneOffsetAction, TransitionDynamics, DynamicsShape, DynamicsDimension};
use openscenario::{OpenScenarioVersion, Scenario};

#[test]
fn test_lane_offset_action_creation() {
    let action = LaneOffsetAction {
        target_offset: 1.5,
        continuous: true,
        dynamics: Some(TransitionDynamics {
            shape: DynamicsShape::Linear,
            dimension: DynamicsDimension::Time,
            value: 3.0,
        }),
    };

    assert_eq!(action.target_offset, 1.5);
    assert_eq!(action.continuous, true);
    assert!(action.dynamics.is_some());
}

#[test]
fn test_lane_offset_action_without_dynamics() {
    // Immediate lane offset change (no transition dynamics)
    let action = LaneOffsetAction {
        target_offset: -0.5,
        continuous: false,
        dynamics: None,
    };

    assert_eq!(action.target_offset, -0.5);
    assert_eq!(action.continuous, false);
    assert!(action.dynamics.is_none());
}

#[test]
fn test_lane_offset_action_with_distance_dynamics() {
    let action = LaneOffsetAction {
        target_offset: 2.0,
        continuous: true,
        dynamics: Some(TransitionDynamics {
            shape: DynamicsShape::Cubic,
            dimension: DynamicsDimension::Distance,
            value: 50.0, // 50m transition
        }),
    };

    assert_eq!(action.dynamics.as_ref().unwrap().dimension, DynamicsDimension::Distance);
    assert_eq!(action.dynamics.as_ref().unwrap().value, 50.0);
}

#[test]
fn test_lane_offset_action_in_action_enum() {
    let lane_offset_action = LaneOffsetAction {
        target_offset: 1.0,
        continuous: true,
        dynamics: Some(TransitionDynamics {
            shape: DynamicsShape::Sinusoidal,
            dimension: DynamicsDimension::Time,
            value: 2.0,
        }),
    };

    let action = Action::LaneOffset(lane_offset_action);
    
    match action {
        Action::LaneOffset(a) => {
            assert_eq!(a.target_offset, 1.0);
            assert_eq!(a.continuous, true);
        }
        _ => panic!("Expected LaneOffset action"),
    }
}

#[test]
fn test_add_lane_offset_action_to_scenario() {
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

    // Add lane offset action
    let result = scenario.add_lane_offset_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        1.5,
        true,
        Some(TransitionDynamics {
            shape: DynamicsShape::Linear,
            dimension: DynamicsDimension::Time,
            value: 3.0,
        }),
    );

    assert!(result.is_ok(), "Failed to add lane offset action");
}

#[test]
fn test_lane_offset_action_xml_export() {
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

    // Add story structure and action
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

    scenario
        .add_lane_offset_action(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "event1",
            2.0,
            true,
            Some(TransitionDynamics {
                shape: DynamicsShape::Cubic,
                dimension: DynamicsDimension::Distance,
                value: 40.0,
            }),
        )
        .unwrap();

    // Export to XML
    let xml = scenario.to_xml().unwrap();

    // Verify XML contains LaneOffsetAction elements
    assert!(xml.contains("LateralAction"));
    assert!(xml.contains("LaneOffsetAction"));
    assert!(xml.contains("continuous=\"true\""));
    assert!(xml.contains("value=\"2"));
    assert!(xml.contains("LaneOffsetActionDynamics"));
    assert!(xml.contains("dynamicsDimension=\"distance\""));
    assert!(xml.contains("dynamicsShape=\"cubic\""));
    assert!(xml.contains("value=\"40\""));
}

#[test]
fn test_lane_offset_action_immediate_no_dynamics() {
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

    // Add lane offset action without dynamics (immediate change)
    scenario
        .add_lane_offset_action(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "event1",
            0.5,
            false,
            None, // No dynamics = immediate
        )
        .unwrap();

    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("LaneOffsetAction"));
    assert!(xml.contains("continuous=\"false\""));
    assert!(xml.contains("value=\"0.5\""));
    // Should NOT contain dynamics elements when None
    assert!(!xml.contains("LaneOffsetActionDynamics"));
}
