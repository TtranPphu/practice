import unittest
from leetcode.is_long_pressed_name import Solution


class LeetcodeIsLongPressedNameTest(unittest.TestCase):
    solution = Solution()

    def test_001(self):
        self.assertEqual(
            self.solution.isLongPressedName(name="alex", typed="aaleex"), True
        )

    def test_002(self):
        self.assertEqual(
            self.solution.isLongPressedName(name="saeed", typed="ssaaedd"), False
        )

    def test_003(self):
        self.assertEqual(
            self.solution.isLongPressedName(name="alex", typed="aaleexa"), False
        )

    def test_004(self):
        self.assertEqual(
            self.solution.isLongPressedName(name="alexd", typed="ale"), False
        )

    def test_005(self):
        self.assertEqual(
            self.solution.isLongPressedName(name="abcd", typed="aaabbbcccddd"), True
        )

    def test_006(self):
        self.assertEqual(
            self.solution.isLongPressedName(
                name="zlexya", typed="aazlllllllllllllleexxxxxxxxxxxxxxxya"
            ),
            False,
        )
