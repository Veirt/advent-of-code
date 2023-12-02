set shell := ["bash", "-uc"]
set dotenv-load := true

year := `date +"%Y"`
AOC_COOKIE := env_var('AOC_COOKIE')

setup day:
    @NAME=$(printf "day%02d" {{day}}) && \\
        curl --cookie "session={{AOC_COOKIE}}" -o "$NAME/$NAME.input.txt" https://adventofcode.com/{{year}}/day/{{day}}/input
