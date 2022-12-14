from itertools import zip_longest
from functools import cmp_to_key


def comp(left, right):
    """
    Oldschool compare function like I was taught in Java in my freshman year.

    I tried normal list comparison before, but that gave the wrong result.
    ~~shoutout to Setsu who taught me Java in Python works:tm:~~

    Returns a number greater than zero if right < left
    Returns 0 if right == left
    Returns a number lower than zero if right > left
    """
    if isinstance(left, int) and isinstance(right, int):
        return left - right
    elif isinstance(left, list) and isinstance(right, list):
        for l, r in zip_longest(left, right, fillvalue=None):
            if l is None:
                return -1
            if r is None:
                return 1
            check = comp(l, r)
            if check != 0:
                return check
        return 0
    else:
        newleft = [left] if isinstance(left, int) else left
        newright = [right] if isinstance(right, int) else right
        return comp(newleft, newright)


def part_one(aoc_data):
    chunks = aoc_data.split("\n\n")
    idxsum = 0
    index = 0
    for chunk in chunks:
        index += 1
        left, right = [eval(line) for line in chunk.splitlines()]
        idxsum += index if comp(left, right) < 1 else 0
    print(idxsum)


def part_two(aoc_data):
    chunks = aoc_data.split("\n\n")
    chunks.append("[[2]]\n[[6]]")
    processed = []
    for chunk in chunks:
        for line in chunk.splitlines():
            processed.append(eval(line))
    processed.sort(key=cmp_to_key(comp))
    two_idx = 0
    curidx = 0;
    find2 = True
    for p in processed:
        curidx += 1
        if find2 and p == [[2]]:
            two_idx = curidx
            find2 = False
        if p == [[6]]:
            return print(curidx * two_idx)


if __name__ == "__main__":
    import os
    os.chdir(os.path.dirname(__file__))
    f = open("../../inputs/13.txt")
    aoc = f.read()
    f.close()
    part_one(aoc)
    part_two(aoc)
