use openscenario::entities::{
    BoundingBox, Entity, MiscObject, MiscObjectParams, Pedestrian, PedestrianParams, Vehicle,
    VehicleCategory, VehicleParams,
};
use openscenario::Position;
use openscenario::{OpenScenarioVersion, Scenario};

#[test]
fn test_add_vehicle() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    let result = scenario.add_vehicle("ego", params);
    assert!(result.is_ok());
}

#[test]
fn test_add_vehicle_conflict() {
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
    assert!(err_msg.contains("car1"));
    assert!(err_msg.contains("already exists"));
}

#[test]
fn test_set_initial_position() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();
    let pos = Position::world(0.0, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_position("ego", pos);

    assert!(result.is_ok());
}

#[test]
fn test_set_initial_position_entity_not_found() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let pos = Position::world(0.0, 0.0, 0.0, 0.0);
    let result = scenario.set_initial_position("nonexistent", pos);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("nonexistent"));
}

#[test]
fn test_add_pedestrian() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = PedestrianParams {
        catalog: None,
        model: None,
        mass: Some(75.0),
    };

    let result = scenario.add_pedestrian("ped1", params);
    assert!(result.is_ok());
}

#[test]
fn test_add_misc_object() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = MiscObjectParams {
        catalog: None,
        category: Some("Barrier".to_string()),
        mass: Some(100.0),
    };

    let result = scenario.add_misc_object("barrier1", params);
    assert!(result.is_ok());
}

#[test]
fn test_get_entity() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("ego", params).unwrap();

    let entity = scenario.get_entity("ego");
    assert!(entity.is_some());
    assert_eq!(entity.unwrap().name(), "ego");

    let missing = scenario.get_entity("nonexistent");
    assert!(missing.is_none());
}

#[test]
fn bounding_box_zero_dimensions_length_is_zero() {
    let bb = BoundingBox {
        length: 0.0,
        width: 0.0,
        height: 0.0,
    };
    assert_eq!(bb.length, 0.0);
}

#[test]
fn vehicle_category_car_default_bbox_sensible() {
    let bb = VehicleCategory::Car.default_bounding_box();
    assert!(bb.length > 3.0 && bb.length < 6.0);
    assert!(bb.width > 1.5 && bb.width < 2.5);
}

#[test]
fn vehicle_category_truck_larger_than_car() {
    let car = VehicleCategory::Car.default_bounding_box();
    let truck = VehicleCategory::Truck.default_bounding_box();
    assert!(truck.length > car.length);
    assert!(truck.width > car.width);
}

#[test]
fn vehicle_category_motorbike_smaller_than_car() {
    let car = VehicleCategory::Car.default_bounding_box();
    let bike = VehicleCategory::Motorbike.default_bounding_box();
    assert!(bike.length < car.length);
    assert!(bike.width < car.width);
}

#[test]
fn entity_default_bounding_box_vehicle() {
    let entity = Entity::Vehicle(Vehicle {
        name: "ego".to_string(),
        params: VehicleParams {
            catalog: None,
            vehicle_category: VehicleCategory::Car,
            properties: None,
        },
    });
    let bb = entity.default_bounding_box();
    assert!(bb.length > 0.0);
}

#[test]
fn entity_default_bounding_box_pedestrian() {
    let entity = Entity::Pedestrian(Pedestrian {
        name: "ped".to_string(),
        params: PedestrianParams {
            catalog: None,
            model: None,
            mass: None,
        },
    });
    let bb = entity.default_bounding_box();
    assert!(bb.width < 1.0);
    assert!(bb.height > 1.0);
}

#[test]
fn entity_default_bounding_box_misc_object() {
    let entity = Entity::MiscObject(MiscObject {
        name: "cone".to_string(),
        params: MiscObjectParams {
            catalog: None,
            category: None,
            mass: None,
        },
    });
    let bb = entity.default_bounding_box();
    assert!(bb.length > 0.0);
}

fn car_params() -> VehicleParams {
    VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    }
}

#[test]
fn set_entity_dimensions_stores_bbox() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_vehicle("ego", car_params()).unwrap();
    let bb = BoundingBox {
        length: 5.0,
        width: 2.0,
        height: 1.6,
    };
    s.set_entity_dimensions("ego", bb.clone()).unwrap();
    assert_eq!(s.effective_bounding_box("ego").unwrap(), bb);
}

#[test]
fn effective_bounding_box_falls_back_to_default() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_vehicle("ego", car_params()).unwrap();
    let bb = s.effective_bounding_box("ego").unwrap();
    assert!(
        bb.length > 3.0,
        "Car default length should be > 3m, got {}",
        bb.length
    );
}

#[test]
fn set_entity_dimensions_errors_on_unknown_entity() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    let result = s.set_entity_dimensions(
        "ghost",
        BoundingBox {
            length: 1.0,
            width: 1.0,
            height: 1.0,
        },
    );
    assert!(result.is_err());
}

#[test]
fn set_entity_dimensions_overrides_default() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_vehicle("ego", car_params()).unwrap();
    let default = s.effective_bounding_box("ego").unwrap();
    let custom = BoundingBox {
        length: default.length + 10.0,
        width: default.width,
        height: default.height,
    };
    s.set_entity_dimensions("ego", custom.clone()).unwrap();
    assert_eq!(
        s.effective_bounding_box("ego").unwrap().length,
        custom.length
    );
}

#[test]
fn set_entity_dimensions_update_overwrites_previous() {
    let mut s = Scenario::new(OpenScenarioVersion::V1_2);
    s.add_vehicle("ego", car_params()).unwrap();
    s.set_entity_dimensions(
        "ego",
        BoundingBox {
            length: 3.0,
            width: 1.5,
            height: 1.3,
        },
    )
    .unwrap();
    s.set_entity_dimensions(
        "ego",
        BoundingBox {
            length: 6.0,
            width: 2.0,
            height: 1.8,
        },
    )
    .unwrap();
    assert_eq!(s.effective_bounding_box("ego").unwrap().length, 6.0);
}

#[test]
fn effective_bounding_box_returns_none_for_unknown_entity() {
    let s = Scenario::new(OpenScenarioVersion::V1_2);
    assert!(s.effective_bounding_box("ghost").is_none());
}
