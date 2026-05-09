use crate::entities::{
    Entity, MiscObject, MiscObjectParams, Pedestrian, PedestrianParams, Vehicle, VehicleParams,
};
use crate::storyboard::{
    Act, Action, DistanceAction, Event, LaneChangeAction, Maneuver, ManeuverGroup, PositionAction,
    SpeedAction, Story, Storyboard, TransitionShape,
};
use crate::Position;
use crate::{OpenScenarioVersion, Result, ScenarioError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterDeclaration {
    pub name: String,
    pub parameter_type: ParameterType,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParameterType {
    Integer,
    Double,
    String,
    Boolean,
}

pub struct Scenario {
    pub(crate) version: OpenScenarioVersion,
    pub(crate) entities: HashMap<String, Entity>,
    pub(crate) initial_positions: HashMap<String, Position>,
    pub(crate) parameters: Vec<ParameterDeclaration>,
    pub(crate) storyboard: Storyboard,
}

impl Scenario {
    pub fn new(version: OpenScenarioVersion) -> Self {
        Self {
            version,
            entities: HashMap::new(),
            initial_positions: HashMap::new(),
            parameters: Vec::new(),
            storyboard: Storyboard::new(),
        }
    }

    pub fn version(&self) -> OpenScenarioVersion {
        self.version
    }

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

    pub fn add_vehicle(&mut self, name: impl Into<String>, params: VehicleParams) -> Result<()> {
        let name = name.into();

        // Validate name is not empty (trim for validation)
        if name.trim().is_empty() {
            return Err(ScenarioError::InvalidValue {
                field: "entity name".to_string(),
                reason: "name cannot be empty or whitespace-only".to_string(),
            });
        }

        // Normalize: store trimmed name for consistency
        let name = name.trim().to_string();

        // Check for entity conflict
        if self.entities.contains_key(&name) {
            return Err(ScenarioError::EntityConflict {
                name,
                existing_location: None,
            });
        }

        let vehicle = Vehicle {
            name: name.clone(),
            params,
        };

        self.entities.insert(name, Entity::Vehicle(vehicle));
        Ok(())
    }

    pub fn add_pedestrian(
        &mut self,
        name: impl Into<String>,
        params: PedestrianParams,
    ) -> Result<()> {
        let name = name.into();

        // Validate name is not empty
        if name.trim().is_empty() {
            return Err(ScenarioError::InvalidValue {
                field: "entity name".to_string(),
                reason: "name cannot be empty or whitespace-only".to_string(),
            });
        }

        // Normalize: store trimmed name
        let name = name.trim().to_string();

        if self.entities.contains_key(&name) {
            return Err(ScenarioError::EntityConflict {
                name,
                existing_location: None,
            });
        }

        let pedestrian = Pedestrian {
            name: name.clone(),
            params,
        };

        self.entities.insert(name, Entity::Pedestrian(pedestrian));
        Ok(())
    }

    pub fn add_misc_object(
        &mut self,
        name: impl Into<String>,
        params: MiscObjectParams,
    ) -> Result<()> {
        let name = name.into();

        // Validate name is not empty
        if name.trim().is_empty() {
            return Err(ScenarioError::InvalidValue {
                field: "entity name".to_string(),
                reason: "name cannot be empty or whitespace-only".to_string(),
            });
        }

        // Normalize: store trimmed name
        let name = name.trim().to_string();

        if self.entities.contains_key(&name) {
            return Err(ScenarioError::EntityConflict {
                name,
                existing_location: None,
            });
        }

        let misc_object = MiscObject {
            name: name.clone(),
            params,
        };

        self.entities.insert(name, Entity::MiscObject(misc_object));
        Ok(())
    }

    pub fn get_entity(&self, name: &str) -> Option<&Entity> {
        self.entities.get(name)
    }

    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    pub fn get_initial_position(&self, entity: &str) -> Option<&Position> {
        self.initial_positions.get(entity)
    }

    pub fn initial_positions(&self) -> impl Iterator<Item = (&String, &Position)> {
        self.initial_positions.iter()
    }

    pub fn set_initial_position(
        &mut self,
        entity: impl Into<String>,
        position: Position,
    ) -> Result<()> {
        let entity = entity.into();

        // Validate entity name is not empty
        if entity.trim().is_empty() {
            return Err(ScenarioError::InvalidValue {
                field: "entity reference".to_string(),
                reason: "entity name cannot be empty or whitespace-only".to_string(),
            });
        }

        // Normalize for lookup
        let entity = entity.trim().to_string();

        // Check entity exists
        if !self.entities.contains_key(&entity) {
            return Err(ScenarioError::EntityNotFound {
                entity,
                context: "set_initial_position".to_string(),
            });
        }

        // Validate referenced entity in relative positions
        if let Some(ref_entity) = position.referenced_entity() {
            if !self.entities.contains_key(ref_entity) {
                return Err(ScenarioError::EntityNotFound {
                    entity: ref_entity.to_string(),
                    context: format!("Position for entity '{}'", entity),
                });
            }
        }

        self.initial_positions.insert(entity, position);
        Ok(())
    }

    pub fn add_story(&mut self, name: impl Into<String>) -> Result<()> {
        let name = name.into();

        // Validate name is not empty
        if name.trim().is_empty() {
            return Err(ScenarioError::InvalidValue {
                field: "story name".to_string(),
                reason: "name cannot be empty or whitespace-only".to_string(),
            });
        }

        // Normalize: store trimmed name
        let name = name.trim().to_string();

        if self.storyboard.stories.contains_key(&name) {
            return Err(ScenarioError::StoryNotFound {
                name: name.clone(),
                available: self.storyboard.stories.keys().cloned().collect(),
            });
        }

        self.storyboard
            .stories
            .insert(name.clone(), Story::new(name));
        Ok(())
    }

    pub fn add_act(&mut self, story: impl Into<String>, name: impl Into<String>) -> Result<()> {
        let story_name = story.into();
        let act_name = name.into();

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;
        story.acts.insert(act_name.clone(), Act::new(act_name));
        Ok(())
    }

    pub fn set_act_start_trigger(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        trigger: crate::storyboard::Trigger,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();

        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        act.set_start_trigger(trigger);
        Ok(())
    }

    pub fn set_event_start_trigger(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        maneuver_group: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        trigger: crate::storyboard::Trigger,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = maneuver_group.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();

        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg =
            act.maneuver_groups
                .get_mut(&mg_name)
                .ok_or_else(|| ScenarioError::EntityNotFound {
                    entity: mg_name.clone(),
                    context: format!("ManeuverGroup in act '{}'", act_name),
                })?;

        let maneuver = mg
            .maneuvers
            .iter_mut()
            .find(|m| m.name == maneuver_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: maneuver_name.clone(),
                context: format!("Maneuver in maneuver group '{}'", mg_name),
            })?;

        let event = maneuver
            .events
            .iter_mut()
            .find(|e| e.name == event_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: event_name.clone(),
                context: format!("Event in maneuver '{}'", maneuver_name),
            })?;

        event.set_start_trigger(trigger);
        Ok(())
    }

    pub fn add_maneuver_group(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = name.into();

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        // Check for duplicate maneuver group name
        if act.maneuver_groups.contains_key(&mg_name) {
            return Err(ScenarioError::NameConflict {
                item_type: "ManeuverGroup".to_string(),
                name: mg_name,
                context: format!("act '{}'", act_name),
            });
        }

        act.maneuver_groups
            .insert(mg_name.clone(), ManeuverGroup::new(mg_name));
        Ok(())
    }

    pub fn add_actor(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        entity: impl Into<String>,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let entity_name = entity.into();

        // Validate entity name is not empty
        if entity_name.trim().is_empty() {
            return Err(ScenarioError::InvalidValue {
                field: "entity reference".to_string(),
                reason: "entity name cannot be empty or whitespace-only".to_string(),
            });
        }

        // Normalize entity name for lookup
        let entity_name = entity_name.trim().to_string();

        // Validate entity exists
        if !self.entities.contains_key(&entity_name) {
            return Err(ScenarioError::EntityNotFound {
                entity: entity_name,
                context: "add_actor".to_string(),
            });
        }

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg =
            act.maneuver_groups
                .get_mut(&mg_name)
                .ok_or_else(|| ScenarioError::EntityNotFound {
                    entity: mg_name.clone(),
                    context: format!("ManeuverGroup in act '{}'", act_name),
                })?;

        // Only add if not already present
        if !mg.actors.contains(&entity_name) {
            mg.actors.push(entity_name);
        }
        Ok(())
    }

    pub fn add_maneuver(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = name.into();

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg =
            act.maneuver_groups
                .get_mut(&mg_name)
                .ok_or_else(|| ScenarioError::EntityNotFound {
                    entity: mg_name.clone(),
                    context: format!("ManeuverGroup in act '{}'", act_name),
                })?;

        mg.maneuvers.push(Maneuver::new(maneuver_name));
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_speed_action(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        target_speed: f64,
        dynamics: crate::storyboard::TransitionDynamics,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();

        // Validate numeric parameters
        if target_speed < 0.0 {
            return Err(ScenarioError::InvalidValue {
                field: "target_speed".to_string(),
                reason: format!("speed cannot be negative (got {})", target_speed),
            });
        }
        if dynamics.value <= 0.0 {
            return Err(ScenarioError::InvalidValue {
                field: "dynamics.value".to_string(),
                reason: format!("dynamics value must be positive (got {})", dynamics.value),
            });
        }

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg =
            act.maneuver_groups
                .get_mut(&mg_name)
                .ok_or_else(|| ScenarioError::EntityNotFound {
                    entity: mg_name.clone(),
                    context: format!("ManeuverGroup in act '{}'", act_name),
                })?;

        let maneuver = mg
            .maneuvers
            .iter_mut()
            .find(|m| m.name == maneuver_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: maneuver_name.clone(),
                context: format!("Maneuver in ManeuverGroup '{}'", mg_name),
            })?;

        let action = Action::Speed(SpeedAction {
            target_speed,
            dynamics,
        });

        // Find or create event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.actions.push(action);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![action],
                start_trigger: None,
            });
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_lane_change_action(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        target_lane_offset: f64,
        duration: f64,
        shape: TransitionShape,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();

        // Validate duration
        if duration <= 0.0 {
            return Err(ScenarioError::InvalidValue {
                field: "duration".to_string(),
                reason: format!("duration must be positive (got {})", duration),
            });
        }

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg =
            act.maneuver_groups
                .get_mut(&mg_name)
                .ok_or_else(|| ScenarioError::EntityNotFound {
                    entity: mg_name.clone(),
                    context: format!("ManeuverGroup in act '{}'", act_name),
                })?;

        let maneuver = mg
            .maneuvers
            .iter_mut()
            .find(|m| m.name == maneuver_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: maneuver_name.clone(),
                context: format!("Maneuver in ManeuverGroup '{}'", mg_name),
            })?;

        let action = Action::LaneChange(LaneChangeAction {
            target_lane_offset,
            transition_duration: duration,
            shape,
        });

        // Find or create event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.actions.push(action);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![action],
                start_trigger: None,
            });
        }

        Ok(())
    }

    pub fn add_lane_offset_action(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        target_offset: f64,
        continuous: bool,
        dynamics: Option<crate::storyboard::TransitionDynamics>,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();

        // Validate dynamics if present
        if let Some(ref dyn_data) = dynamics {
            if dyn_data.value <= 0.0 {
                return Err(ScenarioError::InvalidValue {
                    field: "dynamics.value".to_string(),
                    reason: format!("dynamics value must be positive (got {})", dyn_data.value),
                });
            }
        }

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg =
            act.maneuver_groups
                .get_mut(&mg_name)
                .ok_or_else(|| ScenarioError::EntityNotFound {
                    entity: mg_name.clone(),
                    context: format!("ManeuverGroup in act '{}'", act_name),
                })?;

        let maneuver = mg
            .maneuvers
            .iter_mut()
            .find(|m| m.name == maneuver_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: maneuver_name.clone(),
                context: format!("Maneuver in ManeuverGroup '{}'", mg_name),
            })?;

        let action = Action::LaneOffset(crate::storyboard::LaneOffsetAction {
            target_offset,
            continuous,
            dynamics,
        });

        // Find or create event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.actions.push(action);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![action],
                start_trigger: None,
            });
        }

        Ok(())
    }

    pub fn add_follow_trajectory_action(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        trajectory: crate::storyboard::Trajectory,
        timing_mode: crate::storyboard::TimingMode,
        initial_distance_offset: Option<f64>,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();

        // Validate trajectory has at least 2 vertices
        if trajectory.vertices.len() < 2 {
            return Err(ScenarioError::InvalidValue {
                field: "trajectory.vertices".to_string(),
                reason: format!(
                    "trajectory must have at least 2 vertices (got {})",
                    trajectory.vertices.len()
                ),
            });
        }

        // Validate initial_distance_offset if present
        if let Some(offset) = initial_distance_offset {
            if offset < 0.0 {
                return Err(ScenarioError::InvalidValue {
                    field: "initial_distance_offset".to_string(),
                    reason: format!("offset must be non-negative (got {})", offset),
                });
            }
        }

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg =
            act.maneuver_groups
                .get_mut(&mg_name)
                .ok_or_else(|| ScenarioError::EntityNotFound {
                    entity: mg_name.clone(),
                    context: format!("ManeuverGroup in act '{}'", act_name),
                })?;

        let maneuver = mg
            .maneuvers
            .iter_mut()
            .find(|m| m.name == maneuver_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: maneuver_name.clone(),
                context: format!("Maneuver in ManeuverGroup '{}'", mg_name),
            })?;

        let action = Action::FollowTrajectory(crate::storyboard::FollowTrajectoryAction {
            trajectory,
            timing_mode,
            initial_distance_offset,
        });

        // Find or create event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.actions.push(action);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![action],
                start_trigger: None,
            });
        }

        Ok(())
    }

    pub fn add_assign_route_action(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        route: crate::storyboard::Route,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();

        // Validate route has at least 2 waypoints
        if route.waypoints.len() < 2 {
            return Err(ScenarioError::InvalidValue {
                field: "route.waypoints".to_string(),
                reason: format!(
                    "route must have at least 2 waypoints (got {})",
                    route.waypoints.len()
                ),
            });
        }

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg =
            act.maneuver_groups
                .get_mut(&mg_name)
                .ok_or_else(|| ScenarioError::EntityNotFound {
                    entity: mg_name.clone(),
                    context: format!("ManeuverGroup in act '{}'", act_name),
                })?;

        let maneuver = mg
            .maneuvers
            .iter_mut()
            .find(|m| m.name == maneuver_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: maneuver_name.clone(),
                context: format!("Maneuver in ManeuverGroup '{}'", mg_name),
            })?;

        let action = Action::AssignRoute(crate::storyboard::AssignRouteAction { route });

        // Find or create event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.actions.push(action);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![action],
                start_trigger: None,
            });
        }

        Ok(())
    }

    pub fn add_synchronize_action(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        master_entity_ref: impl Into<String>,
        target_position_master: crate::storyboard::TargetPositionMaster,
        target_position: crate::storyboard::TargetPosition,
        final_speed: Option<f64>,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();
        let entity_ref_str = entity_ref.into();
        let master_entity_ref_str = master_entity_ref.into();

        // Validate entity references exist
        if !self.entities.iter().any(|(name, _)| *name == entity_ref_str) {
            return Err(ScenarioError::EntityNotFound {
                entity: entity_ref_str,
                context: "SynchronizeAction entity_ref".to_string(),
            });
        }

        if !self.entities.iter().any(|(name, _)| *name == master_entity_ref_str) {
            return Err(ScenarioError::EntityNotFound {
                entity: master_entity_ref_str.clone(),
                context: "SynchronizeAction master_entity_ref".to_string(),
            });
        }

        // Validate final_speed if present
        if let Some(speed) = final_speed {
            if speed < 0.0 {
                return Err(ScenarioError::InvalidValue {
                    field: "final_speed".to_string(),
                    reason: format!("final_speed cannot be negative (got {})", speed),
                });
            }
        }

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg =
            act.maneuver_groups
                .get_mut(&mg_name)
                .ok_or_else(|| ScenarioError::EntityNotFound {
                    entity: mg_name.clone(),
                    context: format!("ManeuverGroup in act '{}'", act_name),
                })?;

        let maneuver = mg
            .maneuvers
            .iter_mut()
            .find(|m| m.name == maneuver_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: maneuver_name.clone(),
                context: format!("Maneuver in ManeuverGroup '{}'", mg_name),
            })?;

        let action = Action::Synchronize(crate::storyboard::SynchronizeAction {
            entity_ref: entity_ref_str,
            master_entity_ref: master_entity_ref_str,
            target_position_master,
            target_position,
            final_speed,
        });

        // Find or create event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.actions.push(action);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![action],
                start_trigger: None,
            });
        }

        Ok(())
    }

    pub fn add_position_action(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        position: Position,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg =
            act.maneuver_groups
                .get_mut(&mg_name)
                .ok_or_else(|| ScenarioError::EntityNotFound {
                    entity: mg_name.clone(),
                    context: format!("ManeuverGroup in act '{}'", act_name),
                })?;

        let maneuver = mg
            .maneuvers
            .iter_mut()
            .find(|m| m.name == maneuver_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: maneuver_name.clone(),
                context: format!("Maneuver in ManeuverGroup '{}'", mg_name),
            })?;

        let action = Action::Position(PositionAction { position });

        // Find or create event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.actions.push(action);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![action],
                start_trigger: None,
            });
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_distance_action(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        distance: f64,
        freespace: bool,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();
        let entity_ref = entity_ref.into();

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg =
            act.maneuver_groups
                .get_mut(&mg_name)
                .ok_or_else(|| ScenarioError::EntityNotFound {
                    entity: mg_name.clone(),
                    context: format!("ManeuverGroup in act '{}'", act_name),
                })?;

        let maneuver = mg
            .maneuvers
            .iter_mut()
            .find(|m| m.name == maneuver_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: maneuver_name.clone(),
                context: format!("Maneuver in ManeuverGroup '{}'", mg_name),
            })?;

        let action = Action::Distance(DistanceAction {
            entity_ref,
            distance,
            freespace,
        });

        // Find or create event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.actions.push(action);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![action],
                start_trigger: None,
            });
        }

        Ok(())
    }

    /// Add a longitudinal distance action to maintain distance from another entity
    pub fn add_longitudinal_distance_action(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        distance: f64,
        freespace: bool,
        continuous: bool,
        dynamics: Option<crate::storyboard::TransitionDynamics>,
    ) -> Result<()> {
        use crate::storyboard::LongitudinalDistanceAction;

        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();
        let entity_ref = entity_ref.into();

        // Validate that the referenced entity exists
        if !self.entities.contains_key(&entity_ref) {
            return Err(ScenarioError::EntityNotFound {
                entity: entity_ref.clone(),
                context: "Referenced entity for LongitudinalDistanceAction".to_string(),
            });
        }

        // Collect keys FIRST to avoid borrow checker issues
        let available: Vec<String> = self.storyboard.stories.keys().cloned().collect();

        // Then use ok_or_else with the pre-collected keys
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg =
            act.maneuver_groups
                .get_mut(&mg_name)
                .ok_or_else(|| ScenarioError::EntityNotFound {
                    entity: mg_name.clone(),
                    context: format!("ManeuverGroup in act '{}'", act_name),
                })?;

        let maneuver = mg
            .maneuvers
            .iter_mut()
            .find(|m| m.name == maneuver_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: maneuver_name.clone(),
                context: format!("Maneuver in ManeuverGroup '{}'", mg_name),
            })?;

        let action = Action::LongitudinalDistance(LongitudinalDistanceAction {
            entity_ref,
            distance,
            freespace,
            continuous,
            dynamics,
        });

        // Find or create event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.actions.push(action);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![action],
                start_trigger: None,
            });
        }

        Ok(())
    }

    /// Set a simple time-based stop trigger
    pub fn set_stop_time(&mut self, seconds: f64) {
        use crate::storyboard::StopTrigger;
        self.storyboard
            .set_stop_trigger(StopTrigger::simulation_time(seconds));
    }

    /// Set a stop trigger based on storyboard element state
    pub fn set_stop_on_element_state(
        &mut self,
        element_type: impl Into<String>,
        element_ref: impl Into<String>,
        state: impl Into<String>,
        delay: f64,
    ) {
        use crate::storyboard::StopTrigger;
        self.storyboard
            .set_stop_trigger(StopTrigger::storyboard_element_state(
                element_type,
                element_ref,
                state,
                delay,
            ));
    }
}

