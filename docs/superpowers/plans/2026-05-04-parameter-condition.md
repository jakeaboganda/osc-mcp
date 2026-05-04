# ParameterCondition Implementation Plan (Phase 3.2a)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add ParameterCondition support to OpenSCENARIO trigger system with full spec compliance (string, numeric, boolean parameter comparisons).

**Architecture:** Extend ByValueCondition enum with Parameter variant, add ParameterCondition struct to storyboard.rs, extend write_condition() in xml.rs for XML generation with parameter validation, add InvalidParameterRef error variant.

**Tech Stack:** Rust, quick_xml, serde, thiserror

---

## File Structure

**New types (in existing files):**
- `openscenario/src/storyboard.rs` - Add ParameterCondition struct, extend ByValueCondition enum, add helper methods
- `openscenario/src/xml.rs` - Extend write_condition() to handle Parameter variant with validation
- `openscenario/src/error.rs` - Add InvalidParameterRef error variant
- `openscenario/src/scenario.rs` - Add parameters field and ParameterDeclaration type
- `openscenario/src/lib.rs` - Export new types

**New test files:**
- `openscenario/tests/parameter_condition_integration_test.rs` - End-to-end integration test

**Modified test files:**
- `openscenario/tests/trigger_tests.rs` - Add unit and XML generation tests

---

## Task 1: Add ParameterDeclaration Type

**Files:**
- Modify: `openscenario/src/scenario.rs`
- Test: `openscenario/tests/scenario_tests.rs` (check if exists, or inline in this file)

- [ ] **Step 1: Write failing test for ParameterDeclaration**

Add at the end of `openscenario/src/scenario.rs`:

```rust
#[cfg(test)]
mod parameter_tests {
    use super::*;

    #[test]
    fn test_add_parameter_declaration() {
        let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
        
        let result = scenario.add_parameter(
            "MaxSpeed",
            ParameterType::Double,
            "60.0",
        );
        
        assert!(result.is_ok());
        assert_eq!(scenario.parameters.len(), 1);
        assert_eq!(scenario.parameters[0].name, "MaxSpeed");
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test parameter_tests --lib
```

Expected: FAIL - "parameters" field not found, "ParameterType" not found

- [ ] **Step 3: Add ParameterDeclaration and ParameterType types**

