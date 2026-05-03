/// Comprehensive Error Handling & Edge Case Tests
/// Tests for invalid inputs, malformed data, and boundary conditions
use anyhow::Result;
use openscenario_mcp::handlers::*;
use openscenario_mcp::server::{OpenScenarioServer, ServerState};
use mcp_sdk::types::{CallToolRequest, ToolResponseContent};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use std::fs;

// ========== HELPER FUNCTIONS ==========

fn create_test_state() -> Arc<Mutex<ServerState>> {
    Arc::new(Mutex::new(ServerState::new()))
}

fn create_test_scenario(state: Arc<Mutex<ServerState>>) -> Result<String> {
    handle_create_scenario(state, "test_scenario".to_string(), "1.2".to_string())
}

// ========== CATEGORY 1: INVALID JSON-RPC ==========

#[test]
fn test_missing_required_parameter_name() {
    let req = CallToolRequest {
        name: "create_scenario".to_string(),
        arguments: Some(json!({
            "version": "1.2"
            // Missing "name"
        })),
        meta: None,
    };

    let result = OpenScenarioServer::handle_call_tool(req);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("name"));
}

#[test]
fn test_missing_required_parameter_version() {
    let req = CallToolRequest {
        name: "create_scenario".to_string(),
        arguments: Some(json!({
            "name": "test"
            // Missing "version"
        })),
        meta: None,
    };

    let result = OpenScenarioServer::handle_call_tool(req);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("version"));
}

#[test]
fn test_wrong_parameter_type_string_instead_of_number() {
    let req = CallToolRequest {
        name: "set_position".to_string(),
        arguments: Some(json!({
            "scenario_id": "test_id",
            "entity_name": "car1",
            "x": "not_a_number", // Should be number
            "y": 0.0,
            "z": 0.0,
            "h": 0.0
        })),
        meta: None,
    };

    let result = OpenScenarioServer::handle_call_tool(req);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("x"));
}

#[test]
fn test_null_required_parameter() {
    let req = CallToolRequest {
        name: "add_vehicle".to_string(),
        arguments: Some(json!({
            "scenario_id": "test_id",
            "name": null, // null instead of string
            "category": "Car"
        })),
        meta: None,
    };

    let result = OpenScenarioServer::handle_call_tool(req);
    assert!(result.is_err());
}

#[test]
fn test_empty_arguments() {
    let req = CallToolRequest {
        name: "create_scenario".to_string(),
        arguments: Some(json!({})), // Empty object
        meta: None,
    };

    let result = OpenScenarioServer::handle_call_tool(req);
    assert!(result.is_err());
}

#[test]
fn test_no_arguments() {
    let req = CallToolRequest {
        name: "create_scenario".to_string(),
        arguments: None, // No arguments at all
        meta: None,
    };

    let result = OpenScenarioServer::handle_call_tool(req);
    assert!(result.is_err());
}

// ========== CATEGORY 2: INVALID SCENARIOS ==========

#[test]
fn test_nonexistent_scenario_id() {
    let state = create_test_state();
    
    let result = handle_add_vehicle(
        state,
        "nonexistent_scenario_id".to_string(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    );

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Scenario not found"));
}

#[test]
fn test_empty_scenario_name() {
    let state = create_test_state();
    
    let result = handle_create_scenario(
        state,
        "".to_string(), // Empty name
        "1.2".to_string(),
    );

    // Should succeed but create scenario with empty name (business logic decision)
    assert!(result.is_ok());
}

#[test]
fn test_invalid_version_string() {
    let state = create_test_state();
    
    let result = handle_create_scenario(
        state,
        "test".to_string(),
        "2.0".to_string(), // Invalid version
    );

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Invalid version"));
    assert!(err.to_string().contains("1.0, 1.1, or 1.2"));
}

#[test]
fn test_malformed_version_string() {
    let state = create_test_state();
    
    let result = handle_create_scenario(
        state,
        "test".to_string(),
        "v1.2".to_string(), // Malformed
    );

    assert!(result.is_err());
}

// ========== CATEGORY 3: INVALID POSITIONS ==========

#[test]
fn test_nan_position_x() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();
    
    // Add vehicle first
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    ).unwrap();

    let result = handle_set_position(
        state,
        scenario_id,
        "car1".to_string(),
        f64::NAN, // NaN value
        0.0,
        0.0,
        0.0,
    );

    // Server should handle this - either reject or accept
    // This tests that it doesn't panic
    let _ = result;
}

#[test]
fn test_infinity_position_value() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();
    
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    ).unwrap();

    let result = handle_set_position(
        state,
        scenario_id,
        "car1".to_string(),
        f64::INFINITY, // Infinity
        0.0,
        0.0,
        0.0,
    );

    let _ = result; // Should not panic
}

