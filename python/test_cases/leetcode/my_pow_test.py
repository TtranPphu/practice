import unittest
from leetcode.my_pow import Solution


class MyPowTest(unittest.TestCase):
    solution = Solution()

    def test_my_pow(self):
        self.assertAlmostEqual(self.solution.myPow(x=2.00000, n=10), 1024.00000)

        self.assertAlmostEqual(self.solution.myPow(x=2.10000, n=3), 9.26100)

        self.assertAlmostEqual(self.solution.myPow(x=2.00000, n=-2), 0.25000)

        self.assertAlmostEqual(self.solution.myPow(x=2.00000, n=0), 1)

        self.assertAlmostEqual(self.solution.myPow(x=2.00000, n=1), 2)
