import unittest
from leetcode.length_of_longest_substring import Solution


class LengthOfLongestSubstringTest(unittest.TestCase):
    solution = Solution()

    def test_length_of_longest_substring(self):
        self.assertEqual(self.solution.lengthOfLongestSubstring(s="abcabcbb"), 3)

        self.assertEqual(self.solution.lengthOfLongestSubstring(s="bbbbb"), 1)

        self.assertEqual(self.solution.lengthOfLongestSubstring(s="pwwkew"), 3)

        self.assertEqual(self.solution.lengthOfLongestSubstring(s=""), 0)
