import unittest
from leetcode.min_difference import Solution


class MinDifferenceTest(unittest.TestCase):
    solution = Solution()

    def test_min_difference(self):
        self.assertEqual(self.solution.minDifference(nums=[5, 3, 2, 4]), 0)

        self.assertEqual(self.solution.minDifference(nums=[1, 5, 0, 10, 14]), 1)

        self.assertEqual(self.solution.minDifference(nums=[3, 100, 20]), 0)

        self.assertEqual(self.solution.minDifference(nums=[6, 6, 0, 1, 1, 4, 6]), 2)
