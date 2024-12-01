package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)
import "golang.org/x/exp/constraints"

func Abs[T constraints.Integer](x T) T {
	if x < 0 {
		return -x
	}
	return x
}

func part1(first []int, second []int) {
	sort.Sort(sort.IntSlice(first))
	sort.Sort(sort.IntSlice(second))

	res := 0

	// fmt.Println(first)
	// fmt.Println(second)

	for i := 0; i < len(first); i++ {
		res += Abs(first[i] - second[i])
	}

	fmt.Println(res)
}

func part2(first []int, second []int) {
	counter := map[int]int{}

	// initialize with 0
	for _, num := range first {
		counter[num] = 0
	}

	// count
	for _, num := range second {
		_, ok := counter[num]
		if ok {
			counter[num] += 1
		}
	}

	res := 0
	for _, num := range first {
		res += num * counter[num]
	}

	fmt.Println(res)

}

func main() {
	first := []int{}
	second := []int{}

	contents, _ := os.ReadFile("../day01.input.txt")
	lines := strings.Split(string(contents), "\n")

	for _, line := range lines {
		tmp := strings.Split(string(line), "   ")
		if len(tmp) != 2 {
			continue
		}

		num1, _ := strconv.Atoi(tmp[0])
		num2, _ := strconv.Atoi(tmp[1])

		first = append(first, num1)
		second = append(second, num2)
	}

	part1(first, second)
	part2(first, second)

}
