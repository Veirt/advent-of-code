#!/usr/bin/env python3

input_file = "./day8.input.txt"

grid = []
visible = 0

with open(input_file, "r") as f:
    lines = f.readlines()
    for idx, line in enumerate(lines):
        for tree in line.rstrip():
            try:
                grid[idx].append(tree)
            except IndexError:
                grid.append([])
                grid[-1].append(tree)


for idx_i, i in enumerate(grid):
    for idx_j, j in enumerate(i):
        if idx_i == 0 or idx_j == 0 or idx_i == len(grid) - 1 or idx_j == len(grid) - 1:
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
