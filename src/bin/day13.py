groups = open("src/bin/day13_input.txt").read().split("\n\n")


class NotOrder(Exception):
    pass

class InOrder(Exception):
    pass

def compare(first, second):
    # print(first, second)
    if isinstance(first, list) and isinstance(second, list):
        a = len(first)
        b = len(second)
        min_len = min(a, b)
        for i in range(min_len):
            compare(first[i], second[i])
        if b < a:
            raise NotOrder()
        if b > a:
            raise InOrder()
        
    if isinstance(first, list) and isinstance(second, int):
        compare(first, [second])
    if isinstance(first, int) and isinstance(second, list):
        compare([first], second)
    if isinstance(first, int) and isinstance(second, int):
        if first < second:
            raise InOrder()
        elif first > second:
            raise NotOrder()

in_order = []
for ind, g in enumerate(groups):
    print(g)
    first, second = g.splitlines()
    a = eval(first)
    b = eval(second)
    print(a, b)
    try:
        compare(a, b)
    except NotOrder:
        print(ind + 1, " not in order")
    except InOrder:
        in_order.append(ind + 1)
        print(ind + 1, " in order")

    print()

print(in_order, sum(in_order))