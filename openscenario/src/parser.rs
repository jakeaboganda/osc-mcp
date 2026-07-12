//! XML parsing for OpenSCENARIO files.
//!
//! Implements parsing of OpenSCENARIO XML into Scenario structs.

use crate::entities::{
    CatalogReference, Entity, MiscObject, MiscObjectParams, Pedestrian, PedestrianParams, Vehicle,
    VehicleCategory, VehicleParams,
};
use crate::position::Position;
use crate::scenario::{ParameterDeclaration, ParameterType, Scenario};
use crate::storyboard::{
    Act, Action, ByValueCondition, Condition, ConditionEdge, ConditionGroup, ConditionKind,
    DynamicsDimension, DynamicsShape, Event, LaneChangeAction, Maneuver, ManeuverGroup,
    ParameterCondition, Rule, SpeedAction, Story, Storyboard, TransitionDynamics, TransitionShape,
    Trigger,
};
use crate::{OpenScenarioVersion, Result, ScenarioError};
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesStart, Event as XmlEvent};
use quick_xml::Reader;
use std::collections::HashMap;

fn unescape_attr(attr: &Attribute) -> Result<String> {
    attr.unescape_value()
        .map(|cow| cow.into_owned())
        .map_err(ScenarioError::Xml)
}

impl Scenario {
    /// Parse OpenSCENARIO XML into a Scenario struct.
    ///
    /// Supports OpenSCENARIO versions 1.0, 1.1, and 1.2.
    ///
    /// # Arguments
    /// * `xml` - OpenSCENARIO XML string
    ///
    /// # Returns
    /// * `Ok(Scenario)` - Parsed scenario
    /// * `Err(ScenarioError)` - Parse error
    ///
    /// # Examples
    /// ```no_run
    /// use openscenario::Scenario;
    ///
    /// let xml = std::fs::read_to_string("scenario.xosc").unwrap();
    /// let scenario = Scenario::from_xml(&xml).unwrap();
    /// ```
    pub fn from_xml(xml: &str) -> Result<Self> {
        // Security: Validate file size to prevent DoS
        const MAX_XML_SIZE: usize = 100 * 1024 * 1024; // 100MB
        if xml.len() > MAX_XML_SIZE {
            return Err(ScenarioError::InvalidValue {
                field: "xml_size".to_string(),
                reason: format!(
                    "XML file too large ({} bytes). Maximum size is {} bytes (100MB)",
                    xml.len(),
                    MAX_XML_SIZE
                ),
            });
        }

        // Security: Check for empty input
        if xml.trim().is_empty() {
            return Err(ScenarioError::Parse(
                "XML file is empty or contains only whitespace".to_string(),
            ));
        }

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);
        reader.config_mut().check_end_names = true; // Validate closing tags

        let mut scenario = None;
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(XmlEvent::Start(e)) => {
                    if e.name().as_ref() == b"OpenSCENARIO" {
                        scenario = Some(parse_openscenario(&mut reader)?);
                        break;
                    }
                }
                Ok(XmlEvent::Eof) => break,
                Err(e) => return Err(ScenarioError::Xml(e)),
                _ => {}
            }
            buf.clear();
        }

        scenario
            .ok_or_else(|| ScenarioError::Parse("No OpenSCENARIO root element found".to_string()))
    }
}

