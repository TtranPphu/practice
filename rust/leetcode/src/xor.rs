pub mod minimize_xor {
    pub struct Solution;

    impl Solution {
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
        pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
            return derived.into_iter().fold(0, |acc, n| acc ^ n) == 0;
        }
    }
}
