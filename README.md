# OpenSCENARIO MCP Server

A comprehensive Rust library and MCP server for generating, validating, and exporting OpenSCENARIO test scenarios for autonomous driving simulation.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

## Features

### Core Library (`openscenario`)

- ✅ **Multi-version support**: OpenSCENARIO 1.0, 1.1, 1.2 with automatic version detection
- ✅ **Complete entity system**: Vehicle, Pedestrian, MiscObject with full property support
- ✅ **All position types**: World, Lane, Road, and 4 Relative position variants
- ✅ **Full storyboard hierarchy**: Story → Act → ManeuverGroup → Maneuver → Event → Action
- ✅ **11 action types**: Speed, LaneChange, Acceleration, LaneOffset, FollowTrajectory, AssignRoute, Synchronize, Position, Distance, LongitudinalDistance, SpeedProfile
- ✅ **12 condition types**: ReachPosition, TimeToCollision, Collision, RelativeDistance, TimeHeadway, StandStill (each with simple + advanced variants)
- ✅ **Advanced triggers**: Simulation time, element state, parameter conditions, entity conditions
- ✅ **Type-safe parameters**: Integer, Double, String, Boolean with conflict detection
- ✅ **Catalog support**: Load and reference external XOSC catalogs
- ✅ **XSD validation**: Validate against OpenSCENARIO 1.0/1.1/1.2 schemas
- ✅ **XML export**: Generate spec-compliant OpenSCENARIO XML
- ✅ **Fail-fast validation**: Immediate feedback on conflicts and missing references
- ✅ **100% documented**: Complete rustdoc coverage for all 136 public APIs

### MCP Server (`openscenario-mcp`)

- 🔄 **AI-driven scenario generation** via Model Context Protocol
- 🔄 **7 MCP tools** for scenario construction and validation
- 🔄 **JSON-RPC over stdio** for seamless AI agent integration

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/jakeaboganda/osc-mcp.git
cd osc-mcp

# Build the project
cargo build --release

# Run tests
cargo test
```

### Hello World Example

Create a simple scenario with one vehicle:

```rust
use openscenario::{Scenario, OpenScenarioVersion, Position};
use openscenario::entities::{VehicleParams, VehicleCategory};

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
    
    // Create story structure
    scenario.add_story("main_story")?;
    
    // Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("hello_world.xosc", xml)?;
    
    println!("✅ Scenario exported to hello_world.xosc");
    Ok(())
}
```

### Lane Change Example

Create a vehicle that performs a lane change:

```rust
use openscenario::{Scenario, OpenScenarioVersion, Position};
use openscenario::entities::{VehicleParams, VehicleCategory};
use openscenario::storyboard::{TransitionShape, DynamicsShape, DynamicsDimension};

fn main() -> Result<(), openscenario::ScenarioError> {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    // Add ego vehicle
    let vehicle_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle_params)?;
    scenario.set_initial_position("ego", Position::lane("road1", -1, 0.0, 0.0, None))?;
    
    // Create story and maneuver structure
    scenario.add_story("main_story")?;
    scenario.add_act("main_story", "act1")?;
    scenario.add_maneuver_group("main_story", "act1", "mg1")?;
    scenario.add_actor("main_story", "act1", "mg1", "ego")?;
    scenario.add_maneuver("main_story", "act1", "mg1", "lane_change_maneuver")?;
    
    // Add lane change action (change to lane -2, taking 4 seconds)
    scenario.add_lane_change_action(
        "main_story",
        "act1", 
        "mg1",
        "lane_change_maneuver",
        "lane_change_event",
        -2,         // target lane ID
        4.0,        // transition time
        DynamicsShape::Sinusoidal,
        DynamicsDimension::Time,
    )?;
    
    // Export
    let xml = scenario.to_xml()?;
    std::fs::write("lane_change.xosc", xml)?;
    
    println!("✅ Lane change scenario exported");
    Ok(())
}
```

### Adaptive Cruise Control (ACC) Example

Create a following vehicle with time headway condition:

```rust
use openscenario::{Scenario, OpenScenarioVersion, Position};
use openscenario::entities::{VehicleParams, VehicleCategory};
use openscenario::storyboard::Rule;

