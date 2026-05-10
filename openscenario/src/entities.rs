use serde::{Deserialize, Serialize};

/// Reference to an entity definition in an external catalog.
///
/// Catalogs allow reuse of entity definitions across scenarios. A catalog reference
/// specifies the path to the catalog file and the name of the entry within that catalog.
///
/// # Examples
/// ```
/// use openscenario::entities::CatalogReference;
///
/// # fn main() {
/// let catalog_ref = CatalogReference {
///     path: "VehicleCatalog.xosc".to_string(),
///     entry_name: "sedan_default".to_string(),
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogReference {
    pub path: String,
    pub entry_name: String,
}

/// OpenSCENARIO vehicle category classification.
///
/// Defines the type of vehicle entity according to the OpenSCENARIO standard.
/// Used for behavior simulation, collision detection, and visualization.
///
/// # Examples
/// ```
/// use openscenario::entities::VehicleCategory;
///
/// # fn main() {
/// let car = VehicleCategory::Car;
/// let truck = VehicleCategory::Truck;
/// let bike = VehicleCategory::Motorbike;
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VehicleCategory {
    /// Passenger car
    Car,
    /// Heavy truck
    Truck,
    /// Public transit bus
    Bus,
    /// Trailer or semi-trailer
    Trailer,
    /// Delivery van or minivan
    Van,
    /// Motorcycle or scooter
    Motorbike,
    /// Bicycle
    Bicycle,
}

/// Physical properties of a vehicle.
///
/// Defines optional physical characteristics like mass and 3D model reference.
/// These properties affect simulation physics and visualization.
///
/// # Examples
/// ```
/// use openscenario::entities::VehicleProperties;
///
/// # fn main() {
/// let props = VehicleProperties {
///     mass: Some(1500.0),  // 1500 kg
///     model3d: Some("sedan.fbx".to_string()),
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleProperties {
    pub mass: Option<f64>,
    pub model3d: Option<String>,
}

/// Parameters defining a vehicle entity.
///
/// Contains the vehicle category, optional catalog reference, and optional properties.
/// Can reference a catalog entry for common vehicle types or define custom properties.
///
/// # Examples
/// ```
/// use openscenario::entities::{VehicleParams, VehicleCategory, VehicleProperties};
///
/// # fn main() {
/// let params = VehicleParams {
///     catalog: None,
///     vehicle_category: VehicleCategory::Car,
///     properties: Some(VehicleProperties {
///         mass: Some(1400.0),
///         model3d: None,
///     }),
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleParams {
    pub catalog: Option<CatalogReference>,
    pub vehicle_category: VehicleCategory,
    pub properties: Option<VehicleProperties>,
}

/// A vehicle entity in the scenario.
///
/// Represents a motorized or non-motorized vehicle that participates in the scenario.
/// Each vehicle has a unique name and parameters defining its characteristics.
///
/// # Examples
/// ```
/// use openscenario::entities::{Vehicle, VehicleParams, VehicleCategory};
///
/// # fn main() {
/// let vehicle = Vehicle {
///     name: "Ego".to_string(),
///     params: VehicleParams {
///         catalog: None,
///         vehicle_category: VehicleCategory::Car,
///         properties: None,
///     },
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub name: String,
    pub params: VehicleParams,
}

/// Parameters defining a pedestrian entity.
///
/// Contains optional catalog reference, model name, and mass.
/// Pedestrians have simpler properties than vehicles.
///
/// # Examples
/// ```
/// use openscenario::entities::PedestrianParams;
///
/// # fn main() {
/// let params = PedestrianParams {
///     catalog: None,
///     model: Some("adult_male".to_string()),
///     mass: Some(75.0),  // 75 kg
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PedestrianParams {
    pub catalog: Option<CatalogReference>,
    pub model: Option<String>,
    pub mass: Option<f64>,
}

