#!/usr/bin/env python3


ans = 0


input_file = "./day10.input.txt"

cycles = 0

registers = 1

max_cycle = 20

with open(input_file) as f:
    lines = f.readlines()

    for line in lines:
        if line.startswith("addx"):
            # addx
            _, v = line.split()
            v = int(v)

            for _ in range(2):
                cycles += 1

                if cycles == max_cycle:
                    ans += max_cycle * registers
                    max_cycle += 40

            registers += v
        else:
            # noop
            # print(registers)
            cycles += 1

            if cycles == max_cycle:
                ans += max_cycle * registers
                max_cycle += 40


print(ans)
