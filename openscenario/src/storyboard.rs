use crate::Position;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Container for a sequence of acts that defines a scenario's narrative.
///
/// A Story represents a complete scenario narrative composed of multiple Acts.
/// Each Act contains ManeuverGroups that define coordinated behaviors for entities.
/// Stories are identified by name and organize Acts in a HashMap for efficient lookup.
///
/// # Examples
/// ```
/// use openscenario::storyboard::Story;
///
/// # fn main() -> Result<(), openscenario::ScenarioError> {
/// let mut story = Story::new("HighwayMerge");
/// assert_eq!(story.name, "HighwayMerge");
/// assert!(story.acts.is_empty());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub name: String,
    pub acts: HashMap<String, Act>,
}

impl Story {
    /// Creates a new Story with the given name.
    ///
    /// Initializes an empty Story with no Acts. Acts can be added later
    /// by inserting into the `acts` HashMap.
    ///
    /// # Arguments
    /// * `name` - A unique identifier for this story
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::Story;
    ///
    /// # fn main() {
    /// let story = Story::new("UrbanScenario");
    /// assert_eq!(story.name, "UrbanScenario");
    /// # }
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            acts: HashMap::new(),
        }
    }
}

/// A sequence of maneuver groups that executes as a unit within a Story.
///
/// An Act represents a distinct phase of a scenario, containing multiple ManeuverGroups
/// that execute in parallel. Acts can have an optional start trigger that defines
/// when the Act should begin execution.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{Act, Trigger, ConditionGroup, Condition, Rule};
///
/// # fn main() {
/// let mut act = Act::new("LaneChangePhase");
/// let trigger = Trigger::new(ConditionGroup::new(vec![
///     Condition::simulation_time(5.0, Rule::GreaterThan)
/// ]));
/// act.set_start_trigger(trigger);
/// assert!(act.start_trigger.is_some());
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act {
    pub name: String,
    pub maneuver_groups: HashMap<String, ManeuverGroup>,
    pub start_trigger: Option<Trigger>,
}

impl Act {
    /// Creates a new Act with the given name.
    ///
    /// Initializes an empty Act with no ManeuverGroups or start trigger.
    /// ManeuverGroups can be added by inserting into the `maneuver_groups` HashMap.
    ///
    /// # Arguments
    /// * `name` - A unique identifier for this Act within its Story
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::Act;
    ///
    /// # fn main() {
    /// let act = Act::new("InitialPhase");
    /// assert_eq!(act.name, "InitialPhase");
    /// assert!(act.maneuver_groups.is_empty());
    /// # }
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            maneuver_groups: HashMap::new(),
            start_trigger: None,
        }
    }

    /// Sets the start trigger for this Act.
    ///
    /// Defines when this Act should begin execution. If no trigger is set,
    /// the Act starts immediately when its parent Story begins.
    ///
    /// # Arguments
    /// * `trigger` - The condition(s) that start this Act
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::{Act, Trigger, ConditionGroup, Condition, Rule};
    ///
    /// # fn main() {
    /// let mut act = Act::new("PhaseTwo");
    /// let trigger = Trigger::new(ConditionGroup::new(vec![
    ///     Condition::simulation_time(10.0, Rule::GreaterThan)
    /// ]));
    /// act.set_start_trigger(trigger);
    /// # }
    /// ```
    pub fn set_start_trigger(&mut self, trigger: Trigger) {
        self.start_trigger = Some(trigger);
    }
}

/// A collection of maneuvers executed by a set of actors.
///
/// A ManeuverGroup associates one or more actors (entities) with a sequence
/// of Maneuvers they should execute. Multiple ManeuverGroups within an Act
/// execute in parallel, allowing coordinated multi-entity behaviors.
///
/// # Examples
/// ```
/// use openscenario::storyboard::ManeuverGroup;
///
/// # fn main() {
/// let mut group = ManeuverGroup::new("VehicleManeuvers");
/// group.actors.push("Ego".to_string());
/// group.actors.push("Target".to_string());
/// assert_eq!(group.actors.len(), 2);
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManeuverGroup {
    pub name: String,
    pub actors: Vec<String>,
    pub maneuvers: Vec<Maneuver>,
}

impl ManeuverGroup {
    /// Creates a new ManeuverGroup with the given name.
    ///
    /// Initializes an empty ManeuverGroup with no actors or maneuvers.
    /// Actors and Maneuvers should be added after creation.
    ///
    /// # Arguments
    /// * `name` - A unique identifier for this ManeuverGroup within its Act
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::ManeuverGroup;
    ///
    /// # fn main() {
    /// let group = ManeuverGroup::new("LeadVehicleActions");
    /// assert_eq!(group.name, "LeadVehicleActions");
    /// assert!(group.actors.is_empty());
    /// # }
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            actors: Vec::new(),
            maneuvers: Vec::new(),
        }
    }
}

