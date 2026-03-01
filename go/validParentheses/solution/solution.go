package solution

func IsValid(s string) bool {
	mapping := map[string]string{
		")": "(",
		"}": "{",
		"]": "[",
	}
	stack := []string{}
	for _, char := range s {
		if char == '(' || char == '{' || char == '[' {
			stack = append(stack, string(char))
		} else {
			if len(stack) == 0 || stack[len(stack)-1] != mapping[string(char)] {
				return false
			}
			stack = stack[:len(stack)-1]
		}
	}
	if len(stack) == 0 {
		return true
	} else {
		return false
	}
}
