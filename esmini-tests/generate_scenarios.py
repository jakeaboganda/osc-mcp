#!/usr/bin/env python3
"""
OpenSCENARIO Test Scenario Generator for esmini

Generates multiple test scenarios using the MCP client:
1. Speed change scenario (0 → 30 m/s)
2. Lane change scenario (lane 1 → 2)
3. Multi-action scenario (speed + lane change)

Requirements:
    pip install mcp
"""

import asyncio
import json
import os
from pathlib import Path
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client


# Output directory
SCENARIOS_DIR = Path(__file__).parent / "scenarios"
SCENARIOS_DIR.mkdir(exist_ok=True)


async def create_speed_change_scenario(session):
    """Scenario 1: Vehicle accelerates from 0 to 30 m/s over 5 seconds."""
    print("\n📝 Creating Speed Change Scenario...")
    
    # Create scenario
    result = await session.call_tool(
        "create_scenario",
        arguments={
            "name": "speed_change_test",
            "version": "1.2"
        }
    )
    scenario_id = result.content[0].text.split(": ")[1]
    
    # Add ego vehicle
    await session.call_tool(
        "add_vehicle",
        arguments={
            "scenario_id": scenario_id,
            "name": "ego",
            "category": "Car"
        }
    )
    
    # Set initial position
    await session.call_tool(
        "set_position",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "x": 0.0,
            "y": 0.0,
            "z": 0.0,
            "h": 0.0
        }
    )
    
    # Add speed action: 0 → 30 m/s over 5 seconds
    await session.call_tool(
        "add_speed_action",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "story_name": "main_story",
            "speed": 30.0,
            "duration": 5.0
        }
    )
    
    # Set stop time to 10 seconds (5s for speed change + 5s buffer)
    await session.call_tool(
        "set_stop_time",
        arguments={
            "scenario_id": scenario_id,
            "seconds": 10.0
        }
    )
    
    # Validate
    result = await session.call_tool(
        "validate_scenario",
        arguments={"scenario_id": scenario_id}
    )
    
    # Export
    output_path = str(SCENARIOS_DIR / "01_speed_change.xosc")
    result = await session.call_tool(
        "export_xml",
        arguments={
            "scenario_id": scenario_id,
            "output_path": output_path
        }
    )
    
    print(f"   ✅ Exported to: {output_path}")
    return output_path


async def create_lane_change_scenario(session):
    """Scenario 2: Vehicle changes from lane 1 to lane 2."""
    print("\n📝 Creating Lane Change Scenario...")
    
    # Create scenario
    result = await session.call_tool(
        "create_scenario",
        arguments={
            "name": "lane_change_test",
            "version": "1.2"
        }
    )
    scenario_id = result.content[0].text.split(": ")[1]
    
    # Add ego vehicle
    await session.call_tool(
        "add_vehicle",
        arguments={
            "scenario_id": scenario_id,
            "name": "ego",
            "category": "Car"
        }
    )
    
    # Set initial position (lane 1, y=0)
    await session.call_tool(
        "set_position",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "x": 0.0,
            "y": 0.0,
            "z": 0.0,
            "h": 0.0
        }
    )
    
    # Maintain constant speed
    await session.call_tool(
        "add_speed_action",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "story_name": "main_story",
            "speed": 20.0,
            "duration": 2.0
        }
    )
    
    # Lane change: lane 1 (y=0) → lane 2 (y=3.5)
    await session.call_tool(
        "add_lane_change_action",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "story_name": "main_story",
            "target_lane": 3.5,
            "duration": 4.0
        }
    )
    
    # Validate
    await session.call_tool(
        "validate_scenario",
        arguments={"scenario_id": scenario_id}
    )
    
    # Export
    output_path = str(SCENARIOS_DIR / "02_lane_change.xosc")
    result = await session.call_tool(
        "export_xml",
        arguments={
            "scenario_id": scenario_id,
            "output_path": output_path
        }
    )
    
    print(f"   ✅ Exported to: {output_path}")
    return output_path


async def create_multi_action_scenario(session):
    """Scenario 3: Vehicle accelerates AND changes lanes simultaneously."""
    print("\n📝 Creating Multi-Action Scenario...")
    
    # Create scenario
    result = await session.call_tool(
        "create_scenario",
        arguments={
            "name": "multi_action_test",
            "version": "1.2"
        }
    )
    scenario_id = result.content[0].text.split(": ")[1]
    
    # Add ego vehicle
    await session.call_tool(
        "add_vehicle",
        arguments={
            "scenario_id": scenario_id,
            "name": "ego",
            "category": "Car"
        }
    )
    
    # Set initial position
    await session.call_tool(
        "set_position",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "x": 0.0,
            "y": 0.0,
            "z": 0.0,
            "h": 0.0
        }
    )
    
    # Speed action: 0 → 25 m/s over 6 seconds
    await session.call_tool(
        "add_speed_action",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "story_name": "main_story",
            "speed": 25.0,
            "duration": 6.0
        }
    )
    
    # Lane change: y=0 → y=3.5 over 4 seconds
    await session.call_tool(
        "add_lane_change_action",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "story_name": "main_story",
            "target_lane": 3.5,
            "duration": 4.0
        }
    )
    
    # Validate
    await session.call_tool(
        "validate_scenario",
        arguments={"scenario_id": scenario_id}
    )
    
    # Export
    output_path = str(SCENARIOS_DIR / "03_multi_action.xosc")
    result = await session.call_tool(
        "export_xml",
        arguments={
            "scenario_id": scenario_id,
            "output_path": output_path
        }
    )
    
    print(f"   ✅ Exported to: {output_path}")
    return output_path


