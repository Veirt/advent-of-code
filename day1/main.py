#!/usr/bin/env python


def part_1():
    count = 0
    with open("../day1.input") as f:
        inputs = list(map(lambda i: int(i), f.readlines()))
        prev = inputs[0]

        for curr in inputs:
            if prev < curr:
                count += 1

            prev = curr

    print(count)


def part_2():
    count = 0

    with open("../day2.input") as f:
        inputs = list(map(lambda i: int(i), f.readlines()))
        input_len = len(inputs)

        prev = (inputs[0], inputs[1], inputs[2])
        sum_prev = sum(prev)

        for i in range(input_len):
            if i > input_len - 3:
                break

            curr = (inputs[i], inputs[i + 1], inputs[i + 2])
            sum_curr = sum(curr)

            if sum_prev < sum_curr:
                count += 1

            sum_prev = sum_curr

    print(count)


if __name__ == "__main__":
    part_1()
    part_2()
