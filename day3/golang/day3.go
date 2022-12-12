package main

import (
	"fmt"
	"os"
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
		for _, j := range i[:len(i)/2] {
			if contain(i[len(i)/2:], j) {
				out += getValue(j)
				break
			}
		}
	}
	fmt.Println("stage 1 = ", out)
}

func stage2() {
	file, _ := os.ReadFile("../input.txt")
	out := 0
	hold := []string{}
	for i, j := range strings.Split(string(file), "\n") {
		hold = append(hold, j)
		if (i+1)%3 == 0 {
			for _, l := range hold[0] {
				if contain(hold[1], l) {
					if contain(hold[2], l) {
						out += getValue(l)
						hold = nil
						break
					}
				}
			}
		}
	}
	fmt.Println("stage 2 =", out)
}
func contain(in string, val rune) bool {
	for _, i := range in {
		if i == val {
			return true
		}
	}
	return false
}
func getValue(input rune) int {
	alphabet := []rune{}
	for i := 'a'; i <= 'z'; i++ {
		alphabet = append(alphabet, rune(i))
	}
	for i := 'A'; i <= 'Z'; i++ {
		alphabet = append(alphabet, rune(i))
	}
	return indexOf(alphabet, input) + 1
}
func indexOf(input []rune, val rune) int {
	for i, j := range input {
		if j == val {
			return i
		}
	}
	return -1
}
