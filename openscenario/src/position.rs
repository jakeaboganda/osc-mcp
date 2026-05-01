use serde::{Serialize, Deserialize};
use crate::opendrive_validator::{OpenDriveValidator, ValidationError};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orientation {
    pub h: f64,  // heading
    pub p: f64,  // pitch
    pub r: f64,  // roll
}

impl Default for Orientation {
    fn default() -> Self {
        Self { h: 0.0, p: 0.0, r: 0.0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Position {
    World {
        x: f64,
        y: f64,
        z: f64,
        h: f64,
        p: f64,
        r: f64,
    },
    Lane {
        road_id: String,
        lane_id: i32,
        s: f64,
        offset: f64,
        orientation: Option<Orientation>,
    },
    Road {
        road_id: String,
        s: f64,
        t: f64,
        orientation: Option<Orientation>,
    },
    RelativeWorld {
        entity: String,
        dx: f64,
        dy: f64,
        dz: f64,
        orientation: Orientation,
    },
    RelativeObject {
        entity: String,
        dx: f64,
        dy: f64,
        dz: f64,
        orientation: Orientation,
    },
    RelativeLane {
        entity: String,
        ds: f64,
        d_lane: i32,
        offset: f64,
        orientation: Option<Orientation>,
    },
    RelativeRoad {
        entity: String,
        ds: f64,
        dt: f64,
        orientation: Option<Orientation>,
    },
}

impl Position {
    pub fn world(x: f64, y: f64, z: f64, h: f64) -> Self {
        Self::World { x, y, z, h, p: 0.0, r: 0.0 }
    }
    
    pub fn lane(road_id: impl Into<String>, lane_id: i32, s: f64, offset: f64, orientation: Option<Orientation>) -> Self {
        Self::Lane {
            road_id: road_id.into(),
            lane_id,
            s,
            offset,
            orientation,
        }
    }
    
    pub fn road(road_id: impl Into<String>, s: f64, t: f64, orientation: Option<Orientation>) -> Self {
        Self::Road {
            road_id: road_id.into(),
            s,
            t,
            orientation,
        }
    }
    
    pub fn relative_world(entity: impl Into<String>, dx: f64, dy: f64, dz: f64, orientation: Orientation) -> Self {
        Self::RelativeWorld {
            entity: entity.into(),
            dx,
            dy,
            dz,
            orientation,
        }
    }
    
    pub fn relative_object(entity: impl Into<String>, dx: f64, dy: f64, dz: f64, orientation: Orientation) -> Self {
        Self::RelativeObject {
            entity: entity.into(),
            dx,
            dy,
            dz,
            orientation,
        }
    }
    
    pub fn relative_lane(entity: impl Into<String>, ds: f64, d_lane: i32, offset: f64, orientation: Option<Orientation>) -> Self {
        Self::RelativeLane {
            entity: entity.into(),
            ds,
            d_lane,
            offset,
            orientation,
        }
    }
    
    pub fn relative_road(entity: impl Into<String>, ds: f64, dt: f64, orientation: Option<Orientation>) -> Self {
        Self::RelativeRoad {
            entity: entity.into(),
            ds,
            dt,
            orientation,
        }
    }
    
    /// Get entity reference if this is a relative position
    pub fn referenced_entity(&self) -> Option<&str> {
        match self {
            Self::RelativeWorld { entity, .. } |
            Self::RelativeObject { entity, .. } |
            Self::RelativeLane { entity, .. } |
            Self::RelativeRoad { entity, .. } => Some(entity),
            _ => None,
        }
    }
    
    /// Validate this position against an OpenDRIVE road network
    pub fn validate(&self, validator: &OpenDriveValidator) -> Result<(), ValidationError> {
        match self {
            Self::Lane { road_id, lane_id, s, .. } => {
                validator.validate_lane_position(road_id, *lane_id)?;
                validator.validate_road_position(road_id, *s)?;
                Ok(())
            }
            Self::Road { road_id, s, .. } => {
                validator.validate_road_position(road_id, *s)?;
                Ok(())
            }
            // World and relative positions don't have direct OpenDRIVE validation
            _ => Ok(()),
        }
    }
}
