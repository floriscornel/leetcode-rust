// @leetup=custom
// @leetup=info id=460 lang=rust slug=lfu-cache

/*
* Design and implement a data structure for a [Least Frequently Used (LFU)][1]
* cache.
*
* Implement the `LFUCache` class:
*
* * `LFUCache(int capacity)` Initializes the object with the `capacity` of the
*   data structure.
* * `int get(int key)` Gets the value of the `key` if the `key` exists in the
*   cache. Otherwise, returns `-1`.
* * `void put(int key, int value)` Update the value of the `key` if present, or
*   inserts the `key` if not already present. When the cache reaches its
*   `capacity`, it should invalidate and remove the least frequently used key
*   before inserting a new item. For this problem, when there is a tie (i.e.,
*   two or more keys with the same frequency), the least recently used `key`
*   would be invalidated.
*
* To determine the least frequently used key, a use counter is maintained for
* each key in the cache. The key with the smallest use counter is the least
* frequently used key.
*
* When a key is first inserted into the cache, its use counter is set to `1`
* (due to the `put` operation). The use counter for a key in the cache is
* incremented either a `get` or `put` operation is called on it.
*
* The functions `get` and `put` must each run in `O(1)` average time complexity.
*
*
* Example 1:
*
* Input
* ["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get
* "]
* [[2], [1, 1], [2, 2], [1], [3, 3], [2], [3], [4, 4], [1], [3], [4]]
* Output
* [null, null, null, 1, null, -1, 3, null, -1, 3, 4]
* Explanation
* // cnt(x) = the use counter for key x
* // cache=[] will show the last used order for tiebreakers (leftmost element is
* most recent)
* LFUCache lfu = new LFUCache(2);
* lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
* lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
* lfu.get(1);      // return 1
*                  // cache=[1,2], cnt(2)=1, cnt(1)=2
* lfu.put(3, 3);   // 2 is the LFU key because cnt(2)=1 is the smallest, invalidat
* e 2.
*                  // cache=[3,1], cnt(3)=1, cnt(1)=2
* lfu.get(2);      // return -1 (not found)
* lfu.get(3);      // return 3
*                  // cache=[3,1], cnt(3)=2, cnt(1)=2
* lfu.put(4, 4);   // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
*                  // cache=[4,3], cnt(4)=1, cnt(3)=2
* lfu.get(1);      // return -1 (not found)
* lfu.get(3);      // return 3
*                  // cache=[3,4], cnt(4)=1, cnt(3)=3
* lfu.get(4);      // return 4
*                  // cache=[4,3], cnt(4)=2, cnt(3)=3
*
*
* Constraints:
*
* * `0 <= capacity <= 104`
* * `0 <= key <= 105`
* * `0 <= value <= 109`
* * At most `2 * 105` calls will be made to `get` and `put`.
*
*
* [1] https://en.wikipedia.org/wiki/Least_frequently_used
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
// @leetup=inject:before_code_ex

// @leetup=code

use std::collections::{BTreeMap, HashMap};

struct LFUCache {
    map: HashMap<i32, (usize, usize, i32)>,
    tree: BTreeMap<(usize, usize, i32), bool>,
    capacity: usize,
    counter: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        Self {
            map: HashMap::with_capacity(capacity),
            tree: BTreeMap::new(),
            capacity,
            counter: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        self.counter += 1;
        if let Some(&(count, incr, value)) = self.map.get(&key) {
            self.tree.remove(&(count, incr, key));
            self.tree.insert((count + 1, self.counter, key), true);
            self.map.remove(&key);
            self.map.insert(key, (count + 1, self.counter, value));
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.counter += 1;
        if let Some(&(count, incr, _)) = self.map.get(&key) {
            self.map.insert(key, (count + 1, self.counter, value));
            self.tree.remove(&(count, incr, key));
            self.tree.insert((count + 1, self.counter, key), true);
        } else {
            if self.capacity == 0 {
                return;
            }
            if self.map.len() == self.capacity {
                if let Some(&(count, incr, key)) = self.tree.keys().next() {
                    self.map.remove(&key);
                    self.tree.remove(&(count, incr, key));
                }
            }
            self.map.insert(key, (1, self.counter, value));
            self.tree.insert((1, self.counter, key), true);
        }
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::LFUCache;

    #[test]
    fn example_1() {
        let mut lfu = LFUCache::new(2);
        lfu.put(1, 1);
        lfu.put(2, 2);
        assert_eq!(lfu.get(1), 1);
        lfu.put(3, 3);
        assert_eq!(lfu.get(2), -1);
        assert_eq!(lfu.get(3), 3);
        lfu.put(4, 4);
        assert_eq!(lfu.get(1), -1);
        assert_eq!(lfu.get(3), 3);
        assert_eq!(lfu.get(4), 4);
    }

    #[test]
    fn example_2() {
        let mut lfu = LFUCache::new(2);
        lfu.put(2, 1);
        lfu.put(3, 2);
        assert_eq!(lfu.get(3), 2);
        assert_eq!(lfu.get(2), 1);
        lfu.put(4, 3);
        assert_eq!(lfu.get(2), 1);
        assert_eq!(lfu.get(3), -1);
        assert_eq!(lfu.get(4), 3);
    }
}
// @leetup=inject:after_code
