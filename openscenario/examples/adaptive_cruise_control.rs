//! Adaptive Cruise Control (ACC) Example
//!
//! Demonstrates:
//! - Two-vehicle scenario (lead and follower)
//! - Time headway condition to maintain safe following distance
//! - Speed profile action for ACC behavior
//!
//! The follower vehicle maintains a 2-second time headway from the lead vehicle
//! using a time headway condition and speed adjustment.
//!
//! Run with: cargo run --example adaptive_cruise_control

use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::Rule;
use openscenario::{OpenScenarioVersion, Position, Scenario};

fn main() -> Result<(), openscenario::ScenarioError> {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add two vehicles: lead and follower
    let vehicle_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    scenario.add_vehicle("lead_vehicle", vehicle_params.clone())?;
    scenario.add_vehicle("follower_vehicle", vehicle_params)?;

    // Set initial positions (lead ahead of follower by 50m)
    scenario.set_initial_position("lead_vehicle", Position::world(50.0, 0.0, 0.0, 0.0))?;
    scenario.set_initial_position("follower_vehicle", Position::world(0.0, 0.0, 0.0, 0.0))?;

    // Create story structure
    scenario.add_story("acc_story")?;
    scenario.add_act("acc_story", "act1")?;
    scenario.add_maneuver_group("acc_story", "act1", "follower_group")?;
    scenario.add_actor("acc_story", "act1", "follower_group", "follower_vehicle")?;
    scenario.add_maneuver("acc_story", "act1", "follower_group", "acc_maneuver")?;

    // Add time headway condition: trigger when following too closely (< 2 seconds)
    // Time headway = distance / follower_speed
    scenario.add_event_with_time_headway_condition(
        "acc_story",
        "act1",
        "follower_group",
        "acc_maneuver",
        "too_close_event",
        "follower_vehicle", // Entity being monitored
        "lead_vehicle",     // Lead vehicle to measure gap to
        2.0,                // Threshold: 2 seconds
        Rule::LessThan,     // Trigger when headway < 2 seconds
        true,               // freespace: measure to bounding box
    )?;

    // Add speed profile action: reduce speed to maintain safe distance
    // Speed profile: (time, speed) pairs
    scenario.add_speed_profile_action(
        "acc_story",
        "act1",
        "follower_group",
        "acc_maneuver",
        "adjust_speed",
        vec![
            (0.0, 30.0),  // Start at 30 m/s
            (3.0, 25.0),  // Reduce to 25 m/s over 3 seconds
            (5.0, 25.0),  // Maintain 25 m/s
        ],
        true, // following_mode: time-based speed adjustments
    )?;

    // Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("adaptive_cruise_control.xosc", xml)?;

    println!("✅ ACC scenario exported to adaptive_cruise_control.xosc");
    println!("   - Lead vehicle: 50m ahead");
    println!("   - Follower vehicle: Starting at origin");
    println!("   - Time headway threshold: 2.0 seconds");
    println!("   - ACC behavior: Reduce speed when too close");
    println!("   - Speed adjustment: 30 m/s → 25 m/s over 3 seconds");
    println!();
    println!("Time headway measures the time gap between vehicles:");
    println!("  time_headway = distance / follower_speed");
    println!();
    println!("When the gap drops below 2 seconds, the follower slows down");
    println!("to maintain a safe following distance.");

    Ok(())
}
