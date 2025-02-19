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
            if low.remove(&value) {
                rebalance(low, high);
            } else if high.remove(&value) {
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
        let mut low = BTMSi32::default();
        let mut high = BTMSi32::default();

        for i in 0..k as usize {
            insert(&mut low, &mut high, nums[i]);
        }
        r.push(median(&low, &high));

        for i in k as usize..nums.len() {
            remove(&mut low, &mut high, nums[i - k as usize]);
            insert(&mut low, &mut high, nums[i]);
            r.push(median(&low, &high));
        }

        r
    }
}

use std::collections::BTreeMap;

#[derive(Clone)]
struct BTreeMultiSet<T> {
    map: BTreeMap<T, usize>,
    length: usize,
}

impl<T> Default for BTreeMultiSet<T> {
    fn default() -> Self {
        BTreeMultiSet {
            map: BTreeMap::new(),
            length: 0,
        }
    }
}

impl<T> BTreeMultiSet<T> {
    fn len(&self) -> usize {
        self.length
    }

    fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        self.map
            .entry(value)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        self.length += 1;
    }

    fn remove(&mut self, value: &T) -> bool
    where
        T: Ord,
    {
        if let Some(count) = self.map.get_mut(value) {
            *count -= 1;
            self.length -= 1;
            if *count == 0 {
                self.map.remove(value);
            }
            true
        } else {
            false
        }
    }

    fn first(&self) -> Option<&T>
    where
        T: Ord,
    {
        self.map.first_key_value().map(|(value, _)| value)
    }

    fn last(&self) -> Option<&T>
    where
        T: Ord,
    {
        self.map.last_key_value().map(|(value, _)| value)
    }

    fn pop_first(&mut self) -> Option<T>
    where
        T: Ord + Clone,
    {
        if let Some(mut first) = self.map.first_entry() {
            *first.get_mut() -= 1;
            let value = first.key().clone();
            if *first.get() == 0 {
                first.remove();
            }
            self.length -= 1;
            Some(value)
        } else {
            None
        }
    }

    fn pop_last(&mut self) -> Option<T>
    where
        T: Ord + Clone,
    {
        if let Some(mut last) = self.map.last_entry() {
            *last.get_mut() -= 1;
            let value = last.key().clone();
            if *last.get() == 0 {
                last.remove();
            }
            self.length -= 1;
            Some(value)
        } else {
            None
        }
    }
}
