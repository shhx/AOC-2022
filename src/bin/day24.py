from collections import deque
import re

input = open("src/bin/day24_input.txt").readlines()
board = input

W = len(board[0].strip())
L = len(board)

bliz_list = []
for y, line in enumerate(board):
    chars = list(line)
    for x, char in enumerate(chars):
        if char == "<":
            bliz_list.append((x, y, -1, 0))
        elif char == ">":
            bliz_list.append((x, y, 1, 0))
        elif char == "^":
            bliz_list.append((x, y, 0, -1))
        elif char == "v":
            bliz_list.append((x, y, 0, 1))
bliz = {}
for t in range(1000):
    new_bliz = set()
    list_b = []
    for x, y, dx, dy in bliz_list:
        x = (x-1 + dx * t) % (W-2) + 1
        y = (y-1 + dy * t) % (L-2) + 1
        list_b.append((x + dx, y + dy, dx, dy))
        # print(x + dx*t, y + dy*t, x, y)

        new_bliz.add((x, y))
    
    bliz[t] = new_bliz

print("calculated bliz")
def print_board(i, j, t):
    bl = bliz[t]
    print()
    print("".join(map(str, range(W))))
    for y in range(L):
        for x in range(W):
            if x == i and y == j:
                print("X", end="")
            elif (x,y) in bl:
                print("#", end="")
            else:
                print(".", end="")
        print(f" {y}", end="")
        print()

seen = set()
start = (1, 0, 0)
to_visit = deque([start])
end = (W-2, L-1)
while True:
    x,y,step = to_visit.popleft()
    # print(x, y, step)
    # print_board(x, y, step+1)
    if (x,y,step) in seen:
        continue
    seen.add((start[0], start[1], step))
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
        xn = x + dir[0]
        yn = y + dir[1]
        if (xn, yn) == end:
            print("Found", step+1)
            exit()
        # print(xn, yn, step, xn >= 1 and xn < W-1 and yn >= 1 and yn < L-1 and (xn,yn) not in bliz[step+1])
        if xn >= 1 and xn < W-1 and yn >= 1 and yn < L-1 and (xn,yn) not in bliz[step+1]:
            # print("Moving", xn, yn)
            to_visit.append((xn, yn, step+1))
    # if (x,y) not in bliz[step+1]:
    #     print("Moving", x, y)
    to_visit.append((x, y, step+1))
    # print(x, y, step)
