from collections import defaultdict
import re


input = open("src/bin/day23_input.txt").readlines()


def n_neighs(elf, limits):
    neigh = set()
    j = -1
    for i in range(-1, 2):
        if check_limits(elf, i, j, limits):
            neigh.add((elf[0]+i, elf[1]+j))
    return neigh

def s_neighs(elf, limits):
    neigh = set()
    j = 1
    for i in range(-1, 2):
        if check_limits(elf, i, j, limits):
            neigh.add((elf[0]+i, elf[1]+j))
    return neigh

def w_neighs(elf, limits):
    neigh = set()
    i = -1
    for j in range(-1, 2):
        if check_limits(elf, i, j, limits):
            neigh.add((elf[0]+i, elf[1]+j))
    return neigh

def e_neighs(elf, limits):
    neigh = set()
    i = 1
    for j in range(-1, 2):
        if check_limits(elf, i, j, limits):
            neigh.add((elf[0]+i, elf[1]+j))
    return neigh


def check_limits(elf, i, j, limits):
    # if elf[0]+i < 0 or elf[0]+i >= limits[0]:
    #     return False
    # if elf[1]+j < 0 or elf[1]+j >= limits[1]:
    #     return False
    if i == 0 and j == 0:
        return False
    return True

def print_elfs(elfs, limits):
    for j in range(limits[1]):
        for i in range(limits[0]):
            if (i, j) in elfs:
                print("#", end="")
            else:
                print(".", end="")
        print()
    print()

def get_neigh(elf, limits, limitsd=(0,0)):
    neigh = set()
    for i in range(-1, 2):
        for j in range(-1, 2):
            if check_limits(elf, i, j, limits):
                neigh.add((elf[0]+i, elf[1]+j))
            # print(elf[0]+i, elf[1]+j)
    return neigh

elfs = set()
limits = len(input[0].strip()), len(input)
for j, line in enumerate(input):
    for i, char in enumerate(line):
        if char == "#":
            elfs.add((i, j))
print(elfs)

for r in range(10):
    print("Round", r+1)
    props = defaultdict(list)
    for elf in elfs:
        neighs = get_neigh(elf, limits)
        if len(neighs & elfs) == 0:
            continue
        for i in range(4):
            # print((i + r) % 4)
            dir = [(0, -1), (0, 1), (-1, 0), (1, 0)][(i + r) % 4]
            fun = [n_neighs, s_neighs, w_neighs, e_neighs][(i + r) % 4]
            neighs = fun(elf, limits)
            if neighs and len(neighs & elfs) == 0 :
                props[(elf[0] + dir[0], elf[1] + dir[1])].append(elf)
                break
    
    for pos, lelf in props.items():
        if len(lelf) > 1:
            continue
        elfs.remove(lelf[0])
        elfs.add(pos)
        # print("Elf", lelf, "moved to", pos)

    print_elfs(elfs, limits)

limitsd = min([elf[0] for elf in elfs]), min([elf[1] for elf in elfs])
limits = max([elf[0] for elf in elfs]), max([elf[1] for elf in elfs])
tiles = (limits[0] - limitsd[0] + 1) * (limits[1] - limitsd[1] + 1) - len(elfs)
print("Part 1:", tiles)

# 4158
