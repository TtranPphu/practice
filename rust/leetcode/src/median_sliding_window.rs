use collections::btree_multiset::BTreeMultiSet;
pub struct Solution;

impl Solution {
  pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    fn rebalance(low: &mut BTreeMultiSet<i32>, high: &mut BTreeMultiSet<i32>) {
      while low.len() > high.len() + 1 {
        let max = low.pop_last().unwrap();
        high.insert(max);
      }
      while high.len() > low.len() {
        let min = high.pop_first().unwrap();
        low.insert(min);
      }
    }

    fn insert(low: &mut BTreeMultiSet<i32>, high: &mut BTreeMultiSet<i32>, value: i32) {
      if low.len() == 0 || value <= *low.last().unwrap() {
        low.insert(value);
      } else {
        high.insert(value);
      }
      rebalance(low, high);
    }

    fn remove(low: &mut BTreeMultiSet<i32>, high: &mut BTreeMultiSet<i32>, value: i32) {
      if low.remove(&value) {
        rebalance(low, high);
      } else if high.remove(&value) {
        rebalance(low, high);
      }
    }

    fn median(low: &BTreeMultiSet<i32>, high: &BTreeMultiSet<i32>) -> f64 {
      if low.len() == high.len() {
        (*low.last().unwrap() as f64 + *high.first().unwrap() as f64) / 2.0
      } else {
        *low.last().unwrap() as f64
      }
    }

    let mut r = vec![];
    let mut low: BTreeMultiSet<i32> = BTreeMultiSet::new();
    let mut high: BTreeMultiSet<i32> = BTreeMultiSet::new();

    for i in 0..k as usize {
      insert(&mut low, &mut high, nums[i]);
    }

    for i in k as usize..nums.len() {
      r.push(median(&low, &high));
      remove(&mut low, &mut high, nums[i - k as usize]);
      insert(&mut low, &mut high, nums[i]);
    }
    r.push(median(&low, &high));

    r
  }
}
