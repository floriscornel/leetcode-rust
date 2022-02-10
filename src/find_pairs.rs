#![allow(dead_code, unused_variables)]
struct Solution {}

/** K-diff Pairs in an Array - https://leetcode.com/problems/k-diff-pairs-in-an-array/ */

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut histogram: HashMap<i32, i32> = HashMap::new();
        let mut found_pairs = 0;
        for x in nums.iter() {
            histogram.entry(*x).and_modify(|e| *e += 1).or_insert(1);
        }
        for (x, count) in histogram.iter() {
            let y = *x - k;
            if *x != y {
                match histogram.get(&y) {
                    Some(_count) => found_pairs += 1,
                    None => continue,
                }
            } else if *count > 1 {
                found_pairs += 1;
            }
        }
        found_pairs
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_pairs() {
        assert_eq!(super::Solution::find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
    }
}
