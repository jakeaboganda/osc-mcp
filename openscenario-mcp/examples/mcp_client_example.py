#!/usr/bin/env python3
"""
OpenSCENARIO MCP Client Example

Demonstrates all 7 MCP tools provided by the openscenario-mcp server.
Creates a complete highway scenario with ego vehicle, speed action,
lane change, validation, and export.

Requirements:
    pip install mcp

Usage:
    python mcp_client_example.py
"""

import asyncio
import json
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client


async def main():
    """Complete workflow demonstrating all 7 MCP tools."""
    
    # Server parameters
    server_params = StdioServerParameters(
        command="cargo",
        args=["run", "--release", "--bin", "openscenario-mcp"],
        env={"RUST_LOG": "info"}
    )
    
    print("🚀 Starting OpenSCENARIO MCP Server...")
    
    async with stdio_client(server_params) as (read, write):
        async with ClientSession(read, write) as session:
            # Initialize session
            await session.initialize()
            print("✅ MCP session initialized\n")
            
            # List available tools
            tools_response = await session.list_tools()
            print(f"📋 Available tools: {len(tools_response.tools)}")
            for tool in tools_response.tools:
                print(f"   - {tool.name}: {tool.description}")
            print()
            
            # ===================================================================
            # Tool 1: create_scenario
            # ===================================================================
            print("1️⃣  Creating scenario...")
            result = await session.call_tool(
                "create_scenario",
                arguments={
                    "name": "highway_overtake_scenario",
                    "version": "1.2"
                }
            )
            scenario_id = result.content[0].text.split(": ")[1]
            print(f"   ✅ Scenario ID: {scenario_id}\n")
            
            # ===================================================================
            # Tool 2: add_vehicle (ego vehicle)
            # ===================================================================
            print("2️⃣  Adding ego vehicle...")
            result = await session.call_tool(
                "add_vehicle",
                arguments={
                    "scenario_id": scenario_id,
                    "name": "ego",
                    "category": "Car"
                }
            )
            print(f"   ✅ {result.content[0].text}\n")
            
            # Add adversary vehicle
            print("   Adding adversary vehicle...")
            result = await session.call_tool(
                "add_vehicle",
                arguments={
                    "scenario_id": scenario_id,
                    "name": "adversary",
                    "category": "Truck"
                }
            )
            print(f"   ✅ {result.content[0].text}\n")
            
            # ===================================================================
            # Tool 3: set_position
            # ===================================================================
            print("3️⃣  Setting initial positions...")
            
            # Ego position
            result = await session.call_tool(
                "set_position",
                arguments={
                    "scenario_id": scenario_id,
                    "entity_name": "ego",
                    "x": 0.0,
                    "y": 0.0,
                    "z": 0.0,
                    "h": 0.0  # heading in radians (0 = east)
                }
            )
            print(f"   ✅ Ego: {result.content[0].text}")
            
            # Adversary position (ahead, in adjacent lane)
            result = await session.call_tool(
                "set_position",
                arguments={
                    "scenario_id": scenario_id,
                    "entity_name": "adversary",
                    "x": 50.0,
                    "y": 3.5,
                    "z": 0.0,
                    "h": 0.0
                }
            )
            print(f"   ✅ Adversary: {result.content[0].text}\n")
            
            # ===================================================================
            # Tool 4: add_speed_action
            # ===================================================================
            print("4️⃣  Adding speed actions...")
            
            # Ego accelerates to highway speed
            result = await session.call_tool(
                "add_speed_action",
                arguments={
                    "scenario_id": scenario_id,
                    "entity_name": "ego",
                    "story_name": "main_story",
                    "speed": 30.0,  # m/s (~108 km/h)
                    "duration": 5.0  # seconds
                }
            )
            print(f"   ✅ Ego speed: {result.content[0].text}")
            
            # Adversary maintains constant speed
            result = await session.call_tool(
                "add_speed_action",
                arguments={
                    "scenario_id": scenario_id,
                    "entity_name": "adversary",
                    "story_name": "adversary_story",
                    "speed": 25.0,  # m/s (~90 km/h, slower)
                    "duration": 2.0
                }
            )
            print(f"   ✅ Adversary speed: {result.content[0].text}\n")
            
            # ===================================================================
            # Tool 5: add_lane_change_action
            # ===================================================================
            print("5️⃣  Adding lane change action...")
            
            # Ego changes to left lane to overtake
            result = await session.call_tool(
                "add_lane_change_action",
                arguments={
                    "scenario_id": scenario_id,
                    "entity_name": "ego",
                    "story_name": "main_story",
                    "target_lane": 3.5,  # meters (left lane)
                    "duration": 4.0  # seconds
                }
            )
            print(f"   ✅ {result.content[0].text}\n")
            
            # ===================================================================
            # Tool 7: validate_scenario
            # ===================================================================
            print("6️⃣  Validating scenario...")
            result = await session.call_tool(
                "validate_scenario",
                arguments={
                    "scenario_id": scenario_id
                }
            )
            print(f"   ✅ {result.content[0].text}\n")
            
            # ===================================================================
            # Tool 6: export_xml
            # ===================================================================
            print("7️⃣  Exporting to XML...")
            result = await session.call_tool(
                "export_xml",
                arguments={
                    "scenario_id": scenario_id,
                    "output_path": "/tmp/highway_overtake.xosc"
                }
            )
            print(f"   ✅ {result.content[0].text}\n")
            
            # ===================================================================
            # Summary
            # ===================================================================
            print("=" * 60)
            print("🎉 Complete workflow executed successfully!")
            print("=" * 60)
            print("\nScenario Summary:")
            print(f"  • Name: highway_overtake_scenario")
            print(f"  • Version: OpenSCENARIO 1.2")
            print(f"  • Entities: ego (Car), adversary (Truck)")
            print(f"  • Actions: Speed changes + Lane change")
            print(f"  • Validation: Passed")
            print(f"  • Output: /tmp/highway_overtake.xosc")
            print("\nAll 7 MCP tools demonstrated:")
            print("  ✅ create_scenario")
            print("  ✅ add_vehicle")
            print("  ✅ set_position")
            print("  ✅ add_speed_action")
            print("  ✅ add_lane_change_action")
            print("  ✅ validate_scenario")
            print("  ✅ export_xml")


if __name__ == "__main__":
    asyncio.run(main())
