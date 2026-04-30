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
