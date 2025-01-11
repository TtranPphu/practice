#include <gtest/gtest.h>

#include "find_median_sorted_arrays.hpp"
#include "length_of_longest_substring.hpp"
#include "max_area.hpp"

namespace {

TEST(LeetcodeTest, FindMedianSortedArraysTest) {
  auto solution = fmsa::Solution();
  {
    vector<int> nums1{1, 3};
    vector<int> nums2{2};
    EXPECT_EQ(solution.findMedianSortedArrays(nums1, nums2), 2);
  }
  {
    vector<int> nums1{1, 4};
    vector<int> nums2{2, 3};
    EXPECT_EQ(solution.findMedianSortedArrays(nums1, nums2), 2.5);
  }
}

TEST(LeetcodeTest, LengthOfLongestSubstringTest) {
  auto solution = lols::Solution();
  EXPECT_EQ(solution.lengthOfLongestSubstring("abcabcbb"), 3);
  EXPECT_EQ(solution.lengthOfLongestSubstring("bbbbb"), 1);
  EXPECT_EQ(solution.lengthOfLongestSubstring("pwwkew"), 3);
}

TEST(LeetcodeTest, MaxAreaTest) {
  auto solution = max_area::Solution();
  {
    vector<int> height{1, 8, 6, 2, 5, 4, 8, 3, 7};
    EXPECT_EQ(solution.maxArea(height), 49);
  }
  {
    vector<int> height{1, 1};
    EXPECT_EQ(solution.maxArea(height), 1);
  }
}

}  // namespace
