use mcp_sdk::types::{
    CallToolRequest, CallToolResponse, ListRequest,
    ToolDefinition, ToolResponseContent, ToolsListResponse,
};
use serde_json::{json, Value};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use openscenario::Scenario;
use crate::handlers::{handle_create_scenario, handle_add_vehicle, handle_set_position,
                      handle_add_speed_action, handle_add_lane_change_action, handle_export_xml, handle_validate_scenario};

// Global server state
static GLOBAL_STATE: Lazy<Arc<Mutex<ServerState>>> = Lazy::new(|| {
    Arc::new(Mutex::new(ServerState::new()))
});

pub struct ServerState {
    pub scenarios: HashMap<String, Scenario>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            scenarios: HashMap::new(),
        }
    }
}

pub struct OpenScenarioServer;

impl OpenScenarioServer {
    pub fn new() -> Self {
        Self
    }
    
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
                name: "add_speed_action".to_string(),
                description: Some("Add a speed action to a scenario".to_string()),
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
                        }
                    },
                    "required": ["scenario_id", "entity_name", "story_name", "speed", "duration"]
                }),
            },
            ToolDefinition {
                name: "add_lane_change_action".to_string(),
                description: Some("Add a lane change action to a scenario".to_string()),
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
                description: Some("Validate a scenario against OpenSCENARIO XSD schema".to_string()),
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
        ]
    }
    
    pub fn handle_list_tools(_req: ListRequest) -> Result<ToolsListResponse> {
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
                let scenario_name = args.get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'name' parameter"))?;
                let version = args.get("version")
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
                let scenario_id = args.get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let vehicle_name = args.get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'name' parameter"))?;
                let category = args.get("category")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'category' parameter"))?;
                let catalog = args.get("catalog")
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
                let scenario_id = args.get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let entity_name = args.get("entity_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'entity_name' parameter"))?;
                let x = args.get("x")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'x' parameter"))?;
                let y = args.get("y")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'y' parameter"))?;
                let z = args.get("z")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'z' parameter"))?;
                let h = args.get("h")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'h' parameter"))?;
                
                let result = handle_set_position(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    entity_name.to_string(),
                    x, y, z, h,
                )?;
                
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: result,
                    }],
                    is_error: None,
                    meta: None,
                })
            }
            "add_speed_action" => {
                let scenario_id = args.get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let entity_name = args.get("entity_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'entity_name' parameter"))?;
                let story_name = args.get("story_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'story_name' parameter"))?;
                let speed = args.get("speed")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'speed' parameter"))?;
                let duration = args.get("duration")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'duration' parameter"))?;
                
                let result = handle_add_speed_action(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    entity_name.to_string(),
                    story_name.to_string(),
                    speed,
                    duration,
                )?;
                
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: result,
                    }],
                    is_error: None,
                    meta: None,
                })
            }
            "add_lane_change_action" => {
                let scenario_id = args.get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let entity_name = args.get("entity_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'entity_name' parameter"))?;
                let story_name = args.get("story_name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'story_name' parameter"))?;
                let target_lane = args.get("target_lane")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'target_lane' parameter"))?;
                let duration = args.get("duration")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| anyhow!("Missing or invalid 'duration' parameter"))?;
                
                let result = handle_add_lane_change_action(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    entity_name.to_string(),
                    story_name.to_string(),
                    target_lane,
                    duration,
                )?;
                
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: result,
                    }],
                    is_error: None,
                    meta: None,
                })
            }
            "export_xml" => {
                let scenario_id = args.get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                let output_path = args.get("output_path")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'output_path' parameter"))?;
                
                let result = handle_export_xml(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                    output_path.to_string(),
                )?;
                
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: result,
                    }],
                    is_error: None,
                    meta: None,
                })
            }
            "validate_scenario" => {
                let scenario_id = args.get("scenario_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow!("Missing 'scenario_id' parameter"))?;
                
                let result = handle_validate_scenario(
                    GLOBAL_STATE.clone(),
                    scenario_id.to_string(),
                )?;
                
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: result,
                    }],
                    is_error: None,
                    meta: None,
                })
            }
            _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
        }
    }
}