/// A sequence of events that defines a specific driving behavior.
///
/// A Maneuver contains one or more Events that execute sequentially.
/// Each Event represents a discrete action or set of actions that the
/// actor(s) should perform.
///
/// # Examples
/// ```
/// use openscenario::storyboard::Maneuver;
///
/// # fn main() {
/// let mut maneuver = Maneuver::new("LaneChange");
/// assert_eq!(maneuver.name, "LaneChange");
/// assert!(maneuver.events.is_empty());
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maneuver {
    pub name: String,
    pub events: Vec<Event>,
}

impl Maneuver {
    /// Creates a new Maneuver with the given name.
    ///
    /// Initializes an empty Maneuver with no Events. Events should be
    /// added to the `events` vector after creation.
    ///
    /// # Arguments
    /// * `name` - A unique identifier for this Maneuver within its ManeuverGroup
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::Maneuver;
    ///
    /// # fn main() {
    /// let maneuver = Maneuver::new("SpeedReduction");
    /// assert_eq!(maneuver.name, "SpeedReduction");
    /// # }
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            events: Vec::new(),
        }
    }
}

/// A triggered collection of actions that execute simultaneously.
///
/// An Event contains one or more Actions that start executing when the Event's
/// start trigger condition is met. All Actions within an Event begin execution
/// at the same time.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{Event, Trigger, ConditionGroup, Condition, Rule};
///
/// # fn main() {
/// let mut event = Event::new("BrakeEvent");
/// let trigger = Trigger::new(ConditionGroup::new(vec![
///     Condition::simulation_time(2.0, Rule::GreaterThan)
/// ]));
/// event.set_start_trigger(trigger);
/// assert!(event.start_trigger.is_some());
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub actions: Vec<Action>,
    pub start_trigger: Option<Trigger>,
}

impl Event {
    /// Creates a new Event with the given name.
    ///
    /// Initializes an empty Event with no Actions or start trigger.
    /// Actions should be added to the `actions` vector, and a trigger
    /// should be set before the Event can execute.
    ///
    /// # Arguments
    /// * `name` - A unique identifier for this Event within its Maneuver
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::Event;
    ///
    /// # fn main() {
    /// let event = Event::new("AccelerateEvent");
    /// assert_eq!(event.name, "AccelerateEvent");
    /// assert!(event.actions.is_empty());
    /// # }
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            actions: Vec::new(),
            start_trigger: None,
        }
    }

    /// Sets the start trigger for this Event.
    ///
    /// Defines when this Event's Actions should begin execution.
    /// Without a trigger, the Event will not execute.
    ///
    /// # Arguments
    /// * `trigger` - The condition(s) that start this Event
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::{Event, Trigger, ConditionGroup, Condition, Rule};
    ///
    /// # fn main() {
    /// let mut event = Event::new("StopEvent");
    /// let trigger = Trigger::new(ConditionGroup::new(vec![
    ///     Condition::simulation_time(15.0, Rule::GreaterThan)
    /// ]));
    /// event.set_start_trigger(trigger);
    /// # }
    /// ```
    pub fn set_start_trigger(&mut self, trigger: Trigger) {
        self.start_trigger = Some(trigger);
    }
}

/// A specific behavior or state change to be performed by an entity.
///
/// Actions represent the atomic operations that entities perform during a scenario,
/// such as changing speed, lanes, or position. Each Action variant contains the
/// parameters needed to execute that specific behavior.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{Action, SpeedAction, TransitionDynamics, DynamicsShape, DynamicsDimension};
///
/// # fn main() {
/// let speed_action = Action::Speed(SpeedAction {
///     target_speed: 25.0,
///     dynamics: TransitionDynamics {
///         shape: DynamicsShape::Linear,
///         dimension: DynamicsDimension::Time,
///         value: 3.0,
///     },
/// });
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    /// Change entity speed to a target value
    Speed(SpeedAction),
    /// Change to a different lane
    LaneChange(LaneChangeAction),
    /// Apply a lateral offset from the lane center
    LaneOffset(LaneOffsetAction),
    /// Teleport to a specific position
    Position(PositionAction),
    /// Maintain a specific distance to another entity
    Distance(DistanceAction),
    /// Apply a constant acceleration
    Acceleration(AccelerationAction),
    /// Follow a speed profile over time or distance
    SpeedProfile(SpeedProfileAction),
    /// Maintain a longitudinal distance to another entity
    LongitudinalDistance(LongitudinalDistanceAction),
    /// Follow a predefined trajectory
    FollowTrajectory(FollowTrajectoryAction),
    /// Assign a route for the entity to follow
    AssignRoute(AssignRouteAction),
    /// Synchronize motion with another entity
    Synchronize(SynchronizeAction),
    // More actions to be added
}

