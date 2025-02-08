pub struct Solution;
impl Solution {
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
