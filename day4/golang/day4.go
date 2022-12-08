package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	stage1()
	stage2()
}
func stage1() {
	file, _ := os.ReadFile("../input.txt")
	out := 0
	for _, i := range strings.Split(string(file), "\n") {
		var hold []int
		for _, j := range strings.Split(i, ",") {
			val := strings.Split(j, "-")
			first, _ := strconv.Atoi(val[0])
			last, _ := strconv.Atoi(val[1])
			hold = append(hold, first)
			hold = append(hold, last)
		}
		if fullyHold(hold) {
			out += 1
		}
	}
	fmt.Println("stage 1 =", out)
}
func stage2() {
	file, _ := os.ReadFile("../input.txt")
	out := 0
	for _, i := range strings.Split(string(file), "\n") {
		var hold []int
		for _, j := range strings.Split(i, ",") {
			val := strings.Split(j, "-")
			first, _ := strconv.Atoi(val[0])
			last, _ := strconv.Atoi(val[1])
			hold = append(hold, first)
			hold = append(hold, last)
		}
		if overlaping(hold) {
			out += 1
		}
	}
	fmt.Println("stage 2 =", out)
}
func fullyHold(in []int) bool {
	first, second := toRange(in)
	if contain(first, in[2]) && contain(first, in[3]) || contain(second, in[0]) && contain(second, in[1]) {
		return true
	}
	return false
}
func overlaping(in []int) bool {
	first, second := toRange(in)
	for _, i := range first {
		if contain(second, i) {
			return true
		}
	}
	return false
}
func toRange(in []int) ([]int, []int) {
	var first []int
	var second []int
	for i := in[0]; i <= in[1]; i++ {
		first = append(first, i)
	}
	for i := in[2]; i <= in[3]; i++ {
		second = append(second, i)
	}
	return first, second
}
func contain(in []int, val int) bool {
	for _, i := range in {
		if i == val {
			return true
		}
	}
	return false
}
