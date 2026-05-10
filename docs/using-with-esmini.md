# Using OpenSCENARIO Scenarios with esmini

This guide explains how to run the generated scenarios in esmini with proper road networks.

---

## Quick Start

### 1. Generate Scenarios

```bash
cd openscenario
cargo run --example highway_merge
# Creates: highway_merge.xosc
```

### 2. Add Road Network Reference

The generated .xosc files need a road network. Edit the file and update the `<RoadNetwork>` section:

**Before:**
```xml
<RoadNetwork/>
```

**After:**
```xml
<RoadNetwork>
    <LogicFile filepath="roads/simple_highway.xodr"/>
</RoadNetwork>
```

### 3. Run in esmini

```bash
cd ~/.openclaw/workspace/tools/esmini-demo
./bin/esmini --osc ../../osc-mcp/highway_merge.xosc
```

---

## Automated Script

Use this script to generate scenarios with road networks:

```bash
#!/bin/bash
# generate_with_roads.sh

EXAMPLE=$1
ROAD_FILE="roads/simple_highway.xodr"

# Generate the scenario
cd openscenario
cargo run --example "$EXAMPLE"

# Get the output file name
XOSC_FILE="${EXAMPLE}.xosc"

# Update the RoadNetwork element
sed -i 's|<RoadNetwork/>|<RoadNetwork>\n    <LogicFile filepath="'$ROAD_FILE'"/>\n  </RoadNetwork>|' "../$XOSC_FILE"

echo "✅ Generated $XOSC_FILE with road network reference"
```

**Usage:**
```bash
chmod +x generate_with_roads.sh
./generate_with_roads.sh highway_merge
./generate_with_roads.sh emergency_braking
```

---

## Road Network Files

### Simple Highway (`roads/simple_highway.xodr`)

**Geometry:**
- **Length**: 500 meters straight road
- **Lanes**: 3 total
  - Lane 1 (left): Overtaking lane, 3.5m wide
  - Lane -1 (right): Normal traffic, 3.5m wide  
  - Lane -2 (far right): Slow traffic, 3.5m wide
- **Speed limit**: 130 km/h
- **Markings**: Standard white lines (broken between lanes, solid on edges)

**Coordinate system:**
- Origin at (0, 0)
- Road extends along +X axis
- Lane -1 is the default right lane

**Suitable for:**
- Highway merge scenarios
- Overtaking maneuvers
- Emergency braking
- Platooning
- Adaptive cruise control

---

## Position Mapping

### World Positions → Lane Positions

Our examples use world coordinates. Here's how they map to lanes:

**Example positions:**
```
y = 0.0    → Lane -1 (right lane, default)
y = 3.5    → Lane 1 (left lane, overtaking)
y = -3.5   → Lane -2 (far right)
```

**Lane coordinate system:**
- `s` = distance along road (0 to 500m)
- `t` = lateral offset from reference line
- Lane IDs: positive = left of center, negative = right

---

## Using esmini

### Basic Usage

```bash
# Run scenario
esmini --osc scenario.xosc

# With specific window size
esmini --osc scenario.xosc --window 0 0 1920 1080

# Headless (no viewer)
esmini --osc scenario.xosc --headless
```

### Recording

```bash
# Record simulation data
esmini --osc scenario.xosc --record output.dat

# Replay recorded data
replayer --file output.dat --window 0 0 1920 1080
```

### Screenshot Capture

```bash
# Continuous screenshots (creates many JPEGs)
esmini --osc scenario.xosc --capture_screen

# Single screenshot (press 'c' key during playback)
esmini --osc scenario.xosc --osg_screenshot_event_handler
```

### Useful Options

```bash
# Show on-screen info
esmini --osc scenario.xosc --info_text 3

# Fixed timestep (faster playback)
esmini --osc scenario.xosc --fixed_timestep 0.02

# Disable trails
esmini --osc scenario.xosc --trail_mode 0

# Custom camera position
esmini --osc scenario.xosc --camera_mode top
```

---

## Troubleshooting

### Error: "Failed to load road network"

**Cause**: Road network file not found or path incorrect

**Solution**:
1. Check the filepath in `<LogicFile filepath="..."/>`
2. Ensure the .xodr file exists at that path
3. Use absolute path if needed: `filepath="/absolute/path/to/road.xodr"`

### Error: "Entity position outside road"

**Cause**: Initial position not on the road network

**Solution**:
- For world positions: Ensure y-coordinates align with lane centers (0, ±3.5, ±7.0)
- For lane positions: Use valid lane IDs (-2, -1, 1, 2)
- Check s-coordinate is within road length (0 to 500)

### Vehicles not visible

**Cause**: Z-coordinate underground or very far

**Solution**:
- Ensure z = 0.0 in world positions
- Let esmini auto-calculate z from road elevation

### Actions don't execute

**Cause**: Missing story hierarchy or incorrect event linking

**Solution**:
1. Verify story → act → maneuvergroup → maneuver → event chain
2. Check entity is added as actor: `add_actor(...)`
3. Ensure event name matches in condition and action

---

## Advanced: Creating Custom Roads

For more complex scenarios, create custom .xodr files:

### Using ASAM OpenDRIVE Editor

1. Download OpenDRIVE editor
2. Design road network visually
3. Export as .xodr
4. Reference in OpenSCENARIO

### Using Code Generation

```python
# Example: Generate simple road with Python
from lxml import etree

root = etree.Element("OpenDRIVE")
header = etree.SubElement(root, "header", revMajor="1", revMinor="7")
# ... add road geometry ...

tree = etree.ElementTree(root)
tree.write("custom_road.xodr", pretty_print=True, xml_declaration=True, encoding="UTF-8")
```

### Using odrplot

View and validate .xodr files:

```bash
odrplot --input roads/simple_highway.xodr --output highway_plot.png
```

---

## Integration with Rust Library

**Future feature**: Add road network reference during scenario creation

```rust
// Planned API (not yet implemented)
let mut scenario = Scenario::new(OpenScenarioVersion::V1_2);
scenario.set_road_network("roads/simple_highway.xodr")?;
```

For now, use the sed script or manual editing.

---

## Examples with Roads

### Highway Merge

```bash
# Generate
cargo run --example highway_merge

# Add road
sed -i 's|<RoadNetwork/>|<RoadNetwork><LogicFile filepath="roads/simple_highway.xodr"/></RoadNetwork>|' ../highway_merge.xosc

# Run
esmini --osc ../highway_merge.xosc
```

### Emergency Braking

```bash
cargo run --example emergency_braking
sed -i 's|<RoadNetwork/>|<RoadNetwork><LogicFile filepath="roads/simple_highway.xodr"/></RoadNetwork>|' ../emergency_braking.xosc
esmini --osc ../emergency_braking.xosc
```

---

## Next Steps

1. Generate all examples with the script
2. Run in esmini to visualize behaviors
3. Experiment with different road networks
4. Create custom scenarios with specific road geometries
5. Record and analyze results

---

## Resources

- **esmini docs**: https://esmini.github.io/
- **OpenDRIVE spec**: https://www.asam.net/standards/detail/opendrive/
- **Road network examples**: esmini comes with sample .xodr files in `resources/xodr/`
