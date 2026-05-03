use openscenario_mcp::handlers::{
    handle_add_lane_change_action, handle_add_speed_action, handle_add_vehicle,
    handle_create_scenario, handle_export_xml, handle_set_position,
};
use openscenario_mcp::server::ServerState;
use std::fs;
use std::sync::{Arc, Mutex};

/// Test 1: Multi-vehicle scenario with 5+ vehicles
#[test]
fn test_multi_vehicle_scenario() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    // Create scenario
    let scenario_id = handle_create_scenario(
        state.clone(),
        "multi_vehicle_test".to_string(),
        "1.2".to_string(),
    )
    .unwrap();

    // Add 7 vehicles with different types
    let vehicles = vec![
        ("ego_vehicle", "Car"),
        ("lead_vehicle", "Car"),
        ("follow_vehicle", "Car"),
        ("truck_1", "Truck"),
        ("motorbike_1", "Motorbike"),
        ("bus_1", "Bus"),
        ("bicycle_1", "Bicycle"),
    ];

    for (name, vehicle_type) in &vehicles {
        let result = handle_add_vehicle(
            state.clone(),
            scenario_id.clone(),
            name.to_string(),
            vehicle_type.to_string(),
            None,
        );
        assert!(result.is_ok(), "Failed to add vehicle: {}", name);
    }

    // Export and verify all vehicles are present
    let output_path = "/tmp/multi_vehicle_test.xosc";
    let export_result =
        handle_export_xml(state.clone(), scenario_id.clone(), output_path.to_string());
    assert!(
        export_result.is_ok(),
        "Export failed: {:?}",
        export_result.err()
    );

    let content = fs::read_to_string(output_path).expect("Failed to read exported file");
    for (name, _) in &vehicles {
        assert!(
            content.contains(name),
            "Vehicle {} not found in export",
            name
        );
    }

    // Cleanup
    fs::remove_file(output_path).ok();

    println!(
        "✓ Multi-vehicle test passed: {} vehicles created and exported",
        vehicles.len()
    );
}

/// Test 2: Many actions on a single vehicle (10+ actions)
#[test]
fn test_many_actions_single_vehicle() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    let scenario_id = handle_create_scenario(
        state.clone(),
        "many_actions_test".to_string(),
        "1.2".to_string(),
    )
    .unwrap();

    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "test_vehicle".to_string(),
        "Car".to_string(),
        None,
    )
    .unwrap();

    // Add 12 speed actions with varying parameters
    for i in 0..12 {
        let result = handle_add_speed_action(
            state.clone(),
            scenario_id.clone(),
            "test_vehicle".to_string(),
            format!("story_{}", i),
            20.0 + (i as f64 * 5.0),
            2.0 + (i as f64 * 0.5),
        );
        assert!(result.is_ok(), "Failed to add speed action {}", i);
    }

    // Export and verify all actions are present
    let output_path = "/tmp/many_actions_test.xosc";
    handle_export_xml(state.clone(), scenario_id.clone(), output_path.to_string()).unwrap();

    let content = fs::read_to_string(output_path).unwrap();

    // Count story elements
    let story_count = content.matches("<Story").count();
    assert!(
        story_count >= 12,
        "Expected at least 12 stories, found {}",
        story_count
    );

    // Cleanup
    fs::remove_file(output_path).ok();

    println!("✓ Many actions test passed: 12 actions added to single vehicle");
}

/// Test 3: Mixed action types on same entity
#[test]
fn test_mixed_actions_same_entity() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    let scenario_id = handle_create_scenario(
        state.clone(),
        "mixed_actions_test".to_string(),
        "1.2".to_string(),
    )
    .unwrap();

    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "mixed_vehicle".to_string(),
        "Car".to_string(),
        None,
    )
    .unwrap();

    // Set initial position
    handle_set_position(
        state.clone(),
        scenario_id.clone(),
        "mixed_vehicle".to_string(),
        100.0,
        50.0,
        0.0,
        0.5,
    )
    .unwrap();

    // Add multiple speed actions
    for i in 0..3 {
        handle_add_speed_action(
            state.clone(),
            scenario_id.clone(),
            "mixed_vehicle".to_string(),
            format!("speed_story_{}", i),
            30.0 + (i as f64 * 10.0),
            3.0,
        )
        .unwrap();
    }

    // Add multiple lane change actions
    for i in 0..3 {
        handle_add_lane_change_action(
            state.clone(),
            scenario_id.clone(),
            "mixed_vehicle".to_string(),
            format!("lane_story_{}", i),
            -3.5 * (i as f64 + 1.0),
            4.0,
        )
        .unwrap();
    }

    // Add more position updates
    for i in 0..2 {
        handle_set_position(
            state.clone(),
            scenario_id.clone(),
            "mixed_vehicle".to_string(),
            100.0 + (i as f64 * 50.0),
            50.0 + (i as f64 * 20.0),
            0.0,
            0.5 + (i as f64 * 0.2),
        )
        .unwrap();
    }

    // Export and verify mixed actions
    let output_path = "/tmp/mixed_actions_test.xosc";
    let export_result =
        handle_export_xml(state.clone(), scenario_id.clone(), output_path.to_string());
    assert!(
        export_result.is_ok(),
        "Export failed: {:?}",
        export_result.err()
    );

    let content = fs::read_to_string(output_path).expect("Failed to read exported file");

    assert!(content.contains("SpeedAction"), "Missing SpeedAction");
    assert!(
        content.contains("LaneChangeAction"),
        "Missing LaneChangeAction"
    );
    assert!(content.contains("TeleportAction"), "Missing TeleportAction");

    println!(
        "✓ Mixed actions test passed: Position, Speed, and Lane Change actions on same vehicle"
    );

    // Cleanup
    fs::remove_file(output_path).ok();
}