#[cfg(test)]
mod parameter_tests {
    use super::*;

    #[test]
    fn test_add_parameter_declaration() {
        let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

        let result = scenario.add_parameter("MaxSpeed", ParameterType::Double, "60.0");

        assert!(result.is_ok());
        assert_eq!(scenario.parameters.len(), 1);
        assert_eq!(scenario.parameters[0].name, "MaxSpeed");
    }

    #[test]
    fn test_add_parameter_duplicate_error() {
        let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
        scenario
            .add_parameter("Speed", ParameterType::Double, "50.0")
            .unwrap();

        let result = scenario.add_parameter("Speed", ParameterType::Integer, "60");
        assert!(
            matches!(result, Err(ScenarioError::ParameterConflict { name }) if name == "Speed")
        );
    }

    #[test]
    fn test_add_parameter_all_types() {
        let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

        // Integer
        scenario
            .add_parameter("Count", ParameterType::Integer, "42")
            .unwrap();
        assert_eq!(
            scenario.parameters[0].parameter_type,
            ParameterType::Integer
        );

        // Double (already tested, but verify here too)
        scenario
            .add_parameter("Speed", ParameterType::Double, "60.0")
            .unwrap();
        assert_eq!(scenario.parameters[1].parameter_type, ParameterType::Double);

        // String
        scenario
            .add_parameter("State", ParameterType::String, "active")
            .unwrap();
        assert_eq!(scenario.parameters[2].parameter_type, ParameterType::String);

        // Boolean
        scenario
            .add_parameter("Debug", ParameterType::Boolean, "true")
            .unwrap();
        assert_eq!(
            scenario.parameters[3].parameter_type,
            ParameterType::Boolean
        );
    }
}
