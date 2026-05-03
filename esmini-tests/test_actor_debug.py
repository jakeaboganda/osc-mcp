#!/usr/bin/env python3
"""Debug actor assignment"""
import subprocess
import json
import time
import sys

server = subprocess.Popen(
    ['cargo', 'run', '--release', '--bin', 'openscenario-mcp'],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    text=True,
    cwd='/root/.openclaw/workspace/osc-mcp'
)

def send_request(request):
    request_json = json.dumps(request) + "\n"
    server.stdin.write(request_json)
    server.stdin.flush()
    response_line = server.stdout.readline()
    return json.loads(response_line)

try:
    time.sleep(2)
    
    # Create scenario
    resp = send_request({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "create_scenario",
            "arguments": {"name": "actor_debug", "version": "1.2"}
        }
    })
    scenario_id = resp["result"]["content"][0]["text"].split(": ")[1]
    print(f"Scenario ID: {scenario_id}")
    
    # Add vehicle
    resp = send_request({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "add_vehicle",
            "arguments": {"scenario_id": scenario_id, "name": "ego", "category": "car"}
        }
    })
    print(f"Add vehicle response: {resp}")
    
    # Set position
    resp = send_request({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "set_position",
            "arguments": {"scenario_id": scenario_id, "entity_name": "ego", "x": 0.0, "y": 0.0, "z": 0.0, "h": 0.0}
        }
    })
    print(f"Set position response: {resp}")
    
    # Add speed action (this should create story structure AND add actor)
    resp = send_request({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "add_speed_action",
            "arguments": {"scenario_id": scenario_id, "entity_name": "ego", "story_name": "main_story", "speed": 30.0, "duration": 5.0}
        }
    })
    print(f"Add speed action response: {resp}")
    
    # Set stop time
    resp = send_request({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "tools/call",
        "params": {
            "name": "set_stop_time",
            "arguments": {"scenario_id": scenario_id, "seconds": 10.0}
        }
    })
    print(f"Set stop time response: {resp}")
    
    # Export
    resp = send_request({
        "jsonrpc": "2.0",
        "id": 6,
        "method": "tools/call",
        "params": {
            "name": "export_xml",
            "arguments": {"scenario_id": scenario_id, "output_path": "esmini-tests/scenarios/actor_debug.xosc"}
        }
    })
    print(f"Export response: {resp}")
    
    print("\n✅ Done! Checking XML...")
    
finally:
    server.terminate()
    # Read stderr
    stderr_output = server.stderr.read()
    if stderr_output:
        print(f"\n=== STDERR ===\n{stderr_output}")
    server.wait()
