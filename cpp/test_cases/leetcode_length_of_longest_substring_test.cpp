#include <gtest/gtest.h>

#include "length_of_longest_substring.hpp"

namespace {
auto solution = Solution();

TEST(LeetcodeLengthOfLongestSubstringTest, Test001) {
  EXPECT_EQ(solution.lengthOfLongestSubstring("abcabcbb"), 3);
}

TEST(LeetcodeLengthOfLongestSubstringTest, Test002) {
  EXPECT_EQ(solution.lengthOfLongestSubstring("bbbbb"), 1);
}

TEST(LeetcodeLengthOfLongestSubstringTest, Test003) {
  EXPECT_EQ(solution.lengthOfLongestSubstring("pwwkew"), 3);
}
}  // namespace