/// Action to change entity speed to a target value.
///
/// Specifies a target speed and the transition dynamics (how quickly and smoothly
/// the entity should reach the target speed).
///
/// # Examples
/// ```
/// use openscenario::storyboard::{SpeedAction, TransitionDynamics, DynamicsShape, DynamicsDimension};
///
/// # fn main() {
/// let action = SpeedAction {
///     target_speed: 30.0,  // 30 m/s
///     dynamics: TransitionDynamics {
///         shape: DynamicsShape::Linear,
///         dimension: DynamicsDimension::Time,
///         value: 5.0,  // 5 seconds
///     },
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedAction {
    pub target_speed: f64,
    pub dynamics: TransitionDynamics,
}

/// Action to change lanes with a target offset.
///
/// Specifies the target lane offset, transition duration, and the shape
/// of the transition curve.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{LaneChangeAction, TransitionShape};
///
/// # fn main() {
/// let action = LaneChangeAction {
///     target_lane_offset: -1.0,  // Change to left lane
///     transition_duration: 3.0,   // 3 seconds
///     shape: TransitionShape::Sinusoidal,
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneChangeAction {
    pub target_lane_offset: f64,
    pub transition_duration: f64,
    pub shape: TransitionShape,
}

/// Action to apply a lateral offset from the lane center.
///
/// Can be continuous (maintained) or one-time. Optional dynamics control
/// how the offset is applied.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{LaneOffsetAction, TransitionDynamics, DynamicsShape, DynamicsDimension};
///
/// # fn main() {
/// let action = LaneOffsetAction {
///     target_offset: 0.5,  // 0.5 meters to the right
///     continuous: true,     // Maintain offset
///     dynamics: Some(TransitionDynamics {
///         shape: DynamicsShape::Linear,
///         dimension: DynamicsDimension::Time,
///         value: 2.0,
///     }),
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneOffsetAction {
    pub target_offset: f64,
    pub continuous: bool,
    pub dynamics: Option<TransitionDynamics>,
}

/// Action to teleport an entity to a specific position.
///
/// Immediately moves the entity to the specified position without animation.
/// Position can be in any coordinate system (world, lane, road, relative).
///
/// # Examples
/// ```
/// use openscenario::{storyboard::PositionAction, Position};
///
/// # fn main() {
/// # let position = Position::World { x: 0.0, y: 0.0, z: 0.0, h: 0.0, p: 0.0, r: 0.0 };
/// let action = PositionAction {
///     position,
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionAction {
    pub position: Position,
}

/// Action to maintain a specific distance to another entity.
///
/// The entity will adjust its behavior to maintain the specified distance
/// to the reference entity.
///
/// # Examples
/// ```
/// use openscenario::storyboard::DistanceAction;
///
/// # fn main() {
/// let action = DistanceAction {
///     entity_ref: "LeadVehicle".to_string(),
///     distance: 20.0,    // 20 meters
///     freespace: true,   // Measure bounding box distance
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceAction {
    pub entity_ref: String,
    pub distance: f64,
    pub freespace: bool,
}

/// Action to apply a constant acceleration.
///
/// Applies the specified acceleration value (can be negative for deceleration)
/// according to the transition dynamics.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{AccelerationAction, TransitionDynamics, DynamicsShape, DynamicsDimension};
///
/// # fn main() {
/// let action = AccelerationAction {
///     value: -3.0,  // Decelerate at 3 m/s²
///     dynamics: TransitionDynamics {
///         shape: DynamicsShape::Step,
///         dimension: DynamicsDimension::Rate,
///         value: 1.0,
///     },
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccelerationAction {
    pub value: f64,  // acceleration in m/s² (can be negative for deceleration)
    pub dynamics: TransitionDynamics,
}

/// A waypoint in a speed profile defining speed at a specific position.
///
/// Represents a target speed at a specific time or distance point in the profile.
/// The position interpretation depends on the SpeedProfileAction's following_mode.
///
/// # Examples
/// ```
/// use openscenario::storyboard::SpeedProfileEntry;
///
/// # fn main() {
/// let entry = SpeedProfileEntry {
///     position: 10.0,  // 10 seconds or 10 meters
///     speed: 15.0,     // 15 m/s
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedProfileEntry {
    /// Time (seconds) or distance (meters) depending on following_mode
    pub position: f64,
    /// Target speed at this position (m/s)
    pub speed: f64,
}

/// Action to follow a speed profile over time or distance.
///
/// Defines a series of speed waypoints that the entity should follow.
/// The profile can be time-based or distance-based depending on following_mode.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{SpeedProfileAction, SpeedProfileEntry};
///
/// # fn main() {
/// let action = SpeedProfileAction {
///     entries: vec![
///         SpeedProfileEntry { position: 0.0, speed: 10.0 },
///         SpeedProfileEntry { position: 5.0, speed: 20.0 },
///         SpeedProfileEntry { position: 10.0, speed: 15.0 },
///     ],
///     following_mode: true,  // Time-based
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedProfileAction {
    /// Speed profile waypoints
    pub entries: Vec<SpeedProfileEntry>,
    /// If true, position is time-based; if false, distance-based
    pub following_mode: bool,
}

