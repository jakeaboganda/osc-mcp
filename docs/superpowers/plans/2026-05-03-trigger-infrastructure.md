# Trigger Infrastructure Implementation Plan (Phase 3.1)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace hardcoded StartTriggers with configurable Trigger and ConditionGroup structs, establishing the foundation for comprehensive trigger/condition system.

**Architecture:** Introduce `Trigger` and `ConditionGroup` types in storyboard.rs, add them as optional fields to `Act` and `Event`, refactor XML generation to emit configurable triggers, maintain backward compatibility (None = use hardcoded default).

**Tech Stack:** Rust, quick_xml, serde

---

## File Structure

**New types (in existing files):**
- `openscenario/src/storyboard.rs` - Add Trigger, ConditionGroup, Condition enums
- `openscenario/src/xml.rs` - Refactor write_act_start_trigger and write_event_start_trigger
- `openscenario/tests/trigger_tests.rs` - New test file for trigger infrastructure

**Modified structs:**
- `Act` - add `start_trigger: Option<Trigger>`
- `Event` - add `start_trigger: Option<Trigger>`

---

## Task 1: Define Core Trigger Types

**Files:**
- Modify: `openscenario/src/storyboard.rs`

- [ ] **Step 1: Write failing test for Trigger struct**

Create test file:

```rust
// openscenario/tests/trigger_tests.rs
use openscenario::*;

#[test]
fn test_trigger_with_simulation_time_condition() {
    let trigger = Trigger::new(
        ConditionGroup::new(vec![
            Condition::simulation_time(5.0, ComparisonRule::GreaterOrEqual)
        ])
    );
    
    assert_eq!(trigger.condition_groups.len(), 1);
    assert_eq!(trigger.condition_groups[0].conditions.len(), 1);
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test trigger_tests test_trigger_with_simulation_time_condition
```

Expected: FAIL - "Trigger", "ConditionGroup", "Condition" not found

- [ ] **Step 3: Add Trigger types to storyboard.rs**

Add after `StopTrigger` definition in `openscenario/src/storyboard.rs`:

```rust
/// A trigger defines when something should start
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    pub condition_groups: Vec<ConditionGroup>,
}

impl Trigger {
    pub fn new(condition_group: ConditionGroup) -> Self {
        Self {
            condition_groups: vec![condition_group],
        }
    }
    
    pub fn with_groups(condition_groups: Vec<ConditionGroup>) -> Self {
        Self { condition_groups }
    }
}

/// A group of conditions (AND relationship within group)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionGroup {
    pub conditions: Vec<Condition>,
}

impl ConditionGroup {
    pub fn new(conditions: Vec<Condition>) -> Self {
        Self { conditions }
    }
}

/// Individual condition within a group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub delay: f64,
    pub condition_edge: ConditionEdge,
    pub kind: ConditionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionEdge {
    None,
    Rising,
    Falling,
    RisingOrFalling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionKind {
    ByValue(ByValueCondition),
    // Future: ByEntity(ByEntityCondition)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonRule {
    GreaterOrEqual,
    GreaterThan,
    LessOrEqual,
    LessThan,
    EqualTo,
    NotEqualTo,
}

impl Condition {
    pub fn simulation_time(value: f64, rule: ComparisonRule) -> Self {
        Self {
            name: "SimulationTimeCondition".to_string(),
            delay: 0.0,
            condition_edge: ConditionEdge::None,
            kind: ConditionKind::ByValue(ByValueCondition::SimulationTime { value, rule }),
        }
    }
    
    pub fn storyboard_element_state(
        element_type: impl Into<String>,
        element_ref: impl Into<String>,
        state: impl Into<String>,
    ) -> Self {
        Self {
            name: "StoryboardElementStateCondition".to_string(),
            delay: 0.0,
            condition_edge: ConditionEdge::None,
            kind: ConditionKind::ByValue(ByValueCondition::StoryboardElementState {
                element_type: element_type.into(),
                element_ref: element_ref.into(),
                state: state.into(),
            }),
        }
    }
}
```

- [ ] **Step 4: Export new types from lib.rs**

Add to `openscenario/src/lib.rs` public exports:

