import unittest
from leetcode.valid_square import Solution


class ValidSquareTest(unittest.TestCase):
    solution = Solution()

    def test_valid_square(self):
        self.assertEqual(
            self.solution.validSquare(p1=[0, 0], p2=[1, 1], p3=[1, 0], p4=[0, 1]),
            True,
        )

        self.assertEqual(
            self.solution.validSquare(p1=[0, 0], p2=[1, 1], p3=[1, 0], p4=[0, 12]),
            False,
        )

        self.assertEqual(
            self.solution.validSquare(p1=[1, 0], p2=[-1, 0], p3=[0, 1], p4=[0, -1]),
            True,
        )

        self.assertEqual(
            self.solution.validSquare(p1=[0, 0], p2=[0, 0], p3=[0, 0], p4=[0, 0]),
            False,
        )

        self.assertEqual(
            self.solution.validSquare(p1=[2, 1], p2=[1, 2], p3=[0, 0], p4=[2, 0]),
            False,
        )

        self.assertEqual(
            self.solution.validSquare(p1=[0, 0], p2=[1, 1], p3=[1, 0], p4=[1, 1]),
            False,
        )
