use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::{
    ComparisonRule, Condition, ConditionGroup, TransitionShape, Trigger,
};
use openscenario::{OpenScenarioVersion, Position, Scenario};
use std::process::Command;

#[test]
fn test_scenario_with_custom_triggers_in_esmini() {
    // Create scenario
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

    // Add ego vehicle
    let ego_params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("Ego", ego_params).unwrap();

    // Set initial position
    let ego_pos = Position::world(0.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("Ego", ego_pos).unwrap();

    // Create story with delayed act (starts at t=2)
    scenario.add_story("TriggerTestStory").unwrap();
    scenario.add_act("TriggerTestStory", "DelayedAct").unwrap();

    // Act starts at t=2
    let act_trigger = Trigger::new(ConditionGroup::new(vec![Condition::simulation_time(
        2.0,
        ComparisonRule::GreaterOrEqual,
    )]));
    scenario
        .set_act_start_trigger("TriggerTestStory", "DelayedAct", act_trigger)
        .unwrap();

    // Create maneuver group and add actor
    scenario
        .add_maneuver_group("TriggerTestStory", "DelayedAct", "EgoManeuverGroup")
        .unwrap();
    scenario
        .add_actor("TriggerTestStory", "DelayedAct", "EgoManeuverGroup", "Ego")
        .unwrap();

    // Create maneuver and event
    scenario
        .add_maneuver(
            "TriggerTestStory",
            "DelayedAct",
            "EgoManeuverGroup",
            "SpeedManeuver",
        )
        .unwrap();

    // Add speed action to event
    scenario
        .add_speed_action(
            "TriggerTestStory",
            "DelayedAct",
            "EgoManeuverGroup",
            "SpeedManeuver",
            "SpeedEvent",
            20.0,
            2.0,
            TransitionShape::Linear,
        )
        .unwrap();

    // Event starts 1 second after act (total t=3)
    let event_trigger = Trigger::new(ConditionGroup::new(vec![Condition::simulation_time(
        1.0,
        ComparisonRule::GreaterOrEqual,
    )]));
    scenario
        .set_event_start_trigger(
            "TriggerTestStory",
            "DelayedAct",
            "EgoManeuverGroup",
            "SpeedManeuver",
            "SpeedEvent",
            event_trigger,
        )
        .unwrap();

    // Set stop trigger at t=6
    scenario.set_stop_time(6.0);

    // Generate XML
    let xml = scenario.to_xml().expect("XML generation failed");

    // Verify trigger structure
    assert!(xml.contains("value=\"2\""), "Act should start at t=2");
    assert!(xml.contains("value=\"1\""), "Event should have delay of 1s");

    // Verify basic structure
    assert!(xml.contains("<Story name=\"TriggerTestStory\""));
    assert!(xml.contains("<Act name=\"DelayedAct\""));
    assert!(xml.contains("<StartTrigger"));
    assert!(xml.contains("<SimulationTimeCondition"));

    // Write to temp file
    let test_file = "/tmp/trigger_integration_test.xosc";
    std::fs::write(test_file, xml).expect("Failed to write test file");

    println!("✓ XML structure validated");
    println!("✓ Test file written to {}", test_file);

    // Validate with esmini (if available)
    let esmini_check = Command::new("esmini")
        .args([
            "--osc",
            test_file,
            "--headless",
            "--fixed_timestep",
            "0.01",
            "--record",
            "sim.dat",
        ])
        .output();

    if let Ok(output) = esmini_check {
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("esmini validation failed:\n{}", stderr);
        }
        println!("✓ esmini validation passed");
    } else {
        println!("⚠ esmini not available, skipping runtime validation");
    }
}
