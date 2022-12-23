from collections import deque

input = open("src/bin/day21_input.txt").readlines()

monkeys = {}
for line in input:
    splits = line.strip().split(":")
    # print(splits)
    monkey = splits[0].strip()
    op = splits[1].strip().split(" ")
    if len(op) == 1:
        monkeys[monkey] = int(splits[1].strip())
        continue
    monkeys[monkey] = op


def find_num(name):
    if type(monkeys[name]) == int:
        return monkeys[name]
    op = monkeys[name]
    num = eval (f"{find_num(op[0])} {op[1]} {find_num(op[2])}")
    return num

print(find_num("root"))