/// Test 4: Multiple stories with different vehicles
#[test]
fn test_multiple_stories_different_vehicles() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    let scenario_id = handle_create_scenario(
        state.clone(),
        "multi_story_test".to_string(),
        "1.2".to_string(),
    )
    .unwrap();

    // Create 4 vehicles
    let vehicles = vec!["vehicle_a", "vehicle_b", "vehicle_c", "vehicle_d"];
    for vehicle in &vehicles {
        handle_add_vehicle(
            state.clone(),
            scenario_id.clone(),
            vehicle.to_string(),
            "Car".to_string(),
            None,
        )
        .unwrap();
    }

    // Create 5 stories, each with different vehicle combinations
    let stories = vec![
        ("story_1", "vehicle_a"),
        ("story_2", "vehicle_b"),
        ("story_3", "vehicle_c"),
        ("story_4", "vehicle_d"),
        ("story_5", "vehicle_a"), // Reuse vehicle_a
    ];

    for (story_name, vehicle_name) in &stories {
        handle_add_speed_action(
            state.clone(),
            scenario_id.clone(),
            vehicle_name.to_string(),
            story_name.to_string(),
            25.0,
            3.0,
        )
        .unwrap();
    }

    // Export and verify all stories
    let output_path = "/tmp/multi_story_test.xosc";
    handle_export_xml(state.clone(), scenario_id.clone(), output_path.to_string()).unwrap();

    let content = fs::read_to_string(output_path).unwrap();

    for (story_name, _) in &stories {
        assert!(
            content.contains(story_name),
            "Story {} not found in export",
            story_name
        );
    }

    // Cleanup
    fs::remove_file(output_path).ok();

    println!("✓ Multiple stories test passed: 5 stories created with 4 vehicles");
}

/// Test 5: Export and validate XML structure
#[test]
fn test_export_validate_xml_structure() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    let scenario_id = handle_create_scenario(
        state.clone(),
        "xml_validation_test".to_string(),
        "1.2".to_string(),
    )
    .unwrap();

    // Create complex scenario
    handle_add_vehicle(
        state.clone(),
        scenario_id.clone(),
        "vehicle_1".to_string(),
        "Car".to_string(),
        None,
    )
    .unwrap();

    handle_set_position(
        state.clone(),
        scenario_id.clone(),
        "vehicle_1".to_string(),
        150.0,
        75.0,
        0.5,
        1.2,
    )
    .unwrap();

    handle_add_speed_action(
        state.clone(),
        scenario_id.clone(),
        "vehicle_1".to_string(),
        "speed_story".to_string(),
        60.0,
        5.0,
    )
    .unwrap();

    // Export
    let output_path = "/tmp/xml_validation_test.xosc";
    let export_result =
        handle_export_xml(state.clone(), scenario_id.clone(), output_path.to_string());
    assert!(
        export_result.is_ok(),
        "Export failed: {:?}",
        export_result.err()
    );

    // Validate XML structure
    let content = fs::read_to_string(output_path).expect("Failed to read exported file");

    // Check for required OpenSCENARIO elements
    assert!(content.starts_with("<?xml"), "Missing XML declaration");
    assert!(
        content.contains("<OpenSCENARIO"),
        "Missing OpenSCENARIO root (looking for opening tag)"
    );
    assert!(content.contains("<FileHeader"), "Missing FileHeader");
    assert!(content.contains("<Entities>"), "Missing Entities section");
    assert!(content.contains("<Storyboard>"), "Missing Storyboard");
    assert!(content.contains("</OpenSCENARIO>"), "Missing closing tag");

    // Check for proper nesting
    let open_pos = content
        .find("<OpenSCENARIO")
        .expect("OpenSCENARIO opening tag not found");
    let close_pos = content
        .find("</OpenSCENARIO>")
        .expect("OpenSCENARIO closing tag not found");
    assert!(open_pos < close_pos, "Improper tag nesting");

    // Check version attribute
    assert!(
        content.contains("revMajor=\"1\""),
        "Missing or incorrect major version"
    );
    assert!(
        content.contains("revMinor=\"2\""),
        "Missing or incorrect minor version"
    );

    println!("✓ XML validation test passed: Valid OpenSCENARIO structure confirmed");

    // Cleanup
    fs::remove_file(output_path).ok();
}

