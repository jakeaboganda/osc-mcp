use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
}

impl Storyboard {
    pub fn new() -> Self {
        Self {
            stories: HashMap::new(),
        }
    }
}

impl Default for Storyboard {
    fn default() -> Self {
        Self::new()
    }
}
