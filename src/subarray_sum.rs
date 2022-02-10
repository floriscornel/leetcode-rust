#![allow(dead_code, unused_variables)]
struct Solution {}

/** Subarray Sum Equals K - https://leetcode.com/problems/subarray-sum-equals-k/ */
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let (mut count, mut sum) = (0, 0);
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        map.insert(0, 1);
        for num in nums {
            sum += num;
            if let Some(val) = map.get(&(sum - k)) {
                count += val;
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn subarray_sum_t1() {
        let a = vec![1, 1, 1];
        assert_eq!(super::Solution::subarray_sum(a, 2), 2);

        let b = vec![1, 2, 3];
        assert_eq!(super::Solution::subarray_sum(b, 3), 2);
    }
}
