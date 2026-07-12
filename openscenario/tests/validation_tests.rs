use openscenario::entities::{VehicleCategory, VehicleParams};
use openscenario::storyboard::{DynamicsDimension, DynamicsShape, TransitionDynamics};
use openscenario::validation::XsdValidator;
use openscenario::{OpenScenarioVersion, Position, Scenario};

#[test]
fn test_validate_v1_0_scenario() {
    let validator = XsdValidator::new("1.0");
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO xmlns="http://www.asam.net/xsd/OpenSCENARIO">
    <FileHeader revMajor="1" revMinor="0" date="2024-01-01T00:00:00" description="Test" author="Test"/>
    <ParameterDeclarations/>
    <CatalogLocations/>
    <RoadNetwork/>
    <Entities/>
    <Storyboard/>
</OpenSCENARIO>"#;

    let report = validator.validate(xml);
    // NOTE: Without official XSD files, validation will fail (strict mode)
    // This test verifies well-formed XML is parseable, not XSD-valid
    if !report.valid
        && report
            .errors
            .iter()
            .any(|e| e.contains("XSD schema not available"))
    {
        eprintln!("Skipping validation check - XSD files not installed");
        return; // Skip test if XSD missing
    }
    assert!(report.valid, "Valid XML should pass validation");
}

#[test]
fn test_invalid_xml() {
    let validator = XsdValidator::new("1.0");
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO xmlns="http://www.asam.net/xsd/OpenSCENARIO">
    <FileHeader revMajor="1" revMinor="0"
    <!-- Missing closing tag -->
</OpenSCENARIO>"#;

    let report = validator.validate(xml);
    assert!(!report.valid, "Malformed XML should fail validation");
    assert!(
        !report.errors.is_empty(),
        "Errors expected for malformed XML"
    );
}

#[test]
fn test_missing_xsd_strict() {
    let validator = XsdValidator::new("1.0");
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO xmlns="http://www.asam.net/xsd/OpenSCENARIO">
    <FileHeader revMajor="1" revMinor="0" date="2024-01-01T00:00:00" description="Test" author="Test"/>
    <ParameterDeclarations/>
</OpenSCENARIO>"#;

    let report = validator.validate(xml);
    // Without XSD, validation should FAIL (strict mode)
    if report.valid {
        // If it passed, XSD must be present - skip this specific test
        return;
    }
    assert!(!report.valid, "Should fail without XSD files");
    assert!(
        report
            .errors
            .iter()
            .any(|e| e.contains("XSD schema not available")),
        "Should report missing XSD as error"
    );
}

// v1.1, v1.2 and v1.3 ship real ASAM schemas in openscenario/schemas/ (v1.1.1, v1.2.0,
// v1.3.1 respectively). Unlike the v1.0 tests above, these must not soft-skip: a
// "schema not available" result here means the version-to-schema-path mapping is
// broken. v1.1.1 requires at least one Story under Storyboard (relaxed to optional in
// 1.2+), so build a real scenario via the crate's own API rather than hand-writing XML
// that has to independently satisfy every nested XSD requirement (Act, ManeuverGroup,
// etc.) for each version.
fn valid_scenario_xml(version: OpenScenarioVersion) -> String {
    let mut s = Scenario::new(version);
    s.add_vehicle(
        "ego",
        VehicleParams {
            catalog: None,
            vehicle_category: VehicleCategory::Car,
            properties: None,
        },
    )
    .unwrap();
    s.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))
        .unwrap();
    s.add_story("story1").unwrap();
    s.add_act("story1", "act1").unwrap();
    s.add_maneuver_group("story1", "act1", "ego_mg").unwrap();
    s.add_actor("story1", "act1", "ego_mg", "ego").unwrap();
    s.add_maneuver("story1", "act1", "ego_mg", "maneuver1")
        .unwrap();
    s.add_speed_action(
        "story1",
        "act1",
        "ego_mg",
        "maneuver1",
        "event1",
        10.0,
        TransitionDynamics {
            shape: DynamicsShape::Linear,
            dimension: DynamicsDimension::Time,
            value: 5.0,
        },
    )
    .unwrap();
    s.to_xml()
        .expect("to_xml should succeed for a complete scenario")
}

#[test]
fn test_validate_v1_1_scenario_uses_real_schema() {
    let validator = XsdValidator::new("1.1");
    let report = validator.validate(&valid_scenario_xml(OpenScenarioVersion::V1_1));
    assert!(
        !report
            .errors
            .iter()
            .any(|e| e.contains("XSD schema not available")),
        "v1.1.1 schema is checked into openscenario/schemas/ and must load: {:?}",
        report.errors
    );
    assert!(
        report.valid,
        "Valid v1.1 XML should pass: {:?}",
        report.errors
    );
}

#[test]
fn test_validate_v1_2_scenario_uses_real_schema() {
    let validator = XsdValidator::new("1.2");
    let report = validator.validate(&valid_scenario_xml(OpenScenarioVersion::V1_2));
    assert!(
        !report
            .errors
            .iter()
            .any(|e| e.contains("XSD schema not available")),
        "v1.2.0 schema is checked into openscenario/schemas/ and must load: {:?}",
        report.errors
    );
    assert!(
        report.valid,
        "Valid v1.2 XML should pass: {:?}",
        report.errors
    );
}

#[test]
fn test_validate_v1_3_scenario_uses_real_schema() {
    let validator = XsdValidator::new("1.3");
    let report = validator.validate(&valid_scenario_xml(OpenScenarioVersion::V1_3));
    assert!(
        !report
            .errors
            .iter()
            .any(|e| e.contains("XSD schema not available")),
        "v1.3.1 schema is checked into openscenario/schemas/ and must load: {:?}",
        report.errors
    );
    assert!(
        report.valid,
        "Valid v1.3 XML should pass: {:?}",
        report.errors
    );
}

#[test]
fn test_validate_v1_2_invalid_scenario_rejected() {
    // ScenarioObject's `name` attribute is `use="required"` in the XSD; omitting it
    // would be caught by real XSD validation but not by the old "schema not available"
    // fallback.
    let validator = XsdValidator::new("1.2");
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
    <FileHeader revMajor="1" revMinor="2" date="2024-01-01T00:00:00" description="Test" author="Test"/>
    <CatalogLocations/>
    <RoadNetwork/>
    <Entities>
        <ScenarioObject>
            <Vehicle name="ego" vehicleCategory="car">
                <BoundingBox>
                    <Center x="0" y="0" z="0"/>
                    <Dimensions width="2" length="4" height="1.5"/>
                </BoundingBox>
                <Performance maxSpeed="50" maxDeceleration="10" maxAcceleration="5"/>
                <Axles>
                    <FrontAxle maxSteering="1" wheelDiameter="0.6" trackWidth="1.6" positionX="2.5" positionZ="0.3"/>
                    <RearAxle maxSteering="1" wheelDiameter="0.6" trackWidth="1.6" positionX="0" positionZ="0.3"/>
                </Axles>
            </Vehicle>
        </ScenarioObject>
    </Entities>
    <Storyboard>
        <Init>
            <Actions/>
        </Init>
        <StopTrigger/>
    </Storyboard>
</OpenSCENARIO>"#;

    let report = validator.validate(xml);
    assert!(
        !report.valid,
        "ScenarioObject without its required 'name' attribute violates the XSD"
    );
    assert!(!report.errors.is_empty());
}
