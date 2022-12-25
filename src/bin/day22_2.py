import re
board, orders = open("src/bin/day22_input.txt").read().split("\n\n")
board = board.split("\n")
orders = re.split("(\d+)", orders)[1:-1]

facings = {
    (1, 0): (0, ">"),
    (0, 1): (1, "v"),
    (-1, 0): (2, "<"),
    (0, -1): (3, "^"),
}

row = 0
col = 0
drow = 0
dcol = 1
W = max([len(line) for line in board])
board = [line.ljust(W) for line in board]
# board = [line + " " * (W - len(line)) for line in board]

while board[row][col] != ".":
    col += 1

for ord in orders:
    if ord == "L":
        drow, dcol = -dcol, drow
    elif ord == "R":
        drow, dcol = dcol, -drow
    else:
        for _ in range(int(ord)):
            old_drow = drow
            old_dcol = dcol
            next_row = row + drow
            next_col = col + dcol
            if next_row < 0 and 50 <= next_col < 100 and drow == -1:
                drow, dcol = 0, 1
                next_row = next_col + 100
                next_col = 0
            elif next_col < 0 and 150 <= next_row < 200 and dcol == -1:
                drow, dcol = 1, 0
                next_col = next_row - 100
                next_row = 0
            elif next_row < 0 and 100 <= next_col < 150 and drow == -1:
                next_col = next_col - 100
                next_row = 199
            elif next_row >= 200 and 0 <= next_col < 50 and drow == 1: 
                next_col = next_col + 100
                next_row = 0
            elif next_col >= 150 and 0 <= next_row < 50 and dcol == 1:
                dcol = -1
                next_row = 149 - next_row
                next_col = 99
            elif next_col == 100 and 100 <= next_row < 150 and dcol == 1:
                dcol = -1
                next_row = 149 - next_row
                next_col = 149
            elif next_row == 50 and 100 <= next_col < 150 and drow == 1:
                drow, dcol =  0, -1
                next_row = next_col - 50
                next_col = 99
            elif next_col == 100 and 50 <= next_row < 100 and dcol == 1:
                drow, dcol = -1, 0
                next_col = next_row + 50
                next_row = 49
            elif next_row == 150 and 50 <= next_col < 100 and drow == 1:
                drow, dcol = 0, -1
                next_row = next_col + 100
                next_col = 49
            elif next_col == 50 and 150 <= next_row < 200 and dcol == 1:
                drow, dcol = -1, 0
                next_col = next_row - 100
                next_row = 149
            elif next_row == 99 and 0 <= next_col < 50 and drow == -1:
                drow, dcol = 0, 1
                next_row = next_col + 50
                next_col = 50
            elif next_col == 49 and 50 <= next_row < 100 and dcol == -1:
                drow, dcol = 1, 0
                next_col = next_row - 50
                next_row = 100
            elif next_col == 49 and 0 <= next_row < 50 and dcol == -1:
                dcol = 1
                next_row = 149 - next_row
                next_col = 0
            elif next_col < 0 and 100 <= next_row < 150 and dcol == -1:
                dcol = 1
                next_row = 149 - next_row
                next_col = 50
            print(next_row, next_col)
            if board[next_row][next_col] == "#":
                drow, dcol = old_drow, old_dcol
                break
            row = next_row
            col = next_col

points = 1000 * (row + 1) + 4 * (col + 1) + facings[(dcol, drow)][0]
print(row, col, drow, dcol)
print(points)

# 61360 too low