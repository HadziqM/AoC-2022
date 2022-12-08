package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func input() []int {
	file, _ := os.ReadFile("../input.txt")
	class := strings.Split(string(file), "\n\n")
	var ration []int
	for _, iter := range class {
		total := 0
		for _, j := range strings.Split(iter, "\n") {
			num, _ := strconv.Atoi(j)
			total += num
		}
		ration = append(ration, total)
	}
	return ration
}

func removeList(li []int, i int) []int {
	li[i] = li[len(li)-1]
	return li[:len(li)-1]
}

func highest(in []int) (int, int) {
	highest := in[0]
	index := 0
	for i, j := range in {
		if j > highest {
			highest = j
			index = i
		}
	}
	return index, highest
}

func main() {
	myInput := input()
	_, stage1 := highest(myInput)
	fmt.Println("stage 1 = ", stage1)
	stage2 := 0
	for i := 0; i < 3; i++ {
		hind, val := highest(myInput)
		myInput = removeList(myInput, hind)
		stage2 += val
	}
	fmt.Print("stage 2 = ", stage2)
}
