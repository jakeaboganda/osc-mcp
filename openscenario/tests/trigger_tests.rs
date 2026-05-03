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
