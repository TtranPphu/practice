import unittest
from leetcode.is_long_pressed_name import Solution


class IsLongPressedNameTest(unittest.TestCase):
    solution = Solution()

    def test_is_long_pressed_name(self):
        self.assertEqual(
            self.solution.isLongPressedName(name="alex", typed="aaleex"), True
        )

        self.assertEqual(
            self.solution.isLongPressedName(name="saeed", typed="ssaaedd"), False
        )

        self.assertEqual(
            self.solution.isLongPressedName(name="alex", typed="aaleexa"), False
        )

        self.assertEqual(
            self.solution.isLongPressedName(name="alexd", typed="ale"), False
        )

        self.assertEqual(
            self.solution.isLongPressedName(name="abcd", typed="aaabbbcccddd"), True
        )

        self.assertEqual(
            self.solution.isLongPressedName(
                name="zlexya", typed="aazlllllllllllllleexxxxxxxxxxxxxxxya"
            ),
            False,
        )

        self.assertEqual(
            self.solution.isLongPressedName(name="vtkgn", typed="vttkgnn"), True
        )
