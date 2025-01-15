pub mod minimize_xor {
    pub struct Solution;
    impl Solution {
        pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
            let mut result = num1;
            let nsb1 = num1.count_ones();
            let nsb2 = num2.count_ones();
            if nsb1 > nsb2 {
                for _ in nsb2..nsb1 {
                    result &= result - 1;
                }
            } else if nsb1 < nsb2 {
                for _ in nsb1..nsb2 {
                    result |= result + 1;
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod leetcode_test {
    use crate::leetcode::minimize_xor::minimize_xor::Solution;

    #[test]
    fn test_minimize_xor() {
        let cases = vec![(3, 5, 3), (1, 12, 3), (25, 72, 24)];
        for (num1, num2, expected) in cases {
            let result = Solution::minimize_xor(num1, num2);
            assert_eq!(result, expected);
        }
    }
}
