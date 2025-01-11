import unittest
from leetcode.height_checker import Solution


class HeightCheckerTest(unittest.TestCase):
    solution = Solution()

    def test_height_checker(self):
        self.assertEqual(self.solution.heightChecker(heights = [1,1,4,2,1,3]), 3)

        self.assertEqual(self.solution.heightChecker(heights = [5,1,2,3,4]), 5)

        self.assertEqual(self.solution.heightChecker(heights = [1,2,3,4,5]), 0)


