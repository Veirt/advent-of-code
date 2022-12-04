#!/usr/bin/env python3

input_file = "./day4.input.txt"


def part_1():
    with open(input_file, "r") as f:
        lines = f.readlines()

        total_pairs = 0

        for line in lines:
            elf1, elf2 = line.split(",")
            elf1_start, elf1_end = elf1.split("-")
            elf1_range = {*range(int(elf1_start), int(elf1_end) + 1)}

            elf2_start, elf2_end = elf2.split("-")
            elf2_range = {*range(int(elf2_start), int(elf2_end) + 1)}

            if (
                elf1_range.intersection(elf2_range) == elf1_range
                or elf1_range.intersection(elf2_range) == elf2_range
            ):
                total_pairs += 1

        print(total_pairs)


def part_2():
    with open(input_file, "r") as f:
        lines = f.readlines()

        total_pairs = 0

        for line in lines:
            elf1, elf2 = line.split(",")
            elf1_start, elf1_end = elf1.split("-")
            elf1_range = {*range(int(elf1_start), int(elf1_end) + 1)}

            elf2_start, elf2_end = elf2.split("-")
            elf2_range = {*range(int(elf2_start), int(elf2_end) + 1)}

            if len(elf1_range.intersection(elf2_range)):
                total_pairs += 1

        print(total_pairs)


part_1()
part_2()
