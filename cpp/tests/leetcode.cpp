#include <gtest/gtest.h>

#include "can_be_valid.hpp"
#include "find_median_sorted_arrays.hpp"
#include "length_of_longest_substring.hpp"
#include "max_area.hpp"
#include "solve_n_queens.hpp"

namespace {

TEST(LeetcodeTest, CanBeValidTest) {
  auto solution = cbv::Solution();
  {
    string s = "))()))";
    string locked = "010100";
    EXPECT_TRUE(solution.canBeValid(s, locked));
  }
  {
    string s = "()()";
    string locked = "0000";
    EXPECT_TRUE(solution.canBeValid(s, locked));
  }
  {
    string s = ")";
    string locked = "0";
    EXPECT_FALSE(solution.canBeValid(s, locked));
  }
  {
    string s = "())()))()(()(((())(()()))))((((()())(())";
    string locked = "1011101100010001001011000000110010100101";
    EXPECT_TRUE(solution.canBeValid(s, locked));
  }
  {
    string s = "())(()(()(())()())(())((())(()())((())))))(((((((())(()))))(";
    string locked =
        "100011110110011011010111100111011101111110000101001101001111";
    EXPECT_FALSE(solution.canBeValid(s, locked));
  }
}

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

TEST(LeetcodeTest, SolveNQueensTest) {
  auto solution = solve_n_queens::Solution();
  {
    vector<vector<string>> results = solution.solveNQueens(4);
    vector<vector<string>> expected{{".Q..", "...Q", "Q...", "..Q."},
                                    {"..Q.", "Q...", "...Q", ".Q.."}};
    EXPECT_EQ(results, expected);
  }
  {
    vector<vector<string>> results = solution.solveNQueens(2);
    vector<vector<string>> expected{};
    EXPECT_EQ(results, expected);
  }
  {
    vector<vector<string>> results = solution.solveNQueens(1);
    vector<vector<string>> expected{{"Q"}};
    EXPECT_EQ(results, expected);
  }
}

}  // namespace