```rust
pub use storyboard::{
    Trigger, ConditionGroup, Condition, ConditionEdge, ConditionKind,
    ByValueCondition, ComparisonRule,
    // ... existing exports
};
```

- [ ] **Step 5: Run test to verify it passes**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test trigger_tests test_trigger_with_simulation_time_condition
```

Expected: PASS

- [ ] **Step 6: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/src/storyboard.rs openscenario/src/lib.rs openscenario/tests/trigger_tests.rs
git commit -m "feat: add Trigger, ConditionGroup, and Condition types"
```

---

## Task 2: Add Trigger Fields to Act and Event

**Files:**
- Modify: `openscenario/src/storyboard.rs`
- Test: `openscenario/tests/trigger_tests.rs`

- [ ] **Step 1: Write failing test for Act with trigger**

Add to `openscenario/tests/trigger_tests.rs`:

```rust
#[test]
fn test_act_with_custom_start_trigger() {
    let mut act = Act::new("MyAct");
    
    let trigger = Trigger::new(
        ConditionGroup::new(vec![
            Condition::simulation_time(5.0, ComparisonRule::GreaterOrEqual)
        ])
    );
    
    act.set_start_trigger(trigger.clone());
    
    assert!(act.start_trigger.is_some());
    let act_trigger = act.start_trigger.unwrap();
    assert_eq!(act_trigger.condition_groups.len(), 1);
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test trigger_tests test_act_with_custom_start_trigger
```

Expected: FAIL - "start_trigger" field doesn't exist, "set_start_trigger" method doesn't exist

- [ ] **Step 3: Add start_trigger field to Act**

In `openscenario/src/storyboard.rs`, modify `Act`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act {
    pub name: String,
    pub maneuver_groups: HashMap<String, ManeuverGroup>,
    pub start_trigger: Option<Trigger>,
}

impl Act {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            maneuver_groups: HashMap::new(),
            start_trigger: None,
        }
    }
    
    pub fn set_start_trigger(&mut self, trigger: Trigger) {
        self.start_trigger = Some(trigger);
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test trigger_tests test_act_with_custom_start_trigger
```

Expected: PASS

- [ ] **Step 5: Write failing test for Event with trigger**

Add to `openscenario/tests/trigger_tests.rs`:

```rust
#[test]
fn test_event_with_custom_start_trigger() {
    let mut event = Event::new("MyEvent");
    
    let trigger = Trigger::new(
        ConditionGroup::new(vec![
            Condition::storyboard_element_state("act", "MyAct", "completeState")
        ])
    );
    
    event.set_start_trigger(trigger.clone());
    
    assert!(event.start_trigger.is_some());
}
```

- [ ] **Step 6: Run test to verify it fails**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test trigger_tests test_event_with_custom_start_trigger
```

Expected: FAIL - Event doesn't have "start_trigger" or "set_start_trigger"

- [ ] **Step 7: Add start_trigger field to Event**

In `openscenario/src/storyboard.rs`, modify `Event`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub actions: Vec<Action>,
    pub start_trigger: Option<Trigger>,
}

impl Event {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            actions: Vec::new(),
            start_trigger: None,
        }
    }
    
    pub fn set_start_trigger(&mut self, trigger: Trigger) {
        self.start_trigger = Some(trigger);
    }
}
```

- [ ] **Step 8: Run test to verify it passes**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test trigger_tests test_event_with_custom_start_trigger
```

Expected: PASS

