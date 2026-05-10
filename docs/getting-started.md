# Getting Started with OpenSCENARIO MCP

This guide will help you create your first OpenSCENARIO test scenario using the osc-mcp library.

---

## Prerequisites

- **Rust**: Version 1.70 or higher
  ```bash
  rustc --version  # Should show 1.70+
  ```
- **Cargo**: Rust's package manager (comes with Rust)
- **Git**: For cloning the repository

---

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/jakeaboganda/osc-mcp.git
cd osc-mcp
```

### 2. Build the Project

```bash
cargo build --release
```

This will compile the library and all examples. First build may take a few minutes.

### 3. Run the Tests

```bash
cargo test
```

You should see **436 tests passing**. This confirms everything is working correctly.

### 4. Try an Example

```bash
cargo run --example hello_world
```

You should see:
```
✅ Scenario exported to hello_world.xosc
   - 1 vehicle (ego)
   - Starting at origin (0, 0, 0)
   - OpenSCENARIO 1.2 format
```

**Congratulations!** You've just generated your first OpenSCENARIO XML file.

---

## Core Concepts

Before diving into code, let's understand the key concepts in OpenSCENARIO scenarios.

### 1. Scenario

The top-level container for everything. Think of it as the "world" where your test takes place.

```rust
let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
```

### 2. Entities

The actors in your scenario: vehicles, pedestrians, or miscellaneous objects.

```rust
// Add a car
let vehicle_params = VehicleParams {
    catalog: None,
    vehicle_category: VehicleCategory::Car,
    properties: None,
};
scenario.add_vehicle("ego", vehicle_params)?;
```

**Key point**: Every entity needs a unique name (like `"ego"`).

### 3. Positions

Where entities start in the world. Three main types:

- **World coordinates**: Absolute x, y, z position
  ```rust
  Position::world(0.0, 0.0, 0.0, 0.0)  // x, y, z, heading
  ```

- **Lane coordinates**: Position on a road lane
  ```rust
  Position::lane("road1", -1, 0.0, 0.0, None)  // road, lane, s, offset
  ```

- **Road coordinates**: Position along a road
  ```rust
  Position::road("road1", 100.0, 0.0, None)  // road, s, t
  ```

### 4. Story Hierarchy

OpenSCENARIO organizes actions in a hierarchy:

```
Story
└── Act
    └── ManeuverGroup
        └── Maneuver
            └── Event
                ├── Condition (when to trigger)
                └── Action (what to do)
```

Think of it like a movie script:
- **Story**: The overall narrative ("Highway ACC Test")
- **Act**: A scene or phase ("Following Behavior")
- **ManeuverGroup**: Actions for a group of actors ("Follower Actions")
- **Maneuver**: A specific sequence ("Maintain Distance")
- **Event**: A triggered action ("Slow Down When Too Close")

### 5. Actions

What entities do: change speed, change lanes, follow a trajectory, etc.

```rust
// Change lanes
scenario.add_lane_change_action(
    "story", "act", "group", "maneuver", "event",
    -1.0,  // Move one lane left
    5.0,   // Take 5 seconds
    TransitionShape::Linear,
)?;
```

### 6. Conditions

Triggers that activate events: reaching a position, detecting collision risk, time passing, etc.

```rust
// Trigger when time headway < 2 seconds
scenario.add_event_with_time_headway_condition(
    "story", "act", "group", "maneuver", "event",
    "follower", "lead",
    2.0,  // 2-second threshold
    Rule::LessThan,
    true,
)?;
```

---

## Your First Scenario: Step by Step

Let's build a simple lane change scenario from scratch.

### Step 1: Create a New Rust Project

```bash
cargo new my_scenario
cd my_scenario
```

### Step 2: Add the Dependency

Edit `Cargo.toml`:

```toml
[dependencies]
openscenario = { path = "../osc-mcp/openscenario" }
# Or once published: openscenario = "0.1"
```

### Step 3: Write the Scenario

Edit `src/main.rs`:

```rust
use openscenario::{Scenario, OpenScenarioVersion, Position};
use openscenario::entities::{VehicleParams, VehicleCategory};
use openscenario::storyboard::TransitionShape;

