#!/usr/bin/env python3
# ord('a') is 97, 'a' is prio 1 => ord('a') - 96
# ord('A') is 65, 'A' is prio 27 => ord('A') - (64 - 26)
offset_lower = 96
offset_upper = (64 - 26)


def part_one(aoc_data):
    dupval = 0

    txt = aoc_data.splitlines()
    for line in txt:
        l = len(line) // 2
        first = frozenset(line[:l])
        second = frozenset(line[l:])
        for char in first:
            if char in second:
                dupval += ord(char) - (offset_lower if char.islower()
                                       else offset_upper)

    print(dupval)


def part_two(aoc_data):
    badgeval = 0

    txt = aoc_data.splitlines()
    for i in range(0, len(txt), 3):
        elf1, elf2, elf3 = [frozenset(t) for t in txt[i:i+3]]
        for char in elf1:
            if char in elf2 and char in elf3:
                badgeval += ord(char) - (offset_lower if char.islower()
                                         else offset_upper)
                continue

    print(badgeval)


if __name__ == "__main__":
    import os
    os.chdir(os.path.dirname(__file__))
    f = open("../../inputs/03.txt")
    aoc = f.read()
    f.close()
    part_one(aoc)
    part_two(aoc)
