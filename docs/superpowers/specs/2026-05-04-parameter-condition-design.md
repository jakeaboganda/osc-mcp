# ParameterCondition Design (Phase 3.2a)

**Date:** 2026-05-04  
**Author:** Robbie (AI Assistant)  
**Status:** Approved  
**Related:** Phase 3.1 (Trigger Infrastructure)

---

## Overview

Add ParameterCondition support to the OpenSCENARIO trigger system, enabling triggers that fire based on runtime parameter values. This is the first of three conditions in Phase 3.2 (Parameter → Speed → ReachPosition).

**Scope:** ParameterCondition only (ByValue variant). SpeedCondition and ReachPositionCondition are Phase 3.2b and 3.2c respectively.

**OpenSCENARIO Compliance:** Full spec implementation supporting string, numeric, and boolean parameter comparisons with all six ComparisonRule operators.

---

## Requirements

### Functional

1. **Parameter comparison:** Check runtime parameter value against reference value using comparison rule
2. **Type support:** Handle string, numeric, and boolean parameter types
3. **Validation:** Verify parameter exists in scenario's ParameterDeclarations before XML generation
4. **Error handling:** Clear error message when parameter reference is invalid
5. **OpenSCENARIO compliance:** Generate valid XML matching OpenSCENARIO 1.0 spec

### Non-Functional

1. **Backward compatibility:** No breaking changes to existing trigger infrastructure
2. **Test coverage:** Unit tests, XML generation tests, integration test
3. **Code quality:** Clippy clean, formatted, zero unwrap() in production code
4. **Documentation:** XML generation examples in tests

---

## Architecture

### Component Integration

```
Condition
  └─ ConditionKind::ByValue(ByValueCondition)
       ├─ SimulationTime (existing, Phase 3.1)
       ├─ StoryboardElementState (existing, Phase 3.1)
       └─ Parameter(ParameterCondition) ← NEW
```

**Key decisions:**
- Lives in `storyboard.rs` alongside existing ByValueCondition variants
- Reuses existing `ComparisonRule` enum (6 operators already defined)
- XML generation in `xml.rs` `write_condition()` function
- Parameter validation before XML generation (fail fast)

### Type Definitions

