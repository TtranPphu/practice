import unittest
from leetcode.bulb_switch import Solution


class LeetcodeBulbSwitchTest(unittest.TestCase):
    solution = Solution()

    def test_001(self):
        self.assertEqual(self.solution.bulbSwitch(0), 0)

    def test_002(self):
        self.assertEqual(self.solution.bulbSwitch(1), 1)

    def test_003(self):
        self.assertEqual(self.solution.bulbSwitch(17), 4)

    def test_004(self):
        self.assertEqual(self.solution.bulbSwitch(26), 5)
