use crate::server::ServerState;
use anyhow::{anyhow, Result};
use openscenario::entities::{
    CatalogReference, MiscObjectParams, PedestrianParams, VehicleCategory, VehicleParams,
};
use openscenario::storyboard::{
    DynamicsDimension, DynamicsShape, TransitionDynamics, TransitionShape,
};
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
        "1.3" => OpenScenarioVersion::V1_3,
        _ => {
            return Err(anyhow!(
                "Invalid version: {}. Must be 1.0, 1.1, 1.2, or 1.3",
                version
            ))
        }
    };

    // Create scenario
    let mut scenario = Scenario::new(osc_version);

    // Generate unique ID
    let scenario_id = format!("{}_{}", name, Uuid::new_v4());

    // Store in state - REQUIRE road network to be loaded
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    // STRICT REQUIREMENT: Road network must be loaded before creating scenarios
    let road_network_path = state_lock.current_road_network.as_ref().ok_or_else(|| {
        anyhow!(
            "No road network loaded. Please load a road network first using:\n\
             - get_real_world_road(location) to download from OpenStreetMap, or\n\
             - load_road_network(xodr_path) to use a custom .xodr file"
        )
    })?;

    scenario.set_road_network(road_network_path)?;

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

/// Add a pedestrian to a scenario
pub fn handle_add_pedestrian(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    name: String,
    catalog: Option<String>,
    mass: Option<f64>,
) -> Result<String> {
    // Parse catalog reference if provided
    let catalog_ref = if let Some(ref path) = catalog {
        let parts: Vec<&str> = path.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow!(
                "Invalid catalog format: '{}'. Expected 'path:entry_name'",
                path
            ));
        }
        Some(CatalogReference {
            path: parts[0].to_string(),
            entry_name: parts[1].to_string(),
        })
    } else {
        None
    };

    // Validate mass if provided
    if let Some(m) = mass {
        if m <= 0.0 {
            return Err(anyhow!("Mass must be positive, got: {}", m));
        }
        if m > 500.0 {
            return Err(anyhow!(
                "Mass {} kg seems unrealistic for a pedestrian. Max: 500 kg",
                m
            ));
        }
    }

    let params = PedestrianParams {
        catalog: catalog_ref,
        model: None,
        mass: mass.or(Some(70.0)), // Default to 70kg if not provided
    };
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;
    scenario.add_pedestrian(name.clone(), params)?;
    Ok(name)
}

/// Add a misc object to a scenario
pub fn handle_add_misc_object(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    name: String,
    category: String,
    mass: f64,
) -> Result<String> {
    // Validate mass
    if mass <= 0.0 {
        return Err(anyhow!("Mass must be positive, got: {}", mass));
    }
    if mass > 100000.0 {
        return Err(anyhow!(
            "Mass {} kg seems unrealistic. Max: 100000 kg (100 tons)",
            mass
        ));
    }

    // Validate category
    const VALID_CATEGORIES: &[&str] = &[
        "barrier",
        "obstacle",
        "pole",
        "tree",
        "vegetation",
        "building",
        "vehicle",
        "none",
    ];
    if !VALID_CATEGORIES.contains(&category.as_str()) {
        return Err(anyhow!(
            "Invalid category '{}'. Valid categories: {}",
            category,
            VALID_CATEGORIES.join(", ")
        ));
    }

    let params = MiscObjectParams {
        catalog: None,
        category: Some(category),
        mass: Some(mass),
    };
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;
    scenario.add_misc_object(name.clone(), params)?;
    Ok(name)
}

