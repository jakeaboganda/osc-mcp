use openscenario::entities::{BoundingBox, VehicleCategory, VehicleParams};
use openscenario::{OpenScenarioVersion, Position, Scenario};

// ============================================================================
// Initial Speed Tests
// ============================================================================

#[test]
fn test_set_initial_speed() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let result = scenario.set_initial_speed("ego", 30.0);

    assert!(result.is_ok());
    assert_eq!(scenario.get_initial_speed("ego"), Some(&30.0));
}

#[test]
fn test_set_initial_speed_entity_not_found() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let result = scenario.set_initial_speed("nonexistent", 30.0);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("nonexistent"));
    assert!(err_msg.contains("not found"));
}

#[test]
fn test_set_initial_speed_empty_entity_name() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let result = scenario.set_initial_speed("", 30.0);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("empty"));
}

#[test]
fn test_set_initial_speed_whitespace_entity_name() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let result = scenario.set_initial_speed("   ", 30.0);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("empty") || err_msg.contains("whitespace"));
}

#[test]
fn test_set_initial_speed_negative_speed() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let result = scenario.set_initial_speed("ego", -10.0);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("speed"));
}

#[test]
fn test_set_initial_speed_zero_is_valid() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let result = scenario.set_initial_speed("ego", 0.0);

    assert!(result.is_ok());
    assert_eq!(scenario.get_initial_speed("ego"), Some(&0.0));
}

#[test]
fn test_get_initial_speed_not_set() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    assert_eq!(scenario.get_initial_speed("ego"), None);
}

#[test]
fn test_initial_speeds_iterator() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("car1", params.clone()).unwrap();
    scenario.add_vehicle("car2", params).unwrap();
    scenario.set_initial_speed("car1", 25.0).unwrap();
    scenario.set_initial_speed("car2", 30.0).unwrap();

    let speeds: Vec<_> = scenario.initial_speeds().collect();
    assert_eq!(speeds.len(), 2);
}

#[test]
fn test_set_initial_speed_overwrites() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    scenario.set_initial_speed("ego", 20.0).unwrap();
    scenario.set_initial_speed("ego", 30.0).unwrap();

    assert_eq!(scenario.get_initial_speed("ego"), Some(&30.0));
}

// ============================================================================
// Initial State Tests (Combined Position + Speed)
// ============================================================================

#[test]
fn test_set_initial_state_with_speed() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let position = Position::world(0.0, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_state("ego", position.clone(), Some(30.0));

    assert!(result.is_ok());
    assert_eq!(scenario.get_initial_position("ego"), Some(&position));
    assert_eq!(scenario.get_initial_speed("ego"), Some(&30.0));
}

#[test]
fn test_set_initial_state_without_speed() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let position = Position::world(0.0, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_state("ego", position.clone(), None);

    assert!(result.is_ok());
    assert_eq!(scenario.get_initial_position("ego"), Some(&position));
    assert_eq!(scenario.get_initial_speed("ego"), None);
}

#[test]
fn test_set_initial_state_entity_not_found() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let position = Position::world(0.0, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_state("nonexistent", position, Some(30.0));

    assert!(result.is_err());
}

#[test]
fn test_set_initial_state_negative_speed_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let position = Position::world(0.0, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_state("ego", position.clone(), Some(-10.0));

    assert!(result.is_err());
    // Note: Position IS set because set_initial_position() succeeds first,
    // then set_initial_speed() fails. This is acceptable behavior.
    assert_eq!(scenario.get_initial_position("ego"), Some(&position));
    assert_eq!(scenario.get_initial_speed("ego"), None);
}

// ============================================================================
// Road Network Tests
// ============================================================================

#[test]
fn test_set_road_network() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let result = scenario.set_road_network("roads/highway.xodr");

    assert!(result.is_ok());
    assert_eq!(
        scenario.get_road_network(),
        Some(&"roads/highway.xodr".to_string())
    );
}

#[test]
fn test_set_road_network_empty_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let result = scenario.set_road_network("");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("empty"));
}

#[test]
fn test_set_road_network_whitespace_fails() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let result = scenario.set_road_network("   ");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("empty") || err_msg.contains("whitespace"));
}

#[test]
fn test_set_road_network_trims_whitespace() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    scenario.set_road_network("  roads/highway.xodr  ").unwrap();

    assert_eq!(
        scenario.get_road_network(),
        Some(&"roads/highway.xodr".to_string())
    );
}