/// The shape of a transition curve for lane changes.
///
/// Defines how smoothly the entity transitions between states during a lane change.
/// Different shapes provide different acceleration profiles.
///
/// # Examples
/// ```
/// use openscenario::storyboard::TransitionShape;
///
/// # fn main() {
/// let smooth = TransitionShape::Sinusoidal;  // Smooth acceleration
/// let sharp = TransitionShape::Step;         // Instant change
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionShape {
    /// Constant rate transition
    Linear,
    /// Smooth cubic transition
    Cubic,
    /// Smooth sinusoidal transition
    Sinusoidal,
    /// Immediate transition
    Step,
}

/// The shape of a dynamics transition curve.
///
/// Defines the acceleration profile for actions that change entity state
/// (speed, acceleration, etc.). Identical to TransitionShape but used in
/// TransitionDynamics context.
///
/// # Examples
/// ```
/// use openscenario::storyboard::DynamicsShape;
///
/// # fn main() {
/// let shape = DynamicsShape::Linear;  // Constant acceleration
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DynamicsShape {
    /// Constant rate change
    Linear,
    /// Smooth cubic change
    Cubic,
    /// Smooth sinusoidal change
    Sinusoidal,
    /// Immediate change
    Step,
}

/// The dimension in which dynamics are specified.
///
/// Determines how the dynamics value is interpreted: as time, distance, or rate.
///
/// # Examples
/// ```
/// use openscenario::storyboard::DynamicsDimension;
///
/// # fn main() {
/// let time_based = DynamicsDimension::Time;     // Value in seconds
/// let dist_based = DynamicsDimension::Distance; // Value in meters
/// let rate_based = DynamicsDimension::Rate;     // Value in m/s² or similar
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DynamicsDimension {
    /// Value represents time duration (seconds)
    Time,
    /// Value represents distance (meters)
    Distance,
    /// Value represents rate of change (e.g., m/s²)
    Rate,
}

/// Defines how an entity transitions to a new state.
///
/// Specifies the shape, dimension, and value for state transitions like speed changes.
/// The value interpretation depends on the dimension:
/// - **Time**: Duration of the transition (typical: 2-10 seconds)
/// - **Distance**: Length over which the transition occurs (typical: 10-100 meters)
/// - **Rate**: Rate of change (typical: acceleration 2-5 m/s², speed change 5-20 km/h/s)
///
/// # Examples
/// ```
/// use openscenario::storyboard::{TransitionDynamics, DynamicsShape, DynamicsDimension};
///
/// # fn main() {
/// // Linear transition over 4 seconds
/// let dynamics = TransitionDynamics {
///     shape: DynamicsShape::Linear,
///     dimension: DynamicsDimension::Time,
///     value: 4.0,  // 4 seconds
/// };
/// 
/// // Transition over 50 meters
/// let distance_based = TransitionDynamics {
///     shape: DynamicsShape::Cubic,
///     dimension: DynamicsDimension::Distance,
///     value: 50.0,  // 50 meters
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionDynamics {
    pub shape: DynamicsShape,
    pub dimension: DynamicsDimension,
    pub value: f64,
}

/// Action to maintain a longitudinal distance to another entity.
///
/// Similar to DistanceAction but specifically for longitudinal (along-road) distance.
/// Can be continuous (maintained) with optional transition dynamics.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{LongitudinalDistanceAction, TransitionDynamics, DynamicsShape, DynamicsDimension};
///
/// # fn main() {
/// let action = LongitudinalDistanceAction {
///     entity_ref: "LeadVehicle".to_string(),
///     distance: 30.0,
///     freespace: true,
///     continuous: true,
///     dynamics: Some(TransitionDynamics {
///         shape: DynamicsShape::Linear,
///         dimension: DynamicsDimension::Time,
///         value: 2.0,
///     }),
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongitudinalDistanceAction {
    pub entity_ref: String,
    pub distance: f64,
    pub freespace: bool,
    pub continuous: bool,
    pub dynamics: Option<TransitionDynamics>,
}

/// Action to follow a predefined trajectory.
///
/// The entity follows a series of position waypoints with timestamps.
/// Timing mode controls whether timestamps are followed or ignored.
///
/// # Examples
/// ```
/// use openscenario::{storyboard::{FollowTrajectoryAction, Trajectory, Vertex, TimingMode}, Position};
///
/// # fn main() {
/// # let pos1 = Position::World { x: 0.0, y: 0.0, z: 0.0, h: 0.0, p: 0.0, r: 0.0 };
/// # let pos2 = Position::World { x: 10.0, y: 0.0, z: 0.0, h: 0.0, p: 0.0, r: 0.0 };
/// let trajectory = Trajectory {
///     name: "Path1".to_string(),
///     closed: false,
///     vertices: vec![
///         Vertex { time: 0.0, position: pos1 },
///         Vertex { time: 5.0, position: pos2 },
///     ],
/// };
/// let action = FollowTrajectoryAction {
///     trajectory,
///     timing_mode: TimingMode::Timing,
///     initial_distance_offset: None,
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowTrajectoryAction {
    pub trajectory: Trajectory,
    pub timing_mode: TimingMode,
    pub initial_distance_offset: Option<f64>,
}

