//! Emergency Braking - Vehicle performs emergency stop
//!
//! Demonstrates:
//! - Collision condition detection
//! - Acceleration action (negative for braking)
//! - Safety-critical scenario testing
//!
//! Scenario: A vehicle detects collision risk and performs emergency braking
//!
//! Run with: cargo run --example emergency_braking

use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::{OpenScenarioVersion, Position, Scenario};

fn main() -> Result<(), openscenario::ScenarioError> {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add two vehicles
    let vehicle_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    // Lead vehicle (ahead)
    scenario.add_vehicle("lead_vehicle", vehicle_params.clone())?;
    scenario.set_initial_position("lead_vehicle", Position::world(50.0, 0.0, 0.0, 0.0))?;

    // Following vehicle (needs to brake)
    scenario.add_vehicle("follower_vehicle", vehicle_params)?;
    scenario.set_initial_position("follower_vehicle", Position::world(0.0, 0.0, 0.0, 0.0))?;

    // Create story structure
    scenario.add_story("emergency_story")?;
    scenario.add_act("emergency_story", "brake_act")?;
    scenario.add_maneuver_group("emergency_story", "brake_act", "follower_group")?;
    scenario.add_actor("emergency_story", "brake_act", "follower_group", "follower_vehicle")?;
    scenario.add_maneuver(
        "emergency_story",
        "brake_act",
        "follower_group",
        "emergency_brake_maneuver",
    )?;

    // Condition: Collision risk detected
    scenario.add_event_with_collision_condition(
        "emergency_story",
        "brake_act",
        "follower_group",
        "emergency_brake_maneuver",
        "collision_risk",
        "follower_vehicle",  // Entity being monitored
        "lead_vehicle",      // Target entity
    )?;

    // Action: Emergency brake (strong deceleration: -8 m/s²)
    scenario.add_acceleration_action(
        "emergency_story",
        "brake_act",
        "follower_group",
        "emergency_brake_maneuver",
        "collision_risk", // Same event name links condition to action
        -8.0,             // Deceleration: -8 m/s² (emergency braking)
        2.0,              // Duration: 2 seconds
        None,             // No specific dynamics
    )?;

    // Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("emergency_braking.xosc", xml)?;

    println!("✅ Emergency braking scenario exported to emergency_braking.xosc");
    println!("   - Lead vehicle: 50m ahead");
    println!("   - Follower vehicle: Starting position");
    println!("   - Condition: Collision risk detected");
    println!("   - Action: Emergency brake at -8 m/s² for 2s");
    println!();
    println!("Visualization:");
    println!("  Follower detects collision risk and performs emergency");
    println!("  braking to avoid rear-end collision.");

    Ok(())
}
