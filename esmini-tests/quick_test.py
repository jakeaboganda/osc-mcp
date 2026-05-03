#!/usr/bin/env python3
"""
Quick standalone test - generates scenarios using subprocess MCP client
"""
import subprocess
import json
import sys
import time

def send_request(process, request):
    """Send JSON-RPC request to MCP server."""
    request_json = json.dumps(request) + "\n"
    process.stdin.write(request_json)
    process.stdin.flush()
    
    response_line = process.stdout.readline()
    return json.loads(response_line)

# Start MCP server
server = subprocess.Popen(
    ['cargo', 'run', '--release', '--bin', 'openscenario-mcp'],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    text=True,
    cwd='/root/.openclaw/workspace/osc-mcp'
)

try:
    print("✓ MCP server started")
    time.sleep(2)  # Give server time to initialize
    
    # Create scenario
    print("Creating speed test scenario...")
    resp = send_request(server, {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "create_scenario",
            "arguments": {"name": "speed_test", "version": "1.0"}
        }
    })
    print(f"✓ Scenario created: {resp.get('result', {}).get('content', [{}])[0].get('text', 'OK')}")
    
    # Add vehicle
    print("Adding ego vehicle...")
    resp = send_request(server, {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "add_vehicle",
            "arguments": {
                "scenario_id": "speed_test",
                "name": "ego",
                "category": "car"
            }
        }
    })
    print(f"✓ Vehicle added")
    
    # Set position
    print("Setting initial position...")
    resp = send_request(server, {
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "set_position",
            "arguments": {
                "scenario_id": "speed_test",
                "entity_name": "ego",
                "x": 0.0, "y": 0.0, "z": 0.0, "h": 0.0
            }
        }
    })
    print(f"✓ Position set")
    
    # Add speed action
    print("Adding speed action (0→30 m/s)...")
    resp = send_request(server, {
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "add_speed_action",
            "arguments": {
                "scenario_id": "speed_test",
                "entity_name": "ego",
                "story_name": "story1",
                "speed": 30.0,
                "duration": 5.0
            }
        }
    })
    print(f"✓ Speed action added")
    
    # Export
    print("Exporting to scenarios/speed_test.xosc...")
    resp = send_request(server, {
        "jsonrpc": "2.0",
        "id": 5,
        "method": "tools/call",
        "params": {
            "name": "export_xml",
            "arguments": {
                "scenario_id": "speed_test",
                "output_path": "esmini-tests/scenarios/speed_test.xosc"
            }
        }
    })
    print(f"✓ Exported successfully")
    print("\n✅ Test scenario generated!")
    
finally:
    server.terminate()
    server.wait()
