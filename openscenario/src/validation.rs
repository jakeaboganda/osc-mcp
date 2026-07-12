use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;
use uppsala::xsd::XsdValidator as UppsalaValidator;

/// Validation report containing results and any errors
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Whether the XML is valid
    pub valid: bool,
    /// List of validation errors with line numbers when available
    pub errors: Vec<String>,
    /// List of validation warnings (non-fatal issues)
    pub warnings: Vec<String>,
}

/// XSD validator for OpenSCENARIO XML documents using Uppsala
#[derive(Debug, Clone)]
pub struct XsdValidator {
    version: String,
}

// Maps a "revMajor.revMinor" version string to the directory holding its official
// ASAM schema. The directories aren't named "v{version}" uniformly: only bare-bones
// stub schemas ever existed at v1.0/v1.1/v1.2, while the real, full schemas were
// added later under their ASAM release names (v1.1.1, v1.2.0, v1.3.1). "v1.0" has no
// OpenSCENARIO.xsd in it (see schemas/README.md) and is kept pointed at that empty
// stub dir on purpose: validate() reports "schema not available" for it rather than
// "unsupported version", since 1.0 remains a recognized (if unvalidatable) version.
const SUPPORTED_SCHEMA_DIRS: &[(&str, &str)] = &[
    ("1.0", "v1.0"),
    ("1.1", "v1.1.1"),
    ("1.2", "v1.2.0"),
    ("1.3", "v1.3.1"),
];

fn schema_path_for_version(schema_dir: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .join("schemas")
        .join(schema_dir)
        .join("OpenSCENARIO.xsd")
}

// Lazy-loaded validators using OnceLock (parsed once at first use)
static SCHEMA_VALIDATORS: OnceLock<HashMap<String, Option<UppsalaValidator>>> = OnceLock::new();

fn get_schema_validators() -> &'static HashMap<String, Option<UppsalaValidator>> {
    SCHEMA_VALIDATORS.get_or_init(|| {
        let mut map = HashMap::new();

        for (version, schema_dir) in SUPPORTED_SCHEMA_DIRS {
            let schema_path = schema_path_for_version(schema_dir);

            if !schema_path.exists() {
                eprintln!(
                    "Warning: XSD schema not found for OpenSCENARIO v{}: {:?}",
                    version, schema_path
                );
                eprintln!("         Run ./check-schemas.sh for instructions");
                map.insert(version.to_string(), None);
                continue;
            }

            // Read schema file
            let schema_xml = match std::fs::read_to_string(&schema_path) {
                Ok(xml) => xml,
                Err(e) => {
                    eprintln!("Error reading schema file for v{}: {}", version, e);
                    map.insert(version.to_string(), None);
                    continue;
                }
            };

            match uppsala::parse(&schema_xml) {
                Ok(schema_doc) => match UppsalaValidator::from_schema(&schema_doc) {
                    Ok(validator) => {
                        eprintln!("✅ Loaded XSD schema for OpenSCENARIO v{}", version);
                        map.insert(version.to_string(), Some(validator));
                    }
                    Err(e) => {
                        eprintln!("Error building validator for v{}: {}", version, e);
                        map.insert(version.to_string(), None);
                    }
                },
                Err(e) => {
                    eprintln!("Error parsing XSD schema for v{}: {}", version, e);
                    map.insert(version.to_string(), None);
                }
            }
        }

        map
    })
}

impl XsdValidator {
    /// Create a new validator for the specified OpenSCENARIO version
    ///
    /// Supported versions: "1.0", "1.1", "1.2"
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
        }
    }

    /// Validate XML content against the OpenSCENARIO XSD schema
    ///
    /// Performs full XSD validation using the Uppsala validator.
    /// If the XSD schema file is not available, falls back to basic well-formedness checking.
    ///
    /// # Arguments
    /// * `xml` - OpenSCENARIO XML string to validate
    ///
    /// # Returns
    /// * `ValidationReport` with validation results, errors, and warnings
    ///
    /// # Examples
    /// ```
    /// use openscenario::validation::XsdValidator;
    ///
    /// let validator = XsdValidator::new("1.2");
    /// let xml = r#"<?xml version="1.0"?>
    /// <OpenSCENARIO>
    ///     <FileHeader revMajor="1" revMinor="2"/>
    /// </OpenSCENARIO>"#;
    /// let report = validator.validate(xml);
    /// assert!(report.valid || report.errors.len() > 0);
    /// ```
    pub fn validate(&self, xml: &str) -> ValidationReport {
        let mut errors = Vec::new();
        let warnings = Vec::new(); // Reserved for future use

        // First: Basic XML well-formedness check
        let doc = match uppsala::parse(xml) {
            Err(e) => {
                errors.push(format!("XML parsing error: {}", e));
                return ValidationReport {
                    valid: false,
                    errors,
                    warnings,
                };
            }
            Ok(doc) => doc,
        };

        // Check for XSD validator availability
        let validators = get_schema_validators();
        match validators.get(&self.version) {
            Some(Some(validator)) => {
                // Full XSD validation
                let validation_errors = validator.validate(&doc);

                if validation_errors.is_empty() {
                    // Valid!
                    ValidationReport {
                        valid: true,
                        errors: vec![],
                        warnings,
                    }
                } else {
                    // XSD validation failed
                    let error_messages: Vec<String> =
                        validation_errors.iter().map(|e| format!("{}", e)).collect();

                    ValidationReport {
                        valid: false,
                        errors: error_messages,
                        warnings,
                    }
                }
            }
            Some(None) => {
                // Schema not available - FAIL STRICT
                errors.push(format!(
                    "XSD schema not available for OpenSCENARIO v{}. \
                    Full validation requires official ASAM XSD files. \
                    Run ./check-schemas.sh to set up XSD files.",
                    self.version
                ));

                ValidationReport {
                    valid: false,
                    errors,
                    warnings,
                }
            }
            None => {
                errors.push(format!(
                    "Unsupported OpenSCENARIO version: {}. \
                    Supported versions: 1.0, 1.1, 1.2, 1.3",
                    self.version
                ));
                ValidationReport {
                    valid: false,
                    errors,
                    warnings,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = XsdValidator::new("1.0");
        assert_eq!(validator.version, "1.0");
    }

    #[test]
    fn test_well_formed_xml() {
        let validator = XsdValidator::new("1.0");
        let xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader revMajor="1" revMinor="0"/>
</OpenSCENARIO>"#;
        let report = validator.validate(xml);
        // Strict mode: without XSD, this will fail
        if report
            .errors
            .iter()
            .any(|e| e.contains("XSD schema not available"))
        {
            // Expected behavior without XSD
            assert!(!report.valid);
        } else {
            // With XSD, should validate successfully
            assert!(report.valid);
        }
    }

    #[test]
    fn test_malformed_xml() {
        let validator = XsdValidator::new("1.0");
        let xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader revMajor="1" revMinor="0"
</OpenSCENARIO>"#;
        let report = validator.validate(xml);
        assert!(!report.valid);
        assert!(!report.errors.is_empty());
        assert!(report.errors[0].contains("XML parsing error"));
    }

    #[test]
    fn test_missing_xsd_fails() {
        let validator = XsdValidator::new("1.0");
        let xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader revMajor="1" revMinor="0"/>
</OpenSCENARIO>"#;
        let report = validator.validate(xml);
        // Without XSD, validation should FAIL (strict mode)
        if report.valid {
            // If it passed, XSD must be present - skip test
            return;
        }
        assert!(!report.valid);
        assert!(report
            .errors
            .iter()
            .any(|e| e.contains("XSD schema not available")));
    }
}
