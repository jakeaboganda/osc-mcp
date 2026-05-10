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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Speed(SpeedAction),
    LaneChange(LaneChangeAction),
    LaneOffset(LaneOffsetAction),
    Position(PositionAction),
    Distance(DistanceAction),
    Acceleration(AccelerationAction),
    SpeedProfile(SpeedProfileAction),
    LongitudinalDistance(LongitudinalDistanceAction),
    FollowTrajectory(FollowTrajectoryAction),
    AssignRoute(AssignRouteAction),
    Synchronize(SynchronizeAction),
    // More actions to be added
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedAction {
    pub target_speed: f64,
    pub dynamics: TransitionDynamics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneChangeAction {
    pub target_lane_offset: f64,
    pub transition_duration: f64,
    pub shape: TransitionShape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneOffsetAction {
    pub target_offset: f64,
    pub continuous: bool,
    pub dynamics: Option<TransitionDynamics>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccelerationAction {
    pub value: f64,  // acceleration in m/s² (can be negative for deceleration)
    pub dynamics: TransitionDynamics,
}

/// Speed profile entry: (time_or_distance, speed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedProfileEntry {
    /// Time (seconds) or distance (meters) depending on following_mode
    pub position: f64,
    /// Target speed at this position (m/s)
    pub speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedProfileAction {
    /// Speed profile waypoints
    pub entries: Vec<SpeedProfileEntry>,
    /// If true, position is time-based; if false, distance-based
    pub following_mode: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionShape {
    Linear,
    Cubic,
    Sinusoidal,
    Step,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DynamicsShape {
    Linear,
    Cubic,
    Sinusoidal,
    Step,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DynamicsDimension {
    Time,
    Distance,
    Rate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionDynamics {
    pub shape: DynamicsShape,
    pub dimension: DynamicsDimension,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongitudinalDistanceAction {
    pub entity_ref: String,
    pub distance: f64,
    pub freespace: bool,
    pub continuous: bool,
    pub dynamics: Option<TransitionDynamics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowTrajectoryAction {
    pub trajectory: Trajectory,
    pub timing_mode: TimingMode,
    pub initial_distance_offset: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    pub name: String,
    pub closed: bool,
    pub vertices: Vec<Vertex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex {
    pub time: f64,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimingMode {
    Timing,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRouteAction {
    pub route: Route,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub name: String,
    pub closed: bool,
    pub waypoints: Vec<Waypoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    pub position: Position,
    pub route_strategy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronizeAction {
    pub entity_ref: String,
    pub master_entity_ref: String,
    pub target_position_master: TargetPositionMaster,
    pub target_position: TargetPosition,
    pub final_speed: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetPositionMaster {
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetPosition {
    World(Position),
    Relative(TargetPositionRelative),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetPositionRelative {
    pub entity_ref: String,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub delay: f64,
    pub condition_edge: ConditionEdge,
    pub kind: ConditionKind,
}

impl Condition {
    /// Create a simulation time condition
    pub fn simulation_time(seconds: f64, rule: Rule) -> Self {
        Self {
            name: format!("SimTime_{}", seconds),
            delay: 0.0,
            condition_edge: ConditionEdge::None,
            kind: ConditionKind::ByValue(ByValueCondition::SimulationTime {
                value: seconds,
                rule,
            }),
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

    /// Create a parameter condition
    pub fn parameter(
        parameter_ref: impl Into<String>,
        value: impl Into<String>,
        rule: Rule,
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
}

/// When a condition should be evaluated
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConditionEdge {
    None,
    Rising,
    Falling,
    RisingOrFalling,
}

/// The type of condition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionKind {
    ByValue(ByValueCondition),
    ByEntity(ByEntityCondition),
}

/// Value-based conditions (time, state, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ByValueCondition {
    SimulationTime {
        value: f64,
        rule: Rule,
    },
    StoryboardElementState {
        element_type: String,
        element_ref: String,
        state: String,
    },
    Parameter(ParameterCondition),
}

/// Parameter-based condition (checks runtime parameter value)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterCondition {
    pub parameter_ref: String,
    pub value: String,
    pub rule: Rule,
}

/// OpenSCENARIO 1.0 compliant comparison rule.
///
/// Defines the three comparison operators supported by the OpenSCENARIO 1.0 specification
/// for value-based conditions. Used in SimulationTimeCondition, ParameterCondition, and
/// entity-based conditions like SpeedCondition.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Rule {
    /// Greater than operator (>)
    GreaterThan,
    /// Less than operator (<)
    LessThan,
    /// Equal to operator (==)
    EqualTo,
}

/// Triggering entities rule (any vs all).
///
/// Determines whether at least one entity (`Any`) or all entities (`All`)
/// must satisfy the condition for it to be considered true.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggeringEntitiesRule {
    /// At least one entity must satisfy the condition
    Any,
    /// All entities must satisfy the condition
    All,
}

/// Collection of entities that trigger a condition.
///
/// Contains the list of entity references and the rule determining
/// whether any or all must satisfy the condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggeringEntities {
    /// Rule for combining entity conditions (any or all)
    pub rule: TriggeringEntitiesRule,
    /// List of entity names that will be checked
    pub entity_refs: Vec<String>,
}

/// Speed condition checks entity speed against a target value.
///
/// Compares the triggering entity's speed to a target speed value
/// using the specified comparison rule. Speed is measured in meters per second.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeedCondition {
    /// Target speed value in meters per second
    pub value: f64,
    /// Comparison rule (greaterThan, lessThan, equalTo)
    pub rule: Rule,
}

/// Reach position condition checks if entity reaches a target position.
///
/// Triggers when the triggering entity enters a tolerance sphere around
/// the target position. Position can be in any coordinate system (world, lane, road, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReachPositionCondition {
    /// Target position to reach
    pub position: Position,
    /// Tolerance radius in meters (distance from position center)
    pub tolerance: f64,
}

/// Time to collision condition checks predicted collision time.
///
/// Triggers when the estimated time until collision between the triggering
/// entity and a target entity meets the specified rule and value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeToCollisionCondition {
    /// Target entity to check collision with
    pub target_entity_ref: String,
    /// TTC value in seconds
    pub value: f64,
    /// Comparison rule (lessThan, greaterThan, etc.)
    pub rule: Rule,
}

/// Collision condition checks for actual collisions.
///
/// Triggers when the triggering entity collides with a target entity.
/// Unlike TTC, this detects actual contact between entities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollisionCondition {
    /// Target entity to detect collision with
    pub target_entity_ref: String,
}

/// Relative distance type for distance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelativeDistanceType {
    /// Distance along the road/trajectory (s-coordinate)
    Longitudinal,
    /// Distance perpendicular to road/trajectory (t-coordinate)
    Lateral,
    /// Straight-line distance in 3D space
    Euclidean,
}

/// Coordinate system for distance calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinateSystem {
    /// Entity's local coordinate system
    Entity,
    /// Lane coordinate system
    Lane,
    /// Road coordinate system
    Road,
    /// Trajectory coordinate system
    Trajectory,
}

/// Relative distance condition checks distance between entities.
///
/// Triggers when the distance between the triggering entity and a reference
/// entity meets the specified rule and value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelativeDistanceCondition {
    /// Reference entity to measure distance to
    pub entity_ref: String,
    /// Distance value in meters
    pub value: f64,
    /// Comparison rule (lessThan, greaterThan, equalTo)
    pub rule: Rule,
    /// Type of distance measurement
    pub distance_type: RelativeDistanceType,
    /// If true, distance between bounding boxes; if false, distance between reference points
    pub freespace: bool,
    /// Coordinate system for measurement (optional, defaults to entity)
    pub coordinate_system: Option<CoordinateSystem>,
}

/// Time headway condition checks time gap to a lead vehicle.
///
/// Triggers when the time gap between the triggering entity and a lead entity
/// meets the specified rule and value. Time headway is distance divided by follower's speed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeHeadwayCondition {
    /// Lead entity to measure time gap to
    pub entity_ref: String,
    /// Time headway value in seconds
    pub value: f64,
    /// Comparison rule (lessThan, greaterThan, equalTo)
    pub rule: Rule,
    /// If true, distance between bounding boxes; if false, distance between reference points
    pub freespace: bool,
}

/// Stand still condition checks if entity is stationary.
///
/// Triggers when the triggering entity remains stationary (speed ≈ 0)
/// for at least the specified duration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandStillCondition {
    /// Minimum duration the entity must be stationary (seconds)
    pub duration: f64,
}

/// Entity-based condition types.
///
/// Represents different conditions that can be checked against entity state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntityCondition {
    /// Speed-based condition
    Speed(SpeedCondition),
    /// Position-reaching condition
    ReachPosition(ReachPositionCondition),
    /// Time-to-collision condition
    TimeToCollision(TimeToCollisionCondition),
    /// Collision detection condition
    Collision(CollisionCondition),
    /// Relative distance condition
    RelativeDistance(RelativeDistanceCondition),
    /// Time headway condition
    TimeHeadway(TimeHeadwayCondition),
    /// Stand still condition
    StandStill(StandStillCondition),
    // Future: Acceleration, RelativeSpeed, Offroad, etc.
}

/// By-entity condition with triggering entities.
///
/// Wraps an entity condition with the list of entities to check and
/// the rule for combining their results (any or all).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByEntityCondition {
    /// Entities that trigger this condition and combination rule
    pub triggering_entities: TriggeringEntities,
    /// The specific entity condition to check
    pub entity_condition: EntityCondition,
}
