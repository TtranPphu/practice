import unittest
from leetcode.can_jump import Solution


class LeetcodeCanJumpTest(unittest.TestCase):
    solution = Solution()

    def test_001(self):
        self.assertEqual(
            self.solution.canJump(nums=[2, 3, 1, 1, 4]),
            True,
        )

    def test_002(self):
        self.assertEqual(
            self.solution.canJump(nums=[3, 2, 1, 0, 4]),
            False,
        )

    def test_003(self):
        self.assertEqual(
            self.solution.canJump(nums=[1, 2, 3]),
            True,
        )

    def test_004(self):
        self.assertEqual(
            self.solution.canJump(nums=[2, 5, 0, 0]),
            True,
        )