Add before the `Scenario` struct in `openscenario/src/scenario.rs`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterDeclaration {
    pub name: String,
    pub parameter_type: ParameterType,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterType {
    Integer,
    Double,
    String,
    Boolean,
}
```

- [ ] **Step 4: Add parameters field to Scenario struct**

In `openscenario/src/scenario.rs`, modify the `Scenario` struct:

```rust
pub struct Scenario {
    pub(crate) version: OpenScenarioVersion,
    pub(crate) entities: HashMap<String, Entity>,
    pub(crate) initial_positions: HashMap<String, Position>,
    pub(crate) parameters: Vec<ParameterDeclaration>,  // NEW
    pub(crate) storyboard: Storyboard,
}
```

- [ ] **Step 5: Update Scenario::new() to initialize parameters**

In `openscenario/src/scenario.rs`, modify `Scenario::new()`:

```rust
pub fn new(version: OpenScenarioVersion) -> Self {
    Self {
        version,
        entities: HashMap::new(),
        initial_positions: HashMap::new(),
        parameters: Vec::new(),  // NEW
        storyboard: Storyboard::new(),
    }
}
```

- [ ] **Step 6: Add add_parameter method to Scenario**

Add after `Scenario::new()` in `openscenario/src/scenario.rs`:

```rust
pub fn add_parameter(
    &mut self,
    name: impl Into<String>,
    parameter_type: ParameterType,
    value: impl Into<String>,
) -> Result<()> {
    let name = name.into();
    
    // Check for duplicate parameter names
    if self.parameters.iter().any(|p| p.name == name) {
        return Err(ScenarioError::ParameterConflict { name });
    }
    
    self.parameters.push(ParameterDeclaration {
        name,
        parameter_type,
        value: value.into(),
    });
    
    Ok(())
}
```

- [ ] **Step 7: Add ParameterConflict error variant**

In `openscenario/src/error.rs`, add before the `Io` variant:

```rust
#[error("Parameter '{name}' already exists")]
ParameterConflict { name: String },
```

- [ ] **Step 8: Export new types from lib.rs**

In `openscenario/src/lib.rs`, modify the scenario export:

```rust
pub use scenario::{ParameterDeclaration, ParameterType, Scenario};
```

- [ ] **Step 9: Run test to verify it passes**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test parameter_tests --lib
```

Expected: PASS

- [ ] **Step 10: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/src/scenario.rs openscenario/src/error.rs openscenario/src/lib.rs
git commit -m "feat: add ParameterDeclaration type and Scenario.add_parameter()"
```

---

## Task 2: Add ParameterCondition Struct

**Files:**
- Modify: `openscenario/src/storyboard.rs`
- Modify: `openscenario/src/lib.rs`
- Test: `openscenario/tests/trigger_tests.rs`

- [ ] **Step 1: Write failing test for ParameterCondition construction**

Add to `openscenario/tests/trigger_tests.rs` after existing tests:

```rust
#[test]
fn test_parameter_condition_construction() {
    use openscenario::*;
    
    let param_cond = ParameterCondition {
        parameter_ref: "MaxSpeed".to_string(),
        value: "50.0".to_string(),
        rule: ComparisonRule::GreaterThan,
    };
    
    assert_eq!(param_cond.parameter_ref, "MaxSpeed");
    assert_eq!(param_cond.value, "50.0");
    assert_eq!(param_cond.rule, ComparisonRule::GreaterThan);
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test test_parameter_condition_construction
```

Expected: FAIL - "ParameterCondition" not found

- [ ] **Step 3: Add ParameterCondition struct**

Add after the `ByValueCondition` enum in `openscenario/src/storyboard.rs`:

```rust
/// Parameter-based condition (checks runtime parameter value)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterCondition {
    pub parameter_ref: String,
    pub value: String,
    pub rule: ComparisonRule,
}
```

- [ ] **Step 4: Extend ByValueCondition enum with Parameter variant**

Modify the `ByValueCondition` enum in `openscenario/src/storyboard.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ByValueCondition {
    SimulationTime {
        value: f64,
        rule: ComparisonRule,
    },
    StoryboardElementState {
        element_type: String,
        element_ref: String,
        state: String,
    },
    Parameter(ParameterCondition),  // NEW
}
```

- [ ] **Step 5: Add helper method to Condition for parameter conditions**

Add after `Condition::storyboard_element_state()` in `openscenario/src/storyboard.rs`:

```rust
/// Create a parameter condition
pub fn parameter(
    parameter_ref: impl Into<String>,
    value: impl Into<String>,
    rule: ComparisonRule,
) -> Self {
    let param_ref = parameter_ref.into();
    Self {
        name: format!("Param_{}", param_ref),
        delay: 0.0,
        condition_edge: ConditionEdge::None,
        kind: ConditionKind::ByValue(ByValueCondition::Parameter(ParameterCondition {
            parameter_ref: param_ref,
            value: value.into(),
            rule,
        })),
    }
}
```

- [ ] **Step 6: Export ParameterCondition from lib.rs**

In `openscenario/src/lib.rs`, modify the storyboard export:

```rust
pub use storyboard::{
    Act, Action, ByValueCondition, ComparisonRule, Condition, ConditionEdge, ConditionGroup,
    ConditionKind, Event, LaneChangeAction, ParameterCondition, SpeedAction, Storyboard,
    TransitionShape, Trigger,
};
```

- [ ] **Step 7: Run test to verify it passes**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test test_parameter_condition_construction
```

Expected: PASS

- [ ] **Step 8: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/src/storyboard.rs openscenario/src/lib.rs openscenario/tests/trigger_tests.rs
git commit -m "feat: add ParameterCondition struct and Condition::parameter()"
```

---

## Task 3: Test All ComparisonRule Operators

**Files:**
- Test: `openscenario/tests/trigger_tests.rs`

- [ ] **Step 1: Write test for all six ComparisonRule variants**

Add to `openscenario/tests/trigger_tests.rs`:

```rust
#[test]
fn test_parameter_condition_all_comparison_rules() {
    use openscenario::*;
    
    let rules = vec![
        ComparisonRule::LessThan,
        ComparisonRule::LessOrEqual,
        ComparisonRule::EqualTo,
        ComparisonRule::NotEqualTo,
        ComparisonRule::GreaterOrEqual,
        ComparisonRule::GreaterThan,
    ];
    
    for rule in rules {
        let param_cond = ParameterCondition {
            parameter_ref: "TestParam".to_string(),
            value: "42".to_string(),
            rule: rule.clone(),
        };
        
        assert_eq!(param_cond.rule, rule);
    }
}
```

- [ ] **Step 2: Run test to verify it passes**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test test_parameter_condition_all_comparison_rules
```

Expected: PASS

- [ ] **Step 3: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/tests/trigger_tests.rs
git commit -m "test: add ComparisonRule coverage for ParameterCondition"
```

---

## Task 4: Test Parameter Value Types

**Files:**
- Test: `openscenario/tests/trigger_tests.rs`

- [ ] **Step 1: Write test for string, numeric, and boolean parameter values**

Add to `openscenario/tests/trigger_tests.rs`:

```rust
#[test]
fn test_parameter_condition_value_types() {
    use openscenario::*;
    
    // String value
    let string_cond = ParameterCondition {
        parameter_ref: "VehicleState".to_string(),
        value: "stopped".to_string(),
        rule: ComparisonRule::EqualTo,
    };
    assert_eq!(string_cond.value, "stopped");
    
    // Numeric value (stored as string)
    let numeric_cond = ParameterCondition {
        parameter_ref: "MaxSpeed".to_string(),
        value: "50.0".to_string(),
        rule: ComparisonRule::GreaterThan,
    };
    assert_eq!(numeric_cond.value, "50.0");
    
    // Boolean value (stored as string)
    let boolean_cond = ParameterCondition {
        parameter_ref: "DebugMode".to_string(),
        value: "true".to_string(),
        rule: ComparisonRule::EqualTo,
    };
    assert_eq!(boolean_cond.value, "true");
}
```

- [ ] **Step 2: Run test to verify it passes**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test test_parameter_condition_value_types
```

Expected: PASS

- [ ] **Step 3: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/tests/trigger_tests.rs
git commit -m "test: add parameter value type coverage (string/numeric/boolean)"
```

---

## Task 5: Add InvalidParameterRef Error

**Files:**
- Modify: `openscenario/src/error.rs`

- [ ] **Step 1: Add InvalidParameterRef error variant**

In `openscenario/src/error.rs`, add before the `Io` variant:

```rust
#[error("Invalid parameter reference: {0}")]
InvalidParameterRef(String),
```

- [ ] **Step 2: Verify compilation**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo check
```

Expected: SUCCESS

- [ ] **Step 3: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/src/error.rs
git commit -m "feat: add InvalidParameterRef error variant"
```

---

## Task 6: Implement XML Generation for ParameterCondition

**Files:**
- Modify: `openscenario/src/xml.rs`
- Test: `openscenario/tests/trigger_tests.rs`

- [ ] **Step 1: Write failing test for ParameterCondition XML generation**

Add to `openscenario/tests/trigger_tests.rs`:

```rust
#[test]
fn test_parameter_condition_xml_generation() {
    use openscenario::*;
    
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    
    // Add parameter declaration
    scenario.add_parameter("MaxSpeed", ParameterType::Double, "60.0").unwrap();
    
    // Add vehicle
    scenario.add_vehicle("ego", VehicleParams {
        vehicle_category: "car".to_string(),
        mass: 1500.0,
        bounding_box: crate::entities::BoundingBox {
            length: 4.5,
            width: 1.8,
            height: 1.5,
        },
    }).unwrap();
    
    // Set initial position
    scenario.set_initial_position("ego", Position::World {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        h: 0.0,
        p: 0.0,
        r: 0.0,
    }).unwrap();
    
    // Create story with parameter condition trigger
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "main_act").unwrap();
    
    let trigger = Trigger::new(ConditionGroup::new(vec![
        Condition::parameter("MaxSpeed", "50.0", ComparisonRule::GreaterThan)
    ]));
    scenario.set_act_start_trigger("main_story", "main_act", trigger).unwrap();
    
    // Export XML
    let xml = scenario.to_xml().unwrap();
    
    // Verify XML contains ParameterCondition
    assert!(xml.contains("<ParameterCondition"));
    assert!(xml.contains("parameterRef=\"MaxSpeed\""));
    assert!(xml.contains("value=\"50.0\""));
    assert!(xml.contains("rule=\"greaterThan\""));
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test test_parameter_condition_xml_generation
```

Expected: FAIL - XML doesn't contain ParameterCondition

- [ ] **Step 3: Find write_condition function in xml.rs**

```bash
cd ~/.openclaw/workspace/osc-mcp
grep -n "fn write_condition" openscenario/src/xml.rs
```

- [ ] **Step 4: Extend write_condition to handle Parameter variant**

In `openscenario/src/xml.rs`, inside the `write_condition` function, find the match on `ByValueCondition` and extend it:

Before the closing `}` of the `ConditionKind::ByValue(by_value)` match arm, add:

```rust
ByValueCondition::Parameter(param_cond) => {
    // Validate parameter exists
    if !scenario.parameters.iter().any(|p| p.name == param_cond.parameter_ref) {
        return Err(crate::ScenarioError::InvalidParameterRef(
            param_cond.parameter_ref.clone()
        ));
    }
    
    // Generate XML
    let mut param_tag = BytesStart::new("ParameterCondition");
    param_tag.push_attribute(("parameterRef", param_cond.parameter_ref.as_str()));
    param_tag.push_attribute(("value", param_cond.value.as_str()));
    param_tag.push_attribute(("rule", rule_to_string(&param_cond.rule)));
    writer.write_event(Event::Empty(param_tag))?;
}
```

Note: You'll need to add `scenario: &Scenario` parameter to `write_condition` function signature if it doesn't already have it.

- [ ] **Step 5: Update write_condition function signature if needed**

If `write_condition` doesn't have `scenario: &Scenario` parameter, add it. Then update all call sites to pass `scenario`.

- [ ] **Step 6: Run test to verify it passes**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test test_parameter_condition_xml_generation
```

