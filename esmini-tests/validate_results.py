#!/usr/bin/env python3
"""
esmini Results Validator

Parses esmini output files (.dat or .csv) and validates scenario behaviors.

Validations:
1. Speed changes reach target speeds
2. Lane changes complete successfully
3. Trajectories are smooth (no teleporting/jumps)
4. No NaN or invalid values
"""

import struct
import csv
import json
from pathlib import Path
from typing import List, Dict, Any, Optional
import math


class EsminiDatParser:
    """Parser for esmini .dat binary format."""
    
    # .dat format (per timestep, per object):
    # float: time, x, y, z, h, p, r, speed, wheel_angle, wheel_rotation
    RECORD_SIZE = 10 * 4  # 10 floats * 4 bytes
    
    @staticmethod
    def parse(dat_file: Path) -> List[Dict[str, Any]]:
        """Parse .dat file into list of records."""
        records = []
        
        with open(dat_file, 'rb') as f:
            data = f.read()
        
        num_records = len(data) // EsminiDatParser.RECORD_SIZE
        
        for i in range(num_records):
            offset = i * EsminiDatParser.RECORD_SIZE
            chunk = data[offset:offset + EsminiDatParser.RECORD_SIZE]
            
            if len(chunk) < EsminiDatParser.RECORD_SIZE:
                break
            
            values = struct.unpack('10f', chunk)
            
            records.append({
                'time': values[0],
                'x': values[1],
                'y': values[2],
                'z': values[3],
                'h': values[4],  # heading
                'p': values[5],  # pitch
                'r': values[6],  # roll
                'speed': values[7],
                'wheel_angle': values[8],
                'wheel_rotation': values[9]
            })
        
        return records


class EsminiCsvParser:
    """Parser for esmini CSV format."""
    
    @staticmethod
    def parse(csv_file: Path) -> List[Dict[str, Any]]:
        """Parse CSV file into list of records."""
        records = []
        
        with open(csv_file, 'r') as f:
            reader = csv.DictReader(f)
            for row in reader:
                # Convert numeric fields
                records.append({
                    'time': float(row.get('time', 0)),
                    'x': float(row.get('x', 0)),
                    'y': float(row.get('y', 0)),
                    'z': float(row.get('z', 0)),
                    'h': float(row.get('h', 0)),
                    'p': float(row.get('p', 0)),
                    'r': float(row.get('r', 0)),
                    'speed': float(row.get('speed', 0)),
                })
        
        return records


class ScenarioValidator:
    """Validates scenario execution results."""
    
    def __init__(self, scenario_name: str, records: List[Dict[str, Any]]):
        self.scenario_name = scenario_name
        self.records = records
        self.issues = []
        self.warnings = []
    
    def validate_all(self) -> Dict[str, Any]:
        """Run all validations."""
        if not self.records:
            return {
                'scenario': self.scenario_name,
                'status': 'FAILED',
                'reason': 'No data records found',
                'issues': [],
                'warnings': []
            }
        
        self.validate_data_integrity()
        self.validate_smooth_trajectory()
        
        # Scenario-specific validations
        if 'speed_change' in self.scenario_name:
            self.validate_speed_change()
        elif 'lane_change' in self.scenario_name:
            self.validate_lane_change()
        elif 'multi_action' in self.scenario_name:
            self.validate_speed_change()
            self.validate_lane_change()
        elif 'overtake' in self.scenario_name:
            self.validate_overtake()
        elif 'deceleration' in self.scenario_name:
            self.validate_deceleration()
        
        status = 'PASSED' if not self.issues else 'FAILED'
        
        return {
            'scenario': self.scenario_name,
            'status': status,
            'duration': self.records[-1]['time'] if self.records else 0,
            'num_records': len(self.records),
            'issues': self.issues,
            'warnings': self.warnings,
            'final_state': self.records[-1] if self.records else {}
        }
    
    def validate_data_integrity(self):
        """Check for NaN, inf, or invalid values."""
        for i, rec in enumerate(self.records):
            for key in ['x', 'y', 'z', 'speed']:
                val = rec.get(key, 0)
                if math.isnan(val) or math.isinf(val):
                    self.issues.append(f"Invalid {key} at t={rec['time']:.2f}s: {val}")
    
    def validate_smooth_trajectory(self):
        """Check for sudden jumps (teleportation)."""
        MAX_SPEED = 50.0  # m/s (~180 km/h)
        
        for i in range(1, len(self.records)):
            prev = self.records[i-1]
            curr = self.records[i]
            dt = curr['time'] - prev['time']
            
            if dt <= 0:
                continue
            
            dx = curr['x'] - prev['x']
            dy = curr['y'] - prev['y']
            distance = math.sqrt(dx*dx + dy*dy)
            instant_speed = distance / dt
            
            if instant_speed > MAX_SPEED * 2:  # 2x tolerance
                self.issues.append(
                    f"Trajectory jump at t={curr['time']:.2f}s: "
                    f"instant speed {instant_speed:.1f} m/s (distance={distance:.1f}m, dt={dt:.3f}s)"
                )
    
    def validate_speed_change(self):
        """Validate speed change scenarios reach target speed."""
        if not self.records:
            return
        
        # Expected: accelerate to ~25-30 m/s
        final_speed = self.records[-1]['speed']
        
        if final_speed < 20.0:
            self.issues.append(f"Final speed too low: {final_speed:.1f} m/s (expected ~25-30 m/s)")
        elif final_speed > 35.0:
            self.issues.append(f"Final speed too high: {final_speed:.1f} m/s (expected ~25-30 m/s)")
        else:
            self.warnings.append(f"Speed target reached: {final_speed:.1f} m/s")
    
    def validate_lane_change(self):
        """Validate lane change scenarios complete successfully."""
        if not self.records:
            return
        
        initial_y = self.records[0]['y']
        final_y = self.records[-1]['y']
        delta_y = abs(final_y - initial_y)
        
        # Expected: move ~3.5m laterally (one lane)
        if delta_y < 2.5:
            self.issues.append(f"Lane change incomplete: Δy={delta_y:.2f}m (expected ~3.5m)")
        elif delta_y > 5.0:
            self.issues.append(f"Lane change excessive: Δy={delta_y:.2f}m (expected ~3.5m)")
        else:
            self.warnings.append(f"Lane change completed: Δy={delta_y:.2f}m")
    
    def validate_overtake(self):
        """Validate overtake scenario."""
        # Just check speed and lane change for ego vehicle
        self.validate_speed_change()
        self.validate_lane_change()
    
    def validate_deceleration(self):
        """Validate deceleration scenario."""
        if not self.records:
            return
        
        # Check if vehicle decelerates
        max_speed = max(rec['speed'] for rec in self.records)
        final_speed = self.records[-1]['speed']
        
        if final_speed >= max_speed * 0.9:
            self.issues.append(
                f"No deceleration detected: max={max_speed:.1f}, final={final_speed:.1f} m/s"
            )
        else:
            self.warnings.append(
                f"Deceleration successful: {max_speed:.1f} → {final_speed:.1f} m/s"
            )