fn parse_openscenario(reader: &mut Reader<&[u8]>) -> Result<Scenario> {
    let mut version = OpenScenarioVersion::V1_2; // Default
    let mut entities = HashMap::new();
    let mut initial_positions = HashMap::new();
    let mut initial_speeds = HashMap::new();
    let mut road_network = None;
    let mut parameters = Vec::new();
    let mut storyboard = Storyboard::default();

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"ParameterDeclarations" => {
                    parameters = parse_parameter_declarations(reader)?;
                }
                b"RoadNetwork" => {
                    road_network = parse_road_network(reader)?;
                }
                b"Entities" => {
                    entities = parse_entities(reader)?;
                }
                b"Storyboard" => {
                    (storyboard, initial_positions, initial_speeds) =
                        parse_storyboard(reader, &entities)?;
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            // FileHeader is a self-closing (empty) element in OpenSCENARIO XML
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"FileHeader" => {
                version = parse_file_header_empty(&e)?;
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"OpenSCENARIO" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Scenario {
        version,
        entities,
        initial_positions,
        initial_speeds,
        entity_dimensions: HashMap::new(),
        road_network,
        parameters,
        storyboard,
    })
}

fn parse_file_header_empty(e: &BytesStart) -> Result<OpenScenarioVersion> {
    let mut major: Option<u8> = None;
    let mut minor: Option<u8> = None;

    for attr in e.attributes() {
        let attr = attr.map_err(|e| ScenarioError::Parse(e.to_string()))?;
        match attr.key.as_ref() {
            b"revMajor" => {
                let value = String::from_utf8_lossy(&attr.value);
                major = Some(value.parse::<u8>().map_err(|_| {
                    ScenarioError::Parse(format!(
                        "Invalid revMajor attribute: '{}'. Expected integer.",
                        value
                    ))
                })?);
            }
            b"revMinor" => {
                let value = String::from_utf8_lossy(&attr.value);
                minor = Some(value.parse::<u8>().map_err(|_| {
                    ScenarioError::Parse(format!(
                        "Invalid revMinor attribute: '{}'. Expected integer.",
                        value
                    ))
                })?);
            }
            _ => {}
        }
    }

    let major = major.ok_or_else(|| {
        ScenarioError::Parse("Missing revMajor attribute in FileHeader".to_string())
    })?;
    let minor = minor.ok_or_else(|| {
        ScenarioError::Parse("Missing revMinor attribute in FileHeader".to_string())
    })?;

    OpenScenarioVersion::from_rev(major, minor).ok_or_else(|| {
        ScenarioError::Parse(format!(
            "Unsupported OpenSCENARIO version {}.{}. Supported versions: 1.0, 1.1, 1.2, 1.3.",
            major, minor
        ))
    })
}

fn parse_parameter_declarations(reader: &mut Reader<&[u8]>) -> Result<Vec<ParameterDeclaration>> {
    let mut parameters = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"ParameterDeclaration" => {
                let mut name = String::new();
                let mut parameter_type = ParameterType::String;
                let mut value = String::new();

                for attr in e.attributes() {
                    let attr = attr.map_err(|e| ScenarioError::Parse(e.to_string()))?;
                    match attr.key.as_ref() {
                        b"name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                        b"parameterType" => {
                            parameter_type = match attr.value.as_ref() {
                                b"integer" => ParameterType::Integer,
                                b"double" => ParameterType::Double,
                                b"string" => ParameterType::String,
                                b"boolean" => ParameterType::Boolean,
                                b"unsignedInt" => ParameterType::UnsignedInt,
                                b"unsignedShort" => ParameterType::UnsignedShort,
                                b"dateTime" => ParameterType::DateTime,
                                _ => ParameterType::String,
                            }
                        }
                        b"value" => value = String::from_utf8_lossy(&attr.value).to_string(),
                        _ => {}
                    }
                }

                parameters.push(ParameterDeclaration {
                    name,
                    parameter_type,
                    value,
                });
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"ParameterDeclarations" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(parameters)
}

fn parse_road_network(reader: &mut Reader<&[u8]>) -> Result<Option<String>> {
    let mut road_network = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"LogicFile" => {
                for attr in e.attributes() {
                    let attr = attr.map_err(|e| ScenarioError::Parse(e.to_string()))?;
                    if attr.key.as_ref() == b"filepath" {
                        road_network = Some(String::from_utf8_lossy(&attr.value).to_string());
                        break;
                    }
                }
            }
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"LogicFile" => {
                for attr in e.attributes() {
                    let attr = attr.map_err(|e| ScenarioError::Parse(e.to_string()))?;
                    if attr.key.as_ref() == b"filepath" {
                        road_network = Some(String::from_utf8_lossy(&attr.value).to_string());
                        break;
                    }
                }
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"RoadNetwork" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(road_network)
}

fn parse_entities(reader: &mut Reader<&[u8]>) -> Result<HashMap<String, Entity>> {
    const MAX_ENTITIES: usize = 10_000; // Security: Limit entity count
    let mut entities = HashMap::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => {
                if e.name().as_ref() == b"ScenarioObject" {
                    if entities.len() >= MAX_ENTITIES {
                        return Err(ScenarioError::Parse(format!(
                            "Too many entities (>{}). Possible malicious file.",
                            MAX_ENTITIES
                        )));
                    }
                    // Extract name from the already-consumed start element
                    let mut name = String::new();
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"name" {
                            name = unescape_attr(&attr)?;
                            break;
                        }
                    }
                    let entity = parse_scenario_object_body(reader, &name)?;
                    entities.insert(name, entity);
                }
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Entities" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(entities)
}

/// Parse entity body starting after the `<ScenarioObject name="...">` start tag.
fn parse_scenario_object_body(reader: &mut Reader<&[u8]>, name: &str) -> Result<Entity> {
    let mut entity = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"Vehicle" => entity = Some(parse_vehicle(reader, name, &e)?),
                b"Pedestrian" => entity = Some(parse_pedestrian(reader, name, &e)?),
                b"MiscObject" => entity = Some(parse_misc_object(reader, name, &e)?),
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"CatalogReference" => {
                let catalog = parse_catalog_reference_empty(&e)?;
                // Use catalogName as a type hint — lowercase check covers common conventions
                // like "PedestrianCatalog.xosc", "MiscObjectCatalog.xosc", "VehicleCatalog.xosc"
                let lower = catalog.path.to_lowercase();
                if lower.contains("pedestrian") {
                    entity = Some(Entity::Pedestrian(Pedestrian {
                        name: name.to_string(),
                        params: PedestrianParams {
                            catalog: Some(catalog),
                            model: None,
                            mass: None,
                        },
                    }));
                } else if lower.contains("miscobject") || lower.contains("misc_object") {
                    entity = Some(Entity::MiscObject(MiscObject {
                        name: name.to_string(),
                        params: MiscObjectParams {
                            catalog: Some(catalog),
                            category: None,
                            mass: None,
                        },
                    }));
                } else {
                    entity = Some(Entity::Vehicle(Vehicle {
                        name: name.to_string(),
                        params: VehicleParams {
                            catalog: Some(catalog),
                            vehicle_category: VehicleCategory::Car,
                            properties: None,
                        },
                    }));
                }
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"ScenarioObject" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    entity.ok_or_else(|| {
        ScenarioError::Parse(format!(
            "No entity definition found in ScenarioObject '{}'",
            name
        ))
    })
}

fn parse_vehicle(
    reader: &mut Reader<&[u8]>,
    name: &str,
    start_elem: &BytesStart,
) -> Result<Entity> {
    let mut vehicle_category = VehicleCategory::Car;

    // Parse vehicleCategory attribute from the start element
    for attr_result in start_elem.attributes().flatten() {
        if attr_result.key.as_ref() == b"vehicleCategory" {
            let value = String::from_utf8_lossy(&attr_result.value).to_lowercase();
            vehicle_category = match value.as_str() {
                "car" => VehicleCategory::Car,
                "van" => VehicleCategory::Van,
                "truck" => VehicleCategory::Truck,
                "trailer" => VehicleCategory::Trailer,
                "semitrailer" => VehicleCategory::Semitrailer,
                "bus" => VehicleCategory::Bus,
                "motorbike" => VehicleCategory::Motorbike,
                "bicycle" => VehicleCategory::Bicycle,
                "train" => VehicleCategory::Train,
                "tram" => VehicleCategory::Tram,
                _ => VehicleCategory::Car,
            };
            break;
        }
    }

    let mut catalog = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"CatalogReference" => {
                catalog = Some(parse_catalog_reference_empty(&e)?);
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Vehicle" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Entity::Vehicle(Vehicle {
        name: name.to_string(),
        params: VehicleParams {
            catalog,
            vehicle_category,
            properties: None,
        },
    }))
}

fn parse_pedestrian(
    reader: &mut Reader<&[u8]>,
    name: &str,
    start_elem: &BytesStart,
) -> Result<Entity> {
    let mut model = None;
    for attr in start_elem.attributes().flatten() {
        if attr.key.as_ref() == b"model3d" {
            model = Some(String::from_utf8_lossy(&attr.value).to_string());
        }
    }

    let mut catalog = None;
    let mut mass = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Empty(e)) => match e.name().as_ref() {
                b"CatalogReference" => {
                    catalog = Some(parse_catalog_reference_empty(&e)?);
                }
                b"Property" => {
                    let mut prop_name = String::new();
                    let mut prop_value = String::new();
                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"name" => prop_name = String::from_utf8_lossy(&attr.value).to_string(),
                            b"value" => {
                                prop_value = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            _ => {}
                        }
                    }
                    if prop_name == "mass" {
                        if let Ok(v) = prop_value.parse::<f64>() {
                            mass = Some(v);
                        }
                    }
                }
                _ => {}
            },
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Pedestrian" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Entity::Pedestrian(Pedestrian {
        name: name.to_string(),
        params: PedestrianParams {
            catalog,
            model,
            mass,
        },
    }))
}

