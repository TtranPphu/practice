pub mod my_pow;
pub mod solve_n_queens;
fn main() {
    test_my_pow();
    test_solve_n_queen();
}

fn test_my_pow() {
    use crate::my_pow::my_pow::Solution;
    {
        let x = 2.0;
        let n = 10;
        let result = Solution::my_pow(x, n);
        println!("my_pow({}, {}): {}", x, n, result);
    }
    {
        let x = 2.1;
        let n = 3;
        let result = Solution::my_pow(x, n);
        println!("my_pow({}, {}): {}", x, n, result);
    }
    {
        let x = 2.0;
        let n = -2;
        let result = Solution::my_pow(x, n);
        println!("my_pow({}, {}): {}", x, n, result);
    }
    {
        let x = 2.0;
        let n = 0;
        let result = Solution::my_pow(x, n);
        println!("my_pow({}, {}): {}", x, n, result);
    }
    {
        let x = 2.0;
        let n = 1;
        let result = Solution::my_pow(x, n);
        println!("my_pow({}, {}): {}", x, n, result);
    }
}

fn test_solve_n_queen() {
    use crate::solve_n_queens::solve_n_queens::Solution;
    {
        let n = 4;
        let result = Solution::solve_n_queens(n);
        println!("solve_n_queen({}):", n);
        for i in 0..result.len() {
            println!("{:?}", result[i]);
        }
    }
    {
        let n = 2;
        let result = Solution::solve_n_queens(n);
        println!("solve_n_queen({}):", n);
        for i in 0..result.len() {
            println!("{:?}", result[i]);
        }
    }
    {
        let n = 1;
        let result = Solution::solve_n_queens(n);
        println!("solve_n_queen({}):", n);
        for i in 0..result.len() {
            println!("{:?}", result[i]);
        }
    }
}
