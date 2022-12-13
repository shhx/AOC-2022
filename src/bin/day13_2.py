import functools

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

groups = open("src/bin/day13_input.txt").readlines()
groups = list(filter(lambda x: x != "\n", groups))
lines = list(map(lambda x: eval(x.strip()), groups))
lines.append([[2]])
lines.append([[6]])

def custom_cmp(a, b):
    try: 
        compare(a, b)
    except NotOrder:
        return -1
    except InOrder:
        return 1

g_sorted = sorted(lines, key=functools.cmp_to_key(custom_cmp), reverse=True)
for g in g_sorted:
    print(g)

a = g_sorted.index([[2]])
b = g_sorted.index([[6]])
print(a, b, a * b)
