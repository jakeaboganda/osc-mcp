# openscenario

Rust library for generating and validating OpenSCENARIO test scenarios.

## Features

- Multi-version support (1.0, 1.1, 1.2)
- Complete entity management (Vehicle, Pedestrian, MiscObject)
- All 7 position types (World, Lane, Road, and relative variants)
- Storyboard hierarchy (Story → Act → ManeuverGroup → Maneuver → Event → Action)
- Actions: Speed, LaneChange
- XML export to valid OpenSCENARIO format
- Fail-fast validation with detailed error messages

## Usage

See the main [README](../README.md) and [examples](examples/) for usage.

## Tests

Run `cargo test` in this directory to run the test suite (32 tests).