fn main() -> Result<(), openscenario::ScenarioError> {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    // Add two vehicles: lead and follower
    let vehicle_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("lead", vehicle_params.clone())?;
    scenario.add_vehicle("follower", vehicle_params)?;
    
    // Position vehicles: lead at 50m ahead
    scenario.set_initial_position("follower", Position::world(0.0, 0.0, 0.0, 0.0))?;
    scenario.set_initial_position("lead", Position::world(50.0, 0.0, 0.0, 0.0))?;
    
    // Create story structure
    scenario.add_story("main_story")?;
    scenario.add_act("main_story", "act1")?;
    scenario.add_maneuver_group("main_story", "act1", "mg1")?;
    scenario.add_actor("main_story", "act1", "mg1", "follower")?;
    scenario.add_maneuver("main_story", "act1", "mg1", "acc_maneuver")?;
    
    // Add time headway condition: trigger when following too closely
    scenario.add_event_with_time_headway_condition(
        "main_story",
        "act1",
        "mg1",
        "acc_maneuver",
        "too_close_event",
        "follower",
        "lead",
        2.0,           // 2-second time headway threshold
        Rule::LessThan,
        true,          // freespace (use bounding boxes)
    )?;
    
    // Add speed profile to slow down when too close
    scenario.add_speed_profile_action(
        "main_story",
        "act1",
        "mg1",
        "acc_maneuver",
        "too_close_event",
        vec![
            (0.0, 25.0),  // t=0s: 25 m/s
            (3.0, 20.0),  // t=3s: slow to 20 m/s
        ],
        true,  // time-based
    )?;
    
    // Export
    let xml = scenario.to_xml()?;
    std::fs::write("acc_scenario.xosc", xml)?;
    
    println!("✅ ACC scenario with time headway condition exported");
    Ok(())
}
```

## Documentation

### API Documentation

Full rustdoc documentation is available for all 136 public APIs:

```bash
# Generate and open documentation
cargo doc --open
```

**Documented modules:**
- `scenario` - Main Scenario API (43 methods)
- `storyboard` - Story, Act, Maneuver, Event, Action, Condition types (54 items)
- `entities` - Vehicle, Pedestrian, MiscObject (10 items)
- `position` - All 7 position types (11 items)
- `error` - ScenarioError variants (16 variants)
- `xml` - XML export utilities (2 functions)

### Key API Methods

#### Scenario Construction

```rust
// Create scenario
Scenario::new(version: OpenScenarioVersion) -> Scenario

// Add entities
scenario.add_vehicle(name, params) -> Result<()>
scenario.add_pedestrian(name, params) -> Result<()>
scenario.add_misc_object(name, params) -> Result<()>

// Set positions
scenario.set_initial_position(entity, position) -> Result<()>

// Create story structure
scenario.add_story(name) -> Result<()>
scenario.add_act(story, name) -> Result<()>
scenario.add_maneuver_group(story, act, name) -> Result<()>
scenario.add_actor(story, act, mg, entity) -> Result<()>
scenario.add_maneuver(story, act, mg, name) -> Result<()>
```

#### Actions

```rust
// Speed and lane changes
scenario.add_speed_action(...) -> Result<()>
scenario.add_lane_change_action(...) -> Result<()>
scenario.add_acceleration_action(...) -> Result<()>
scenario.add_lane_offset_action(...) -> Result<()>

// Trajectory and routing
scenario.add_follow_trajectory_action(...) -> Result<()>
scenario.add_assign_route_action(...) -> Result<()>

// Positioning and distance
scenario.add_position_action(...) -> Result<()>
scenario.add_distance_action(...) -> Result<()>
scenario.add_longitudinal_distance_action(...) -> Result<()>
scenario.add_synchronize_action(...) -> Result<()>

// Speed profiles
scenario.add_speed_profile_action(...) -> Result<()>
```

#### Conditions

```rust
// Position and collision
scenario.add_event_with_reach_position_condition(...) -> Result<()>
scenario.add_event_with_collision_condition(...) -> Result<()>

// Distance and headway
scenario.add_event_with_relative_distance_condition(...) -> Result<()>
scenario.add_event_with_time_headway_condition(...) -> Result<()>
scenario.add_event_with_standstill_condition(...) -> Result<()>

// Time to collision
scenario.add_event_with_ttc_condition(...) -> Result<()>

