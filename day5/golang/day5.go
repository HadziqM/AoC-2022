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
	inst := instruction()
	crate := inputCrate()
	for _, i := range inst {
		for j := 0; j < i[0]; j++ {
			length := len(crate[i[1]-1])
			crate[i[2]-1] = append(crate[i[2]-1], crate[i[1]-1][length-1])
			crate[i[1]-1] = crate[i[1]-1][:length-1]
		}
	}
	fmt.Println("stage 1 =", lastAlp(crate))
}
func stage2() {
	inst := instruction()
	crate := inputCrate()
	for _, i := range inst {
		length := len(crate[i[1]-1])
		for j := i[0]; j > 0; j-- {
			crate[i[2]-1] = append(crate[i[2]-1], crate[i[1]-1][length-j])
		}
		crate[i[1]-1] = crate[i[1]-1][:length-i[0]]
	}
	fmt.Println("stage 2 =", lastAlp(crate))
}
func lastAlp(in [][]rune) string {
	var out []rune
	for _, i := range in {
		out = append(out, i[len(i)-1])
	}
	return string(out)
}

func inputCrate() [][]rune {
	var input [][]rune
	input = append(input, reverse("FLMW"))
	input = append(input, reverse("FMVZB"))
	input = append(input, reverse("QLSRVH"))
	input = append(input, reverse("JTMPQVSF"))
	input = append(input, reverse("WSL"))
	input = append(input, reverse("WJRMPVF"))
	input = append(input, reverse("FRNPCQJ"))
	input = append(input, reverse("BRWZSPHV"))
	input = append(input, reverse("WZHGCJMB"))
	return input
}
func instruction() [][]int {
	var input [][]int
	file, _ := os.ReadFile("../input.txt")
	inst := strings.Split(string(file), "\n\n")
	for _, i := range strings.Split(inst[1], "\n") {
		var hold []int
		for _, j := range strings.Split(i, " ") {
			integer, err := strconv.Atoi(j)
			if err == nil {
				hold = append(hold, integer)
			}
		}
		input = append(input, hold)
	}
	return input
}
func reverse(in string) []rune {
	var result string
	for _, i := range in {
		result = string(i) + result
	}
	return []rune(result)
}
