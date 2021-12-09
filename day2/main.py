#!/usr/bin/env python

# forward X increases the horizontal position by X units.
# down X increases the depth by X units.
# up X decreases the depth by X units.


def part_1():
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


# down X increases your aim by X units.
# up X decreases your aim by X units.
# forward X does two things:
#     It increases your horizontal position by X units.
#     It increases your depth by your aim multiplied by X.


def part_2():
    horizontal = 0
    depth = 0
    aim = 0

    with open("./day2.input") as f:
        for line in f.readlines():
            # split and convert into int
            command, num = line.split()
            num = int(num)

            if command == "forward":
                horizontal += num
                if aim > 0:
                    depth += num * aim
            elif command == "down":
                aim += num
            else:
                aim -= num

    print(horizontal * depth)


if __name__ == "__main__":
    part_1()
    part_2()