fn parse_misc_object(
    reader: &mut Reader<&[u8]>,
    name: &str,
    start_elem: &BytesStart,
) -> Result<Entity> {
    let mut category = None;
    for attr in start_elem.attributes().flatten() {
        if attr.key.as_ref() == b"miscObjectCategory" {
            category = Some(String::from_utf8_lossy(&attr.value).to_string());
        }
    }

    let mut mass = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"Property" => {
                let mut prop_name = String::new();
                let mut prop_value = String::new();
                for attr in e.attributes().flatten() {
                    match attr.key.as_ref() {
                        b"name" => prop_name = String::from_utf8_lossy(&attr.value).to_string(),
                        b"value" => prop_value = String::from_utf8_lossy(&attr.value).to_string(),
                        _ => {}
                    }
                }
                if prop_name == "mass" {
                    if let Ok(v) = prop_value.parse::<f64>() {
                        mass = Some(v);
                    }
                }
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"MiscObject" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Entity::MiscObject(MiscObject {
        name: name.to_string(),
        params: MiscObjectParams {
            catalog: None,
            category,
            mass,
        },
    }))
}

fn parse_catalog_reference_empty(e: &BytesStart) -> Result<CatalogReference> {
    let mut path = String::new();
    let mut entry_name = String::new();

    for attr in e.attributes() {
        let attr = attr.map_err(|e| ScenarioError::Parse(e.to_string()))?;
        match attr.key.as_ref() {
            b"catalogName" => path = unescape_attr(&attr)?,
            b"entryName" => entry_name = unescape_attr(&attr)?,
            _ => {}
        }
    }

    Ok(CatalogReference { path, entry_name })
}

