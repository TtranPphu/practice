pub mod my_pow;
fn main() {
    test_my_pow();
}

fn test_my_pow() {
    use crate::my_pow::my_pow::Solution;
    let x = 2.0;
    let n = 10;
    let result = Solution::my_pow(x, n);
    println!("my_pow({}, {}): {}", x, n, result);
}
