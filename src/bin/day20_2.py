from collections import deque

nums = list(map(int, open("src/bin/day20_input.txt")))
nums = [n * 811589153 for n in nums]

new = list(enumerate(nums))
L = len(new)
for x in list(enumerate(nums)) * 10:
    new.pop(j := new.index(x))
    new.insert((j + x[1]) % (L - 1), x)

out = [x[1] for x in new]
zero = out.index(0)
print(out, zero)
values = [out[i % L] for i in [1000 + zero, 2000 + zero, 3000 + zero]]
print(sum(values))

