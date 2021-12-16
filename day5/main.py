#!/usr/bin/env python

import functools
from typing import Any, List, Tuple


def count_overlap(list):
    def increment(acc, curr):
        for num in curr:
            if num >= 2:
                acc += 1

        return acc

    result = functools.reduce(increment, list, 0)
    return result


def get_input() -> List[Tuple[int, int, int, int]]:
    with open("./day5.test") as f:

        # turn input to suitable data structure
        def split_to_tuple(acc, curr):
            result_temp = curr.replace("->", ",").split(",")

            result = tuple(map(lambda item: int(item), result_temp))
            result = (result[1], result[0], result[3], result[2])

            acc.append(result)

            return acc

        lines = f.read().splitlines()
        serialized = functools.reduce(split_to_tuple, lines, [])
        return serialized


def create_initial_list() -> List[List[Any]]:
    max_x = 0
    max_y = 0

    for tup in input:
        max_tup_x = max(tup[0], tup[1])
        max_tup_y = max(tup[2], tup[3])
        if max_tup_x > max_x:
            max_x = max_tup_x

        if max_tup_y > max_y:
            max_y = max_tup_y

    result = [[0 for _ in range(max_y + 1)] for _ in range(max_x + 1)]

    return result


def part_1():
    initial_list = create_initial_list()

    for coordinate in input:
        print(coordinate)
        x1 = coordinate[0]
        y1 = coordinate[1]
        x2 = coordinate[2]
        y2 = coordinate[3]

        if x1 == x2:
            y = y1
            if y1 < y2:
                while y <= y2:
                    initial_list[x1][y] += 1

                    y += 1
            else:
                while y >= y2:
                    initial_list[x1][y] += 1

                    y -= 1

        elif y1 == y2:
            x = x1
            if x1 < x2:
                while x <= x2:
                    initial_list[x][y1] += 1

                    x += 1
            else:
                while x >= x2:
                    initial_list[x][y1] += 1

                    x -= 1

    print("part 1")
    print(initial_list)
    print(count_overlap(initial_list))


if __name__ == "__main__":
    input = get_input()
    part_1()
