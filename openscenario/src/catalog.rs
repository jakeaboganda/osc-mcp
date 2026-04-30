//! Catalog support for OpenSCENARIO
//!
//! This module provides read-only loading of OpenSCENARIO catalogs containing
//! reusable entity definitions (Vehicle, Pedestrian, MiscObject).

use crate::entities::{
    Entity, Vehicle, VehicleParams, VehicleCategory,
    Pedestrian, PedestrianParams, MiscObject, MiscObjectParams,
};
use crate::error::{Result, ScenarioError};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::BufRead;
use std::path::Path;

/// Types of catalogs supported
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalogType {
    Vehicle,
    Pedestrian,
    MiscObject,
}

/// A catalog containing reusable entity definitions
#[derive(Debug, Clone)]
pub struct Catalog {
    catalog_type: CatalogType,
    entries: Vec<CatalogEntry>,
}

/// A single entry in a catalog
#[derive(Debug, Clone)]
pub struct CatalogEntry {
    name: String,
    entity: Entity,
}

impl Catalog {
    /// Load a catalog from an XML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())?;
        Self::from_xml(&content)
    }

    /// Parse a catalog from XML string
    pub fn from_xml(xml: &str) -> Result<Self> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut catalog_type = None;
        let mut entries = Vec::new();
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"VehicleCatalog" => catalog_type = Some(CatalogType::Vehicle),
                        b"PedestrianCatalog" => catalog_type = Some(CatalogType::Pedestrian),
                        b"MiscObjectCatalog" => catalog_type = Some(CatalogType::MiscObject),
                        b"Vehicle" | b"Pedestrian" | b"MiscObject" => {
                            if let Some(entry) = Self::parse_entry(&mut reader, name.as_ref())? {
                                entries.push(entry);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(ScenarioError::Xml(e)),
                _ => {}
            }
            buf.clear();
        }

        let catalog_type = catalog_type
            .ok_or_else(|| ScenarioError::InvalidCatalog("No catalog type found".to_string()))?;

        Ok(Catalog { catalog_type, entries })
    }

    /// Parse a single catalog entry
    fn parse_entry<R: BufRead>(
        reader: &mut Reader<R>,
        entry_type: &[u8],
    ) -> Result<Option<CatalogEntry>> {
        let mut entry_name = None;
        let mut dimensions = (0.0, 0.0, 0.0); // length, width, height
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    match e.name().as_ref() {
                        b"Properties" => {
                            // Extract name from Properties
                            if let Some(name) = Self::parse_properties(reader)? {
                                entry_name = Some(name);
                            }
                        }
                        b"Dimensions" => {
                            // Parse dimensions
                            for attr in e.attributes() {
                                let attr = attr.map_err(|e| ScenarioError::Xml(quick_xml::Error::InvalidAttr(e)))?;
                                let value_str = String::from_utf8_lossy(&attr.value);
                                let value = value_str.parse::<f64>().unwrap_or(0.0);
                                match attr.key.as_ref() {
                                    b"length" => dimensions.0 = value,
                                    b"width" => dimensions.1 = value,
                                    b"height" => dimensions.2 = value,
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == entry_type {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(ScenarioError::Xml(e)),
                _ => {}
            }
            buf.clear();
        }

        if let Some(name) = entry_name {
            let entity = Self::create_entity(entry_type, &name, dimensions)?;
            Ok(Some(CatalogEntry { name, entity }))
        } else {
            Ok(None)
        }
    }

    /// Parse properties to extract name
    fn parse_properties<R: BufRead>(reader: &mut Reader<R>) -> Result<Option<String>> {
        let mut name = None;
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    if e.name().as_ref() == b"Property" {
                        let mut prop_name = None;
                        let mut prop_value = None;

                        for attr in e.attributes() {
                            let attr = attr.map_err(|e| ScenarioError::Xml(quick_xml::Error::InvalidAttr(e)))?;
                            match attr.key.as_ref() {
                                b"name" => prop_name = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                b"value" => prop_value = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                _ => {}
                            }
                        }

                        if prop_name.as_deref() == Some("name") {
                            name = prop_value;
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"Properties" {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(ScenarioError::Xml(e)),
                _ => {}
            }
            buf.clear();
        }

        Ok(name)
    }

    /// Create entity from parsed data
    fn create_entity(
        entity_type: &[u8],
        name: &str,
        _dimensions: (f64, f64, f64),
    ) -> Result<Entity> {
        match entity_type {
            b"Vehicle" => Ok(Entity::Vehicle(Vehicle {
                name: name.to_string(),
                params: VehicleParams {
                    catalog: None,
                    vehicle_category: VehicleCategory::Car,
                    properties: None,
                },
            })),
            b"Pedestrian" => Ok(Entity::Pedestrian(Pedestrian {
                name: name.to_string(),
                params: PedestrianParams {
                    catalog: None,
                    model: None,
                    mass: Some(80.0),
                },
            })),
            b"MiscObject" => Ok(Entity::MiscObject(MiscObject {
                name: name.to_string(),
                params: MiscObjectParams {
                    catalog: None,
                    category: None,
                    mass: Some(100.0),
                },
            })),
            _ => Err(ScenarioError::InvalidCatalog(
                format!("Unknown entity type: {}", String::from_utf8_lossy(entity_type))
            )),
        }
    }

    /// Get catalog type
    pub fn catalog_type(&self) -> CatalogType {
        self.catalog_type
    }

    /// Get all entries
    pub fn entries(&self) -> &[CatalogEntry] {
        &self.entries
    }

    /// Find entry by name
    pub fn find(&self, name: &str) -> Option<&CatalogEntry> {
        self.entries.iter().find(|e| e.name == name)
    }
}

impl CatalogEntry {
    /// Get entry name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get entity
    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    /// Clone the entity
    pub fn clone_entity(&self) -> Entity {
        self.entity.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_type_enum() {
        assert_eq!(CatalogType::Vehicle, CatalogType::Vehicle);
        assert_ne!(CatalogType::Vehicle, CatalogType::Pedestrian);
    }

    #[test]
    fn test_parse_vehicle_catalog() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <VehicleCatalog>
    <Vehicle name="car1">
      <Properties>
        <Property name="name" value="sedan"/>
      </Properties>
      <BoundingBox>
        <Dimensions length="4.5" width="1.8" height="1.5"/>
      </BoundingBox>
    </Vehicle>
  </VehicleCatalog>
</OpenSCENARIO>"#;

        let catalog = Catalog::from_xml(xml).unwrap();
        assert_eq!(catalog.catalog_type(), CatalogType::Vehicle);
        assert_eq!(catalog.entries().len(), 1);

        let entry = catalog.find("sedan").unwrap();
        assert_eq!(entry.name(), "sedan");
        match entry.entity() {
            Entity::Vehicle(vehicle) => {
                assert_eq!(vehicle.name, "sedan");
            }
            _ => panic!("Expected Vehicle entity"),
        }
    }
}
