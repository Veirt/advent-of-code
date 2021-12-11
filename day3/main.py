#!/usr/bin/env python3

import functools


def increment(acc, curr):
    curr_len = len(curr)

    for idx in range(curr_len):
        bit = curr[idx]

        acc[idx][int(bit)] += 1

    return acc


def calculate_ans(acc, curr):
    common = curr.index(max(curr))
    least = curr.index(min(curr))

    acc["gamma"] += str(common)
    acc["epsilon"] += str(least)

    return acc


def part_1():
    with open("./day3.input") as f:
        lines = f.read().splitlines()

        initial_list = [[0, 0] for i in range(len(lines[0]))]

        reports = functools.reduce(increment, lines, initial_list)

        result = functools.reduce(calculate_ans, reports, {"gamma": "", "epsilon": ""})

        ans = int(result["gamma"], 2) * int(result["epsilon"], 2)
        print(ans)


if __name__ == "__main__":
    part_1()
