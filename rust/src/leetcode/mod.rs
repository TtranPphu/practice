pub mod minimize_xor;
pub mod my_pow;
pub mod solve_n_queens;

pub fn leetcode() {
    my_pow::my_pow::Solution::my_pow(2.0, 10);
    solve_n_queens::solve_n_queens::Solution::solve_n_queens(4);
    minimize_xor::minimize_xor::Solution::minimize_xor(1, 2);
}