/// Set initial lane position for an entity in a scenario
pub fn handle_set_lane_position(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    entity_name: String,
    road_id: String,
    lane_id: i32,
    s: f64,
    offset: f64,
) -> Result<String> {
    // Validate inputs
    if s < 0.0 {
        return Err(anyhow!("Position 's' cannot be negative, got: {}", s));
    }
    if lane_id == 0 {
        return Err(anyhow!("Lane ID cannot be 0 (center lane is invalid)"));
    }
    if offset.abs() > 10.0 {
        return Err(anyhow!(
            "Lateral offset {} m seems excessive. Typical range: ±10m",
            offset
        ));
    }

    let position = Position::lane(road_id, lane_id, s, offset, None);

    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario not found: {}", scenario_id))?;

    scenario.set_initial_position(entity_name.clone(), position)?;

    Ok(format!("Lane position set for entity: {}", entity_name))
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
    start_time: Option<f64>,
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
        eprintln!(
            "Note: add_actor returned error (may be ok if already exists): {}",
            e
        );
    }

    // Add speed action
    scenario.add_speed_action(
        &story_name,
        &act_name,
        &mg_name,
        &maneuver_name,
        event_name,
        speed,
        TransitionDynamics {
            shape: DynamicsShape::Linear,
            dimension: DynamicsDimension::Time,
            value: duration,
        },
    )?;

    // Auto-add start trigger if start_time provided
    let trigger_msg = if let Some(start_time) = start_time {
        use openscenario::storyboard::{Condition, ConditionEdge, ConditionGroup, Rule, Trigger};

        let mut condition = Condition::simulation_time(start_time, Rule::GreaterThan);
        condition.condition_edge = ConditionEdge::Rising;

        let condition_group = ConditionGroup::new(vec![condition]);
        let trigger = Trigger::new(condition_group);

        scenario
            .set_act_start_trigger(&story_name, &act_name, trigger)
            .map_err(|e| anyhow!("Failed to set Act trigger: {}", e))?;

        format!("\nAuto-set start trigger: Act starts at t={}s", start_time)
    } else {
        "\n⚠️  Warning: No start trigger set. Act will not execute unless you call set_trigger_time or set_collision_trigger.".to_string()
    };

    Ok(format!(
        "Speed action added: {} m/s over {} seconds\nCreated hierarchy: story='{}', act='{}', maneuver_group='{}', maneuver='{}', event='{}'{}" ,
        speed, duration, story_name, act_name, mg_name, maneuver_name, event_name, trigger_msg
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
    start_time: Option<f64>,
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
        eprintln!(
            "Note: add_actor returned error (may be ok if already exists): {}",
            e
        );
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

    // Auto-add start trigger if start_time provided
    let trigger_msg = if let Some(start_time) = start_time {
        use openscenario::storyboard::{Condition, ConditionEdge, ConditionGroup, Rule, Trigger};

        let mut condition = Condition::simulation_time(start_time, Rule::GreaterThan);
        condition.condition_edge = ConditionEdge::Rising;

        let condition_group = ConditionGroup::new(vec![condition]);
        let trigger = Trigger::new(condition_group);

        scenario
            .set_act_start_trigger(&story_name, &act_name, trigger)
            .map_err(|e| anyhow!("Failed to set Act trigger: {}", e))?;

        format!("\nAuto-set start trigger: Act starts at t={}s", start_time)
    } else {
        "\n⚠️  Warning: No start trigger set. Act will not execute unless you call set_trigger_time or set_collision_trigger.".to_string()
    };

    Ok(format!(
        "Lane change action added: target lane offset {} over {} seconds\nCreated hierarchy: story='{}', act='{}', maneuver_group='{}', maneuver='{}', event='{}'{}",
        target_lane, duration, story_name, act_name, mg_name, maneuver_name, event_name, trigger_msg
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

    scenario.set_stop_on_element_state(
        element_type.clone(),
        element_ref.clone(),
        state_name.clone(),
        delay,
    );
    Ok(format!(
        "Set stop trigger on {} element '{}' reaching state '{}'",
        element_type, element_ref, state_name
    ))
}

/// Load and analyze an OpenDRIVE road network
pub fn handle_load_road_network(
    state: Arc<Mutex<ServerState>>,
    xodr_path: String,
) -> Result<String> {
    use openscenario::opendrive_validator::OpenDriveValidator;
    use std::path::Path;

    let path = Path::new(&xodr_path);
    let validator = OpenDriveValidator::load(path)
        .map_err(|e| anyhow!("Failed to load OpenDRIVE file: {}", e))?;

    // Get road information
    let roads = validator.list_roads();
    let quality = validator.assess_quality();

    // Store validator in state
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    state_lock.road_validator = Some(validator);
    state_lock.current_road_network = Some(xodr_path.clone());

    Ok(json!({
        "status": "success",
        "file": xodr_path,
        "road_count": roads.len(),
        "roads": roads,
        "quality": {
            "score": quality.score,
            "has_lanes": quality.has_lanes,
            "has_geometry": quality.has_geometry,
            "has_valid_length": quality.has_valid_length,
            "issues": quality.issues
        }
    })
    .to_string())
}

/// List all roads in the loaded network
pub fn handle_list_roads(state: Arc<Mutex<ServerState>>) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let validator = state_lock
        .road_validator
        .as_ref()
        .ok_or_else(|| anyhow!("No road network loaded. Use load_road_network first."))?;

    let roads = validator.list_roads();
    Ok(json!({
        "roads": roads,
        "count": roads.len()
    })
    .to_string())
}

/// Get detailed information about a specific road
pub fn handle_get_road_info(state: Arc<Mutex<ServerState>>, road_id: String) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let validator = state_lock
        .road_validator
        .as_ref()
        .ok_or_else(|| anyhow!("No road network loaded. Use load_road_network first."))?;

    let info = validator
        .get_road_info(&road_id)
        .ok_or_else(|| anyhow!("Road '{}' not found", road_id))?;

    Ok(json!(info).to_string())
}

/// Suggest valid spawn points for vehicles
pub fn handle_suggest_spawn_points(
    state: Arc<Mutex<ServerState>>,
    road_id: String,
    count: usize,
) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let validator = state_lock
        .road_validator
        .as_ref()
        .ok_or_else(|| anyhow!("No road network loaded. Use load_road_network first."))?;

    let points = validator
        .suggest_spawn_points(&road_id, count)
        .map_err(|e| anyhow!("Failed to generate spawn points: {}", e))?;

    Ok(json!({
        "spawn_points": points,
        "count": points.len(),
        "road_id": road_id
    })
    .to_string())
}

