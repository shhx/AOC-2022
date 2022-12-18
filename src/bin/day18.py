# input = list(map(lambda x: list(map(int, x.strip().split(","))),
#                  open("src/bin/day18_example.txt", 'r').readlines()))
input = list(map(lambda x: list(map(int, x.strip().split(","))),
                 open("src/bin/day18_input.txt", 'r').readlines()))

cubes = set()
total_faces = 0
for x, y, z in input:
    faces = 6
    cubes.add((x, y, z))
    if (x - 1, y, z) in cubes:
        faces -= 2
    if (x + 1, y, z) in cubes:
        faces -= 2
    if (x, y + 1, z) in cubes:
        faces -= 2
    if (x, y - 1, z) in cubes:
        faces -= 2
    if (x, y, z + 1) in cubes:
        faces -= 2
    if (x, y, z - 1) in cubes:
        faces -= 2
    total_faces += faces

print(total_faces)
