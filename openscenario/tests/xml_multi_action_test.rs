use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::TransitionShape;
use openscenario::{OpenScenarioVersion, Position, Scenario};

#[test]
fn test_multiple_actions_per_entity_export() {
    // Create a minimal scenario with one vehicle
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

    // Add vehicle entity
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("TestCar", params).unwrap();

    // Add initial position
    let pos = Position::world(0.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("TestCar", pos).unwrap();

    // Set up storyboard structure
    scenario.add_story("test_story").unwrap();
    scenario.add_act("test_story", "test_act").unwrap();
    scenario
        .add_maneuver_group("test_story", "test_act", "test_mg")
        .unwrap();
    scenario
        .add_actor("test_story", "test_act", "test_mg", "TestCar")
        .unwrap();
    scenario
        .add_maneuver("test_story", "test_act", "test_mg", "test_maneuver")
        .unwrap();

    // Add FIRST action: speed action
    scenario
        .add_speed_action(
            "test_story",
            "test_act",
            "test_mg",
            "test_maneuver",
            "speed_event",
            30.0,
            1.0,
            TransitionShape::Step,
        )
        .unwrap();

    // Add SECOND action: lane change to same maneuver
    scenario
        .add_lane_change_action(
            "test_story",
            "test_act",
            "test_mg",
            "test_maneuver",
            "lane_change_event",
            1.0,
            5.0,
            TransitionShape::Sinusoidal,
        )
        .unwrap();

    // Export to XML
    let xml = scenario.to_xml().expect("Failed to export XML");

    println!("Generated XML:\n{}", xml);

    // Parse and verify BOTH actions are present
    assert!(xml.contains("SpeedAction"), "SpeedAction not found in XML");
    assert!(
        xml.contains("LaneChangeAction"),
        "LaneChangeAction not found in XML"
    );

    // Count occurrences of PrivateAction (should have at least 2)
    let private_action_count = xml.matches("<PrivateAction>").count();
    assert!(
        private_action_count >= 2,
        "Expected at least 2 PrivateAction elements, found {}",
        private_action_count
    );

    println!("✓ Both actions found in XML");
    println!("✓ PrivateAction count: {}", private_action_count);
}

#[test]
fn test_multiple_actions_same_event() {
    // Test that multiple actions can be added to the SAME event
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("TestCar", params).unwrap();

    let pos = Position::world(0.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("TestCar", pos).unwrap();

    scenario.add_story("test_story").unwrap();
    scenario.add_act("test_story", "test_act").unwrap();
    scenario
        .add_maneuver_group("test_story", "test_act", "test_mg")
        .unwrap();
    scenario
        .add_actor("test_story", "test_act", "test_mg", "TestCar")
        .unwrap();
    scenario
        .add_maneuver("test_story", "test_act", "test_mg", "test_maneuver")
        .unwrap();

    // Add BOTH actions to the SAME event name
    scenario
        .add_speed_action(
            "test_story",
            "test_act",
            "test_mg",
            "test_maneuver",
            "combined_event", // SAME event name
            30.0,
            1.0,
            TransitionShape::Step,
        )
        .unwrap();

    scenario
        .add_lane_change_action(
            "test_story",
            "test_act",
            "test_mg",
            "test_maneuver",
            "combined_event", // SAME event name
            1.0,
            5.0,
            TransitionShape::Sinusoidal,
        )
        .unwrap();

    let xml = scenario.to_xml().expect("Failed to export XML");

    println!("\nGenerated XML (same event):\n{}", xml);

    // Both actions should be present
    assert!(xml.contains("SpeedAction"), "SpeedAction not found");
    assert!(
        xml.contains("LaneChangeAction"),
        "LaneChangeAction not found"
    );

    // Should have ONE event with BOTH actions
    let event_count = xml.matches("<Event name=\"combined_event\"").count();
    assert_eq!(
        event_count, 1,
        "Expected exactly 1 event, found {}",
        event_count
    );

    println!("✓ Both actions in same event");
    println!("✓ Event count: {}", event_count);
}
