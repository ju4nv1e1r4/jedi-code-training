package main

import (
	"fmt"
	"longestCommonPrefix/solution"
)

func main() {
	case1 := []string {"flower", "flow", "flight"}
	case2 := []string {"dog", "racecar", "car"}
	case3 := []string {"ab", "a"}
	
	fmt.Println("Response: ", solution.LongestCommonPrefix(case1))
	fmt.Println("Response: ", solution.LongestCommonPrefix(case2))
	fmt.Println("Response: ", solution.LongestCommonPrefix(case3))
}
