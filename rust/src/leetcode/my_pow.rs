
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n < 0 {
            return Solution::my_pow(1.0 / x, -n);
        }
        match x {
            0.0 => 0.0,
            1.0 => 1.0,
            _ => {
                let mut result = 1.0;
                let mut x = x;
                let mut n = n;
                while n != 0 {
                    if n % 2 != 0 {
                        result *= x;
                    }
                    x *= x;
                    n >>= 1;
                }
                result
            }
        }
    }
}

#[cfg(test)]
mod leetcode_test {
    use super::Solution;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_my_pow() {
        let cases = vec![
            (2.0, 10, 1024.0),
            (2.1, 3, 9.261),
            (2.0, -2, 0.25),
            (2.0, 0, 1.0),
            (2.0, 1, 2.0),
        ];
        for (x, n, expected) in cases {
            let result = Solution::my_pow(x, n);
            assert_approx_eq!(result, expected);
        }
    }
}
