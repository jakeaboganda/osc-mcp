use openscenario::storyboard::{Action, LongitudinalDistanceAction, TransitionDynamics, DynamicsShape, DynamicsDimension};
use openscenario::{OpenScenarioVersion, Scenario};

#[test]
fn test_longitudinal_distance_action_creation() {
    // Create a LongitudinalDistanceAction with basic parameters
    let action = LongitudinalDistanceAction {
        entity_ref: "lead_vehicle".to_string(),
        distance: 50.0,
        freespace: true,
        continuous: true,
        dynamics: Some(TransitionDynamics {
            shape: DynamicsShape::Linear,
            dimension: DynamicsDimension::Time,
            value: 2.0,
        }),
    };

    assert_eq!(action.entity_ref, "lead_vehicle");
    assert_eq!(action.distance, 50.0);
    assert!(action.freespace);
    assert!(action.continuous);
    assert!(action.dynamics.is_some());
}

#[test]
fn test_longitudinal_distance_action_in_action_enum() {
    // Ensure LongitudinalDistanceAction can be wrapped in Action enum
    let action = Action::LongitudinalDistance(LongitudinalDistanceAction {
        entity_ref: "lead_vehicle".to_string(),
        distance: 30.0,
        freespace: false,
        continuous: true,
        dynamics: None,
    });

    match action {
        Action::LongitudinalDistance(ld) => {
            assert_eq!(ld.entity_ref, "lead_vehicle");
            assert_eq!(ld.distance, 30.0);
            assert!(!ld.freespace);
        }
        _ => panic!("Expected LongitudinalDistance action"),
    }
}

#[test]
fn test_add_longitudinal_distance_action_to_scenario() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add entities
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
        .add_vehicle(
            "lead_vehicle",
            openscenario::entities::VehicleParams {
                catalog: None,
                vehicle_category: openscenario::entities::VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    // Set initial positions
    scenario
        .set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    scenario
        .set_initial_position("lead_vehicle", openscenario::Position::world(50.0, 0.0, 0.0, 0.0))
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

    // Add longitudinal distance action
    let result = scenario.add_longitudinal_distance_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        "lead_vehicle",
        50.0,
        true,
        true,
        Some(TransitionDynamics {
            shape: DynamicsShape::Linear,
            dimension: DynamicsDimension::Time,
            value: 3.0,
        }),
    );

    assert!(result.is_ok(), "Failed to add longitudinal distance action");
}

#[test]
fn test_longitudinal_distance_action_xml_export() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add entities
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
        .add_vehicle(
            "lead_vehicle",
            openscenario::entities::VehicleParams {
                catalog: None,
                vehicle_category: openscenario::entities::VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    // Set initial positions
    scenario
        .set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    scenario
        .set_initial_position("lead_vehicle", openscenario::Position::world(50.0, 0.0, 0.0, 0.0))
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
        .add_longitudinal_distance_action(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "event1",
            "lead_vehicle",
            50.0,
            true,
            true,
            Some(TransitionDynamics {
                shape: DynamicsShape::Linear,
                dimension: DynamicsDimension::Time,
                value: 3.0,
            }),
        )
        .unwrap();

    // Export to XML
    let xml = scenario.to_xml().unwrap();

    // Debug: print the XML to see what we got
    println!("XML output:\n{}", xml);

    // Verify XML contains LongitudinalDistanceAction elements
    assert!(xml.contains("LongitudinalDistanceAction"));
    assert!(xml.contains("entityRef=\"lead_vehicle\""));
    assert!(xml.contains("value=\"50"));
    assert!(xml.contains("freespace=\"true\""));
    assert!(xml.contains("continuous=\"true\""));
    assert!(xml.contains("DynamicConstraints") || xml.contains("Dynamics"));
    assert!(xml.contains("time"));
}

#[test]
fn test_longitudinal_distance_action_edge_cases() {
    // Test with zero distance
    let action_zero = LongitudinalDistanceAction {
        entity_ref: "target".to_string(),
        distance: 0.0,
        freespace: true,
        continuous: false,
        dynamics: None,
    };
    assert_eq!(action_zero.distance, 0.0);

    // Test with large distance
    let action_large = LongitudinalDistanceAction {
        entity_ref: "target".to_string(),
        distance: 1000.0,
        freespace: false,
        continuous: true,
        dynamics: Some(TransitionDynamics {
            shape: DynamicsShape::Cubic,
            dimension: DynamicsDimension::Distance,
            value: 100.0,
        }),
    };
    assert_eq!(action_large.distance, 1000.0);
}
