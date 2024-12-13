import unittest
from utils.log_util import local_logger


class Solution:
    def findMedianSortedArrays(self, nums1: list[int], nums2: list[int]) -> float:
        n1 = len(nums1)
        n2 = len(nums2)
        if n1 > n2:
            return self.findMedianSortedArrays(nums2, nums1)
        n = n1 + n2
        mli = n // 2 - (n - 1) % 2
        mri = n // 2
        i1 = -1
        i2 = -1
        lv = 0.0
        rv = 0.0
        for i in range(mri + 1):
            if i1 == n1-1:
                i2 += 1
                if i == mli:
                    lv = nums2[i2]
                if i == mri:
                    rv = nums2[i2]
            elif i2 == n2-1:
                i1 += 1
                if i == mli:
                    lv = nums1[i1]
                if i == mri:
                    rv = nums1[i1]
            elif nums1[i1 + 1] < nums2[i2 + 1]:
                i1 += 1
                if i == mli:
                    lv = nums1[i1]
                if i == mri:
                    rv = nums1[i1]
            else:
                i2 += 1
                if i == mli:
                    lv = nums2[i2]
                if i == mri:
                    rv = nums2[i2]
        return (lv + rv) / 2


class TestSolution(unittest.TestCase):
    solution = Solution()

    def test_ex1(self):
        self.assertEqual(
            self.solution.findMedianSortedArrays(nums1=[1, 3], nums2=[2]), 2
        )

    def test_ex2(self):
        self.assertEqual(
            self.solution.findMedianSortedArrays(nums1=[1, 2], nums2=[3, 4]), 2.5
        )


if __name__ == "__main__":
    unittest.main()