async def create_overtake_scenario(session):
    """Scenario 4: Ego overtakes slower adversary vehicle."""
    print("\n📝 Creating Overtake Scenario...")
    
    # Create scenario
    result = await session.call_tool(
        "create_scenario",
        arguments={
            "name": "overtake_test",
            "version": "1.2"
        }
    )
    scenario_id = result.content[0].text.split(": ")[1]
    
    # Add ego vehicle
    await session.call_tool(
        "add_vehicle",
        arguments={
            "scenario_id": scenario_id,
            "name": "ego",
            "category": "Car"
        }
    )
    
    # Add adversary vehicle
    await session.call_tool(
        "add_vehicle",
        arguments={
            "scenario_id": scenario_id,
            "name": "adversary",
            "category": "Truck"
        }
    )
    
    # Ego position (behind)
    await session.call_tool(
        "set_position",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "x": 0.0,
            "y": 0.0,
            "z": 0.0,
            "h": 0.0
        }
    )
    
    # Adversary position (ahead, same lane)
    await session.call_tool(
        "set_position",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "adversary",
            "x": 30.0,
            "y": 0.0,
            "z": 0.0,
            "h": 0.0
        }
    )
    
    # Ego accelerates to overtake
    await session.call_tool(
        "add_speed_action",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "story_name": "main_story",
            "speed": 30.0,
            "duration": 4.0
        }
    )
    
    # Adversary maintains slower speed
    await session.call_tool(
        "add_speed_action",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "adversary",
            "story_name": "adversary_story",
            "speed": 20.0,
            "duration": 2.0
        }
    )
    
    # Ego changes lane to overtake
    await session.call_tool(
        "add_lane_change_action",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "story_name": "main_story",
            "target_lane": 3.5,
            "duration": 3.0
        }
    )
    
    # Validate
    await session.call_tool(
        "validate_scenario",
        arguments={"scenario_id": scenario_id}
    )
    
    # Export
    output_path = str(SCENARIOS_DIR / "04_overtake.xosc")
    result = await session.call_tool(
        "export_xml",
        arguments={
            "scenario_id": scenario_id,
            "output_path": output_path
        }
    )
    
    print(f"   ✅ Exported to: {output_path}")
    return output_path


async def create_deceleration_scenario(session):
    """Scenario 5: Vehicle decelerates from 30 m/s to 10 m/s."""
    print("\n📝 Creating Deceleration Scenario...")
    
    # Create scenario
    result = await session.call_tool(
        "create_scenario",
        arguments={
            "name": "deceleration_test",
            "version": "1.2"
        }
    )
    scenario_id = result.content[0].text.split(": ")[1]
    
    # Add ego vehicle
    await session.call_tool(
        "add_vehicle",
        arguments={
            "scenario_id": scenario_id,
            "name": "ego",
            "category": "Car"
        }
    )
    
    # Set initial position
    await session.call_tool(
        "set_position",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "x": 0.0,
            "y": 0.0,
            "z": 0.0,
            "h": 0.0
        }
    )
    
    # Initial speed to 30 m/s
    await session.call_tool(
        "add_speed_action",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "story_name": "main_story",
            "speed": 30.0,
            "duration": 2.0
        }
    )
    
    # Decelerate to 10 m/s
    await session.call_tool(
        "add_speed_action",
        arguments={
            "scenario_id": scenario_id,
            "entity_name": "ego",
            "story_name": "main_story",
            "speed": 10.0,
            "duration": 5.0
        }
    )
    
    # Validate
    await session.call_tool(
        "validate_scenario",
        arguments={"scenario_id": scenario_id}
    )
    
    # Export
    output_path = str(SCENARIOS_DIR / "05_deceleration.xosc")
    result = await session.call_tool(
        "export_xml",
        arguments={
            "scenario_id": scenario_id,
            "output_path": output_path
        }
    )
    
    print(f"   ✅ Exported to: {output_path}")
    return output_path


async def main():
    """Generate all test scenarios."""
    
    print("=" * 70)
    print("🚀 OpenSCENARIO Test Scenario Generator for esmini")
    print("=" * 70)
    
    # Server parameters
    server_params = StdioServerParameters(
        command="cargo",
        args=["run", "--release", "--bin", "openscenario-mcp"],
        env={"RUST_LOG": "warn"}  # Reduce noise
    )
    
    scenarios = []
    
    try:
        print("\n🔧 Starting MCP server...")
        async with stdio_client(server_params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                print("✅ MCP session initialized")
                
                # Generate all scenarios
                scenarios.append(await create_speed_change_scenario(session))
                scenarios.append(await create_lane_change_scenario(session))
                scenarios.append(await create_multi_action_scenario(session))
                scenarios.append(await create_overtake_scenario(session))
                scenarios.append(await create_deceleration_scenario(session))
                
    except Exception as e:
        print(f"\n❌ Error: {e}")
        return 1
    
    print("\n" + "=" * 70)
    print("✅ All scenarios generated successfully!")
    print("=" * 70)
    print(f"\nGenerated {len(scenarios)} scenarios:")
    for i, path in enumerate(scenarios, 1):
        print(f"   {i}. {Path(path).name}")
    
    return 0


if __name__ == "__main__":
    exit(asyncio.run(main()))