#[test]
fn test_get_road_network_not_set() {
    let scenario = Scenario::new(OpenScenarioVersion::V1_2);
    assert_eq!(scenario.get_road_network(), None);
}

#[test]
fn test_set_road_network_overwrites() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    scenario.set_road_network("roads/first.xodr").unwrap();
    scenario.set_road_network("roads/second.xodr").unwrap();

    assert_eq!(
        scenario.get_road_network(),
        Some(&"roads/second.xodr".to_string())
    );
}

// ============================================================================
// XML Generation Tests
// ============================================================================

#[test]
fn test_xml_with_initial_speed() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    scenario.set_initial_speed("ego", 25.0).unwrap();

    let xml = scenario.to_xml().unwrap();

    // Check that SpeedAction is present
    assert!(xml.contains("<SpeedAction>"));
    assert!(xml.contains("<AbsoluteTargetSpeed value=\"25\""));
    assert!(xml.contains("dynamicsShape=\"step\""));
}

#[test]
fn test_xml_speed_action_before_teleport_action() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    scenario
        .set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    scenario.set_initial_speed("ego", 25.0).unwrap();

    let xml = scenario.to_xml().unwrap();

    // Find positions of SpeedAction and TeleportAction
    let speed_pos = xml.find("<SpeedAction>").unwrap();
    let teleport_pos = xml.find("<TeleportAction>").unwrap();

    // SpeedAction must come before TeleportAction
    assert!(
        speed_pos < teleport_pos,
        "SpeedAction must come before TeleportAction in Init section"
    );
}

#[test]
fn test_xml_with_road_network() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    scenario.set_road_network("roads/highway.xodr").unwrap();

    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("<RoadNetwork>"));
    assert!(xml.contains("<LogicFile filepath=\"roads/highway.xodr\""));
}

#[test]
fn test_xml_without_road_network() {
    let scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("<RoadNetwork/>"));
    assert!(!xml.contains("<LogicFile"));
}

#[test]
fn test_xml_with_lane_position_and_speed() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    scenario.set_road_network("roads/highway.xodr").unwrap();
    scenario
        .set_initial_state("ego", Position::lane("1", -2, 10.0, 0.0, None), Some(25.0))
        .unwrap();

    let xml = scenario.to_xml().unwrap();

    // Check all critical elements are present
    assert!(xml.contains("<LogicFile filepath=\"roads/highway.xodr\""));
    assert!(xml.contains("<LanePosition"));
    assert!(xml.contains("roadId=\"1\""));
    assert!(xml.contains("laneId=\"-2\""));
    assert!(xml.contains("<SpeedAction>"));
    assert!(xml.contains("<AbsoluteTargetSpeed value=\"25\""));
}

// ============================================================================
// Spawn Collision Tests (World-position overlap check)
// ============================================================================

use openscenario::error::ScenarioError;

fn two_car_scenario() -> openscenario::Scenario {
    use openscenario::{OpenScenarioVersion, Scenario};
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_vehicle(
        "ego",
        VehicleParams {
            catalog: None,
            vehicle_category: VehicleCategory::Car,
            properties: None,
        },
    )
    .unwrap();
    s.add_vehicle(
        "npc",
        VehicleParams {
            catalog: None,
            vehicle_category: VehicleCategory::Car,
            properties: None,
        },
    )
    .unwrap();
    s
}

#[test]
fn two_cars_at_same_world_position_errors() {
    let mut s = two_car_scenario();
    s.set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    let result = s.set_initial_position("npc", openscenario::Position::world(0.0, 0.0, 0.0, 0.0));
    assert!(
        matches!(result, Err(ScenarioError::SpawnCollision { .. })),
        "expected SpawnCollision, got {:?}",
        result
    );
}

#[test]
fn two_cars_far_apart_does_not_error() {
    let mut s = two_car_scenario();
    s.set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    s.set_initial_position("npc", openscenario::Position::world(50.0, 0.0, 0.0, 0.0))
        .unwrap();
}

#[test]
fn first_entity_position_always_succeeds() {
    use openscenario::{OpenScenarioVersion, Scenario};
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_vehicle(
        "ego",
        VehicleParams {
            catalog: None,
            vehicle_category: VehicleCategory::Car,
            properties: None,
        },
    )
    .unwrap();
    s.set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
}