**New struct:**

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterCondition {
    pub parameter_ref: String,  // Name of parameter to check
    pub value: String,           // Reference value (stored as string)
    pub rule: ComparisonRule,    // <, <=, ==, !=, >=, >
}
```

**Extend ByValueCondition enum:**

```rust
pub enum ByValueCondition {
    SimulationTime { value: f64, rule: ComparisonRule },
    StoryboardElementState { element_ref: String, state: ElementState },
    Parameter(ParameterCondition),  // NEW
}
```

**Design rationale:**
- `value` stored as `String`: OpenSCENARIO parameters are declared separately with types. The condition references parameter by name, not by typed value. Type validation is the simulator's responsibility (esmini, CARLA, etc.), not the library's.
- Reuse `ComparisonRule`: Already supports all 6 operators (lessThan, lessOrEqual, equalTo, notEqualTo, greaterOrEqual, greaterThan)
- `PartialEq` derive: Enables equality testing in unit tests

---

## XML Generation

### Strategy

Extend `write_condition()` in `xml.rs` to handle the new Parameter variant:

```rust
match &condition.kind {
    ConditionKind::ByValue(by_value) => {
        writer.write_event(Event::Start(BytesStart::new("ByValueCondition")))?;
        
        match by_value {
            ByValueCondition::SimulationTime { ... } => { /* existing */ }
            ByValueCondition::StoryboardElementState { ... } => { /* existing */ }
            ByValueCondition::Parameter(param_cond) => {
                // Validate parameter exists before generating XML
                if !scenario.parameters.iter().any(|p| p.name == param_cond.parameter_ref) {
                    return Err(ScenarioError::InvalidParameterRef(
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
        }
        
        writer.write_event(Event::End(BytesEnd::new("ByValueCondition")))?;
    }
}
```

### XML Output Example

**Input scenario:**
```rust
let param = ParameterDeclaration {
    name: "MaxSpeed".to_string(),
    parameter_type: ParameterType::Double,
    value: "60.0".to_string(),
};

let condition = Condition {
    name: "SpeedLimitCheck".to_string(),
    delay: 0.0,
    edge: ConditionEdge::Rising,
    kind: ConditionKind::ByValue(
        ByValueCondition::Parameter(ParameterCondition {
            parameter_ref: "MaxSpeed".to_string(),
            value: "50.0".to_string(),
            rule: ComparisonRule::GreaterThan,
        })
    ),
};
```

**Generated XML:**
```xml
<Condition name="SpeedLimitCheck" delay="0" conditionEdge="rising">
  <ByValueCondition>
    <ParameterCondition parameterRef="MaxSpeed" value="50.0" rule="greaterThan"/>
  </ByValueCondition>
</Condition>
```

### Parameter Validation

**Validation logic:**
- Before writing ParameterCondition XML, search `scenario.parameters` for matching name
- If not found, return `ScenarioError::InvalidParameterRef(parameter_name)`
- Error is descriptive and includes the parameter name for debugging

**Error variant (add to `error.rs`):**
```rust
#[error("Invalid parameter reference: {0}")]
InvalidParameterRef(String),
```

---

## Testing Strategy

### Test Organization

1. **Unit tests** (`trigger_tests.rs`):
   - ParameterCondition struct construction
   - All 6 ComparisonRule variants
   - Clone and PartialEq behavior

2. **XML generation tests** (`trigger_tests.rs`):
   - Valid parameter reference → correct XML attributes
   - Invalid parameter reference → InvalidParameterRef error
   - All ComparisonRule operators render correctly
   - String, numeric, and boolean parameter values

3. **Integration test** (`parameter_condition_integration_test.rs`):
   - Build complete scenario with ParameterDeclarations
   - Add Act/Event with ParameterCondition trigger
   - Export XML and validate structure
   - (Optional) esmini validation if available

### Test Coverage Goals

**Comparison rules (6 tests):**
- lessThan
- lessOrEqual
- equalTo
- notEqualTo
- greaterOrEqual
- greaterThan

**Parameter types (3 tests):**
- String values (e.g., "stopped")
- Numeric values (e.g., "50.0")
- Boolean values (e.g., "true")

**Error cases (1 test):**
- Non-existent parameter reference

**Integration (1 test):**
- End-to-end scenario → XML → validation

### Acceptance Criteria

- ✅ All tests pass
- ✅ Clippy clean (no warnings with `-D warnings`)
- ✅ Formatted with `cargo fmt`
- ✅ Manual XML inspection confirms OpenSCENARIO compliance
- ✅ Zero `unwrap()` or `panic!()` in production code

---

## Implementation Notes

### TDD Approach

1. Write failing unit tests for ParameterCondition struct
2. Implement struct to make tests pass
3. Write failing tests for XML generation
4. Implement XML generation to make tests pass
5. Write failing integration test
6. Implement parameter validation to make test pass

### Files Modified

- `openscenario/src/storyboard.rs` - Add ParameterCondition struct, extend ByValueCondition enum
- `openscenario/src/xml.rs` - Extend write_condition() with Parameter variant handling
- `openscenario/src/error.rs` - Add InvalidParameterRef error variant
- `openscenario/tests/trigger_tests.rs` - Add unit and XML generation tests
- `openscenario/tests/parameter_condition_integration_test.rs` - New integration test file

### Dependencies

- No new external dependencies
- Reuses existing `ComparisonRule` enum
- Reuses existing `ParameterDeclaration` type (from `scenario.rs`)

---

## Future Work (Out of Scope)

- Phase 3.2b: SpeedCondition (ByEntity)
- Phase 3.2c: ReachPositionCondition (ByEntity)
- Phase 3.3: MCP tool integration for parameter-based triggers

---

## References

- OpenSCENARIO 1.0 Specification: ParameterCondition element
- Phase 3.1 implementation: Trigger infrastructure foundation
- Existing code: `storyboard.rs`, `xml.rs`, `trigger_tests.rs`
