pub struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut checks = vec![false; grid.len() * grid.len()];
        let mut repeated = 0;
        for i in grid.iter().flatten() {
            if checks[*i as usize - 1] {
                repeated = *i;
            } else {
                checks[*i as usize - 1] = true;
            }
        }
        let missing = checks.iter().position(|&x| !x).unwrap_or_default() as i32 + 1;
        vec![repeated, missing]
    }
}
