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
    if name == "humn":
        return "humn"
    op = monkeys[name]
    if type(op) == int:
        return str(monkeys[name])

    num = f"({find_num(op[0])} {op[1]} {find_num(op[2])})"
    if "humn" in num:
        return num
    else:
        return str(eval(num))

# "humn"
root_names = (monkeys["root"][0], monkeys["root"][2])
first = []
second = []
print(root_names)
first = find_num(root_names[0])
second = find_num(root_names[1])
print(first)
print(second)

h_num = 100000000
while True:
    if eval(first.replace("humn", str(h_num))) == float(second):
        print(h_num)
        break
    # print(h_num, eval(first.replace("humn", str(h_num))))
    h_num += 1
        

# print(float(first) - float(second))
    



