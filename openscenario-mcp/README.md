# OpenSCENARIO MCP Server

Model Context Protocol (MCP) server for AI-driven OpenSCENARIO scenario generation.

## Overview

This MCP server exposes 7 tools that allow AI agents to create, modify, validate, and export OpenSCENARIO test scenarios. It manages scenario state in-memory and provides a JSON-RPC interface over stdio.

## Installation

```bash
# Build the MCP server
cd openscenario-mcp
cargo build --release

# The binary will be at:
# target/release/openscenario-mcp
```

## Usage

### Starting the Server

```bash
cargo run --release --bin openscenario-mcp
```

The server communicates via **JSON-RPC over stdio**:
- Reads requests from stdin
- Writes responses to stdout
- Logs to stderr (use `RUST_LOG=info` for detailed logs)

### Integration with OpenClaw

Add to your OpenClaw MCP server configuration:

```json
{
  "mcpServers": {
    "openscenario": {
      "command": "cargo",
      "args": ["run", "--release", "--bin", "openscenario-mcp"],
      "cwd": "/path/to/osc-mcp/openscenario-mcp",
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

## Available Tools

### 1. create_scenario

Create a new OpenSCENARIO scenario.

**Parameters:**
- `name` (string, required): Scenario name
- `version` (string, required): OpenSCENARIO version (`"1.0"`, `"1.1"`, or `"1.2"`)

**Returns:** Scenario ID (string)

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "create_scenario",
    "arguments": {
      "name": "highway_overtake",
      "version": "1.2"
    }
  },
  "id": 1
}
```

### 2. add_vehicle

Add a vehicle entity to a scenario.

**Parameters:**
- `scenario_id` (string, required): Scenario ID from `create_scenario`
- `name` (string, required): Vehicle name (must be unique)
- `category` (string, required): Vehicle category (`"Car"`, `"Truck"`, `"Bus"`, `"Trailer"`, `"Van"`, `"Motorbike"`, `"Bicycle"`)
- `catalog` (string, optional): Catalog reference in format `"path:entry_name"` (e.g., `"VehicleCatalog.xosc:vehicle.car.bmw.3_series"`)

**Returns:** Success message

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "add_vehicle",
    "arguments": {
      "scenario_id": "scenario_abc123",
      "name": "ego",
      "category": "Car",
      "catalog": "VehicleCatalog.xosc:vehicle.car.bmw.3_series"
    }
  },
  "id": 2
}
```

### 3. set_position

Set the initial world position for an entity.

**Parameters:**
- `scenario_id` (string, required): Scenario ID
- `entity_name` (string, required): Entity name (must exist)
- `x` (number, required): X coordinate (meters)
- `y` (number, required): Y coordinate (meters)
- `z` (number, required): Z coordinate (meters)
- `h` (number, required): Heading (radians)

**Returns:** Success message

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "set_position",
    "arguments": {
      "scenario_id": "scenario_abc123",
      "entity_name": "ego",
      "x": 0.0,
      "y": 0.0,
      "z": 0.0,
      "h": 0.0
    }
  },
  "id": 3
}
```

### 4. add_speed_action

Add a speed change action to a scenario. Creates story/act/maneuver structure automatically if it doesn't exist.

**Parameters:**
- `scenario_id` (string, required): Scenario ID
- `entity_name` (string, required): Entity to apply action to
- `story_name` (string, required): Story name (created if doesn't exist)
- `speed` (number, required): Target speed in m/s
- `duration` (number, required): Transition duration in seconds

**Returns:** Success message

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "add_speed_action",
    "arguments": {
      "scenario_id": "scenario_abc123",
      "entity_name": "ego",
      "story_name": "main_story",
      "speed": 50.0,
      "duration": 5.0
    }
  },
  "id": 4
}
```

### 5. add_lane_change_action

Add a lane change action to a scenario. Creates story/act/maneuver structure automatically if it doesn't exist.

**Parameters:**
- `scenario_id` (string, required): Scenario ID
- `entity_name` (string, required): Entity to apply action to
- `story_name` (string, required): Story name (created if doesn't exist)
- `target_lane` (number, required): Target lane offset in meters
- `duration` (number, required): Transition duration in seconds

**Returns:** Success message

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "add_lane_change_action",
    "arguments": {
      "scenario_id": "scenario_abc123",
      "entity_name": "ego",
      "story_name": "main_story",
      "target_lane": 3.5,
      "duration": 4.0
    }
  },
  "id": 5
}
```

