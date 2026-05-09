use openscenario::storyboard::{Action, SpeedAction, TransitionDynamics, DynamicsShape, DynamicsDimension, TransitionShape};
use openscenario::{OpenScenarioVersion, Scenario};

#[test]
fn test_speed_action_with_dynamics() {
    // Test SpeedAction with new dynamics structure
    let action = SpeedAction {
        target_speed: 30.0,
        dynamics: TransitionDynamics {
            shape: DynamicsShape::Linear,
            dimension: DynamicsDimension::Time,
            value: 5.0,
        },
    };

    assert_eq!(action.target_speed, 30.0);
    assert_eq!(action.dynamics.value, 5.0);
    assert_eq!(action.dynamics.dimension, DynamicsDimension::Time);
}

#[test]
fn test_speed_action_with_rate_dimension() {
    // Test SpeedAction with rate dimension (acceleration-based)
    let action = SpeedAction {
        target_speed: 50.0,
        dynamics: TransitionDynamics {
            shape: DynamicsShape::Cubic,
            dimension: DynamicsDimension::Rate,
            value: 2.0, // 2 m/s² acceleration
        },
    };

    assert_eq!(action.dynamics.dimension, DynamicsDimension::Rate);
    assert_eq!(action.dynamics.value, 2.0);
}

#[test]
fn test_speed_action_with_distance_dimension() {
    // Test SpeedAction with distance dimension
    let action = SpeedAction {
        target_speed: 25.0,
        dynamics: TransitionDynamics {
            shape: DynamicsShape::Sinusoidal,
            dimension: DynamicsDimension::Distance,
            value: 100.0, // reach target speed over 100m
        },
    };

    assert_eq!(action.dynamics.dimension, DynamicsDimension::Distance);
    assert_eq!(action.dynamics.value, 100.0);
}

#[test]
fn test_add_speed_action_to_scenario() {
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

    // Add speed action with new dynamics
    let result = scenario.add_speed_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        50.0, // target speed
        TransitionDynamics {
            shape: DynamicsShape::Linear,
            dimension: DynamicsDimension::Time,
            value: 3.0,
        },
    );

    assert!(result.is_ok(), "Failed to add speed action");
}

#[test]
fn test_speed_action_xml_export() {
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
        .add_speed_action(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "event1",
            50.0,
            TransitionDynamics {
                shape: DynamicsShape::Cubic,
                dimension: DynamicsDimension::Rate,
                value: 2.5,
            },
        )
        .unwrap();

    // Export to XML
    let xml = scenario.to_xml().unwrap();

    // Verify XML contains SpeedAction elements with correct dynamics
    assert!(xml.contains("SpeedAction"));
    assert!(xml.contains("SpeedActionDynamics"));
    assert!(xml.contains("value=\"50"));
    assert!(xml.contains("dynamicsDimension=\"rate\""));
    assert!(xml.contains("dynamicsShape=\"cubic\""));
    assert!(xml.contains("value=\"2.5\""));
}

#[test]
fn test_speed_action_backward_compatibility() {
    // Ensure legacy SpeedAction usage still works if needed
    // This might require keeping the old API or providing a migration path
    let action = SpeedAction {
        target_speed: 30.0,
        dynamics: TransitionDynamics {
            shape: DynamicsShape::Linear,
            dimension: DynamicsDimension::Time,
            value: 5.0,
        },
    };

    // Old structure assumed time dimension and shape
    // New structure makes it explicit
    assert_eq!(action.target_speed, 30.0);
    assert_eq!(action.dynamics.dimension, DynamicsDimension::Time);
}
