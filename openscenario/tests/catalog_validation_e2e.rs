use openscenario::catalog::{Catalog, CatalogType};
use openscenario::{OpenScenarioVersion, Scenario};
use openscenario::entities::{Entity, CatalogReference, VehicleParams};
use openscenario::validation::XsdValidator;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_end_to_end_catalog_usage() {
    // Setup: Create a vehicle catalog
    let temp_dir = TempDir::new().unwrap();
    let catalog_path = temp_dir.path().join("VehicleCatalog.xosc");
    
    let catalog_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <CatalogDefinition>
    <Catalog name="VehicleCatalog">
      <VehicleCatalog>
        <Vehicle name="sports_car">
          <Properties>
            <Property name="name" value="ferrari"/>
          </Properties>
          <BoundingBox>
            <Dimensions length="4.2" width="1.9" height="1.3"/>
          </BoundingBox>
        </Vehicle>
      </VehicleCatalog>
    </Catalog>
  </CatalogDefinition>
</OpenSCENARIO>"#;
    
    fs::write(&catalog_path, catalog_xml).unwrap();
    
    // Step 1: Load the catalog
    let catalog = Catalog::from_file(&catalog_path).unwrap();
    assert_eq!(catalog.catalog_type(), CatalogType::Vehicle);
    
    // Step 2: Find the vehicle in the catalog
    let ferrari_entry = catalog.find("ferrari").unwrap();
    assert_eq!(ferrari_entry.name(), "ferrari");
    
    // Step 3: Create a scenario and add vehicle from catalog
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    match ferrari_entry.entity() {
        Entity::Vehicle(vehicle) => {
            scenario.add_vehicle("hero_car".to_string(), vehicle.params.clone()).unwrap();
        }
        _ => panic!("Expected vehicle entity"),
    }
    
    // Step 4: Export and validate
    let xml_output = scenario.to_xml().unwrap();
    assert!(xml_output.contains("hero_car"));
    
    // Validate the generated scenario
    let validator = XsdValidator::new("1.2");
    let report = validator.validate(&xml_output);
    assert!(report.valid, "Generated scenario should be valid");
}

#[test]
fn test_catalog_reference_in_scenario() {
    // Create a scenario with a catalog reference
    let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
    
    let params = VehicleParams {
        catalog: Some(CatalogReference {
            path: "./catalogs/VehicleCatalog.xosc".to_string(),
            entry_name: "sedan".to_string(),
        }),
        vehicle_category: openscenario::entities::VehicleCategory::Car,
        properties: None,
    };
    
    scenario.add_vehicle("car1".to_string(), params).unwrap();
    
    // Export to XML
    let xml = scenario.to_xml().unwrap();
    
    // Verify catalog reference is present in XML
    assert!(xml.contains("CatalogReference"));
    // The actual catalog reference format may vary, so just check that entity name is there
    assert!(xml.contains("car1"));
}

#[test]
fn test_multiple_catalogs() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create vehicle catalog
    let vehicle_catalog_path = temp_dir.path().join("VehicleCatalog.xosc");
    let vehicle_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <VehicleCatalog>
    <Vehicle name="v1">
      <Properties>
        <Property name="name" value="truck"/>
      </Properties>
      <BoundingBox>
        <Dimensions length="8.0" width="2.5" height="3.0"/>
      </BoundingBox>
    </Vehicle>
  </VehicleCatalog>
</OpenSCENARIO>"#;
    fs::write(&vehicle_catalog_path, vehicle_xml).unwrap();
    
    // Create pedestrian catalog
    let ped_catalog_path = temp_dir.path().join("PedestrianCatalog.xosc");
    let ped_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <PedestrianCatalog>
    <Pedestrian name="p1">
      <Properties>
        <Property name="name" value="walker"/>
      </Properties>
      <BoundingBox>
        <Dimensions length="0.5" width="0.5" height="1.75"/>
      </BoundingBox>
    </Pedestrian>
  </PedestrianCatalog>
</OpenSCENARIO>"#;
    fs::write(&ped_catalog_path, ped_xml).unwrap();
    
    // Load both catalogs
    let vehicle_catalog = Catalog::from_file(&vehicle_catalog_path).unwrap();
    let ped_catalog = Catalog::from_file(&ped_catalog_path).unwrap();
    
    assert_eq!(vehicle_catalog.catalog_type(), CatalogType::Vehicle);
    assert_eq!(ped_catalog.catalog_type(), CatalogType::Pedestrian);
    
    assert!(vehicle_catalog.find("truck").is_some());
    assert!(ped_catalog.find("walker").is_some());
}

#[test]
fn test_validation_error_messages() {
    // Test that validation provides helpful error messages
    
    // Test 1: Malformed XML
    let validator = XsdValidator::new("1.0");
    let malformed = "<OpenSCENARIO><FileHeader></OpenSCENARIO>";
    let report = validator.validate(malformed);
    assert!(!report.valid);
    assert!(!report.errors.is_empty());
    assert!(report.errors[0].contains("parsing error") || report.errors[0].contains("XML"));
    
    // Test 2: Version mismatch
    let wrong_version = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader revMajor="2" revMinor="0"/>
</OpenSCENARIO>"#;
    let report = validator.validate(wrong_version);
    assert!(!report.valid);
    assert!(report.errors.iter().any(|e| e.contains("Version mismatch")));
    
    // Test 3: Missing version attributes
    let no_version = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader date="2024-01-01"/>
</OpenSCENARIO>"#;
    let report = validator.validate(no_version);
    // Should either pass (if no version check needed) or report missing attributes
    // This tests that the validator doesn't crash on missing attributes
    assert!(report.valid || !report.errors.is_empty());
}

#[test]
fn test_wrong_catalog_type() {
    // Try to load a vehicle catalog as pedestrian (wrong type detection)
    let vehicle_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <VehicleCatalog>
    <Vehicle name="car">
      <Properties>
        <Property name="name" value="sedan"/>
      </Properties>
      <BoundingBox>
        <Dimensions length="4.5" width="1.8" height="1.5"/>
      </BoundingBox>
    </Vehicle>
  </VehicleCatalog>
</OpenSCENARIO>"#;
    
    let catalog = Catalog::from_xml(vehicle_xml).unwrap();
    
    // Verify it's detected as a vehicle catalog
    assert_eq!(catalog.catalog_type(), CatalogType::Vehicle);
    
    // Verify we can't accidentally use it as a pedestrian
    match catalog.find("sedan").unwrap().entity() {
        Entity::Vehicle(_) => {
            // Correct!
        }
        Entity::Pedestrian(_) => {
            panic!("Should not be a pedestrian!");
        }
        _ => {}
    }
}