/// Validate a position against the loaded road network
pub fn handle_validate_position(
    state: Arc<Mutex<ServerState>>,
    road_id: String,
    lane_id: i32,
    s: f64,
) -> Result<String> {
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let validator = state_lock
        .road_validator
        .as_ref()
        .ok_or_else(|| anyhow!("No road network loaded. Use load_road_network first."))?;

    // Validate road + s position
    validator
        .validate_road_position(&road_id, s)
        .map_err(|e| anyhow!("Position validation failed: {}", e))?;

    // Validate lane
    validator
        .validate_lane_position(&road_id, lane_id)
        .map_err(|e| anyhow!("Lane validation failed: {}", e))?;

    Ok(json!({
        "valid": true,
        "road_id": road_id,
        "lane_id": lane_id,
        "s": s,
        "message": "Position is valid"
    })
    .to_string())
}

/// Get a real-world road network from OpenStreetMap
pub fn handle_get_real_world_road(
    state: Arc<Mutex<ServerState>>,
    location: String,
    output_name: Option<String>,
) -> Result<String> {
    use std::process::Command;

    // Determine output name
    let name = output_name.unwrap_or_else(|| location.replace(' ', "_").to_lowercase());

    // Get workspace root (assuming we're in openscenario-mcp/)
    let workspace_root =
        std::env::current_dir().map_err(|e| anyhow!("Failed to get current directory: {}", e))?;

    let script_path = workspace_root.join("tools/osm/osm_to_opendrive.py");

    if !script_path.exists() {
        return Err(anyhow!(
            "OSM converter script not found at: {:?}",
            script_path
        ));
    }

    println!(
        "🌍 Fetching real-world road: {} (output: {})",
        location, name
    );

    // Run Python script
    let output = Command::new("python3")
        .arg(&script_path)
        .arg(&location)
        .arg("-o")
        .arg(&name)
        .current_dir(&workspace_root)
        .output()
        .map_err(|e| anyhow!("Failed to execute OSM converter: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(anyhow!(
            "OSM conversion failed:\nStdout: {}\nStderr: {}",
            stdout,
            stderr
        ));
    }

    // Expected output path
    let xodr_path = workspace_root.join(format!("cache/osm/{}.xodr", name));

    if !xodr_path.exists() {
        return Err(anyhow!("OpenDRIVE file not created: {:?}", xodr_path));
    }

    // Load the road network
    let validator = openscenario::opendrive_validator::OpenDriveValidator::load(&xodr_path)
        .map_err(|e| anyhow!("Failed to load generated OpenDRIVE: {}", e))?;

    // Analyze the network
    let quality = validator.assess_quality();
    let roads = validator.list_roads();

    // Find good roads (>50m, has lanes)
    let mut good_roads: Vec<_> = roads
        .iter()
        .filter(|r| r.length > 50.0 && r.lane_count > 1)
        .collect();
    good_roads.sort_by(|a, b| b.length.partial_cmp(&a.length).unwrap());

    // Get recommended road (longest good road)
    let recommended = good_roads.first().map(|r| {
        let spawn_points = validator.suggest_spawn_points(&r.id, 5).unwrap_or_default();
        json!({
            "road_id": r.id,
            "length": r.length,
            "lane_count": r.lane_count,
            "name": r.name,
            "spawn_points": spawn_points
        })
    });

    // Store validator in state
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;
    state_lock.road_validator = Some(validator);
    state_lock.current_road_network = Some(xodr_path.to_string_lossy().to_string());

    Ok(json!({
        "status": "success",
        "location": location,
        "xodr_path": xodr_path.to_string_lossy(),
        "total_roads": roads.len(),
        "good_roads": good_roads.len(),
        "quality": {
            "score": quality.score,
            "has_lanes": quality.has_lanes,
            "has_geometry": quality.has_geometry,
            "issues": quality.issues
        },
        "recommended_road": recommended,
        "top_roads": good_roads.iter().take(10).map(|r| json!({
            "id": r.id,
            "length": r.length,
            "lane_count": r.lane_count,
            "name": r.name
        })).collect::<Vec<_>>()
    })
    .to_string())
}

