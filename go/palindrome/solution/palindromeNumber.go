package solution

// Check if a integer number is a palindrome.
func IsPalindrome(x int) bool {
	xThatWillBeDestroyed := x
	reversedX := 0
	if x < 0 {
		return false
	}
	if x == 0 {
		return true
	}

	for int(xThatWillBeDestroyed) > 0 {
		digit := xThatWillBeDestroyed % 10
		reversedX = reversedX * 10 + digit
		xThatWillBeDestroyed = xThatWillBeDestroyed / 10
	}

	return reversedX == x
}
