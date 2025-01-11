import unittest
from leetcode.find_median_sorted_arrays import Solution


class FindMedianSortedArraysTest(unittest.TestCase):
    solution = Solution()

    def test_find_median_sorted_arrays(self):
        self.assertEqual(
            self.solution.findMedianSortedArrays(nums1=[1, 3], nums2=[2]), 2
        )

        self.assertEqual(
            self.solution.findMedianSortedArrays(nums1=[1, 2], nums2=[3, 4]), 2.5
        )

        self.assertEqual(
            self.solution.findMedianSortedArrays(nums1=[0, 0], nums2=[0, 0]), 0
        )

        self.assertEqual(self.solution.findMedianSortedArrays(nums1=[], nums2=[1]), 1)

        self.assertEqual(self.solution.findMedianSortedArrays(nums1=[2], nums2=[]), 2)

        self.assertEqual(
            self.solution.findMedianSortedArrays(nums1=[1, 3], nums2=[2, 7]), 2.5
        )
