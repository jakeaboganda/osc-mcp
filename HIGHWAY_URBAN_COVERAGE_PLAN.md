# OpenSCENARIO Coverage Plan: Highway & Urban Driving

**Goal**: Complete implementation of OpenSCENARIO elements required for realistic highway and urban driving scenarios.

**Date**: 2026-05-10  
**Current Status**: ~30% coverage (basic actions, simple conditions)  
**Target**: 95% coverage for highway/urban use cases

---

## Current Implementation Status

### ✅ Fully Implemented

**Actions**:
- `SpeedAction` - Speed changes with dynamics
- `LaneChangeAction` - Lane changes with duration

**Conditions**:
- `SimulationTimeCondition` - Time-based triggers
- `StoryboardElementStateCondition` - Element state triggers
- `ParameterCondition` - Parameter-based logic

**Positions**: All 7 types implemented
- WorldPosition, LanePosition, RoadPosition
- RelativeWorld, RelativeObject, RelativeLane, RelativeRoad

**Entities**: All types supported
- Vehicle (Car, Truck, Bus, Motorcycle, Bicycle)
- Pedestrian
- MiscObject

### 🚧 Partially Implemented (structs defined, not fully integrated)

**Actions**:
- `LaneOffsetAction` ⚠️ - Struct exists, needs XML export + MCP tool
- `PositionAction` ⚠️ - Struct exists, needs XML export + MCP tool
- `DistanceAction` ⚠️ - Struct exists, needs integration
- `LongitudinalDistanceAction` ⚠️ - Struct exists, needs integration
- `FollowTrajectoryAction` ⚠️ - Struct exists, needs integration
- `AssignRouteAction` ⚠️ - Struct exists, needs integration
- `SynchronizeAction` ⚠️ - Struct exists, needs integration

**Conditions**:
- `SpeedCondition` ⚠️ - Struct exists, needs full integration
- `ByEntityCondition` ⚠️ - Partial, needs expansion

### ❌ Not Yet Implemented

**Critical for Highway/Urban**:
- `AccelerationAction` - Acceleration/deceleration control
- `TeleportAction` - Instant position changes
- `VisibilityAction` - Entity visibility control
- `ReachPositionCondition` - Position-based triggers
- `TimeHeadwayCondition` - Following distance triggers
- `RelativeSpeedCondition` - Speed differential triggers
- `CollisionCondition` - Collision detection triggers
- `OffroadCondition` - Lane departure detection
- `TimeToCollisionCondition` - TTC-based triggers
- `AccelerationCondition` - Acceleration-based triggers
- `StandStillCondition` - Stop detection
- `RelativeDistanceCondition` - Distance-based triggers

**Traffic Control**:
- `TrafficSignalAction` - Traffic light control
- `TrafficSignalStateCondition` - Signal state triggers
- `TrafficSignalControllerAction` - Signal controller management

**Advanced**:
- `UserDefinedAction` - Custom actions
- `UserDefinedValueCondition` - Custom conditions
- `ControllerAction` - Controller assignment/override
- `GlobalAction` (EnvironmentAction, EntityAction, ParameterAction)

---

## Priority Matrix: Highway & Urban Scenarios

### 🔴 **Critical Priority** (Must-have for basic scenarios)

#### Actions
1. **AccelerationAction** - Essential for realistic vehicle dynamics
   - Target: Acceleration value (m/s²)
   - Dynamics: Time/distance/rate-based
   - Use case: Smooth merging, emergency braking

2. **TeleportAction** - Scenario initialization and resets
   - Target: Position
   - Use case: Place vehicles at start, test edge cases

3. **LaneOffsetAction** (complete integration)
   - Use case: Lane keeping, avoiding obstacles

4. **PositionAction** (complete integration)
   - Use case: Waypoint following, scenario checkpoints

#### Conditions
1. **ReachPositionCondition** - Trigger on spatial goals
   - Tolerance: Distance threshold
   - Use case: "When ego reaches intersection, pedestrian crosses"

2. **TimeHeadwayCondition** - Following behavior
   - Target: Time gap (seconds)
   - Use case: "Maintain 2-second following distance"

3. **RelativeDistanceCondition** - Proximity triggers
   - Distance type: Longitudinal, lateral, Euclidean
   - Freespace: Boolean
   - Use case: "When vehicle within 5m, brake"

4. **CollisionCondition** - Safety-critical events
   - Collision type: Entity-entity, entity-object
   - Use case: "Trigger evasive maneuver on potential collision"

5. **SpeedCondition** (complete integration)
   - Use case: "When speed exceeds 50 km/h, activate cruise control"

### 🟡 **High Priority** (Important for realistic scenarios)

#### Actions
1. **LongitudinalDistanceAction** (complete)
   - Use case: Adaptive cruise control, platooning

2. **FollowTrajectoryAction** (complete)
   - Use case: Complex paths, curved roads

3. **AssignRouteAction** (complete)
   - Use case: Navigation, multi-waypoint paths

4. **SynchronizeAction** (complete)
   - Use case: Coordinated maneuvers, platooning

