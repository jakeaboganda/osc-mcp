//! Lane Change Example - Vehicle performing a lateral lane change maneuver
//!
//! Demonstrates:
//! - Adding a vehicle with initial position
//! - Creating a story/act/maneuver structure
//! - Adding a lane change action
//! - Exporting to OpenSCENARIO XML
//!
//! Run with: cargo run --example lane_change

use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::TransitionShape;
use openscenario::{OpenScenarioVersion, Position, Scenario};

fn main() -> Result<(), openscenario::ScenarioError> {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add ego vehicle
    let vehicle_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params)?;

    // Set initial position in world coordinates
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))?;

    // Create story structure
    scenario.add_story("main_story")?;
    scenario.add_act("main_story", "act1")?;
    scenario.add_maneuver_group("main_story", "act1", "lane_change_group")?;
    scenario.add_actor("main_story", "act1", "lane_change_group", "ego")?;
    scenario.add_maneuver("main_story", "act1", "lane_change_group", "lane_change_maneuver")?;

    // Add lane change action: move left 1 lane
    scenario.add_lane_change_action(
        "main_story",
        "act1",
        "lane_change_group",
        "lane_change_maneuver",
        "change_to_left_lane",
        -1.0,                        // Target lane offset (-1 = one lane left)
        5.0,                         // Duration (seconds)
        TransitionShape::Linear,     // Linear transition
    )?;

    // Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("lane_change.xosc", xml)?;

    println!("✅ Lane change scenario exported to lane_change.xosc");
    println!("   - Vehicle: ego");
    println!("   - Maneuver: Change one lane to the left");
    println!("   - Duration: 5 seconds with linear transition");

    Ok(())
}