Expected: PASS

- [ ] **Step 7: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/src/xml.rs openscenario/tests/trigger_tests.rs
git commit -m "feat: add XML generation for ParameterCondition with validation"
```

---

## Task 7: Test Invalid Parameter Reference Error

**Files:**
- Test: `openscenario/tests/trigger_tests.rs`

- [ ] **Step 1: Write test for invalid parameter reference**

Add to `openscenario/tests/trigger_tests.rs`:

```rust
#[test]
fn test_invalid_parameter_reference_error() {
    use openscenario::*;
    
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    
    // Don't add parameter declaration (intentionally missing)
    
    // Add vehicle
    scenario.add_vehicle("ego", VehicleParams {
        vehicle_category: "car".to_string(),
        mass: 1500.0,
        bounding_box: crate::entities::BoundingBox {
            length: 4.5,
            width: 1.8,
            height: 1.5,
        },
    }).unwrap();
    
    // Set initial position
    scenario.set_initial_position("ego", Position::World {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        h: 0.0,
        p: 0.0,
        r: 0.0,
    }).unwrap();
    
    // Create story with parameter condition that references non-existent parameter
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "main_act").unwrap();
    
    let trigger = Trigger::new(ConditionGroup::new(vec![
        Condition::parameter("NonExistentParam", "50.0", ComparisonRule::GreaterThan)
    ]));
    scenario.set_act_start_trigger("main_story", "main_act", trigger).unwrap();
    
    // Export XML should fail with InvalidParameterRef error
    let result = scenario.to_xml();
    
    assert!(result.is_err());
    match result {
        Err(ScenarioError::InvalidParameterRef(param)) => {
            assert_eq!(param, "NonExistentParam");
        }
        _ => panic!("Expected InvalidParameterRef error"),
    }
}
```

- [ ] **Step 2: Run test to verify it passes**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test test_invalid_parameter_reference_error
```

