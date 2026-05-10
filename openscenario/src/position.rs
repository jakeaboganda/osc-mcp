use crate::opendrive_validator::{OpenDriveValidator, ValidationError};
use serde::{Deserialize, Serialize};

/// Orientation in 3D space using heading, pitch, and roll angles.
///
/// Defines the rotation of an entity or position using Euler angles.
/// All angles are in radians.
///
/// # Examples
/// ```
/// use openscenario::position::Orientation;
///
/// # fn main() {
/// let orientation = Orientation {
///     h: 1.57,  // 90 degrees heading (facing east)
///     p: 0.0,   // No pitch
///     r: 0.0,   // No roll
/// };
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orientation {
    /// Heading angle (rotation around z-axis) in radians
    pub h: f64, // heading
    /// Pitch angle (rotation around y-axis) in radians
    pub p: f64, // pitch
    /// Roll angle (rotation around x-axis) in radians
    pub r: f64, // roll
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            h: 0.0,
            p: 0.0,
            r: 0.0,
        }
    }
}

/// A position specification in various coordinate systems.
///
/// Position can be specified in multiple coordinate systems: absolute world coordinates,
/// road/lane coordinates (for OpenDRIVE compatibility), or relative to another entity.
/// Each variant contains the parameters needed for that coordinate system.
///
/// # Examples
/// ```
/// use openscenario::position::Position;
///
/// # fn main() {
/// // Absolute world position
/// let world_pos = Position::world(100.0, 50.0, 0.0, 0.0);
///
/// // Lane-based position
/// let lane_pos = Position::lane("road1", -1, 20.0, 0.0, None);
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Position {
    /// Absolute position in world coordinates (Cartesian)
    World {
        x: f64,
        y: f64,
        z: f64,
        h: f64,
        p: f64,
        r: f64,
    },
    /// Position in lane coordinates (OpenDRIVE)
    Lane {
        road_id: String,
        lane_id: i32,
        s: f64,
        offset: f64,
        orientation: Option<Orientation>,
    },
    /// Position in road coordinates (OpenDRIVE)
    Road {
        road_id: String,
        s: f64,
        t: f64,
        orientation: Option<Orientation>,
    },
    /// Position relative to another entity in world coordinates
    RelativeWorld {
        entity: String,
        dx: f64,
        dy: f64,
        dz: f64,
        orientation: Orientation,
    },
    /// Position relative to another entity in object coordinates
    RelativeObject {
        entity: String,
        dx: f64,
        dy: f64,
        dz: f64,
        orientation: Orientation,
    },
    /// Position relative to another entity in lane coordinates
    RelativeLane {
        entity: String,
        ds: f64,
        d_lane: i32,
        offset: f64,
        orientation: Option<Orientation>,
    },
    /// Position relative to another entity in road coordinates
    RelativeRoad {
        entity: String,
        ds: f64,
        dt: f64,
        orientation: Option<Orientation>,
    },
}

impl Position {
    /// Creates a position in world coordinates.
    ///
    /// Convenience constructor for world positions with zero pitch and roll.
    ///
    /// # Arguments
    /// * `x` - X coordinate (meters)
    /// * `y` - Y coordinate (meters)
    /// * `z` - Z coordinate (meters, altitude)
    /// * `h` - Heading angle (radians)
    ///
    /// # Examples
    /// ```
    /// use openscenario::position::Position;
    ///
    /// # fn main() {
    /// let pos = Position::world(100.0, 50.0, 0.0, 1.57);
    /// # }
    /// ```
    pub fn world(x: f64, y: f64, z: f64, h: f64) -> Self {
        Self::World {
            x,
            y,
            z,
            h,
            p: 0.0,
            r: 0.0,
        }
    }

    /// Creates a position in lane coordinates.
    ///
    /// Lane coordinates are defined in the OpenDRIVE standard.
    ///
    /// # Arguments
    /// * `road_id` - OpenDRIVE road identifier
    /// * `lane_id` - Lane ID (negative for right lanes, positive for left)
    /// * `s` - Position along the road (meters)
    /// * `offset` - Lateral offset from lane center (meters)
    /// * `orientation` - Optional orientation override
    ///
    /// # Examples
    /// ```
    /// use openscenario::position::Position;
    ///
    /// # fn main() {
    /// let pos = Position::lane("road1", -1, 50.0, 0.5, None);
    /// # }
    /// ```
    pub fn lane(
        road_id: impl Into<String>,
        lane_id: i32,
        s: f64,
        offset: f64,
        orientation: Option<Orientation>,
    ) -> Self {
        Self::Lane {
            road_id: road_id.into(),
            lane_id,
            s,
            offset,
            orientation,
        }
    }

    /// Creates a position in road coordinates.
    ///
    /// Road coordinates use the s-t coordinate system from OpenDRIVE.
    ///
    /// # Arguments
    /// * `road_id` - OpenDRIVE road identifier
    /// * `s` - Position along the road reference line (meters)
    /// * `t` - Lateral offset from the reference line (meters)
    /// * `orientation` - Optional orientation override
    ///
    /// # Examples
    /// ```
    /// use openscenario::position::Position;
    ///
    /// # fn main() {
    /// let pos = Position::road("highway1", 100.0, 2.0, None);
    /// # }
    /// ```
    pub fn road(
        road_id: impl Into<String>,
        s: f64,
        t: f64,
        orientation: Option<Orientation>,
    ) -> Self {
        Self::Road {
            road_id: road_id.into(),
            s,
            t,
            orientation,
        }
    }

