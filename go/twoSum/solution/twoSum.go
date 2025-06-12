package solution

func TwoSum(nums []int, target int) []int {
	var output []int

	for num := 0; num < len(nums) - 1; num++ {
		for num2 := num +1; num2 < len(nums); num2++ {
			if num == num2 {
				continue
			}

			if nums[num] + nums[num2] == target {
				output = append(output, num, num2)
				return output
			}
		}
	}

	return []int{}
}