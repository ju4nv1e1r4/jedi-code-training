class Solution:
    def two_sum(self, nums: list, target: int) -> list:
        hashmap = {}
        for i, n in enumerate(nums):
            complement = target - n
            if complement in hashmap:
                return [hashmap[complement], i]
            hashmap[n] = i
        return []