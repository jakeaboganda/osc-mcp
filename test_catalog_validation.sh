#!/bin/bash
# Comprehensive test suite for catalog loading and validation features

set -e

echo "================================"
echo "CATALOG & VALIDATION TEST SUITE"
echo "================================"
echo ""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASSED=0
FAILED=0
TOTAL=0

test_result() {
    TOTAL=$((TOTAL + 1))
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ PASS${NC}: $2"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗ FAIL${NC}: $2"
        FAILED=$((FAILED + 1))
    fi
}

cd ~/.openclaw/workspace/osc-mcp

# Build the project first
echo "Building project..."
cargo build --release 2>&1 | tail -5
echo ""

echo "Running test suite..."
echo ""

# ========================================
# 1. CATALOG LOADING TESTS
# ========================================
echo -e "${YELLOW}[1] Catalog Loading Tests${NC}"
echo "----------------------------"

# Test: Load vehicle catalog
cargo test test_load_vehicle_catalog --quiet 2>&1
test_result $? "Load vehicle catalog"

# Test: Load pedestrian catalog
cargo test test_load_pedestrian_catalog --quiet 2>&1
test_result $? "Load pedestrian catalog"

# Test: Find catalog entry
cargo test test_find_entry --quiet 2>&1
test_result $? "Find entry by name in catalog"

# Test: Clone entity from catalog
cargo test test_clone_entity --quiet 2>&1
test_result $? "Clone entity from catalog"

echo ""

# ========================================
# 2. CATALOG REFERENCES TESTS
# ========================================
echo -e "${YELLOW}[2] Catalog Reference Tests${NC}"
echo "----------------------------"

# Test: Entity tests include catalog reference functionality
cargo test entity_tests --quiet 2>&1
test_result $? "Entity catalog references"

echo ""

# ========================================
# 3. INVALID CATALOG TESTS
# ========================================
echo -e "${YELLOW}[3] Invalid Catalog Tests${NC}"
echo "----------------------------"

# Test: Invalid catalog type
cargo test test_invalid_catalog --quiet 2>&1
test_result $? "Reject invalid catalog type"

# Test: Malformed XML in catalog
cargo test test_malformed_xml --quiet 2>&1
test_result $? "Reject malformed XML catalog"

echo ""

# ========================================
# 4. VALIDATION TESTS
# ========================================
echo -e "${YELLOW}[4] Scenario Validation Tests${NC}"
echo "-------------------------------"

# Test: Valid scenario validation
cargo test test_validate_v1_0_scenario --quiet 2>&1
test_result $? "Validate well-formed scenario"

# Test: Invalid XML validation
cargo test test_invalid_xml --quiet 2>&1
test_result $? "Reject malformed scenario XML"

# Test: Version mismatch detection
cargo test test_version_mismatch --quiet 2>&1
test_result $? "Detect version mismatch"

echo ""

# ========================================
# 5. OPENDRIVE VALIDATION TESTS
# ========================================
echo -e "${YELLOW}[5] OpenDRIVE Validation Tests${NC}"
echo "--------------------------------"

# Test: Load OpenDRIVE file
cargo test test_load_opendrive --quiet 2>&1
test_result $? "Load OpenDRIVE file"

# Test: Validate lane positions
cargo test test_validate_lane_position --quiet 2>&1
test_result $? "Validate lane positions with OpenDRIVE"

# Test: Validate road positions
cargo test test_validate_road_position --quiet 2>&1
test_result $? "Validate road positions with OpenDRIVE"

# Test: Center lane validation
cargo test test_center_lane --quiet 2>&1
test_result $? "Validate center lane (lane_id=0)"

# Test: Special float handling
cargo test test_special_floats --quiet 2>&1
test_result $? "Reject NaN/Infinity positions"

# Test: Empty sections handling
cargo test test_empty_sections --quiet 2>&1
test_result $? "Handle edge cases gracefully"

echo ""

# ========================================
# 6. INTEGRATION TESTS
# ========================================
echo -e "${YELLOW}[6] Integration Tests${NC}"
echo "----------------------"

# Run full integration test
cargo test integration_test --quiet 2>&1
test_result $? "Full integration workflow"

echo ""

# ========================================
# SUMMARY
# ========================================
echo "================================"
echo "TEST SUMMARY"
echo "================================"
echo -e "Total Tests:  ${TOTAL}"
echo -e "Passed:       ${GREEN}${PASSED}${NC}"
echo -e "Failed:       ${RED}${FAILED}${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}✗ Some tests failed${NC}"
    exit 1
fi