5. **VisibilityAction**
   - Use case: Fog scenarios, blind spot simulation

#### Conditions
1. **TimeToCollisionCondition**
   - Target: TTC threshold (seconds)
   - Use case: "When TTC < 2s, warn driver"

2. **RelativeSpeedCondition**
   - Use case: "When approaching slower vehicle, change lane"

3. **AccelerationCondition**
   - Use case: "When hard braking detected, warn following vehicle"

4. **StandStillCondition**
   - Duration: Time stationary
   - Use case: "When stopped at red light for 30s, proceed"

5. **OffroadCondition**
   - Duration: Time off-road
   - Use case: "When vehicle leaves lane, trigger warning"

### 🟢 **Medium Priority** (Nice-to-have for advanced scenarios)

#### Traffic Control
1. **TrafficSignalAction**
   - Use case: Intersection scenarios, signal timing

2. **TrafficSignalStateCondition**
   - Use case: "When light turns green, accelerate"

3. **TrafficSignalControllerAction**
   - Use case: Adaptive signal control testing

#### Advanced
1. **ControllerAction**
   - Controller types: Lateral, longitudinal, combined
   - Use case: ADAS system activation, autopilot engagement

2. **UserDefinedAction**
   - Use case: Custom behaviors not in spec

3. **EnvironmentAction**
   - Weather, time of day, road friction
   - Use case: Rain scenario, night driving

---

## Implementation Roadmap

### Phase 1: Critical Actions (2 weeks)

**Week 1: Core Actions**
- [ ] **AccelerationAction** implementation
  - Struct definition (if not exists)
  - XML export in `xml.rs`
  - MCP tool: `add_acceleration_action`
  - Unit tests: basic acceleration
  - Integration test: emergency braking scenario

- [ ] **TeleportAction** implementation
  - Struct definition
  - XML export
  - MCP tool: `teleport_entity`
  - Unit tests: position teleport
  - Integration test: scenario reset

- [ ] **Complete LaneOffsetAction**
  - XML export (likely missing)
  - MCP tool: `add_lane_offset_action`
  - Unit tests
  - Integration test: obstacle avoidance

**Week 2: Position & Distance**
- [ ] **Complete PositionAction**
  - XML export
  - MCP tool: `set_entity_position`
  - Unit tests
  - Integration test: waypoint following

- [ ] **Complete DistanceAction**
  - XML export
  - MCP tool: `add_distance_action`
  - Unit tests
  - Integration test: following distance

### Phase 2: Critical Conditions (2 weeks)

**Week 3: Spatial Conditions**
- [ ] **ReachPositionCondition**
  - Struct definition
  - XML export
  - Condition evaluation logic
  - Unit tests
  - Integration test: intersection entry

- [ ] **RelativeDistanceCondition**
  - Struct definition (with distance types: longitudinal, lateral, Euclidean)
  - XML export
  - Condition evaluation
  - Unit tests
  - Integration test: proximity warning

- [ ] **CollisionCondition**
  - Struct definition
  - XML export
  - Collision detection integration
  - Unit tests
  - Integration test: crash avoidance

**Week 4: Dynamic Conditions**
- [ ] **Complete SpeedCondition**
  - Ensure XML export complete
  - Add MCP condition creation tool
  - Unit tests
  - Integration test: speed-based behavior

- [ ] **TimeHeadwayCondition**
  - Struct definition
  - XML export
  - Evaluation logic
  - Unit tests
  - Integration test: following scenario

### Phase 3: High Priority Actions (2 weeks)

**Week 5: Trajectory & Distance**
- [ ] **Complete LongitudinalDistanceAction**
  - XML export validation
  - MCP tool
  - Integration test: ACC scenario

- [ ] **Complete FollowTrajectoryAction**
  - XML export validation
  - MCP tool: `add_trajectory_action`
  - Trajectory vertex handling
  - Integration test: curved path

**Week 6: Route & Sync**
- [ ] **Complete AssignRouteAction**
  - XML export validation
  - MCP tool: `assign_route`
  - Waypoint handling
  - Integration test: navigation scenario

- [ ] **Complete SynchronizeAction**
  - XML export validation
  - MCP tool: `add_synchronize_action`
  - Integration test: platooning

- [ ] **VisibilityAction**
  - Struct definition
  - XML export
  - MCP tool
  - Integration test: sensor simulation

### Phase 4: High Priority Conditions (1 week)

**Week 7: Advanced Conditions**
- [ ] **TimeToCollisionCondition**
  - Struct, XML, evaluation
  - Integration test: TTC-based warning

- [ ] **RelativeSpeedCondition**
  - Struct, XML, evaluation
  - Integration test: adaptive behavior

- [ ] **AccelerationCondition**
  - Struct, XML, evaluation
  - Integration test: hard braking detection

- [ ] **StandStillCondition**
  - Struct, XML, evaluation
  - Integration test: stopped vehicle

- [ ] **OffroadCondition**
  - Struct, XML, evaluation
  - Integration test: lane departure

### Phase 5: Traffic Control (1 week)

