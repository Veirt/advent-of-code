#!/usr/bin/env python3

# A/X: Rock
# B/Y: Paper
# C/Z: Scissor

shape_score = {
    "rock": 1,
    "paper": 2,
    "scissors": 3,
}

result_score = {
    "win": 6,
    "lose": 0,
    "draw": 3,
}


def decrypt(puzzle_input):
    if puzzle_input == "X" or puzzle_input == "A":
        return "rock"

    if puzzle_input == "Y" or puzzle_input == "B":
        return "paper"

    if puzzle_input == "Z" or puzzle_input == "C":
        return "scissors"


def get_score(opponent, player):
    score = shape_score[player]
    win_score = 0

    if player == opponent:
        # draw
        win_score = 3

    elif player == "rock" and opponent == "paper":
        win_score = 0
    elif player == "rock" and opponent == "scissors":
        win_score = 6

    elif player == "paper" and opponent == "rock":
        win_score = 6
    elif player == "paper" and opponent == "scissors":
        win_score = 0

    elif player == "scissors" and opponent == "rock":
        win_score = 0
    elif player == "scissors" and opponent == "paper":
        win_score = 6

    return win_score + score


def part_1():
    with open("./day2.example.txt", "r") as f:
        lines = f.readlines()

        total = 0
        for line in lines:
            opponent, player = line.split()

            inp = decrypt(opponent), decrypt(player)

            result = get_score(*inp)
            total += result

    print(total)  # part 1


def get_score_2(opponent, player):
    table = {
        "rock": {"win": "scissors", "lose": "paper"},
        "paper": {"win": "rock", "lose": "scissors"},
        "scissors": {"win": "paper", "lose": "rock"},
    }

    result = player
    opponent = decrypt(opponent)
    player = decrypt(player)

    assert player
    assert opponent

    if result == "Z":
        return shape_score[table[opponent]["lose"]], result_score["win"]

    elif result == "X":
        return shape_score[table[opponent]["win"]], result_score["lose"]
    else:
        return shape_score[opponent], result_score["draw"]


def part_2():
    with open("./day2.input.txt", "r") as f:
        lines = f.readlines()

        total = 0
        for line in lines:
            opponent, player = line.split()
            temp = get_score_2(opponent, player)
            total += sum(temp)

        print(total)


if __name__ == "__main__":
    # part_1()
    part_2()