/// Generic trigger setter for Acts and Events
///
/// Handles the common logic for setting triggers on Acts or Events:
/// - Parameter validation (element_type, Event requirements)
/// - State management (lock, scenario lookup)
/// - Branching (Act vs Event path)
/// - Error handling and descriptive return messages
///
/// # Arguments
/// * `state` - Shared server state
/// * `scenario_id` - Target scenario
/// * `element_type` - "Act" or "Event"
/// * `story_name` - Parent story name
/// * `act_name` - Parent act name
/// * `maneuver_group` - Parent maneuver group (Event only)
/// * `maneuver` - Parent maneuver (Event only)
/// * `event_name` - Target event name (Event only)
/// * `condition_builder` - Closure that creates the condition
/// * `description` - Human-readable description of the trigger for response message
///
/// # Returns
/// Success message describing what was set
///
/// # Errors
/// * If scenario not found
/// * If element type is invalid
/// * If Event requirements not met
/// * If Act/Event not found in hierarchy
fn set_element_trigger<F>(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    element_type: String,
    story_name: String,
    act_name: String,
    maneuver_group: Option<String>,
    maneuver: Option<String>,
    event_name: Option<String>,
    condition_builder: F,
    description: String,
) -> Result<String>
where
    F: FnOnce() -> openscenario::storyboard::Condition,
{
    use openscenario::storyboard::{ConditionGroup, Trigger};

    // Validate element_type
    if element_type != "Act" && element_type != "Event" {
        return Err(anyhow!(
            "element_type must be 'Act' or 'Event' (got '{}'). Expected: Set to 'Act' for Act-level triggers or 'Event' for Event-level triggers.",
            element_type
        ));
    }

    // Validate Event has required fields
    let event_params = if element_type == "Event" {
        let mg = maneuver_group.ok_or_else(|| {
            anyhow!(
                "Missing 'maneuver_group' parameter. Event triggers require: maneuver_group, maneuver, and event_name. For auto-generated names from add_speed_action/add_lane_change_action, use pattern '{{entity}}_mg' (e.g., 'ego_mg')."
            )
        })?;
        let mn = maneuver.ok_or_else(|| {
            anyhow!(
                "Missing 'maneuver' parameter. Event triggers require: maneuver_group, maneuver, and event_name. For auto-generated names, use pattern '{{entity}}_maneuver' (e.g., 'ego_maneuver')."
            )
        })?;
        let ev = event_name.ok_or_else(|| {
            anyhow!(
                "Missing 'event_name' parameter. Event triggers require: maneuver_group, maneuver, and event_name. For auto-generated names, use pattern '{{entity}}_event' (e.g., 'ego_event')."
            )
        })?;
        Some((mg, mn, ev))
    } else {
        None
    };

    // Acquire state lock
    let mut state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    // Get scenario
    let scenario = state_lock
        .scenarios
        .get_mut(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario '{}' not found", scenario_id))?;

    // Create condition using provided builder
    let condition = condition_builder();

    // Create trigger with condition group
    let condition_group = ConditionGroup::new(vec![condition]);
    let trigger = Trigger::new(condition_group);

    // Apply trigger based on element type
    match element_type.as_str() {
        "Act" => {
            scenario
                .set_act_start_trigger(&story_name, &act_name, trigger)
                .map_err(|e| anyhow!("Failed to set Act trigger: {}", e))?;
            Ok(format!(
                "Set {} for Act '{}' in story '{}'",
                description, act_name, story_name
            ))
        }
        "Event" => {
            let (mg, mn, ev) = event_params.unwrap(); // Safe: validated above

            scenario
                .set_event_start_trigger(&story_name, &act_name, &mg, &mn, &ev, trigger)
                .map_err(|e| anyhow!("Failed to set Event trigger: {}", e))?;
            Ok(format!(
                "Set {} for Event '{}' (in maneuver '{}')",
                description, ev, mn
            ))
        }
        _ => unreachable!("Already validated element_type"),
    }
}

/// Set a time-based trigger for an Act or Event
///
/// # Arguments
/// * `state` - Shared server state
/// * `scenario_id` - ID of the scenario
/// * `element_type` - "Act" or "Event"
/// * `story_name` - Name of the story containing the element
/// * `act_name` - Name of the act (for both Act and Event triggers)
/// * `maneuver_group` - Name of the maneuver group (required for Event triggers)
/// * `maneuver` - Name of the maneuver (required for Event triggers)
/// * `event_name` - Name of the event (required for Event triggers)
/// * `time_seconds` - Simulation time in seconds for the trigger
/// * `delay_seconds` - Optional delay after condition is met (default: 0.0)
///
/// # Returns
/// Success message
///
/// # Errors
/// * If scenario not found
/// * If element type is invalid
/// * If Act/Event not found
pub fn handle_set_trigger_time(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    element_type: String,
    story_name: String,
    act_name: String,
    maneuver_group: Option<String>,
    maneuver: Option<String>,
    event_name: Option<String>,
    time_seconds: f64,
    delay_seconds: Option<f64>,
) -> Result<String> {
    use openscenario::storyboard::{Condition, ConditionEdge, Rule};

    // Validate time_seconds is non-negative
    if time_seconds < 0.0 {
        return Err(anyhow!(
            "time_seconds must be non-negative (got {}). Expected: Use a positive simulation time value (e.g., 5.0 for 5 seconds).",
            time_seconds
        ));
    }

    // Validate delay_seconds if provided
    let delay = delay_seconds.unwrap_or(0.0);
    if delay < 0.0 {
        return Err(anyhow!(
            "delay_seconds must be non-negative (got {}). Expected: Use a positive delay value or omit for immediate trigger.",
            delay
        ));
    }

    let description = format!(
        "time-based trigger at t={}s (delay: {}s)",
        time_seconds, delay
    );

    set_element_trigger(
        state,
        scenario_id,
        element_type,
        story_name,
        act_name,
        maneuver_group,
        maneuver,
        event_name,
        || {
            let mut condition = Condition::simulation_time(time_seconds, Rule::GreaterThan);
            condition.delay = delay;
            condition.condition_edge = ConditionEdge::Rising;
            condition
        },
        description,
    )
}

/// Set a collision-based trigger for an Act or Event
///
/// # Arguments
/// * `state` - Shared server state
/// * `scenario_id` - ID of the scenario
/// * `element_type` - "Act" or "Event"
/// * `story_name` - Name of the story containing the element
/// * `act_name` - Name of the act (for both Act and Event triggers)
/// * `maneuver_group` - Name of the maneuver group (required for Event triggers)
/// * `maneuver` - Name of the maneuver (required for Event triggers)
/// * `event_name` - Name of the event (required for Event triggers)
/// * `entity_refs` - List of entities to monitor for collisions
/// * `target_entity` - Entity to detect collisions with
/// * `trigger_rule` - "any" or "all" (whether any or all entities must collide)
/// * `delay_seconds` - Optional delay after condition is met (default: 0.0)
///
/// # Returns
/// Success message
///
/// # Errors
/// * If scenario not found
/// * If element type is invalid
/// * If Act/Event not found
pub fn handle_set_collision_trigger(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    element_type: String,
    story_name: String,
    act_name: String,
    maneuver_group: Option<String>,
    maneuver: Option<String>,
    event_name: Option<String>,
    entity_refs: Vec<String>,
    target_entity: String,
    trigger_rule: String,
    delay_seconds: Option<f64>,
) -> Result<String> {
    use openscenario::storyboard::{Condition, TriggeringEntitiesRule};

    // Validate trigger_rule
    let rule = match trigger_rule.to_lowercase().as_str() {
        "any" => TriggeringEntitiesRule::Any,
        "all" => TriggeringEntitiesRule::All,
        _ => {
            return Err(anyhow!(
                "trigger_rule must be 'any' or 'all' (got '{}'). Expected: Use 'any' if at least one entity should trigger, or 'all' if all entities must trigger.",
                trigger_rule
            ))
        }
    };

    // Validate entity_refs not empty
    if entity_refs.is_empty() {
        return Err(anyhow!(
            "entity_refs must contain at least one entity. Expected: Provide an array of entity names to monitor (e.g., ['ego', 'vehicle2'])."
        ));
    }

    // Check for duplicate entities
    let unique_count = entity_refs
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len();
    if unique_count != entity_refs.len() {
        return Err(anyhow!(
            "entity_refs contains duplicate entities. Expected: Each entity should appear only once in the array."
        ));
    }

    // Validate delay_seconds if provided
    let delay = delay_seconds.unwrap_or(0.0);
    if delay < 0.0 {
        return Err(anyhow!(
            "delay_seconds must be non-negative (got {}). Expected: Use a positive delay value or omit for immediate trigger.",
            delay
        ));
    }

    let description = format!(
        "collision trigger: {} of [{}] collide with '{}' (delay: {}s)",
        trigger_rule,
        entity_refs.join(", "),
        target_entity,
        delay
    );

    set_element_trigger(
        state,
        scenario_id,
        element_type,
        story_name,
        act_name,
        maneuver_group,
        maneuver,
        event_name,
        || {
            let mut condition = Condition::collision(entity_refs.clone(), &target_entity, rule);
            condition.delay = delay;
            condition
        },
        description,
    )
}

/// List triggers for an Act or Event
///
/// Retrieves and displays the start trigger configuration for a specified Act or Event.
///
/// # Arguments
/// * `state` - Shared server state
/// * `scenario_id` - Target scenario
/// * `element_type` - "Act" or "Event"
/// * `story_name` - Parent story name (optional for Act-level search)
/// * `act_name` - Parent act name
/// * `maneuver_group` - Parent maneuver group (Event only)
/// * `maneuver` - Parent maneuver (Event only)
/// * `event_name` - Target event name (Event only)
///
/// # Returns
/// Human-readable trigger description or "No trigger set"
///
/// # Errors
/// * If scenario not found
/// * If element type is invalid
/// * If Act/Event not found
pub fn handle_list_triggers(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    element_type: String,
    act_name: String,
    story_name: Option<String>,
    maneuver_group: Option<String>,
    maneuver: Option<String>,
    event_name: Option<String>,
) -> Result<String> {
    // Validate element_type
    if element_type != "Act" && element_type != "Event" {
        return Err(anyhow!(
            "element_type must be 'Act' or 'Event' (got '{}')",
            element_type
        ));
    }

    // Acquire state lock
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    // Get scenario
    let scenario = state_lock
        .scenarios
        .get(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario '{}' not found", scenario_id))?;

    // Get trigger based on element type
    let trigger_opt = match element_type.as_str() {
        "Act" => scenario.get_act_start_trigger(&act_name)?,
        "Event" => {
            let story =
                story_name.ok_or_else(|| anyhow!("story_name required for Event triggers"))?;
            let mg = maneuver_group
                .ok_or_else(|| anyhow!("maneuver_group required for Event triggers"))?;
            let mn = maneuver.ok_or_else(|| anyhow!("maneuver required for Event triggers"))?;
            let ev = event_name.ok_or_else(|| anyhow!("event_name required for Event triggers"))?;

            scenario.get_event_start_trigger(&story, &act_name, &mg, &mn, &ev)?
        }
        _ => unreachable!("Already validated element_type"),
    };

    // Format trigger information
    match trigger_opt {
        None => Ok(format!(
            "No trigger set for {} '{}'",
            element_type, act_name
        )),
        Some(trigger) => {
            let mut parts = Vec::new();

            for cond_group in &trigger.condition_groups {
                for condition in &cond_group.conditions {
                    let desc = format_condition_description(condition);
                    parts.push(desc);
                }
            }

            if parts.is_empty() {
                Ok(format!(
                    "Trigger exists but has no conditions for {} '{}'",
                    element_type, act_name
                ))
            } else {
                Ok(format!(
                    "Triggers for {} '{}':\n{}",
                    element_type,
                    act_name,
                    parts.join("\n")
                ))
            }
        }
    }
}

/// Format a condition into a human-readable description
fn format_condition_description(condition: &openscenario::storyboard::Condition) -> String {
    use openscenario::storyboard::{ByValueCondition, ConditionKind, EntityCondition};

    let delay_str = if condition.delay > 0.0 {
        format!(" (delay: {}s)", condition.delay)
    } else {
        String::new()
    };

    let edge_str = format!("{:?}", condition.condition_edge).to_lowercase();

    match &condition.kind {
        ConditionKind::ByValue(by_value) => match by_value {
            ByValueCondition::SimulationTime { value, rule } => {
                format!(
                    "- SimulationTime: {:?} {} (edge: {}){}",
                    rule, value, edge_str, delay_str
                )
            }
            ByValueCondition::StoryboardElementState {
                element_type,
                element_ref,
                state,
            } => {
                format!(
                    "- StoryboardElement: {} '{}' reaches '{}' (edge: {}){}",
                    element_type, element_ref, state, edge_str, delay_str
                )
            }
            ByValueCondition::Parameter(param) => {
                format!(
                    "- Parameter: {} {:?} '{}' (edge: {}){}",
                    param.parameter_ref, param.rule, param.value, edge_str, delay_str
                )
            }
        },
        ConditionKind::ByEntity(by_entity) => {
            let entities: Vec<&str> = by_entity
                .triggering_entities
                .entity_refs
                .iter()
                .map(|e| e.as_str())
                .collect();

            match &by_entity.entity_condition {
                EntityCondition::Collision(collision) => {
                    format!(
                        "- Collision: {:?} of [{}] with '{}' (edge: {}){}",
                        by_entity.triggering_entities.rule,
                        entities.join(", "),
                        collision.target_entity_ref,
                        edge_str,
                        delay_str
                    )
                }
                EntityCondition::Speed(speed) => {
                    format!(
                        "- Speed: {:?} of [{}] {:?} {} (edge: {}){}",
                        by_entity.triggering_entities.rule,
                        entities.join(", "),
                        speed.rule,
                        speed.value,
                        edge_str,
                        delay_str
                    )
                }
                _ => {
                    format!(
                        "- EntityCondition: {:?} of [{}] (edge: {}){}",
                        by_entity.triggering_entities.rule,
                        entities.join(", "),
                        edge_str,
                        delay_str
                    )
                }
            }
        }
    }
}

/// Validate scenario for common issues before export
///
/// Checks for:
/// - Acts without start triggers ("dead" maneuvers)
/// - Unreferenced entities
/// - Other structural issues
///
/// # Arguments
/// * `state` - Shared server state
/// * `scenario_id` - Target scenario
/// * `auto_fix` - If true, auto-inject t=0 triggers for Acts without triggers
///
/// # Returns
/// Validation report with warnings and errors
pub fn handle_validate_scenario_structure(
    state: Arc<Mutex<ServerState>>,
    scenario_id: String,
    _auto_fix: bool,
) -> Result<String> {
    // No current structural check has a safe automatic fix (each one needs domain
    // knowledge -- what Act to add, which entity should be an actor, where to place
    // an entity -- that can't be guessed without risking a wrong answer, e.g. an
    // arbitrary spawn position colliding with another entity). auto_fix is kept as
    // a parameter for API stability but is currently a no-op.
    let state_lock = state
        .lock()
        .map_err(|_| anyhow!("Failed to acquire state lock: mutex poisoned"))?;

    let scenario = state_lock
        .scenarios
        .get(&scenario_id)
        .ok_or_else(|| anyhow!("Scenario '{}' not found", scenario_id))?;

    let mut warnings: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    // Structural cardinality gaps: the XSD requires at least one Act per Story, one
    // ManeuverGroup per Act, and one Event per Maneuver (none has minOccurs="0").
    // A scenario with any of these would fail real XSD validation, so these are
    // errors, not warnings. None of them has a safe auto-fix (there's no sensible
    // default Act/ManeuverGroup/Event to invent), so auto_fix doesn't apply here.
    for story in scenario.stories() {
        if story.acts.is_empty() {
            errors.push(format!(
                "Story '{}' has no Acts (the XSD requires at least one; export will fail validation)",
                story.name
            ));
            continue;
        }
        for (act_name, act) in &story.acts {
            if act.maneuver_groups.is_empty() {
                errors.push(format!(
                    "Act '{}' in story '{}' has no ManeuverGroups (the XSD requires at least one)",
                    act_name, story.name
                ));
                continue;
            }
            for (mg_name, mg) in &act.maneuver_groups {
                if mg.actors.is_empty() {
                    warnings.push(format!(
                        "ManeuverGroup '{}' in act '{}' has no actors (its maneuvers will never act on anyone)",
                        mg_name, act_name
                    ));
                }
                for maneuver in &mg.maneuvers {
                    if maneuver.events.is_empty() {
                        errors.push(format!(
                            "Maneuver '{}' in group '{}' has no Events (the XSD requires at least one)",
                            maneuver.name, mg_name
                        ));
                    }
                }
            }
        }
    }

    // An entity with neither an initial position nor an initial speed is silently
    // omitted from <Init><Actions> by the XML writer (see write_init in xml.rs) --
    // it's declared in <Entities> but never placed in the simulation. There's no
    // safe default location to auto-fix this with (an arbitrary position could
    // itself collide with another entity's spawn point), so this stays a warning.
    for entity in scenario.entities() {
        let name = entity.name();
        if scenario.get_initial_position(name).is_none()
            && scenario.get_initial_speed(name).is_none()
        {
            warnings.push(format!(
                "Entity '{}' has no initial position or speed (it won't be placed in the simulation)",
                name
            ));
        }
    }

    // Build report
    let mut report = String::new();

    if errors.is_empty() && warnings.is_empty() {
        report.push_str("✅ Scenario validation passed: No issues found\n");
    } else {
        report.push_str("📋 Scenario Validation Report\n\n");

        if !errors.is_empty() {
            report.push_str(&format!("❌ Errors ({}):\n", errors.len()));
            for error in &errors {
                report.push_str(&format!("  • {}\n", error));
            }
            report.push('\n');
        }

        if !warnings.is_empty() {
            report.push_str(&format!("⚠️  Warnings ({}):\n", warnings.len()));
            for warning in &warnings {
                report.push_str(&format!("  • {}\n", warning));
            }
            report.push('\n');
        }
    }

    Ok(report)
}
