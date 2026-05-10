//! Core OpenSCENARIO scenario builder.
//!
//! This module provides the main `Scenario` struct and its methods for building
//! OpenSCENARIO XML files programmatically.
//!
//! # Design Note: Method Signatures
//!
//! Many methods in this module have numerous parameters (5-10+). This complexity
//! reflects the OpenSCENARIO standard itself, which requires detailed specification
//! of scenario elements. The API prioritizes explicitness and type safety over
//! conciseness, ensuring users provide all required information for valid scenarios.
//!
//! For convenience methods with fewer parameters, see the "simple" variants (e.g.,
//! `add_event_with_ttc_condition` vs `add_event_with_ttc_condition_advanced`).

use crate::entities::{
    Entity, MiscObject, MiscObjectParams, Pedestrian, PedestrianParams, Vehicle, VehicleParams,
};
use crate::storyboard::{
    Act, Action, ByEntityCondition, CollisionCondition, Condition, ConditionEdge,
    ConditionGroup, ConditionKind, DistanceAction, EntityCondition, Event, LaneChangeAction,
    Maneuver, ManeuverGroup, PositionAction, ReachPositionCondition, Rule, SpeedAction, Story,
    Storyboard, TimeToCollisionCondition, TransitionShape, Trigger, TriggeringEntities,
    TriggeringEntitiesRule,
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
    /// Creates a new OpenSCENARIO scenario.
    ///
    /// # Arguments
    /// * `version` - OpenSCENARIO specification version (V1_0, V1_1, or V1_2)
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    ///
    /// let scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// ```
    pub fn new(version: OpenScenarioVersion) -> Self {
        Self {
            version,
            entities: HashMap::new(),
            initial_positions: HashMap::new(),
            parameters: Vec::new(),
            storyboard: Storyboard::new(),
        }
    }

    /// Returns the OpenSCENARIO version of this scenario.
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    ///
    /// let scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// assert_eq!(scenario.version(), OpenScenarioVersion::V1_2);
    /// ```
    pub fn version(&self) -> OpenScenarioVersion {
        self.version
    }

    /// Adds a parameter declaration to the scenario.
    ///
    /// Parameters can be used for runtime configuration of scenario values.
    /// Parameter names must be unique within the scenario.
    ///
    /// # Arguments
    /// * `name` - Unique parameter name
    /// * `parameter_type` - Type of the parameter (Integer, Double, String, Boolean, etc.)
    /// * `value` - Default value as a string
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError::ParameterConflict)` if parameter name already exists
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion, ParameterType};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// scenario.add_parameter("MaxSpeed", ParameterType::Double, "60.0")?;
    /// scenario.add_parameter("EnableACC", ParameterType::Boolean, "true")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::ParameterConflict` - If a parameter with the same name already exists
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

    /// Adds a vehicle entity to the scenario.
    ///
    /// Vehicles are the primary entity type for automotive scenarios.
    /// Entity names must be unique across all entities in the scenario.
    ///
    /// # Arguments
    /// * `name` - Unique entity name (whitespace is trimmed)
    /// * `params` - Vehicle parameters (category, catalog reference, properties)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// 
    /// let vehicle_params = VehicleParams {
    ///     catalog: None,
    ///     vehicle_category: VehicleCategory::Car,
    ///     properties: None,
    /// };
    /// 
    /// scenario.add_vehicle("ego_vehicle", vehicle_params)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If name is empty or whitespace-only
    /// * `ScenarioError::EntityConflict` - If an entity with this name already exists
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

    /// Adds a pedestrian entity to the scenario.
    ///
    /// Pedestrians represent human actors in the scenario (e.g., crossing streets,
    /// walking on sidewalks). Entity names must be unique.
    ///
    /// # Arguments
    /// * `name` - Unique entity name (whitespace is trimmed)
    /// * `params` - Pedestrian parameters (model, mass, catalog reference)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::entities::PedestrianParams;
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// 
    /// let pedestrian_params = PedestrianParams {
    ///     catalog: None,
    ///     model: Some("adult_male".to_string()),
    ///     mass: Some(75.0),
    /// };
    /// 
    /// scenario.add_pedestrian("pedestrian1", pedestrian_params)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If name is empty or whitespace-only
    /// * `ScenarioError::EntityConflict` - If an entity with this name already exists
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

    /// Adds a miscellaneous object entity to the scenario.
    ///
    /// Miscellaneous objects represent static or dynamic non-vehicle, non-pedestrian
    /// entities (e.g., traffic cones, barriers, obstacles). Entity names must be unique.
    ///
    /// # Arguments
    /// * `name` - Unique entity name (whitespace is trimmed)
    /// * `params` - Miscellaneous object parameters (category, mass, catalog reference)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::entities::MiscObjectParams;
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// 
    /// let object_params = MiscObjectParams {
    ///     catalog: None,
    ///     category: Some("obstacle".to_string()),
    ///     mass: Some(50.0),
    /// };
    /// 
    /// scenario.add_misc_object("traffic_cone", object_params)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If name is empty or whitespace-only
    /// * `ScenarioError::EntityConflict` - If an entity with this name already exists
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

    /// Gets a reference to an entity by name.
    ///
    /// # Arguments
    /// * `name` - Name of the entity to retrieve
    ///
    /// # Returns
    /// * `Some(&Entity)` if entity exists
    /// * `None` if no entity with that name exists
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::entities::VehicleParams;
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vehicle_params = VehicleParams {
    /// #     catalog: None,
    /// #     vehicle_category: openscenario::entities::VehicleCategory::Car,
    /// #     properties: None,
    /// # };
    /// scenario.add_vehicle("ego", vehicle_params)?;
    /// 
    /// assert!(scenario.get_entity("ego").is_some());
    /// assert!(scenario.get_entity("nonexistent").is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_entity(&self, name: &str) -> Option<&Entity> {
        self.entities.get(name)
    }

    /// Returns an iterator over all entities in the scenario.
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    ///
    /// let scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// let entity_count = scenario.entities().count();
    /// assert_eq!(entity_count, 0);
    /// ```
    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    /// Gets the initial position of an entity.
    ///
    /// # Arguments
    /// * `entity` - Name of the entity
    ///
    /// # Returns
    /// * `Some(&Position)` if entity has an initial position
    /// * `None` if no initial position is set
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion, Position};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vehicle_params = openscenario::entities::VehicleParams {
    /// #     catalog: None,
    /// #     vehicle_category: openscenario::entities::VehicleCategory::Car,
    /// #     properties: None,
    /// # };
    /// # scenario.add_vehicle("ego", vehicle_params)?;
    /// scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))?;
    /// 
    /// assert!(scenario.get_initial_position("ego").is_some());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_initial_position(&self, entity: &str) -> Option<&Position> {
        self.initial_positions.get(entity)
    }

    /// Returns an iterator over all initial positions.
    ///
    /// # Returns
    /// Iterator of (entity_name, position) tuples
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    ///
    /// let scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// let position_count = scenario.initial_positions().count();
    /// assert_eq!(position_count, 0);
    /// ```
    pub fn initial_positions(&self) -> impl Iterator<Item = (&String, &Position)> {
        self.initial_positions.iter()
    }

    /// Sets the initial position for an entity.
    ///
    /// Initial positions define where entities start in the scenario.
    /// The entity must exist before setting its initial position.
    ///
    /// # Arguments
    /// * `entity` - Name of the entity (whitespace is trimmed)
    /// * `position` - Initial position (World, Lane, Road, or Relative)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion, Position};
    /// use openscenario::entities::VehicleParams;
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vehicle_params = VehicleParams {
    /// #     catalog: None,
    /// #     vehicle_category: openscenario::entities::VehicleCategory::Car,
    /// #     properties: None,
    /// # };
    /// scenario.add_vehicle("ego", vehicle_params)?;
    /// 
    /// // Set world position (x, y, z, heading)
    /// scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))?;
    /// 
    /// // Or use lane position
    /// // scenario.set_initial_position("ego", Position::lane("road1", 1, 10.0, 0.0))?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If entity name is empty or whitespace-only
    /// * `ScenarioError::EntityNotFound` - If entity doesn't exist
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

    /// Adds a story to the scenario's storyboard.
    ///
    /// Stories are the top-level organizational unit in OpenSCENARIO.
    /// A story contains acts, which contain maneuver groups and maneuvers.
    /// Story names must be unique.
    ///
    /// # Arguments
    /// * `name` - Unique story name (whitespace is trimmed)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// scenario.add_story("main_story")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If name is empty or whitespace-only
    /// * `ScenarioError::StoryNotFound` - If a story with this name already exists
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

    /// Adds an act to a story.
    ///
    /// Acts are organizational units within a story. Each act contains maneuver groups
    /// which define behaviors for specific actors. Act names must be unique within a story.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `name` - Unique act name within the story
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// scenario.add_story("main_story")?;
    /// scenario.add_act("main_story", "act1")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If act name already exists in this story
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

    /// Sets the start trigger for an act.
    ///
    /// The start trigger defines the conditions that must be met before
    /// the act begins executing. Acts are organizational units within a story.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the act
    /// * `trigger` - Trigger definition with condition groups
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::Trigger;
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// scenario.add_story("s1")?;
    /// scenario.add_act("s1", "a1")?;
    /// 
    /// let trigger = Trigger { condition_groups: vec![] };
    /// scenario.set_act_start_trigger("s1", "a1", trigger)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If act doesn't exist
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

    /// Sets the start trigger for an event.
    ///
    /// The start trigger defines the conditions that must be met before
    /// the event's actions are executed. Events contain actions and are
    /// part of maneuvers.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `maneuver_group` - Name of the maneuver group
    /// * `maneuver` - Name of the maneuver
    /// * `event` - Name of the event
    /// * `trigger` - Trigger definition with condition groups
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion, Position};
    /// use openscenario::storyboard::Trigger;
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// # // Create the event first by adding an action
    /// # let pos = Position::world(0.0, 0.0, 0.0, 0.0);
    /// # scenario.add_position_action("s1", "a1", "mg1", "m1", "event1", pos)?;
    /// 
    /// let trigger = Trigger { condition_groups: vec![] };
    /// scenario.set_event_start_trigger("s1", "a1", "mg1", "m1", "event1", trigger)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent structures don't exist
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

    /// Adds a maneuver group to an act.
    ///
    /// Maneuver groups contain maneuvers and define which actors (entities) participate
    /// in those maneuvers. Each act can have multiple maneuver groups.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `name` - Unique maneuver group name within the act
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// scenario.add_story("main_story")?;
    /// scenario.add_act("main_story", "act1")?;
    /// scenario.add_maneuver_group("main_story", "act1", "mg1")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent act doesn't exist or mg name already exists
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

    /// Adds an actor (entity) to a maneuver group.
    ///
    /// Actors are the entities that will perform the maneuvers defined in the maneuver group.
    /// Each actor must be an existing entity in the scenario. An entity can only be added once
    /// to a given maneuver group (duplicates are ignored).
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `entity` - Name of the entity to add as an actor (whitespace is trimmed)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vehicle_params = VehicleParams {
    /// #     catalog: None,
    /// #     vehicle_category: VehicleCategory::Car,
    /// #     properties: None,
    /// # };
    /// scenario.add_vehicle("ego", vehicle_params)?;
    /// scenario.add_story("story1")?;
    /// scenario.add_act("story1", "act1")?;
    /// scenario.add_maneuver_group("story1", "act1", "mg1")?;
    /// scenario.add_actor("story1", "act1", "mg1", "ego")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If entity name is empty or whitespace-only
    /// * `ScenarioError::EntityNotFound` - If entity doesn't exist or story/act/mg not found
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
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

    /// Adds a maneuver to a maneuver group.
    ///
    /// Maneuvers are sequences of events that define actions for actors.
    /// Each maneuver contains events that trigger actions based on conditions.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `name` - Unique maneuver name within the maneuver group
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// scenario.add_story("story1")?;
    /// scenario.add_act("story1", "act1")?;
    /// scenario.add_maneuver_group("story1", "act1", "mg1")?;
    /// scenario.add_maneuver("story1", "act1", "mg1", "lane_change")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent act or maneuver group doesn't exist
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

    /// Adds a speed action to an event.
    ///
    /// Speed actions command an actor to change its speed to a target value
    /// using specified dynamics (shape, dimension, and value).
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name of the event (created if it doesn't exist)
    /// * `target_speed` - Target speed in m/s (must be non-negative)
    /// * `dynamics` - Transition dynamics (shape, dimension, value > 0)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::{TransitionDynamics, DynamicsShape, DynamicsDimension};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # scenario.add_vehicle("ego", VehicleParams {
    /// #     catalog: None,
    /// #     vehicle_category: VehicleCategory::Car,
    /// #     properties: None,
    /// # })?;
    /// # scenario.add_story("story1")?;
    /// # scenario.add_act("story1", "act1")?;
    /// # scenario.add_maneuver_group("story1", "act1", "mg1")?;
    /// # scenario.add_actor("story1", "act1", "mg1", "ego")?;
    /// # scenario.add_maneuver("story1", "act1", "mg1", "m1")?;
    /// 
    /// let dynamics = TransitionDynamics {
    ///     shape: DynamicsShape::Linear,
    ///     dimension: DynamicsDimension::Time,
    ///     value: 3.0,  // 3 seconds
    /// };
    /// scenario.add_speed_action("story1", "act1", "mg1", "m1", "event1", 20.0, dynamics)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If target_speed < 0 or dynamics.value <= 0
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent act, mg, or maneuver doesn't exist
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

    /// Adds a lane change action to an event.
    ///
    /// Lane change actions command an actor to change lanes with a specified
    /// target lane offset, duration, and transition shape.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name of the event (created if it doesn't exist)
    /// * `target_lane_offset` - Target lateral offset from lane center in meters
    /// * `duration` - Duration of the lane change in seconds (must be positive)
    /// * `shape` - Transition shape (Linear, Cubic, Sinusoidal, Step)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::TransitionShape;
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # scenario.add_vehicle("ego", VehicleParams {
    /// #     catalog: None,
    /// #     vehicle_category: VehicleCategory::Car,
    /// #     properties: None,
    /// # })?;
    /// # scenario.add_story("story1")?;
    /// # scenario.add_act("story1", "act1")?;
    /// # scenario.add_maneuver_group("story1", "act1", "mg1")?;
    /// # scenario.add_actor("story1", "act1", "mg1", "ego")?;
    /// # scenario.add_maneuver("story1", "act1", "mg1", "m1")?;
    /// 
    /// // Change to left lane (offset -3.5m) over 4 seconds with sinusoidal profile
    /// scenario.add_lane_change_action("story1", "act1", "mg1", "m1", "event1", 
    ///                                  -3.5, 4.0, TransitionShape::Sinusoidal)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If duration <= 0
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent act, mg, or maneuver doesn't exist
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

    /// Adds an acceleration action to an event.
    ///
    /// Acceleration actions command an actor to accelerate or decelerate at a specified
    /// rate over a given duration. Negative acceleration values represent deceleration.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name of the event (created if it doesn't exist)
    /// * `acceleration` - Target acceleration in m/s² (positive or negative)
    /// * `duration` - Duration of acceleration in seconds (must be positive)
    /// * `dynamics` - Optional transition dynamics (defaults to linear time-based)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # scenario.add_vehicle("ego", VehicleParams {
    /// #     catalog: None,
    /// #     vehicle_category: VehicleCategory::Car,
    /// #     properties: None,
    /// # })?;
    /// # scenario.add_story("story1")?;
    /// # scenario.add_act("story1", "act1")?;
    /// # scenario.add_maneuver_group("story1", "act1", "mg1")?;
    /// # scenario.add_actor("story1", "act1", "mg1", "ego")?;
    /// # scenario.add_maneuver("story1", "act1", "mg1", "m1")?;
    /// 
    /// // Accelerate at 2 m/s² for 5 seconds
    /// scenario.add_acceleration_action("story1", "act1", "mg1", "m1", "event1", 
    ///                                   2.0, 5.0, None)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If duration <= 0 or dynamics.value <= 0
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent act, mg, or maneuver doesn't exist
    #[allow(clippy::too_many_arguments)]
    pub fn add_acceleration_action(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        acceleration: f64,
        duration: f64,
        dynamics: Option<crate::storyboard::TransitionDynamics>,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();

        // Validate duration (must be positive)
        if duration <= 0.0 {
            return Err(ScenarioError::InvalidValue {
                field: "duration".to_string(),
                reason: format!("duration must be positive (got {})", duration),
            });
        }

        // Use provided dynamics or default to linear time-based
        let final_dynamics = dynamics.unwrap_or(crate::storyboard::TransitionDynamics {
            shape: crate::storyboard::DynamicsShape::Linear,
            dimension: crate::storyboard::DynamicsDimension::Time,
            value: duration,
        });

        // Validate dynamics value
        if final_dynamics.value <= 0.0 {
            return Err(ScenarioError::InvalidValue {
                field: "dynamics.value".to_string(),
                reason: format!("dynamics value must be positive (got {})", final_dynamics.value),
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

        let action = Action::Acceleration(crate::storyboard::AccelerationAction {
            value: acceleration,
            dynamics: final_dynamics,
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

    /// Adds a lane offset action to an event.
    ///
    /// Lane offset actions command an actor to move to a lateral offset from the lane center.
    /// The action can be continuous (ongoing) or discrete (one-time).
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name of the event (created if it doesn't exist)
    /// * `target_offset` - Target lateral offset from lane center in meters
    /// * `continuous` - If true, maintains offset; if false, returns to lane center after
    /// * `dynamics` - Optional transition dynamics (how to reach the offset)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::{TransitionDynamics, DynamicsShape, DynamicsDimension};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # scenario.add_vehicle("ego", VehicleParams {
    /// #     catalog: None,
    /// #     vehicle_category: VehicleCategory::Car,
    /// #     properties: None,
    /// # })?;
    /// # scenario.add_story("story1")?;
    /// # scenario.add_act("story1", "act1")?;
    /// # scenario.add_maneuver_group("story1", "act1", "mg1")?;
    /// # scenario.add_actor("story1", "act1", "mg1", "ego")?;
    /// # scenario.add_maneuver("story1", "act1", "mg1", "m1")?;
    /// 
    /// let dynamics = TransitionDynamics {
    ///     shape: DynamicsShape::Sinusoidal,
    ///     dimension: DynamicsDimension::Time,
    ///     value: 2.0,
    /// };
    /// // Move 0.5m right continuously
    /// scenario.add_lane_offset_action("story1", "act1", "mg1", "m1", "event1",
    ///                                  0.5, true, Some(dynamics))?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If dynamics.value <= 0
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent act, mg, or maneuver doesn't exist
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

    /// Adds a follow trajectory action to an event.
    ///
    /// Follow trajectory actions command an actor to follow a predefined path
    /// (trajectory) defined by a sequence of vertices. The timing mode determines
    /// whether to match timestamps exactly or just follow the path.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name of the event (created if it doesn't exist)
    /// * `trajectory` - Trajectory with at least 2 vertices
    /// * `timing_mode` - How to interpret vertex timing (strict, relative, etc.)
    /// * `initial_distance_offset` - Optional starting offset along trajectory (>= 0)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion, Position};
    /// use openscenario::storyboard::{Trajectory, Vertex, TimingMode};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # scenario.add_vehicle("ego", VehicleParams {
    /// #     catalog: None,
    /// #     vehicle_category: VehicleCategory::Car,
    /// #     properties: None,
    /// # })?;
    /// # scenario.add_story("story1")?;
    /// # scenario.add_act("story1", "act1")?;
    /// # scenario.add_maneuver_group("story1", "act1", "mg1")?;
    /// # scenario.add_actor("story1", "act1", "mg1", "ego")?;
    /// # scenario.add_maneuver("story1", "act1", "mg1", "m1")?;
    /// 
    /// let trajectory = Trajectory {
    ///     name: "path1".to_string(),
    ///     closed: false,
    ///     vertices: vec![
    ///         Vertex {
    ///             position: Position::world(0.0, 0.0, 0.0, 0.0),
    ///             time: 0.0,
    ///         },
    ///         Vertex {
    ///             position: Position::world(100.0, 0.0, 0.0, 0.0),
    ///             time: 10.0,
    ///         },
    ///     ],
    /// };
    /// scenario.add_follow_trajectory_action("story1", "act1", "mg1", "m1", "event1",
    ///                                       trajectory, TimingMode::Timing, None)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If trajectory has < 2 vertices or offset < 0
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent act, mg, or maneuver doesn't exist
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

    /// Adds an assign route action to an event.
    ///
    /// Assign route actions command an actor to follow a route defined by waypoints.
    /// The route planner will determine the specific path between waypoints.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name of the event (created if it doesn't exist)
    /// * `route` - Route definition with at least 2 waypoints
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion, Position};
    /// use openscenario::storyboard::{Route, Waypoint};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # scenario.add_vehicle("ego", VehicleParams {
    /// #     catalog: None,
    /// #     vehicle_category: VehicleCategory::Car,
    /// #     properties: None,
    /// # })?;
    /// # scenario.add_story("story1")?;
    /// # scenario.add_act("story1", "act1")?;
    /// # scenario.add_maneuver_group("story1", "act1", "mg1")?;
    /// # scenario.add_actor("story1", "act1", "mg1", "ego")?;
    /// # scenario.add_maneuver("story1", "act1", "mg1", "m1")?;
    /// 
    /// let route = Route {
    ///     name: "route1".to_string(),
    ///     closed: false,
    ///     waypoints: vec![
    ///         Waypoint {
    ///             position: Position::world(0.0, 0.0, 0.0, 0.0),
    ///             route_strategy: None,
    ///         },
    ///         Waypoint {
    ///             position: Position::world(1000.0, 500.0, 0.0, 1.57),
    ///             route_strategy: None,
    ///         },
    ///     ],
    /// };
    /// scenario.add_assign_route_action("story1", "act1", "mg1", "m1", "event1", route)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If route has < 2 waypoints
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent act, mg, or maneuver doesn't exist
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

    /// Adds a synchronize action to an event.
    ///
    /// Synchronize actions coordinate two entities so they reach target positions
    /// at the same time. One entity is designated as the master, and the other
    /// (entity_ref) adjusts its speed to synchronize arrival.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name of the event (created if it doesn't exist)
    /// * `entity_ref` - Name of the entity that will synchronize (must exist)
    /// * `master_entity_ref` - Name of the master entity to sync with (must exist)
    /// * `target_position_master` - Target position for the master entity
    /// * `target_position` - Target position for the synchronizing entity
    /// * `final_speed` - Optional final speed in m/s (must be non-negative)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::{TargetPositionMaster, TargetPosition};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp.clone())?;
    /// # scenario.add_vehicle("target", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_actor("s1", "a1", "mg1", "ego")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// // Synchronization action would be added here (requires position types)
    /// // scenario.add_synchronize_action(...)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::EntityNotFound` - If entity_ref or master_entity_ref doesn't exist
    /// * `ScenarioError::InvalidValue` - If final_speed < 0
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
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

    /// Adds a position action to an event.
    ///
    /// Position actions teleport an actor to a specific position instantly.
    /// This is useful for setup, testing, or sudden position changes.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name of the event (created if it doesn't exist)
    /// * `position` - Target position (World, Lane, Road, or Relative)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion, Position};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_actor("s1", "a1", "mg1", "ego")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// let position = Position::world(100.0, 50.0, 0.0, 1.57);
    /// scenario.add_position_action("s1", "a1", "mg1", "m1", "event1", position)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent act, mg, or maneuver doesn't exist
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

    /// Adds a distance action to an event.
    ///
    /// Distance actions maintain a specified distance to another entity.
    /// The distance can be measured in freespace (shortest distance) or
    /// along the road/lane network.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name of the event (created if it doesn't exist)
    /// * `entity_ref` - Name of the reference entity (must exist)
    /// * `distance` - Target distance in meters (must be non-negative)
    /// * `freespace` - If true, use straight-line distance; if false, use road distance
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp.clone())?;
    /// # scenario.add_vehicle("lead", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_actor("s1", "a1", "mg1", "ego")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Maintain 20m distance from lead vehicle using road distance
    /// scenario.add_distance_action("s1", "a1", "mg1", "m1", "event1", "lead", 20.0, false)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If distance < 0
    /// * `ScenarioError::EntityNotFound` - If entity_ref doesn't exist or parent structures not found
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
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

    /// Adds a longitudinal distance action to maintain distance from another entity.
    ///
    /// Longitudinal distance actions control the fore-aft spacing between entities,
    /// maintaining a specified distance along the direction of travel. The action can
    /// be continuous (ongoing) or one-time.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name of the event (created if it doesn't exist)
    /// * `entity_ref` - Name of the reference entity (must exist)
    /// * `distance` - Target longitudinal distance in meters
    /// * `freespace` - If true, use straight-line distance; if false, use road distance
    /// * `continuous` - If true, maintains distance; if false, reaches distance once
    /// * `dynamics` - Optional transition dynamics for reaching the target distance
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp.clone())?;
    /// # scenario.add_vehicle("lead", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_actor("s1", "a1", "mg1", "ego")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Continuously maintain 30m longitudinal distance from lead vehicle
    /// scenario.add_longitudinal_distance_action("s1", "a1", "mg1", "m1", "event1",
    ///                                            "lead", 30.0, false, true, None)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::EntityNotFound` - If entity_ref doesn't exist or parent structures not found
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
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

    /// Sets a simple time-based stop trigger for the scenario.
    ///
    /// The stop trigger defines when the scenario simulation should end.
    /// This convenience method creates a trigger based on simulation time.
    ///
    /// # Arguments
    /// * `seconds` - Simulation time in seconds when the scenario should stop
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    ///
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// scenario.set_stop_time(60.0);  // Stop after 60 seconds
    /// ```
    pub fn set_stop_time(&mut self, seconds: f64) {
        use crate::storyboard::StopTrigger;
        self.storyboard
            .set_stop_trigger(StopTrigger::simulation_time(seconds));
    }

    /// Sets a stop trigger based on storyboard element state.
    ///
    /// The scenario will stop when a specified element (story, act, maneuver, event, action)
    /// reaches a target state (e.g., completeState, startTransition, endTransition).
    ///
    /// # Arguments
    /// * `element_type` - Type of storyboard element. Valid types:
    ///   - `"story"` - A Story element
    ///   - `"act"` - An Act element
    ///   - `"maneuver"` - A Maneuver element
    ///   - `"event"` - An Event element
    ///   - `"action"` - An Action element
    /// * `element_ref` - Name of the storyboard element to watch
    /// * `state` - Target state that triggers stopping. Common states:
    ///   - `"completeState"` - Element has completed
    ///   - `"startTransition"` - Element is starting
    ///   - `"endTransition"` - Element is ending
    ///   - `"runningState"` - Element is running
    ///   - `"standbyState"` - Element is on standby
    /// * `delay` - Delay in seconds after state is reached before stopping
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    ///
    /// let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// 
    /// // Stop when the main story completes
    /// scenario.set_stop_on_element_state("story", "main_story", "completeState", 0.0);
    /// 
    /// // Stop 2 seconds after an act starts
    /// scenario.set_stop_on_element_state("act", "act1", "startTransition", 2.0);
    /// ```
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

    /// Adds an event with a reach position condition trigger.
    ///
    /// Creates an event that triggers when the specified entity reaches
    /// the target position within the given tolerance.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event_name` - Name for the new event
    /// * `entity` - Name of the entity to watch (must exist)
    /// * `position` - Target position to reach
    /// * `tolerance` - Distance tolerance in meters for reaching the position
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion, Position};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// let target = Position::world(100.0, 50.0, 0.0, 0.0);
    /// scenario.add_event_with_reach_position_condition(
    ///     "s1", "a1", "mg1", "m1", "reach_event", "ego", target, 2.0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::EntityNotFound` - If entity doesn't exist or parent structures not found
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    #[allow(clippy::too_many_arguments)]
    pub fn add_event_with_reach_position_condition(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        position: Position,
        tolerance: f64,
    ) -> Result<()> {
        self.add_event_with_reach_position_condition_advanced(
            story,
            act,
            mg,
            maneuver,
            event,
            entity_ref,
            position,
            tolerance,
            ConditionEdge::None,
            0.0,
        )
    }

    /// Adds an event with a reach position condition trigger (advanced).
    ///
    /// Creates an event with full control over condition edge and delay.
    /// The edge parameter controls when the condition triggers (rising/falling/none),
    /// and delay adds time after the condition becomes true.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name for the new event
    /// * `entity_ref` - Name of the entity to watch (must exist)
    /// * `position` - Target position to reach
    /// * `tolerance` - Distance tolerance in meters (must be non-negative)
    /// * `edge` - Condition edge (Rising, Falling, RisingOrFalling, None)
    /// * `delay` - Delay in seconds after condition becomes true
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion, Position};
    /// use openscenario::storyboard::ConditionEdge;
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// let target = Position::world(100.0, 50.0, 0.0, 0.0);
    /// scenario.add_event_with_reach_position_condition_advanced(
    ///     "s1", "a1", "mg1", "m1", "reach_event", "ego", 
    ///     target, 2.0, ConditionEdge::Rising, 1.0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If tolerance < 0
    /// * `ScenarioError::InvalidEntityRef` - If entity doesn't exist
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent structures not found
    #[allow(clippy::too_many_arguments)]
    pub fn add_event_with_reach_position_condition_advanced(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        position: Position,
        tolerance: f64,
        edge: ConditionEdge,
        delay: f64,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();
        let entity_name = entity_ref.into();

        // Validate tolerance (must be non-negative)
        if tolerance < 0.0 {
            return Err(ScenarioError::InvalidValue {
                field: "tolerance".to_string(),
                reason: format!("tolerance cannot be negative (got {})", tolerance),
            });
        }

        // Validate entity exists
        let available: Vec<String> = self.entities.keys().cloned().collect();
        if !self.entities.contains_key(&entity_name) {
            return Err(ScenarioError::InvalidEntityRef {
                entity: entity_name.clone(),
                available,
            });
        }

        // Get story
        let story_keys: Vec<String> = self.storyboard.stories.keys().cloned().collect();
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available: story_keys,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg = act
            .maneuver_groups
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

        // Create the reach position condition
        let reach_condition = ReachPositionCondition {
            position,
            tolerance,
        };

        let entity_condition = EntityCondition::ReachPosition(reach_condition);

        let triggering_entities = TriggeringEntities {
            rule: TriggeringEntitiesRule::Any,
            entity_refs: vec![entity_name],
        };

        let by_entity_condition = ByEntityCondition {
            triggering_entities,
            entity_condition,
        };

        let condition = Condition {
            name: format!("{}Condition", event_name),
            delay,
            condition_edge: edge,
            kind: ConditionKind::ByEntity(by_entity_condition),
        };

        let condition_group = ConditionGroup {
            conditions: vec![condition],
        };

        let trigger = Trigger {
            condition_groups: vec![condition_group],
        };

        // Create or update event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.start_trigger = Some(trigger);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![],
                start_trigger: Some(trigger),
            });
        }

        Ok(())
    }

    /// Adds an event with a time-to-collision (TTC) condition trigger.
    ///
    /// Creates an event that triggers when the time-to-collision between
    /// the specified entity and a target entity meets the rule threshold.
    /// TTC is the estimated time until collision if current velocities are maintained.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name for the new event
    /// * `entity_ref` - Name of the entity to monitor (must exist)
    /// * `target_entity_ref` - Name of the target entity (must exist)
    /// * `ttc_value` - Time-to-collision threshold in seconds
    /// * `rule` - Comparison rule (LessThan, GreaterThan, EqualTo)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::Rule;
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp.clone())?;
    /// # scenario.add_vehicle("target", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Trigger when TTC drops below 3 seconds
    /// scenario.add_event_with_ttc_condition(
    ///     "s1", "a1", "mg1", "m1", "ttc_event", 
    ///     "ego", "target", 3.0, Rule::LessThan)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::EntityNotFound` - If entities don't exist or parent structures not found
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    #[allow(clippy::too_many_arguments)]
    pub fn add_event_with_ttc_condition(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        target_entity_ref: impl Into<String>,
        ttc_value: f64,
        rule: Rule,
    ) -> Result<()> {
        self.add_event_with_ttc_condition_advanced(
            story,
            act,
            mg,
            maneuver,
            event,
            entity_ref,
            target_entity_ref,
            ttc_value,
            rule,
            ConditionEdge::None,
            0.0,
        )
    }

    /// Adds an event with a time-to-collision condition trigger (advanced).
    ///
    /// Creates an event with full control over condition edge and delay.
    /// The edge parameter controls when the condition triggers, and delay
    /// adds time after the condition becomes true.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name for the new event
    /// * `entity_ref` - Name of the entity to monitor (must exist)
    /// * `target_entity_ref` - Name of the target entity (must exist)
    /// * `ttc_value` - Time-to-collision threshold in seconds (must be non-negative)
    /// * `rule` - Comparison rule (LessThan, GreaterThan, EqualTo)
    /// * `edge` - Condition edge (Rising, Falling, RisingOrFalling, None)
    /// * `delay` - Delay in seconds after condition becomes true
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::{Rule, ConditionEdge};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp.clone())?;
    /// # scenario.add_vehicle("target", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Trigger on rising edge when TTC drops below 3s, with 0.5s delay
    /// scenario.add_event_with_ttc_condition_advanced(
    ///     "s1", "a1", "mg1", "m1", "ttc_event", 
    ///     "ego", "target", 3.0, Rule::LessThan, ConditionEdge::Rising, 0.5)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If ttc_value < 0
    /// * `ScenarioError::InvalidEntityRef` - If entities don't exist
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent structures not found
    #[allow(clippy::too_many_arguments)]
    pub fn add_event_with_ttc_condition_advanced(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        target_entity_ref: impl Into<String>,
        ttc_value: f64,
        rule: Rule,
        edge: ConditionEdge,
        delay: f64,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();
        let entity_name = entity_ref.into();
        let target_name = target_entity_ref.into();

        // Validate TTC value (must be non-negative)
        if ttc_value < 0.0 {
            return Err(ScenarioError::InvalidValue {
                field: "ttc_value".to_string(),
                reason: format!("TTC value cannot be negative (got {})", ttc_value),
            });
        }

        // Validate both entities exist
        let available: Vec<String> = self.entities.keys().cloned().collect();
        if !self.entities.contains_key(&entity_name) {
            return Err(ScenarioError::InvalidEntityRef {
                entity: entity_name.clone(),
                available: available.clone(),
            });
        }
        if !self.entities.contains_key(&target_name) {
            return Err(ScenarioError::InvalidEntityRef {
                entity: target_name.clone(),
                available,
            });
        }

        // Get story
        let story_keys: Vec<String> = self.storyboard.stories.keys().cloned().collect();
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available: story_keys,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg = act
            .maneuver_groups
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

        // Create the TTC condition
        let ttc_condition = TimeToCollisionCondition {
            target_entity_ref: target_name,
            value: ttc_value,
            rule,
        };

        let entity_condition = EntityCondition::TimeToCollision(ttc_condition);

        let triggering_entities = TriggeringEntities {
            rule: TriggeringEntitiesRule::Any,
            entity_refs: vec![entity_name],
        };

        let by_entity_condition = ByEntityCondition {
            triggering_entities,
            entity_condition,
        };

        let condition = Condition {
            name: format!("{}Condition", event_name),
            delay,
            condition_edge: edge,
            kind: ConditionKind::ByEntity(by_entity_condition),
        };

        let condition_group = ConditionGroup {
            conditions: vec![condition],
        };

        let trigger = Trigger {
            condition_groups: vec![condition_group],
        };

        // Create or update event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.start_trigger = Some(trigger);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![],
                start_trigger: Some(trigger),
            });
        }

        Ok(())
    }

    /// Adds an event with a collision condition trigger.
    ///
    /// Creates an event that triggers when the specified entity collides
    /// with a target entity. Collision is typically detected by physics simulation
    /// or geometric overlap.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name for the new event
    /// * `entity_ref` - Name of the entity to monitor (must exist)
    /// * `target_entity_ref` - Name of the target entity to collide with (must exist)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp.clone())?;
    /// # scenario.add_vehicle("obstacle", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Trigger when ego collides with obstacle
    /// scenario.add_event_with_collision_condition(
    ///     "s1", "a1", "mg1", "m1", "collision_event", "ego", "obstacle")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::EntityNotFound` - If entities don't exist or parent structures not found
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    #[allow(clippy::too_many_arguments)]
    pub fn add_event_with_collision_condition(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        target_entity_ref: impl Into<String>,
    ) -> Result<()> {
        self.add_event_with_collision_condition_advanced(
            story,
            act,
            mg,
            maneuver,
            event,
            entity_ref,
            target_entity_ref,
            ConditionEdge::None,
            0.0,
        )
    }

    /// Adds an event with a collision condition trigger (advanced).
    ///
    /// Creates an event with full control over condition edge and delay.
    /// Allows specification of when exactly the trigger fires relative to
    /// the collision event.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name for the new event
    /// * `entity_ref` - Name of the entity to monitor (must exist)
    /// * `target_entity_ref` - Name of the target entity (must exist)
    /// * `edge` - Condition edge (Rising, Falling, RisingOrFalling, None)
    /// * `delay` - Delay in seconds after condition becomes true
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::ConditionEdge;
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp.clone())?;
    /// # scenario.add_vehicle("obstacle", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Trigger on collision with 0.1s delay
    /// scenario.add_event_with_collision_condition_advanced(
    ///     "s1", "a1", "mg1", "m1", "collision_event", 
    ///     "ego", "obstacle", ConditionEdge::Rising, 0.1)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidEntityRef` - If entities don't exist
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent structures not found
    #[allow(clippy::too_many_arguments)]
    pub fn add_event_with_collision_condition_advanced(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        target_entity_ref: impl Into<String>,
        edge: ConditionEdge,
        delay: f64,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();
        let entity_name = entity_ref.into();
        let target_name = target_entity_ref.into();

        // Validate both entities exist
        let available: Vec<String> = self.entities.keys().cloned().collect();
        if !self.entities.contains_key(&entity_name) {
            return Err(ScenarioError::InvalidEntityRef {
                entity: entity_name.clone(),
                available: available.clone(),
            });
        }
        if !self.entities.contains_key(&target_name) {
            return Err(ScenarioError::InvalidEntityRef {
                entity: target_name.clone(),
                available,
            });
        }

        // Get story
        let story_keys: Vec<String> = self.storyboard.stories.keys().cloned().collect();
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available: story_keys,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg = act
            .maneuver_groups
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

        // Create the collision condition
        let collision_condition = CollisionCondition {
            target_entity_ref: target_name,
        };

        let entity_condition = EntityCondition::Collision(collision_condition);

        let triggering_entities = TriggeringEntities {
            rule: TriggeringEntitiesRule::Any,
            entity_refs: vec![entity_name],
        };

        let by_entity_condition = ByEntityCondition {
            triggering_entities,
            entity_condition,
        };

        let condition = Condition {
            name: format!("{}Condition", event_name),
            delay,
            condition_edge: edge,
            kind: ConditionKind::ByEntity(by_entity_condition),
        };

        let condition_group = ConditionGroup {
            conditions: vec![condition],
        };

        let trigger = Trigger {
            condition_groups: vec![condition_group],
        };

        // Create or update event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.start_trigger = Some(trigger);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![],
                start_trigger: Some(trigger),
            });
        }

        Ok(())
    }

    /// Adds a speed profile action to an event.
    ///
    /// Speed profile defines target speeds at specific time or distance points.
    /// The actor will follow the speed profile by adjusting its speed at each waypoint.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name of the event (created if it doesn't exist)
    /// * `waypoints` - Vector of (time_or_distance, speed) tuples (must have at least 1)
    /// * `following_mode` - If true, time-based; if false, distance-based
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_actor("s1", "a1", "mg1", "ego")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Time-based profile: (time, speed)
    /// let waypoints = vec![(0.0, 10.0), (5.0, 20.0), (10.0, 15.0)];
    /// scenario.add_speed_profile_action("s1", "a1", "mg1", "m1", "event1", 
    ///                                    waypoints, true)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If waypoints empty, speed < 0, or time/distance < 0
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent structures not found
    #[allow(clippy::too_many_arguments)]
    pub fn add_speed_profile_action(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        waypoints: Vec<(f64, f64)>,  // (time_or_distance, speed)
        following_mode: bool,  // true = time-based, false = distance-based
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();

        // Validate waypoints
        if waypoints.is_empty() {
            return Err(ScenarioError::InvalidValue {
                field: "waypoints".to_string(),
                reason: "Speed profile must have at least one waypoint".to_string(),
            });
        }

        // Validate each waypoint
        for (i, (position, speed)) in waypoints.iter().enumerate() {
            if *position < 0.0 {
                return Err(ScenarioError::InvalidValue {
                    field: format!("waypoint[{}].position", i),
                    reason: format!("Position cannot be negative (got {})", position),
                });
            }
            if *speed < 0.0 {
                return Err(ScenarioError::InvalidValue {
                    field: format!("waypoint[{}].speed", i),
                    reason: format!("Speed cannot be negative (got {})", speed),
                });
            }
        }

        // Get story
        let story_keys: Vec<String> = self.storyboard.stories.keys().cloned().collect();
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available: story_keys,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg = act
            .maneuver_groups
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

        // Create speed profile entries
        let entries: Vec<crate::storyboard::SpeedProfileEntry> = waypoints
            .into_iter()
            .map(|(position, speed)| crate::storyboard::SpeedProfileEntry { position, speed })
            .collect();

        let action = Action::SpeedProfile(crate::storyboard::SpeedProfileAction {
            entries,
            following_mode,
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

    /// Adds an event with a relative distance condition trigger.
    ///
    /// Creates an event that triggers when the distance between the specified
    /// entity and a reference entity meets the rule threshold. Distance can be
    /// measured in various ways (longitudinal, lateral, euclidean) and coordinate systems.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name for the new event
    /// * `entity_ref` - Name of the entity to monitor (must exist)
    /// * `reference_entity_ref` - Name of the reference entity (must exist)
    /// * `distance_value` - Distance threshold in meters (must be non-negative)
    /// * `rule` - Comparison rule (LessThan, GreaterThan, EqualTo)
    /// * `distance_type` - Type of distance measurement (Longitudinal, Lateral, Euclidean)
    /// * `freespace` - If true, use straight-line distance; if false, use road distance
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::{Rule, RelativeDistanceType};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp.clone())?;
    /// # scenario.add_vehicle("lead", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Trigger when longitudinal distance to lead vehicle < 10m
    /// scenario.add_event_with_relative_distance_condition(
    ///     "s1", "a1", "mg1", "m1", "distance_event", 
    ///     "ego", "lead", 10.0, Rule::LessThan, 
    ///     RelativeDistanceType::Longitudinal, false)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::EntityNotFound` - If entities don't exist or parent structures not found
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    #[allow(clippy::too_many_arguments)]
    pub fn add_event_with_relative_distance_condition(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        reference_entity_ref: impl Into<String>,
        distance_value: f64,
        rule: Rule,
        distance_type: crate::storyboard::RelativeDistanceType,
        freespace: bool,
    ) -> Result<()> {
        self.add_event_with_relative_distance_condition_advanced(
            story,
            act,
            mg,
            maneuver,
            event,
            entity_ref,
            reference_entity_ref,
            distance_value,
            rule,
            distance_type,
            freespace,
            ConditionEdge::None,
            0.0,
            crate::storyboard::CoordinateSystem::Entity,
        )
    }

    /// Adds an event with a relative distance condition trigger (advanced).
    ///
    /// Creates an event with full control over condition edge, delay, and coordinate system.
    /// This advanced version allows fine-grained control over when and how the condition triggers,
    /// and which coordinate system to use for distance measurement.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name for the new event
    /// * `entity_ref` - Name of the entity to monitor (must exist)
    /// * `reference_entity_ref` - Name of the reference entity (must exist)
    /// * `distance_value` - Distance threshold in meters (must be non-negative)
    /// * `rule` - Comparison rule (LessThan, GreaterThan, EqualTo)
    /// * `distance_type` - Type of distance measurement
    /// * `freespace` - If true, use straight-line distance; if false, use road distance
    /// * `edge` - Condition edge (Rising, Falling, RisingOrFalling, None)
    /// * `delay` - Delay in seconds after condition becomes true
    /// * `coordinate_system` - Coordinate system for distance measurement (Entity, Road, etc.)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::{Rule, RelativeDistanceType, ConditionEdge, CoordinateSystem};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp.clone())?;
    /// # scenario.add_vehicle("lead", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Trigger on rising edge when lateral distance > 2m, with 0.5s delay
    /// scenario.add_event_with_relative_distance_condition_advanced(
    ///     "s1", "a1", "mg1", "m1", "distance_event", "ego", "lead",
    ///     2.0, Rule::GreaterThan, RelativeDistanceType::Lateral, true,
    ///     ConditionEdge::Rising, 0.5, CoordinateSystem::Entity)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If distance_value < 0
    /// * `ScenarioError::InvalidEntityRef` - If entities don't exist
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent structures not found
    #[allow(clippy::too_many_arguments)]
    pub fn add_event_with_relative_distance_condition_advanced(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        reference_entity_ref: impl Into<String>,
        distance_value: f64,
        rule: Rule,
        distance_type: crate::storyboard::RelativeDistanceType,
        freespace: bool,
        edge: ConditionEdge,
        delay: f64,
        coordinate_system: crate::storyboard::CoordinateSystem,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();
        let entity_name = entity_ref.into();
        let reference_name = reference_entity_ref.into();

        // Validate distance value (must be non-negative)
        if distance_value < 0.0 {
            return Err(ScenarioError::InvalidValue {
                field: "distance_value".to_string(),
                reason: format!("Distance value cannot be negative (got {})", distance_value),
            });
        }

        // Validate both entities exist
        let available: Vec<String> = self.entities.keys().cloned().collect();
        if !self.entities.contains_key(&entity_name) {
            return Err(ScenarioError::InvalidEntityRef {
                entity: entity_name.clone(),
                available: available.clone(),
            });
        }
        if !self.entities.contains_key(&reference_name) {
            return Err(ScenarioError::InvalidEntityRef {
                entity: reference_name.clone(),
                available,
            });
        }

        // Get story
        let story_keys: Vec<String> = self.storyboard.stories.keys().cloned().collect();
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available: story_keys,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg = act
            .maneuver_groups
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

        // Create the relative distance condition
        let distance_condition = crate::storyboard::RelativeDistanceCondition {
            entity_ref: reference_name,
            value: distance_value,
            rule,
            distance_type,
            freespace,
            coordinate_system: Some(coordinate_system),
        };

        let entity_condition = EntityCondition::RelativeDistance(distance_condition);

        let triggering_entities = TriggeringEntities {
            rule: TriggeringEntitiesRule::Any,
            entity_refs: vec![entity_name],
        };

        let by_entity_condition = ByEntityCondition {
            triggering_entities,
            entity_condition,
        };

        let condition = Condition {
            name: format!("{}Condition", event_name),
            delay,
            condition_edge: edge,
            kind: ConditionKind::ByEntity(by_entity_condition),
        };

        let condition_group = ConditionGroup {
            conditions: vec![condition],
        };

        let trigger = Trigger {
            condition_groups: vec![condition_group],
        };

        // Create or update event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.start_trigger = Some(trigger);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![],
                start_trigger: Some(trigger),
            });
        }

        Ok(())
    }

    /// Adds an event with a time headway condition trigger.
    ///
    /// Creates an event that triggers when the time gap between the entity
    /// and a lead vehicle meets the rule threshold. Time headway is calculated as
    /// distance / follower_speed, representing the time to reach the lead vehicle.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name for the new event
    /// * `entity_ref` - Name of the following entity (must exist)
    /// * `lead_entity_ref` - Name of the lead entity (must exist)
    /// * `time_headway_value` - Time headway threshold in seconds (must be non-negative)
    /// * `rule` - Comparison rule (LessThan, GreaterThan, EqualTo)
    /// * `freespace` - If true, use straight-line distance; if false, use road distance
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::Rule;
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("follower", vp.clone())?;
    /// # scenario.add_vehicle("leader", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Trigger when time headway drops below 2 seconds
    /// scenario.add_event_with_time_headway_condition(
    ///     "s1", "a1", "mg1", "m1", "headway_event",
    ///     "follower", "leader", 2.0, Rule::LessThan, false)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If time_headway_value <= 0
    /// * `ScenarioError::InvalidEntityRef` - If entities don't exist
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent structures not found
    #[allow(clippy::too_many_arguments)]
    pub fn add_event_with_time_headway_condition(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        lead_entity_ref: impl Into<String>,
        time_headway_value: f64,
        rule: Rule,
        freespace: bool,
    ) -> Result<()> {
        self.add_event_with_time_headway_condition_advanced(
            story,
            act,
            mg,
            maneuver,
            event,
            entity_ref,
            lead_entity_ref,
            time_headway_value,
            rule,
            freespace,
            ConditionEdge::None,
            0.0,
        )
    }

    /// Adds an event with a time headway condition trigger (advanced).
    ///
    /// Creates an event with full control over condition edge and delay.
    /// Time headway represents the time it would take the following entity to reach
    /// the lead entity at its current speed.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name for the new event
    /// * `entity_ref` - Name of the following entity (must exist)
    /// * `lead_entity_ref` - Name of the lead entity (must exist)
    /// * `time_headway_value` - Time headway threshold in seconds (must be non-negative)
    /// * `rule` - Comparison rule (LessThan, GreaterThan, EqualTo)
    /// * `freespace` - If true, use straight-line distance; if false, use road distance
    /// * `edge` - Condition edge (Rising, Falling, RisingOrFalling, None)
    /// * `delay` - Delay in seconds after condition becomes true
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::{Rule, ConditionEdge};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("follower", vp.clone())?;
    /// # scenario.add_vehicle("leader", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Trigger on rising edge when headway < 2s, with 0.5s delay
    /// scenario.add_event_with_time_headway_condition_advanced(
    ///     "s1", "a1", "mg1", "m1", "headway_event", "follower", "leader",
    ///     2.0, Rule::LessThan, false, ConditionEdge::Rising, 0.5)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If time_headway_value < 0
    /// * `ScenarioError::InvalidEntityRef` - If entities don't exist
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent structures not found
    #[allow(clippy::too_many_arguments)]
    pub fn add_event_with_time_headway_condition_advanced(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        lead_entity_ref: impl Into<String>,
        time_headway_value: f64,
        rule: Rule,
        freespace: bool,
        edge: ConditionEdge,
        delay: f64,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();
        let entity_name = entity_ref.into();
        let lead_name = lead_entity_ref.into();

        // Validate time headway value (must be positive)
        if time_headway_value <= 0.0 {
            return Err(ScenarioError::InvalidValue {
                field: "time_headway_value".to_string(),
                reason: format!("Time headway value must be positive (got {})", time_headway_value),
            });
        }

        // Validate both entities exist
        let available: Vec<String> = self.entities.keys().cloned().collect();
        if !self.entities.contains_key(&entity_name) {
            return Err(ScenarioError::InvalidEntityRef {
                entity: entity_name.clone(),
                available: available.clone(),
            });
        }
        if !self.entities.contains_key(&lead_name) {
            return Err(ScenarioError::InvalidEntityRef {
                entity: lead_name.clone(),
                available,
            });
        }

        // Get story
        let story_keys: Vec<String> = self.storyboard.stories.keys().cloned().collect();
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available: story_keys,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg = act
            .maneuver_groups
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

        // Create the time headway condition
        let headway_condition = crate::storyboard::TimeHeadwayCondition {
            entity_ref: lead_name,
            value: time_headway_value,
            rule,
            freespace,
        };

        let entity_condition = EntityCondition::TimeHeadway(headway_condition);

        let triggering_entities = TriggeringEntities {
            rule: TriggeringEntitiesRule::Any,
            entity_refs: vec![entity_name],
        };

        let by_entity_condition = ByEntityCondition {
            triggering_entities,
            entity_condition,
        };

        let condition = Condition {
            name: format!("{}Condition", event_name),
            delay,
            condition_edge: edge,
            kind: ConditionKind::ByEntity(by_entity_condition),
        };

        let condition_group = ConditionGroup {
            conditions: vec![condition],
        };

        let trigger = Trigger {
            condition_groups: vec![condition_group],
        };

        // Create or update event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.start_trigger = Some(trigger);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![],
                start_trigger: Some(trigger),
            });
        }

        Ok(())
    }

    /// Adds an event with a standstill condition trigger.
    ///
    /// Creates an event that triggers when the entity has been stationary
    /// (velocity near zero) for at least the specified duration. Useful for
    /// detecting stopped vehicles or waiting scenarios.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name for the new event
    /// * `entity_ref` - Name of the entity to monitor (must exist)
    /// * `duration` - Required standstill duration in seconds (must be non-negative)
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Trigger when ego has been stopped for 5 seconds
    /// scenario.add_event_with_standstill_condition(
    ///     "s1", "a1", "mg1", "m1", "stopped_event", "ego", 5.0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::EntityNotFound` - If entity doesn't exist or parent structures not found
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    pub fn add_event_with_standstill_condition(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        duration: f64,
    ) -> Result<()> {
        self.add_event_with_standstill_condition_advanced(
            story,
            act,
            mg,
            maneuver,
            event,
            entity_ref,
            duration,
            ConditionEdge::None,
            0.0,
        )
    }

    /// Adds an event with a standstill condition trigger (advanced).
    ///
    /// Creates an event with full control over condition edge and delay.
    /// Allows specification of when exactly the trigger fires relative to
    /// the standstill condition being met.
    ///
    /// # Arguments
    /// * `story` - Name of the parent story
    /// * `act` - Name of the parent act
    /// * `mg` - Name of the maneuver group
    /// * `maneuver` - Name of the parent maneuver
    /// * `event` - Name for the new event
    /// * `entity_ref` - Name of the entity to monitor (must exist)
    /// * `duration` - Required standstill duration in seconds (must be non-negative)
    /// * `edge` - Condition edge (Rising, Falling, RisingOrFalling, None)
    /// * `delay` - Delay in seconds after condition becomes true
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(ScenarioError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use openscenario::{Scenario, OpenScenarioVersion};
    /// use openscenario::storyboard::ConditionEdge;
    /// # use openscenario::entities::{VehicleParams, VehicleCategory};
    ///
    /// # fn main() -> Result<(), openscenario::ScenarioError> {
    /// # let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    /// # let vp = VehicleParams { catalog: None, vehicle_category: VehicleCategory::Car, properties: None };
    /// # scenario.add_vehicle("ego", vp)?;
    /// # scenario.add_story("s1")?;
    /// # scenario.add_act("s1", "a1")?;
    /// # scenario.add_maneuver_group("s1", "a1", "mg1")?;
    /// # scenario.add_maneuver("s1", "a1", "mg1", "m1")?;
    /// 
    /// // Trigger on rising edge when stopped for 5s, with 1s delay
    /// scenario.add_event_with_standstill_condition_advanced(
    ///     "s1", "a1", "mg1", "m1", "stopped_event", "ego",
    ///     5.0, ConditionEdge::Rising, 1.0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `ScenarioError::InvalidValue` - If duration < 0
    /// * `ScenarioError::InvalidEntityRef` - If entity doesn't exist
    /// * `ScenarioError::StoryNotFound` - If parent story doesn't exist
    /// * `ScenarioError::EntityNotFound` - If parent structures not found
    #[allow(clippy::too_many_arguments)]
    pub fn add_event_with_standstill_condition_advanced(
        &mut self,
        story: impl Into<String>,
        act: impl Into<String>,
        mg: impl Into<String>,
        maneuver: impl Into<String>,
        event: impl Into<String>,
        entity_ref: impl Into<String>,
        duration: f64,
        edge: ConditionEdge,
        delay: f64,
    ) -> Result<()> {
        let story_name = story.into();
        let act_name = act.into();
        let mg_name = mg.into();
        let maneuver_name = maneuver.into();
        let event_name = event.into();
        let entity_name = entity_ref.into();

        // Validate duration (must be positive)
        if duration <= 0.0 {
            return Err(ScenarioError::InvalidValue {
                field: "duration".to_string(),
                reason: format!("Duration must be positive (got {})", duration),
            });
        }

        // Validate entity exists
        let available: Vec<String> = self.entities.keys().cloned().collect();
        if !self.entities.contains_key(&entity_name) {
            return Err(ScenarioError::InvalidEntityRef {
                entity: entity_name.clone(),
                available,
            });
        }

        // Get story
        let story_keys: Vec<String> = self.storyboard.stories.keys().cloned().collect();
        let story = self
            .storyboard
            .stories
            .get_mut(&story_name)
            .ok_or_else(|| ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available: story_keys,
            })?;

        let act = story
            .acts
            .get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;

        let mg = act
            .maneuver_groups
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

        // Create the stand still condition
        let standstill_condition = crate::storyboard::StandStillCondition { duration };

        let entity_condition = EntityCondition::StandStill(standstill_condition);

        let triggering_entities = TriggeringEntities {
            rule: TriggeringEntitiesRule::Any,
            entity_refs: vec![entity_name],
        };

        let by_entity_condition = ByEntityCondition {
            triggering_entities,
            entity_condition,
        };

        let condition = Condition {
            name: format!("{}Condition", event_name),
            delay,
            condition_edge: edge,
            kind: ConditionKind::ByEntity(by_entity_condition),
        };

        let condition_group = ConditionGroup {
            conditions: vec![condition],
        };

        let trigger = Trigger {
            condition_groups: vec![condition_group],
        };

        // Create or update event
        if let Some(event) = maneuver.events.iter_mut().find(|e| e.name == event_name) {
            event.start_trigger = Some(trigger);
        } else {
            maneuver.events.push(Event {
                name: event_name,
                actions: vec![],
                start_trigger: Some(trigger),
            });
        }

        Ok(())
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
