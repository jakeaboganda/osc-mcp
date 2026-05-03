use crate::server::ServerState;
use anyhow::{anyhow, Result};
use openscenario::entities::{CatalogReference, VehicleCategory, VehicleParams};
use openscenario::storyboard::TransitionShape;
use openscenario::validation::XsdValidator;
use openscenario::Position;
use openscenario::{OpenScenarioVersion, Scenario};
use serde_json::json;
use std::fs;
use std::sync::{Arc, Mutex};
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
        _ => {
            return Err(anyhow!(
                "Invalid version: {}. Must be 1.0, 1.1, or 1.2",
                version
            ))
        }
    };

    // Create scenario
    let scenario = Scenario::new(osc_version);

    // Generate unique ID
    let scenario_id = format!("{}_{}", name, Uuid::new_v4());

    // Store in state
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
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
    // Parse vehicle category (case-insensitive)
    let vehicle_category = match category.to_lowercase().as_str() {
        "car" => VehicleCategory::Car,
        "truck" => VehicleCategory::Truck,
        "bus" => VehicleCategory::Bus,
        "trailer" => VehicleCategory::Trailer,
        "van" => VehicleCategory::Van,
        "motorbike" => VehicleCategory::Motorbike,
        "bicycle" => VehicleCategory::Bicycle,
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
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
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
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    scenario.set_initial_position(entity_name.clone(), position)?;

    Ok(format!("Position set for entity: {}", entity_name))
}

/// Add a speed action to a scenario
/// Creates default story structure if it doesn't exist: story -> act -> maneuver_group -> maneuver -> event
pub fn handle_add_speed_action(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    entity_name: String,
    story_name: String,
    speed: f64,
    duration: f64,
) -> Result<String> {
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    // Ensure story structure exists
    let act_name = format!("{}_act", story_name);
    let mg_name = format!("{}_mg", entity_name);
    let maneuver_name = format!("{}_maneuver", entity_name);
    let event_name = "speed_event";

    // Try to create story structure (ignore errors if already exists)
    let _ = scenario.add_story(&story_name);
    let _ = scenario.add_act(&story_name, &act_name);
    let _ = scenario.add_maneuver_group(&story_name, &act_name, &mg_name);
    let _ = scenario.add_maneuver(&story_name, &act_name, &mg_name, &maneuver_name);
    
    // Ensure actor is added (try multiple times if needed)
    if let Err(e) = scenario.add_actor(&story_name, &act_name, &mg_name, entity_name.clone()) {
        // If it failed, log but continue - the actor might already exist
        eprintln!("Note: add_actor returned error (may be ok if already exists): {}", e);
    }

    // Add speed action
    scenario.add_speed_action(
        &story_name,
        &act_name,
        &mg_name,
        &maneuver_name,
        event_name,
        speed,
        duration,
        TransitionShape::Linear,
    )?;

    Ok(format!(
        "Speed action added: {} m/s over {} seconds",
        speed, duration
    ))
}

/// Add a lane change action to a scenario
/// Creates default story structure if it doesn't exist: story -> act -> maneuver_group -> maneuver -> event
pub fn handle_add_lane_change_action(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    entity_name: String,
    story_name: String,
    target_lane: f64,
    duration: f64,
) -> Result<String> {
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    // Ensure story structure exists
    let act_name = format!("{}_act", story_name);
    let mg_name = format!("{}_mg", entity_name);
    let maneuver_name = format!("{}_maneuver", entity_name);
    let event_name = "lane_change_event";

    // Try to create story structure (ignore errors if already exists)
    let _ = scenario.add_story(&story_name);
    let _ = scenario.add_act(&story_name, &act_name);
    let _ = scenario.add_maneuver_group(&story_name, &act_name, &mg_name);
    let _ = scenario.add_maneuver(&story_name, &act_name, &mg_name, &maneuver_name);
    
    // Ensure actor is added
    if let Err(e) = scenario.add_actor(&story_name, &act_name, &mg_name, entity_name.clone()) {
        eprintln!("Note: add_actor returned error (may be ok if already exists): {}", e);
    }

    // Add lane change action
    scenario.add_lane_change_action(
        &story_name,
        &act_name,
        &mg_name,
        &maneuver_name,
        event_name,
        target_lane,
        duration,
        TransitionShape::Linear,
    )?;

    Ok(format!(
        "Lane change action added: target lane offset {} over {} seconds",
        target_lane, duration
    ))
}

/// Export a scenario to an XML file
pub fn handle_export_xml(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    output_path: String,
) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    // Generate XML
    let xml_content = scenario.to_xml()?;

    // Write to file
    fs::write(&output_path, xml_content).map_err(|e| anyhow!("Failed to write XML file: {}", e))?;

    Ok(format!("Exported scenario to: {}", output_path))
}

/// Validate a scenario using XSD validation
pub fn handle_validate_scenario(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    // Generate XML
    let xml_content = scenario.to_xml()?;

    // Get version string
    let version_str = scenario.version().to_string();

    // Create validator and validate
    let validator = XsdValidator::new(version_str);
    let report = validator.validate(&xml_content);

    // Format as JSON report
    let json_report = json!({
        "valid": report.valid,
        "errors": report.errors
    });

    Ok(json_report.to_string())
}

pub fn handle_set_stop_time(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    seconds: f64,
) -> Result<String> {
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    scenario.set_stop_time(seconds);
    Ok(format!("Set stop time to {} seconds", seconds))
}

pub fn handle_set_stop_on_element(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    element_type: String,
    element_ref: String,
    state_name: String,
    delay: f64,
) -> Result<String> {
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    scenario.set_stop_on_element_state(element_type.clone(), element_ref.clone(), state_name.clone(), delay);
    Ok(format!("Set stop trigger on {} element '{}' reaching state '{}'", element_type, element_ref, state_name))
}
