import unittest
from leetcode.bulb_switch import Solution


class BulbSwitchTest(unittest.TestCase):
    solution = Solution()

    def test_bulb_switch(self):
        self.assertEqual(self.solution.bulbSwitch(0), 0)

        self.assertEqual(self.solution.bulbSwitch(1), 1)

        self.assertEqual(self.solution.bulbSwitch(17), 4)

        self.assertEqual(self.solution.bulbSwitch(26), 5)