def load_results(results_dir: Path) -> Dict[str, List[Dict[str, Any]]]:
    """Load all result files from directory."""
    results = {}
    
    for dat_file in results_dir.glob("*.dat"):
        scenario_name = dat_file.stem
        try:
            records = EsminiDatParser.parse(dat_file)
            results[scenario_name] = records
        except Exception as e:
            print(f"   ⚠️  Failed to parse {dat_file.name}: {e}")
    
    # Try CSV if .dat parsing failed or no .dat files
    for csv_file in results_dir.glob("*.csv"):
        scenario_name = csv_file.stem
        if scenario_name not in results:
            try:
                records = EsminiCsvParser.parse(csv_file)
                results[scenario_name] = records
            except Exception as e:
                print(f"   ⚠️  Failed to parse {csv_file.name}: {e}")
    
    return results


def main():
    """Main validation entry point."""
    
    print("=" * 70)
    print("🔍 esmini Results Validator")
    print("=" * 70)
    print()
    
    # Locate results directory
    script_dir = Path(__file__).parent
    results_dir = script_dir / "results"
    
    if not results_dir.exists():
        print(f"❌ Results directory not found: {results_dir}")
        print("   Run run_esmini.sh first!")
        return 1
    
    # Load all results
    print("📂 Loading results...")
    results = load_results(results_dir)
    
    if not results:
        print("❌ No result files found in results/")
        print("   esmini may not have been run, or parsing failed")
        return 1
    
    print(f"✅ Loaded {len(results)} scenario result(s)\n")
    
    # Validate each scenario
    all_results = []
    passed = 0
    failed = 0
    
    for scenario_name, records in results.items():
        print(f"🔍 Validating: {scenario_name}")
        validator = ScenarioValidator(scenario_name, records)
        result = validator.validate_all()
        all_results.append(result)
        
        if result['status'] == 'PASSED':
            print(f"   ✅ PASSED ({result['num_records']} records, {result['duration']:.1f}s)")
            passed += 1
        else:
            print(f"   ❌ FAILED")
            failed += 1
        
        # Show issues
        for issue in result['issues']:
            print(f"      ❌ {issue}")
        
        # Show warnings (non-fatal)
        for warning in result['warnings']:
            print(f"      ℹ️  {warning}")
        
        print()
    
    # Write JSON report
    report_file = results_dir / "validation_report.json"
    with open(report_file, 'w') as f:
        json.dump({
            'summary': {
                'total': len(all_results),
                'passed': passed,
                'failed': failed
            },
            'results': all_results
        }, f, indent=2)
    
    print("=" * 70)
    print("📊 Validation Summary")
    print("=" * 70)
    print(f"Total scenarios: {len(all_results)}")
    print(f"✅ Passed:       {passed}")
    print(f"❌ Failed:       {failed}")
    print()
    print(f"📄 Full report: {report_file}")
    print()
    
    if failed > 0:
        print("⚠️  Some validations failed. Review issues above.")
        return 1
    
    print("✅ All validations passed!")
    return 0


if __name__ == "__main__":
    exit(main())
