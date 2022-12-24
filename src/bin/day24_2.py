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
for t in range(800):
    new_bliz = set()
    for x, y, dx, dy in bliz_list:
        x = (x-1 + dx * t) % (W-2) + 1
        y = (y-1 + dy * t) % (L-2) + 1
        new_bliz.add((x, y))

    bliz[t] = new_bliz

print("calculated bliz")
def print_board(i, j, t):
    bl = bliz[t]
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
start = (1, 0)
to_visit = deque([(start[0], start[1], 0)])
iend = (W-2, L-1)
end = iend
ride = 0
while True:
    x,y,step = to_visit.popleft()
    if (x,y,step) in seen:
        continue
    if (x, y) == end:
        print("Found", step-1)
        seen.clear()
        to_visit.clear()
        start, end = end, start
        to_visit.append((start[0], start[1], step-1))
        print("Going back: ", ride, step-1)
        ride += 1
        if ride == 3:
            print("Found", step-1)
            break
    # print(x, y, step)
    # print_board(x, y, step)
    seen.add((x,y,step))
    if (x,y) not in bliz[step]:
        to_visit.append((x, y, step+1))
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
        xn = x + dir[0]
        yn = y + dir[1]
        if (xn, yn) == end:
            to_visit.append((xn, yn, step+1))
        if xn >= 1 and xn < W-1 and yn >= 1 and yn < L-1 and (xn,yn) not in bliz[step]:
            to_visit.append((xn, yn, step+1))
    # if len(seen) % 1000 == 0:
    #     print(len(seen), step)

# 247