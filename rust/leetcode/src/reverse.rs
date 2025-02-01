pub struct Solution;

impl Solution {
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