// Type alias for complex return type
type StoryboardResult = Result<(Storyboard, HashMap<String, Position>, HashMap<String, f64>)>;

fn parse_storyboard(
    reader: &mut Reader<&[u8]>,
    entities: &HashMap<String, Entity>,
) -> StoryboardResult {
    let mut stories = HashMap::new();
    let mut initial_positions = HashMap::new();
    let mut initial_speeds = HashMap::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"Init" => {
                    (initial_positions, initial_speeds) = parse_init(reader, entities)?;
                }
                b"Story" => {
                    // Extract story name from the already-consumed start element
                    let mut story_name = String::new();
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"name" {
                            story_name = unescape_attr(&attr)?;
                            break;
                        }
                    }
                    let story = parse_story(reader, story_name)?;
                    stories.insert(story.name.clone(), story);
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Storyboard" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok((
        Storyboard {
            stories,
            stop_trigger: None,
        },
        initial_positions,
        initial_speeds,
    ))
}

fn parse_init(
    reader: &mut Reader<&[u8]>,
    _entities: &HashMap<String, Entity>,
) -> Result<(HashMap<String, Position>, HashMap<String, f64>)> {
    let mut initial_positions = HashMap::new();
    let mut initial_speeds = HashMap::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"Private" => {
                let mut entity_ref = String::new();
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"entityRef" {
                        entity_ref = unescape_attr(&attr)?;
                        break;
                    }
                }
                let (pos, speed) = parse_private_section(reader)?;
                if let Some(p) = pos {
                    initial_positions.insert(entity_ref.clone(), p);
                }
                if let Some(s) = speed {
                    initial_speeds.insert(entity_ref, s);
                }
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Init" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok((initial_positions, initial_speeds))
}

