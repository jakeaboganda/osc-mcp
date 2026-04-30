use std::collections::HashMap;
use crate::{Result, ScenarioError, OpenScenarioVersion};
use crate::entities::{Entity, Vehicle, VehicleParams, Pedestrian, PedestrianParams, MiscObject, MiscObjectParams};
use crate::Position;

pub struct Scenario {
    version: OpenScenarioVersion,
    entities: HashMap<String, Entity>,
    initial_positions: HashMap<String, Position>,
}

impl Scenario {
    pub fn new(version: OpenScenarioVersion) -> Self {
        Self {
            version,
            entities: HashMap::new(),
            initial_positions: HashMap::new(),
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
}
