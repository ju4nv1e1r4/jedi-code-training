package main

import (
	"fmt"
	"validParentheses/solution"
)

func main() {
	case1 := solution.IsValid("()")
	case2 := solution.IsValid("()[]{}")
	case3 := solution.IsValid("(]")
	case4 := solution.IsValid("([])")
	case5 := solution.IsValid("([)]")
	case6 := solution.IsValid("(")

	fmt.Println(case1)
	fmt.Println(case2)
	fmt.Println(case3)
	fmt.Println(case4)
	fmt.Println(case5)
	fmt.Println(case6)
}