/// Parse a `<Private entityRef="..."> ... </Private>` block, returning position and speed.
fn parse_private_section(reader: &mut Reader<&[u8]>) -> Result<(Option<Position>, Option<f64>)> {
    let mut position = None;
    let mut speed = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"PrivateAction" => {
                let (pos, spd) = parse_init_private_action(reader)?;
                if pos.is_some() {
                    position = pos;
                }
                if spd.is_some() {
                    speed = spd;
                }
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Private" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok((position, speed))
}

/// Parse a single `<PrivateAction>` inside an `<Init>` block.
fn parse_init_private_action(
    reader: &mut Reader<&[u8]>,
) -> Result<(Option<Position>, Option<f64>)> {
    let mut position = None;
    let mut speed = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"TeleportAction" => {
                    position = Some(parse_teleport_action(reader)?);
                }
                b"LongitudinalAction" => {
                    speed = parse_longitudinal_action_for_speed(reader)?;
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"PrivateAction" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok((position, speed))
}

fn parse_teleport_action(reader: &mut Reader<&[u8]>) -> Result<Position> {
    let mut position = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"Position" => {
                position = Some(parse_position_element(reader)?);
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"TeleportAction" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    position
        .ok_or_else(|| ScenarioError::Parse("TeleportAction has no Position element".to_string()))
}

fn parse_position_element(reader: &mut Reader<&[u8]>) -> Result<Position> {
    let mut pos = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Empty(e)) => {
                pos = match e.name().as_ref() {
                    b"WorldPosition" => Some(parse_world_position(&e)?),
                    b"LanePosition" => Some(parse_lane_position(&e)?),
                    b"RoadPosition" => Some(parse_road_position(&e)?),
                    _ => None,
                };
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Position" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    pos.ok_or_else(|| {
        ScenarioError::Parse(
            "Position element has no recognized position type (WorldPosition, LanePosition, ...)"
                .to_string(),
        )
    })
}

fn parse_world_position(e: &BytesStart) -> Result<Position> {
    let mut x = 0.0f64;
    let mut y = 0.0f64;
    let mut z = 0.0f64;
    let mut h = 0.0f64;
    let mut p = 0.0f64;
    let mut r = 0.0f64;

    for attr in e.attributes().flatten() {
        let raw = String::from_utf8_lossy(&attr.value);
        let key = attr.key.as_ref();
        let val: f64 = raw.parse().map_err(|_| {
            ScenarioError::Parse(format!(
                "invalid float for WorldPosition attribute '{}': '{}'",
                String::from_utf8_lossy(key),
                raw
            ))
        })?;
        match key {
            b"x" => x = val,
            b"y" => y = val,
            b"z" => z = val,
            b"h" => h = val,
            b"p" => p = val,
            b"r" => r = val,
            _ => {}
        }
    }

    Ok(Position::World { x, y, z, h, p, r })
}

fn parse_lane_position(e: &BytesStart) -> Result<Position> {
    let mut road_id = String::new();
    let mut lane_id = 0i32;
    let mut s = 0.0f64;
    let mut offset = 0.0f64;

    for attr in e.attributes().flatten() {
        let raw = String::from_utf8_lossy(&attr.value);
        let key = attr.key.as_ref();
        match key {
            b"roadId" => road_id = raw.to_string(),
            b"laneId" => {
                lane_id = raw.parse().map_err(|_| {
                    ScenarioError::Parse(format!(
                        "invalid integer for LanePosition laneId: '{}'",
                        raw
                    ))
                })?
            }
            b"s" => {
                s = raw.parse().map_err(|_| {
                    ScenarioError::Parse(format!("invalid float for LanePosition s: '{}'", raw))
                })?
            }
            b"offset" => {
                offset = raw.parse().map_err(|_| {
                    ScenarioError::Parse(format!(
                        "invalid float for LanePosition offset: '{}'",
                        raw
                    ))
                })?
            }
            _ => {}
        }
    }

    Ok(Position::Lane {
        road_id,
        lane_id,
        s,
        offset,
        orientation: None,
    })
}

fn parse_road_position(e: &BytesStart) -> Result<Position> {
    let mut road_id = String::new();
    let mut s = 0.0f64;
    let mut t = 0.0f64;

    for attr in e.attributes().flatten() {
        let raw = String::from_utf8_lossy(&attr.value);
        let key = attr.key.as_ref();
        match key {
            b"roadId" => road_id = raw.to_string(),
            b"s" => {
                s = raw.parse().map_err(|_| {
                    ScenarioError::Parse(format!("invalid float for RoadPosition s: '{}'", raw))
                })?
            }
            b"t" => {
                t = raw.parse().map_err(|_| {
                    ScenarioError::Parse(format!("invalid float for RoadPosition t: '{}'", raw))
                })?
            }
            _ => {}
        }
    }

    Ok(Position::Road {
        road_id,
        s,
        t,
        orientation: None,
    })
}

/// Parse `<LongitudinalAction>...<AbsoluteTargetSpeed value="..."/>...</LongitudinalAction>`
/// and extract the absolute target speed value.
fn parse_longitudinal_action_for_speed(reader: &mut Reader<&[u8]>) -> Result<Option<f64>> {
    let mut speed = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"AbsoluteTargetSpeed" => {
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"value" {
                        speed = String::from_utf8_lossy(&attr.value).parse().ok();
                    }
                }
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"LongitudinalAction" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(speed)
}

/// Parse `<Story name="...">` body starting after the start tag has been consumed.
fn parse_story(reader: &mut Reader<&[u8]>, name: String) -> Result<Story> {
    let mut acts = HashMap::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"Act" => {
                // Extract act name from the already-consumed start element
                let mut act_name = String::new();
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"name" {
                        act_name = unescape_attr(&attr)?;
                        break;
                    }
                }
                let act = parse_act(reader, act_name)?;
                acts.insert(act.name.clone(), act);
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Story" => break,
            Ok(XmlEvent::Eof) => {
                return Err(ScenarioError::Parse("Unexpected EOF in Story".to_string()))
            }
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Story { name, acts })
}