**Week 8: Traffic Signals**
- [ ] **TrafficSignalAction**
  - Struct definition
  - XML export
  - MCP tool
  - Integration test: intersection with signals

- [ ] **TrafficSignalStateCondition**
  - Struct, XML, evaluation
  - Integration test: signal-based triggers

- [ ] **TrafficSignalControllerAction**
  - Struct definition
  - XML export
  - MCP tool

### Phase 6: Testing & Validation (1 week)

**Week 9: Comprehensive Testing**
- [ ] **Highway scenario suite**
  - Merge scenarios (on-ramp, lane merge)
  - Following scenarios (ACC, platooning)
  - Lane change scenarios (overtaking, exit)
  - Emergency scenarios (hard braking, collision avoidance)

- [ ] **Urban scenario suite**
  - Intersection scenarios (4-way stop, traffic light)
  - Pedestrian scenarios (crosswalk, jaywalking)
  - Parking scenarios (parallel, perpendicular)
  - Traffic scenarios (congestion, stop-and-go)

- [ ] **Validation with esmini**
  - Run all scenarios in simulator
  - Visual inspection
  - Performance benchmarks

---

## Definition of Done (per feature)

### For Each Action
- [x] Struct defined in `storyboard.rs`
- [x] XML export implemented in `xml.rs`
- [x] MCP tool created in `openscenario-mcp/src/tools.rs`
- [x] Unit tests in `openscenario/tests/`
- [x] Integration test with scenario
- [x] Documented in README
- [x] Example in `examples/`
- [x] Validated in esmini

### For Each Condition
- [x] Struct defined in `storyboard.rs`
- [x] XML export implemented in `xml.rs`
- [x] Evaluation logic (if applicable)
- [x] Unit tests
- [x] Integration test with triggers
- [x] Documented
- [x] Example
- [x] Validated in esmini

---

## Success Metrics

### Code Coverage
- **Actions**: 15/20 implemented (75%)
- **Conditions**: 12/15 implemented (80%)
- **Test coverage**: >90% line coverage
- **Integration tests**: 30+ end-to-end scenarios

### Scenario Coverage
**Highway scenarios**:
- ✅ Lane merge (on-ramp)
- ✅ Adaptive cruise control
- ✅ Lane change (overtaking)
- ✅ Emergency braking
- ✅ Exit maneuver

**Urban scenarios**:
- ✅ 4-way intersection (stop sign)
- ✅ Signalized intersection (traffic light)
- ✅ Pedestrian crossing
- ✅ Stop-and-go traffic
- ✅ Parking maneuver

### Validation
- ✅ All scenarios run in esmini without errors
- ✅ XSD validation passes
- ✅ Visual inspection confirms expected behavior
- ✅ Performance: <100ms scenario generation time

---

## Example Scenarios Enabled by Full Coverage

### 1. Highway Merge (Critical Actions + Conditions)
```
Ego vehicle accelerates (AccelerationAction)
When distance to merge point < 100m (ReachPositionCondition):
  - Check traffic speed (SpeedCondition)
  - Adjust speed to match (LongitudinalDistanceAction)
  - When gap available (RelativeDistanceCondition):
    - Change lane (LaneChangeAction)
    - Maintain distance (TimeHeadwayCondition)
```

### 2. Urban Intersection (Traffic Control)
```
Ego approaches intersection (FollowTrajectoryAction)
When traffic light is red (TrafficSignalStateCondition):
  - Decelerate to stop (AccelerationAction)
  - Wait until stopped (StandStillCondition)
When light turns green (TrafficSignalStateCondition):
  - Accelerate (AccelerationAction)
  - Cross intersection (PositionAction)
```

### 3. Emergency Braking (Safety Conditions)
```
Ego following lead vehicle (LongitudinalDistanceAction)
When TTC < 2 seconds (TimeToCollisionCondition):
  - Emergency brake (AccelerationAction, -9.81 m/s²)
When collision imminent (CollisionCondition):
  - Full brake + warning
```

### 4. Adaptive Cruise Control (High Priority Features)
```
Set cruise speed (SpeedAction)
Detect lead vehicle (RelativeDistanceCondition)
When lead vehicle present:
  - Match speed (RelativeSpeedCondition)
  - Maintain headway (TimeHeadwayCondition)
When lead vehicle faster:
  - Resume cruise speed (SpeedAction)
```

---

## Next Steps

### Immediate (This Week)
1. **Review current XML export** for partially-implemented actions
2. **Create feature branches** for each action/condition
3. **Start with AccelerationAction** (highest priority)
4. **Write tests first** (TDD approach)

### Tools & Resources
- **OpenSCENARIO 1.2 Spec**: Reference for exact XML structure
- **esmini**: Validation and visualization
- **Existing tests**: Pattern for new tests
- **XSD schemas**: Validation targets

### Team Coordination (if applicable)
- Assign features to developers
- Daily standups for blockers
- Code reviews before merge
- Integration testing every Friday

---

**Ready to start?** Recommend beginning with **AccelerationAction** as it's critical for both highway and urban scenarios and will establish patterns for other actions.

Let me know which feature you'd like to tackle first! 🚗