/// A pedestrian entity in the scenario.
///
/// Represents a walking person that participates in the scenario.
/// Each pedestrian has a unique name and parameters defining their characteristics.
///
/// # Examples
/// ```
/// use openscenario::entities::{Pedestrian, PedestrianParams};
///
/// # fn main() {
/// let pedestrian = Pedestrian {
///     name: "Ped1".to_string(),
///     params: PedestrianParams {
///         catalog: None,
///         model: Some("adult".to_string()),
///         mass: Some(70.0),
///     },
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pedestrian {
    pub name: String,
    pub params: PedestrianParams,
}

/// Parameters defining a miscellaneous object entity.
///
/// Contains optional catalog reference, category, and mass for objects
/// that are neither vehicles nor pedestrians (barriers, signs, props, etc.).
///
/// # Examples
/// ```
/// use openscenario::entities::MiscObjectParams;
///
/// # fn main() {
/// let params = MiscObjectParams {
///     catalog: None,
///     category: Some("barrier".to_string()),
///     mass: Some(500.0),  // 500 kg
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscObjectParams {
    pub catalog: Option<CatalogReference>,
    pub category: Option<String>,
    pub mass: Option<f64>,
}

/// A miscellaneous object entity in the scenario.
///
/// Represents non-vehicle, non-pedestrian objects like barriers, traffic signs,
/// obstacles, or environmental props. Each object has a unique name and parameters.
///
/// # Examples
/// ```
/// use openscenario::entities::{MiscObject, MiscObjectParams};
///
/// # fn main() {
/// let misc = MiscObject {
///     name: "Barrier1".to_string(),
///     params: MiscObjectParams {
///         catalog: None,
///         category: Some("barrier".to_string()),
///         mass: Some(300.0),
///     },
/// };
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscObject {
    pub name: String,
    pub params: MiscObjectParams,
}

/// A scenario entity that can be a vehicle, pedestrian, or miscellaneous object.
///
/// The Entity enum provides a unified type for all scenario participants.
/// Each variant contains the specific entity type with its properties.
///
/// # Choosing the Right Entity Type
/// - **Vehicle**: Use for motorized vehicles (cars, trucks, buses, motorcycles)
/// - **Pedestrian**: Use for humans walking, running, or stationary
/// - **MiscObject**: Use for non-motorized objects (traffic cones, barriers, obstacles, debris)
///
/// # Examples
/// ```
/// use openscenario::entities::{Entity, Vehicle, VehicleParams, VehicleCategory};
///
/// # fn main() {
/// let vehicle = Vehicle {
///     name: "Car1".to_string(),
///     params: VehicleParams {
///         catalog: None,
///         vehicle_category: VehicleCategory::Car,
///         properties: None,
///     },
/// };
/// let entity = Entity::Vehicle(vehicle);
/// assert_eq!(entity.name(), "Car1");
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Entity {
    /// Vehicle entity
    Vehicle(Vehicle),
    /// Pedestrian entity
    Pedestrian(Pedestrian),
    /// Miscellaneous object entity
    MiscObject(MiscObject),
}

impl Entity {
    /// Returns the name of the entity.
    ///
    /// Provides a unified way to access the entity name regardless of type.
    ///
    /// # Examples
    /// ```
    /// use openscenario::entities::{Entity, Vehicle, VehicleParams, VehicleCategory};
    ///
    /// # fn main() {
    /// let vehicle = Vehicle {
    ///     name: "Ego".to_string(),
    ///     params: VehicleParams {
    ///         catalog: None,
    ///         vehicle_category: VehicleCategory::Car,
    ///         properties: None,
    ///     },
    /// };
    /// let entity = Entity::Vehicle(vehicle);
    /// assert_eq!(entity.name(), "Ego");
    /// # }
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Entity::Vehicle(v) => &v.name,
            Entity::Pedestrian(p) => &p.name,
            Entity::MiscObject(m) => &m.name,
        }
    }
}
