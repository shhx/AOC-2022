from collections import deque


# input = list(map(lambda x: list(map(int, x.strip().split(","))),
#               open("src/bin/day18_example.txt", 'r').readlines()))
input = list(map(lambda x: list(map(int, x.strip().split(","))),
                 open("src/bin/day18_input.txt", 'r').readlines()))


def add_tuple(a, b):
    return tuple(x1 + x2 for x1, x2 in zip(a, b))


cubes = set()
total_faces = 0
for x, y, z in input:
    cube = (x, y, z)
    cubes.add(cube)

grid_limits_x = (min(cubes, key=lambda x: x[0])[0], max(cubes, key=lambda x: x[0])[0])
grid_limits_y = (min(cubes, key=lambda x: x[1])[1], max(cubes, key=lambda x: x[1])[1])
grid_limits_z = (min(cubes, key=lambda x: x[2])[2], max(cubes, key=lambda x: x[2])[2])
print(grid_limits_x, grid_limits_y, grid_limits_z)
space_seen = set()
faces = 0
this_cube = (0, 0, 0)
to_visit = deque()
to_visit.append((0, 0, 0))
while len(to_visit) > 0:
    cube = to_visit.popleft()
    if cube in space_seen:
        continue
    space_seen.add(cube)

    # sorrounding cubes
    for side in [(1, 0, 0), (-1, 0, 0),
                 (0, -1, 0), (0, 1, 0),
                 (0, 0, -1), (0, 0, 1)]:

        sor = add_tuple(cube, side)
        if sor in cubes:
            faces += 1
            continue
        if sor[0] in range(grid_limits_x[0] - 1, grid_limits_x[1] + 2) and \
                sor[1] in range(grid_limits_y[0] - 1, grid_limits_y[1] + 2) and \
                sor[2] in range(grid_limits_z[0] - 1, grid_limits_z[1] + 2):
            to_visit.append(sor)

print(faces)
# 4036 too high
