use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogReference {
    pub path: String,
    pub entry_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VehicleCategory {
    Car,
    Truck,
    Bus,
    Trailer,
    Van,
    Motorbike,
    Bicycle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleProperties {
    pub mass: Option<f64>,
    pub model3d: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleParams {
    pub catalog: Option<CatalogReference>,
    pub vehicle_category: VehicleCategory,
    pub properties: Option<VehicleProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub name: String,
    pub params: VehicleParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PedestrianParams {
    pub catalog: Option<CatalogReference>,
    pub model: Option<String>,
    pub mass: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pedestrian {
    pub name: String,
    pub params: PedestrianParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscObjectParams {
    pub catalog: Option<CatalogReference>,
    pub category: Option<String>,
    pub mass: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscObject {
    pub name: String,
    pub params: MiscObjectParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Entity {
    Vehicle(Vehicle),
    Pedestrian(Pedestrian),
    MiscObject(MiscObject),
}

impl Entity {
    pub fn name(&self) -> &str {
        match self {
            Entity::Vehicle(v) => &v.name,
            Entity::Pedestrian(p) => &p.name,
            Entity::MiscObject(m) => &m.name,
        }
    }
}
