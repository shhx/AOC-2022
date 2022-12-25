from collections import defaultdict
import re

class Line:
    offset = 2**32
    length = 0

    def __repr__(self) -> str:
        return f"Line(offset={self.offset}, length={self.length})"

def rotate(dir, right):
    x, y = dir
    if not right:
        return y, -x
    else:
        return -y, x

facings = {
    (1, 0): (0, ">"),
    (0, 1): (1, "v"),
    (-1, 0): (2, "<"),
    (0, -1): (3, "^"),
}

board, orders = open("src/bin/day22_input.txt").read().split("\n\n")
board = board.split("\n")
orders = re.split("(\d+)", orders)[1:-1]

walls = set()
rows = defaultdict(Line)
columns = defaultdict(Line)
for y, line in enumerate(board):
    chars = list(line)
    # print(chars)
    for x, char in enumerate(chars):
        if char == " ":
            continue
        else:
            rows[y].offset = min(x, rows[y].offset)
            rows[y].length = max(x, rows[y].length)
        if char == "#":
            walls.add((x, y))
    rows[y].length -= rows[y].offset

for y, line in enumerate(board):
    for x, char in enumerate(line):
        if char == " ":
            continue
        else:
            columns[x].offset = min(y, columns[x].offset)
            columns[x].length = max(y, columns[x].length)

for col in columns.values():
    col.length -= col.offset


dir = (1, 0)
start = (rows[0].offset, 0)
for ord in orders:
    if ord == "L":
        dir = rotate(dir, False)
    elif ord == "R":
        dir = rotate(dir, True)
    else:
        assert start[0] in columns
        assert start[1] in rows
        column = columns[start[0]]
        row = rows[start[1]]
        cnt = int(ord)
        end = start
        while cnt > 0:
            x = end[0] + dir[0]
            y = end[1] + dir[1]
            if row.offset > x:
                x = (x + row.offset) % (row.length+1) + row.offset
            elif x > row.offset + row.length:
                x = x % (row.length+1 + row.offset) + row.offset
            if column.offset > y:
                y = (y + column.offset) % (column.length+1) + column.offset
            elif y > column.offset + column.length:
                y = y % (column.length+1+column.offset) + column.offset
            if (x, y) in walls:
                # print("hit wall", end)
                break
            end = (x, y)
            cnt -= 1
        start = end
        # print(start, facings[dir][1])
    # print(start, facings[dir][0], ord)

points = 1000 * (start[1] + 1) + 4 * (start[0] + 1) + facings[dir][0]
print(points)

# 2352 too low
# 131172 too high
# 64256
# (64, 64) 0