fn main() -> Result<(), openscenario::ScenarioError> {
    // Step 1: Create the scenario
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    // Step 2: Add a vehicle
    let vehicle = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", vehicle)?;
    
    // Step 3: Set starting position
    scenario.set_initial_position(
        "ego",
        Position::world(0.0, 0.0, 0.0, 0.0)  // Start at origin
    )?;
    
    // Step 4: Create the story structure
    scenario.add_story("main_story")?;
    scenario.add_act("main_story", "act1")?;
    scenario.add_maneuver_group("main_story", "act1", "ego_group")?;
    scenario.add_actor("main_story", "act1", "ego_group", "ego")?;
    scenario.add_maneuver("main_story", "act1", "ego_group", "lane_change")?;
    
    // Step 5: Add the lane change action
    scenario.add_lane_change_action(
        "main_story",
        "act1",
        "ego_group",
        "lane_change",
        "change_left",
        -1.0,                        // Move one lane to the left
        5.0,                         // Take 5 seconds
        TransitionShape::Linear,     // Linear transition
    )?;
    
    // Step 6: Export to XML
    let xml = scenario.to_xml()?;
    std::fs::write("my_scenario.xosc", xml)?;
    
    println!("✅ Scenario exported to my_scenario.xosc");
    Ok(())
}
```

### Step 4: Run It

```bash
cargo run
```

You should see:
```
✅ Scenario exported to my_scenario.xosc
```

### Step 5: Inspect the Output

```bash
cat my_scenario.xosc
```

You'll see valid OpenSCENARIO 1.2 XML describing your lane change scenario.

---

## Understanding the Story Structure

The story hierarchy might seem complex at first. Here's why each level exists:

### Why Story?

Groups related behaviors. You might have:
- `"highway_entry"` - Story about merging onto highway
- `"highway_cruise"` - Story about maintaining speed
- `"highway_exit"` - Story about exiting

### Why Act?

Represents phases within a story. For "highway_cruise":
- `"act1"` - Accelerate to cruising speed
- `"act2"` - Maintain speed
- `"act3"` - Respond to traffic

### Why ManeuverGroup?

Groups actors that coordinate together:
- `"lead_vehicle_group"` - Controls the lead vehicle
- `"follower_group"` - Controls following vehicles

### Why Maneuver?

A specific behavior sequence:
- `"lane_change_left"` - Complete left lane change
- `"emergency_brake"` - Brake hard

### Why Event?

The actual trigger + action:
- `"change_when_clear"` - Condition: lane clear → Action: change lanes
- `"brake_on_collision_risk"` - Condition: TTC < 2s → Action: brake

---

## Common Patterns

### Pattern 1: Single Vehicle Test

```rust
// Minimal structure for testing one vehicle
scenario.add_story("test")?;
```

That's it! You can add actions directly without creating acts/groups/maneuvers if you're testing a single vehicle behavior in isolation.

### Pattern 2: Two-Vehicle Interaction

```rust
// Lead vehicle
scenario.add_vehicle("lead", vehicle_params.clone())?;
scenario.set_initial_position("lead", Position::world(50.0, 0.0, 0.0, 0.0))?;

// Follower
scenario.add_vehicle("follower", vehicle_params)?;
scenario.set_initial_position("follower", Position::world(0.0, 0.0, 0.0, 0.0))?;

// Structure for follower actions
scenario.add_story("main")?;
scenario.add_act("main", "follow")?;
scenario.add_maneuver_group("main", "follow", "follower_group")?;
scenario.add_actor("main", "follow", "follower_group", "follower")?;
```

### Pattern 3: Condition + Action

```rust
// Trigger an action when a condition is met
scenario.add_event_with_time_headway_condition(
    "main", "act1", "group", "maneuver",
    "too_close",      // Event name
    "follower",       // Entity to monitor
    "lead",           // Lead vehicle
    2.0,              // Threshold
    Rule::LessThan,   // Trigger when < threshold
    true,             // Use bounding boxes
)?;