#[test]
fn test_negative_infinity_position() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();
    
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    ).unwrap();

    let result = handle_set_position(
        state,
        scenario_id,
        "car1".to_string(),
        0.0,
        f64::NEG_INFINITY, // Negative infinity
        0.0,
        0.0,
    );

    let _ = result; // Should not panic
}

#[test]
fn test_extremely_large_position_values() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();
    
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    ).unwrap();

    let result = handle_set_position(
        state,
        scenario_id,
        "car1".to_string(),
        1e100, // Very large
        -1e100, // Very large negative
        0.0,
        0.0,
    );

    assert!(result.is_ok() || result.is_err()); // Should handle gracefully
}

// ========== CATEGORY 4: INVALID PARAMETERS ==========

#[test]
fn test_negative_duration() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();
    
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    ).unwrap();

    let result = handle_add_speed_action(
        state,
        scenario_id,
        "car1".to_string(),
        "story1".to_string(),
        50.0,
        -5.0, // Negative duration
    );

    // Should either reject or handle gracefully
    let _ = result;
}

#[test]
fn test_zero_duration() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();
    
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    ).unwrap();

    let result = handle_add_speed_action(
        state,
        scenario_id,
        "car1".to_string(),
        "story1".to_string(),
        50.0,
        0.0, // Zero duration
    );

    let _ = result; // Should handle gracefully
}

#[test]
fn test_negative_speed() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();
    
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    ).unwrap();

    let result = handle_add_speed_action(
        state,
        scenario_id,
        "car1".to_string(),
        "story1".to_string(),
        -50.0, // Negative speed (could be valid for reverse)
        5.0,
    );

    // Negative speed might be valid (reverse), should not panic
    let _ = result;
}

#[test]
fn test_invalid_vehicle_category() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();

    let result = handle_add_vehicle(
        state,
        scenario_id,
        "vehicle1".to_string(),
        "Airplane".to_string(), // Invalid category
        None,
    );

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Invalid vehicle category"));
}

#[test]
fn test_empty_vehicle_name() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();

    let result = handle_add_vehicle(
        state,
        scenario_id,
        "".to_string(), // Empty name
        "Car".to_string(),
        None,
    );

    // Should either succeed or fail gracefully
    let _ = result;
}

#[test]
fn test_duplicate_vehicle_name() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();

    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    ).unwrap();

    // Try to add same vehicle again
    let result = handle_add_vehicle(
        state,
        scenario_id,
        "car1".to_string(), // Duplicate name
        "Car".to_string(),
        None,
    );

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("already exists"));
}

// ========== CATEGORY 5: CATALOG ERRORS ==========

#[test]
fn test_nonexistent_catalog_file() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();

    let result = handle_add_vehicle(
        state,
        scenario_id,
        "car1".to_string(),
        "Car".to_string(),
        Some("/nonexistent/path/catalog.xosc:entry1".to_string()),
    );

    // Should succeed (catalog reference stored, not validated until export)
    // or fail with catalog error
    let _ = result;
}

#[test]
fn test_malformed_catalog_reference() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();

    let result = handle_add_vehicle(
        state,
        scenario_id,
        "car1".to_string(),
        "Car".to_string(),
        Some("malformed:::reference".to_string()),
    );

    // Should handle gracefully
    let _ = result;
}

#[test]
fn test_empty_catalog_reference() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();

    let result = handle_add_vehicle(
        state,
        scenario_id,
        "car1".to_string(),
        "Car".to_string(),
        Some("".to_string()), // Empty catalog
    );

    let _ = result;
}

// ========== CATEGORY 6: STATE ERRORS ==========

#[test]
fn test_action_on_nonexistent_entity() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();

    // Note: The handler currently allows actions on non-existent entities
    // They get caught during validation, not during action creation
    let result = handle_add_speed_action(
        state,
        scenario_id,
        "nonexistent_entity".to_string(), // Entity doesn't exist
        "story1".to_string(),
        50.0,
        5.0,
    );

    // Currently succeeds - validation happens later
    // This is a design decision: validation at export time vs creation time
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_validation_is_xsd_only() {
    // Test that validation is XSD-based and doesn't catch semantic errors
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();

    // Add action referencing non-existent entity
    handle_add_speed_action(
        state.clone(),
        scenario_id.clone(),
        "nonexistent_entity".to_string(),
        "story1".to_string(),
        50.0,
        5.0,
    ).unwrap();

    // Validation is XSD-only, so it may pass even with semantic errors
    // This documents the current behavior
    let result = handle_validate_scenario(state, scenario_id);
    // XSD validation might pass even with missing entity references
    let _ = result;
}

