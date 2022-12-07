#!/usr/bin/env python3

from collections import defaultdict

directory = defaultdict(int)

input_file = "./day7.input.txt"

with open(input_file, "r") as f:
    path = []

    f.readline()  # skip /

    while line := f.readline():
        if line.startswith("$ cd"):
            dir_name = line.split()[2]

            if dir_name == "..":
                path.pop()
            else:
                path.append(dir_name)
        elif line.startswith("$ ls"):
            pass
        else:
            if not line.startswith("dir"):
                size = line.split()[0]

                for i in range(len(path) + 1):
                    curr = "/".join(path[:i])

                    if curr == "":
                        curr = "/"

                    directory[curr] += int(size)


total_size = sum(filter(lambda size: size < 100_000, directory.values()))

disk_space = 70_000_000
update_space = 30_000_000
outermost = directory["/"]

unused_space = disk_space - outermost
required_space = update_space - unused_space

filtered = filter(lambda size: size >= required_space, directory.values())

print(min(filtered))
