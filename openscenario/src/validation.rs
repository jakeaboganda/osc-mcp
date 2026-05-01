use quick_xml::Reader;

/// Validation report containing results and any errors
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Whether the XML is valid
    pub valid: bool,
    /// List of validation errors
    pub errors: Vec<String>,
}

/// XSD validator for OpenSCENARIO XML documents
#[derive(Debug, Clone)]
pub struct XsdValidator {
    version: String,
}

impl XsdValidator {
    /// Create a new validator for the specified OpenSCENARIO version
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
        }
    }

    /// Validate XML content
    ///
    /// Performs basic XML well-formedness checking and version validation.
    /// Note: This is a basic implementation; full XSD validation would require
    /// runtime schema loading which xsd-parser doesn't support.
    pub fn validate(&self, xml: &str) -> ValidationReport {
        let mut errors = Vec::new();

        // Check XML well-formedness
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut found_file_header = false;
        let mut file_header_version = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(quick_xml::events::Event::Start(e)) | Ok(quick_xml::events::Event::Empty(e))
                    if e.name().as_ref() == b"FileHeader" =>
                {
                    found_file_header = true;
                    // Extract revMajor and revMinor attributes
                    let mut rev_major = None;
                    let mut rev_minor = None;

                    for attr in e.attributes() {
                        match attr {
                            Ok(attr) => match attr.key.as_ref() {
                                b"revMajor" => {
                                    if let Ok(value) = attr.unescape_value() {
                                        rev_major = Some(value.to_string());
                                    }
                                }
                                b"revMinor" => {
                                    if let Ok(value) = attr.unescape_value() {
                                        rev_minor = Some(value.to_string());
                                    }
                                }
                                _ => {}
                            },
                            Err(e) => {
                                errors.push(format!("Error reading attribute: {}", e));
                            }
                        }
                    }

                    if let (Some(major), Some(minor)) = (rev_major, rev_minor) {
                        file_header_version = Some(format!("{}.{}", major, minor));
                    }
                }
                Ok(quick_xml::events::Event::Eof) => break,
                Err(e) => {
                    errors.push(format!("XML parsing error: {}", e));
                    break;
                }
                _ => {}
            }
            buf.clear();
        }

        // Validate version match
        if let Some(file_version) = file_header_version {
            if file_version != self.version {
                errors.push(format!(
                    "Version mismatch: expected {}, found {}",
                    self.version, file_version
                ));
            }
        } else if found_file_header {
            errors.push("FileHeader found but version attributes missing".to_string());
        }

        ValidationReport {
            valid: errors.is_empty(),
            errors,
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
        assert!(report.valid);
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
    }
}
