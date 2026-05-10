//! Highway Merge - Vehicle merging onto highway from on-ramp
//!
//! Demonstrates:
//! - Position-based starting point
//! - Lane change maneuver
//!
//! Scenario: A vehicle merges from on-ramp into highway traffic
//!
//! Run with: cargo run --example highway_merge

use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::TransitionShape;
use openscenario::{OpenScenarioVersion, Position, Scenario};

fn main() -> Result<(), openscenario::ScenarioError> {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add merging vehicle (starts on on-ramp)
    let vehicle_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("merging_vehicle", vehicle_params)?;

    // Start on on-ramp (offset from highway lane)
    scenario.set_initial_position("merging_vehicle", Position::world(0.0, 3.5, 0.0, 0.0))?;

    // Create story structure
    scenario.add_story("merge_story")?;
    scenario.add_act("merge_story", "merge_act")?;
    scenario.add_maneuver_group("merge_story", "merge_act", "merge_group")?;
    scenario.add_actor("merge_story", "merge_act", "merge_group", "merging_vehicle")?;
    scenario.add_maneuver("merge_story", "merge_act", "merge_group", "merge_maneuver")?;

    // Merge into highway lane (move right from on-ramp to highway)
    scenario.add_lane_change_action(
        "merge_story",
        "merge_act",
        "merge_group",
        "merge_maneuver",
        "merge_right",
        -1.0, // Move one lane to the right (into highway)
        4.0,  // Take 4 seconds to merge
        TransitionShape::Sinusoidal,
    )?;

    // Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("highway_merge.xosc", xml)?;

    println!("✅ Highway merge scenario exported to highway_merge.xosc");
    println!("   - Vehicle: merging_vehicle");
    println!("   - Initial position: On-ramp (x=0, y=3.5m)");
    println!("   - Action: Merge right into highway over 4s");
    println!();
    println!("Visualization:");
    println!("  The vehicle starts on an on-ramp and smoothly merges");
    println!("  into the right lane of the highway using sinusoidal dynamics.");

    Ok(())
}
