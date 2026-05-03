use crate::Position;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub name: String,
    pub acts: HashMap<String, Act>,
}

impl Story {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            acts: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act {
    pub name: String,
    pub maneuver_groups: HashMap<String, ManeuverGroup>,
}

impl Act {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            maneuver_groups: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManeuverGroup {
    pub name: String,
    pub actors: Vec<String>,
    pub maneuvers: Vec<Maneuver>,
}

impl ManeuverGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            actors: Vec::new(),
            maneuvers: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maneuver {
    pub name: String,
    pub events: Vec<Event>,
}

impl Maneuver {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            events: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Speed(SpeedAction),
    LaneChange(LaneChangeAction),
    Position(PositionAction),
    Distance(DistanceAction),
    // More actions to be added
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedAction {
    pub target_speed: f64,
    pub transition_duration: f64,
    pub shape: TransitionShape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneChangeAction {
    pub target_lane_offset: f64,
    pub transition_duration: f64,
    pub shape: TransitionShape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionAction {
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceAction {
    pub entity_ref: String,
    pub distance: f64,
    pub freespace: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionShape {
    Linear,
    Cubic,
    Sinusoidal,
    Step,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storyboard {
    pub stories: HashMap<String, Story>,
    pub stop_trigger: Option<StopTrigger>,
}

impl Storyboard {
    pub fn new() -> Self {
        Self {
            stories: HashMap::new(),
            stop_trigger: None,
        }
    }

    pub fn set_stop_trigger(&mut self, trigger: StopTrigger) {
        self.stop_trigger = Some(trigger);
    }
}

impl Default for Storyboard {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopTrigger {
    pub condition: StopCondition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StopCondition {
    /// Stop after a simulation time duration
    SimulationTime { seconds: f64 },
    /// Stop when a story element reaches a specific state
    StoryboardElementState {
        element_type: String,
        element_ref: String,
        state: String,
        delay: f64,
    },
}

impl StopTrigger {
    pub fn simulation_time(seconds: f64) -> Self {
        Self {
            condition: StopCondition::SimulationTime { seconds },
        }
    }

    pub fn storyboard_element_state(
        element_type: impl Into<String>,
        element_ref: impl Into<String>,
        state: impl Into<String>,
        delay: f64,
    ) -> Self {
        Self {
            condition: StopCondition::StoryboardElementState {
                element_type: element_type.into(),
                element_ref: element_ref.into(),
                state: state.into(),
                delay,
            },
        }
    }
}

/// A trigger defines when an action should start or stop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    pub condition_groups: Vec<ConditionGroup>,
}

impl Trigger {
    /// Create a new trigger with a single condition group
    pub fn new(condition_group: ConditionGroup) -> Self {
        Self {
            condition_groups: vec![condition_group],
        }
    }

    /// Create a trigger with multiple condition groups
    pub fn with_groups(condition_groups: Vec<ConditionGroup>) -> Self {
        Self { condition_groups }
    }
}

/// A group of conditions (logical AND within group, OR between groups)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionGroup {
    pub conditions: Vec<Condition>,
}

impl ConditionGroup {
    /// Create a new condition group with the given conditions
    pub fn new(conditions: Vec<Condition>) -> Self {
        Self { conditions }
    }
}

/// A single condition that can trigger an action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub delay: f64,
    pub condition_edge: ConditionEdge,
    pub kind: ConditionKind,
}

impl Condition {
    /// Create a simulation time condition
    pub fn simulation_time(seconds: f64, rule: ComparisonRule) -> Self {
        Self {
            name: format!("SimTime_{}", seconds),
            delay: 0.0,
            condition_edge: ConditionEdge::None,
            kind: ConditionKind::ByValue(ByValueCondition::SimulationTime { value: seconds, rule }),
        }
    }

    /// Create a storyboard element state condition
    pub fn storyboard_element_state(
        element_type: impl Into<String>,
        element_ref: impl Into<String>,
        state: impl Into<String>,
    ) -> Self {
        let element_type = element_type.into();
        let element_ref = element_ref.into();
        let state_val = state.into();
        Self {
            name: format!("{}_{}_{}", element_type, element_ref, state_val),
            delay: 0.0,
            condition_edge: ConditionEdge::None,
            kind: ConditionKind::ByValue(ByValueCondition::StoryboardElementState {
                element_type,
                element_ref,
                state: state_val,
            }),
        }
    }
}

/// When a condition should be evaluated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionEdge {
    None,
    Rising,
    Falling,
    RisingOrFalling,
}

/// The type of condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionKind {
    ByValue(ByValueCondition),
}

/// Value-based conditions (time, state, etc.)
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

/// Comparison rules for value-based conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonRule {
    GreaterOrEqual,
    GreaterThan,
    LessOrEqual,
    LessThan,
    EqualTo,
    NotEqualTo,
}
