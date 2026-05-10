//! Hello World - Simplest OpenSCENARIO example
//!
//! Creates a minimal scenario with one vehicle at a starting position.
//!
//! Run with: cargo run --example hello_world

use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::{OpenScenarioVersion, Position, Scenario};

fn main() -> Result<(), openscenario::ScenarioError> {
    // Create a new OpenSCENARIO 1.2 scenario
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);

    // Add a vehicle entity
    let vehicle_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params)?;

    // Set initial position (world coordinates: x, y, z, heading)
    scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))?;

    // Create story structure (required for valid OpenSCENARIO)
    scenario.add_story("main_story")?;

    // Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("hello_world.xosc", xml)?;

    println!("✅ Scenario exported to hello_world.xosc");
    println!("   - 1 vehicle (ego)");
    println!("   - Starting at origin (0, 0, 0)");
    println!("   - OpenSCENARIO 1.2 format");

    Ok(())
}
