#!/usr/bin/env python3

input_file = "./day5.input.txt"


def serialize(cargo):
    stacks = []
    length = len(cargo[0])

    for line in cargo:
        # disini salah
        for idx, (i, j) in enumerate(zip(range(0, length, 4), range(3, length, 4))):
            crate = line[i:j].strip()
            try:
                stacks[idx].append(crate)
            except IndexError:
                stacks.insert(idx, [crate])

    stacks = [
        list(filter(lambda crate: crate != "" and not crate.isnumeric(), stack))
        for stack in stacks
    ]

    return stacks


def part_1():
    with open(input_file, "r") as f:
        cargo = []
        lines = f.readlines()
        separator = lines.index("\n")
        cargo = lines[:separator]
        rearrangements = lines[separator + 1 :]

        cargo = serialize(cargo)

        for rearrangement in rearrangements:
            splitted = rearrangement.split()
            # how many
            moves = int(splitted[1])
            # which stack
            stack = int(splitted[3])
            # to which
            destination = int(splitted[5])

            for _ in range(moves):
                crate = cargo[stack - 1].pop(0)
                cargo[destination - 1].insert(0, crate)

        for stack in cargo:
            print(stack[0][1], end="")


def part_2():
    with open(input_file, "r") as f:
        cargo = []
        lines = f.readlines()
        separator = lines.index("\n")
        cargo = lines[:separator]
        rearrangements = lines[separator + 1 :]

        cargo = serialize(cargo)

        for rearrangement in rearrangements:
            splitted = rearrangement.split()
            # how many
            moves = int(splitted[1])
            # which stack
            stack = int(splitted[3])
            # to which
            destination = int(splitted[5])

            crate = cargo[stack - 1][:moves]
            for _ in range(moves):
                cargo[stack - 1].pop(0)
            cargo[destination - 1] = [*crate, *cargo[destination - 1]]

        for stack in cargo:
            print(stack[0][1], end="")


part_2()
