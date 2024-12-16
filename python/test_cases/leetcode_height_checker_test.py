import unittest
from leetcode.height_checker import Solution


class LeetcodeHeightCheckerTest(unittest.TestCase):
    solution = Solution()

    def test_001(self):
        self.assertEqual(self.solution.heightChecker(heights = [1,1,4,2,1,3]), 3)

    def test_002(self):
        self.assertEqual(self.solution.heightChecker(heights = [5,1,2,3,4]), 5)


