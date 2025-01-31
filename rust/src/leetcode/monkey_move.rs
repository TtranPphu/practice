pub struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn monkey_move(n: i32) -> i32 {
        static M: i64 = 1_000_000_007;
        let mut r = 1_i64;
        let mut x = 2_i64;
        let mut n = n;
        while n > 0 {
            r = if n % 2 == 1 { r * x % M } else { r };
            x = x * x % M;
            n >>= 1;
        }
        r = if r >= 2 { r - 2 } else { r + M - 2 };
        r as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_monkey_move() {
        assert_eq!(Solution::monkey_move(3), 6);
        assert_eq!(Solution::monkey_move(4), 14);
        assert_eq!(Solution::monkey_move(55), 766762394);
        assert_eq!(Solution::monkey_move(500000004), 0);
    }
}
