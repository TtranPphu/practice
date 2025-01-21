pub mod cnds {
    pub struct Solution;

    impl Solution {
        #[allow(dead_code)]
        pub fn count_non_decreasing_subarrays(_nums: Vec<i32>, _k: i32) -> i64 {
            0
        }
    }
}

#[cfg(test)]
mod leetcode_test {
    use crate::leetcode::count_non_decreasing_subarrays::cnds::Solution;

    #[test]
    fn test_count_non_decreasing_subarrays() {
        let cases = vec![
            (vec![6, 3, 1, 2, 4, 4], 7, 17),
            (vec![6, 3, 1, 3, 6], 4, 12),
        ];
        for (nums, k, expected) in cases {
            let result = Solution::count_non_decreasing_subarrays(nums, k);
            assert_eq!(result, expected);
        }
    }
}
