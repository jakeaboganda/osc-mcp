use openscenario::storyboard::{Action, AssignRouteAction, Route, Waypoint};
use openscenario::{OpenScenarioVersion, Scenario};

#[test]
fn test_waypoint_creation() {
    let waypoint = Waypoint {
        position: openscenario::Position::world(10.0, 20.0, 0.0, 0.0),
        route_strategy: None,
    };

    // Position should be stored correctly
    match &waypoint.position {
        openscenario::Position::World { x, y, .. } => {
            assert_eq!(x, &10.0);
            assert_eq!(y, &20.0);
        }
        _ => panic!("Expected World position"),
    }
    assert!(waypoint.route_strategy.is_none());
}

#[test]
fn test_route_creation() {
    let route = Route {
        name: "highway_route".to_string(),
        closed: false,
        waypoints: vec![
            Waypoint {
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
                route_strategy: None,
            },
            Waypoint {
                position: openscenario::Position::world(100.0, 0.0, 0.0, 0.0),
                route_strategy: None,
            },
            Waypoint {
                position: openscenario::Position::world(200.0, 50.0, 0.0, 0.0),
                route_strategy: None,
            },
        ],
    };

    assert_eq!(route.name, "highway_route");
    assert_eq!(route.closed, false);
    assert_eq!(route.waypoints.len(), 3);
}

#[test]
fn test_assign_route_action_creation() {
    let route = Route {
        name: "test_route".to_string(),
        closed: false,
        waypoints: vec![
            Waypoint {
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
                route_strategy: None,
            },
            Waypoint {
                position: openscenario::Position::world(50.0, 0.0, 0.0, 0.0),
                route_strategy: None,
            },
        ],
    };

    let action = AssignRouteAction { route };

    assert_eq!(action.route.name, "test_route");
    assert_eq!(action.route.waypoints.len(), 2);
}

#[test]
fn test_closed_route() {
    let route = Route {
        name: "loop_route".to_string(),
        closed: true,
        waypoints: vec![
            Waypoint {
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
                route_strategy: None,
            },
            Waypoint {
                position: openscenario::Position::world(10.0, 0.0, 0.0, 0.0),
                route_strategy: None,
            },
            Waypoint {
                position: openscenario::Position::world(10.0, 10.0, 0.0, 0.0),
                route_strategy: None,
            },
            Waypoint {
                position: openscenario::Position::world(0.0, 10.0, 0.0, 0.0),
                route_strategy: None,
            },
        ],
    };

    assert_eq!(route.closed, true);
    assert_eq!(route.waypoints.len(), 4);
}

#[test]
fn test_assign_route_action_in_action_enum() {
    let route = Route {
        name: "test".to_string(),
        closed: false,
        waypoints: vec![
            Waypoint {
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
                route_strategy: None,
            },
            Waypoint {
                position: openscenario::Position::world(10.0, 0.0, 0.0, 0.0),
                route_strategy: None,
            },
        ],
    };

    let assign_route = AssignRouteAction { route };
    let action = Action::AssignRoute(assign_route);

    match action {
        Action::AssignRoute(a) => {
            assert_eq!(a.route.name, "test");
            assert_eq!(a.route.waypoints.len(), 2);
        }
        _ => panic!("Expected AssignRoute action"),
    }
}

#[test]
fn test_add_assign_route_action_to_scenario() {
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

    let route = Route {
        name: "city_route".to_string(),
        closed: false,
        waypoints: vec![
            Waypoint {
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
                route_strategy: None,
            },
            Waypoint {
                position: openscenario::Position::world(50.0, 0.0, 0.0, 0.0),
                route_strategy: None,
            },
            Waypoint {
                position: openscenario::Position::world(50.0, 50.0, 0.0, 0.0),
                route_strategy: None,
            },
        ],
    };

    let result = scenario.add_assign_route_action(
        "main_story",
        "act1",
        "mg1",
        "maneuver1",
        "event1",
        route,
    );

    assert!(result.is_ok(), "Failed to add assign route action");
}

#[test]
fn test_assign_route_action_xml_export() {
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

    let route = Route {
        name: "export_test_route".to_string(),
        closed: true,
        waypoints: vec![
            Waypoint {
                position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
                route_strategy: None,
            },
            Waypoint {
                position: openscenario::Position::world(25.0, 0.0, 0.0, 0.5),
                route_strategy: None,
            },
            Waypoint {
                position: openscenario::Position::world(50.0, 25.0, 0.0, 1.0),
                route_strategy: None,
            },
        ],
    };

    scenario
        .add_assign_route_action(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "event1",
            route,
        )
        .unwrap();

    let xml = scenario.to_xml().unwrap();

    // Verify XML contains route elements
    assert!(xml.contains("RoutingAction"));
    assert!(xml.contains("AssignRouteAction"));
    assert!(xml.contains("Route"));
    assert!(xml.contains("name=\"export_test_route\""));
    assert!(xml.contains("closed=\"true\""));
    assert!(xml.contains("Waypoint"));
    assert!(xml.contains("WorldPosition"));
}

#[test]
fn test_route_validation_minimum_waypoints() {
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

    // Single waypoint route (invalid)
    let invalid_route = Route {
        name: "invalid".to_string(),
        closed: false,
        waypoints: vec![Waypoint {
            position: openscenario::Position::world(0.0, 0.0, 0.0, 0.0),
            route_strategy: None,
        }],
    };

    let result = scenario.add_assign_route_action(
        "story",
        "act",
        "mg",
        "maneuver",
        "event",
        invalid_route,
    );

    assert!(result.is_err(), "Should reject route with < 2 waypoints");
}
