#![allow(dead_code, unused_variables)]
/** Insert Delete GetRandom O(1) - https://leetcode.com/problems/insert-delete-getrandom-o1/ */
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

pub struct RandomizedSet {
    indexes: HashMap<i32, usize>,
    values: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            indexes: HashMap::new(),
            values: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        match self.indexes.get(&val) {
            Some(_) => false,
            None => {
                self.indexes.insert(val, self.values.len());
                self.values.push(val);
                true
            }
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.indexes.get(&val) {
            None => false,
            Some(index_pointer) => {
                let index = *index_pointer;
                self.values[index] = self.values[self.values.len() - 1];
                self.indexes
                    .insert(self.values[self.values.len() - 1], index);
                self.indexes.remove(&val);
                self.values.pop();
                true
            }
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        *self.values.choose(&mut rng).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::RandomizedSet;
    #[test]
    fn example_1() {
        let mut obj = RandomizedSet::new();
        assert!(obj.insert(1));
        assert!(!obj.remove(2));
        assert!(obj.insert(2));
        assert!(vec![1, 2].contains(&obj.get_random()));
        assert!(obj.remove(1));
        assert!(!obj.insert(2));
        assert_eq!(obj.get_random(), 2);
    }
}
