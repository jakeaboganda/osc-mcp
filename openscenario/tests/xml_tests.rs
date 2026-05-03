use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::position::Orientation;
use openscenario::storyboard::{
    ComparisonRule, Condition, ConditionGroup, TransitionShape, Trigger,
};
use openscenario::{OpenScenarioVersion, Position, Scenario};

#[test]
fn test_xml_export_minimal() {
    let scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let xml = scenario.to_xml().unwrap();

    // Should have basic structure
    assert!(xml.contains("<?xml"));
    assert!(xml.contains("<OpenSCENARIO"));
    assert!(xml.contains("</OpenSCENARIO>"));
    assert!(xml.contains("FileHeader"));
}

#[test]
fn test_xml_export_with_vehicle() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();

    let pos = Position::world(0.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("ego", pos).unwrap();

    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("<ScenarioObject name=\"ego\""));
    assert!(xml.contains("<Vehicle"));
    assert!(xml.contains("<Init>"));
    assert!(xml.contains("<WorldPosition"));
}

#[test]
fn test_xml_export_with_story() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();
    scenario
        .add_speed_action(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "event1",
            50.0,
            5.0,
            TransitionShape::Linear,
        )
        .unwrap();

    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("<Storyboard"));
    assert!(xml.contains("<Story"));
    assert!(xml.contains("<Act"));
    assert!(xml.contains("<ManeuverGroup"));
    assert!(xml.contains("<Maneuver"));
    assert!(xml.contains("<Event"));
    assert!(xml.contains("<Action"));
    assert!(xml.contains("<SpeedAction"));
}

#[test]
fn test_xml_export_with_relative_positions() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params.clone()).unwrap();
    scenario.add_vehicle("target", params).unwrap();

    // Test RelativeWorld position
    let pos_relative_world = Position::relative_world(
        "target",
        10.0,
        5.0,
        0.0,
        Orientation {
            h: 0.0,
            p: 0.0,
            r: 0.0,
        },
    );
    scenario
        .set_initial_position("ego", pos_relative_world)
        .unwrap();

    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("<RelativeWorldPosition"));
    assert!(xml.contains("entityRef=\"target\""));
    assert!(xml.contains("dx=\"10\""));
    assert!(xml.contains("dy=\"5\""));
    assert!(xml.contains("<Orientation"));
    assert!(xml.contains("type=\"relative\""));
}

#[test]
fn test_xml_export_lane_position_with_orientation() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();

    // Test Lane position with orientation
    let pos = Position::lane(
        "road1",
        -1,
        100.0,
        0.5,
        Some(Orientation {
            h: 1.57,
            p: 0.0,
            r: 0.0,
        }),
    );
    scenario.set_initial_position("ego", pos).unwrap();

    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("<LanePosition"));
    assert!(xml.contains("roadId=\"road1\""));
    assert!(xml.contains("laneId=\"-1\""));
    assert!(xml.contains("s=\"100\""));
    assert!(xml.contains("offset=\"0.5\""));
    assert!(xml.contains("h=\"1.57\""));
}

#[test]
fn test_xml_export_road_position() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();

    // Test Road position
    let pos = Position::road(
        "road1",
        50.0,
        2.5,
        Some(Orientation {
            h: 0.0,
            p: 0.0,
            r: 0.0,
        }),
    );
    scenario.set_initial_position("ego", pos).unwrap();

    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("<RoadPosition"));
    assert!(xml.contains("roadId=\"road1\""));
    assert!(xml.contains("s=\"50\""));
    assert!(xml.contains("t=\"2.5\""));
}

#[test]
fn test_xml_export_lane_change_with_actor() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("ego", params).unwrap();
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "act1").unwrap();
    scenario
        .add_maneuver_group("main_story", "act1", "mg1")
        .unwrap();
    scenario
        .add_actor("main_story", "act1", "mg1", "ego")
        .unwrap();
    scenario
        .add_maneuver("main_story", "act1", "mg1", "maneuver1")
        .unwrap();
    scenario
        .add_lane_change_action(
            "main_story",
            "act1",
            "mg1",
            "maneuver1",
            "event1",
            1.0,
            3.0,
            TransitionShape::Linear,
        )
        .unwrap();

    let xml = scenario.to_xml().unwrap();

    assert!(xml.contains("<LaneChangeAction"));
    assert!(xml.contains("<RelativeTargetLane"));
    // Verify entityRef is set to the actor, not empty
    assert!(xml.contains("entityRef=\"ego\""));
    assert!(xml.contains("value=\"1\""));
}

