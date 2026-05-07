//! End-to-end integration test for SpeedCondition with esmini validation.
//!
//! Creates complete OpenSCENARIO files with SpeedCondition triggers,
//! validates XML structure, and optionally runs esmini validation.

use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::{
    ByEntityCondition, Condition, ConditionEdge, ConditionGroup, ConditionKind, EntityCondition,
    Rule, SpeedCondition, TransitionShape, Trigger, TriggeringEntities, TriggeringEntitiesRule,
};
use openscenario::{OpenScenarioVersion, Position, Scenario};
use std::fs;
use std::process::Command;

/// Helper function to validate with esmini if available
fn validate_with_esmini(scenario_path: &str) {
    let esmini_result = Command::new("esmini")
        .arg("--osc")
        .arg(scenario_path)
        .arg("--disable_controllers")
        .arg("--fixed_timestep")
        .arg("0.01")
        .arg("--headless")
        .output();

    match esmini_result {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                panic!("esmini validation failed:\n{}", stderr);
            }
            println!("✓ esmini validation passed for {}", scenario_path);
        }
        Err(e) => {
            println!("⚠ esmini not available ({}), skipping validation", e);
        }
    }
}

#[test]
fn test_speed_condition_integration_any_rule() {
    // Build complete scenario with single entity and Any rule
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

    // Add Ego vehicle
    scenario
        .add_vehicle(
            "Ego",
            VehicleParams {
                catalog: None,
                vehicle_category: VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    // Set initial position
    scenario
        .set_initial_position(
            "Ego",
            Position::World {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                h: 0.0,
                p: 0.0,
                r: 0.0,
            },
        )
        .unwrap();

    // Create story and act
    scenario.add_story("MainStory").unwrap();
    scenario.add_act("MainStory", "MainAct").unwrap();

    // Create TriggeringEntities with Any rule, entity_refs: ["Ego"]
    let triggering_entities = TriggeringEntities {
        rule: TriggeringEntitiesRule::Any,
        entity_refs: vec!["Ego".to_string()],
    };

    // Create SpeedCondition: value 30.0, rule GreaterThan
    let speed_condition = SpeedCondition {
        value: 30.0,
        rule: Rule::GreaterThan,
    };

    // Create ByEntityCondition
    let by_entity_condition = ByEntityCondition {
        triggering_entities,
        entity_condition: EntityCondition::Speed(speed_condition),
    };

    // Create Condition with ByEntity kind
    let condition = Condition {
        name: "SpeedAbove30".to_string(),
        delay: 0.0,
        condition_edge: ConditionEdge::None,
        kind: ConditionKind::ByEntity(by_entity_condition),
    };

    // Create ConditionGroup and Trigger
    let condition_group = ConditionGroup::new(vec![condition]);
    let trigger = Trigger::new(condition_group);

    // Set act start trigger
    scenario
        .set_act_start_trigger("MainStory", "MainAct", trigger)
        .unwrap();

    // Add maneuver group and actor
    scenario
        .add_maneuver_group("MainStory", "MainAct", "EgoManeuverGroup")
        .unwrap();
    scenario
        .add_actor("MainStory", "MainAct", "EgoManeuverGroup", "Ego")
        .unwrap();

    // Add maneuver
    scenario
        .add_maneuver("MainStory", "MainAct", "EgoManeuverGroup", "SpeedManeuver")
        .unwrap();

    // Add speed action (creates event)
    scenario
        .add_speed_action(
            "MainStory",
            "MainAct",
            "EgoManeuverGroup",
            "SpeedManeuver",
            "SpeedEvent",
            20.0,
            5.0,
            TransitionShape::Linear,
        )
        .unwrap();

    // Set stop trigger
    scenario.set_stop_time(10.0);

    // Export to XML
    let xml = scenario.to_xml().unwrap();

    // Create output directory
    let output_dir = "esmini-tests/scenarios";
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Write to file
    let scenario_path = format!("{}/speed_condition_any_test.xosc", output_dir);
    fs::write(&scenario_path, &xml).expect("Failed to write scenario file");

    println!("✓ Scenario written to {}", scenario_path);

    // Validate XML structure
    assert!(
        xml.contains("<TriggeringEntities"),
        "XML should contain TriggeringEntities"
    );
    assert!(
        xml.contains("triggeringEntitiesRule=\"any\""),
        "XML should specify any rule"
    );
    assert!(
        xml.contains("<EntityRef entityRef=\"Ego\""),
        "XML should reference Ego entity"
    );
    assert!(
        xml.contains("<SpeedCondition"),
        "XML should contain SpeedCondition"
    );
    assert!(
        xml.contains("value=\"30\""),
        "XML should have speed value 30"
    );
    assert!(
        xml.contains("rule=\"greaterThan\""),
        "XML should have greaterThan rule"
    );

    println!("✓ XML structure validated");

    // Validate with esmini if available
    validate_with_esmini(&scenario_path);
}

#[test]
fn test_speed_condition_integration_all_rule() {
    // Build complete scenario with multiple entities and All rule
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

    // Add Ego vehicle
    scenario
        .add_vehicle(
            "Ego",
            VehicleParams {
                catalog: None,
                vehicle_category: VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    // Add Target vehicle
    scenario
        .add_vehicle(
            "Target",
            VehicleParams {
                catalog: None,
                vehicle_category: VehicleCategory::Car,
                properties: None,
            },
        )
        .unwrap();

    // Set initial positions
    scenario
        .set_initial_position(
            "Ego",
            Position::World {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                h: 0.0,
                p: 0.0,
                r: 0.0,
            },
        )
        .unwrap();

    scenario
        .set_initial_position(
            "Target",
            Position::World {
                x: 10.0,
                y: 0.0,
                z: 0.0,
                h: 0.0,
                p: 0.0,
                r: 0.0,
            },
        )
        .unwrap();

    // Create story and act
    scenario.add_story("MainStory").unwrap();
    scenario.add_act("MainStory", "MainAct").unwrap();

    // Create TriggeringEntities with All rule, entity_refs: ["Ego", "Target"]
    let triggering_entities = TriggeringEntities {
        rule: TriggeringEntitiesRule::All,
        entity_refs: vec!["Ego".to_string(), "Target".to_string()],
    };

    // Create SpeedCondition: value 15.0, rule LessThan
    let speed_condition = SpeedCondition {
        value: 15.0,
        rule: Rule::LessThan,
    };

    // Create ByEntityCondition
    let by_entity_condition = ByEntityCondition {
        triggering_entities,
        entity_condition: EntityCondition::Speed(speed_condition),
    };

    // Create Condition with ByEntity kind
    let condition = Condition {
        name: "BothBelow15".to_string(),
        delay: 0.0,
        condition_edge: ConditionEdge::None,
        kind: ConditionKind::ByEntity(by_entity_condition),
    };

    // Create ConditionGroup and Trigger
    let condition_group = ConditionGroup::new(vec![condition]);
    let trigger = Trigger::new(condition_group);

    // Set act start trigger
    scenario
        .set_act_start_trigger("MainStory", "MainAct", trigger)
        .unwrap();

    // Add maneuver group and actors
    scenario
        .add_maneuver_group("MainStory", "MainAct", "EgoManeuverGroup")
        .unwrap();
    scenario
        .add_actor("MainStory", "MainAct", "EgoManeuverGroup", "Ego")
        .unwrap();

    // Add maneuver
    scenario
        .add_maneuver("MainStory", "MainAct", "EgoManeuverGroup", "SpeedManeuver")
        .unwrap();

    // Add speed action (creates event)
    scenario
        .add_speed_action(
            "MainStory",
            "MainAct",
            "EgoManeuverGroup",
            "SpeedManeuver",
            "SpeedEvent",
            25.0,
            3.0,
            TransitionShape::Linear,
        )
        .unwrap();

    // Set stop trigger
    scenario.set_stop_time(15.0);

    // Export to XML
    let xml = scenario.to_xml().unwrap();

    // Create output directory
    let output_dir = "esmini-tests/scenarios";
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Write to file
    let scenario_path = format!("{}/speed_condition_all_test.xosc", output_dir);
    fs::write(&scenario_path, &xml).expect("Failed to write scenario file");

    println!("✓ Scenario written to {}", scenario_path);

    // Validate XML structure
    assert!(
        xml.contains("<TriggeringEntities"),
        "XML should contain TriggeringEntities"
    );
    assert!(
        xml.contains("triggeringEntitiesRule=\"all\""),
        "XML should specify all rule"
    );
    assert!(
        xml.contains("<EntityRef entityRef=\"Ego\""),
        "XML should reference Ego entity"
    );
    assert!(
        xml.contains("<EntityRef entityRef=\"Target\""),
        "XML should reference Target entity"
    );
    assert!(
        xml.contains("<SpeedCondition"),
        "XML should contain SpeedCondition"
    );
    assert!(
        xml.contains("value=\"15\""),
        "XML should have speed value 15"
    );
    assert!(
        xml.contains("rule=\"lessThan\""),
        "XML should have lessThan rule"
    );

    println!("✓ XML structure validated");

    // Validate with esmini if available
    validate_with_esmini(&scenario_path);
}
