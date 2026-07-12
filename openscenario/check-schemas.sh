#!/bin/bash
# Check if OpenSCENARIO XSD schema files are present

SCHEMA_DIR="$(dirname "$0")/schemas"
MISSING=()

# Must match SUPPORTED_SCHEMA_DIRS in src/validation.rs.
for version in v1.1.1 v1.2.0 v1.3.1; do
    xsd_file="$SCHEMA_DIR/$version/OpenSCENARIO.xsd"
    if [ ! -f "$xsd_file" ]; then
        MISSING+=("$version")
    else
        echo "✅ Found: $xsd_file"
    fi
done

if [ ${#MISSING[@]} -gt 0 ]; then
    echo ""
    echo "⚠️  Missing XSD files for: ${MISSING[*]}"
    echo ""
    echo "To obtain XSD files:"
    echo "1. Visit: https://www.asam.net/standards/detail/openscenario/"
    echo "2. Download the OpenSCENARIO release matching the missing directory"
    echo "3. Extract OpenSCENARIO.xsd to schemas/<version>/"
    echo ""
    echo "Note: OpenSCENARIO 1.0 has no official full XSD checked in (schemas/v1.0/ is"
    echo "a stub); validate_scenario reports 'schema not available' for that version."
    exit 1
else
    echo ""
    echo "✅ All XSD schema files present!"
    exit 0
fi
