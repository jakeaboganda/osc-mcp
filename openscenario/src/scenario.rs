use std::collections::HashMap;
use crate::{Result, ScenarioError, OpenScenarioVersion};
use crate::entities::{Entity, Vehicle, VehicleParams, Pedestrian, PedestrianParams, MiscObject, MiscObjectParams};
use crate::Position;
use crate::storyboard::{Storyboard, Story, Act, ManeuverGroup};

pub struct Scenario {
    version: OpenScenarioVersion,
    entities: HashMap<String, Entity>,
    initial_positions: HashMap<String, Position>,
    storyboard: Storyboard,
}

impl Scenario {
    pub fn new(version: OpenScenarioVersion) -> Self {
        Self {
            version,
            entities: HashMap::new(),
            initial_positions: HashMap::new(),
            storyboard: Storyboard::new(),
        }
    }
    
    pub fn version(&self) -> OpenScenarioVersion {
        self.version
    }
    
    pub fn add_vehicle(&mut self, name: impl Into<String>, params: VehicleParams) -> Result<()> {
        let name = name.into();
        
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
    
    pub fn add_pedestrian(&mut self, name: impl Into<String>, params: PedestrianParams) -> Result<()> {
        let name = name.into();
        
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
    
    pub fn add_misc_object(&mut self, name: impl Into<String>, params: MiscObjectParams) -> Result<()> {
        let name = name.into();
        
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
    
    pub fn set_initial_position(&mut self, entity: impl Into<String>, position: Position) -> Result<()> {
        let entity = entity.into();
        
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
        
        if self.storyboard.stories.contains_key(&name) {
            return Err(ScenarioError::StoryNotFound {
                name: name.clone(),
                available: self.storyboard.stories.keys().cloned().collect(),
            });
        }
        
        self.storyboard.stories.insert(name.clone(), Story::new(name));
        Ok(())
    }
    
    pub fn add_act(&mut self, story: impl Into<String>, name: impl Into<String>) -> Result<()> {
        let story_name = story.into();
        let act_name = name.into();
        
        // Check if story exists first
        if !self.storyboard.stories.contains_key(&story_name) {
            return Err(ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available: self.storyboard.stories.keys().cloned().collect(),
            });
        }
        
        let story = self.storyboard.stories.get_mut(&story_name).unwrap();
        story.acts.insert(act_name.clone(), Act::new(act_name));
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
        
        // Check if story exists first
        if !self.storyboard.stories.contains_key(&story_name) {
            return Err(ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available: self.storyboard.stories.keys().cloned().collect(),
            });
        }
        
        let story = self.storyboard.stories.get_mut(&story_name).unwrap();
        
        let act = story.acts.get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;
        
        act.maneuver_groups.insert(mg_name.clone(), ManeuverGroup::new(mg_name));
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
        
        // Validate entity exists
        if !self.entities.contains_key(&entity_name) {
            return Err(ScenarioError::EntityNotFound {
                entity: entity_name,
                context: "add_actor".to_string(),
            });
        }
        
        // Check if story exists first
        if !self.storyboard.stories.contains_key(&story_name) {
            return Err(ScenarioError::StoryNotFound {
                name: story_name.clone(),
                available: self.storyboard.stories.keys().cloned().collect(),
            });
        }
        
        let story = self.storyboard.stories.get_mut(&story_name).unwrap();
        
        let act = story.acts.get_mut(&act_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: act_name.clone(),
                context: format!("Act in story '{}'", story_name),
            })?;
        
        let mg = act.maneuver_groups.get_mut(&mg_name)
            .ok_or_else(|| ScenarioError::EntityNotFound {
                entity: mg_name.clone(),
                context: format!("ManeuverGroup in act '{}'", act_name),
            })?;
        
        mg.actors.push(entity_name);
        Ok(())
    }
}
