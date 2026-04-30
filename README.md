# OpenSCENARIO MCP Server

Rust library and MCP server for generating and validating OpenSCENARIO test scenarios for autonomous driving.

## Features

### Phase 1 (v1.0) ✅

- **Multi-version support**: OpenSCENARIO 1.0, 1.1, 1.2 with version detection
- **Entity management**: Vehicle, Pedestrian, MiscObject with properties
- **Position types**: All 7 OpenSCENARIO position types
  - Absolute: World, Lane, Road
  - Relative: RelativeWorld, RelativeObject, RelativeLane, RelativeRoad
- **Storyboard hierarchy**: Story → Act → ManeuverGroup → Maneuver → Event → Action
- **Actions**: Speed, LaneChange (more coming in Phase 2)
- **XML export**: Generate valid OpenSCENARIO 1.x XML files
- **Fail-fast validation**: Immediate feedback on entity conflicts and missing references
- **MCP server**: Model Context Protocol server for AI agent integration

## Project Structure

```
osc-mcp/
├── openscenario/          # Core Rust library
│   ├── src/
│   │   ├── entities.rs    # Vehicle, Pedestrian, MiscObject
│   │   ├── position.rs    # All position types
│   │   ├── storyboard.rs  # Story, Act, ManeuverGroup, etc.
│   │   ├── scenario.rs    # Main Scenario API
│   │   ├── version.rs     # Version enum
│   │   ├── error.rs       # Error types
│   │   └── xml.rs         # XML export
│   ├── tests/             # Comprehensive test suite
│   └── examples/          # Usage examples
├── openscenario-mcp/      # MCP server
│   └── src/
│       ├── main.rs        # MCP server entry point
│       ├── server.rs      # Server infrastructure
│       └── tools.rs       # MCP tool handlers (Phase 2)
└── README.md              # This file
```

## Installation

```bash
# Clone the repository
git clone https://github.com/jakeaboganda/osc-mcp.git
cd osc-mcp

# Build the project
cargo build --release

# Run tests
cargo test
```

## Usage

### Library Example

```rust
use openscenario::{Scenario, OpenScenarioVersion, Position};
use openscenario::entities::{VehicleParams, VehicleCategory};
use openscenario::storyboard::TransitionShape;

// Create scenario
let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

// Add vehicle
let params = VehicleParams {
    catalog: None,
    vehicle_category: VehicleCategory::Car,
    properties: None,
};
scenario.add_vehicle("ego", params)?;

// Set initial position
let pos = Position::world(0.0, 0.0, 0.0, 0.0);
scenario.set_initial_position("ego", pos)?;

// Create story
scenario.add_story("main_story")?;
scenario.add_act("main_story", "act1")?;
scenario.add_maneuver_group("main_story", "act1", "mg1")?;
scenario.add_actor("main_story", "act1", "mg1", "ego")?;
scenario.add_maneuver("main_story", "act1", "mg1", "maneuver1")?;

// Add speed action
scenario.add_speed_action(
    "main_story",
    "act1",
    "mg1",
    "maneuver1",
    "event1",
    50.0,              // target speed (m/s)
    5.0,               // transition duration (s)
    TransitionShape::Linear,
)?;

// Export to XML
let xml = scenario.to_xml()?;
std::fs::write("scenario.xosc", xml)?;
```

### MCP Server

```bash
# Start the MCP server
cargo run --release --bin openscenario-mcp
```

The server communicates via JSON-RPC over stdio, compatible with AI agents and OpenClaw.

## Testing

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration_test

# Run with verbose output
cargo test -- --nocapture
```

Test coverage: **32 tests** across 9 test files, covering:
- Version detection and comparison
- Error handling (all error types)
- Entity management (conflict detection, validation)
- Position types (all 7 types)
- Storyboard hierarchy (stories, acts, maneuvers)
- Actions (speed, lane change)
- XML export (structure, entities, positions, actions)
- Integration workflows (end-to-end scenarios)

## Roadmap

### Phase 2 (Next)
- Catalog loading (read-only)
- OpenDRIVE integration for road validation
- XSD validation
- More actions (Position, Distance, etc.)
- MCP tool implementations

### Phase 3+
- Pedestrian/MiscObject actions
- Conditions and triggers
- Parameters and value references
- Catalog creation
- Advanced OpenDRIVE features

## Contributing

This project follows Test-Driven Development (TDD):
1. Write tests first
2. Implement to pass tests
3. Review for spec compliance and code quality
4. Commit with conventional commits (`feat:`, `fix:`, `test:`, etc.)

## License

MIT OR Apache-2.0

## References

- [OpenSCENARIO Specification](https://www.asam.net/standards/detail/openscenario/)
- [Model Context Protocol](https://modelcontextprotocol.io/)
