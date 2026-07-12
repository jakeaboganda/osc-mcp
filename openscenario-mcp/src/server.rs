use crate::handlers::{
    handle_add_lane_change_action, handle_add_misc_object, handle_add_pedestrian,
    handle_add_speed_action, handle_add_vehicle, handle_create_scenario, handle_export_xml,
    handle_get_real_world_road, handle_get_road_info, handle_list_roads, handle_list_triggers,
    handle_load_road_network, handle_set_collision_trigger, handle_set_lane_position,
    handle_set_position, handle_set_stop_on_element, handle_set_stop_time, handle_set_trigger_time,
    handle_suggest_spawn_points, handle_validate_position, handle_validate_scenario,
    handle_validate_scenario_structure,
};
use crate::scenario_templates::{
    handle_create_cutin_scenario, handle_create_lane_change_scenario,
    handle_create_platoon_scenario, handle_create_quick_scenario,
};
use anyhow::{anyhow, Result};
use mcp_sdk::types::{
    CallToolRequest, CallToolResponse, ListRequest, ToolDefinition, ToolResponseContent,
    ToolsListResponse,
};
use once_cell::sync::Lazy;
use openscenario::opendrive_validator::OpenDriveValidator;
use openscenario::Scenario;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Global server state
static GLOBAL_STATE: Lazy<Arc<Mutex<ServerState>>> =
    Lazy::new(|| Arc::new(Mutex::new(ServerState::new())));

