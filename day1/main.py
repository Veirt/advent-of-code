#!/usr/bin/env python

count = 0
with open("./day1.input") as f:
    inputs = list(map(lambda i: int(i), f.readlines()))
    prev = inputs[0]

    for curr in inputs:
        if prev < curr:
            count += 1

        prev = curr


print(count)
