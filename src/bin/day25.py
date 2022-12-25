input = open("src/bin/day25_input.txt").readlines()

snafu = {"2": 2, "1": 1, "0": 0, "-": -1, "=":-2}
to_snafu = {2: "2", 1: "1", 0: "0", -1: "-", -2: "="}

total = 0
for line in input:
    num = 0
    L = len(line.strip())
    for i, c in enumerate(line.strip()):
        num += snafu[c] * 5 ** (L - 1 - i)
    total += num

print(total)
div = total
rems = []
while div > 0:
    div, rem = divmod(div, 5)
    print(div, rem)
    if rem > 2:
        div += 1
        rems.append(rem - 5)
    else:
        rems.append(rem)

rems.reverse()
# print(rems)
print("".join((map(lambda x: to_snafu[x], rems))))

# 2=-1=0