#!/usr/bin/env python3

input_file = "./day10.input.txt"

registers = 1
cycles = 0
max_cycle = 40


positions = 0

ans = ""


with open(input_file) as f:
    lines = f.readlines()

    for line in lines:
        if line.startswith("addx"):
            # addx
            _, v = line.split()
            v = int(v)

            for _ in range(2):
                cycles += 1

                sprite = [(registers - 1) + i for i in range(3)]

                if positions in sprite:
                    ans += "#"
                else:
                    ans += "."

                # print(f"cycle: {cycles}")
                # print(positions, line.rstrip())
                # print(f"register: {registers}")
                # print(sprite)
                # print(ans)
                # print()
                # positions += 1

                if cycles == max_cycle:
                    ans += "\n"
                    positions = 0
                    max_cycle += 40

            registers += v
        else:
            # noop
            cycles += 1

            if positions in [(registers - 1) + i for i in range(3)]:
                ans += "#"
            else:
                ans += "."

            positions += 1

            if cycles == max_cycle:
                ans += "\n"
                positions = 0
                max_cycle += 40
