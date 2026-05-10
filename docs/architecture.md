# Architecture Guide

This document explains the internal architecture of the osc-mcp OpenSCENARIO library.

---

## Table of Contents

- [Project Structure](#project-structure)
- [Module Responsibilities](#module-responsibilities)
- [Design Patterns](#design-patterns)
- [Error Handling Philosophy](#error-handling-philosophy)
- [XML Export Pipeline](#xml-export-pipeline)
- [Testing Strategy](#testing-strategy)
- [Extension Points](#extension-points)

---

## Project Structure

```
osc-mcp/
├── openscenario/           # Core library crate
│   ├── src/
│   │   ├── scenario.rs     # Main API (2066 lines, 136 public methods)
│   │   ├── storyboard.rs   # Story hierarchy types (54 items)
│   │   ├── entities.rs     # Vehicle/Pedestrian/MiscObject (10 items)
│   │   ├── position.rs     # Position types (11 items)
│   │   ├── error.rs        # Error types (16 variants)
│   │   ├── xml.rs          # XML serialization (1232 lines)
│   │   └── lib.rs          # Public API exports
│   ├── tests/              # 302 integration tests
│   ├── examples/           # 4 runnable examples
│   └── Cargo.toml
├── openscenario-mcp/       # MCP server crate
│   ├── src/
│   │   └── main.rs         # MCP server implementation
│   └── Cargo.toml
├── docs/                   # Documentation
│   ├── getting-started.md
│   ├── architecture.md (this file)
│   └── cookbook/
└── README.md               # Project overview
```

**Key metrics**:
- **Total LOC**: ~5,000 lines of Rust
- **Public API surface**: 136 documented methods
- **Test coverage**: 436 tests (302 integration + 125 doc + 9 unit)
- **Documentation coverage**: 100% of public API

---

## Module Responsibilities

### `scenario.rs` - The Main API

**Purpose**: Provides the user-facing API for building OpenSCENARIO scenarios.

**Key responsibilities**:
1. **Entity management**: Add vehicles, pedestrians, misc objects
2. **Position management**: Set initial positions for entities
3. **Story construction**: Build Story → Act → ManeuverGroup → Maneuver hierarchy
4. **Action creation**: Add 11 types of actions (speed, lane change, acceleration, etc.)
5. **Condition creation**: Add 12 types of conditions (time headway, collision, distance, etc.)
6. **Validation**: Fail-fast checks for invalid configurations
7. **XML export**: Delegate to `xml.rs` for serialization

**Design principle**: *Builder pattern with validation*

```rust
// Every method validates immediately
pub fn add_vehicle(&mut self, name: impl Into<String>, params: VehicleParams) -> Result<()> {
    let entity_name = name.into().trim().to_string();
    
    // Validate: non-empty name
    if entity_name.is_empty() {
        return Err(ScenarioError::InvalidValue { ... });
    }
    
    // Validate: unique name
    if self.entities.contains_key(&entity_name) {
        return Err(ScenarioError::DuplicateEntity { ... });
    }
    
    // Add to internal state
    self.entities.insert(entity_name, Entity::Vehicle(params));
    Ok(())
}
```

**State management**:
- `entities`: HashMap of all entities
- `init`: Initial positions and speeds
- `storyboard`: Story hierarchy
- `catalogs`: External catalog references
- `parameters`: Scenario parameters

---

### `storyboard.rs` - Story Hierarchy Types

**Purpose**: Define the types that represent OpenSCENARIO's story structure.

**Key types**:

```rust
// Story hierarchy
pub struct Story { pub name: String, pub acts: HashMap<String, Act> }
pub struct Act { pub name: String, pub maneuver_groups: HashMap<String, ManeuverGroup> }
pub struct ManeuverGroup { pub name: String, pub actors: Vec<String>, pub maneuvers: HashMap<String, Maneuver> }
pub struct Maneuver { pub name: String, pub events: HashMap<String, Event> }
pub struct Event { pub name: String, pub actions: Vec<Action>, pub conditions: Vec<Condition> }
```

**Enums for actions**:
- `Action`: Speed, LaneChange, Acceleration, LaneOffset, etc.
- `Condition`: ReachPosition, TimeToCollision, RelativeDistance, etc.

**Enums for configuration**:
- `Rule`: LessThan, GreaterThan, EqualTo
- `TransitionShape`: Linear, Cubic, Sinusoidal, Step
- `DynamicsShape` / `DynamicsDimension`: For dynamic behavior specification

**Design principle**: *Type-safe representation*

Each enum variant carries its specific parameters:

```rust
pub enum Action {
    Speed { target_speed: f64, transition_dynamics: TransitionDynamics },
    LaneChange { target_lane_offset: f64, transition_dynamics: TransitionDynamics },
    Acceleration { value: f64, transition_dynamics: TransitionDynamics },
    // ... 8 more variants
}
```

This prevents invalid combinations at compile time.

---

### `entities.rs` - Entity Definitions

**Purpose**: Define the types for entities (vehicles, pedestrians, misc objects).

**Key types**:

```rust
pub enum Entity {
    Vehicle(VehicleParams),
    Pedestrian(PedestrianParams),
    MiscObject(MiscObjectParams),
}

pub struct VehicleParams {
    pub catalog: Option<CatalogReference>,
    pub vehicle_category: VehicleCategory,
    pub properties: Option<HashMap<String, String>>,
}

pub enum VehicleCategory {
    Car, Truck, Bus, Motorbike, Bicycle, Semitrailer, Trailer, Van,
}
```

**Design principle**: *Extensibility with catalog support*

Entities can be defined:
1. **Inline**: Specify parameters directly
2. **From catalog**: Reference external catalog definitions

```rust
// Inline definition
let inline = VehicleParams {
    catalog: None,
    vehicle_category: VehicleCategory::Car,
    properties: Some(hashmap! { "mass" => "1500" }),
};

// Catalog reference
let from_catalog = VehicleParams {
    catalog: Some(CatalogReference {
        catalog_name: "VehicleCatalog".to_string(),
        entry_name: "car1".to_string(),
    }),
    vehicle_category: VehicleCategory::Car,
    properties: None,
};
```

---

### `position.rs` - Position Types

**Purpose**: Define all position types supported by OpenSCENARIO.

**Position types**:

```rust
pub enum Position {
    World(WorldPosition),           // Absolute x, y, z
    Lane(LanePosition),             // Road, lane, s, offset
    Road(RoadPosition),             // Road, s, t
    RelativeWorld(RelativeWorldPosition),     // Relative to entity (world coords)
    RelativeLane(RelativeLanePosition),       // Relative to entity (lane coords)
    RelativeRoad(RelativeRoadPosition),       // Relative to entity (road coords)
    RelativeObject(RelativeObjectPosition),   // Relative to entity (object space)
}
```

**Design principle**: *Convenience constructors*

```rust
impl Position {
    pub fn world(x: f64, y: f64, z: f64, h: f64) -> Self { ... }
    pub fn lane(road: impl Into<String>, lane_id: i32, s: f64, offset: f64, ...) -> Self { ... }
    pub fn road(road: impl Into<String>, s: f64, t: f64, ...) -> Self { ... }
    // ... 4 more convenience constructors
}
```

Users can construct positions ergonomically:

```rust
Position::world(0.0, 0.0, 0.0, 0.0)
Position::lane("road1", -1, 50.0, 0.0, None)
```

---

### `error.rs` - Error Handling

**Purpose**: Define all error types with detailed context.

**Error variants**:

```rust
#[derive(Debug, thiserror::Error)]
pub enum ScenarioError {
    #[error("Invalid value for {field}: {reason}")]
    InvalidValue { field: String, reason: String },
    
    #[error("Duplicate entity: {name}")]
    DuplicateEntity { name: String },
    
    #[error("Entity not found: {name}")]
    EntityNotFound { name: String },
    
    #[error("Story not found: {name} (available: {available:?})")]
    StoryNotFound { name: String, available: Vec<String> },
    
    // ... 12 more variants
}
```

**Design principle**: *Fail-fast with helpful context*

Every error includes:
1. **What went wrong**: Clear error message
2. **Why it failed**: Specific reason with values
3. **How to fix**: Context like available options

Example:

```rust
// User tries to add action to non-existent story
Err(ScenarioError::StoryNotFound {
    name: "highway".to_string(),
    available: vec!["main_story".to_string()],
})
// Error: Story not found: highway (available: ["main_story"])
```

---

### `xml.rs` - XML Serialization

**Purpose**: Convert internal `Scenario` representation to OpenSCENARIO XML.

**Key responsibilities**:
1. **Version detection**: Support OpenSCENARIO 1.0, 1.1, 1.2
2. **XSD compliance**: Generate spec-compliant XML structure
3. **Pretty printing**: Readable XML output with proper indentation
4. **Namespace handling**: Correct XML namespaces and schema locations

**Pipeline**:

```
Scenario (internal representation)
    ↓
to_xml_internal() - Build XML tree
    ↓
serialize_with_namespaces() - Add namespaces
    ↓
pretty_print() - Format with indentation
    ↓
String (OpenSCENARIO XML)
```

**Design principle**: *Separate concerns*

XML generation is completely isolated from scenario construction. The `Scenario` struct knows nothing about XML - it's a pure business logic representation.

**Key challenge**: OpenSCENARIO XML structure differences across versions

```rust
match self.version {
    OpenScenarioVersion::V1_0 => {
        // V1.0 uses <Private> element
        writer.write_element("Private", ...)?;
    }
    OpenScenarioVersion::V1_1 | OpenScenarioVersion::V1_2 => {
        // V1.1+ uses <PrivateAction>
        writer.write_element("PrivateAction", ...)?;
    }
}
```

---

## Design Patterns

### 1. Builder Pattern

The `Scenario` struct follows the builder pattern:

```rust
let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
scenario.add_vehicle("ego", params)?;
scenario.set_initial_position("ego", position)?;
scenario.add_story("main")?;
// ... build incrementally
let xml = scenario.to_xml()?;
```

**Benefits**:
- Fluent API (chain methods)
- Incremental construction
- Validation at each step

### 2. Type-Safe Enums

Instead of strings or integers, use enums:

```rust
// ❌ Error-prone: typos, invalid values
add_action("speed", 30.0, "linear");

// ✅ Type-safe: compile-time validation
add_action(Action::Speed {
    target_speed: 30.0,
    transition_dynamics: TransitionDynamics {
        shape: DynamicsShape::Linear,
        dimension: DynamicsDimension::Time,
    },
});
```

### 3. Fail-Fast Validation

Errors are caught immediately, not at export time:

```rust
// Invalid name detected immediately
scenario.add_vehicle("", params)?;  // Error: empty name

// Not later during XML export
let xml = scenario.to_xml()?;  // Would fail here if validation delayed
```

**Benefit**: Users get immediate feedback during development.

### 4. Internal State Encapsulation

Users interact with simple methods, complexity is hidden:

```rust
// Simple user-facing API
scenario.add_lane_change_action(...)?;

// Complex internal state management (hidden)
impl Scenario {
    fn add_lane_change_action(...) -> Result<()> {
        // 1. Validate story exists
        // 2. Validate act exists
        // 3. Validate maneuver group exists
        // 4. Validate maneuver exists
        // 5. Validate entity is an actor
        // 6. Create Action enum variant
        // 7. Add to event's action list
        // 8. Update internal state
    }
}
```

---

## Error Handling Philosophy

### Principle: Fail-Fast, Fail-Loud

**Goal**: Catch errors as early as possible with maximum context.

**Why**:
1. **Developer experience**: Immediate feedback during coding
2. **Debugging**: Clear error messages with context
3. **Correctness**: Invalid scenarios can't be constructed

**Example**:

```rust
// ❌ Silent failure (bad)
scenario.add_vehicle("ego", params);  // Silently ignores error

// ❌ Generic error (bad)
scenario.add_vehicle("ego", params).expect("Failed");  // No context

// ✅ Explicit error handling (good)
scenario.add_vehicle("ego", params)
    .map_err(|e| eprintln!("Failed to add vehicle: {}", e))?;

// Error output:
// Failed to add vehicle: Duplicate entity: ego
```

### Error Categories

1. **Invalid input**: User provided bad data
   - Empty names
   - Negative durations
   - Out-of-range values

2. **Missing dependencies**: Referenced item doesn't exist
   - Story not found
   - Entity not found
   - Act not found

3. **Duplicate items**: Name collision
   - Duplicate entity
   - Duplicate story
   - Duplicate parameter

4. **XML errors**: Serialization failures
   - IO errors
   - Encoding errors

---

## XML Export Pipeline

### High-Level Flow

```
┌─────────────┐
│  Scenario   │  (Internal representation)
└──────┬──────┘
       │
       │ to_xml()
       ▼
┌─────────────┐
│  XML Tree   │  (In-memory XML structure)
└──────┬──────┘
       │
       │ serialize()
       ▼
┌─────────────┐
│ Raw XML     │  (String with namespaces)
└──────┬──────┘
       │
       │ pretty_print()
       ▼
┌─────────────┐
│ Formatted   │  (Indented, readable XML)
│     XML     │
└─────────────┘
```

### XML Structure

OpenSCENARIO 1.2 structure:

```xml
<OpenSCENARIO>
  <FileHeader />
  <CatalogLocations />
  <RoadNetwork />
  <Entities>
    <ScenarioObject />
    ...
  </Entities>
  <Storyboard>
    <Init>
      <Actions>
        <Private />
        ...
      </Actions>
    </Init>
    <Story>
      <Act>
        <ManeuverGroup>
          <Actors />
          <Maneuver>
            <Event>
              <Action />
              <StartTrigger>
                <ConditionGroup>
                  <Condition />
                </ConditionGroup>
              </StartTrigger>
            </Event>
          </Maneuver>
        </ManeuverGroup>
        <StartTrigger />
      </Act>
    </Story>
    <StopTrigger />
  </Storyboard>
</OpenSCENARIO>
```

### Version Handling

Different OpenSCENARIO versions have structural differences:

```rust
match version {
    V1_0 => {
        // V1.0: <Dimensions /> element
        write_dimensions(writer)?;
    }
    V1_1 | V1_2 => {
        // V1.1+: <BoundingBox> with <Dimensions />
        write_bounding_box(writer)?;
    }
}
```

The `xml.rs` module handles these differences internally.

---

## Testing Strategy

### Test Pyramid

```
      ┌───────────┐
      │ Doc Tests │  125 tests (inline examples)
      └─────┬─────┘
         ┌──┴──┐
         │ Unit│     9 tests (module-level)
         └──┬──┘
      ┌─────┴─────┐
      │Integration│  302 tests (full scenarios)
      └───────────┘
```

### 1. Integration Tests (302 tests)

**Location**: `openscenario/tests/*.rs`

**Purpose**: Test complete scenario workflows

**Pattern**:

```rust
#[test]
fn test_lane_change_scenario() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    // Build complete scenario
    scenario.add_vehicle("ego", params).unwrap();
    scenario.set_initial_position("ego", position).unwrap();
    scenario.add_story("main").unwrap();
    // ... full setup
    
    scenario.add_lane_change_action(...).unwrap();
    
    // Validate XML output
    let xml = scenario.to_xml().unwrap();
    assert!(xml.contains("<LateralAction>"));
    assert!(xml.contains("targetLaneOffset=\"-1.0\""));
}
```

**Coverage**: Every action type, every condition type, error cases

### 2. Documentation Tests (125 tests)

**Location**: Inline in rustdoc comments

**Purpose**: Ensure examples in documentation compile and run

**Pattern**:

```rust
/// # Examples
/// ```
/// use openscenario::{Scenario, OpenScenarioVersion};
///
/// # fn main() -> Result<(), openscenario::ScenarioError> {
/// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
/// // Example code that MUST compile and run
/// # Ok(())
/// # }
/// ```
```

**Coverage**: All 136 public API methods

### 3. Unit Tests (9 tests)

**Location**: Inline in source files (`#[cfg(test)]` modules)

**Purpose**: Test internal helper functions

**Pattern**:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_internal_helper() {
        let result = internal_function(input);
        assert_eq!(result, expected);
    }
}
```

**Coverage**: Parsing, validation logic, edge cases

### Test-Driven Development (TDD)

The library was built following TDD:

1. **Write tests first** (9-12 tests per feature)
2. **Implement feature** to make tests pass
3. **Refactor** while keeping tests green
4. **Add documentation** with doc tests

**Example**: When adding `TimeHeadwayCondition`:

```rust
// Step 1: Write tests (9 tests)
- test_time_headway_basic()
- test_time_headway_less_than()
- test_time_headway_greater_than()
- test_time_headway_invalid_threshold()
- test_time_headway_invalid_entity()
- test_time_headway_missing_lead()
- test_time_headway_xml_output()
- test_time_headway_freespace_true()
- test_time_headway_freespace_false()

// Step 2: Implement until all pass

// Step 3: Add rustdoc with examples
```

---

## Extension Points

### Adding a New Action Type

1. **Add enum variant** in `storyboard.rs`:

```rust
pub enum Action {
    // ... existing variants
    NewAction {
        param1: f64,
        param2: String,
    },
}
```

2. **Add public method** in `scenario.rs`:

```rust
pub fn add_new_action(
    &mut self,
    story: impl Into<String>,
    // ... hierarchy parameters
    param1: f64,
    param2: impl Into<String>,
) -> Result<()> {
    // Validation
    if param1 <= 0.0 {
        return Err(ScenarioError::InvalidValue { ... });
    }
    
    // Create action
    let action = Action::NewAction {
        param1,
        param2: param2.into(),
    };
    
    // Add to event
    self.add_action_to_event(story, act, mg, maneuver, event, action)
}
```

3. **Add XML serialization** in `xml.rs`:

```rust
Action::NewAction { param1, param2 } => {
    writer.start_element("NewAction")?;
    writer.write_attribute("param1", &param1.to_string())?;
    writer.write_attribute("param2", param2)?;
    writer.end_element()?;
}
```

4. **Write tests** in `tests/new_action_test.rs`:

```rust
#[test]
fn test_new_action_basic() { ... }

#[test]
fn test_new_action_invalid_param() { ... }

// ... 9-12 tests total
```

5. **Add rustdoc** with example:

```rust
/// Add a new action to an event.
///
/// # Examples
/// ```
/// # use openscenario::*;
/// # fn main() -> Result<(), ScenarioError> {
/// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
/// scenario.add_new_action("story", "act", "mg", "maneuver", "event", 1.0, "value")?;
/// # Ok(())
/// # }
/// ```
```

### Adding a New Condition Type

Follow the same pattern as actions:
1. Enum variant in `storyboard.rs`
2. Public method in `scenario.rs`
3. XML serialization in `xml.rs`
4. Tests in `tests/`
5. Rustdoc with examples

### Adding a New Entity Type

If adding a new entity category:

1. **Add to enum** in `entities.rs`:

```rust
pub enum Entity {
    Vehicle(VehicleParams),
    Pedestrian(PedestrianParams),
    MiscObject(MiscObjectParams),
    NewType(NewTypeParams),  // Add here
}
```

2. **Add constructor** in `scenario.rs`:

```rust
pub fn add_new_type(&mut self, name: impl Into<String>, params: NewTypeParams) -> Result<()> {
    // Similar to add_vehicle implementation
}
```

3. **Add XML serialization** for new entity type

4. **Add tests** for new entity type

---

## Performance Considerations

### Memory Usage

- **HashMap for lookups**: O(1) entity/story/act lookups
- **String ownership**: Uses `String` for names (not `&str`) to avoid lifetime complexity
- **Cloning**: Minimal - most operations use references

### XML Generation

- **Single-pass**: XML is generated in one traversal
- **String building**: Uses `String::with_capacity` for efficiency
- **No validation during export**: All validation happens during construction

### Scalability

**Current limits** (soft, no hard enforcement):
- Entities: Hundreds (typical scenarios: 2-10)
- Stories: Tens (typical: 1-3)
- Actions per scenario: Hundreds (typical: 10-50)

**Memory footprint**: ~1-10 KB per scenario (small)

---

## Summary

The osc-mcp architecture follows these principles:

1. **Separation of concerns**: API ↔ Types ↔ XML are independent
2. **Type safety**: Enums prevent invalid states
3. **Fail-fast validation**: Errors caught early with context
4. **Builder pattern**: Incremental, fluent construction
5. **Comprehensive testing**: 436 tests covering all features
6. **100% documentation**: Every public method documented with examples

**Next steps**:
- Explore the [Cookbook](cookbook/) for specific use cases
- Read the [API documentation](`cargo doc --open`)
- Check the [examples](../openscenario/examples/) for complete scenarios
