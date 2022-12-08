def part_one(aoc_data):
    dupcount = 0
    for line in aoc_data.splitlines():
        a, b = (set(range(int(x), int(y) + 1)) for x, y in
                [s.split("-") for s in line.split(",")])
        dupcount += (a.issubset(b) or b.issubset(a))

    print(dupcount)


def part_two(aoc_data):
    overlaps = 0
    for line in aoc_data.splitlines():
        a, b = (set(range(int(x), int(y) + 1)) for x, y in
                [s.split("-") for s in line.split(",")])
        overlaps += not a.isdisjoint(b)

    print(overlaps)


if __name__ == "__main__":
    import os
    os.chdir(os.path.dirname(__file__))
    f = open("../../inputs/04.txt")
    aoc = f.read()
    f.close()
    part_one(aoc)
    part_two(aoc)
