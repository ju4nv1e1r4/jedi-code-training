class Solution:
    def two_sum(self, nums: list, target: int) -> list:
        output = []
        for num in range(len(nums)):
            for num2 in range(len(nums)):
                if num == num2:
                    continue
                if nums[num] + nums[num2] == target:
                    output.append(num)
                    output.append(num2)
                    return output