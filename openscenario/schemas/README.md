# OpenSCENARIO XSD Schemas

## ✅ Official Schemas Installed

This directory contains **official ASAM OpenSCENARIO XSD schemas** for the following versions:

- **v1.1.1** - OpenSCENARIO 1.1.1
- **v1.2.0** - OpenSCENARIO 1.2.0  
- **v1.3.0** - OpenSCENARIO 1.3.0
- **v1.3.1** - OpenSCENARIO 1.3.1 (Latest)

These are the **complete, production-ready schemas** from ASAM.

---

## Supported Versions

The library now supports **four OpenSCENARIO versions**:

| Version | Schema Location | Status |
|---------|----------------|---------|
| **1.1.1** | `v1.1.1/OpenSCENARIO.xsd` | ✅ Full XSD validation |
| **1.2.0** | `v1.2.0/OpenSCENARIO.xsd` | ✅ Full XSD validation |
| **1.3.0** | `v1.3.0/OpenSCENARIO.xsd` | ✅ Full XSD validation |
| **1.3.1** | `v1.3.1/OpenSCENARIO.xsd` | ✅ Full XSD validation (Latest) |

---

## Version Selection

The validator automatically selects the correct XSD based on the scenario's `revMajor` and `revMinor` attributes:

```xml
<FileHeader revMajor="1" revMinor="2" ...>
  <!-- Uses v1.2.0 schema -->
</FileHeader>
```

**Mapping**:
- `revMajor="1" revMinor="1"` → v1.1.1 schema
- `revMajor="1" revMinor="2"` → v1.2.0 schema
- `revMajor="1" revMinor="3"` → v1.3.0 or v1.3.1 schema

---

## Legacy Versions

These directories remain for backward compatibility but are no longer actively supported:

- `v1.0/` - OpenSCENARIO 1.0 (legacy)
- `v1.1/` - OpenSCENARIO 1.1.0 (superseded by 1.1.1)
- `v1.2/` - Contains minimal test schema (superseded by v1.2.0)

**Recommendation**: Use v1.1.1, v1.2.0, or v1.3.0 for new scenarios.

---

## Validation Behavior

### With Official Schemas (Current State)

✅ **Full XSD Validation**:
- Complete element structure checking
- Attribute constraint validation
- Cardinality enforcement (minOccurs, maxOccurs)
- Data type validation
- Enumeration checking

**Example**:
```rust
let report = scenario.validate_with_xsd()?;
assert!(report.valid);  // Strict validation
```

### Without Schemas (Fallback)

If schemas are missing, the validator returns an error:

```rust
ValidationReport {
    valid: false,
    errors: vec!["XSD schema not available for OpenSCENARIO v1.2.0. Full validation requires official ASAM XSD files."],
    warnings: vec![]
}
```

**Strict Mode**: No graceful fallback. This ensures you know when full validation isn't happening.

---

## Schema Sources

These schemas are the **official ASAM OpenSCENARIO XML schemas**:

- **Source**: ASAM e.V. (Association for Standardization of Automation and Measuring Systems)
- **Standard**: https://www.asam.net/standards/detail/openscenario/
- **License**: ASAM license terms
- **Copyright**: © ASAM e.V., 2021-2024

**Usage**: These files are distributable under ASAM license terms for implementation purposes.

---

## Local Patch: OpenScenarioCategory Inlined

`OpenSCENARIO.xsd` in `v1.1.1/`, `v1.2.0/`, `v1.3.0/`, and `v1.3.1/` each carry one
deliberate deviation from the official ASAM download, applied identically to all four
files at the `OpenScenario` complexType:

```xml
<!-- Official ASAM schema -->
<xsd:complexType name="OpenScenario">
    <xsd:sequence>
        <xsd:element name="FileHeader" type="FileHeader"/>
        <xsd:group ref="OpenScenarioCategory"/>
    </xsd:sequence>
</xsd:complexType>
<xsd:group name="OpenScenarioCategory">
    <xsd:choice>
        <xsd:group ref="ScenarioDefinition"/>
        <xsd:group ref="CatalogDefinition"/>
        <xsd:group ref="ParameterValueDistributionDefinition"/>
    </xsd:choice>
</xsd:group>
```

```xml
<!-- Patched: choice inlined directly into OpenScenario's sequence -->
<xsd:complexType name="OpenScenario">
    <xsd:sequence>
        <xsd:element name="FileHeader" type="FileHeader"/>
        <xsd:choice>
            <xsd:group ref="ScenarioDefinition"/>
            <xsd:group ref="CatalogDefinition"/>
            <xsd:group ref="ParameterValueDistributionDefinition"/>
        </xsd:choice>
    </xsd:sequence>
</xsd:complexType>
<!-- OpenScenarioCategory is left in place, unreferenced, to keep the diff minimal -->
```

