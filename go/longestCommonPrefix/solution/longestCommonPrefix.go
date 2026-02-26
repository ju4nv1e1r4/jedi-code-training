package solution

func LongestCommonPrefix(strs []string) string {
	for i := range(len(strs[0])) {
		for j := range(len(strs)) {
			if i == len(strs[j]) || strs[0][i] != strs[j][i] {
				return strs[0][:i]
			}
		}

	}

	return strs[0]
}