### 6. export_xml

Export a scenario to an OpenSCENARIO XML file.

**Parameters:**
- `scenario_id` (string, required): Scenario ID
- `output_path` (string, required): Output file path (`.xosc` extension recommended)

**Returns:** Success message with file path

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "export_xml",
    "arguments": {
      "scenario_id": "scenario_abc123",
      "output_path": "output/highway_overtake.xosc"
    }
  },
  "id": 6
}
```

### 7. validate_scenario

Validate a scenario against the OpenSCENARIO XSD schema (version-specific).

**Parameters:**
- `scenario_id` (string, required): Scenario ID

**Returns:** Validation result (success or error details)

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "validate_scenario",
    "arguments": {
      "scenario_id": "scenario_abc123"
    }
  },
  "id": 7
}
```

## Complete Workflow Example

```json
// 1. Create scenario
{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "create_scenario", "arguments": {"name": "test", "version": "1.2"}}, "id": 1}

// 2. Add vehicle
{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "add_vehicle", "arguments": {"scenario_id": "scenario_abc123", "name": "ego", "category": "Car"}}, "id": 2}

// 3. Set position
{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "set_position", "arguments": {"scenario_id": "scenario_abc123", "entity_name": "ego", "x": 0.0, "y": 0.0, "z": 0.0, "h": 0.0}}, "id": 3}

// 4. Add speed action
{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "add_speed_action", "arguments": {"scenario_id": "scenario_abc123", "entity_name": "ego", "story_name": "main", "speed": 30.0, "duration": 3.0}}, "id": 4}

// 5. Validate
{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "validate_scenario", "arguments": {"scenario_id": "scenario_abc123"}}, "id": 5}

// 6. Export
{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "export_xml", "arguments": {"scenario_id": "scenario_abc123", "output_path": "test.xosc"}}, "id": 6}
```

## Architecture

### State Management

The server maintains scenario state in-memory using a global `ServerState`:

```rust
pub struct ServerState {
    pub scenarios: HashMap<String, Scenario>,
}
```

- Each scenario is identified by a unique ID (generated on creation)
- Scenarios persist for the lifetime of the MCP server process
- No disk persistence (use `export_xml` to save scenarios)

### Tool Handlers

Each tool is implemented as a handler function in `src/handlers.rs`:

- `handle_create_scenario` - Creates scenario, generates ID, stores in state
- `handle_add_vehicle` - Adds vehicle entity with catalog resolution
- `handle_set_position` - Sets entity initial position
- `handle_add_speed_action` - Creates story structure and adds speed action
- `handle_add_lane_change_action` - Creates story structure and adds lane change action
- `handle_export_xml` - Exports scenario XML to file
- `handle_validate_scenario` - Validates against XSD schema

### Error Handling

All errors are propagated as JSON-RPC error responses:
- Missing parameters → "Missing '<param>' parameter"
- Unknown scenario ID → "Scenario not found: <id>"
- Entity conflicts → "Entity '<name>' already exists"
- Validation errors → XSD validation details

## Development

### Running Tests

```bash
cargo test
```

### Logging

Set `RUST_LOG` environment variable for detailed logs:

```bash
RUST_LOG=debug cargo run --release --bin openscenario-mcp
```

Logs are written to **stderr** (stdout is reserved for JSON-RPC).

## Troubleshooting

**Q: Server not responding**
- Check that you're sending valid JSON-RPC 2.0 requests to stdin
- Ensure stdout/stderr are not redirected
- Check stderr for error logs

**Q: Scenario ID not found**
- Scenario IDs are session-specific (lost on restart)
- Always use the ID returned by `create_scenario`

**Q: XSD validation fails**
- Check scenario version matches expected XSD version
- Ensure all required entities/positions are set
- Use `export_xml` to inspect generated XML

**Q: Catalog references not working**
- Verify catalog path is relative or absolute file path
- Format must be `"path:entry_name"`
- Catalog must be valid XOSC catalog file

## References

- [Model Context Protocol Specification](https://modelcontextprotocol.io/)
- [OpenSCENARIO Specification](https://www.asam.net/standards/detail/openscenario/)
- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)

## License

MIT OR Apache-2.0
