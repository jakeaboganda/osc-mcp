#!/bin/bash
# generate_with_roads.sh - Generate OpenSCENARIO files with road network references

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR"
ROAD_FILE="roads/simple_highway.xodr"

# Check if example name provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <example_name>"
    echo ""
    echo "Available examples:"
    ls -1 "$PROJECT_ROOT/openscenario/examples/"*.rs | xargs -n1 basename | sed 's/.rs$//' | sed 's/^/  /'
    exit 1
fi

EXAMPLE=$1
XOSC_FILE="${EXAMPLE}.xosc"

echo "📝 Generating scenario: $EXAMPLE"

# Generate the scenario
cd "$PROJECT_ROOT/openscenario"
cargo run --example "$EXAMPLE" --quiet

# Check if file was generated
cd "$PROJECT_ROOT"
if [ ! -f "$XOSC_FILE" ]; then
    echo "❌ Error: $XOSC_FILE not found"
    exit 1
fi

echo "🛣️  Adding road network reference..."

# Update the RoadNetwork element
sed -i.bak "s|<RoadNetwork/>|<RoadNetwork>\\
    <LogicFile filepath=\"$ROAD_FILE\"/>\\
  </RoadNetwork>|" "$XOSC_FILE"

# Remove backup file
rm -f "${XOSC_FILE}.bak"

echo "✅ Generated $XOSC_FILE with road network"
echo ""
echo "To visualize:"
echo "  esmini --osc $XOSC_FILE"
echo ""
echo "Or using installed esmini:"
echo "  ~/.openclaw/workspace/tools/esmini-demo/bin/esmini --osc $PWD/$XOSC_FILE"
