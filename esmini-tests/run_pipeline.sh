#!/bin/bash
set -e

# esmini Testing Pipeline
# Complete automated testing workflow

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "======================================================================"
echo "🧪 OpenSCENARIO esmini Testing Pipeline"
echo "======================================================================"
echo ""
echo "This pipeline will:"
echo "  1. Generate test scenarios using MCP"
echo "  2. Run scenarios in esmini (if available)"
echo "  3. Validate results"
echo ""

# Step 1: Generate scenarios
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 1: Generate Scenarios"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if ! python3 "${SCRIPT_DIR}/generate_scenarios.py"; then
    echo ""
    echo "❌ Scenario generation failed!"
    exit 1
fi

echo ""

# Step 2: Run esmini
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 2: Run esmini Simulations"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if "${SCRIPT_DIR}/run_esmini.sh"; then
    ESMINI_SUCCESS=true
else
    ESMINI_SUCCESS=false
    echo "⚠️  esmini step failed or skipped"
fi

echo ""

# Step 3: Validate results
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 3: Validate Results"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$ESMINI_SUCCESS" = true ]; then
    if ! python3 "${SCRIPT_DIR}/validate_results.py"; then
        echo ""
        echo "❌ Validation failed!"
        exit 1
    fi
else
    echo "⚠️  Skipping validation (esmini not run)"
    echo ""
    echo "Scenarios were generated successfully but not simulated."
    echo "To complete the pipeline:"
    echo "  1. Install esmini: https://github.com/esmini/esmini/releases"
    echo "  2. Add esmini to PATH"
    echo "  3. Re-run this pipeline"
fi

echo ""
echo "======================================================================"
echo "✅ Pipeline Complete!"
echo "======================================================================"
echo ""
echo "Results:"
echo "  📁 Scenarios: ${SCRIPT_DIR}/scenarios/"
echo "  📁 Results:   ${SCRIPT_DIR}/results/"
echo ""
