import unittest
from leetcode.can_jump import Solution


class CanJumpTest(unittest.TestCase):
    solution = Solution()

    def test_can_jump(self):
        self.assertEqual(
            self.solution.canJump(nums=[2, 3, 1, 1, 4]),
            True,
        )

        self.assertEqual(
            self.solution.canJump(nums=[3, 2, 1, 0, 4]),
            False,
        )

        self.assertEqual(
            self.solution.canJump(nums=[1, 2, 3]),
            True,
        )

        self.assertEqual(
            self.solution.canJump(nums=[2, 5, 0, 0]),
            True,
        )
