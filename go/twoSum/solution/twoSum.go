package solution

func TwoSum(nums []int, target int) []int {
	hashmap := make(map[int]int)
	for i, n := range nums {
		complement := target - n
		if val, ok := hashmap[complement]; ok {
			return []int{val, i}
		}
		hashmap[n] = i
	}
	return []int{}
}