/// Parse `<Act name="...">` body starting after the start tag has been consumed.
fn parse_act(reader: &mut Reader<&[u8]>, name: String) -> Result<Act> {
    let mut maneuver_groups = HashMap::new();
    let mut start_trigger = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"ManeuverGroup" => {
                    let mut mg_name = String::new();
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"name" {
                            mg_name = unescape_attr(&attr)?;
                            break;
                        }
                    }
                    let mg = parse_maneuver_group(reader, mg_name)?;
                    maneuver_groups.insert(mg.name.clone(), mg);
                }
                b"StartTrigger" => {
                    start_trigger = parse_start_trigger(reader, b"StartTrigger")?;
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"StartTrigger" => {
                // Empty <StartTrigger/> — no trigger conditions
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Act" => break,
            Ok(XmlEvent::Eof) => {
                return Err(ScenarioError::Parse("Unexpected EOF in Act".to_string()))
            }
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Act {
        name,
        maneuver_groups,
        start_trigger,
    })
}

/// Parse `<ManeuverGroup name="...">` body starting after the start tag.
fn parse_maneuver_group(reader: &mut Reader<&[u8]>, name: String) -> Result<ManeuverGroup> {
    let mut actors: Vec<String> = Vec::new();
    let mut maneuvers: Vec<Maneuver> = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"Actors" => {
                    actors = parse_actors(reader)?;
                }
                b"Maneuver" => {
                    let mut maneuver_name = String::new();
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"name" {
                            maneuver_name = unescape_attr(&attr)?;
                            break;
                        }
                    }
                    maneuvers.push(parse_maneuver(reader, maneuver_name)?);
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"ManeuverGroup" => break,
            Ok(XmlEvent::Eof) => {
                return Err(ScenarioError::Parse(
                    "Unexpected EOF in ManeuverGroup".to_string(),
                ))
            }
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(ManeuverGroup {
        name,
        actors,
        maneuvers,
    })
}

fn parse_actors(reader: &mut Reader<&[u8]>) -> Result<Vec<String>> {
    let mut actors = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"EntityRef" => {
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"entityRef" {
                        actors.push(unescape_attr(&attr)?);
                    }
                }
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Actors" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(actors)
}

fn parse_maneuver(reader: &mut Reader<&[u8]>, name: String) -> Result<Maneuver> {
    let mut events: Vec<Event> = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"Event" => {
                let mut event_name = String::new();
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"name" {
                        event_name = unescape_attr(&attr)?;
                        break;
                    }
                }
                events.push(parse_event(reader, event_name)?);
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Maneuver" => break,
            Ok(XmlEvent::Eof) => {
                return Err(ScenarioError::Parse(
                    "Unexpected EOF in Maneuver".to_string(),
                ))
            }
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Maneuver { name, events })
}

