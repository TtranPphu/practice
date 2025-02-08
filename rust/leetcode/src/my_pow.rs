pub struct Solution;

impl Solution {
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
