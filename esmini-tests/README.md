# esmini Testing Pipeline

Automated testing pipeline for OpenSCENARIO validation using esmini simulator.

## Overview

This pipeline generates test scenarios using the OpenSCENARIO MCP server, runs them in esmini, and validates the results.

## Components

### 1. Scenario Generator (`generate_scenarios.py`)

Python script that uses the MCP client to generate 5 test scenarios:

1. **Speed Change** (`01_speed_change.xosc`)
   - Vehicle accelerates from 0 to 30 m/s over 5 seconds
   - Tests basic speed action functionality

2. **Lane Change** (`02_lane_change.xosc`)
   - Vehicle changes from lane 1 (y=0) to lane 2 (y=3.5m)
   - Maintains constant speed of 20 m/s
   - Tests lateral maneuver actions

3. **Multi-Action** (`03_multi_action.xosc`)
   - Combined speed change (0→25 m/s) and lane change (y=0→3.5m)
   - Tests simultaneous longitudinal and lateral actions

4. **Overtake** (`04_overtake.xosc`)
   - Ego vehicle overtakes slower adversary
   - Ego accelerates to 30 m/s and changes lanes
   - Adversary maintains 20 m/s
   - Tests multi-vehicle scenarios

5. **Deceleration** (`05_deceleration.xosc`)
   - Vehicle accelerates to 30 m/s, then decelerates to 10 m/s
   - Tests braking/deceleration actions

**Requirements:**
```bash
pip install mcp
```

### 2. esmini Runner (`run_esmini.sh`)

Bash script that:
- Checks if esmini is installed
- Runs each scenario with recording enabled
- Converts `.dat` output to CSV (if `dat2csv` available)
- Stores results in `results/` directory

**esmini options used:**
- `--osc`: OpenSCENARIO file path
- `--record`: Record simulation data to .dat file
- `--headless`: Run without visualization
- `--fixed_timestep 0.01`: 100Hz simulation rate

**Graceful fallback:** If esmini is not installed, the script exits cleanly with a warning instead of failing.

### 3. Results Validator (`validate_results.py`)

Python script that:
- Parses esmini output (`.dat` binary or `.csv` format)
- Validates scenario-specific behaviors:
  - **Speed changes**: Final speed within expected range (20-35 m/s)
  - **Lane changes**: Lateral displacement ~3.5m (±1m tolerance)
  - **Trajectory smoothness**: No sudden jumps/teleportation
  - **Data integrity**: No NaN or inf values
- Generates JSON validation report

**Output:** `results/validation_report.json`

### 4. Main Pipeline (`run_pipeline.sh`)

Master script that orchestrates the complete workflow:
```bash
./run_pipeline.sh
```

**Workflow:**
1. Generate scenarios using MCP
2. Run scenarios in esmini (if available)
3. Validate results
4. Generate report

## Installation

### Prerequisites

1. **Rust & Cargo** (for MCP server)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Python 3.7+** with pip

3. **MCP Python Client**
   ```bash
   pip install mcp
   ```

4. **esmini** (optional, for simulation)
   - Download: https://github.com/esmini/esmini/releases
   - Extract and add `bin/` to PATH:
     ```bash
     export PATH=/path/to/esmini/bin:$PATH
     ```

### Setup

Clone and navigate to the testing directory:
```bash
cd ~/.openclaw/workspace/osc-mcp/esmini-tests
```

## Usage

### Quick Start

Run the complete pipeline:
```bash
./run_pipeline.sh
```

### Step-by-Step

Or run each component individually:

1. **Generate scenarios:**
   ```bash
   python3 generate_scenarios.py
   ```

2. **Run simulations:**
   ```bash
   ./run_esmini.sh
   ```

3. **Validate results:**
   ```bash
   python3 validate_results.py
   ```

### Output Files

