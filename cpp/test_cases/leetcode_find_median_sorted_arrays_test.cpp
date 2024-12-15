#include <gtest/gtest.h>

#include "find_median_sorted_arrays.hpp"

namespace {
auto solution = Solution();

TEST(LeetcodeFindMedianSortedArraysTest, Test001) {
  vector<int> nums1{1, 3};
  vector<int> nums2{2};
  EXPECT_EQ(solution.findMedianSortedArrays(nums1, nums2), 2);
}

TEST(LeetcodeFindMedianSortedArraysTest, Test002) {
  vector<int> nums1{1, 4};
  vector<int> nums2{2, 3};
  EXPECT_EQ(solution.findMedianSortedArrays(nums1, nums2), 2.5);
}
}  // namespace