// Each condition has an "advanced" variant for fine-grained control
scenario.add_event_with_*_condition_advanced(...) -> Result<()>
```

## Project Structure

```
osc-mcp/
├── openscenario/              # Core Rust library
│   ├── src/
│   │   ├── lib.rs             # Public API exports
│   │   ├── scenario.rs        # Main Scenario API (43 methods)
│   │   ├── storyboard.rs      # OpenSCENARIO types (54 items)
│   │   ├── entities.rs        # Entity system (10 items)
│   │   ├── position.rs        # Position types (11 items)
│   │   ├── error.rs           # Error types (16 variants)
│   │   ├── xml.rs             # XML export (2 functions)
│   │   ├── version.rs         # Version detection
│   │   └── opendrive_validator.rs  # OpenDRIVE validation
│   ├── tests/                 # 311 passing tests
│   │   ├── integration_test.rs
│   │   ├── relative_distance_condition_test.rs
│   │   ├── time_headway_condition_test.rs
│   │   ├── standstill_condition_test.rs
│   │   └── ...
│   └── examples/              # Usage examples
├── openscenario-mcp/          # MCP server
│   └── src/
│       ├── main.rs            # MCP server entry point
│       ├── server.rs          # Server infrastructure
│       └── tools.rs           # MCP tool handlers
└── README.md                  # This file
```

## Testing

```bash
# Run all tests (311 tests)
cargo test

# Run doc tests (125 tests)
cargo test --doc

# Run integration tests
cargo test --test integration_test

# Run specific test
cargo test test_time_headway_condition

# Verbose output
cargo test -- --nocapture
```

**Test Coverage:**
- **311 unit/integration tests** covering all features
- **125 documentation tests** ensuring examples compile and run
- **100% public API documentation** with runnable examples

## MCP Server

The MCP server provides AI-driven scenario generation capabilities via the Model Context Protocol.

### Available Tools

1. **create_scenario** - Create a new scenario
2. **add_vehicle** - Add vehicles with catalog support
3. **set_position** - Set entity positions
4. **add_speed_action** - Add speed change actions
5. **add_lane_change_action** - Add lane change actions
6. **export_xml** - Export to .xosc files
7. **validate_scenario** - Validate against XSD schemas

### Usage

```bash
# Start the MCP server
cargo run --release --bin openscenario-mcp

# The server communicates via JSON-RPC over stdio
# Connect from OpenClaw, Claude Desktop, or any MCP-compatible client
```

See [openscenario-mcp/README.md](openscenario-mcp/README.md) for detailed setup instructions.

## Roadmap

### ✅ Completed (Milestone 1-3)
- [x] Core scenario builder API
- [x] 11 action types
- [x] 6 condition types (12 variants with simple/advanced)
- [x] All 7 position types
- [x] Entity system (Vehicle/Pedestrian/MiscObject)
- [x] XML export
- [x] 100% API documentation with examples
- [x] 311 passing tests

### 🔄 In Progress (Milestone 4-5)
- [ ] Additional condition types (Speed, Distance, etc.)
- [ ] OpenDRIVE road network integration
- [ ] Controller actions
- [ ] Phase 2 documentation (README, examples, guides)

### 📋 Planned
- [ ] Pedestrian-specific actions
- [ ] Catalog creation and modification
- [ ] Advanced OpenDRIVE features (junctions, signals)
- [ ] Scenario visualization
- [ ] Performance optimizations
- [ ] Additional MCP tools

## Contributing

This project follows **Test-Driven Development (TDD)**:

1. **Write tests first** - Define expected behavior with tests
2. **Implement to pass tests** - Write minimal code to satisfy tests
3. **Review for quality** - Check spec compliance and code health
4. **Document thoroughly** - Add rustdoc comments with examples
5. **Commit with convention** - Use `feat:`, `fix:`, `test:`, `docs:`, etc.

### Development Guidelines

- Maintain **100% public API documentation** with runnable examples
- Ensure **all tests pass** before committing (`cargo test`)
- Follow **Rust best practices** (rustfmt, clippy)
- **OpenSCENARIO 1.2 compliance** is the specification authority

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## References

- [OpenSCENARIO 1.2 Specification](https://www.asam.net/standards/detail/openscenario/)
- [OpenDRIVE Specification](https://www.asam.net/standards/detail/opendrive/)
- [Model Context Protocol](https://modelcontextprotocol.io/)
- [Rust Documentation](https://doc.rust-lang.org/)

---

**Status**: 🚀 Production-ready API with comprehensive documentation and 311 passing tests.
