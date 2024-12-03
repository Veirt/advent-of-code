package main

import (
	"os"
	"strconv"
	"strings"

	"golang.org/x/exp/constraints"
)

func Abs[T constraints.Integer](x T) T {
	if x < 0 {
		return -x
	}
	return x
}

func remove(slice []string, s int) []string {
	result := make([]string, 0, len(slice)-1)
	result = append(result, slice[:s]...)
	return append(result, slice[s+1:]...)

}

func loadFile(name string) string {
	contents, _ := os.ReadFile(name)
	return string(contents)
}

func isSafe(nums []string) bool {
	// increasing or decreasing
	status := ""
	for i, num := range nums {
		if i == 0 {
			continue
		}
		num, _ := strconv.Atoi(num)
		numBefore, _ := strconv.Atoi(nums[i-1])

		diff := num - numBefore
		var statusNow string
		if diff < 0 {
			statusNow = "decreasing"
		} else {
			statusNow = "increasing"
		}

		if status != "" && status != statusNow {
			return false
		}

		if Abs(diff) > 3 || diff == 0 {
			return false
		}

		status = statusNow

	}

	return true
}

func part1(input string) int {
	lines := strings.Split(string(input), "\n")
	lines = lines[:len(lines)-1]
	count := 0
	for _, line := range lines {
		nums := strings.Split(line, " ")
		safe := isSafe(nums)
		if safe {
			count++
		}
	}

	return count
}

// bruteforce method
func part2(input string) int {
	lines := strings.Split(string(input), "\n")
	lines = lines[:len(lines)-1]
	count := 0
	for _, line := range lines {
		nums := strings.Split(line, " ")
		if isSafe(nums) {
			count++
			continue
		}

		// bruteforce removing one by one and check it safe or not
		for i := range nums {
			tmp := remove(nums, i)
			if isSafe(tmp) {
				count++
				break
			}
		}
	}

	return count
}
