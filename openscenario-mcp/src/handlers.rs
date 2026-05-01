use crate::server::ServerState;
use openscenario::{Scenario, OpenScenarioVersion};
use openscenario::entities::{VehicleCategory, VehicleParams, CatalogReference};
use openscenario::Position;
use std::sync::{Arc, Mutex};
use anyhow::{Result, anyhow};
use uuid::Uuid;

/// Create a new OpenSCENARIO scenario
pub fn handle_create_scenario(
    state: Arc<Mutex<ServerState>>,
    name: String,
    version: String,
) -> Result<String> {
    // Parse version
    let osc_version = match version.as_str() {
        "1.0" => OpenScenarioVersion::V1_0,
        "1.1" => OpenScenarioVersion::V1_1,
        "1.2" => OpenScenarioVersion::V1_2,
        _ => return Err(anyhow!("Invalid version: {}. Must be 1.0, 1.1, or 1.2", version)),
    };
    
    // Create scenario
    let scenario = Scenario::new(osc_version);
    
    // Generate unique ID
    let scenario_id = format!("{}_{}", name, Uuid::new_v4());
    
    // Store in state
    let mut state_lock = state.lock().unwrap();
    state_lock.scenarios.insert(scenario_id.clone(), scenario);
    
    Ok(scenario_id)
}

/// Add a vehicle to a scenario
pub fn handle_add_vehicle(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    name: String,
    category: String,
    catalog: Option<String>,
) -> Result<String> {
    // Parse vehicle category
    let vehicle_category = match category.as_str() {
        "Car" => VehicleCategory::Car,
        "Truck" => VehicleCategory::Truck,
        "Bus" => VehicleCategory::Bus,
        "Trailer" => VehicleCategory::Trailer,
        "Van" => VehicleCategory::Van,
        "Motorbike" => VehicleCategory::Motorbike,
        "Bicycle" => VehicleCategory::Bicycle,
        _ => return Err(anyhow!("Invalid vehicle category: {}", category)),
    };
    
    // Parse catalog if provided
    let catalog_ref = catalog.map(|path| {
        // Simple format: "path:entry_name"
        let parts: Vec<&str> = path.split(':').collect();
        if parts.len() == 2 {
            CatalogReference {
                path: parts[0].to_string(),
                entry_name: parts[1].to_string(),
            }
        } else {
            CatalogReference {
                path: path.clone(),
                entry_name: name.clone(),
            }
        }
    });
    
    let params = VehicleParams {
        catalog: catalog_ref,
        vehicle_category,
        properties: None,
    };
    
    // Get scenario and add vehicle
    let mut state_lock = state.lock().unwrap();
    let scenario = state_lock.scenarios.get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;
    
    scenario.add_vehicle(name.clone(), params)?;
    
    Ok(name)
}

/// Set initial position for an entity in a scenario
pub fn handle_set_position(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    entity_name: String,
    x: f64,
    y: f64,
    z: f64,
    h: f64,
) -> Result<String> {
    let position = Position::world(x, y, z, h);
    
    // Get scenario and set position
    let mut state_lock = state.lock().unwrap();
    let scenario = state_lock.scenarios.get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;
    
    scenario.set_initial_position(entity_name.clone(), position)?;
    
    Ok(format!("Position set for entity: {}", entity_name))
}
