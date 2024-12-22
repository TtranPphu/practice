import unittest
from leetcode.min_difference import Solution


class LeetcodeMinDifferenceTest(unittest.TestCase):
    solution = Solution()

    def test_001(self):
        self.assertEqual(self.solution.minDifference(nums=[5, 3, 2, 4]), 0)

    def test_002(self):
        self.assertEqual(self.solution.minDifference(nums=[1, 5, 0, 10, 14]), 1)

    def test_003(self):
        self.assertEqual(self.solution.minDifference(nums = [3,100,20]), 0)

    def test_004(self):
        self.assertEqual(self.solution.minDifference(nums = [6,6,0,1,1,4,6]), 2)
