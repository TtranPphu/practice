pub mod minimize_xor {
    pub struct Solution;

    impl Solution {
        #[allow(dead_code)]
        pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
            let mut r = num1;
            let nsb1 = num1.count_ones();
            let nsb2 = num2.count_ones();
            if nsb1 > nsb2 {
                for _ in nsb2..nsb1 {
                    r &= r - 1;
                }
            } else if nsb1 < nsb2 {
                for _ in nsb1..nsb2 {
                    r |= r + 1;
                }
            }
            r
        }
    }
}

pub mod xor_all_nums {
    pub struct Solution;

    impl Solution {
        #[allow(dead_code)]
        pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
            let mut r = 0;
            let if1odd = nums1.len() % 2 != 0;
            let if2odd = nums2.len() % 2 != 0;
            if if2odd {
                r ^= nums1.into_iter().fold(0_i32, |r, n| r ^ n);
            }
            if if1odd {
                r ^= nums2.into_iter().fold(0_i32, |r, n| r ^ n);
            }
            r
        }
        #[allow(dead_code)]
        pub fn xor_all_nums_v1(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
            let mut r = 0;
            if nums1.len() % 2 != 0 {
                for n in &nums2 {
                    r ^= n;
                }
            }
            if nums2.len() % 2 != 0 {
                for n in &nums1 {
                    r ^= n;
                }
            }
            r
        }
    }
}

pub mod does_valid_array_exist {
    pub struct Solution;

    impl Solution {
        #[allow(dead_code)]
        pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
            return derived.into_iter().fold(0, |acc, n| acc ^ n) == 0;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_minimize_xor() {
        use super::minimize_xor::Solution;
        let cases: Vec<(i32, i32, i32)> = vec![(3, 5, 3), (1, 12, 3), (25, 72, 24)];
        for (num1, num2, expected) in cases {
            let result = Solution::minimize_xor(num1, num2);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_xor_all_nums() {
        use super::xor_all_nums::Solution;
        let cases = vec![
            (vec![2, 1, 3], vec![10, 2, 5, 0], 13),
            (vec![1, 2], vec![3, 4], 0),
            (
                vec![
                    365, 744, 407, 833, 993, 455, 904, 808, 116, 853, 121, 380, 137, 53, 846, 50,
                    338, 460, 630, 276, 509, 48, 530, 440, 975, 434, 556, 875, 795, 317, 749, 164,
                    736, 554, 887, 455, 706, 311, 682, 548, 56, 632, 818, 538, 681, 312, 837, 833,
                    565, 842, 725, 27, 330, 0, 572, 701, 343, 967, 287, 959, 113, 136, 538, 752,
                    454, 22, 805, 421, 281, 906, 119, 51, 152, 632, 848, 158, 19, 997, 184, 447,
                    38, 515, 440, 540, 195, 743, 939, 476, 860, 77, 66,
                ],
                vec![
                    537, 817, 983, 527, 547, 804, 300, 486, 96, 674, 654, 71, 465, 441, 675, 287,
                    749, 38, 501, 967, 292, 460, 763, 611, 105, 27, 215, 658, 328, 37, 864, 581,
                    683, 499, 325, 884, 954, 601, 86, 981, 926, 273, 586, 139, 246, 293, 107, 157,
                    635, 738, 693, 888, 598, 433, 860, 165, 718, 502, 31, 164, 689, 604, 213,
                ],
                772,
            ),
        ];
        for (nums1, nums2, expected) in cases {
            let result = Solution::xor_all_nums(nums1, nums2);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_does_valid_array_exist() {
        use super::does_valid_array_exist::Solution;
        let cases = vec![
            (vec![1, 1, 0], true),
            (vec![1, 1], true),
            (vec![1, 0], false),
        ];
        for (derived, expected) in cases {
            let result = Solution::does_valid_array_exist(derived);
            assert_eq!(result, expected);
        }
    }
}
