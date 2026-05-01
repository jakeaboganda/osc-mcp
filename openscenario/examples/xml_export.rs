use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::TransitionShape;
use openscenario::{OpenScenarioVersion, Position, Scenario};

fn main() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();

    let pos = Position::world(0.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("ego", pos).unwrap();

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
            5.0,
            TransitionShape::Linear,
        )
        .unwrap();

    let xml = scenario.to_xml().unwrap();
    println!("{}", xml);
}
