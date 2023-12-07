import re

pattern = "(?=(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine))"

letter_digit = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}

result = 0

lines = open("../day01.input.txt").readlines()
for line in lines:
    x = re.findall(pattern, line)

    first_digit = x[0]
    last_digit = x[-1]

    if first_digit in letter_digit:
        first_digit = letter_digit[first_digit]

    if last_digit in letter_digit:
        last_digit = letter_digit[last_digit]

    num = int(f"{first_digit}{last_digit}")
    result += num

print(result)
