import unittest
from leetcode.two_sum import Solution


class TwoSumTest(unittest.TestCase):
    solution = Solution()

    def test_two_sum(self):
        self.assertListEqual(
            self.solution.twoSum(nums=[2, 7, 11, 15], target=9), [0, 1]
        )

        self.assertListEqual(self.solution.twoSum(nums=[3, 2, 4], target=6), [1, 2])

        self.assertListEqual(self.solution.twoSum(nums=[3, 3], target=6), [0, 1])
