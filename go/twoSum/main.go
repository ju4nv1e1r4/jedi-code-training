package main

import "twoSum/solution"

func main() {
	testCase1 := solution.TwoSum([]int{2, 7, 11, 15}, 9)
	testCase2 := solution.TwoSum([]int{3, 2, 4}, 6)
	testCase3 := solution.TwoSum([]int{3, 3}, 6)
	println(testCase1)
	println(testCase2)
	println(testCase3)
}