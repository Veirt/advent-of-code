#!/usr/bin/env python3


def solution_1():
    max = 0
    sum_list = []
    with open("./day1.input.txt", "r") as f:
        lines = [*f.readlines(), "\n"]
        temp = 0
        for line in lines:
            if line != "\n":
                temp += int(line)
            else:
                max = temp if temp > max else max
                sum_list.append(temp)
                temp = 0

    print(max)  # part 1
    print(sum(sorted(sum_list)[-3:]))  # part 2


def solution_2():
    max_list = [0]
    with open("./day1.input.txt", "r") as f:
        lines = [*f.readlines(), "\n"]
        temp = 0
        for line in lines:
            if line != "\n":
                temp += int(line)
            else:
                if temp > min(max_list):
                    max_list.append(temp)

                if len(max_list) > 3:
                    max_list.pop(0)

                temp = 0

    print(max(max_list))  # part 1
    print(max_list)  # part 2