// Add action to the same event
scenario.add_speed_profile_action(
    "main", "act1", "group", "maneuver",
    "too_close",      // Same event name links condition to action
    vec![(0.0, 30.0), (3.0, 25.0)],  // Speed profile
    true,
)?;
```

**Key insight**: Use the same event name to link conditions with actions.

---

## Running Scenarios in esmini

esmini is an open-source OpenSCENARIO player that can visualize and execute your scenarios.

### 1. Install esmini

Download from: https://github.com/esmini/esmini/releases

### 2. Run Your Scenario

```bash
./esmini --osc my_scenario.xosc
```

You'll see a 3D visualization of your vehicle performing the lane change.

### 3. Useful esmini Options

```bash
# Run with specific road network
./esmini --osc my_scenario.xosc --odr my_road.xodr

# Record to video
./esmini --osc my_scenario.xosc --record my_test.mp4

# Headless mode (no visualization)
./esmini --osc my_scenario.xosc --headless
```

---

## Common Pitfalls

### 1. Forgetting the Story

```rust
// ❌ This will fail - no story created
scenario.add_act("main", "act1")?;  // Error: story "main" doesn't exist
```

```rust
// ✅ Always create story first
scenario.add_story("main")?;
scenario.add_act("main", "act1")?;
```

### 2. Duplicate Entity Names

```rust
// ❌ This will fail - duplicate name
scenario.add_vehicle("ego", params.clone())?;
scenario.add_vehicle("ego", params)?;  // Error: "ego" already exists
```

```rust
// ✅ Use unique names
scenario.add_vehicle("ego", params.clone())?;
scenario.add_vehicle("lead", params)?;
```

### 3. Invalid Lane Offset

```rust
// ❌ This will fail - lane offset must be float
scenario.add_lane_change_action(..., -1, ...)?;  // Error: expected f64
```

```rust
// ✅ Use float literal
scenario.add_lane_change_action(..., -1.0, ...)?;
```

### 4. Missing Entity in Story

```rust
// ❌ This will fail - entity not added to maneuver group
scenario.add_maneuver_group("main", "act1", "group")?;
scenario.add_maneuver("main", "act1", "group", "maneuver")?;
// Trying to add action for "ego" but never called add_actor
```

```rust
// ✅ Add actor to maneuver group first
scenario.add_maneuver_group("main", "act1", "group")?;
scenario.add_actor("main", "act1", "group", "ego")?;  // Add actor
scenario.add_maneuver("main", "act1", "group", "maneuver")?;
```

---

## Next Steps

Now that you understand the basics:

1. **Explore the examples**:
   ```bash
   cargo run --example adaptive_cruise_control
   ```
   
2. **Read the API documentation**:
   ```bash
   cargo doc --open
   ```
   
3. **Check the architecture guide**: [`docs/architecture.md`](architecture.md)

4. **Browse the cookbook**: [`docs/cookbook/`](cookbook/) for recipes on common tasks

---

## Getting Help

- **Examples**: `openscenario/examples/*.rs` - Runnable code samples
- **API Docs**: `cargo doc --open` - Full rustdoc for all 136 methods
- **Tests**: `openscenario/tests/*.rs` - 436 test cases showing usage patterns
- **Issues**: GitHub issues for bugs or questions

---

## Summary

You've learned:
- ✅ How to install and set up osc-mcp
- ✅ Core OpenSCENARIO concepts (entities, positions, story hierarchy)
- ✅ How to create your first scenario step-by-step
- ✅ Common patterns for single and multi-vehicle tests
- ✅ How to link conditions with actions
- ✅ How to run scenarios in esmini
- ✅ Common mistakes to avoid

**Next**: Read [`architecture.md`](architecture.md) to understand how the library works internally.
