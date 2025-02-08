pub struct Solution;

impl Solution {
  pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    fn valid(result: &Vec<i32>, row: usize, col: usize) -> bool {
      for (i, &r) in result.iter().enumerate() {
        if r == col as i32 || (r - col as i32).abs() == (i as i32 - row as i32).abs() {
          return false;
        }
      }
      true
    }

    fn next(result: &mut Vec<i32>, row: usize, n: usize, results: &mut Vec<Vec<String>>) {
      if row == n {
        results.push(
          result
            .iter()
            .map(|&c| ".".repeat(c as usize) + "Q" + &".".repeat(n - c as usize - 1))
            .collect(),
        );
        return;
      }
      for col in 0..n {
        if valid(result, row, col) {
          result.push(col as i32);
          next(result, row + 1, n, results);
          result.pop();
        }
      }
    }
    let mut results = vec![];
    let mut result = vec![];
    next(&mut result, 0 as usize, n as usize, &mut results);
    results
  }
}
