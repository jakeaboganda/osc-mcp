use openscenario_mcp::handlers::{
    handle_add_lane_change_action, handle_add_speed_action, handle_add_vehicle,
    handle_create_scenario, handle_export_xml, handle_set_position,
};
use openscenario_mcp::server::ServerState;
use std::sync::{Arc, Mutex};

fn main() {
    let state = Arc::new(Mutex::new(ServerState::new()));
    
    let scenario_id = handle_create_scenario(
        state.clone(),
        "complex_sample".to_string(),
        "1.2".to_string(),
    ).unwrap();
    
    println!("Created scenario: {}", scenario_id);
    
    // Add 3 vehicles with different configurations
    let vehicles = vec![
        ("ego_vehicle", "Car"),
        ("lead_vehicle", "Truck"),
        ("follow_vehicle", "Car"),
    ];
    
    for (i, (name, vtype)) in vehicles.iter().enumerate() {
        handle_add_vehicle(
            state.clone(),
            scenario_id.clone(),
            name.to_string(),
            vtype.to_string(),
            None,
        ).unwrap();
        println!("Added vehicle: {}", name);
        
        // Set initial position
        handle_set_position(
            state.clone(),
            scenario_id.clone(),
            name.to_string(),
            100.0 + (i as f64 * 50.0),
            50.0 + (i as f64 * 10.0),
            0.0,
            0.5,
        ).unwrap();
        
        // Add speed action
        handle_add_speed_action(
            state.clone(),
            scenario_id.clone(),
            name.to_string(),
            format!("{}_speed_story", name),
            30.0 + (i as f64 * 10.0),
            3.0,
        ).unwrap();
        
        // Add lane change for non-ego vehicles
        if i > 0 {
            handle_add_lane_change_action(
                state.clone(),
                scenario_id.clone(),
                name.to_string(),
                format!("{}_lane_story", name),
                -3.5 * (i as f64),
                4.0,
            ).unwrap();
        }
    }
    
    let output_path = "/tmp/complex_sample.xosc";
    handle_export_xml(
        state.clone(),
        scenario_id.clone(),
        output_path.to_string()
    ).unwrap();
    
    println!("\n✓ Sample scenario exported to {}", output_path);
    println!("  - {} vehicles", vehicles.len());
    println!("  - {} speed actions", vehicles.len());
    println!("  - {} lane change actions", vehicles.len() - 1);
}