/// A sequence of position waypoints with timestamps.
///
/// Defines a path through space that an entity can follow. Can be closed (forms a loop)
/// or open (has distinct start and end points).
///
/// # Examples
/// ```
/// use openscenario::{storyboard::Trajectory, Position};
///
/// # fn main() {
/// # let pos = Position::World { x: 0.0, y: 0.0, z: 0.0, h: 0.0, p: 0.0, r: 0.0 };
/// let trajectory = Trajectory {
///     name: "CircuitPath".to_string(),
///     closed: true,  // Forms a loop
///     vertices: vec![],
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    pub name: String,
    pub closed: bool,
    pub vertices: Vec<Vertex>,
}

/// A position waypoint with a timestamp in a trajectory.
///
/// Combines a position in space with a time, defining where the entity should
/// be at a specific moment when following a trajectory.
///
/// # Examples
/// ```
/// use openscenario::{storyboard::Vertex, Position};
///
/// # fn main() {
/// # let position = Position::World { x: 100.0, y: 50.0, z: 0.0, h: 0.0, p: 0.0, r: 0.0 };
/// let vertex = Vertex {
///     time: 10.0,  // 10 seconds into trajectory
///     position,
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex {
    pub time: f64,
    pub position: Position,
}

/// Timing mode for trajectory following.
///
/// Controls whether the trajectory's timestamps should be followed precisely
/// or if the entity can traverse the path at its own pace.
///
/// # Examples
/// ```
/// use openscenario::storyboard::TimingMode;
///
/// # fn main() {
/// let strict = TimingMode::Timing;  // Follow timestamps
/// let free = TimingMode::None;      // Ignore timestamps
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimingMode {
    /// Follow trajectory timestamps precisely
    Timing,
    /// Ignore timestamps, traverse at natural pace
    None,
}

/// Action to assign a route for the entity to follow.
///
/// Defines a route consisting of waypoints that the entity should navigate.
/// The route can be closed (returns to start) or open.
///
/// # Examples
/// ```
/// use openscenario::{storyboard::{AssignRouteAction, Route, Waypoint}, Position};
///
/// # fn main() {
/// # let pos = Position::World { x: 0.0, y: 0.0, z: 0.0, h: 0.0, p: 0.0, r: 0.0 };
/// let route = Route {
///     name: "Highway1".to_string(),
///     closed: false,
///     waypoints: vec![
///         Waypoint { position: pos.clone(), route_strategy: None },
///     ],
/// };
/// let action = AssignRouteAction { route };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRouteAction {
    pub route: Route,
}

/// A navigable path defined by waypoints.
///
/// Defines a series of positions the entity should navigate through.
/// Can be closed (circular route) or open (point-to-point).
///
/// # Examples
/// ```
/// use openscenario::storyboard::Route;
///
/// # fn main() {
/// let route = Route {
///     name: "DeliveryRoute".to_string(),
///     closed: false,
///     waypoints: vec![],
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub name: String,
    pub closed: bool,
    pub waypoints: Vec<Waypoint>,
}

/// A navigation waypoint in a route.
///
/// Defines a target position and optional routing strategy for how
/// the entity should navigate to reach this waypoint.
///
/// # Examples
/// ```
/// use openscenario::{storyboard::Waypoint, Position};
///
/// # fn main() {
/// # let position = Position::World { x: 200.0, y: 100.0, z: 0.0, h: 0.0, p: 0.0, r: 0.0 };
/// let waypoint = Waypoint {
///     position,
///     route_strategy: Some("shortest".to_string()),
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    pub position: Position,
    pub route_strategy: Option<String>,
}

/// Action to synchronize motion with another entity.
///
/// Coordinates the entity's motion so it reaches a target position at the same time
/// as a master entity reaches its target position. Used for complex coordination scenarios.
///
/// # Examples
/// ```
/// use openscenario::{storyboard::{SynchronizeAction, TargetPositionMaster, TargetPosition}, Position};
///
/// # fn main() {
/// # let pos1 = Position::World { x: 0.0, y: 0.0, z: 0.0, h: 0.0, p: 0.0, r: 0.0 };
/// # let pos2 = Position::World { x: 100.0, y: 0.0, z: 0.0, h: 0.0, p: 0.0, r: 0.0 };
/// let action = SynchronizeAction {
///     entity_ref: "Follower".to_string(),
///     master_entity_ref: "Leader".to_string(),
///     target_position_master: TargetPositionMaster { position: pos1 },
///     target_position: TargetPosition::World(pos2),
///     final_speed: Some(20.0),
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronizeAction {
    pub entity_ref: String,
    pub master_entity_ref: String,
    pub target_position_master: TargetPositionMaster,
    pub target_position: TargetPosition,
    pub final_speed: Option<f64>,
}

