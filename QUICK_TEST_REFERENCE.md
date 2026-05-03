# Quick Test Reference

## Running Tests

### Run All Tests
```bash
cd ~/.openclaw/workspace/osc-mcp
cargo test
```

### Run Specific Test Categories

#### Catalog Tests
```bash
cargo test --test catalog_tests
cargo test --test catalog_validation_e2e
```

#### Validation Tests  
```bash
cargo test --test validation_tests
```

#### OpenDRIVE Tests
```bash
cargo test --test opendrive_tests
```

#### Integration Tests
```bash
cargo test --test integration_test
```

### Run Custom Test Script
```bash
cd ~/.openclaw/workspace/osc-mcp
./test_catalog_validation.sh
```

## Test Files

| File | Purpose |
|------|---------|
| `openscenario/tests/catalog_tests.rs` | Catalog loading tests |
| `openscenario/tests/validation_tests.rs` | XSD validation tests |
| `openscenario/tests/opendrive_tests.rs` | OpenDRIVE integration tests |
| `openscenario/tests/catalog_validation_e2e.rs` | End-to-end tests (NEW) |
| `test_catalog_validation.sh` | Automated test runner (NEW) |

## Test Reports

- **`TEST_SUMMARY.md`** - Executive summary
- **`TEST_REPORT_CATALOG_VALIDATION.md`** - Detailed report with examples
- **`QUICK_TEST_REFERENCE.md`** - This file

## Quick Checks

### Verify Catalog Loading
```bash
cargo test test_load_vehicle_catalog
cargo test test_load_pedestrian_catalog
```

### Verify Validation
```bash
cargo test test_validate_v1_0_scenario
cargo test test_version_mismatch
```

### Verify OpenDRIVE
```bash
cargo test test_validate_lane_position
cargo test test_validate_road_position
```

## Expected Output

All tests should show:
```
test result: ok. X passed; 0 failed; 0 ignored
```

Total: **65+ tests passing, 0 failures**

## Troubleshooting

If tests fail:
1. Ensure you're in the project root: `~/.openclaw/workspace/osc-mcp`
2. Build first: `cargo build`
3. Check test data files exist in `openscenario/tests/`
4. Run with verbose output: `cargo test -- --nocapture`

## Success Criteria

All these should pass:
- ✅ Catalogs load correctly
- ✅ Validation catches errors
- ✅ OpenDRIVE validation works
- ✅ Error messages are clear
