#!/usr/bin/env python


# might be wrong.
def check_bingo_win(marked_boards):
    for key, value in marked_boards.items():
        board = value

        vertical = [0 for _ in range(5)]
        for row in board:
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
                    win = check_bingo_win(marked_boards)
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


def part_1():
    drawn_numbers, boards = get_boards()

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


# fuck this
# def get_last_win(boards, drawn_numbers):
#     marked_boards = {}
#     winner = {_: False for _ in range(len(boards))}
#     winner_left = len(boards)
#
#     for i in range(len(boards)):
#         # 5 = length of bingo board
#         marked_boards[i] = [[0, 0, 0, 0, 0] for _ in range(5)]
#
#     # oh look at my code
#     # disgusting
#     for num in drawn_numbers:
#         for key, value in boards.items():
#             board = value
#
#             if not winner[key]:
#                 for row_idx, row in enumerate(board):
#                     # if the drawn number is in board row
#                     if num in row:
#                         # get the index
#                         idx = row.index(num)
#                         marked_boards[key][row_idx][idx] = 1
#                         win = check_bingo_win(marked_boards)
#                         if win:
#                             print(marked_boards)
#                             winner[key] = True
#                             winner_left -= 1
#
#                             if winner_left == 0:
#                                 return (key, marked_boards[key], num)
#
#
# def part_2():
#     drawn_numbers, boards = get_boards()
#
#     last_winner_key, marked_boards, last_num = get_last_win(boards, drawn_numbers)
#     # print(last_winner_key, last_num)


if __name__ == "__main__":
    file = open("./day4.input")
    part_1()
    # part_2()
    file.close()
