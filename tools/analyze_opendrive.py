#!/usr/bin/env python3
"""
OpenDRIVE Road Network Analyzer

Parses .xodr files and generates human-readable descriptions of road geometry.
Helps understand what the road network looks like without 3D visualization.
"""

import xml.etree.ElementTree as ET
import sys
from typing import List, Tuple, Dict

def parse_xodr(filepath: str) -> ET.Element:
    """Parse OpenDRIVE XML file."""
    tree = ET.parse(filepath)
    return tree.getroot()

def describe_road(road: ET.Element) -> Dict:
    """Extract road information."""
    road_id = road.get('id')
    name = road.get('name', 'Unnamed')
    length = float(road.get('length', 0))
    
    # Get geometry
    planview = road.find('planView')
    geometries = []
    if planview is not None:
        for geom in planview.findall('geometry'):
            s = float(geom.get('s'))
            x = float(geom.get('x'))
            y = float(geom.get('y'))
            hdg = float(geom.get('hdg'))
            geom_length = float(geom.get('length'))
            
            geom_type = None
            if geom.find('line') is not None:
                geom_type = 'straight'
            elif geom.find('arc') is not None:
                geom_type = 'curved'
            elif geom.find('spiral') is not None:
                geom_type = 'spiral'
            
            geometries.append({
                's': s,
                'x': x,
                'y': y,
                'heading': hdg,
                'length': geom_length,
                'type': geom_type
            })
    
    # Get lanes
    lanes_elem = road.find('lanes')
    lanes = {'left': [], 'center': [], 'right': []}
    
    if lanes_elem is not None:
        for lane_section in lanes_elem.findall('laneSection'):
            s_start = float(lane_section.get('s', 0))
            
            # Left lanes
            left = lane_section.find('left')
            if left is not None:
                for lane in left.findall('lane'):
                    lane_id = int(lane.get('id'))
                    lane_type = lane.get('type')
                    width_elem = lane.find('width')
                    width = float(width_elem.get('a', 0)) if width_elem is not None else 0
                    lanes['left'].append({
                        'id': lane_id,
                        'type': lane_type,
                        'width': width,
                        's_start': s_start
                    })
            
            # Right lanes
            right = lane_section.find('right')
            if right is not None:
                for lane in right.findall('lane'):
                    lane_id = int(lane.get('id'))
                    lane_type = lane.get('type')
                    width_elem = lane.find('width')
                    width = float(width_elem.get('a', 0)) if width_elem is not None else 0
                    lanes['right'].append({
                        'id': lane_id,
                        'type': lane_type,
                        'width': width,
                        's_start': s_start
                    })
    
    return {
        'id': road_id,
        'name': name,
        'length': length,
        'geometries': geometries,
        'lanes': lanes
    }

def print_road_ascii(road_info: Dict):
    """Print ASCII art representation of road."""
    print(f"\n{'='*80}")
    print(f"Road {road_info['id']}: {road_info['name']}")
    print(f"Length: {road_info['length']}m")
    print(f"{'='*80}\n")
    
    # Geometry
    print("Geometry:")
    for geom in road_info['geometries']:
        print(f"  Start: s={geom['s']}m, (x={geom['x']}, y={geom['y']})")
        print(f"  Type: {geom['type']}, Length: {geom['length']}m")
        print(f"  Heading: {geom['heading']:.2f} rad ({geom['heading']*180/3.14159:.1f}°)")
        print()
    
    # Lanes
    print("Lane Layout:")
    print()
    
    # Left lanes (top in ASCII)
    if road_info['lanes']['left']:
        print("  LEFT LANES (overtaking):")
        for lane in sorted(road_info['lanes']['left'], key=lambda x: x['id'], reverse=True):
            print(f"    Lane {lane['id']:2d}: {lane['type']:10s} | Width: {lane['width']:.1f}m")
    
    print("  " + "─" * 60)
    print("  CENTER LINE (reference, lane 0)")
    print("  " + "─" * 60)
    
    # Right lanes (bottom in ASCII)
    if road_info['lanes']['right']:
        print("  RIGHT LANES (normal traffic):")
        for lane in sorted(road_info['lanes']['right'], key=lambda x: x['id']):
            print(f"    Lane {lane['id']:2d}: {lane['type']:10s} | Width: {lane['width']:.1f}m")
    
    print()

