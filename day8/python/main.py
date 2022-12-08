#!/usr/bin/env python3
import math

input_file = "./day8.input.txt"

grid = []

with open(input_file, "r") as f:
    lines = f.readlines()
    for idx, line in enumerate(lines):
        for tree in line.rstrip():
            try:
                grid[idx].append(tree)
            except IndexError:
                grid.append([])
                grid[-1].append(tree)


def part_1():
    visible = 0

    for idx_i, i in enumerate(grid):
        for idx_j, j in enumerate(i):
            if (
                idx_i == 0
                or idx_j == 0
                or idx_i == len(grid) - 1
                or idx_j == len(grid) - 1
            ):
                visible += 1
                continue

            horizontal_left = i[:idx_j]
            horizontal_right = i[idx_j + 1 :]

            # print(horizontal_left, horizontal_right)
            visible_from_left = all(j > height for height in horizontal_left)
            visible_from_right = all(j > height for height in horizontal_right)

            if visible_from_left or visible_from_right:
                visible += 1
                continue

            vertical = [i[idx_j] for i in grid]

            vertical_up = vertical[:idx_i]
            vertical_down = vertical[idx_i + 1 :]

            visible_from_up = all(j > height for height in vertical_up)
            visible_from_down = all(j > height for height in vertical_down)
            if visible_from_up or visible_from_down:
                visible += 1
                continue

    print(visible)


def part_2():
    highest_scenic_score = 0
    for idx_i, i in enumerate(grid):
        for idx_j, j in enumerate(i):
            left = 0
            right = 0
            top = 0
            bottom = 0

            horizontal_left = i[:idx_j]
            horizontal_right = i[idx_j + 1 :]

            for tree in horizontal_left[::-1]:
                left += 1

                if tree >= j:
                    break

            for tree in horizontal_right:
                right += 1

                if tree >= j:
                    break

            vertical = [i[idx_j] for i in grid]

            vertical_up = vertical[:idx_i]
            vertical_down = vertical[idx_i + 1 :]

            for tree in vertical_up[::-1]:
                top += 1

                if tree >= j:
                    break

            for tree in vertical_down:
                bottom += 1

                if tree >= j:
                    break

            # print(i)
            # print(horizontal_left, horizontal_right)
            # print(left, right)
            # print(top, bottom)

            scenic_score = math.prod([left, right, top, bottom])
            if scenic_score > highest_scenic_score:
                highest_scenic_score = scenic_score

    print(highest_scenic_score)


part_2()
