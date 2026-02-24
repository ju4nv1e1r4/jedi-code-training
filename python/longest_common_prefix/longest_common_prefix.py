class Solution(object):
    def longest_common_prefix(self, strs: list) -> str:
        for i in range(len(strs[0])):
            for j in range(1, len(strs)):
                if i == len(strs[j]) or strs[0][i] != strs[j][i]:
                    return strs[0][:i]
        return strs[0]
    
    def longest_common_prefix_v2(self, strs: list):
        zip_list = zip(*strs)
        for i, chars in enumerate(zip_list):
            if len(set(chars)) > 1:
                return strs[0][:i]
        return min(strs, key=len)
