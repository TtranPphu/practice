pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = std::collections::HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            let diff = target - v;
            if let Some(&j) = m.get(&diff) {
                return vec![j as i32, i as i32];
            }
            m.insert(v, i);
        }
        vec![]
    }
}