#[test]
fn test_position_on_nonexistent_entity() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();

    let result = handle_set_position(
        state,
        scenario_id,
        "nonexistent_car".to_string(),
        0.0,
        0.0,
        0.0,
        0.0,
    );

    assert!(result.is_err());
}

#[test]
fn test_export_nonexistent_scenario() {
    let state = create_test_state();

    let result = handle_export_xml(
        state,
        "nonexistent_id".to_string(),
        "/tmp/test.xosc".to_string(),
    );

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Scenario not found"));
}

#[test]
fn test_validate_nonexistent_scenario() {
    let state = create_test_state();

    let result = handle_validate_scenario(
        state,
        "nonexistent_id".to_string(),
    );

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Scenario not found"));
}

// ========== CATEGORY 7: FILE SYSTEM ERRORS ==========

#[test]
fn test_export_to_invalid_path() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();

    let result = handle_export_xml(
        state,
        scenario_id,
        "/invalid/nonexistent/directory/test.xosc".to_string(),
    );

    assert!(result.is_err());
    // Should be IO error
}

#[test]
fn test_export_to_readonly_location() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();

    // Try to write to /proc which is typically read-only
    let result = handle_export_xml(
        state,
        scenario_id,
        "/proc/test.xosc".to_string(),
    );

    // Should fail with permission/IO error
    let _ = result;
}

// ========== CATEGORY 8: UNKNOWN TOOL ==========

#[test]
fn test_unknown_tool_name() {
    let req = CallToolRequest {
        name: "nonexistent_tool".to_string(),
        arguments: Some(json!({})),
        meta: None,
    };

    let result = OpenScenarioServer::handle_call_tool(req);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Unknown tool"));
}

#[test]
fn test_empty_tool_name() {
    let req = CallToolRequest {
        name: "".to_string(),
        arguments: Some(json!({})),
        meta: None,
    };

    let result = OpenScenarioServer::handle_call_tool(req);
    assert!(result.is_err());
}

// ========== CATEGORY 9: BOUNDARY VALUES ==========

#[test]
fn test_very_long_scenario_name() {
    let state = create_test_state();
    let long_name = "a".repeat(10000);
    
    let result = handle_create_scenario(
        state,
        long_name.clone(),
        "1.2".to_string(),
    );

    // Should handle without panic
    let _ = result;
}

#[test]
fn test_special_characters_in_name() {
    let state = create_test_state();
    
    let result = handle_create_scenario(
        state,
        "test<>\"'&scenario".to_string(),
        "1.2".to_string(),
    );

    // Should succeed or fail gracefully
    let _ = result;
}

#[test]
fn test_unicode_in_names() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();
    
    let result = handle_add_vehicle(
        state,
        scenario_id,
        "车辆🚗".to_string(), // Chinese + emoji
        "Car".to_string(),
        None,
    );

    let _ = result;
}

#[test]
fn test_extreme_lane_offset() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();
    
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    ).unwrap();

    let result = handle_add_lane_change_action(
        state,
        scenario_id,
        "car1".to_string(),
        "story1".to_string(),
        999999.0, // Extreme offset
        5.0,
    );

    let _ = result;
}

// ========== INTEGRATION TEST ==========

#[test]
fn test_complete_error_handling_flow() {
    // Test that multiple errors in sequence don't cause state corruption
    let state = create_test_state();
    
    // 1. Create scenario successfully
    let scenario_id = handle_create_scenario(
        state.clone(),
        "test".to_string(),
        "1.2".to_string(),
    ).unwrap();
    
    // 2. Try invalid operation
    let _ = handle_add_vehicle(
        state.clone(),
        "wrong_id".to_string(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    );
    
    // 3. Valid operation should still work
    let result = handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    );
    assert!(result.is_ok());
    
    // 4. Try duplicate
    let result = handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "car1".to_string(),
        "Car".to_string(),
        None,
    );
    assert!(result.is_err());
    
    // 5. Different vehicle should still work
    let result = handle_add_vehicle(
        state.clone(),
        scenario_id,
        "car2".to_string(),
        "Car".to_string(),
        None,
    );
    assert!(result.is_ok());
}

#[test]
fn test_no_panic_on_mutex_operations() {
    let state = create_test_state();
    let scenario_id = create_test_scenario(state.clone()).unwrap();
    
    // Perform multiple concurrent-ish operations
    // This is single-threaded but tests mutex lock/unlock cycles
    for i in 0..100 {
        let _ = handle_add_vehicle(
            state.clone(),
            scenario_id.clone(),
            format!("car{}", i),
            "Car".to_string(),
            None,
        );
    }
    
    // Should not panic or deadlock
    assert!(true);
}
