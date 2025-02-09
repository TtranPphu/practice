use std::cmp::Ordering;
use std::collections::btree_map::BTreeMap;
use std::hash::{Hash, Hasher};

pub struct BTreeMultiSet<T> {
  map: BTreeMap<T, usize>,
  lenght: usize,
}

impl<T> BTreeMultiSet<T> {
  pub const fn new() -> Self {
    BTreeMultiSet {
      map: BTreeMap::new(),
      lenght: 0,
    }
  }

  pub const fn len(&self) -> usize {
    self.lenght
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
    self.lenght += 1;
  }

  pub fn remove(&mut self, value: &T) -> bool
  where
    T: Ord,
  {
    if let Some(count) = self.map.get_mut(value) {
      *count -= 1;
      self.lenght -= 1;
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

  pub fn pop_first(&mut self) -> Option<T>
  where
    T: Clone + Ord,
  {
    let first = self.map.pop_first();
    match first {
      Some((value, count)) => {
        if count > 1 {
          self.map.insert(value.clone(), count - 1);
        }
        self.lenght -= 1;
        Some(value)
      }
      None => None,
    }
  }

  pub fn last(&self) -> Option<&T>
  where
    T: Ord,
  {
    self.map.last_key_value().map(|(value, _)| value)
  }

  pub fn pop_last(&mut self) -> Option<T>
  where
    T: Clone + Ord,
  {
    let last = self.map.pop_last();
    match last {
      Some((value, count)) => {
        if count > 1 {
          self.map.insert(value.clone(), count - 1);
        }
        self.lenght -= 1;
        Some(value)
      }
      None => None,
    }
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
      lenght: self.lenght,
    }
  }

  fn clone_from(&mut self, source: &Self) {
    self.map.clone_from(&source.map);
  }
}
