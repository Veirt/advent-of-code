#!/usr/bin/env python


def check_bingo_win(marked_boards):
    vertical = [0 for _ in range(5)]
    for row in marked_boards:
        horizontal = 0

        for item_idx, item in enumerate(row):
            horizontal += int(item)
            vertical[item_idx] += int(item)

            if horizontal == 5 or vertical[item_idx] == 5:
                return True


def loop_drawn_number(boards, drawn_numbers):
    marked_boards = {}
    for i in range(len(boards)):
        # 5 = length of bingo board
        marked_boards[i] = [[0, 0, 0, 0, 0] for _ in range(5)]

    for num in drawn_numbers:
        for key, value in boards.items():
            board = value

            for row_idx, row in enumerate(board):
                # if the drawn number is in board row
                if num in row:
                    # get the index
                    idx = row.index(num)
                    marked_boards[key][row_idx][idx] = 1
                    win = check_bingo_win(marked_boards[key])
                    if win:
                        return (key, marked_boards[key], num)


def get_boards():
    lines = file.read().splitlines()

    drawn_numbers = lines.pop(0).split(",")
    boards = {}

    idx = None

    while len(lines) != 0:
        current = lines.pop(0)

        if current == "":
            idx = 0 if idx is None else idx + 1
            boards[idx] = []
            continue

        if idx in boards:
            current = current.split(" ")
            # filter the empty string
            current = list(filter(lambda i: i != "", current))

            boards[idx].append(current)

    return drawn_numbers, boards


def part_1(drawn_numbers, boards):
    # i could have check that it must be drawn 5 times first.
    # but no. i'm done with this.
    # i will just check it every time, lol

    winner_key, winner_marked_boards, last_num = loop_drawn_number(
        boards, drawn_numbers
    )

    total_unmarked_number = 0

    for (
        marked_board,
        board,
    ) in zip(winner_marked_boards, boards[winner_key]):
        for m, b in zip(marked_board, board):
            if m == 0:
                total_unmarked_number += int(b)

    ans = int(last_num) * int(total_unmarked_number)
    print(ans)


def get_last_win(boards, drawn_numbers):
    marked_boards = {}
    winner_list = []

    for i in range(len(boards)):
        # 5 = length of bingo board
        marked_boards[i] = [[0, 0, 0, 0, 0] for _ in range(5)]

    for num in drawn_numbers:

        for key, value in boards.items():
            board = value

            # if already win, no need to loop again.
            if key in winner_list:
                continue

            for row_idx, row in enumerate(board):
                # if the drawn number is in board row
                if num in row:
                    # get the index
                    idx = row.index(num)
                    # mark the number (change to 1)
                    marked_boards[key][row_idx][idx] = 1

                    # check win
                    win = check_bingo_win(marked_boards[key])

                    if win:
                        # append to winner list
                        print(key, marked_boards[key])
                        print(num)
                        winner_list.append(key)

                        # this means if all of the players win
                        if len(winner_list) == len(boards):
                            return key, marked_boards[key], num


def part_2(drawn_numbers, boards):

    last_winner_key, last_winner_marked_boards, last_num = get_last_win(
        boards, drawn_numbers
    )

    total_unmarked_number = 0

    for (
        marked_board,
        board,
    ) in zip(last_winner_marked_boards, boards[last_winner_key]):
        for m, b in zip(marked_board, board):
            if m == 0:
                total_unmarked_number += int(b)

    ans = int(last_num) * int(total_unmarked_number)
    print(ans)


if __name__ == "__main__":
    file = open("./day4.input")
    drawn_numbers, boards = get_boards()
    file.close()

    part_1(drawn_numbers, boards)
    part_2(drawn_numbers, boards)
