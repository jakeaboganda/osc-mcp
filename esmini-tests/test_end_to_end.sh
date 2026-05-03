#!/bin/bash
set -e

echo "🚀 End-to-End OpenSCENARIO Pipeline Test"
echo "=========================================="

# Generate scenario
echo "1️⃣  Generating scenario with MCP server..."
python3 test_actor_debug.py 2>&1 | grep "✅"

# Add road network
echo "2️⃣  Adding road network reference..."
sed -i 's|<RoadNetwork/>|<RoadNetwork>\n    <LogicFile filepath="straight_road.xodr"/>\n  </RoadNetwork>|' scenarios/actor_debug.xosc

# Validate XML structure
echo "3️⃣  Validating XML structure..."
grep -q '<Vehicle name="ego" vehicleCategory="car">' scenarios/actor_debug.xosc && echo "  ✓ Inline Vehicle definition"
grep -q '<EntityRef entityRef="ego"/>' scenarios/actor_debug.xosc && echo "  ✓ Actor assignment"
grep -q '<SimulationTimeCondition value="10" rule="greaterThan"/>' scenarios/actor_debug.xosc && echo "  ✓ StopTrigger"
grep -q 'entityRef="ego"' scenarios/actor_debug.xosc | grep -q '<Private' && echo "  ✓ Init section"
grep -q '<SpeedActionDynamics dynamicsShape=' scenarios/actor_debug.xosc && echo "  ✓ SpeedAction dynamics"

# Run with esmini
echo "4️⃣  Running scenario in esmini..."
cd scenarios
timeout 15 esmini --headless --fixed_timestep 0.05 --osc actor_debug.xosc 2>&1 | tee /tmp/esmini_out.log | grep -q "Closing"

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ END-TO-END TEST PASSED!"
    echo ""
    echo "Summary:"
    grep "Loaded actor_debug.xosc" /tmp/esmini_out.log
    grep "Loaded OpenDRIVE" /tmp/esmini_out.log
    grep "Closing" /tmp/esmini_out.log
    exit 0
else
    echo "❌ esmini failed to complete"
    tail -20 /tmp/esmini_out.log
    exit 1
fi
