use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::{ComparisonRule, Condition, ConditionGroup, TransitionShape, Trigger};
use openscenario::{OpenScenarioVersion, ParameterType, Position, Scenario};

#[test]
fn test_parameter_condition_end_to_end() {
    // Create scenario with parameters
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_0);
    
    // Add multiple parameters (string, numeric, boolean)
    scenario.add_parameter("MaxSpeed", ParameterType::Double, "60.0").unwrap();
    scenario.add_parameter("VehicleState", ParameterType::String, "moving").unwrap();
    scenario.add_parameter("DebugMode", ParameterType::Boolean, "false").unwrap();
    
    // Add vehicle
    scenario.add_vehicle("ego", VehicleParams {
        catalog: None,
        vehicle_category: VehicleCategory::Car,
        properties: None,
    }).unwrap();
    
    // Set initial position
    scenario.set_initial_position("ego", Position::World {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        h: 0.0,
        p: 0.0,
        r: 0.0,
    }).unwrap();
    
    // Create story with parameter-based trigger
    scenario.add_story("main_story").unwrap();
    scenario.add_act("main_story", "main_act").unwrap();
    
    // Act starts when MaxSpeed > 50.0
    let act_trigger = Trigger::new(ConditionGroup::new(vec![
        Condition::parameter("MaxSpeed", "50.0", ComparisonRule::GreaterThan)
    ]));
    scenario.set_act_start_trigger("main_story", "main_act", act_trigger).unwrap();
    
    // Add maneuver group and event
    scenario.add_maneuver_group("main_story", "main_act", "mg1").unwrap();
    scenario.add_actor("main_story", "main_act", "mg1", "ego").unwrap();
    scenario.add_maneuver("main_story", "main_act", "mg1", "maneuver1").unwrap();
    
    // Add speed action (automatically creates event)
    scenario.add_speed_action(
        "main_story",
        "main_act",
        "mg1",
        "maneuver1",
        "event1",
        20.0,
        5.0,
        TransitionShape::Linear,
    ).unwrap();
    
    // Event starts when VehicleState == "moving"
    let event_trigger = Trigger::new(ConditionGroup::new(vec![
        Condition::parameter("VehicleState", "moving", ComparisonRule::EqualTo)
    ]));
    scenario.set_event_start_trigger("main_story", "main_act", "mg1", "maneuver1", "event1", event_trigger).unwrap();
    
    // Set stop trigger
    scenario.set_stop_time(10.0);
    
    // Export XML
    let xml = scenario.to_xml().unwrap();
    
    // Validate XML structure
    assert!(xml.contains("<ParameterDeclarations>"));
    assert!(xml.contains("<ParameterDeclaration name=\"MaxSpeed\""));
    assert!(xml.contains("parameterType=\"double\""));
    assert!(xml.contains("value=\"60.0\""));
    
    assert!(xml.contains("<ParameterDeclaration name=\"VehicleState\""));
    assert!(xml.contains("parameterType=\"string\""));
    assert!(xml.contains("value=\"moving\""));
    
    assert!(xml.contains("<ParameterCondition parameterRef=\"MaxSpeed\""));
    assert!(xml.contains("rule=\"greaterThan\""));
    
    assert!(xml.contains("<ParameterCondition parameterRef=\"VehicleState\""));
    assert!(xml.contains("rule=\"equalTo\""));
    
    // Optional: esmini validation if esmini is available
    #[cfg(feature = "esmini_validation")]
    {
        use std::fs;
        use std::process::Command;
        
        // Write XML to temp file
        let temp_path = "/tmp/test_parameter_condition.xosc";
        fs::write(temp_path, xml).unwrap();
        
        // Run esmini in headless mode to validate
        let output = Command::new("esmini")
            .args(&["--osc", temp_path, "--headless", "--fixed_timestep", "0.01", "--record", "sim.dat"])
            .output();
        
        if let Ok(result) = output {
            assert!(result.status.success(), "esmini validation failed: {:?}", 
                String::from_utf8_lossy(&result.stderr));
        }
        
        // Cleanup
        let _ = fs::remove_file(temp_path);
        let _ = fs::remove_file("sim.dat");
    }
}
