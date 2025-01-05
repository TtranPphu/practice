import unittest
from leetcode.two_sum import Solution


class LeetcodeTwoSumTest(unittest.TestCase):
    solution = Solution()

    def test_001(self):
        self.assertListEqual(
            self.solution.twoSum(nums=[2, 7, 11, 15], target=9), [0, 1]
        )

    def test_002(self):
        self.assertListEqual(self.solution.twoSum(nums=[3, 2, 4], target=6), [1, 2])

    def test_003(self):
        self.assertListEqual(self.solution.twoSum(nums=[3, 3], target=6), [0, 1])
