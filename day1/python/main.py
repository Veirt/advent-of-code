#!/usr/bin/env python3


def solution_1():
    max = 0
    sum_list = []
    with open("./day1.input.txt", "r") as f:
        lines = [*f.readlines(), "\n"]
        temp_max = 0
        for line in lines:
            if line != "\n":
                temp_max += int(line)
            else:
                max = temp_max if temp_max > max else max
                sum_list.append(temp_max)
                temp_max = 0

    print(max)  # part 1
    print((sorted(sum_list)[-3:]))  # part 2
    print(sum(sorted(sum_list)[-3:]))  # part 2


def solution_2():
    max_list = [0]
    with open("./day1.input.txt", "r") as f:
        lines = [*f.readlines(), "\n"]
        temp_max = 0
        for line in lines:
            if line != "\n":
                temp_max += int(line)
            else:
                if temp_max > min(max_list):
                    max_list.append(temp_max)

                if len(max_list) > 3:
                    max_list.remove(min(max_list))

                temp_max = 0

    print(max(max_list))  # part 1
    print((max_list))  # part 2
    print(sum(max_list))  # part 2


solution_1()
print()
solution_2()
