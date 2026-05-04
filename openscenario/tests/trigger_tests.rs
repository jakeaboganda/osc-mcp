use openscenario::*;

#[test]
fn test_trigger_with_simulation_time_condition() {
    let trigger = Trigger::new(ConditionGroup::new(vec![Condition::simulation_time(
        5.0,
        ComparisonRule::GreaterOrEqual,
    )]));

    assert_eq!(trigger.condition_groups.len(), 1);
    assert_eq!(trigger.condition_groups[0].conditions.len(), 1);
}

#[test]
fn test_act_with_custom_start_trigger() {
    let mut act = Act::new("MyAct");

    let trigger = Trigger::new(ConditionGroup::new(vec![Condition::simulation_time(
        5.0,
        ComparisonRule::GreaterOrEqual,
    )]));

    act.set_start_trigger(trigger.clone());

    assert!(act.start_trigger.is_some());
    let act_trigger = act.start_trigger.unwrap();
    assert_eq!(act_trigger.condition_groups.len(), 1);
}

#[test]
fn test_event_with_custom_start_trigger() {
    let mut event = Event::new("MyEvent");

    let trigger = Trigger::new(ConditionGroup::new(vec![
        Condition::storyboard_element_state("act", "MyAct", "completeState"),
    ]));

    event.set_start_trigger(trigger.clone());

    assert!(event.start_trigger.is_some());
}

#[test]
fn test_trigger_with_multiple_condition_groups() {
    let trigger = Trigger::with_groups(vec![
        ConditionGroup::new(vec![Condition::simulation_time(
            5.0,
            ComparisonRule::GreaterOrEqual,
        )]),
        ConditionGroup::new(vec![
            Condition::storyboard_element_state("act", "Act1", "completeState"),
            Condition::simulation_time(10.0, ComparisonRule::GreaterOrEqual),
        ]),
    ]);

    assert_eq!(trigger.condition_groups.len(), 2);
    assert_eq!(trigger.condition_groups[0].conditions.len(), 1);
    assert_eq!(trigger.condition_groups[1].conditions.len(), 2);
}

#[test]
fn test_parameter_condition_construction() {
    use openscenario::*;
    
    let param_cond = ParameterCondition {
        parameter_ref: "MaxSpeed".to_string(),
        value: "50.0".to_string(),
        rule: ComparisonRule::GreaterThan,
    };
    
    assert_eq!(param_cond.parameter_ref, "MaxSpeed");
    assert_eq!(param_cond.value, "50.0");
    assert_eq!(param_cond.rule, ComparisonRule::GreaterThan);
}

#[test]
fn test_parameter_helper_method() {
    use openscenario::*;
    
    let condition = Condition::parameter("MaxSpeed", "50.0", ComparisonRule::GreaterThan);
    
    assert_eq!(condition.name, "Param_MaxSpeed");
    assert_eq!(condition.delay, 0.0);
    assert_eq!(condition.condition_edge, ConditionEdge::None);
    
    match &condition.kind {
        ConditionKind::ByValue(ByValueCondition::Parameter(param_cond)) => {
            assert_eq!(param_cond.parameter_ref, "MaxSpeed");
            assert_eq!(param_cond.value, "50.0");
            assert_eq!(param_cond.rule, ComparisonRule::GreaterThan);
        }
        _ => panic!("Expected ByValueCondition::Parameter"),
    }
}

#[test]
fn test_parameter_condition_all_comparison_rules() {
    use openscenario::*;
    
    let rules = vec![
        ComparisonRule::LessThan,
        ComparisonRule::LessOrEqual,
        ComparisonRule::EqualTo,
        ComparisonRule::NotEqualTo,
        ComparisonRule::GreaterOrEqual,
        ComparisonRule::GreaterThan,
    ];
    
    for rule in rules {
        let param_cond = ParameterCondition {
            parameter_ref: "TestParam".to_string(),
            value: "42".to_string(),
            rule: rule.clone(),
        };
        
        assert_eq!(param_cond.rule, rule);
    }
}

#[test]
fn test_parameter_condition_xml_serialization() {
    use openscenario::*;
    use openscenario::entities::{VehicleCategory, VehicleParams};
    
    // Create a scenario with a ParameterCondition in a trigger
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    
    // Add a vehicle so we have valid scenario structure
    let params = VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    };
    scenario.add_vehicle("Ego", params).unwrap();
    
    // Set initial position
    let ego_pos = Position::world(0.0, 0.0, 0.0, 0.0);
    scenario.set_initial_position("Ego", ego_pos).unwrap();
    
    // Add story, act, maneuver group structure to hold event with parameter condition
    scenario.add_story("TestStory").unwrap();
    scenario.add_act("TestStory", "TestAct").unwrap();
    
    // Use a parameter condition as the act start trigger
    let trigger = Trigger::new(ConditionGroup::new(vec![
        Condition::parameter("MaxSpeed", "50.0", ComparisonRule::GreaterThan)
    ]));
    
    scenario
        .set_act_start_trigger("TestStory", "TestAct", trigger)
        .unwrap();
    
    // Generate XML and verify ParameterCondition appears correctly
    let xml = scenario.to_xml().unwrap();
    
    // Verify critical XML elements and attributes are present
    assert!(xml.contains("<ParameterCondition"));
    assert!(xml.contains("parameterRef=\"MaxSpeed\""));
    assert!(xml.contains("value=\"50.0\""));
    assert!(xml.contains("rule=\"greaterThan\""));
}

#[test]
fn test_parameter_condition_value_types() {
    use openscenario::*;
    
    // String value
    let string_cond = ParameterCondition {
        parameter_ref: "VehicleState".to_string(),
        value: "stopped".to_string(),
        rule: ComparisonRule::EqualTo,
    };
    assert_eq!(string_cond.value, "stopped");
    
    // Numeric value (stored as string)
    let numeric_cond = ParameterCondition {
        parameter_ref: "MaxSpeed".to_string(),
        value: "50.0".to_string(),
        rule: ComparisonRule::GreaterThan,
    };
    assert_eq!(numeric_cond.value, "50.0");
    
    // Boolean value (stored as string)
    let boolean_cond = ParameterCondition {
        parameter_ref: "DebugMode".to_string(),
        value: "true".to_string(),
        rule: ComparisonRule::EqualTo,
    };
    assert_eq!(boolean_cond.value, "true");
}