**Why**: the `uppsala` crate (pinned to 0.4.0 in `openscenario/Cargo.toml`; confirmed
still present in 0.9.0, the latest release on crates.io as of this writing) fails to
resolve an `<xsd:choice>` when its alternatives are `<xsd:group ref>` particles reached
through *another* intermediate `<xsd:group ref>` (double indirection). Every child of
whichever branch should have matched gets reported as `"Unexpected element '...' in
sequence"`. This is the exact shape of the real schema's root: `OpenScenario` ->
`group ref="OpenScenarioCategory"` -> `choice` of three `group ref`s. The bug is
independent of scenario content — it broke every single OpenSCENARIO document,
scenario or catalog, at the root element, before this patch. It reproduces in a
20-line schema with no ASAM content at all; see the git history of this file / the
`osc-mcp` project's development notes for the isolated repro. Inlining the choice one
level up (so the choice's alternatives are reached with only one `<xsd:group ref>`
hop, not two) works around it without touching the validator crate.

**Consequence**: these four `OpenSCENARIO.xsd` files are *not* byte-identical to the
official ASAM download. If you ever re-sync a schema from ASAM (e.g. to pick up a new
patch release), re-apply this same edit, or `validate_scenario` will silently regress
to reporting every valid scenario document as invalid. Check with:
```bash
grep -A2 'group ref="OpenScenarioCategory"' schemas/v*/OpenSCENARIO.xsd
```
If this prints the *unpatched* form (`OpenScenario`'s sequence containing only
`FileHeader` + `group ref="OpenScenarioCategory"`, no inline `<xsd:choice>`), the patch
needs to be re-applied.

Separately, `openscenario/src/validation.rs`'s `SUPPORTED_SCHEMA_DIRS` maps
`"1.0"/"1.1"/"1.2"/"1.3"` version strings to these directories; that mapping — not the
directory names themselves — is what previously pointed at empty stub directories
(`v1.0/`, `v1.1/`, `v1.2/`, none of which contain `OpenSCENARIO.xsd`) instead of the
real ones added here, silently making `validate_scenario` always report "schema not
available" regardless of scenario content. Both bugs had to be fixed together for
validation to actually run.

---

## Version Differences

### v1.1.1 → v1.2.0
- New `ColorType`, `AutomaticGearType` enumerations
- `ControllerType` enum added
- `DirectionalDimension` for acceleration conditions
- Enhanced lighting and animation support
- `VariableAction` and `VariableCondition` added
- Improved vehicle component definitions

### v1.2.0 → v1.3.0
- New `AngleType`, `AngleCondition` for orientation checks
- `RelativeAngleCondition` added
- `CoordinateSystem` enum extended (added `world`)
- `ClothoidSpline` and `ClothoidSplineSegment` for advanced trajectories
- `ConnectTrailerAction`, `DisconnectTrailerAction` for trailer handling
- `SetMonitorAction` and `MonitorDeclarations` for monitoring
- `LogNormalDistribution` for stochastic distributions
- `Lane` element for lane references
- Enhanced controller management (`objectControllerRef`)
- Improved `GeoPosition` with vertical road selection

### v1.3.0 → v1.3.1
- Bug fixes and minor improvements
- Schema refinements for edge cases
- Improved validation coverage
- Maintains full backward compatibility with v1.3.0

---

## Adding New Versions

To add support for new OpenSCENARIO versions:

1. **Create version directory**:
   ```bash
   mkdir -p schemas/v1.4.0
   ```

2. **Add official XSD**:
   ```bash
   cp /path/to/OpenSCENARIO_v1.4.0.xsd schemas/v1.4.0/OpenSCENARIO.xsd
   ```

3. **Update validator code** (if needed):
   - `openscenario/src/validation.rs` - Add version mapping
   - Usually automatic if schema follows naming convention

4. **Test validation**:
   ```bash
   cargo test validation_tests
   ```

---

## Verification

Check schema installation:

```bash
cd ../..  # Back to openscenario/ directory
./check-schemas.sh
```

Expected output:
```
✅ Found: schemas/v1.1.1/OpenSCENARIO.xsd
✅ Found: schemas/v1.2.0/OpenSCENARIO.xsd  
✅ Found: schemas/v1.3.0/OpenSCENARIO.xsd
✅ Found: schemas/v1.3.1/OpenSCENARIO.xsd

✅ All XSD schema files present!
```

Or check with `find`:
```bash
cd schemas
find . -name "OpenSCENARIO.xsd" | sort
```

---

## File Requirements

**For Uppsala validator to recognize schemas:**
- File MUST be named exactly: `OpenSCENARIO.xsd`
- File MUST be XML Schema Definition (XSD) format
- File MUST be in version-specific directory (e.g., `v1.2.0/`)

**Directory naming**:
- Use semantic versioning: `v{major}.{minor}.{patch}`
- Examples: `v1.1.1`, `v1.2.0`, `v1.3.0`

---

## License & Copyright

**OpenSCENARIO XSD Schemas**:
- Copyright © ASAM e.V., 2021-2024
- Licensed under ASAM terms
- See: https://www.asam.net/license.html

**This Library (openscenario-rs)**:
- Uses schemas for validation purposes
- Compliant with ASAM implementation guidelines

---

## Summary

✅ **Four production versions supported**: 1.1.1, 1.2.0, 1.3.0, 1.3.1  
✅ **Full XSD validation** for all supported versions  
✅ **Automatic version detection** from FileHeader  
✅ **Strict validation mode** - no silent failures  

**Current Status**: Ready for production use with complete schema coverage.
**Latest Version**: v1.3.1 (recommended for new scenarios)
