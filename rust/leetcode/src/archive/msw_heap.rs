pub struct Solution;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        use std::collections::BinaryHeap;
        type MaxHeap = BinaryHeap<i32>;
        type MinHeap = BinaryHeap<Ri32>;

        fn rebalance(low: &mut MaxHeap, high: &mut MinHeap) {
            while low.len() > high.len() + 1 {
                let max = low.pop().unwrap().into();
                high.push(max);
            }
            while high.len() > low.len() {
                let min = high.pop().unwrap().0;
                low.push(min);
            }
        }

        fn insert(low: &mut MaxHeap, high: &mut MinHeap, value: i32) {
            if low.len() == 0 || value <= *low.peek().unwrap() {
                low.push(value);
            } else {
                high.push(value.into());
            }
            rebalance(low, high);
        }

        fn remove(low: &mut MaxHeap, high: &mut MinHeap, value: i32) {
            fn r<T: Ord>(heap: &mut BinaryHeap<T>, value: T) -> bool {
                let mut removed = false;
                heap.retain(|v| {
                    let should_remove = !removed && *v == value;
                    if should_remove {
                        removed = true;
                    }
                    !should_remove
                });
                removed
            }
            if r(low, value) {
                rebalance(low, high);
            } else if r(high, value.into()) {
                rebalance(low, high);
            }
        }

        fn median(low: &MaxHeap, high: &MinHeap) -> f64 {
            if low.len() == high.len() {
                (*low.peek().unwrap() as f64 + high.peek().unwrap().0 as f64) / 2.0
            } else {
                *low.peek().unwrap() as f64
            }
        }

        let mut r = vec![];
        let mut low = MaxHeap::new();
        let mut high = MinHeap::new();

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

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Ri32(i32);

impl Ord for Ri32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl From<i32> for Ri32 {
    fn from(i: i32) -> Self {
        Self(i)
    }
}