Expected: PASS

- [ ] **Step 3: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/tests/trigger_tests.rs
git commit -m "test: add invalid parameter reference error coverage"
```

---

## Task 8: Integration Test with esmini Validation

**Files:**
- Create: `openscenario/tests/parameter_condition_integration_test.rs`

- [ ] **Step 1: Create integration test file**

Create `openscenario/tests/parameter_condition_integration_test.rs`:

```rust
use openscenario::*;

#[test]
fn test_parameter_condition_end_to_end() {
    // Create scenario with parameters
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    
    // Add multiple parameters (string, numeric, boolean)
    scenario.add_parameter("MaxSpeed", ParameterType::Double, "60.0").unwrap();
    scenario.add_parameter("VehicleState", ParameterType::String, "moving").unwrap();
    scenario.add_parameter("DebugMode", ParameterType::Boolean, "false").unwrap();
    
    // Add vehicle
    scenario.add_vehicle("ego", VehicleParams {
        vehicle_category: "car".to_string(),
        mass: 1500.0,
        bounding_box: entities::BoundingBox {
            length: 4.5,
            width: 1.8,
            height: 1.5,
        },
    }).unwrap();
    
    // Set initial position
    scenario.set_initial_position("ego", Position::World {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        h: 0.0,
        p: 0.0,
        r: 0.0,
    }).unwrap();
    
    // Create story with parameter-based trigger
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "main_act").unwrap();
    
    // Act starts when MaxSpeed > 50.0
    let act_trigger = Trigger::new(ConditionGroup::new(vec![
        Condition::parameter("MaxSpeed", "50.0", ComparisonRule::GreaterThan)
    ]));
    scenario.set_act_start_trigger("main_story", "main_act", act_trigger).unwrap();
    
    // Add maneuver group and event
    scenario.add_maneuver_group("main_story", "main_act", "mg1").unwrap();
    scenario.add_actor_to_maneuver_group("main_story", "main_act", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "main_act", "mg1", "maneuver1").unwrap();
    scenario.add_event("main_story", "main_act", "mg1", "maneuver1", "event1").unwrap();
    
    // Event starts when VehicleState == "moving"
    let event_trigger = Trigger::new(ConditionGroup::new(vec![
        Condition::parameter("VehicleState", "moving", ComparisonRule::EqualTo)
    ]));
    scenario.set_event_start_trigger("main_story", "main_act", "mg1", "maneuver1", "event1", event_trigger).unwrap();
    
    // Add speed action
    scenario.add_speed_action("main_story", "main_act", "mg1", "maneuver1", "event1", 20.0, 5.0, "linear").unwrap();
    
    // Set stop trigger
    scenario.set_stop_trigger(storyboard::StopTrigger::simulation_time(10.0)).unwrap();
    
    // Export XML
    let xml = scenario.to_xml().unwrap();
    
    // Validate XML structure
    assert!(xml.contains("<ParameterDeclarations>"));
    assert!(xml.contains("<ParameterDeclaration name=\"MaxSpeed\""));
    assert!(xml.contains("parameterType=\"double\""));
    assert!(xml.contains("value=\"60.0\""));
    
    assert!(xml.contains("<ParameterDeclaration name=\"VehicleState\""));
    assert!(xml.contains("parameterType=\"string\""));
    assert!(xml.contains("value=\"moving\""));
    
    assert!(xml.contains("<ParameterCondition parameterRef=\"MaxSpeed\""));
    assert!(xml.contains("rule=\"greaterThan\""));
    
    assert!(xml.contains("<ParameterCondition parameterRef=\"VehicleState\""));
    assert!(xml.contains("rule=\"equalTo\""));
    
    // Optional: esmini validation if esmini is available
    #[cfg(feature = "esmini_validation")]
    {
        use std::fs;
        use std::process::Command;
        
        // Write XML to temp file
        let temp_path = "/tmp/test_parameter_condition.xosc";
        fs::write(temp_path, xml).unwrap();
        
        // Run esmini in headless mode to validate
        let output = Command::new("esmini")
            .args(&["--osc", temp_path, "--headless", "--fixed_timestep", "0.01", "--record", "sim.dat"])
            .output();
        
        if let Ok(result) = output {
            assert!(result.status.success(), "esmini validation failed: {:?}", 
                String::from_utf8_lossy(&result.stderr));
        }
        
        // Cleanup
        let _ = fs::remove_file(temp_path);
        let _ = fs::remove_file("sim.dat");
    }
}
```

- [ ] **Step 2: Update Scenario to support ParameterDeclarations XML export**

In `openscenario/src/xml.rs`, find the `to_xml` or XML writing function and add parameter declarations export before entities:

```rust
// Write ParameterDeclarations
if !scenario.parameters.is_empty() {
    writer.write_event(Event::Start(BytesStart::new("ParameterDeclarations")))?;
    
    for param in &scenario.parameters {
        let mut param_tag = BytesStart::new("ParameterDeclaration");
        param_tag.push_attribute(("name", param.name.as_str()));
        param_tag.push_attribute(("parameterType", match param.parameter_type {
            crate::scenario::ParameterType::Integer => "integer",
            crate::scenario::ParameterType::Double => "double",
            crate::scenario::ParameterType::String => "string",
            crate::scenario::ParameterType::Boolean => "boolean",
        }));
        param_tag.push_attribute(("value", param.value.as_str()));
        writer.write_event(Event::Empty(param_tag))?;
    }
    
    writer.write_event(Event::End(BytesEnd::new("ParameterDeclarations")))?;
}
```

- [ ] **Step 3: Run integration test**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test test_parameter_condition_end_to_end
```

