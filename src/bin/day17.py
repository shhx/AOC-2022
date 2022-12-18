# input = list(open("src/bin/day17_input.txt", 'r').read().strip())
input = list(open("src/bin/day17_example.txt", 'r').read().strip())
streams = [1 if x == '>' else -1 for x in input]
print(input)


def get_rock(t, y): # set of (x,y) pairs
    if t == 0:
        return set([(2,y), (3,y), (4,y), (5,y)])
    elif t == 1:
        return set([(3, y+2), (2, y+1), (3,y+1), (4,y+1), (3,y)])
    elif t == 2:
        return set([(2, y), (3,y), (4,y), (4,y+1), (4,y+2)])
    elif t == 3:
        return set([(2,y),(2,y+1),(2,y+2),(2,y+3)])
    elif t == 4:
        return set([(2,y+1),(2,y),(3,y+1),(3,y)])
    else:
        assert False


def in_bounds(rock, ocupied):
    for p in rock:
        if  p[0] < 0 or p[0] >= 7 or p in ocupied:
            return False
    return True


def print_rocks(ocupied, max=20):
    for y in range(max, 0, -1):
        for x in range(0, 7):
            print('#' if (x, y) in ocupied else ".", end="")
        print()
    print()

def get_state():
    a = [-20] * 7
    for r in ocupied:
        x = r[0]
        y = r[1]
        a[x] = max(a[x], y)
    top = max(a)
    return tuple(x - top for x in a)


rock_count = 0
top = 0
MAX = 1000000000000
# MAX = 2022
ocupied = set([(x, -1) for x in range(7)])
seen = {}
len_streams = len(streams)
counter = 0
offset = 0
while rock_count < MAX:
    rock = get_rock(rock_count % 5, top + 3)
    while True:
        stream = streams[counter % len_streams]
        counter += 1
        if in_bounds([(x[0] + stream, x[1]) for x in rock], ocupied):
            # print("Moved: ", stream)
            rock = set([(x[0] + stream, x[1]) for x in rock])
        if in_bounds([(x[0], x[1] - 1) for x in rock], ocupied):
            # print("Moved down: ", - 1)
            rock = set([(x[0], x[1] - 1) for x in rock])
        else:
            ocupied |= rock
            top = max(x[1] for x in ocupied) + 1
            rock_count += 1
            state = (rock_count % 5, counter % len_streams, get_state())
            if state in seen:
                print("Got state")
                r_cnt, h = seen[state]
                rem = MAX - rock_count
                rep = rem // (rock_count - r_cnt)
                print(r_cnt, h, rem, rep)
                offset = rep * (top - h)
                rock_count += rep * (rock_count - r_cnt)
                seen = {}
            seen[state] = (rock_count, top)
            # print(seen)
            # print(top)
            break
    # print_rocks(ocupied, top + 5)

print(top + offset)

# 1514285714288
