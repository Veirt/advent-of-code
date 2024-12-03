package main

import (
	"fmt"
	"testing"
)

func TestPart1Example(t *testing.T) {
	input := loadFile("../day02.example.txt")
	res := part1(input)
	ans := 2

	if res != ans {
		t.Fatalf("Wrong")
	}
}

func TestPart1Input(t *testing.T) {
	input := loadFile("../day02.input.txt")
	res := part1(input)
	fmt.Println(res)
}

func TestPart2Example(t *testing.T) {
	input := loadFile("../day02.example.txt")
	res := part2(input)
	ans := 4
	if res != ans {
		t.Fatalf("Got %d, expected %d", res, ans)
	}
}

func TestPart2Input(t *testing.T) {
	input := loadFile("../day02.input.txt")
	res := part2(input)
	fmt.Println(res)
}