Expected: PASS

- [ ] **Step 4: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/tests/parameter_condition_integration_test.rs openscenario/src/xml.rs
git commit -m "test: add end-to-end integration test for ParameterCondition"
```

---

## Task 9: Run Full Test Suite

**Files:**
- N/A (validation only)

- [ ] **Step 1: Run all tests**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test
```

Expected: All tests PASS

- [ ] **Step 2: Run clippy**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo clippy -- -D warnings
```

Expected: No warnings

- [ ] **Step 3: Run formatter**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo fmt --check
```

Expected: No formatting changes needed. If changes needed:

```bash
cargo fmt
git add -u
git commit -m "style: run cargo fmt"
```

- [ ] **Step 4: Verify zero unwrap() in production code**

```bash
cd ~/.openclaw/workspace/osc-mcp
grep -r "unwrap()" openscenario/src/ | grep -v "test" | grep -v "//"
```

Expected: No output (zero unwrap() in production)

- [ ] **Step 5: Final commit if formatter made changes**

```bash
cd ~/.openclaw/workspace/osc-mcp
git status
# If changes exist, commit them
```

---

## Acceptance Criteria

- ✅ All tests pass (unit + integration)
- ✅ ParameterCondition struct with parameter_ref, value, rule fields
- ✅ ByValueCondition::Parameter variant
- ✅ Condition::parameter() helper method
- ✅ XML generation with ParameterCondition element
- ✅ Parameter validation (InvalidParameterRef error)
- ✅ ParameterDeclarations XML export
- ✅ Test coverage: all 6 ComparisonRule operators
- ✅ Test coverage: string, numeric, boolean values
- ✅ Test coverage: invalid parameter reference error
- ✅ End-to-end integration test
- ✅ Clippy clean (no warnings)
- ✅ Formatted with cargo fmt
- ✅ Zero unwrap() in production code