/// Target position for the master entity in a synchronize action.
///
/// Defines where the master entity should be when synchronization completes.
///
/// # Examples
/// ```
/// use openscenario::{storyboard::TargetPositionMaster, Position};
///
/// # fn main() {
/// # let position = Position::World { x: 150.0, y: 0.0, z: 0.0, h: 0.0, p: 0.0, r: 0.0 };
/// let target = TargetPositionMaster { position };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetPositionMaster {
    pub position: Position,
}

/// Target position specification for synchronize actions.
///
/// Can be either an absolute world position or a position relative to another entity.
///
/// # Examples
/// ```
/// use openscenario::{storyboard::{TargetPosition, TargetPositionRelative}, Position};
///
/// # fn main() {
/// # let pos = Position::World { x: 50.0, y: 0.0, z: 0.0, h: 0.0, p: 0.0, r: 0.0 };
/// let absolute = TargetPosition::World(pos);
/// let relative = TargetPosition::Relative(TargetPositionRelative {
///     entity_ref: "RefEntity".to_string(),
///     dx: 10.0,
///     dy: 0.0,
///     dz: 0.0,
/// });
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetPosition {
    /// Absolute position in world coordinates
    World(Position),
    /// Position relative to another entity
    Relative(TargetPositionRelative),
}

/// A position defined relative to another entity.
///
/// Specifies an offset from a reference entity's position in x, y, and z dimensions.
///
/// # Examples
/// ```
/// use openscenario::storyboard::TargetPositionRelative;
///
/// # fn main() {
/// let relative = TargetPositionRelative {
///     entity_ref: "LeadVehicle".to_string(),
///     dx: -10.0,  // 10 meters behind
///     dy: 1.5,    // 1.5 meters to the right
///     dz: 0.0,
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetPositionRelative {
    pub entity_ref: String,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

/// Container for all Stories and the simulation stop trigger.
///
/// The Storyboard is the top-level container for scenario behavior, holding
/// all Stories and defining when the simulation should stop.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{Storyboard, StopTrigger};
///
/// # fn main() {
/// let mut storyboard = Storyboard::new();
/// let stop = StopTrigger::simulation_time(60.0);
/// storyboard.set_stop_trigger(stop);
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storyboard {
    pub stories: HashMap<String, Story>,
    pub stop_trigger: Option<StopTrigger>,
}

impl Storyboard {
    /// Creates a new empty Storyboard.
    ///
    /// Initializes a Storyboard with no Stories or stop trigger.
    /// Stories and stop trigger should be added after creation.
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::Storyboard;
    ///
    /// # fn main() {
    /// let storyboard = Storyboard::new();
    /// assert!(storyboard.stories.is_empty());
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            stories: HashMap::new(),
            stop_trigger: None,
        }
    }

    /// Sets the stop trigger for the simulation.
    ///
    /// Defines when the scenario execution should stop. Without a stop trigger,
    /// the simulation may run indefinitely.
    ///
    /// # Arguments
    /// * `trigger` - The condition that stops the simulation
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::{Storyboard, StopTrigger};
    ///
    /// # fn main() {
    /// let mut storyboard = Storyboard::new();
    /// storyboard.set_stop_trigger(StopTrigger::simulation_time(30.0));
    /// # }
    /// ```
    pub fn set_stop_trigger(&mut self, trigger: StopTrigger) {
        self.stop_trigger = Some(trigger);
    }
}

impl Default for Storyboard {
    fn default() -> Self {
        Self::new()
    }
}

/// Condition that stops the simulation.
///
/// Defines when the scenario execution should terminate. Can be based on
/// simulation time or the state of a storyboard element.
///
/// # Examples
/// ```
/// use openscenario::storyboard::StopTrigger;
///
/// # fn main() {
/// let time_stop = StopTrigger::simulation_time(45.0);
/// let state_stop = StopTrigger::storyboard_element_state(
///     "Act",
///     "FinalAct",
///     "completeState",
///     0.0
/// );
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopTrigger {
    pub condition: StopCondition,
}