fn parse_event(reader: &mut Reader<&[u8]>, name: String) -> Result<Event> {
    let mut actions: Vec<Action> = Vec::new();
    let mut start_trigger = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"Action" => {
                    if let Some(action) = parse_action(reader)? {
                        actions.push(action);
                    }
                }
                b"StartTrigger" => {
                    start_trigger = parse_start_trigger(reader, b"StartTrigger")?;
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Ok(XmlEvent::Empty(e)) if e.name().as_ref() == b"StartTrigger" => {
                // Empty <StartTrigger/> — no conditions
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Event" => break,
            Ok(XmlEvent::Eof) => {
                return Err(ScenarioError::Parse("Unexpected EOF in Event".to_string()))
            }
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Event {
        name,
        actions,
        start_trigger,
    })
}

/// Parse `<Action name="..."> <PrivateAction> ... </PrivateAction> </Action>`.
fn parse_action(reader: &mut Reader<&[u8]>) -> Result<Option<Action>> {
    let mut action = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"PrivateAction" => {
                action = parse_private_action_body(reader)?;
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Action" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(action)
}

/// Parse the body of `<PrivateAction>` (inside Story > Act > ManeuverGroup > Event).
fn parse_private_action_body(reader: &mut Reader<&[u8]>) -> Result<Option<Action>> {
    let mut action = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"LongitudinalAction" => {
                    action = parse_longitudinal_action_as_action(reader)?;
                }
                b"LateralAction" => {
                    action = parse_lateral_action(reader)?;
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"PrivateAction" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(action)
}

fn parse_longitudinal_action_as_action(reader: &mut Reader<&[u8]>) -> Result<Option<Action>> {
    let mut action = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"SpeedAction" => {
                action = Some(parse_speed_action(reader)?);
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"LongitudinalAction" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(action)
}

fn parse_speed_action(reader: &mut Reader<&[u8]>) -> Result<Action> {
    let mut target_speed = 0.0f64;
    let mut dynamics = TransitionDynamics {
        shape: DynamicsShape::Linear,
        dimension: DynamicsDimension::Time,
        value: 0.0,
    };
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Empty(e)) => match e.name().as_ref() {
                b"SpeedActionDynamics" => {
                    for attr in e.attributes().flatten() {
                        let raw = String::from_utf8_lossy(&attr.value);
                        match attr.key.as_ref() {
                            b"dynamicsShape" => {
                                dynamics.shape = match raw.as_ref() {
                                    "linear" => DynamicsShape::Linear,
                                    "sinusoidal" => DynamicsShape::Sinusoidal,
                                    "cubic" => DynamicsShape::Cubic,
                                    _ => DynamicsShape::Linear,
                                }
                            }
                            b"dynamicsDimension" => {
                                dynamics.dimension = match raw.as_ref() {
                                    "time" => DynamicsDimension::Time,
                                    "distance" => DynamicsDimension::Distance,
                                    "rate" => DynamicsDimension::Rate,
                                    _ => DynamicsDimension::Time,
                                }
                            }
                            b"value" => dynamics.value = raw.parse().unwrap_or(0.0),
                            _ => {}
                        }
                    }
                }
                b"AbsoluteTargetSpeed" => {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"value" {
                            target_speed =
                                String::from_utf8_lossy(&attr.value).parse().unwrap_or(0.0);
                        }
                    }
                }
                _ => {}
            },
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"SpeedAction" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Action::Speed(SpeedAction {
        target_speed,
        dynamics,
    }))
}

fn parse_lateral_action(reader: &mut Reader<&[u8]>) -> Result<Option<Action>> {
    let mut action = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"LaneChangeAction" => {
                action = Some(parse_lane_change_action(reader)?);
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"LateralAction" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(action)
}

fn parse_lane_change_action(reader: &mut Reader<&[u8]>) -> Result<Action> {
    let mut target_lane_offset = 0.0f64;
    let mut transition_duration = 0.0f64;
    let mut shape = TransitionShape::Linear;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Empty(e)) => match e.name().as_ref() {
                b"LaneChangeActionDynamics" => {
                    for attr in e.attributes().flatten() {
                        let raw = String::from_utf8_lossy(&attr.value);
                        match attr.key.as_ref() {
                            b"dynamicsShape" => {
                                shape = match raw.as_ref() {
                                    "sinusoidal" => TransitionShape::Sinusoidal,
                                    "linear" => TransitionShape::Linear,
                                    "cubic" => TransitionShape::Cubic,
                                    _ => TransitionShape::Linear,
                                }
                            }
                            b"value" => transition_duration = raw.parse().unwrap_or(0.0),
                            _ => {}
                        }
                    }
                }
                b"RelativeTargetLane" => {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"value" {
                            target_lane_offset =
                                String::from_utf8_lossy(&attr.value).parse().unwrap_or(0.0);
                        }
                    }
                }
                _ => {}
            },
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"LaneChangeAction" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Action::LaneChange(LaneChangeAction {
        target_lane_offset,
        transition_duration,
        shape,
    }))
}

/// Parse a `<StartTrigger>` element body (called after the start tag is consumed).
/// Returns `None` if the trigger has no condition groups (default/empty trigger).
fn parse_start_trigger(reader: &mut Reader<&[u8]>, end_tag: &[u8]) -> Result<Option<Trigger>> {
    let mut condition_groups: Vec<ConditionGroup> = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"ConditionGroup" => {
                condition_groups.push(parse_condition_group(reader)?);
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == end_tag => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    if condition_groups.is_empty() {
        return Ok(None);
    }

    Ok(Some(Trigger::with_groups(condition_groups)))
}

fn parse_condition_group(reader: &mut Reader<&[u8]>) -> Result<ConditionGroup> {
    let mut conditions: Vec<Condition> = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == b"Condition" => {
                let mut name = String::new();
                let mut delay = 0.0f64;
                let mut condition_edge = ConditionEdge::None;

                for attr in e.attributes().flatten() {
                    let raw = String::from_utf8_lossy(&attr.value);
                    match attr.key.as_ref() {
                        b"name" => name = raw.to_string(),
                        b"delay" => delay = raw.parse().unwrap_or(0.0),
                        b"conditionEdge" => {
                            condition_edge = match raw.as_ref() {
                                "rising" => ConditionEdge::Rising,
                                "falling" => ConditionEdge::Falling,
                                "risingOrFalling" => ConditionEdge::RisingOrFalling,
                                _ => ConditionEdge::None,
                            }
                        }
                        _ => {}
                    }
                }

                let kind = parse_condition_kind(reader)?;
                conditions.push(Condition {
                    name,
                    delay,
                    condition_edge,
                    kind,
                });
            }
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"ConditionGroup" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(ConditionGroup::new(conditions))
}

fn parse_condition_kind(reader: &mut Reader<&[u8]>) -> Result<ConditionKind> {
    let mut kind = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) => match e.name().as_ref() {
                b"ByValueCondition" => {
                    kind = Some(ConditionKind::ByValue(parse_by_value_condition(reader)?));
                }
                _ => skip_element(reader, e.name().as_ref())?,
            },
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"Condition" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    kind.ok_or_else(|| ScenarioError::Parse("Condition has no recognized kind".to_string()))
}

