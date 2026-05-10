//! Platooning - Multiple vehicles following in coordinated formation
//!
//! Demonstrates:
//! - Multi-vehicle coordination
//! - Time headway conditions for multiple followers
//! - Platoon formation with consistent spacing
//!
//! Scenario: Three vehicles maintain 1.5-second time headway in a platoon
//!
//! Run with: cargo run --example platooning

use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::Rule;
use openscenario::{OpenScenarioVersion, Position, Scenario};

fn main() -> Result<(), openscenario::ScenarioError> {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    let vehicle_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    // Platoon leader
    scenario.add_vehicle("leader", vehicle_params.clone())?;
    scenario.set_initial_position("leader", Position::world(100.0, 0.0, 0.0, 0.0))?;

    // Follower 1 (follows leader)
    scenario.add_vehicle("follower1", vehicle_params.clone())?;
    scenario.set_initial_position("follower1", Position::world(50.0, 0.0, 0.0, 0.0))?;

    // Follower 2 (follows follower1)
    scenario.add_vehicle("follower2", vehicle_params)?;
    scenario.set_initial_position("follower2", Position::world(0.0, 0.0, 0.0, 0.0))?;

    // Story structure
    scenario.add_story("platoon_story")?;
    scenario.add_act("platoon_story", "follow_act")?;

    // Follower 1 group
    scenario.add_maneuver_group("platoon_story", "follow_act", "follower1_group")?;
    scenario.add_actor("platoon_story", "follow_act", "follower1_group", "follower1")?;
    scenario.add_maneuver(
        "platoon_story",
        "follow_act",
        "follower1_group",
        "follow1_maneuver",
    )?;

    // Follower 1: Maintain 1.5s headway from leader
    scenario.add_event_with_time_headway_condition(
        "platoon_story",
        "follow_act",
        "follower1_group",
        "follow1_maneuver",
        "maintain_headway1",
        "follower1",
        "leader",
        1.5,
        Rule::LessThan,
        true,
    )?;

    scenario.add_speed_profile_action(
        "platoon_story",
        "follow_act",
        "follower1_group",
        "follow1_maneuver",
        "maintain_headway1",
        vec![(0.0, 25.0), (2.0, 22.0)], // Adjust speed to maintain distance
        true,
    )?;

    // Follower 2 group
    scenario.add_maneuver_group("platoon_story", "follow_act", "follower2_group")?;
    scenario.add_actor("platoon_story", "follow_act", "follower2_group", "follower2")?;
    scenario.add_maneuver(
        "platoon_story",
        "follow_act",
        "follower2_group",
        "follow2_maneuver",
    )?;

    // Follower 2: Maintain 1.5s headway from follower1
    scenario.add_event_with_time_headway_condition(
        "platoon_story",
        "follow_act",
        "follower2_group",
        "follow2_maneuver",
        "maintain_headway2",
        "follower2",
        "follower1",
        1.5,
        Rule::LessThan,
        true,
    )?;

    scenario.add_speed_profile_action(
        "platoon_story",
        "follow_act",
        "follower2_group",
        "follow2_maneuver",
        "maintain_headway2",
        vec![(0.0, 25.0), (2.0, 22.0)], // Adjust speed to maintain distance
        true,
    )?;

    // Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("platooning.xosc", xml)?;

    println!("✅ Platooning scenario exported to platooning.xosc");
    println!("   - Leader: 100m ahead");
    println!("   - Follower 1: 50m ahead, follows leader");
    println!("   - Follower 2: Starting position, follows follower 1");
    println!("   - Time headway: 1.5 seconds for all followers");
    println!();
    println!("Visualization:");
    println!("  Three vehicles travel in formation, each maintaining 1.5s");
    println!("  time headway from the vehicle ahead. When the leader slows,");
    println!("  all followers adjust speed to maintain consistent spacing.");

    Ok(())
}
