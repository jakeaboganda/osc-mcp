#!/bin/bash
set -e

# esmini Runner Script
# Runs OpenSCENARIO files in esmini and captures output

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCENARIOS_DIR="${SCRIPT_DIR}/scenarios"
RESULTS_DIR="${SCRIPT_DIR}/results"

echo "============================================================"
echo "🚗 esmini Scenario Runner"
echo "============================================================"
echo ""

# Check if esmini is installed
if ! command -v esmini &> /dev/null; then
    echo "⚠️  WARNING: esmini not found in PATH"
    echo ""
    echo "esmini is required to run simulations."
    echo "Installation options:"
    echo "  1. Download from: https://github.com/esmini/esmini/releases"
    echo "  2. Build from source: https://github.com/esmini/esmini"
    echo ""
    echo "❌ Skipping simulation step (validation will use MCP data only)"
    exit 0
fi

echo "✅ esmini found: $(which esmini)"
echo ""

# Create results directory
mkdir -p "${RESULTS_DIR}"

# Check if scenarios exist
if [ ! -d "${SCENARIOS_DIR}" ] || [ -z "$(ls -A ${SCENARIOS_DIR}/*.xosc 2>/dev/null)" ]; then
    echo "❌ No .xosc scenarios found in ${SCENARIOS_DIR}"
    echo "   Run generate_scenarios.py first!"
    exit 1
fi

# Count scenarios
SCENARIO_COUNT=$(ls -1 ${SCENARIOS_DIR}/*.xosc 2>/dev/null | wc -l)
echo "📁 Found ${SCENARIO_COUNT} scenario(s) to run"
echo ""

# Run each scenario
SUCCESS_COUNT=0
FAILED_COUNT=0

for scenario in ${SCENARIOS_DIR}/*.xosc; do
    scenario_name=$(basename "$scenario" .xosc)
    echo "▶️  Running: ${scenario_name}"
    
    output_dat="${RESULTS_DIR}/${scenario_name}.dat"
    output_csv="${RESULTS_DIR}/${scenario_name}.csv"
    output_log="${RESULTS_DIR}/${scenario_name}.log"
    
    # Run esmini with recording
    # Options:
    #   --osc: OpenSCENARIO file
    #   --record: Record simulation data
    #   --headless: No visualization window
    #   --fixed_timestep 0.01: 100Hz simulation
    if esmini --osc "$scenario" \
              --record "$output_dat" \
              --headless \
              --fixed_timestep 0.01 \
              > "$output_log" 2>&1; then
        echo "   ✅ Simulation completed"
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        
        # Convert .dat to .csv if dat2csv is available
        if command -v dat2csv &> /dev/null; then
            echo "   📊 Converting to CSV..."
            if dat2csv "$output_dat" > /dev/null 2>&1; then
                echo "   ✅ CSV exported: ${scenario_name}.csv"
            else
                echo "   ⚠️  CSV conversion failed (will parse .dat directly)"
            fi
        else
            echo "   ℹ️  dat2csv not found (will parse .dat directly)"
        fi
    else
        echo "   ❌ Simulation failed (see ${scenario_name}.log)"
        FAILED_COUNT=$((FAILED_COUNT + 1))
    fi
    
    echo ""
done

echo "============================================================"
echo "📊 Summary"
echo "============================================================"
echo "Total scenarios: ${SCENARIO_COUNT}"
echo "✅ Successful:   ${SUCCESS_COUNT}"
echo "❌ Failed:       ${FAILED_COUNT}"
echo ""

if [ ${FAILED_COUNT} -gt 0 ]; then
    echo "⚠️  Some scenarios failed. Check logs in ${RESULTS_DIR}/"
    exit 1
fi

echo "✅ All scenarios completed successfully!"
echo "📂 Results saved to: ${RESULTS_DIR}/"
