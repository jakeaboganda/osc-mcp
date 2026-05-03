use openscenario::*;

#[test]
fn test_trigger_with_simulation_time_condition() {
    let trigger = Trigger::new(
        ConditionGroup::new(vec![
            Condition::simulation_time(5.0, ComparisonRule::GreaterOrEqual)
        ])
    );
    
    assert_eq!(trigger.condition_groups.len(), 1);
    assert_eq!(trigger.condition_groups[0].conditions.len(), 1);
}