- [ ] **Step 9: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/src/storyboard.rs openscenario/tests/trigger_tests.rs
git commit -m "feat: add start_trigger fields to Act and Event"
```

---

## Task 3: Refactor XML Generation for Act StartTrigger

**Files:**
- Modify: `openscenario/src/xml.rs`
- Test: `openscenario/tests/xml_tests.rs`

- [ ] **Step 1: Write failing test for Act with custom trigger XML**

Add to `openscenario/tests/xml_tests.rs`:

```rust
#[test]
fn test_act_custom_start_trigger_xml() {
    let mut scenario = Scenario::new("1.0");
    let mut story = Story::new("MyStory");
    let mut act = Act::new("MyAct");
    
    // Set custom trigger: start at t=10
    let trigger = Trigger::new(
        ConditionGroup::new(vec![
            Condition::simulation_time(10.0, ComparisonRule::GreaterOrEqual)
        ])
    );
    act.set_start_trigger(trigger);
    
    story.acts.insert("MyAct".to_string(), act);
    scenario.add_story(story);
    
    let xml = scenario.to_xml().expect("XML generation failed");
    
    // Should contain SimulationTimeCondition with value=10
    assert!(xml.contains("<SimulationTimeCondition"));
    assert!(xml.contains("value=\"10\""));
    assert!(xml.contains("rule=\"greaterOrEqual\""));
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test xml_tests test_act_custom_start_trigger_xml
```

Expected: FAIL - XML still contains hardcoded value="0"

- [ ] **Step 3: Extract write_trigger helper method**

Add to `openscenario/src/xml.rs` before `write_act`:

```rust
impl Scenario {
    // ... existing methods

    fn write_trigger<W: std::io::Write>(
        &self,
        writer: &mut Writer<W>,
        trigger: &crate::storyboard::Trigger,
    ) -> Result<()> {
        writer.write_event(XmlEvent::Start(BytesStart::new("StartTrigger")))?;
        
        for condition_group in &trigger.condition_groups {
            self.write_condition_group(writer, condition_group)?;
        }
        
        writer.write_event(XmlEvent::End(BytesEnd::new("StartTrigger")))?;
        Ok(())
    }
    
    fn write_condition_group<W: std::io::Write>(
        &self,
        writer: &mut Writer<W>,
        group: &crate::storyboard::ConditionGroup,
    ) -> Result<()> {
        writer.write_event(XmlEvent::Start(BytesStart::new("ConditionGroup")))?;
        
        for condition in &group.conditions {
            self.write_condition(writer, condition)?;
        }
        
        writer.write_event(XmlEvent::End(BytesEnd::new("ConditionGroup")))?;
        Ok(())
    }
    
    fn write_condition<W: std::io::Write>(
        &self,
        writer: &mut Writer<W>,
        condition: &crate::storyboard::Condition,
    ) -> Result<()> {
        use crate::storyboard::{ByValueCondition, ConditionEdge, ConditionKind, ComparisonRule};
        
        let mut cond = BytesStart::new("Condition");
        cond.push_attribute(("name", condition.name.as_str()));
        cond.push_attribute(("delay", condition.delay.to_string().as_str()));
        
        let edge_str = match condition.condition_edge {
            ConditionEdge::None => "none",
            ConditionEdge::Rising => "rising",
            ConditionEdge::Falling => "falling",
            ConditionEdge::RisingOrFalling => "risingOrFalling",
        };
        cond.push_attribute(("conditionEdge", edge_str));
        
        writer.write_event(XmlEvent::Start(cond))?;
        
        match &condition.kind {
            ConditionKind::ByValue(by_value) => {
                writer.write_event(XmlEvent::Start(BytesStart::new("ByValueCondition")))?;
                
                match by_value {
                    ByValueCondition::SimulationTime { value, rule } => {
                        let mut sim_time = BytesStart::new("SimulationTimeCondition");
                        sim_time.push_attribute(("value", value.to_string().as_str()));
                        
                        let rule_str = match rule {
                            ComparisonRule::GreaterOrEqual => "greaterOrEqual",
                            ComparisonRule::GreaterThan => "greaterThan",
                            ComparisonRule::LessOrEqual => "lessOrEqual",
                            ComparisonRule::LessThan => "lessThan",
                            ComparisonRule::EqualTo => "equalTo",
                            ComparisonRule::NotEqualTo => "notEqualTo",
                        };
                        sim_time.push_attribute(("rule", rule_str));
                        
                        writer.write_event(XmlEvent::Empty(sim_time))?;
                    }
                    ByValueCondition::StoryboardElementState {
                        element_type,
                        element_ref,
                        state,
                    } => {
                        let mut elem_state = BytesStart::new("StoryboardElementStateCondition");
                        elem_state.push_attribute(("storyboardElementType", element_type.as_str()));
                        elem_state.push_attribute(("storyboardElementRef", element_ref.as_str()));
                        elem_state.push_attribute(("state", state.as_str()));
                        writer.write_event(XmlEvent::Empty(elem_state))?;
                    }
                }
                
                writer.write_event(XmlEvent::End(BytesEnd::new("ByValueCondition")))?;
            }
        }
        
        writer.write_event(XmlEvent::End(BytesEnd::new("Condition")))?;
        Ok(())
    }
}
```

- [ ] **Step 4: Refactor write_act to use new trigger method**

In `openscenario/src/xml.rs`, find the `write_act` method and replace the hardcoded StartTrigger section (lines ~471-487) with:

```rust
// StartTrigger
if let Some(trigger) = &act.start_trigger {
    self.write_trigger(writer, trigger)?;
} else {
    // Default: start immediately at t=0
    writer.write_event(XmlEvent::Start(BytesStart::new("StartTrigger")))?;
    writer.write_event(XmlEvent::Start(BytesStart::new("ConditionGroup")))?;
    let mut cond = BytesStart::new("Condition");
    cond.push_attribute(("name", "ActStartCondition"));
    cond.push_attribute(("delay", "0"));
    cond.push_attribute(("conditionEdge", "none"));
    writer.write_event(XmlEvent::Start(cond))?;
    writer.write_event(XmlEvent::Start(BytesStart::new("ByValueCondition")))?;
    let mut sim_time = BytesStart::new("SimulationTimeCondition");
    sim_time.push_attribute(("value", "0"));
    sim_time.push_attribute(("rule", "greaterOrEqual"));
    writer.write_event(XmlEvent::Empty(sim_time))?;
    writer.write_event(XmlEvent::End(BytesEnd::new("ByValueCondition")))?;
    writer.write_event(XmlEvent::End(BytesEnd::new("Condition")))?;
    writer.write_event(XmlEvent::End(BytesEnd::new("ConditionGroup")))?;
    writer.write_event(XmlEvent::End(BytesEnd::new("StartTrigger")))?;
}
```

- [ ] **Step 5: Run test to verify it passes**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test xml_tests test_act_custom_start_trigger_xml
```

Expected: PASS

- [ ] **Step 6: Test backward compatibility (no trigger = default)**

Add test to `openscenario/tests/xml_tests.rs`:

```rust
#[test]
fn test_act_default_start_trigger_xml() {
    let mut scenario = Scenario::new("1.0");
    let mut story = Story::new("MyStory");
    let act = Act::new("MyAct"); // No trigger set
    
    story.acts.insert("MyAct".to_string(), act);
    scenario.add_story(story);
    
    let xml = scenario.to_xml().expect("XML generation failed");
    
    // Should contain default t=0 trigger
    assert!(xml.contains("<SimulationTimeCondition"));
    assert!(xml.contains("value=\"0\""));
}
```

- [ ] **Step 7: Run backward compatibility test**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test xml_tests test_act_default_start_trigger_xml
```

Expected: PASS

- [ ] **Step 8: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/src/xml.rs openscenario/tests/xml_tests.rs
git commit -m "feat: refactor Act StartTrigger to use configurable triggers"
```

---

## Task 4: Refactor XML Generation for Event StartTrigger

**Files:**
- Modify: `openscenario/src/xml.rs`
- Test: `openscenario/tests/xml_tests.rs`

- [ ] **Step 1: Write failing test for Event with custom trigger XML**

Add to `openscenario/tests/xml_tests.rs`:

```rust
#[test]
fn test_event_custom_start_trigger_xml() {
    let mut scenario = Scenario::new("1.0");
    let mut story = Story::new("MyStory");
    let mut act = Act::new("MyAct");
    let mut maneuver_group = ManeuverGroup::new("MyManeuverGroup");
    let mut maneuver = Maneuver::new("MyManeuver");
    let mut event = Event::new("MyEvent");
    
    // Set custom trigger: wait for act completion
    let trigger = Trigger::new(
        ConditionGroup::new(vec![
            Condition::storyboard_element_state("act", "MyAct", "completeState")
        ])
    );
    event.set_start_trigger(trigger);
    
    maneuver.events.push(event);
    maneuver_group.maneuvers.push(maneuver);
    act.maneuver_groups.insert("MyManeuverGroup".to_string(), maneuver_group);
    story.acts.insert("MyAct".to_string(), act);
    scenario.add_story(story);
    
    let xml = scenario.to_xml().expect("XML generation failed");
    
    // Should contain StoryboardElementStateCondition
    assert!(xml.contains("<StoryboardElementStateCondition"));
    assert!(xml.contains("storyboardElementRef=\"MyAct\""));
    assert!(xml.contains("state=\"completeState\""));
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test xml_tests test_event_custom_start_trigger_xml
```

Expected: FAIL - XML still contains hardcoded SimulationTimeCondition for events

- [ ] **Step 3: Refactor write_event to use trigger method**

In `openscenario/src/xml.rs`, find the `write_event` method and replace the hardcoded StartTrigger section (lines ~558-574) with:

```rust
// StartTrigger
if let Some(trigger) = &event.start_trigger {
    self.write_trigger(writer, trigger)?;
} else {
    // Default: start when act begins (t=0)
    writer.write_event(XmlEvent::Start(BytesStart::new("StartTrigger")))?;
    writer.write_event(XmlEvent::Start(BytesStart::new("ConditionGroup")))?;
    let mut cond = BytesStart::new("Condition");
    cond.push_attribute(("name", "EventStartCondition"));
    cond.push_attribute(("delay", "0"));
    cond.push_attribute(("conditionEdge", "none"));
    writer.write_event(XmlEvent::Start(cond))?;
    writer.write_event(XmlEvent::Start(BytesStart::new("ByValueCondition")))?;
    let mut sim_time = BytesStart::new("SimulationTimeCondition");
    sim_time.push_attribute(("value", "0"));
    sim_time.push_attribute(("rule", "greaterOrEqual"));
    writer.write_event(XmlEvent::Empty(sim_time))?;
    writer.write_event(XmlEvent::End(BytesEnd::new("ByValueCondition")))?;
    writer.write_event(XmlEvent::End(BytesEnd::new("Condition")))?;
    writer.write_event(XmlEvent::End(BytesEnd::new("ConditionGroup")))?;
    writer.write_event(XmlEvent::End(BytesEnd::new("StartTrigger")))?;
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test xml_tests test_event_custom_start_trigger_xml
```

Expected: PASS

- [ ] **Step 5: Test backward compatibility**

Add to `openscenario/tests/xml_tests.rs`:

```rust
#[test]
fn test_event_default_start_trigger_xml() {
    let mut scenario = Scenario::new("1.0");
    let mut story = Story::new("MyStory");
    let mut act = Act::new("MyAct");
    let mut maneuver_group = ManeuverGroup::new("MyManeuverGroup");
    let mut maneuver = Maneuver::new("MyManeuver");
    let event = Event::new("MyEvent"); // No trigger set
    
    maneuver.events.push(event);
    maneuver_group.maneuvers.push(maneuver);
    act.maneuver_groups.insert("MyManeuverGroup".to_string(), maneuver_group);
    story.acts.insert("MyAct".to_string(), act);
    scenario.add_story(story);
    
    let xml = scenario.to_xml().expect("XML generation failed");
    
    // Should contain default t=0 trigger
    assert!(xml.contains("EventStartCondition"));
    assert!(xml.contains("<SimulationTimeCondition"));
}
```

- [ ] **Step 6: Run backward compatibility test**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test xml_tests test_event_default_start_trigger_xml
```

Expected: PASS

- [ ] **Step 7: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/src/xml.rs openscenario/tests/xml_tests.rs
git commit -m "feat: refactor Event StartTrigger to use configurable triggers"
```

---

## Task 5: Integration Test with esmini

**Files:**
- Create: `openscenario/tests/trigger_integration_test.rs`

- [ ] **Step 1: Write integration test with custom triggers**

```rust
// openscenario/tests/trigger_integration_test.rs
use openscenario::*;
use std::process::Command;

#[test]
fn test_scenario_with_custom_triggers_in_esmini() {
    let mut scenario = Scenario::new("1.0");
    
    // Add ego vehicle
    let ego = Vehicle::new("Ego")
        .with_catalog("VehicleCatalog.xosc", "car_white");
    scenario.add_entity(Entity::Vehicle(ego));
    
    // Create story with delayed act (starts at t=2)
    let mut story = Story::new("TriggerTestStory");
    let mut act = Act::new("DelayedAct");
    
    // Act starts at t=2
    let act_trigger = Trigger::new(
        ConditionGroup::new(vec![
            Condition::simulation_time(2.0, ComparisonRule::GreaterOrEqual)
        ])
    );
    act.set_start_trigger(act_trigger);
    
    // Maneuver with event
    let mut maneuver_group = ManeuverGroup::new("EgoManeuverGroup");
    maneuver_group.actors.push("Ego".to_string());
    
    let mut maneuver = Maneuver::new("SpeedManeuver");
    let mut event = Event::new("SpeedEvent");
    
    // Event starts 1 second after act (total t=3)
    let event_trigger = Trigger::new(
        ConditionGroup::new(vec![
            Condition::simulation_time(1.0, ComparisonRule::GreaterOrEqual)
        ])
    );
    event.set_start_trigger(event_trigger);
    
    event.actions.push(Action::Speed(SpeedAction {
        target_speed: 20.0,
        transition_duration: 2.0,
        shape: TransitionShape::Linear,
    }));
    
    maneuver.events.push(event);
    maneuver_group.maneuvers.push(maneuver);
    act.maneuver_groups.insert("EgoManeuverGroup".to_string(), maneuver_group);
    story.acts.insert("DelayedAct".to_string(), act);
    scenario.add_story(story);
    
    // Set stop trigger at t=6
    scenario.storyboard_mut().set_stop_trigger(
        StopTrigger::simulation_time(6.0)
    );
    
    // Generate XML
    let xml = scenario.to_xml().expect("XML generation failed");
    
    // Verify trigger structure
    assert!(xml.contains("value=\"2\""), "Act should start at t=2");
    assert!(xml.contains("value=\"1\""), "Event should have delay of 1s");
    
    // Write to temp file
    let test_file = "/tmp/trigger_integration_test.xosc";
    std::fs::write(test_file, xml).expect("Failed to write test file");
    
    // Validate with esmini (if available)
    let esmini_check = Command::new("esmini")
        .args(["--osc", test_file, "--headless", "--fixed_timestep", "0.01", "--record", "sim.dat"])
        .output();
    
    if let Ok(output) = esmini_check {
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("esmini validation failed:\n{}", stderr);
        }
        println!("✓ esmini validation passed");
    } else {
        println!("⚠ esmini not available, skipping runtime validation");
    }
}
```

- [ ] **Step 2: Run integration test**

```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test --test trigger_integration_test test_scenario_with_custom_triggers_in_esmini
```

Expected: PASS (or skip if esmini not available)

- [ ] **Step 3: Manual esmini validation (if available)**

If esmini is installed:

```bash
esmini --osc /tmp/trigger_integration_test.xosc --window 60 60 800 400
```

Expected: 
- Simulation starts
- Vehicle appears at t=2 (delayed act)
- Speed change begins at t=3 (event trigger)
- Simulation stops at t=6

- [ ] **Step 4: Commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add openscenario/tests/trigger_integration_test.rs
git commit -m "test: add integration test for custom trigger system"
```

---

## Task 6: Run Full Test Suite

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
cargo fmt
```

Expected: Code formatted

- [ ] **Step 4: Final commit**

```bash
cd ~/.openclaw/workspace/osc-mcp
git add -A
git commit -m "chore: format code and fix clippy warnings"
```

---

## Summary

**What we built:**
- ✅ `Trigger`, `ConditionGroup`, `Condition` types with two condition variants (SimulationTime, StoryboardElementState)
- ✅ Optional `start_trigger` fields on `Act` and `Event`
- ✅ Configurable XML generation with backward compatibility
- ✅ 8 new tests covering trigger functionality
- ✅ Integration test with esmini validation

**Next steps:**
Phase 3.2 will add more condition types (Parameter, Speed, Position, Distance) using this infrastructure.

**Time estimate:** 2-3 hours focused work
