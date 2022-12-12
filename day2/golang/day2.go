package main

import (
	"fmt"
	"os"
	"strings"
)

func parse(input string) int {
	switch input {
	case "A":
		return 1
	case "B":
		return 2
	case "C":
		return 3
	case "X":
		return 1
	case "Y":
		return 2
	case "Z":
		return 3
	default:
		return 100000
	}
}
func score(x int, y int) int {
	if x == y {
		return x + 3
	} else if x == y+1 || x == y-2 {
		return x + 6
	} else {
		return x
	}
}
func guess(x int, y int) int {
	if x == 2 {
		return score(y, y)
	} else if x == 1 {
		if y == 1 {
			return score(3, 1)
		} else {
			return score(y-1, y)
		}
	} else {
		if y == 3 {
			return score(1, 3)
		} else {
			return score(y+1, y)
		}
	}
}
func sumofArray(in []int) int {
	total := 0
	for _, val := range in {
		total += val
	}
	return total
}
func stage1() int {
	file, _ := os.ReadFile("../input.txt")
	var out []int
	for _, in := range strings.Split(string(file), "\n") {
		var oout []int
		for _, iin := range strings.Split(in, " ") {
			oout = append(oout, parse(iin))
		}
		out = append(out, score(oout[1], oout[0]))
	}
	return sumofArray(out)
}
func stage2() int {
	file, _ := os.ReadFile("../input.txt")
	var out []int
	for _, in := range strings.Split(string(file), "\n") {
		var oout []int
		for _, iin := range strings.Split(in, " ") {
			oout = append(oout, parse(iin))
		}
		out = append(out, guess(oout[1], oout[0]))
	}
	return sumofArray(out)
}
func main() {
	fmt.Println("stage 1 = ", stage1())
	fmt.Println("stage 2 = ", stage2())
}