def print_ascii_diagram(road_info: Dict):
    """Print visual ASCII diagram of road cross-section."""
    print("Cross-Section View (looking along road direction):")
    print()
    
    left_lanes = sorted(road_info['lanes']['left'], key=lambda x: x['id'], reverse=True)
    right_lanes = sorted(road_info['lanes']['right'], key=lambda x: x['id'])
    
    # Print left lanes
    for lane in left_lanes:
        lane_width = int(lane['width'] * 2)  # Scale for display
        print(f"  Lane {lane['id']:2d} │ {'─' * lane_width} │")
    
    # Center line
    print(f"  Lane  0 ║ {'═' * 20} ║ (reference line)")
    
    # Right lanes
    for lane in right_lanes:
        lane_width = int(lane['width'] * 2)  # Scale for display
        print(f"  Lane {lane['id']:2d} │ {'─' * lane_width} │")
    
    print()

def print_position_guide(road_info: Dict):
    """Print guide for placing vehicles."""
    print("Vehicle Position Guide:")
    print()
    print("Use these parameters for lane-based positions:")
    print()
    
    left_lanes = sorted(road_info['lanes']['left'], key=lambda x: x['id'], reverse=True)
    right_lanes = sorted(road_info['lanes']['right'], key=lambda x: x['id'])
    
    all_lanes = left_lanes + right_lanes
    
    for lane in all_lanes:
        if lane['type'] == 'driving':
            print(f"  Lane {lane['id']:2d}: Position::lane(\"{road_info['id']}\", {lane['id']:2d}, s, 0.0, None)")
            if lane['id'] > 0:
                print(f"           ↑ Left of center (overtaking)")
            else:
                print(f"           ↑ Right of center (normal traffic)")
            print()
    
    print("Parameters:")
    print(f"  road_id: \"{road_info['id']}\"")
    print(f"  s: 0.0 to {road_info['length']}m (distance along road)")
    print("  offset: 0.0 (center of lane)")
    print()

def print_world_position_guide(road_info: Dict):
    """Print approximate world positions for lanes."""
    print("Approximate World Position Coordinates:")
    print("(for simple straight roads starting at origin)")
    print()
    
    # Assuming road starts at (0, 0) going along +X axis
    geom = road_info['geometries'][0] if road_info['geometries'] else None
    if not geom:
        return
    
    start_x = geom['x']
    start_y = geom['y']
    
    left_lanes = sorted(road_info['lanes']['left'], key=lambda x: x['id'], reverse=True)
    right_lanes = sorted(road_info['lanes']['right'], key=lambda x: x['id'])
    
    y_offset = 0
    
    # Left lanes are positive y
    for i, lane in enumerate(left_lanes):
        y_offset += lane['width']
        print(f"  Lane {lane['id']:2d}: y ≈ {y_offset:.1f}m (left of center)")
    
    print(f"  Lane  0: y = 0.0m (center line)")
    
    # Right lanes are negative y
    y_offset = 0
    for lane in right_lanes:
        y_offset -= lane['width']
        print(f"  Lane {lane['id']:2d}: y ≈ {y_offset:.1f}m (right of center)")
    
    print()
    print("Example world positions:")
    print(f"  Start of road: (x={start_x}, y=<lane_y>, z=0)")
    print(f"  Mid-point: (x={start_x + road_info['length']/2:.1f}, y=<lane_y>, z=0)")
    print(f"  End of road: (x={start_x + road_info['length']:.1f}, y=<lane_y>, z=0)")
    print()

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 analyze_opendrive.py <file.xodr>")
        sys.exit(1)
    
    filepath = sys.argv[1]
    
    try:
        root = parse_xodr(filepath)
        
        print("\n" + "="*80)
        print("OpenDRIVE Road Network Analysis")
        print("="*80)
        
        roads = root.findall('.//road')
        
        for road in roads:
            road_info = describe_road(road)
            print_road_ascii(road_info)
            print_ascii_diagram(road_info)
            print_position_guide(road_info)
            print_world_position_guide(road_info)
        
        print("="*80)
        print(f"Total roads: {len(roads)}")
        print("="*80)
        
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)

if __name__ == '__main__':
    main()
