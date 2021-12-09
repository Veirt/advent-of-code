#!/usr/bin/env python

# forward X increases the horizontal position by X units.
# down X increases the depth by X units.
# up X decreases the depth by X units.

horizontal = 0
depth = 0


with open("./day2.input") as f:
    for line in f.readlines():
        # split and convert into int
        command, num = line.split()
        num = int(num)

        if command == "forward":
            horizontal += num
        elif command == "down":
            depth += num
        else:
            depth -= num

print(horizontal * depth)