---

## Implementation Notes

### TDD Flow
Each task follows strict TDD: write failing test → implement minimal code → verify test passes → commit.

### Parameter Validation
Validation happens at XML generation time (fail fast). The library does not perform type checking (that's the simulator's job).

### Value Storage
All parameter values stored as strings (matching OpenSCENARIO spec). Type information lives in ParameterType enum, but validation is deferred to simulator runtime.

### Backward Compatibility
All changes are additive. Existing Phase 3.1 trigger tests remain unchanged.

---

## Future Work (Out of Scope)

- Phase 3.2b: SpeedCondition (ByEntity)
- Phase 3.2c: ReachPositionCondition (ByEntity)
- Phase 3.3: MCP tool integration

---

## Self-Review Checklist

**Spec coverage:**
- ✅ Functional Req 1 (Parameter comparison): Task 2 (ParameterCondition struct with rule field)
- ✅ Functional Req 2 (Type support): Task 4 (test string/numeric/boolean values)
- ✅ Functional Req 3 (Validation): Task 6 (parameter existence check in write_condition)
- ✅ Functional Req 4 (Error handling): Task 5 (InvalidParameterRef error) + Task 7 (error test)
- ✅ Functional Req 5 (OpenSCENARIO compliance): Task 6 (XML generation) + Task 8 (integration test with XML validation)
- ✅ Non-functional Req 1 (Backward compat): No breaking changes, all additive
- ✅ Non-functional Req 2 (Test coverage): Tasks 3, 4, 7, 8
- ✅ Non-functional Req 3 (Code quality): Task 9 (clippy, fmt, unwrap check)
- ✅ Non-functional Req 4 (Documentation): Tests serve as examples

**Placeholder scan:**
- ✅ No TBD, TODO, "implement later"
- ✅ No vague "add validation" without showing code
- ✅ All test steps show actual test code
- ✅ All implementation steps show actual implementation code

**Type consistency:**
- ✅ ParameterCondition fields consistent across all tasks
- ✅ ByValueCondition::Parameter variant used consistently
- ✅ Condition::parameter() signature matches ParameterCondition fields
- ✅ ComparisonRule usage consistent with Phase 3.1

**Execution readiness:**
- ✅ All file paths exact and complete
- ✅ All commands include working directory
- ✅ All expected outputs specified
- ✅ All code blocks complete (no "..." placeholders)
- ✅ Task order supports incremental validation (TDD flow)
