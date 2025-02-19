use multi_containers::BTreeMultiSet;

pub struct Solution;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        type BTMSi32 = BTreeMultiSet<i32>;
        fn rebalance(low: &mut BTMSi32, high: &mut BTMSi32) {
            while low.len() > high.len() + 1 {
                let max = low.pop_last().unwrap();
                high.insert(max);
            }
            while high.len() > low.len() {
                let min = high.pop_first().unwrap();
                low.insert(min);
            }
        }

        fn insert(low: &mut BTMSi32, high: &mut BTMSi32, value: i32) {
            if low.len() == 0 || value <= *low.last().unwrap() {
                low.insert(value);
            } else {
                high.insert(value);
            }
            rebalance(low, high);
        }

        fn remove(low: &mut BTMSi32, high: &mut BTMSi32, value: i32) {
            if low.remove(&value) > 0 {
                rebalance(low, high);
            } else if high.remove(&value) > 0 {
                rebalance(low, high);
            }
        }

        fn median(low: &BTMSi32, high: &BTMSi32) -> f64 {
            if low.len() == high.len() {
                (*low.last().unwrap() as f64 + *high.first().unwrap() as f64) / 2.0
            } else {
                *low.last().unwrap() as f64
            }
        }

        let mut r = vec![];
        let mut low = BTMSi32::new();
        let mut high = BTMSi32::new();

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
