import unittest
from leetcode.length_of_longest_substring import Solution


class LeetcodeLengthOfLongestSubstringTest(unittest.TestCase):
    solution = Solution()

    def test_001(self):
        self.assertEqual(self.solution.lengthOfLongestSubstring(s="abcabcbb"), 3)

    def test_002(self):
        self.assertEqual(self.solution.lengthOfLongestSubstring(s="bbbbb"), 1)

    def test_003(self):
        self.assertEqual(self.solution.lengthOfLongestSubstring(s="pwwkew"), 3)
