from dataclasses import dataclass
from collections import defaultdict, deque
from queue import PriorityQueue
from functools import reduce
# importing "heapq" to implement heap queue
import heapq
import re
input = open("src/bin/day19_input.txt", 'r').readlines()

regex = re.compile(r"(\d+)")

@dataclass
class Blueprint:
    id: int
    ore_cost: int
    cy_ct: int
    obs_ct: tuple
    geo_ct: tuple

blueprints: list[Blueprint] = []
for line in input:
    mchs = list(map(int, regex.findall(line)))
    id = mchs[0]
    ore_robot = mchs[1]
    clay = mchs[2]
    obsidian = (mchs[3], mchs[4])
    geode = (mchs[5], mchs[6])
    # blueprints.append((id, ))
    blue = Blueprint(id, ore_robot, clay, obsidian, geode)
    blueprints.append(blue)
    # print(line, blue)


@dataclass(frozen=True, order=True)
class State:
    time: int
    ore: int
    cy: int
    obs: int
    geo: int
    rb_ore: int
    rb_cy: int
    rb_obs: int
    rb_geo: int

def get_priority(state: State):
    materials = state.geo * 1000 + state.obs * 100 + state.cy * 10 + state.ore
    a = 1
    robots = state.rb_geo * 1000*a + state.rb_obs * 100*a + state.rb_cy * 10*a + state.rb_ore*a
    return materials + robots

TIME = 32

def get_geodes(blueprint):
    obsidian = blueprint[4][1]
    max_geode_robots = TIME / 2 / obsidian
    print(obsidian, max_geode_robots)
    
def add_materials(state: State):
    ore = state.ore + state.rb_ore
    clay = state.cy + state.rb_cy
    obsidian = state.obs + state.rb_obs
    geode = state.geo + state.rb_geo
    return State(state.time, ore, clay, obsidian, geode, state.rb_ore, state.rb_cy,
                 state.rb_obs, state.rb_geo)

def get_max_geodes(state: State):
    rem = TIME - state.time
    out = state.geo + state.rb_geo * rem
    out += sum(range(rem))
    return out

out = []
prune = 50000
for blueprint in blueprints[:3]:
    print(blueprint)
    visited = set()
    to_visit: list[State] = list()
    old_state = State(0, 0, 0, 0, 0, 1, 0, 0, 0)
    best_state = old_state
    # print(old_state)
    to_visit.append(old_state)
    max_geodes = 0
    max_ore = max([blueprint.ore_cost, blueprint.cy_ct, blueprint.obs_ct[0], blueprint.geo_ct[0]])
    max_clay = blueprint.obs_ct[1]
    max_obsidian = blueprint.geo_ct[1]

    while to_visit:
        old_state = to_visit.pop(0)
        
        if len(to_visit) >= prune*1.5:
            # print("Pruning")
            rem = list(to_visit)
            rem.sort(key=lambda x: get_priority(x), reverse=True)
            to_visit = rem[:prune]

        if old_state.time >= TIME and old_state.geo > max_geodes:
            max_geodes = max(max_geodes, old_state.geo)
            best_state = old_state
        time = old_state.time + 1
        
        if old_state in visited:
            continue
        visited.add(old_state)
        if old_state.time >= TIME:
            continue
        # Geodes
        if old_state.ore >= blueprint.geo_ct[0] and old_state.obs >= blueprint.geo_ct[1]:
            state = add_materials(old_state)
            ore = state.ore - blueprint.geo_ct[0]
            obsidian = state.obs - blueprint.geo_ct[1]
            robots_geode = state.rb_geo + 1
            state = State(time, ore, state.cy, obsidian, state.geo, state.rb_ore,
                          state.rb_cy, state.rb_obs, robots_geode)
            to_visit.append(state)
        # Obsidian
        if old_state.ore >= blueprint.obs_ct[0] and old_state.cy >= blueprint.obs_ct[1] \
            and not old_state.rb_obs > max_obsidian:
            state = add_materials(old_state)
            ore = state.ore - blueprint.obs_ct[0]
            clay = state.cy - blueprint.obs_ct[1]
            robots_obsidian = state.rb_obs + 1
            state = State(time, ore, clay, state.obs, state.geo, state.rb_ore,
                          state.rb_cy, robots_obsidian, state.rb_geo)
            to_visit.append(state)
        # Clay
        if old_state.ore >= blueprint.cy_ct and \
                not old_state.rb_cy > max_clay:
            state = add_materials(old_state)
            ore = state.ore - blueprint.cy_ct
            robots_clay = state.rb_cy + 1
            state = State(time, ore, state.cy, state.obs, state.geo, state.rb_ore,
                            robots_clay, state.rb_obs, state.rb_geo)
            to_visit.append(state)
        # Ore
        if old_state.ore >= blueprint.ore_cost and \
                not old_state.rb_ore > max_ore:
            state = add_materials(old_state)
            ore = state.ore - blueprint.ore_cost
            robots_ore = state.rb_ore + 1
            state = State(time, ore, state.cy, state.obs, state.geo, robots_ore,
                            state.rb_cy, state.rb_obs, state.rb_geo)
            to_visit.append(state)

        old_state = add_materials(old_state)
        state = State(time, old_state.ore, old_state.cy, old_state.obs, old_state.geo,
                        old_state.rb_ore, old_state.rb_cy, old_state.rb_obs, old_state.rb_geo)
        to_visit.append(state)
        # saturate_queue(to_visit)
        if len(visited) % 50000 == 0:
            print(state, len(to_visit), get_max_geodes(old_state), max_geodes)
    
    print("Best: ", best_state)
    print(max_geodes)
    out.append(max_geodes)

print(out)
from operator import mul
print(reduce(mul, out))


# 1093 too low

# 1144 part1