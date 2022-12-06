#!/usr/bin/env python3

input_file = "./day6.input.txt"


def part_1():

    with open(input_file, "r") as f:
        line = f.readline().rstrip()

        for i in range(len(line) - 3):
            current_buffer = line[i : i + 4]

            if (len({*current_buffer})) == 4:
                print(i + 4)
                return


def part_2():

    with open(input_file, "r") as f:
        line = f.readline().rstrip()

        for i in range(len(line) - 13):
            current_buffer = line[i : i + 14]

            if (len({*current_buffer})) == 14:
                print(i + 14)
                return


part_1()
part_2()