/// Stop condition variants.
///
/// Defines the different ways a simulation can be stopped, either by reaching
/// a specific simulation time or when a storyboard element reaches a state.
///
/// # Examples
/// ```
/// use openscenario::storyboard::StopCondition;
///
/// # fn main() {
/// let time_cond = StopCondition::SimulationTime { seconds: 60.0 };
/// let state_cond = StopCondition::StoryboardElementState {
///     element_type: "Event".to_string(),
///     element_ref: "BrakeEvent".to_string(),
///     state: "endTransition".to_string(),
///     delay: 2.0,
/// };
/// # }
/// ```
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
    /// Creates a stop trigger based on simulation time.
    ///
    /// The simulation will stop after the specified number of seconds.
    ///
    /// # Arguments
    /// * `seconds` - Duration after which to stop (in seconds)
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::StopTrigger;
    ///
    /// # fn main() {
    /// let trigger = StopTrigger::simulation_time(120.0);
    /// # }
    /// ```
    pub fn simulation_time(seconds: f64) -> Self {
        Self {
            condition: StopCondition::SimulationTime { seconds },
        }
    }

    /// Creates a stop trigger based on storyboard element state.
    ///
    /// The simulation will stop when the specified element reaches the given state,
    /// optionally after a delay period.
    ///
    /// # Arguments
    /// * `element_type` - Type of element (e.g., "Act", "Event", "Maneuver")
    /// * `element_ref` - Name of the specific element
    /// * `state` - Target state name
    /// * `delay` - Additional delay in seconds after state is reached
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::StopTrigger;
    ///
    /// # fn main() {
    /// let trigger = StopTrigger::storyboard_element_state(
    ///     "Event",
    ///     "CollisionEvent",
    ///     "completeState",
    ///     1.0
    /// );
    /// # }
    /// ```
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
    /// Creates a new trigger with a single condition group.
    ///
    /// Convenience constructor for the common case of a single condition group.
    /// For multiple groups (OR logic), use `with_groups`.
    ///
    /// # Arguments
    /// * `condition_group` - The condition group to trigger on
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::{Trigger, ConditionGroup, Condition, Rule};
    ///
    /// # fn main() {
    /// let group = ConditionGroup::new(vec![
    ///     Condition::simulation_time(5.0, Rule::GreaterThan)
    /// ]);
    /// let trigger = Trigger::new(group);
    /// # }
    /// ```
    pub fn new(condition_group: ConditionGroup) -> Self {
        Self {
            condition_groups: vec![condition_group],
        }
    }

    /// Creates a trigger with multiple condition groups.
    ///
    /// Multiple groups are combined with OR logic: the trigger fires when
    /// any condition group is satisfied.
    ///
    /// # Arguments
    /// * `condition_groups` - The condition groups (OR-combined)
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::{Trigger, ConditionGroup, Condition, Rule};
    ///
    /// # fn main() {
    /// let groups = vec![
    ///     ConditionGroup::new(vec![
    ///         Condition::simulation_time(5.0, Rule::GreaterThan)
    ///     ]),
    ///     ConditionGroup::new(vec![
    ///         Condition::simulation_time(10.0, Rule::GreaterThan)
    ///     ]),
    /// ];
    /// let trigger = Trigger::with_groups(groups);
    /// # }
    /// ```
    pub fn with_groups(condition_groups: Vec<ConditionGroup>) -> Self {
        Self { condition_groups }
    }
}

/// A group of conditions combined with AND logic.
///
/// Multiple groups within a Trigger are combined with OR logic: the trigger fires
/// when any group is satisfied. Within each group, all conditions must be true.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{ConditionGroup, Condition, Rule};
///
/// # fn main() {
/// let group = ConditionGroup::new(vec![
///     Condition::simulation_time(5.0, Rule::GreaterThan),
///     Condition::parameter("weather", "rainy", Rule::EqualTo),
/// ]);
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionGroup {
    pub conditions: Vec<Condition>,
}

impl ConditionGroup {
    /// Creates a new condition group with the given conditions.
    ///
    /// Conditions within a group are combined with AND logic: all conditions
    /// must be satisfied for the group to trigger.
    ///
    /// # Arguments
    /// * `conditions` - The conditions to AND together
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::{ConditionGroup, Condition, Rule};
    ///
    /// # fn main() {
    /// let group = ConditionGroup::new(vec![
    ///     Condition::simulation_time(5.0, Rule::GreaterThan),
    ///     Condition::parameter("weather", "rainy", Rule::EqualTo),
    /// ]);
    /// # }
    /// ```
    pub fn new(conditions: Vec<Condition>) -> Self {
        Self { conditions }
    }
}

/// A testable condition that determines when an action or trigger fires.
///
/// Conditions can be based on simulation values (time, element state, parameters)
/// or entity states (speed, position, collisions). Each condition has a name,
/// optional delay, edge detection mode, and the specific condition type.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{Condition, Rule};
///
/// # fn main() {
/// let time_cond = Condition::simulation_time(5.0, Rule::GreaterThan);
/// let param_cond = Condition::parameter("scenario_type", "urban", Rule::EqualTo);
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub delay: f64,
    pub condition_edge: ConditionEdge,
    pub kind: ConditionKind,
}

