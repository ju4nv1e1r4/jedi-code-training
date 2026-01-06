package main

import (
	"fmt"
	"palindromeNumber/solution"
)

func main() {
	testCase1 := solution.IsPalindrome(121)
	testCase2 := solution.IsPalindrome(-121)
	testCase3 := solution.IsPalindrome(10)
	
	fmt.Println("Test Case 1: ", testCase1)
	fmt.Println("Test Case 2: ", testCase2)
	fmt.Println("Test Case 3: ", testCase3)
}