    /// Creates a position relative to another entity in world coordinates.
    ///
    /// Offsets are applied in the world coordinate system.
    ///
    /// # Arguments
    /// * `entity` - Name of the reference entity
    /// * `dx` - X offset (meters)
    /// * `dy` - Y offset (meters)
    /// * `dz` - Z offset (meters)
    /// * `orientation` - Orientation
    ///
    /// # Examples
    /// ```
    /// use openscenario::position::{Position, Orientation};
    ///
    /// # fn main() {
    /// let pos = Position::relative_world(
    ///     "LeadVehicle",
    ///     -10.0,  // 10m behind
    ///     0.0,
    ///     0.0,
    ///     Orientation::default()
    /// );
    /// # }
    /// ```
    pub fn relative_world(
        entity: impl Into<String>,
        dx: f64,
        dy: f64,
        dz: f64,
        orientation: Orientation,
    ) -> Self {
        Self::RelativeWorld {
            entity: entity.into(),
            dx,
            dy,
            dz,
            orientation,
        }
    }

    /// Creates a position relative to another entity in object coordinates.
    ///
    /// Offsets are applied in the reference entity's local coordinate system.
    ///
    /// # Arguments
    /// * `entity` - Name of the reference entity
    /// * `dx` - Longitudinal offset (meters, forward is positive)
    /// * `dy` - Lateral offset (meters, right is positive)
    /// * `dz` - Vertical offset (meters, up is positive)
    /// * `orientation` - Orientation
    ///
    /// # Examples
    /// ```
    /// use openscenario::position::{Position, Orientation};
    ///
    /// # fn main() {
    /// let pos = Position::relative_object(
    ///     "Ego",
    ///     5.0,   // 5m in front
    ///     -2.0,  // 2m to the left
    ///     0.0,
    ///     Orientation::default()
    /// );
    /// # }
    /// ```
    pub fn relative_object(
        entity: impl Into<String>,
        dx: f64,
        dy: f64,
        dz: f64,
        orientation: Orientation,
    ) -> Self {
        Self::RelativeObject {
            entity: entity.into(),
            dx,
            dy,
            dz,
            orientation,
        }
    }

    /// Creates a position relative to another entity in lane coordinates.
    ///
    /// Uses lane-relative offsets from the reference entity's position.
    ///
    /// # Arguments
    /// * `entity` - Name of the reference entity
    /// * `ds` - Longitudinal offset along the road (meters)
    /// * `d_lane` - Lane offset (number of lanes, negative=right, positive=left)
    /// * `offset` - Lateral offset within the lane (meters)
    /// * `orientation` - Optional orientation override
    ///
    /// # Examples
    /// ```
    /// use openscenario::position::Position;
    ///
    /// # fn main() {
    /// let pos = Position::relative_lane("Target", 20.0, -1, 0.0, None);
    /// # }
    /// ```
    pub fn relative_lane(
        entity: impl Into<String>,
        ds: f64,
        d_lane: i32,
        offset: f64,
        orientation: Option<Orientation>,
    ) -> Self {
        Self::RelativeLane {
            entity: entity.into(),
            ds,
            d_lane,
            offset,
            orientation,
        }
    }

    /// Creates a position relative to another entity in road coordinates.
    ///
    /// Uses road-relative offsets from the reference entity's position.
    ///
    /// # Arguments
    /// * `entity` - Name of the reference entity
    /// * `ds` - Longitudinal offset along the road reference line (meters)
    /// * `dt` - Lateral offset from the reference line (meters)
    /// * `orientation` - Optional orientation override
    ///
    /// # Examples
    /// ```
    /// use openscenario::position::Position;
    ///
    /// # fn main() {
    /// let pos = Position::relative_road("Leader", 15.0, 1.5, None);
    /// # }
    /// ```
    pub fn relative_road(
        entity: impl Into<String>,
        ds: f64,
        dt: f64,
        orientation: Option<Orientation>,
    ) -> Self {
        Self::RelativeRoad {
            entity: entity.into(),
            ds,
            dt,
            orientation,
        }
    }

    /// Returns the referenced entity name if this is a relative position.
    ///
    /// Returns Some(entity_name) for relative positions, None for absolute positions.
    ///
    /// # Examples
    /// ```
    /// use openscenario::position::{Position, Orientation};
    ///
    /// # fn main() {
    /// let rel_pos = Position::relative_world("Target", 0.0, 0.0, 0.0, Orientation::default());
    /// assert_eq!(rel_pos.referenced_entity(), Some("Target"));
    ///
    /// let abs_pos = Position::world(0.0, 0.0, 0.0, 0.0);
    /// assert_eq!(abs_pos.referenced_entity(), None);
    /// # }
    /// ```
    pub fn referenced_entity(&self) -> Option<&str> {
        match self {
            Self::RelativeWorld { entity, .. }
            | Self::RelativeObject { entity, .. }
            | Self::RelativeLane { entity, .. }
            | Self::RelativeRoad { entity, .. } => Some(entity),
            _ => None,
        }
    }

    /// Validates this position against an OpenDRIVE road network.
    ///
    /// Checks that lane and road positions reference valid roads and lanes
    /// in the OpenDRIVE network. World and relative positions always pass validation.
    ///
    /// # Arguments
    /// * `validator` - The OpenDRIVE validator containing road network data
    ///
    /// # Returns
    /// * `Ok(())` if the position is valid
    /// * `Err(ValidationError)` if validation fails
    ///
    /// # Examples
    /// ```no_run
    /// use openscenario::position::Position;
    /// use openscenario::opendrive_validator::OpenDriveValidator;
    /// use std::path::Path;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let validator = OpenDriveValidator::load(Path::new("map.xodr"))?;
    /// let pos = Position::lane("road1", -1, 50.0, 0.0, None);
    /// pos.validate(&validator)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn validate(&self, validator: &OpenDriveValidator) -> Result<(), ValidationError> {
        match self {
            Self::Lane {
                road_id,
                lane_id,
                s,
                ..
            } => {
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
