// @leetup=custom
// @leetup=info id=380 lang=rust slug=insert-delete-getrandom-o1

/*
* Implement the `RandomizedSet` class:
*
* * `RandomizedSet()` Initializes the `RandomizedSet` object.
* * `bool insert(int val)` Inserts an item `val` into the set if not present.
*   Returns `true` if the item was not present, `false` otherwise.
* * `bool remove(int val)` Removes an item `val` from the set if present. Returns
*   `true` if the item was present, `false` otherwise.
* * `int getRandom()` Returns a random element from the current set of elements
*   (it's guaranteed that at least one element exists when this method is called).
*   Each element must have the same probability of being returned.
*
* You must implement the functions of the class such that each function works in
* average `O(1)` time complexity.
*
*
* Example 1:
*
* Input
* ["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert",
*  "getRandom"]
* [[], [1], [2], [2], [], [1], [2], []]
* Output
* [null, true, false, true, 2, true, false, 2]
* Explanation
* RandomizedSet randomizedSet = new RandomizedSet();
* randomizedSet.insert(1); // Inserts 1 to the set. Returns true as 1 was inserted
*  successfully.
* randomizedSet.remove(2); // Returns false as 2 does not exist in the set.
* randomizedSet.insert(2); // Inserts 2 to the set, returns true. Set now contains
*  [1,2].
* randomizedSet.getRandom(); // getRandom() should return either 1 or 2 randomly.
* randomizedSet.remove(1); // Removes 1 from the set, returns true. Set now contai
* ns [2].
* randomizedSet.insert(2); // 2 was already in the set, so return false.
* randomizedSet.getRandom(); // Since 2 is the only number in the set, getRandom()
*  will always return 2.
*
*
* Constraints:
*
* * `-231 <= val <= 231 - 1`
* * At most `2 * ``105` calls will be made to `insert`, `remove`, and `getRandom`.
* * There will be at least one element in the data structure when `getRandom`
*   is called.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
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
/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
// @leetup=code

// @leetup=inject:after_code
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
// @leetup=inject:after_code
