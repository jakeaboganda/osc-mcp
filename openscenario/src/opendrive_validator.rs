use opendrive::core::OpenDrive;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Failed to load OpenDRIVE file: {0}")]
    LoadError(String),
    
    #[error("Road '{0}' not found in OpenDRIVE file")]
    RoadNotFound(String),
    
    #[error("Lane {1} not found in road '{0}'")]
    LaneNotFound(String, i32),
    
    #[error("Position {1} is out of bounds for road '{0}' (length: {2})")]
    PositionOutOfBounds(String, f64, f64),
}

pub struct OpenDriveValidator {
    #[allow(dead_code)]
    opendrive: OpenDrive,
    road_cache: HashMap<String, RoadInfo>,
}

struct RoadInfo {
    length: f64,
    lane_ids: Vec<i32>,
}

impl OpenDriveValidator {
    /// Load an OpenDRIVE file and create a validator
    pub fn load(path: &Path) -> Result<Self, ValidationError> {
        let file = std::fs::File::open(path).map_err(|e| {
            ValidationError::LoadError(format!("Failed to open file: {}", e))
        })?;
        
        let opendrive = OpenDrive::from_xml_read(file).map_err(|e| {
            ValidationError::LoadError(format!("Failed to parse OpenDRIVE XML: {}", e))
        })?;
        
        let mut road_cache = HashMap::new();
        for road in &opendrive.road {
            let mut lane_ids = Vec::new();
            
            // Collect all lane IDs from all lane sections
            for lane_section in road.lanes.lane_section.iter() {
                // Add left lanes
                if let Some(left) = &lane_section.left {
                    for lane in &left.lane {
                        // Cast i64 to i32 (OpenDRIVE spec keeps lane IDs in i32 range)
                        let lane_id = lane.id as i32;
                        if !lane_ids.contains(&lane_id) {
                            lane_ids.push(lane_id);
                        }
                    }
                }
                
                // Add center lanes
                for lane in &lane_section.center.lane {
                    let lane_id = lane.id as i32;
                    if !lane_ids.contains(&lane_id) {
                        lane_ids.push(lane_id);
                    }
                }
                
                // Add right lanes
                if let Some(right) = &lane_section.right {
                    for lane in &right.lane {
                        let lane_id = lane.id as i32;
                        if !lane_ids.contains(&lane_id) {
                            lane_ids.push(lane_id);
                        }
                    }
                }
            }
            
            road_cache.insert(
                road.id.clone(),
                RoadInfo {
                    length: road.length.value,
                    lane_ids,
                },
            );
        }
        
        Ok(Self {
            opendrive,
            road_cache,
        })
    }
    
    /// Check if a road exists in the OpenDRIVE file
    pub fn road_exists(&self, road_id: &str) -> bool {
        self.road_cache.contains_key(road_id)
    }
    
    /// Validate a position against the OpenDRIVE road network
    pub fn validate_position(&self, road_id: &str, s: f64) -> Result<(), ValidationError> {
        self.validate_road_position(road_id, s)
    }
    
    /// Validate a lane position (road + lane ID)
    pub fn validate_lane_position(&self, road_id: &str, lane_id: i32) -> Result<(), ValidationError> {
        let road_info = self
            .road_cache
            .get(road_id)
            .ok_or_else(|| ValidationError::RoadNotFound(road_id.to_string()))?;
        
        if !road_info.lane_ids.contains(&lane_id) {
            return Err(ValidationError::LaneNotFound(road_id.to_string(), lane_id));
        }
        
        Ok(())
    }
    
    /// Validate a road position (road + s-coordinate)
    pub fn validate_road_position(&self, road_id: &str, s: f64) -> Result<(), ValidationError> {
        // Check for NaN and infinity
        if !s.is_finite() {
            return Err(ValidationError::PositionOutOfBounds(
                road_id.to_string(),
                s,
                0.0, // length is irrelevant for NaN/infinity
            ));
        }
        
        let road_info = self
            .road_cache
            .get(road_id)
            .ok_or_else(|| ValidationError::RoadNotFound(road_id.to_string()))?;
        
        if s < 0.0 || s > road_info.length {
            return Err(ValidationError::PositionOutOfBounds(
                road_id.to_string(),
                s,
                road_info.length,
            ));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_validator_creation() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/test_road.xodr");
        
        let result = OpenDriveValidator::load(&path);
        assert!(result.is_ok());
    }
}
