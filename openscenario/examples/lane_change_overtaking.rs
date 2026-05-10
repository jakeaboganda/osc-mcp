//! Lane Change Overtaking - Vehicle overtaking a slower vehicle
//!
//! Demonstrates:
//! - Two-vehicle interaction
//! - Speed-based decision making
//! - Multi-step overtaking maneuver
//!
//! Scenario: A faster vehicle approaches a slower vehicle and overtakes it
//!
//! Run with: cargo run --example lane_change_overtaking

use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::TransitionShape;
use openscenario::{OpenScenarioVersion, Position, Scenario};

fn main() -> Result<(), openscenario::ScenarioError> {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add two vehicles
    let vehicle_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };

    // Slow vehicle ahead in right lane
    scenario.add_vehicle("slow_vehicle", vehicle_params.clone())?;
    scenario.set_initial_position("slow_vehicle", Position::world(100.0, 0.0, 0.0, 0.0))?;

    // Fast vehicle behind in right lane
    scenario.add_vehicle("fast_vehicle", vehicle_params)?;
    scenario.set_initial_position("fast_vehicle", Position::world(0.0, 0.0, 0.0, 0.0))?;

    // Create story structure for overtaking vehicle
    scenario.add_story("overtake_story")?;
    scenario.add_act("overtake_story", "overtake_act")?;
    scenario.add_maneuver_group("overtake_story", "overtake_act", "fast_group")?;
    scenario.add_actor("overtake_story", "overtake_act", "fast_group", "fast_vehicle")?;
    scenario.add_maneuver("overtake_story", "overtake_act", "fast_group", "overtake_maneuver")?;

    // Step 1: Change to left lane to overtake
    scenario.add_lane_change_action(
        "overtake_story",
        "overtake_act",
        "fast_group",
        "overtake_maneuver",
        "move_left",
        1.0, // Move one lane to the left
        3.0, // Take 3 seconds
        TransitionShape::Sinusoidal,
    )?;

    // Step 2: Return to right lane after passing
    scenario.add_lane_change_action(
        "overtake_story",
        "overtake_act",
        "fast_group",
        "overtake_maneuver",
        "move_right",
        -1.0, // Move one lane to the right (back to original)
        3.0,  // Take 3 seconds
        TransitionShape::Sinusoidal,
    )?;

    // Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("lane_change_overtaking.xosc", xml)?;

    println!("✅ Overtaking scenario exported to lane_change_overtaking.xosc");
    println!("   - Slow vehicle: 100m ahead, right lane");
    println!("   - Fast vehicle: Starting position, right lane");
    println!("   - Action 1: Move left to overtake (3s)");
    println!("   - Action 2: Return to right lane (3s)");
    println!();
    println!("Visualization:");
    println!("  Fast vehicle approaches slow vehicle, changes to left lane,");
    println!("  passes the slow vehicle, then returns to right lane.");

    Ok(())
}