impl Condition {
    /// Creates a simulation time condition.
    ///
    /// Triggers when simulation time meets the specified rule and value.
    ///
    /// # Arguments
    /// * `seconds` - The time value to compare against (in seconds)
    /// * `rule` - The comparison rule (GreaterThan, LessThan, or EqualTo)
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::{Condition, Rule};
    ///
    /// # fn main() {
    /// let cond = Condition::simulation_time(10.0, Rule::GreaterThan);
    /// assert_eq!(cond.name, "SimTime_10");
    /// # }
    /// ```
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

    /// Creates a storyboard element state condition.
    ///
    /// Triggers when a storyboard element (Act, Event, etc.) reaches a specified state.
    ///
    /// # Arguments
    /// * `element_type` - Type of element (e.g., "Act", "Event")
    /// * `element_ref` - Name of the element
    /// * `state` - Target state name
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::Condition;
    ///
    /// # fn main() {
    /// let cond = Condition::storyboard_element_state(
    ///     "Event",
    ///     "BrakeEvent",
    ///     "completeState"
    /// );
    /// # }
    /// ```
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

    /// Creates a parameter condition.
    ///
    /// Triggers when a runtime parameter meets the specified rule and value.
    ///
    /// # Arguments
    /// * `parameter_ref` - Name of the parameter to check
    /// * `value` - Value to compare against
    /// * `rule` - The comparison rule
    ///
    /// # Examples
    /// ```
    /// use openscenario::storyboard::{Condition, Rule};
    ///
    /// # fn main() {
    /// let cond = Condition::parameter("traffic_density", "high", Rule::EqualTo);
    /// # }
    /// ```
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

/// Edge detection mode for condition evaluation.
///
/// Determines when a condition should trigger based on state transitions.
/// None means the condition triggers whenever true; Rising/Falling/RisingOrFalling
/// trigger only on state changes.
///
/// # Practical Usage
/// - **Rising**: Best for "start" events (e.g., "when speed exceeds 50 km/h")
/// - **Falling**: Best for "end" events (e.g., "when speed drops below 30 km/h")
/// - **RisingOrFalling**: Detect any change (e.g., "when distance crosses threshold")
/// - **None**: Continuous condition (e.g., "while distance < 10m, maintain speed")
///
/// # Examples
/// ```
/// use openscenario::storyboard::ConditionEdge;
///
/// # fn main() {
/// // Trigger once when speed first exceeds limit
/// let rising = ConditionEdge::Rising;             // false→true: start event
/// 
/// // Trigger once when speed drops back below limit  
/// let falling = ConditionEdge::Falling;           // true→false: end event
/// 
/// // Trigger on every threshold crossing (both directions)
/// let both = ConditionEdge::RisingOrFalling;      // any change
/// 
/// // Trigger continuously while condition is true
/// let continuous = ConditionEdge::None;           // no edge detection
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConditionEdge {
    /// No edge detection, triggers whenever condition is true
    None,
    /// Triggers on false-to-true transition
    Rising,
    /// Triggers on true-to-false transition
    Falling,
    /// Triggers on any state change
    RisingOrFalling,
}

/// Classification of condition types.
///
/// Conditions are either ByValue (based on simulation state like time or parameters)
/// or ByEntity (based on entity state like speed or position).
///
/// # Examples
/// ```
/// use openscenario::storyboard::{ConditionKind, ByValueCondition, Rule};
///
/// # fn main() {
/// let value_cond = ConditionKind::ByValue(ByValueCondition::SimulationTime {
///     value: 10.0,
///     rule: Rule::GreaterThan,
/// });
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionKind {
    /// Condition based on simulation values
    ByValue(ByValueCondition),
    /// Condition based on entity states
    ByEntity(ByEntityCondition),
}

/// Conditions based on simulation or storyboard values.
///
/// These conditions check global simulation state rather than individual entity states.
/// Includes time-based, element state, and parameter conditions.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{ByValueCondition, Rule};
///
/// # fn main() {
/// let time_cond = ByValueCondition::SimulationTime {
///     value: 15.0,
///     rule: Rule::GreaterThan,
/// };
/// let state_cond = ByValueCondition::StoryboardElementState {
///     element_type: "Act".to_string(),
///     element_ref: "Act1".to_string(),
///     state: "running".to_string(),
/// };
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ByValueCondition {
    /// Condition based on simulation elapsed time
    SimulationTime {
        value: f64,
        rule: Rule,
    },
    /// Condition based on storyboard element reaching a state
    StoryboardElementState {
        element_type: String,
        element_ref: String,
        state: String,
    },
    /// Condition based on runtime parameter value
    Parameter(ParameterCondition),
}

/// Condition that checks a runtime parameter value.
///
/// Compares a named parameter against a string value using the specified rule.
/// Parameters are typically set when the scenario is initialized.
///
/// # Examples
/// ```
/// use openscenario::storyboard::{ParameterCondition, Rule};
///
/// # fn main() {
/// let param = ParameterCondition {
///     parameter_ref: "weather".to_string(),
///     value: "sunny".to_string(),
///     rule: Rule::EqualTo,
/// };
/// # }
/// ```
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
