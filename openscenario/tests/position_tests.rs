use openscenario::position::{Position, Orientation};

#[test]
fn test_world_position() {
    let pos = Position::world(10.0, 20.0, 0.0, 1.57);
    match pos {
        Position::World { x, y, z, h, .. } => {
            assert_eq!(x, 10.0);
            assert_eq!(y, 20.0);
            assert_eq!(z, 0.0);
            assert_eq!(h, 1.57);
        }
        _ => panic!("Expected WorldPosition"),
    }
}

#[test]
fn test_lane_position() {
    let pos = Position::lane("road1", 1, 50.0, 0.5, None);
    match pos {
        Position::Lane { road_id, lane_id, s, offset, .. } => {
            assert_eq!(road_id, "road1");
            assert_eq!(lane_id, 1);
            assert_eq!(s, 50.0);
            assert_eq!(offset, 0.5);
        }
        _ => panic!("Expected LanePosition"),
    }
}

#[test]
fn test_relative_world_position() {
    let pos = Position::relative_world("ego", 10.0, 5.0, 0.0, Orientation::default());
    match pos {
        Position::RelativeWorld { entity, dx, dy, .. } => {
            assert_eq!(entity, "ego");
            assert_eq!(dx, 10.0);
            assert_eq!(dy, 5.0);
        }
        _ => panic!("Expected RelativeWorldPosition"),
    }
}

#[test]
fn test_referenced_entity_helper() {
    // Relative positions should return entity reference
    let rel_world = Position::relative_world("ego", 10.0, 5.0, 0.0, Orientation::default());
    assert_eq!(rel_world.referenced_entity(), Some("ego"));
    
    let rel_obj = Position::relative_object("target", 2.0, 3.0, 0.0, Orientation::default());
    assert_eq!(rel_obj.referenced_entity(), Some("target"));
    
    let rel_lane = Position::relative_lane("car1", 5.0, 1, 0.5, None);
    assert_eq!(rel_lane.referenced_entity(), Some("car1"));
    
    let rel_road = Position::relative_road("truck", 10.0, 2.0, None);
    assert_eq!(rel_road.referenced_entity(), Some("truck"));
    
    // Absolute positions should return None
    let world = Position::world(0.0, 0.0, 0.0, 0.0);
    assert_eq!(world.referenced_entity(), None);
    
    let lane = Position::lane("road1", 1, 50.0, 0.5, None);
    assert_eq!(lane.referenced_entity(), None);
    
    let road = Position::road("road2", 100.0, 1.5, None);
    assert_eq!(road.referenced_entity(), None);
}
