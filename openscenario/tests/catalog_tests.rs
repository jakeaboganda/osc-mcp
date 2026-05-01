use openscenario::catalog::{Catalog, CatalogType};
use openscenario::entities::{Entity, VehicleCategory};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn test_data_dir() -> TempDir {
    TempDir::new().unwrap()
}

fn create_vehicle_catalog(dir: &TempDir) -> PathBuf {
    let path = dir.path().join("VehicleCatalog.xosc");
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <CatalogDefinition>
    <Catalog name="VehicleCatalog">
      <VehicleCatalog>
        <Vehicle name="car1">
          <Properties>
            <Property name="name" value="sedan"/>
          </Properties>
          <BoundingBox>
            <Dimensions length="4.5" width="1.8" height="1.5"/>
          </BoundingBox>
        </Vehicle>
        <Vehicle name="car2">
          <Properties>
            <Property name="name" value="suv"/>
          </Properties>
          <BoundingBox>
            <Dimensions length="5.0" width="2.0" height="1.8"/>
          </BoundingBox>
        </Vehicle>
      </VehicleCatalog>
    </Catalog>
  </CatalogDefinition>
</OpenSCENARIO>"#;
    fs::write(&path, xml).unwrap();
    path
}

fn create_pedestrian_catalog(dir: &TempDir) -> PathBuf {
    let path = dir.path().join("PedestrianCatalog.xosc");
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <CatalogDefinition>
    <Catalog name="PedestrianCatalog">
      <PedestrianCatalog>
        <Pedestrian name="ped1">
          <Properties>
            <Property name="name" value="adult"/>
          </Properties>
          <BoundingBox>
            <Dimensions length="0.5" width="0.5" height="1.8"/>
          </BoundingBox>
        </Pedestrian>
      </PedestrianCatalog>
    </Catalog>
  </CatalogDefinition>
</OpenSCENARIO>"#;
    fs::write(&path, xml).unwrap();
    path
}

#[test]
fn test_load_vehicle_catalog() {
    let dir = test_data_dir();
    let path = create_vehicle_catalog(&dir);

    let catalog = Catalog::from_file(&path).unwrap();
    assert_eq!(catalog.catalog_type(), CatalogType::Vehicle);
    assert_eq!(catalog.entries().len(), 2);
}

#[test]
fn test_find_entry() {
    let dir = test_data_dir();
    let path = create_vehicle_catalog(&dir);

    let catalog = Catalog::from_file(&path).unwrap();

    let entry = catalog.find("sedan").unwrap();
    assert_eq!(entry.name(), "sedan");
    match entry.entity() {
        Entity::Vehicle(vehicle) => {
            assert_eq!(vehicle.name, "sedan");
            assert_eq!(vehicle.params.vehicle_category, VehicleCategory::Car);
        }
        _ => panic!("Expected Vehicle entity"),
    }

    let suv = catalog.find("suv").unwrap();
    assert_eq!(suv.name(), "suv");
    match suv.entity() {
        Entity::Vehicle(vehicle) => {
            assert_eq!(vehicle.name, "suv");
            assert_eq!(vehicle.params.vehicle_category, VehicleCategory::Car);
        }
        _ => panic!("Expected Vehicle entity"),
    }

    assert!(catalog.find("nonexistent").is_none());
}

#[test]
fn test_load_pedestrian_catalog() {
    let dir = test_data_dir();
    let path = create_pedestrian_catalog(&dir);

    let catalog = Catalog::from_file(&path).unwrap();
    assert_eq!(catalog.catalog_type(), CatalogType::Pedestrian);
    assert_eq!(catalog.entries().len(), 1);

    let adult = catalog.find("adult").unwrap();
    assert_eq!(adult.name(), "adult");
    match adult.entity() {
        Entity::Pedestrian(_) => {}
        _ => panic!("Expected Pedestrian entity"),
    }
}

#[test]
fn test_clone_entity() {
    let dir = test_data_dir();
    let path = create_vehicle_catalog(&dir);

    let catalog = Catalog::from_file(&path).unwrap();
    let entry = catalog.find("sedan").unwrap();

    let cloned = entry.clone_entity();
    match cloned {
        Entity::Vehicle(vehicle) => {
            assert_eq!(vehicle.name, "sedan");
        }
        _ => panic!("Expected Vehicle entity"),
    }
}

#[test]
fn test_invalid_catalog() {
    let invalid_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <InvalidCatalog/>
</OpenSCENARIO>"#;

    let result = Catalog::from_xml(invalid_xml);
    assert!(result.is_err());
}

#[test]
fn test_malformed_xml() {
    let malformed = "<not valid xml";
    let result = Catalog::from_xml(malformed);
    assert!(result.is_err());
}