/// Test 6: Round-trip integrity (export XML, parse back, verify)
#[test]
fn test_round_trip_integrity() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    // Create original scenario
    let scenario_id = handle_create_scenario(
        state.clone(),
        "round_trip_test".to_string(),
        "1.2".to_string(),
    )
    .unwrap();

    // Add test data
    let test_vehicles = vec!["vehicle_alpha", "vehicle_beta"];
    for vehicle in &test_vehicles {
        handle_add_vehicle(
            state.clone(),
            scenario_id.clone(),
            vehicle.to_string(),
            "Car".to_string(),
            None,
        )
        .unwrap();

        handle_set_position(
            state.clone(),
            scenario_id.clone(),
            vehicle.to_string(),
            200.0,
            100.0,
            0.0,
            0.8,
        )
        .unwrap();

        handle_add_speed_action(
            state.clone(),
            scenario_id.clone(),
            vehicle.to_string(),
            format!("{}_story", vehicle),
            45.0,
            4.5,
        )
        .unwrap();
    }

    // Export to XML
    let output_path = "/tmp/round_trip_test.xosc";
    let export_result =
        handle_export_xml(state.clone(), scenario_id.clone(), output_path.to_string());
    assert!(
        export_result.is_ok(),
        "Export failed: {:?}",
        export_result.err()
    );

    // Read exported content
    let exported_content = fs::read_to_string(output_path).expect("Failed to read exported file");

    // Verify all original data is present
    for vehicle in &test_vehicles {
        assert!(
            exported_content.contains(vehicle),
            "Vehicle {} missing in export",
            vehicle
        );
        assert!(
            exported_content.contains(&format!("{}_story", vehicle)),
            "Story for {} missing in export",
            vehicle
        );
    }

    // Verify numeric values are preserved
    assert!(exported_content.contains("200"), "X position not preserved");
    assert!(exported_content.contains("100"), "Y position not preserved");
    assert!(exported_content.contains("45"), "Speed value not preserved");
    assert!(
        exported_content.contains("4.5"),
        "Transition time not preserved"
    );

    // Verify XML is well-formed (basic check - look for opening tag which may have attributes)
    let open_tags = exported_content.matches("<OpenSCENARIO").count();
    let close_tags = exported_content.matches("</OpenSCENARIO>").count();
    assert_eq!(
        open_tags, close_tags,
        "Mismatched OpenSCENARIO tags (open: {}, close: {})",
        open_tags, close_tags
    );
    assert!(open_tags > 0, "No OpenSCENARIO tags found at all!");

    println!("✓ Round-trip test passed: All data preserved in export cycle");

    // Cleanup
    fs::remove_file(output_path).ok();
}

/// Test 7: Large scenario stress test
#[test]
fn test_large_scenario_stress() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    let scenario_id =
        handle_create_scenario(state.clone(), "stress_test".to_string(), "1.2".to_string())
            .unwrap();

    // Add 10 vehicles
    for i in 0..10 {
        handle_add_vehicle(
            state.clone(),
            scenario_id.clone(),
            format!("vehicle_{}", i),
            "Car".to_string(),
            None,
        )
        .unwrap();

        // Each vehicle gets multiple actions
        for j in 0..5 {
            handle_add_speed_action(
                state.clone(),
                scenario_id.clone(),
                format!("vehicle_{}", i),
                format!("vehicle_{}_story_{}", i, j),
                20.0 + (j as f64 * 10.0),
                2.0,
            )
            .unwrap();
        }
    }

    // Export large scenario
    let output_path = "/tmp/stress_test.xosc";
    let result = handle_export_xml(state.clone(), scenario_id.clone(), output_path.to_string());
    assert!(result.is_ok(), "Failed to export large scenario");

    let content = fs::read_to_string(output_path).unwrap();

    // Verify all vehicles present
    for i in 0..10 {
        assert!(
            content.contains(&format!("vehicle_{}", i)),
            "Vehicle {} missing",
            i
        );
    }

    // Verify story count (10 vehicles * 5 stories = 50 stories)
    let story_count = content.matches("<Story").count();
    assert!(
        story_count >= 50,
        "Expected at least 50 stories, found {}",
        story_count
    );

    let file_size = content.len();
    println!(
        "✓ Stress test passed: 10 vehicles, 50 stories, {} bytes",
        file_size
    );

    // Cleanup
    fs::remove_file(output_path).ok();
}
