import unittest
from leetcode.LengthOfLongestSubstring import Solution


class TestLengthOfLongestSubstring(unittest.TestCase):
    solution = Solution()

    def test_ex1(self):
        self.assertEqual(self.solution.lengthOfLongestSubstring(s="abcabcbb"), 3)

    def test_ex2(self):
        self.assertEqual(self.solution.lengthOfLongestSubstring(s="bbbbb"), 1)

    def test_ex3(self):
        self.assertEqual(self.solution.lengthOfLongestSubstring(s="pwwkew"), 3)
