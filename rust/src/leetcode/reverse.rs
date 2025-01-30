pub mod reverse {
    pub struct Solution;

    impl Solution {
        #[allow(dead_code)]
        pub fn reverse(x: i32) -> i32 {
            let mut r: i32 = 0;
            let mut x: i32 = x;
            while x != 0 {
                if r > i32::MAX / 10 || r < i32::MIN / 10 {
                    return 0;
                }
                r = r * 10 + x % 10;
                x /= 10;
            }
            r
        }
    }
}

#[cfg(test)]
mod leetcode_test {
    use crate::leetcode::reverse::reverse::Solution;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(1534236469), 0);
        assert_eq!(Solution::reverse(-2147483648), 0);
    }
}
