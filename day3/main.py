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

        initial_list = [[0, 0] for _ in range(len(lines[0]))]

        reports = functools.reduce(increment, lines, initial_list)

        initial_result = {"gamma": "", "epsilon": ""}
        result = functools.reduce(calculate_ans, reports, initial_result)

        ans = int(result["gamma"], 2) * int(result["epsilon"], 2)
        print(ans)


def calculate_oxygen_rating(sample_len, lines):
    for i in range(sample_len):
        count = [0, 0]

        def increment(acc, curr):
            acc[int(curr[i])] += 1

            return acc

        reports = functools.reduce(increment, lines, count)

        most = reports.index(max(reports))
        least = reports.index(min(reports))

        # if most and least is equal
        if most == least:
            lines = list(filter(lambda line: line[i] == "1", lines))
            continue

        # print(f"most: {most}")
        # print(list(lines))
        lines = list(filter(lambda line: line[i] == str(most), lines))

        if len(lines) == 1:
            return lines

        # print(list(lines))


def calculate_co2_rating(sample_len, lines):
    for i in range(sample_len):
        count = [0, 0]

        def increment(acc, curr):
            acc[int(curr[i])] += 1

            return acc

        reports = functools.reduce(increment, lines, count)

        most = reports.index(max(reports))
        least = reports.index(min(reports))

        # if most and least is equal
        if most == least:
            lines = list(filter(lambda line: line[i] == "0", lines))
            continue

        # print(f"least: {least}")
        # print(list(lines))
        lines = list(filter(lambda line: line[i] == str(least), lines))

        if len(lines) == 1:
            return lines

        # print(list(lines))


def part_2():
    with open("./day3.input") as f:
        lines = f.read().splitlines()

        sample_len = len(lines[0])

        oxygen_rating_list = calculate_oxygen_rating(sample_len, lines)
        co2_scrubber_list = calculate_co2_rating(sample_len, lines)

        if oxygen_rating_list and co2_scrubber_list:
            print(oxygen_rating_list)
            print(co2_scrubber_list)
            ans = int(oxygen_rating_list[0], 2) * int(co2_scrubber_list[0], 2)
            print(ans)


if __name__ == "__main__":
    part_1()
    # fuck part 2
    part_2()
