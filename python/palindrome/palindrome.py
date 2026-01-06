class Solution(object):
    def isPalindrome_reservedStr(self, x):
        """Using reversed string"""
        x_str = str(x)
        x_str_reverse = x_str[::-1]

        if x < 0:
            return False
        elif x == 0:
            return True
        elif x_str == x_str_reverse:
            return True
        elif x_str != x_str_reverse:
            return False

    def isPalindrome_mathApproach(self, x):
        """Using Math Approach"""
        x_that_will_be_destroyed = abs(x)
        reversed_x = 0

        if x == 0:
            return True
        if x < 0:
            return False

        while x_that_will_be_destroyed > 0:
            digit = x_that_will_be_destroyed % 10
            reversed_x = reversed_x * 10 + digit
            x_that_will_be_destroyed //=10

        if x == reversed_x:
            return True
        else:
            return False
