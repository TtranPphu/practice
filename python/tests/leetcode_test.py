from leetcode import *
from math import isclose
from functools import wraps


def notatest(obj):
    obj.__test__ = False
    return obj


@notatest
def test_equal(cases, solution):
    for case in cases:
        expect = case.pop("expect")
        assert solution(**case) == expect


@notatest
def test_almost_equal(cases, solution):
    for case in cases:
        expect = case.pop("expect")
        assert isclose(solution(**case), expect)


@notatest
def test_list_equal(cases, solution):
    for case in cases:
        expect = case.pop("expect")
        assert all(x == y for x, y in zip(solution(**case), expect))


def test_bulb_switch():
    test_equal(
        cases=[
            {"n": 0, "expect": 0},
            {"n": 1, "expect": 1},
            {"n": 17, "expect": 4},
            {"n": 26, "expect": 5},
        ],
        solution=BulbSwitch().bulbSwitch,
    )


def test_can_jump():
    test_equal(
        cases=[
            {"nums": [2, 3, 1, 1, 4], "expect": True},
            {"nums": [3, 2, 1, 0, 4], "expect": False},
            {"nums": [1, 2, 3], "expect": True},
            {"nums": [2, 5, 0, 0], "expect": True},
        ],
        solution=CanJump().canJump,
    )


def test_find_median_sorted_arrays():
    test_almost_equal(
        cases=[
            {"nums1": [1, 3], "nums2": [2], "expect": 2},
            {"nums1": [1, 2], "nums2": [3, 4], "expect": 2.5},
            {"nums1": [0, 0], "nums2": [0, 0], "expect": 0},
            {"nums1": [], "nums2": [1], "expect": 1},
            {"nums1": [2], "nums2": [], "expect": 2},
            {"nums1": [1, 3], "nums2": [2, 7], "expect": 2.5},
        ],
        solution=FindMedianSortedArrays().findMedianSortedArrays,
    )


def test_height_checker():
    height_checker = HeightChecker()
    assert height_checker.heightChecker(heights=[1, 1, 4, 2, 1, 3]) == 3
    assert height_checker.heightChecker(heights=[5, 1, 2, 3, 4]) == 5
    assert height_checker.heightChecker(heights=[1, 2, 3, 4, 5]) == 0


def test_is_long_pressed_name():
    test_equal(
        cases=[
            {"name": "alex", "typed": "aaleex", "expect": True},
            {"name": "saeed", "typed": "ssaaedd", "expect": False},
            {"name": "alex", "typed": "aaleexa", "expect": False},
            {"name": "alexd", "typed": "ale", "expect": False},
            {"name": "abcd", "typed": "aaabbbcccddd", "expect": True},
            {"name": "zlexya", "typed": "aazlllllleexxxxxxxxya", "expect": False},
            {"name": "vtkgn", "typed": "vttkgnn", "expect": True},
        ],
        solution=IsLongPressedName().isLongPressedName,
    )


def test_length_of_longest_substring():
    lols = LengthOfLongestSubstring()
    assert lols.lengthOfLongestSubstring(s="abcabcbb") == 3
    assert lols.lengthOfLongestSubstring(s="bbbbb") == 1
    assert lols.lengthOfLongestSubstring(s="pwwkew") == 3
    assert lols.lengthOfLongestSubstring(s="") == 0


def test_min_difference():
    min_difference = MinDifference()
    assert min_difference.minDifference(nums=[5, 3, 2, 4]) == 0
    assert min_difference.minDifference(nums=[1, 5, 0, 10, 14]) == 1
    assert min_difference.minDifference(nums=[3, 100, 20]) == 0
    assert min_difference.minDifference(nums=[6, 6, 0, 1, 1, 4, 6]) == 2


def test_my_pow():
    my_pow = MyPow()
    assert isclose(my_pow.myPow(x=2.00000, n=10), 1024.00000)
    assert isclose(my_pow.myPow(x=2.10000, n=3), 9.26100)
    assert isclose(my_pow.myPow(x=2.00000, n=-2), 0.25000)
    assert isclose(my_pow.myPow(x=2.00000, n=0), 1)
    assert isclose(my_pow.myPow(x=2.00000, n=1), 2)


def test_two_sum():
    solution = TwoSum().twoSum
    cases = [
        {"nums": [2, 7, 11, 15], "target": 9, "expect": [0, 1]},
        {"nums": [3, 2, 4], "target": 6, "expect": [1, 2]},
        {"nums": [3, 3], "target": 6, "expect": [0, 1]},
    ]
    test_list_equal(cases=cases, solution=solution)
