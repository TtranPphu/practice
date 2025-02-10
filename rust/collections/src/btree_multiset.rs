use std::cmp::Ordering;
use std::collections::btree_map::BTreeMap;
use std::hash::{Hash, Hasher};

/// Incomplete implementation of multiset collection using BtreeMap.
/// To be refined along the way.
pub struct BTreeMultiSet<T> {
  map: BTreeMap<T, usize>,
  length: usize,
}

impl<T> BTreeMultiSet<T> {
  pub const fn new() -> Self {
    BTreeMultiSet {
      map: BTreeMap::new(),
      length: 0,
    }
  }

  pub const fn len(&self) -> usize {
    self.length
  }

  pub fn insert(&mut self, value: T)
  where
    T: Ord,
  {
    self
      .map
      .entry(value)
      .and_modify(|count| *count += 1)
      .or_insert(1);
    self.length += 1;
  }

  pub fn remove(&mut self, value: &T) -> bool
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

  pub fn first(&self) -> Option<&T>
  where
    T: Ord,
  {
    self.map.first_key_value().map(|(value, _)| value)
  }

  pub fn last(&self) -> Option<&T>
  where
    T: Ord,
  {
    self.map.last_key_value().map(|(value, _)| value)
  }

  pub fn pop_first(&mut self) -> Option<T>
  where
    T: Clone + Ord,
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

  pub fn pop_last(&mut self) -> Option<T>
  where
    T: Clone + Ord,
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

  pub fn iter(&self) -> impl Iterator<Item = &T>
  where
    T: Ord,
  {
    self
      .map
      .iter()
      .flat_map(|(value, &count)| std::iter::repeat(value).take(count))
  }

  pub fn clear(&mut self) {
    self.map.clear();
    self.length = 0;
  }

  pub fn is_empty(&self) -> bool {
    self.length == 0
  }

  pub fn contains(&self, value: &T) -> bool
  where
    T: Ord,
  {
    self.map.contains_key(value)
  }
}

impl<T: Hash> Hash for BTreeMultiSet<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.map.hash(state);
  }
}

impl<T: PartialEq> PartialEq for BTreeMultiSet<T> {
  fn eq(&self, other: &Self) -> bool {
    self.map.eq(&other.map)
  }
}

impl<T: Eq> Eq for BTreeMultiSet<T> {}

impl<T: PartialOrd> PartialOrd for BTreeMultiSet<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.map.partial_cmp(&other.map)
  }
}

impl<T: Ord> Ord for BTreeMultiSet<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.map.cmp(&other.map)
  }
}

impl<T: Clone> Clone for BTreeMultiSet<T> {
  fn clone(&self) -> Self {
    BTreeMultiSet {
      map: self.map.clone(),
      length: self.length,
    }
  }

  fn clone_from(&mut self, source: &Self) {
    self.map.clone_from(&source.map);
  }
}
