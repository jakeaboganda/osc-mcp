# Phase 4 Examples - Scenario Visualizations

This document describes what each Phase 4 scenario looks like when visualized in esmini.

---

## Running the Examples

### Generate Scenario Files

```bash
cd openscenario
cargo run --example highway_merge
cargo run --example lane_change_overtaking
cargo run --example emergency_braking
cargo run --example platooning
```

### Visualize in esmini

```bash
# Install esmini from https://github.com/esmini/esmini/releases

# Run a scenario
./esmini --osc highway_merge.xosc
```

---

## Highway Scenarios

### 1. Highway Merge

**File**: `highway_merge.xosc`

**Description**: A vehicle merges from an on-ramp onto the highway using smooth sinusoidal lane change dynamics.

**Initial State**:
```
On-ramp:    [🚗]
            │
Highway:    ─────────────────
```

**Action Sequence**:
1. Vehicle starts on on-ramp (y=3.5m offset)
2. Executes lane change over 4 seconds
3. Merges smoothly into highway lane using sinusoidal transition

**Final State**:
```
On-ramp:    
            
Highway:    ───[🚗]───────────
```

**Key Features**:
- Sinusoidal dynamics for smooth, natural merging
- Single vehicle scenario
- Position-based starting point

**What to observe**:
- Smooth S-curve trajectory during merge
- Lateral velocity gradually increases then decreases
- Natural-looking merge behavior

---

### 2. Lane Change Overtaking

**File**: `lane_change_overtaking.xosc`

**Description**: A faster vehicle approaches a slower vehicle and performs an overtaking maneuver.

**Initial State**:
```
Left lane:   ─────────────────

Right lane:  ───[🚗fast]────[🚗slow]───
                 (x=0)      (x=100)
```

**Action Sequence**:
1. Fast vehicle approaches slow vehicle
2. Changes to left lane (3 seconds)
3. Passes the slow vehicle
4. Returns to right lane (3 seconds)

**Mid-maneuver**:
```
Left lane:   ──────[🚗fast]────

Right lane:  ───────────[🚗slow]───
```

**Final State**:
```
Left lane:   ─────────────────

Right lane:  ─────[🚗slow]───[🚗fast]──
```

**Key Features**:
- Two-vehicle interaction
- Multi-step maneuver (left, then right)
- Sinusoidal lane change dynamics

**What to observe**:
- Smooth lane changes in both directions
- Fast vehicle maintains forward velocity while changing lanes
- Realistic overtaking behavior

---

### 3. Emergency Braking

**File**: `emergency_braking.xosc`

**Description**: A following vehicle detects collision risk and performs emergency braking.

**Initial State**:
```
───[🚗follower]──────────[🚗lead]───
    (x=0)              (x=50)
```

**Trigger**: Collision condition detected

**Action**: Emergency brake at -8 m/s² for 2 seconds

**During Braking**:
```
──────[🚗follower]🛑────[🚗lead]───
       (braking hard)
```

**Final State** (collision avoided):
```
─────────[🚗follower]──[🚗lead]───
          (stopped safely)
```

**Key Features**:
- Collision condition trigger
- Strong deceleration (-8 m/s² ≈ 0.8g)
- Safety-critical scenario

**What to observe**:
- Immediate brake response when condition triggers
- Strong but realistic deceleration
- Safe stopping distance maintained

**Physics**:
- -8 m/s² = emergency brake force
- 2 second duration
- Δv = -16 m/s (57.6 km/h speed reduction)

---

### 4. Platooning

**File**: `platooning.xosc`

**Description**: Three vehicles travel in coordinated formation, maintaining 1.5-second time headway.

**Initial State**:
```
───[🚗f2]─────────[🚗f1]─────────[🚗leader]───
   (x=0)          (x=50)         (x=100)
    
   f2 = follower 2
   f1 = follower 1
```

**Formation**:
```
Time headway = 1.5 seconds between each vehicle
```

**Steady State**:
```
──[🚗f2]══1.5s══[🚗f1]══1.5s══[🚗leader]──
   └─follows f1─┘        └─follows leader┘
```

