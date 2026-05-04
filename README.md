# OpenSCENARIO MCP Server

Rust library and MCP server for generating and validating OpenSCENARIO test scenarios for autonomous driving.

## Features

- **Multi-version support**: OpenSCENARIO 1.0, 1.1, 1.2 with version detection
- **Entity management**: Vehicle, Pedestrian, MiscObject with properties
- **Position types**: All 7 OpenSCENARIO position types
  - Absolute: World, Lane, Road
  - Relative: RelativeWorld, RelativeObject, RelativeLane, RelativeRoad
- **Storyboard hierarchy**: Story → Act → ManeuverGroup → Maneuver → Event → Action
- **Actions**: Speed, LaneChange
- **Triggers**: Simulation time, storyboard element state, parameter conditions
- **Parameters**: Declare and reference typed parameters (Integer, Double, String, Boolean)
- **Catalog loading**: Read and reference external XOSC vehicle/pedestrian/controller catalogs
- **XSD validation**: Validate scenarios against OpenSCENARIO 1.0/1.1/1.2 XSD schemas
- **XML export**: Generate valid OpenSCENARIO 1.x XML files
- **Fail-fast validation**: Immediate feedback on entity conflicts and missing references
- **MCP server**: Model Context Protocol server for AI agent integration with 7 tools:
  - `create_scenario` - Create new OpenSCENARIO scenarios (versions 1.0, 1.1, 1.2)
  - `add_vehicle` - Add vehicles with catalog support
  - `set_position` - Set entity world positions
  - `add_speed_action` - Add speed change actions to stories
  - `add_lane_change_action` - Add lane change actions to stories
  - `export_xml` - Export scenarios to valid .xosc files
  - `validate_scenario` - Validate against XSD schemas

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

The MCP server provides 7 tools for AI-driven scenario generation:

```bash
# Start the MCP server
cargo run --release --bin openscenario-mcp
```

**Available Tools:**

1. **create_scenario** - Create a new scenario
   ```json
   {"name": "highway_scenario", "version": "1.2"}
   ```

2. **add_vehicle** - Add a vehicle (with optional catalog)
   ```json
   {
     "scenario_id": "scenario_abc123",
     "name": "ego",
     "category": "Car",
     "catalog": "VehicleCatalog.xosc:vehicle.car.bmw.3_series"
   }
   ```

3. **set_position** - Set entity initial position
   ```json
   {
     "scenario_id": "scenario_abc123",
     "entity_name": "ego",
     "x": 0.0, "y": 0.0, "z": 0.0, "h": 0.0
   }
   ```

4. **add_speed_action** - Add speed change action
   ```json
   {
     "scenario_id": "scenario_abc123",
     "entity_name": "ego",
     "story_name": "main_story",
     "speed": 50.0,
     "duration": 5.0
   }
   ```

5. **add_lane_change_action** - Add lane change action
   ```json
   {
     "scenario_id": "scenario_abc123",
     "entity_name": "ego",
     "story_name": "main_story",
     "target_lane": 3.5,
     "duration": 4.0
   }
   ```

6. **export_xml** - Export to .xosc file
   ```json
   {
     "scenario_id": "scenario_abc123",
     "output_path": "output/scenario.xosc"
   }
   ```

7. **validate_scenario** - Validate against XSD
   ```json
   {"scenario_id": "scenario_abc123"}
   ```

The server communicates via JSON-RPC over stdio, compatible with AI agents and OpenClaw.

See [openscenario-mcp/README.md](openscenario-mcp/README.md) for detailed MCP setup instructions.

## Testing

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration_test

# Run with verbose output
cargo test -- --nocapture
```

Test coverage: **73 tests** covering:
- Version detection and comparison
- Error handling (all error types)
- Entity management (conflict detection, validation)
- Position types (all 7 types)
- Storyboard hierarchy (stories, acts, maneuvers)
- Actions (speed, lane change)
- XML export (structure, entities, positions, actions)
- Integration workflows (end-to-end scenarios)

## Roadmap

- OpenDRIVE integration for road/lane validation
- Additional actions (Position, Distance, Teleport, etc.)
- Additional conditions (Speed, ReachPosition, etc.)
- Pedestrian/MiscObject-specific actions
- Catalog creation and modification
- Advanced OpenDRIVE features (junctions, signals)
- Scenario visualization
- Performance optimizations

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