pub struct ServerState {
    pub scenarios: HashMap<String, Scenario>,
    pub road_validator: Option<OpenDriveValidator>,
    pub current_road_network: Option<String>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            scenarios: HashMap::new(),
            road_validator: None,
            current_road_network: None,
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OpenScenarioServer;

impl OpenScenarioServer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OpenScenarioServer {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenScenarioServer {
    pub fn register_tools() -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                name: "create_scenario".to_string(),
                description: Some("Create a new OpenSCENARIO scenario".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Scenario name"
                        },
                        "version": {
                            "type": "string",
                            "enum": ["1.0", "1.1", "1.2"],
                            "description": "OpenSCENARIO version"
                        }
                    },
                    "required": ["name", "version"]
                }),
            },
            ToolDefinition {
                name: "add_vehicle".to_string(),
                description: Some("Add a vehicle to a scenario".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID"
                        },
                        "name": {
                            "type": "string",
                            "description": "Vehicle name"
                        },
                        "category": {
                            "type": "string",
                            "enum": ["Car", "Truck", "Bus", "Trailer", "Van", "Motorbike", "Bicycle"],
                            "description": "Vehicle category"
                        },
                        "catalog": {
                            "type": "string",
                            "description": "Optional catalog reference (format: path:entry_name)"
                        }
                    },
                    "required": ["scenario_id", "name", "category"]
                }),
            },
            ToolDefinition {
                name: "set_position".to_string(),
                description: Some("Set initial world position for an entity".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID"
                        },
                        "entity_name": {
                            "type": "string",
                            "description": "Entity name"
                        },
                        "x": {
                            "type": "number",
                            "description": "X coordinate"
                        },
                        "y": {
                            "type": "number",
                            "description": "Y coordinate"
                        },
                        "z": {
                            "type": "number",
                            "description": "Z coordinate"
                        },
                        "h": {
                            "type": "number",
                            "description": "Heading (radians)"
                        }
                    },
                    "required": ["scenario_id", "entity_name", "x", "y", "z", "h"]
                }),
            },
            ToolDefinition {
                name: "add_pedestrian".to_string(),
                description: Some("Add a pedestrian entity to a scenario".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {"type": "string"},
                        "name": {"type": "string"},
                        "catalog": {"type": "string", "description": "Optional catalog reference (format: path:entry_name)"},
                        "mass": {"type": "number", "description": "Mass in kg (default: 70.0)"}
                    },
                    "required": ["scenario_id", "name"]
                }),
            },
            ToolDefinition {
                name: "add_misc_object".to_string(),
                description: Some("Add a miscellaneous object (barrier, obstacle, etc.) to a scenario".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {"type": "string"},
                        "name": {"type": "string"},
                        "category": {"type": "string", "description": "Object category: barrier, obstacle, pole, tree, vegetation, building, vehicle, none"},
                        "mass": {"type": "number", "description": "Mass in kg (max: 100000)"}
                    },
                    "required": ["scenario_id", "name", "category", "mass"]
                }),
            },
            ToolDefinition {
                name: "set_lane_position".to_string(),
                description: Some("Set initial lane position for an entity using OpenDRIVE road/lane coordinates".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {"type": "string", "description": "Scenario ID"},
                        "entity_name": {"type": "string", "description": "Entity name"},
                        "road_id": {"type": "string", "description": "OpenDRIVE road ID"},
                        "lane_id": {"type": "number", "description": "Lane ID (negative = right/forward lanes in LHT)"},
                        "s": {"type": "number", "description": "Position along road in meters"},
                        "offset": {"type": "number", "description": "Lateral offset from lane center (meters, positive = left)"}
                    },
                    "required": ["scenario_id", "entity_name", "road_id", "lane_id", "s", "offset"]
                }),
            },
            ToolDefinition {
                name: "add_speed_action".to_string(),
                description: Some("Add a speed action to a scenario. Auto-creates Act/Maneuver hierarchy.".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID"
                        },
                        "entity_name": {
                            "type": "string",
                            "description": "Entity name to apply action to"
                        },
                        "story_name": {
                            "type": "string",
                            "description": "Story name (will be created if doesn't exist)"
                        },
                        "speed": {
                            "type": "number",
                            "description": "Target speed in m/s"
                        },
                        "duration": {
                            "type": "number",
                            "description": "Duration in seconds"
                        },
                        "start_time": {
                            "type": "number",
                            "description": "Optional: Simulation time when Act should start (default: no trigger, must set manually)"
                        }
                    },
                    "required": ["scenario_id", "entity_name", "story_name", "speed", "duration"]
                }),
            },
            ToolDefinition {
                name: "add_lane_change_action".to_string(),
                description: Some("Add a lane change action to a scenario. Auto-creates Act/Maneuver hierarchy.".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID"
                        },
                        "entity_name": {
                            "type": "string",
                            "description": "Entity name to apply action to"
                        },
                        "story_name": {
                            "type": "string",
                            "description": "Story name (will be created if doesn't exist)"
                        },
                        "target_lane": {
                            "type": "number",
                            "description": "Target lane offset in meters"
                        },
                        "duration": {
                            "type": "number",
                            "description": "Duration in seconds"
                        },
                        "start_time": {
                            "type": "number",
                            "description": "Optional: Simulation time when Act should start (default: no trigger, must set manually)"
                        }
                    },
                    "required": ["scenario_id", "entity_name", "story_name", "target_lane", "duration"]
                }),
            },
            ToolDefinition {
                name: "export_xml".to_string(),
                description: Some("Export a scenario to an OpenSCENARIO XML file".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID"
                        },
                        "output_path": {
                            "type": "string",
                            "description": "Output file path (.xosc extension recommended)"
                        }
                    },
                    "required": ["scenario_id", "output_path"]
                }),
            },
            ToolDefinition {
                name: "validate_scenario".to_string(),
                description: Some(
                    "Validate a scenario against OpenSCENARIO XSD schema".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID to validate"
                        }
                    },
                    "required": ["scenario_id"]
                }),
            },
            ToolDefinition {
                name: "validate_scenario_structure".to_string(),
                description: Some(
                    "Validate scenario structure for issues XSD validation alone won't catch as clearly: Stories/Acts/Maneuvers missing required child elements, ManeuverGroups with no actors, and entities with no initial position or speed (silently excluded from the simulation).".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID to validate"
                        },
                        "auto_fix": {
                            "type": "boolean",
                            "description": "Reserved for future checks that have a safe automatic fix. No current check does (each needs domain knowledge, e.g. which entity should be an actor); has no effect today."
                        }
                    },
                    "required": ["scenario_id"]
                }),
            },
            ToolDefinition {
                name: "set_stop_time".to_string(),
                description: Some("Set a time-based stop trigger for the scenario".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID"
                        },
                        "seconds": {
                            "type": "number",
                            "description": "Simulation time in seconds after which to stop"
                        }
                    },
                    "required": ["scenario_id", "seconds"]
                }),
            },
            ToolDefinition {
                name: "set_stop_on_element".to_string(),
                description: Some(
                    "Set a stop trigger based on storyboard element state".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID"
                        },
                        "element_type": {
                            "type": "string",
                            "description": "Element type (e.g., 'maneuver', 'act', 'story')"
                        },
                        "element_ref": {
                            "type": "string",
                            "description": "Name/reference of the element"
                        },
                        "state": {
                            "type": "string",
                            "description": "Target state (e.g., 'completeState', 'endTransition')"
                        },
                        "delay": {
                            "type": "number",
                            "description": "Delay in seconds after condition is met"
                        }
                    },
                    "required": ["scenario_id", "element_type", "element_ref", "state", "delay"]
                }),
            },
            ToolDefinition {
                name: "set_trigger_time".to_string(),
                description: Some(
                    "Set a time-based trigger for an Act or Event to start at a specific simulation time. For Acts created by add_speed_action/add_lane_change_action, use story name + '_act' for act_name (e.g., 'main_act').".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID"
                        },
                        "element_type": {
                            "type": "string",
                            "enum": ["Act", "Event"],
                            "description": "Element type: 'Act' or 'Event'"
                        },
                        "story_name": {
                            "type": "string",
                            "description": "Name of the story (e.g., 'main')"
                        },
                        "act_name": {
                            "type": "string",
                            "description": "Name of the act (e.g., 'main_act')"
                        },
                        "maneuver_group": {
                            "type": "string",
                            "description": "Name of maneuver group (required for Event triggers only)"
                        },
                        "maneuver": {
                            "type": "string",
                            "description": "Name of maneuver (required for Event triggers only)"
                        },
                        "event_name": {
                            "type": "string",
                            "description": "Name of the event (required for Event triggers only)"
                        },
                        "time_seconds": {
                            "type": "number",
                            "description": "Simulation time in seconds when the trigger should activate"
                        },
                        "delay_seconds": {
                            "type": "number",
                            "description": "Optional delay in seconds after condition is met (default: 0.0)"
                        }
                    },
                    "required": ["scenario_id", "element_type", "story_name", "act_name", "time_seconds"]
                }),
            },
            ToolDefinition {
                name: "set_collision_trigger".to_string(),
                description: Some(
                    "Set a collision-based trigger for an Act or Event. Triggers when specified entities collide with a target entity. Useful for emergency scenarios, obstacle detection, or interaction triggers.".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID"
                        },
                        "element_type": {
                            "type": "string",
                            "enum": ["Act", "Event"],
                            "description": "Element type: 'Act' or 'Event'"
                        },
                        "story_name": {
                            "type": "string",
                            "description": "Name of the story (e.g., 'main')"
                        },
                        "act_name": {
                            "type": "string",
                            "description": "Name of the act (e.g., 'main_act')"
                        },
                        "maneuver_group": {
                            "type": "string",
                            "description": "Name of maneuver group (required for Event triggers only)"
                        },
                        "maneuver": {
                            "type": "string",
                            "description": "Name of maneuver (required for Event triggers only)"
                        },
                        "event_name": {
                            "type": "string",
                            "description": "Name of the event (required for Event triggers only)"
                        },
                        "entity_refs": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "List of entity names to monitor for collisions (e.g., ['ego', 'vehicle2'])"
                        },
                        "target_entity": {
                            "type": "string",
                            "description": "Name of the entity to detect collisions with (e.g., 'obstacle_1')"
                        },
                        "trigger_rule": {
                            "type": "string",
                            "enum": ["any", "all"],
                            "description": "Triggering rule: 'any' (at least one entity) or 'all' (all entities must collide)"
                        },
                        "delay_seconds": {
                            "type": "number",
                            "description": "Optional delay in seconds after collision detected (default: 0.0)"
                        }
                    },
                    "required": ["scenario_id", "element_type", "story_name", "act_name", "entity_refs", "target_entity", "trigger_rule"]
                }),
            },
            ToolDefinition {
                name: "list_triggers".to_string(),
                description: Some(
                    "List and inspect triggers for an Act or Event. Shows all configured trigger conditions including type, parameters, delay, and edge. Useful for debugging and verification.".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID"
                        },
                        "element_type": {
                            "type": "string",
                            "enum": ["Act", "Event"],
                            "description": "Element type: 'Act' or 'Event'"
                        },
                        "act_name": {
                            "type": "string",
                            "description": "Name of the act to inspect"
                        },
                        "story_name": {
                            "type": "string",
                            "description": "Name of the story (required for Event triggers)"
                        },
                        "maneuver_group": {
                            "type": "string",
                            "description": "Name of maneuver group (required for Event triggers only)"
                        },
                        "maneuver": {
                            "type": "string",
                            "description": "Name of maneuver (required for Event triggers only)"
                        },
                        "event_name": {
                            "type": "string",
                            "description": "Name of the event (required for Event triggers only)"
                        }
                    },
                    "required": ["scenario_id", "element_type", "act_name"]
                }),
            },
            ToolDefinition {
                name: "load_road_network".to_string(),
                description: Some(
                    "Load and analyze an OpenDRIVE road network file. Call this BEFORE creating scenarios on real roads.".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "xodr_path": {
                            "type": "string",
                            "description": "Path to OpenDRIVE (.xodr) file"
                        }
                    },
                    "required": ["xodr_path"]
                }),
            },
            ToolDefinition {
                name: "list_roads".to_string(),
                description: Some(
                    "List all roads in the loaded road network.".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            ToolDefinition {
                name: "get_road_info".to_string(),
                description: Some(
                    "Get detailed information about a specific road (lanes, length, etc.).".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "road_id": {
                            "type": "string",
                            "description": "Road ID to query"
                        }
                    },
                    "required": ["road_id"]
                }),
            },
            ToolDefinition {
                name: "suggest_spawn_points".to_string(),
                description: Some(
                    "Get valid spawn points for placing vehicles on a road. Returns positions with road_id, lane_id, and s-coordinate.".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "road_id": {
                            "type": "string",
                            "description": "Road ID where vehicles should be placed"
                        },
                        "count": {
                            "type": "number",
                            "description": "Number of spawn points needed"
                        }
                    },
                    "required": ["road_id", "count"]
                }),
            },
            ToolDefinition {
                name: "validate_position".to_string(),
                description: Some(
                    "Validate that a position (road_id, lane_id, s) exists in the loaded road network.".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "road_id": {
                            "type": "string",
                            "description": "Road ID"
                        },
                        "lane_id": {
                            "type": "number",
                            "description": "Lane ID (negative for driving lanes)"
                        },
                        "s": {
                            "type": "number",
                            "description": "Position along road in meters"
                        }
                    },
                    "required": ["road_id", "lane_id", "s"]
                }),
            },
            ToolDefinition {
                name: "get_real_world_road".to_string(),
                description: Some(
                    "Download and convert a real-world road from OpenStreetMap to OpenDRIVE. Returns road analysis with recommended spawn points. Automatically loads the road network for use with other tools.".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "Location name (e.g., 'nihonbashi', 'tokyo_station', 'ginza') or custom bbox 'lon1,lat1,lon2,lat2'"
                        },
                        "output_name": {
                            "type": "string",
                            "description": "Optional output file base name (defaults to location name)"
                        }
                    },
                    "required": ["location"]
                }),
            },
            ToolDefinition {
                name: "create_lane_change_scenario".to_string(),
                description: Some(
                    "Create a complete lane change scenario with ego vehicle and one other vehicle on a real road.".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "road_id": {"type": "string", "description": "Road ID"},
                        "lane_from": {"type": "number", "description": "Starting lane ID (negative for driving lanes)"},
                        "lane_to": {"type": "number", "description": "Target lane ID"},
                        "ego_start_s": {"type": "number", "description": "Ego start position (meters)"},
                        "other_start_s": {"type": "number", "description": "Other vehicle start position (meters)"},
                        "other_lane": {"type": "number", "description": "Other vehicle lane ID"},
                        "ego_speed": {"type": "number", "description": "Ego speed (m/s)"},
                        "other_speed": {"type": "number", "description": "Other vehicle speed (m/s)"},
                        "scenario_name": {"type": "string", "description": "Optional scenario name"}
                    },
                    "required": ["road_id", "lane_from", "lane_to", "ego_start_s", "other_start_s", "other_lane", "ego_speed", "other_speed"]
                }),
            },
            ToolDefinition {
                name: "create_cutin_scenario".to_string(),
                description: Some(
                    "Create a cut-in scenario where another vehicle cuts in front of ego vehicle.".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "road_id": {"type": "string"},
                        "ego_lane": {"type": "number"},
                        "other_lane": {"type": "number"},
                        "ego_start_s": {"type": "number"},
                        "other_start_s": {"type": "number"},
                        "ego_speed": {"type": "number"},
                        "other_speed": {"type": "number"},
                        "cutin_trigger_distance": {"type": "number", "description": "Distance that triggers cut-in (meters)"},
                        "scenario_name": {"type": "string"}
                    },
                    "required": ["road_id", "ego_lane", "other_lane", "ego_start_s", "other_start_s", "ego_speed", "other_speed", "cutin_trigger_distance"]
                }),
            },
            ToolDefinition {
                name: "create_platoon_scenario".to_string(),
                description: Some(
                    "Create a platoon/convoy scenario with multiple vehicles following in a line.".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "road_id": {"type": "string"},
                        "lane_id": {"type": "number"},
                        "vehicle_count": {"type": "number", "description": "Number of vehicles (2-10)"},
                        "start_s": {"type": "number", "description": "Starting position (meters)"},
                        "spacing": {"type": "number", "description": "Spacing between vehicles (meters)"},
                        "speed": {"type": "number", "description": "Convoy speed (m/s)"},
                        "scenario_name": {"type": "string"}
                    },
                    "required": ["road_id", "lane_id", "vehicle_count", "start_s", "spacing", "speed"]
                }),
            },
            ToolDefinition {
                name: "create_quick_scenario".to_string(),
                description: Some(
                    "Quick scenario generator! Creates a complete scenario on the best available road. Use after get_real_world_road. Types: 'lane_change', 'cutin', 'platoon'.".to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_type": {
                            "type": "string",
                            "enum": ["lane_change", "cutin", "platoon"],
                            "description": "Type of scenario to generate"
                        },
                        "vehicle_count": {
                            "type": "number",
                            "description": "Number of vehicles (optional, defaults to 3)"
                        }
                    },
                    "required": ["scenario_type"]
                }),
            },
            ToolDefinition {
                name: "list_scenarios".to_string(),
                description: Some("List all scenarios in the current session".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            ToolDefinition {
                name: "inspect_scenario".to_string(),
                description: Some("Inspect a scenario and return comprehensive JSON structure with entities, actions, triggers, conditions, timing, and parameter details".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID to inspect"
                        }
                    },
                    "required": ["scenario_id"]
                }),
            },
            ToolDefinition {
                name: "describe_scenario".to_string(),
                description: Some("Get a human-readable Markdown description of a scenario".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID to describe"
                        }
                    },
                    "required": ["scenario_id"]
                }),
            },
            ToolDefinition {
                name: "check_scenario".to_string(),
                description: Some("Check a scenario for completeness and get helpful suggestions (checks entities, positions, speeds, etc.)".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "scenario_id": {
                            "type": "string",
                            "description": "Scenario ID to validate"
                        }
                    },
                    "required": ["scenario_id"]
                }),
            },
            ToolDefinition {
                name: "import_scenario".to_string(),
                description: Some(
                    "Import an existing OpenSCENARIO .xosc file to inspect or modify it.\n\n\
                    Example:\n\
                    import_scenario(\n\
                        xosc_path='./scenarios/lane_change.xosc',\n\
                        scenario_name='my_import'  # optional, defaults to filename\n\
                    )\n\n\
                    After importing, use:\n\
                    - inspect_scenario() for full JSON structure\n\
                    - describe_scenario() for human-readable summary\n\
                    - check_scenario() for validation and suggestions\n\
                    - export_xml() to save modifications"
                        .to_string(),
                ),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "xosc_path": {
                            "type": "string",
                            "description": "Path to the .xosc file"
                        },
                        "scenario_name": {
                            "type": "string",
                            "description": "Optional name for the imported scenario (defaults to filename)"
                        }
                    },
                    "required": ["xosc_path"]
                }),
            },
        ]
    }

    /// List available MCP tools
    ///
    /// Note: Request parameter is optional for compatibility with different MCP clients.
    /// Some clients may not send a request body for the list_tools method.
    /// The parameter is currently unused as we return all tools without filtering.
    ///
    /// Future: Could support pagination/filtering via the request parameter.
    pub fn handle_list_tools(_req: Option<ListRequest>) -> Result<ToolsListResponse> {
        Ok(ToolsListResponse {
            tools: Self::register_tools(),
            next_cursor: None,
            meta: None,
        })
    }

    pub fn handle_call_tool(req: CallToolRequest) -> Result<CallToolResponse> {
        let name = req.name.as_str();
        let args = req.arguments.unwrap_or_default();

        match name {
            "create_scenario" => {
                let scenario_name = args
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'name' parameter"))?;
                let version = args
                    .get("version")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'version' parameter"))?;

                let result = handle_create_scenario(
                    GLOBAL_STATE.clone(),
                    scenario_name.to_string(),
                    version.to_string(),
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: format!("Created scenario with ID: {}", result),
                    }],
                    is_error: None,
                    meta: None,
                })
            }
            "add_vehicle" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let vehicle_name = args
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'name' parameter"))?;
                let category = args
                    .get("category")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'category' parameter"))?;
                let catalog = args
                    .get("catalog")
                    .and_then(Value::as_str)
                    .map(String::from);

                let result = handle_add_vehicle(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    vehicle_name.to_string(),
                    category.to_string(),
                    catalog,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: format!("Added vehicle: {}", result),
                    }],
                    is_error: None,
                    meta: None,
                })
            }
            "set_position" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let entity_name = args
                    .get("entity_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'entity_name' parameter"))?;
                let x = args
                    .get("x")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'x' parameter"))?;
                let y = args
                    .get("y")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'y' parameter"))?;
                let z = args
                    .get("z")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'z' parameter"))?;
                let h = args
                    .get("h")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'h' parameter"))?;

                let result = handle_set_position(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    entity_name.to_string(),
                    x,
                    y,
                    z,
                    h,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "add_pedestrian" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id'"))?;
                let name = args
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'name'"))?;
                let catalog = args
                    .get("catalog")
                    .and_then(Value::as_str)
                    .map(String::from);
                let mass = args.get("mass").and_then(Value::as_f64);
                let result = handle_add_pedestrian(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    name.to_string(),
                    catalog,
                    mass,
                )?;
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: format!("Added pedestrian: {}", result),
                    }],
                    is_error: None,
                    meta: None,
                })
            }
            "add_misc_object" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id'"))?;
                let name = args
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'name'"))?;
                let category = args
                    .get("category")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'category'"))?;
                let mass = args
                    .get("mass")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'mass'"))?;
                let result = handle_add_misc_object(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    name.to_string(),
                    category.to_string(),
                    mass,
                )?;
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: format!("Added misc object: {}", result),
                    }],
                    is_error: None,
                    meta: None,
                })
            }
            "set_lane_position" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let entity_name = args
                    .get("entity_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'entity_name' parameter"))?;
                let road_id = args
                    .get("road_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'road_id' parameter"))?;
                let lane_id = args
                    .get("lane_id")
                    .and_then(Value::as_i64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'lane_id' parameter"))?
                    as i32;
                let s = args
                    .get("s")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 's' parameter"))?;
                let offset = args.get("offset").and_then(Value::as_f64).unwrap_or(0.0);

                let result = handle_set_lane_position(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    entity_name.to_string(),
                    road_id.to_string(),
                    lane_id,
                    s,
                    offset,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "add_speed_action" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let entity_name = args
                    .get("entity_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'entity_name' parameter"))?;
                let story_name = args
                    .get("story_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'story_name' parameter"))?;
                let speed = args
                    .get("speed")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'speed' parameter"))?;
                let duration = args
                    .get("duration")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'duration' parameter"))?;
                let start_time = args.get("start_time").and_then(Value::as_f64);

                let result = handle_add_speed_action(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    entity_name.to_string(),
                    story_name.to_string(),
                    speed,
                    duration,
                    start_time,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "add_lane_change_action" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let entity_name = args
                    .get("entity_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'entity_name' parameter"))?;
                let story_name = args
                    .get("story_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'story_name' parameter"))?;
                let target_lane = args
                    .get("target_lane")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'target_lane' parameter"))?;
                let duration = args
                    .get("duration")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'duration' parameter"))?;
                let start_time = args.get("start_time").and_then(Value::as_f64);

                let result = handle_add_lane_change_action(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    entity_name.to_string(),
                    story_name.to_string(),
                    target_lane,
                    duration,
                    start_time,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "export_xml" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let output_path = args
                    .get("output_path")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'output_path' parameter"))?;

                let result = handle_export_xml(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    output_path.to_string(),
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "validate_scenario" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;

                let result =
                    handle_validate_scenario(GLOBAL_STATE.clone(), scenario_id.to_string())?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "validate_scenario_structure" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let auto_fix = args
                    .get("auto_fix")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);

                let result = handle_validate_scenario_structure(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    auto_fix,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "set_stop_time" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let seconds = args
                    .get("seconds")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'seconds' parameter"))?;

                let result =
                    handle_set_stop_time(GLOBAL_STATE.clone(), scenario_id.to_string(), seconds)?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "set_stop_on_element" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let element_type = args
                    .get("element_type")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'element_type' parameter"))?;
                let element_ref = args
                    .get("element_ref")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'element_ref' parameter"))?;
                let state = args
                    .get("state")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'state' parameter"))?;
                let delay = args
                    .get("delay")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'delay' parameter"))?;

                let result = handle_set_stop_on_element(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    element_type.to_string(),
                    element_ref.to_string(),
                    state.to_string(),
                    delay,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "set_trigger_time" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let element_type = args
                    .get("element_type")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'element_type' parameter"))?;
                let story_name = args
                    .get("story_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'story_name' parameter"))?;
                let act_name = args
                    .get("act_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'act_name' parameter"))?;
                let time_seconds = args
                    .get("time_seconds")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'time_seconds' parameter"))?;
                let delay_seconds = args.get("delay_seconds").and_then(Value::as_f64);

                // Optional params for Event triggers
                let maneuver_group = args
                    .get("maneuver_group")
                    .and_then(Value::as_str)
                    .map(String::from);
                let maneuver = args
                    .get("maneuver")
                    .and_then(Value::as_str)
                    .map(String::from);
                let event_name = args
                    .get("event_name")
                    .and_then(Value::as_str)
                    .map(String::from);

                let result = handle_set_trigger_time(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    element_type.to_string(),
                    story_name.to_string(),
                    act_name.to_string(),
                    maneuver_group,
                    maneuver,
                    event_name,
                    time_seconds,
                    delay_seconds,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "set_collision_trigger" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let element_type = args
                    .get("element_type")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'element_type' parameter"))?;
                let story_name = args
                    .get("story_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'story_name' parameter"))?;
                let act_name = args
                    .get("act_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'act_name' parameter"))?;
                let entity_refs = args
                    .get("entity_refs")
                    .and_then(Value::as_array)
                    .ok_or_else(|| anyhow!("Missing or invalid 'entity_refs' parameter"))?
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect::<Vec<String>>();
                let target_entity = args
                    .get("target_entity")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'target_entity' parameter"))?;
                let trigger_rule = args
                    .get("trigger_rule")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'trigger_rule' parameter"))?;
                let delay_seconds = args.get("delay_seconds").and_then(Value::as_f64);

                // Optional params for Event triggers
                let maneuver_group = args
                    .get("maneuver_group")
                    .and_then(Value::as_str)
                    .map(String::from);
                let maneuver = args
                    .get("maneuver")
                    .and_then(Value::as_str)
                    .map(String::from);
                let event_name = args
                    .get("event_name")
                    .and_then(Value::as_str)
                    .map(String::from);

                let result = handle_set_collision_trigger(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    element_type.to_string(),
                    story_name.to_string(),
                    act_name.to_string(),
                    maneuver_group,
                    maneuver,
                    event_name,
                    entity_refs,
                    target_entity.to_string(),
                    trigger_rule.to_string(),
                    delay_seconds,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "list_triggers" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let element_type = args
                    .get("element_type")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'element_type' parameter"))?;
                let act_name = args
                    .get("act_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'act_name' parameter"))?;

                // Optional params
                let story_name = args
                    .get("story_name")
                    .and_then(Value::as_str)
                    .map(String::from);
                let maneuver_group = args
                    .get("maneuver_group")
                    .and_then(Value::as_str)
                    .map(String::from);
                let maneuver = args
                    .get("maneuver")
                    .and_then(Value::as_str)
                    .map(String::from);
                let event_name = args
                    .get("event_name")
                    .and_then(Value::as_str)
                    .map(String::from);

                let result = handle_list_triggers(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    element_type.to_string(),
                    act_name.to_string(),
                    story_name,
                    maneuver_group,
                    maneuver,
                    event_name,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "load_road_network" => {
                let xodr_path = args
                    .get("xodr_path")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'xodr_path' parameter"))?;

                let result = handle_load_road_network(GLOBAL_STATE.clone(), xodr_path.to_string())?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "list_roads" => {
                let result = handle_list_roads(GLOBAL_STATE.clone())?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "get_road_info" => {
                let road_id = args
                    .get("road_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'road_id' parameter"))?;

                let result = handle_get_road_info(GLOBAL_STATE.clone(), road_id.to_string())?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "suggest_spawn_points" => {
                let road_id = args
                    .get("road_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'road_id' parameter"))?;
                let count = args
                    .get("count")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'count' parameter"))?
                    as usize;

                let result =
                    handle_suggest_spawn_points(GLOBAL_STATE.clone(), road_id.to_string(), count)?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "validate_position" => {
                let road_id = args
                    .get("road_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'road_id' parameter"))?;
                let lane_id = args
                    .get("lane_id")
                    .and_then(Value::as_i64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'lane_id' parameter"))?
                    as i32;
                let s = args
                    .get("s")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 's' parameter"))?;

                let result = handle_validate_position(
                    GLOBAL_STATE.clone(),
                    road_id.to_string(),
                    lane_id,
                    s,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "get_real_world_road" => {
                let location = args
                    .get("location")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'location' parameter"))?;
                let output_name = args
                    .get("output_name")
                    .and_then(Value::as_str)
                    .map(|s| s.to_string());

                let result = handle_get_real_world_road(
                    GLOBAL_STATE.clone(),
                    location.to_string(),
                    output_name,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "create_lane_change_scenario" => {
                let road_id = args
                    .get("road_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'road_id'"))?;
                let lane_from =
                    args.get("lane_from")
                        .and_then(Value::as_i64)
                        .ok_or_else(|| anyhow!("Missing 'lane_from'"))? as i32;
                let lane_to =
                    args.get("lane_to")
                        .and_then(Value::as_i64)
                        .ok_or_else(|| anyhow!("Missing 'lane_to'"))? as i32;
                let ego_start_s = args
                    .get("ego_start_s")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'ego_start_s'"))?;
                let other_start_s = args
                    .get("other_start_s")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'other_start_s'"))?;
                let other_lane =
                    args.get("other_lane")
                        .and_then(Value::as_i64)
                        .ok_or_else(|| anyhow!("Missing 'other_lane'"))? as i32;
                let ego_speed = args
                    .get("ego_speed")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'ego_speed'"))?;
                let other_speed = args
                    .get("other_speed")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'other_speed'"))?;
                let scenario_name = args
                    .get("scenario_name")
                    .and_then(Value::as_str)
                    .map(|s| s.to_string());

                let result = handle_create_lane_change_scenario(
                    GLOBAL_STATE.clone(),
                    road_id.to_string(),
                    lane_from,
                    lane_to,
                    ego_start_s,
                    other_start_s,
                    other_lane,
                    ego_speed,
                    other_speed,
                    scenario_name,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "create_cutin_scenario" => {
                let road_id = args
                    .get("road_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'road_id'"))?;
                let ego_lane =
                    args.get("ego_lane")
                        .and_then(Value::as_i64)
                        .ok_or_else(|| anyhow!("Missing 'ego_lane'"))? as i32;
                let other_lane =
                    args.get("other_lane")
                        .and_then(Value::as_i64)
                        .ok_or_else(|| anyhow!("Missing 'other_lane'"))? as i32;
                let ego_start_s = args
                    .get("ego_start_s")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'ego_start_s'"))?;
                let other_start_s = args
                    .get("other_start_s")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'other_start_s'"))?;
                let ego_speed = args
                    .get("ego_speed")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'ego_speed'"))?;
                let other_speed = args
                    .get("other_speed")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'other_speed'"))?;
                let cutin_trigger_distance = args
                    .get("cutin_trigger_distance")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'cutin_trigger_distance'"))?;
                let scenario_name = args
                    .get("scenario_name")
                    .and_then(Value::as_str)
                    .map(|s| s.to_string());

                let result = handle_create_cutin_scenario(
                    GLOBAL_STATE.clone(),
                    road_id.to_string(),
                    ego_lane,
                    other_lane,
                    ego_start_s,
                    other_start_s,
                    ego_speed,
                    other_speed,
                    cutin_trigger_distance,
                    scenario_name,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "create_platoon_scenario" => {
                let road_id = args
                    .get("road_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'road_id'"))?;
                let lane_id =
                    args.get("lane_id")
                        .and_then(Value::as_i64)
                        .ok_or_else(|| anyhow!("Missing 'lane_id'"))? as i32;
                let vehicle_count = args
                    .get("vehicle_count")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| anyhow!("Missing 'vehicle_count'"))?
                    as usize;
                let start_s = args
                    .get("start_s")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'start_s'"))?;
                let spacing = args
                    .get("spacing")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'spacing'"))?;
                let speed = args
                    .get("speed")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing 'speed'"))?;
                let scenario_name = args
                    .get("scenario_name")
                    .and_then(Value::as_str)
                    .map(|s| s.to_string());

                let result = handle_create_platoon_scenario(
                    GLOBAL_STATE.clone(),
                    road_id.to_string(),
                    lane_id,
                    vehicle_count,
                    start_s,
                    spacing,
                    speed,
                    scenario_name,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "create_quick_scenario" => {
                let scenario_type = args
                    .get("scenario_type")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_type'"))?;
                let vehicle_count = args
                    .get("vehicle_count")
                    .and_then(Value::as_u64)
                    .map(|n| n as usize);

                let result = handle_create_quick_scenario(
                    GLOBAL_STATE.clone(),
                    scenario_type.to_string(),
                    vehicle_count,
                )?;

                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "list_scenarios" => {
                let result = crate::inspection::handle_list_scenarios(GLOBAL_STATE.clone())?;
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "inspect_scenario" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id'"))?;
                let result = crate::inspection::handle_inspect_scenario(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                )?;
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "describe_scenario" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id'"))?;
                let result = crate::inspection::handle_describe_scenario(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                )?;
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "check_scenario" => {
                let scenario_id = args
                    .get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id'"))?;
                let result = crate::inspection::handle_check_scenario(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                )?;
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            "import_scenario" => {
                let xosc_path = args
                    .get("xosc_path")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'xosc_path'"))?;
                let scenario_name = args
                    .get("scenario_name")
                    .and_then(Value::as_str)
                    .map(String::from);
                let result = crate::import::handle_import_scenario(
                    GLOBAL_STATE.clone(),
                    xosc_path.to_string(),
                    scenario_name,
                )?;
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text { text: result }],
                    is_error: None,
                    meta: None,
                })
            }
            _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
        }
    }
}