#[test]
fn test_act_custom_start_trigger_xml() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    scenario.add_story("MyStory").unwrap();
    scenario.add_act("MyStory", "MyAct").unwrap();

    // Create custom trigger: start at t=10
    let trigger = Trigger::new(ConditionGroup::new(vec![Condition::simulation_time(
        10.0,
        ComparisonRule::GreaterOrEqual,
    )]));

    scenario
        .set_act_start_trigger("MyStory", "MyAct", trigger)
        .unwrap();

    let xml = scenario.to_xml().expect("XML generation failed");

    // Should contain SimulationTimeCondition with value=10
    assert!(xml.contains("<SimulationTimeCondition"));
    assert!(xml.contains("value=\"10\""));
    assert!(xml.contains("rule=\"greaterOrEqual\""));
}

#[test]
fn test_act_default_start_trigger_xml() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    scenario.add_story("MyStory").unwrap();
    scenario.add_act("MyStory", "MyAct").unwrap(); // No trigger set

    let xml = scenario.to_xml().expect("XML generation failed");

    // Should contain default t=0 trigger
    assert!(xml.contains("<SimulationTimeCondition"));
    assert!(xml.contains("value=\"0\""));
}

#[test]
fn test_event_custom_start_trigger_xml() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

    // Set up story structure
    scenario.add_story("MyStory").unwrap();
    scenario.add_act("MyStory", "MyAct").unwrap();
    scenario
        .add_maneuver_group("MyStory", "MyAct", "MyManeuverGroup")
        .unwrap();
    scenario
        .add_maneuver("MyStory", "MyAct", "MyManeuverGroup", "MyManeuver")
        .unwrap();

    // Add an event (via speed action as a convenient way to create one)
    scenario
        .add_speed_action(
            "MyStory",
            "MyAct",
            "MyManeuverGroup",
            "MyManeuver",
            "MyEvent",
            50.0,
            5.0,
            TransitionShape::Linear,
        )
        .unwrap();

    // Set custom trigger: wait for act completion
    let trigger = Trigger::new(ConditionGroup::new(vec![
        Condition::storyboard_element_state("act", "MyAct", "completeState"),
    ]));
    scenario
        .set_event_start_trigger(
            "MyStory",
            "MyAct",
            "MyManeuverGroup",
            "MyManeuver",
            "MyEvent",
            trigger,
        )
        .unwrap();

    let xml = scenario.to_xml().expect("XML generation failed");

    // Should contain StoryboardElementStateCondition
    assert!(xml.contains("<StoryboardElementStateCondition"));
    assert!(xml.contains("storyboardElementRef=\"MyAct\""));
    assert!(xml.contains("state=\"completeState\""));
}

#[test]
fn test_event_default_start_trigger_xml() {
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);

    // Set up story structure
    scenario.add_story("MyStory").unwrap();
    scenario.add_act("MyStory", "MyAct").unwrap();
    scenario
        .add_maneuver_group("MyStory", "MyAct", "MyManeuverGroup")
        .unwrap();
    scenario
        .add_maneuver("MyStory", "MyAct", "MyManeuverGroup", "MyManeuver")
        .unwrap();

    // Add an event (via speed action) - No trigger set
    scenario
        .add_speed_action(
            "MyStory",
            "MyAct",
            "MyManeuverGroup",
            "MyManeuver",
            "MyEvent",
            50.0,
            5.0,
            TransitionShape::Linear,
        )
        .unwrap();

    let xml = scenario.to_xml().expect("XML generation failed");

    // Should contain default t=0 trigger
    assert!(xml.contains("EventStartCondition"));
    assert!(xml.contains("<SimulationTimeCondition"));
}