fn parse_by_value_condition(reader: &mut Reader<&[u8]>) -> Result<ByValueCondition> {
    let mut condition = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Empty(e)) => match e.name().as_ref() {
                b"SimulationTimeCondition" => {
                    let mut value = 0.0f64;
                    let mut rule = Rule::GreaterThan;
                    for attr in e.attributes().flatten() {
                        let raw = String::from_utf8_lossy(&attr.value);
                        match attr.key.as_ref() {
                            b"value" => value = raw.parse().unwrap_or(0.0),
                            b"rule" => rule = parse_rule(&raw),
                            _ => {}
                        }
                    }
                    condition = Some(ByValueCondition::SimulationTime { value, rule });
                }
                b"StoryboardElementStateCondition" => {
                    let mut element_type = String::new();
                    let mut element_ref = String::new();
                    let mut state = String::new();
                    for attr in e.attributes().flatten() {
                        let raw = String::from_utf8_lossy(&attr.value).to_string();
                        match attr.key.as_ref() {
                            b"storyboardElementType" => element_type = raw,
                            b"storyboardElementRef" => element_ref = raw,
                            b"state" => state = raw,
                            _ => {}
                        }
                    }
                    condition = Some(ByValueCondition::StoryboardElementState {
                        element_type,
                        element_ref,
                        state,
                    });
                }
                b"ParameterCondition" => {
                    let mut parameter_ref = String::new();
                    let mut value = String::new();
                    let mut rule = Rule::EqualTo;
                    for attr in e.attributes().flatten() {
                        let raw = String::from_utf8_lossy(&attr.value).to_string();
                        match attr.key.as_ref() {
                            b"parameterRef" => parameter_ref = raw,
                            b"value" => value = raw,
                            b"rule" => rule = parse_rule(&raw),
                            _ => {}
                        }
                    }
                    condition = Some(ByValueCondition::Parameter(ParameterCondition {
                        parameter_ref,
                        value,
                        rule,
                    }));
                }
                _ => {}
            },
            Ok(XmlEvent::End(e)) if e.name().as_ref() == b"ByValueCondition" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    condition.ok_or_else(|| {
        ScenarioError::Parse("ByValueCondition has no recognized condition type".to_string())
    })
}

fn parse_rule(s: &str) -> Rule {
    match s {
        "greaterThan" => Rule::GreaterThan,
        "lessThan" => Rule::LessThan,
        "equalTo" => Rule::EqualTo,
        "greaterOrEqual" => Rule::GreaterOrEqual,
        "lessOrEqual" => Rule::LessOrEqual,
        _ => Rule::GreaterThan,
    }
}

// Helper functions

fn skip_element(reader: &mut Reader<&[u8]>, element_name: &[u8]) -> Result<()> {
    skip_to_end(reader, element_name)
}

fn skip_to_end(reader: &mut Reader<&[u8]>, end_name: &[u8]) -> Result<()> {
    const MAX_DEPTH: usize = 100; // Security: Prevent stack overflow from deeply nested XML
    let mut depth = 1;
    let mut buf = Vec::new();

    loop {
        // Security check: Enforce maximum nesting depth
        if depth > MAX_DEPTH {
            return Err(ScenarioError::Parse(format!(
                "XML nesting too deep (>{} levels). Possible XML bomb attack or malformed file.",
                MAX_DEPTH
            )));
        }

        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(e)) if e.name().as_ref() == end_name => depth += 1,
            Ok(XmlEvent::End(e)) if e.name().as_ref() == end_name => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(ScenarioError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}
