# Copy/paste template from LeetCode below
from typing import List

class Solution:
    def search(self, nums: List[int], target: int) -> int:
        low = 0
        high = len(nums) - 1
        while low <= high:
            mid = low + ((high - low) // 2)
            if nums[mid] == target:
                return mid
            elif nums[mid] < target:
                low = mid + 1
            else:
                high = mid - 1

        return -1
        

# After copy/pasting the template from LeetCode, uncomment the following to start testing.
solution = Solution()
print(solution.search([-1, 0, 1, 2, 4, 6, 8], 4))

