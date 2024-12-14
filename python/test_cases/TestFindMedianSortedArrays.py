import unittest
from leetcode.FindMedianSortedArrays import Solution


class TestFindMedianSortedArrays(unittest.TestCase):
    solution = Solution()

    def test_ex1(self):
        self.assertEqual(
            self.solution.findMedianSortedArrays(nums1=[1, 3], nums2=[2]), 2
        )

    def test_ex2(self):
        self.assertEqual(
            self.solution.findMedianSortedArrays(nums1=[1, 2], nums2=[3, 4]), 2.5
        )
