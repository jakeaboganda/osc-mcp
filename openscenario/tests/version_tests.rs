use openscenario::OpenScenarioVersion;

#[test]
fn test_version_from_header() {
    let v = OpenScenarioVersion::from_rev(1, 0);
    assert_eq!(v, Some(OpenScenarioVersion::V1_0));

    let v = OpenScenarioVersion::from_rev(1, 2);
    assert_eq!(v, Some(OpenScenarioVersion::V1_2));

    let v = OpenScenarioVersion::from_rev(2, 0);
    assert_eq!(v, None);
}

#[test]
fn test_version_to_string() {
    assert_eq!(OpenScenarioVersion::V1_0.to_string(), "1.0");
    assert_eq!(OpenScenarioVersion::V1_2.to_string(), "1.2");
}

#[test]
fn test_version_comparison() {
    assert!(OpenScenarioVersion::V1_2 > OpenScenarioVersion::V1_0);
    assert!(OpenScenarioVersion::V1_1 >= OpenScenarioVersion::V1_1);
}