#[test]
fn spawn_collision_error_names_both_entities() {
    let mut s = two_car_scenario();
    s.set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    let result = s.set_initial_position("npc", openscenario::Position::world(0.5, 0.0, 0.0, 0.0));
    match result {
        Err(ScenarioError::SpawnCollision { entity_a, entity_b }) => {
            let names = [entity_a.as_str(), entity_b.as_str()];
            assert!(
                names.contains(&"ego") && names.contains(&"npc"),
                "expected both entity names, got {:?}",
                names
            );
        }
        other => panic!("expected SpawnCollision, got {:?}", other),
    }
}

#[test]
fn custom_small_bbox_allows_closer_spawn() {
    let mut s = two_car_scenario();
    let tiny = BoundingBox {
        length: 0.1,
        width: 0.1,
        height: 0.1,
    };
    s.set_entity_dimensions("ego", tiny.clone()).unwrap();
    s.set_entity_dimensions("npc", tiny.clone()).unwrap();
    s.set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    // tiny OBB (0.1×0.1m) half-extents 0.05m → gap of 0.9m on x-axis → no overlap
    s.set_initial_position("npc", openscenario::Position::world(1.0, 0.0, 0.0, 0.0))
        .unwrap();
}

#[test]
fn large_truck_requires_more_clearance_than_cars() {
    use openscenario::{OpenScenarioVersion, Scenario};
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_vehicle(
        "truck",
        VehicleParams {
            catalog: None,
            vehicle_category: VehicleCategory::Truck,
            properties: None,
        },
    )
    .unwrap();
    s.add_vehicle(
        "car",
        VehicleParams {
            catalog: None,
            vehicle_category: VehicleCategory::Car,
            properties: None,
        },
    )
    .unwrap();
    s.set_initial_position("truck", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    // truck (8.5m, heading 0) projects to [-4.25, 4.25] on x-axis, car (4.5m) at x=6 projects to [3.75, 8.25] → overlap
    let result = s.set_initial_position("car", openscenario::Position::world(6.0, 0.0, 0.0, 0.0));
    assert!(
        matches!(result, Err(ScenarioError::SpawnCollision { .. })),
        "truck+car 6m apart should collide, got {:?}",
        result
    );
}

#[test]
fn cars_end_to_end_with_bumper_gap_not_flagged_as_collision() {
    // Car default: 4.5m long, half-length 2.25m
    // Two cars at x=0 and x=4.6 → 0.1m gap between bumpers
    // With circles (radius≈2.44m each, clearance≈4.88m) this would be a false positive.
    // OBB correctly reports no overlap: A projects to [-2.25, 2.25], B to [2.35, 6.85] on x-axis.
    let mut s = two_car_scenario();
    s.set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    s.set_initial_position("npc", openscenario::Position::world(4.6, 0.0, 0.0, 0.0))
        .unwrap();
}

#[test]
fn lane_position_is_not_collision_checked() {
    let mut s = two_car_scenario();
    s.set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    // Lane position on npc — should NOT trigger SpawnCollision (road network check may fire instead)
    let result = s.set_initial_position(
        "npc",
        openscenario::Position::Lane {
            road_id: "road1".to_string(),
            lane_id: -1,
            s: 10.0,
            offset: 0.0,
            orientation: None,
        },
    );
    assert!(
        !matches!(result, Err(ScenarioError::SpawnCollision { .. })),
        "Lane position should not trigger SpawnCollision"
    );
}

#[test]
fn relative_position_is_not_collision_checked() {
    let mut s = two_car_scenario();
    s.set_initial_position("ego", openscenario::Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    // RelativeWorld at dx=0, dy=0 would overlap geometrically, but relative positions are not checked
    s.set_initial_position(
        "npc",
        openscenario::Position::RelativeWorld {
            entity: "ego".to_string(),
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            orientation: openscenario::position::Orientation::default(),
        },
    )
    .unwrap();
}

#[test]
fn perpendicular_cars_cleared_by_obb_where_circles_would_false_positive() {
    // Car A: 4.5×1.9m at (0,0) heading 0. Car B: 4.5×1.9m at (4.0,0) heading π/2.
    // On A's forward axis (x): A projects to [-2.25, 2.25], B (rotated 90°) projects
    // its half-width (0.95m) onto x → B spans [3.05, 4.95]. Gap 2.25 < 3.05 → SAT says clear.
    // Circumscribed circle radius ≈ 2.44m each; center distance 4.0m < 4.88m → would false-positive.
    let mut s = two_car_scenario();
    s.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    s.set_initial_position(
        "npc",
        Position::World {
            x: 4.0,
            y: 0.0,
            z: 0.0,
            h: std::f64::consts::FRAC_PI_2,
            p: 0.0,
            r: 0.0,
        },
    )
    .unwrap();
}