**When Leader Slows**:
```
──[🚗f2]🛑─[🚗f1]🛑─[🚗leader]🛑──
   (adjusts)   (adjusts)  (braking)
   
   All followers adjust speed to maintain headway
```

**Key Features**:
- Three-vehicle coordination
- Time headway conditions (1.5s)
- Speed profile actions for distance maintenance
- Chain of following behavior

**What to observe**:
- Consistent spacing maintained at all speeds
- Cascade effect: leader slows → f1 responds → f2 responds
- Smooth speed adjustments (no jerky braking)

**Physics**:
- Time headway (THW) = distance / velocity
- THW < threshold → speed adjustment triggered
- Example: At 25 m/s, 1.5s THW = 37.5m distance

---

## Technical Notes

### Sinusoidal vs Linear Dynamics

**Sinusoidal** (used in merge/overtaking):
- Smooth acceleration/deceleration
- S-curve trajectory
- Natural human-like behavior

**Linear** (simpler scenarios):
- Constant rate
- Straight-line trajectory
- Easier to predict

### Collision Detection

The `add_event_with_collision_condition` method monitors:
- Bounding box overlap
- Continuous collision checking
- Triggers immediately when detected

### Time Headway

Formula: `THW = distance / relative_velocity`

Safety guidelines:
- THW < 1.0s = dangerous
- THW = 1.5s = comfortable (used in platooning)
- THW = 2.0s = safe (common ACC setting)
- THW > 3.0s = conservative

---

## Creating Your Own Scenarios

### Pattern 1: Single Vehicle Behavior

```rust
scenario.add_vehicle("ego", params)?;
scenario.set_initial_position("ego", Position::world(0.0, 0.0, 0.0, 0.0))?;
scenario.add_story("test")?;
// Add actions directly
```

Use for: Testing maneuvers, lane changes, speed changes

### Pattern 2: Two-Vehicle Interaction

```rust
scenario.add_vehicle("vehicle1", params.clone())?;
scenario.add_vehicle("vehicle2", params)?;
// Set different initial positions
// Add interaction conditions (collision, distance, headway)
```

Use for: Following behavior, overtaking, collision avoidance

### Pattern 3: Multi-Vehicle Coordination

```rust
// Add N vehicles
// Create separate maneuver groups for each
// Link behaviors with conditions
```

Use for: Platooning, traffic scenarios, complex interactions

---

## Recommended Reading Order

1. **hello_world.rs** - Simplest scenario, understand basics
2. **lane_change.rs** - Single vehicle lane change
3. **highway_merge.rs** - Lane change with realistic context
4. **lane_change_overtaking.rs** - Two-vehicle interaction
5. **adaptive_cruise_control.rs** - Condition + action linkage
6. **emergency_braking.rs** - Collision detection
7. **platooning.rs** - Multi-vehicle coordination

---

## Troubleshooting

### Scenario doesn't run in esmini

**Problem**: "No road network found"
**Solution**: Our examples use simple world positions. For lane-based scenarios, add an OpenDRIVE road network:

```rust
scenario.set_road_network(Some("path/to/road.xodr".to_string()));
```

**Problem**: Vehicles disappear
**Solution**: Check initial positions are valid (not underground: z ≥ 0)

**Problem**: Actions don't trigger
**Solution**: Verify story hierarchy is complete:
- Story exists
- Act exists within story
- ManeuverGroup exists within act
- Actor added to maneuver group
- Maneuver exists
- Event exists within maneuver

### Performance Issues

esmini may run slowly on complex scenarios with many vehicles. Try:
- `--fixed_timestep 0.02` for faster playback
- `--headless` to run without viewer
- Reduce number of vehicles in scene

---

## Next Steps

- Try modifying the examples (change speeds, distances, timings)
- Combine maneuvers (e.g., merge + lane change + acceleration)
- Add more vehicles to test scaling
- Experiment with different dynamics shapes
- Add road networks for realistic environments

---

## Resources

- **esmini documentation**: https://esmini.github.io/
- **OpenSCENARIO spec**: https://www.asam.net/standards/detail/openscenario/
- **Our API docs**: `cargo doc --open`
- **More examples**: `openscenario/examples/`
