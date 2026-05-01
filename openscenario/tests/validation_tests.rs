use openscenario::validation::XsdValidator;

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
    assert!(report.valid, "Valid XML should pass validation");
    assert!(report.errors.is_empty(), "No errors expected for valid XML");
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
fn test_version_mismatch() {
    let validator = XsdValidator::new("1.0");
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO xmlns="http://www.asam.net/xsd/OpenSCENARIO">
    <FileHeader revMajor="1" revMinor="2" date="2024-01-01T00:00:00" description="Test" author="Test"/>
    <ParameterDeclarations/>
</OpenSCENARIO>"#;

    let report = validator.validate(xml);
    assert!(!report.valid, "Version mismatch should fail validation");
    assert!(
        report
            .errors
            .iter()
            .any(|e| e.to_lowercase().contains("version")),
        "Should report version mismatch error"
    );
}
