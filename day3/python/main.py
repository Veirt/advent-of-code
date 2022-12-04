#!/usr/bin/env python3


input_file = "./day3.input.txt"


def get_priority(char):
    if char.isupper():
        return ord(char) - 38

    return ord(char) - 96


def part_1():
    with open(input_file, "r") as f:
        priority = 0
        lines = f.readlines()

        for line in lines:
            half = int(len(line) / 2)
            first_compartment = {*line[:half]}
            second_compartment = {*line[half:]}

            duplicate = list(first_compartment.intersection(second_compartment))[0]

            priority += get_priority(duplicate)

        print(priority)


def part_2():
    with open(input_file, "r") as f:
        priority = 0
        groups = []
        current_group = []
        lines = f.readlines()

        for line in lines:
            if len(current_group) == 3:
                groups.append([*current_group])
                current_group.clear()

            current_group.append(line.rstrip())

        groups.append([*current_group])

        for group in groups:
            first_group = {*group[0]}
            second_group = {*group[1]}
            third_group = {*group[2]}

            badge = first_group.intersection(second_group, third_group)

            priority += get_priority(list(badge)[0])

        print(priority)


# part_1()
part_2()