```
esmini-tests/
├── scenarios/           # Generated .xosc files
│   ├── 01_speed_change.xosc
│   ├── 02_lane_change.xosc
│   ├── 03_multi_action.xosc
│   ├── 04_overtake.xosc
│   └── 05_deceleration.xosc
│
└── results/            # Simulation outputs
    ├── 01_speed_change.dat         # esmini binary data
    ├── 01_speed_change.csv         # Converted CSV (optional)
    ├── 01_speed_change.log         # Simulation log
    ├── ... (other scenarios)
    └── validation_report.json      # Validation summary
```

## Validation Report

Example `validation_report.json`:
```json
{
  "summary": {
    "total": 5,
    "passed": 5,
    "failed": 0
  },
  "results": [
    {
      "scenario": "01_speed_change",
      "status": "PASSED",
      "duration": 10.0,
      "num_records": 1000,
      "issues": [],
      "warnings": ["Speed target reached: 30.1 m/s"],
      "final_state": {
        "time": 10.0,
        "x": 150.5,
        "y": 0.0,
        "speed": 30.1
      }
    }
  ]
}
```

## Without esmini

If esmini is not installed, the pipeline will:
1. ✅ Generate scenarios successfully
2. ⚠️  Skip simulation step with warning
3. ⚠️  Skip validation (no data to validate)

Scenarios are still valid OpenSCENARIO 1.2 files and can be:
- Opened in other simulators (CARLA, SUMO, etc.)
- Validated with XML schema tools
- Inspected manually

## Troubleshooting

### esmini not found
```
⚠️  WARNING: esmini not found in PATH
```
**Solution:** Install esmini and add to PATH (see Installation section)

### MCP connection failed
```
❌ Error: Connection refused
```
**Solution:** Ensure you're in the correct directory:
```bash
cd ~/.openclaw/workspace/osc-mcp/esmini-tests
```

### Validation fails
Check the specific issues in the validation report:
```bash
cat results/validation_report.json
```

Common issues:
- **Speed mismatch**: Check MCP server speed action implementation
- **Lane change incomplete**: Check target_lane parameter
- **Trajectory jumps**: May indicate esmini timestep issues

## Extending the Pipeline

### Adding New Scenarios

Edit `generate_scenarios.py` and add a new async function:

```python
async def create_my_scenario(session):
    """Your scenario description."""
    # Create scenario
    result = await session.call_tool(
        "create_scenario",
        arguments={"name": "my_test", "version": "1.2"}
    )
    scenario_id = result.content[0].text.split(": ")[1]
    
    # Add entities and actions...
    
    # Export
    output_path = str(SCENARIOS_DIR / "06_my_scenario.xosc")
    await session.call_tool(
        "export_xml",
        arguments={"scenario_id": scenario_id, "output_path": output_path}
    )
    return output_path
```

Then call it in `main()`:
```python
scenarios.append(await create_my_scenario(session))
```

### Adding Custom Validations

Edit `validate_results.py` and add methods to `ScenarioValidator`:

```python
def validate_my_behavior(self):
    """Validate custom behavior."""
    # Your validation logic
    if some_condition_failed:
        self.issues.append("Description of issue")
```

Then call it in `validate_all()`:
```python
if 'my_scenario' in self.scenario_name:
    self.validate_my_behavior()
```

## CI/CD Integration

This pipeline can be integrated into CI/CD:

```yaml
# Example GitHub Actions workflow
name: esmini Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install dependencies
        run: |
          pip install mcp
          # Install esmini...
      - name: Run pipeline
        run: cd esmini-tests && ./run_pipeline.sh
      - name: Upload results
        uses: actions/upload-artifact@v2
        with:
          name: test-results
          path: esmini-tests/results/
```

## References

- **esmini**: https://github.com/esmini/esmini
- **OpenSCENARIO**: https://www.asam.net/standards/detail/openscenario/
- **MCP Protocol**: https://modelcontextprotocol.io/

## License

Same as parent project.
