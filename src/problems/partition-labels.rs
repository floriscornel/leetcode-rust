// @leetup=custom
// @leetup=info id=763 lang=rust slug=partition-labels

/*
* You are given a string `s`. We want to partition the string into as many parts
* as possible so that each letter appears in at most one part.
*
* Note that the partition is done so that after concatenating all the parts in
* order, the resultant string should be `s`.
*
* Return *a list of integers representing the size of these parts*.
*
*
* Example 1:
*
* Input: s = "ababcbacadefegdehijhklij"
* Output: [9,7,8]
* Explanation:
* The partition is "ababcbaca", "defegde", "hijhklij".
* This is a partition so that each letter appears in at most one part.
* A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits
* s into less parts.
*
* Example 2:
*
* Input: s = "eccbbbbdec"
* Output: [10]
*
*
* Constraints:
*
* * `1 <= s.length <= 500`
* * `s` consists of lowercase English letters.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last = HashMap::<char, usize>::new();
        for (position, char) in s.chars().enumerate() {
            last.insert(char, position);
        }
        let mut solution = Vec::new();
        let (mut segment_length, mut segment_min_position) = (0, 0);
        for (position, char) in s.chars().enumerate() {
            segment_length += 1;
            segment_min_position = usize::max(segment_min_position, *last.get(&char).unwrap());
            if position == segment_min_position {
                solution.push(segment_length);
                segment_length = 0;
            }
        }
        solution
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_owned()),
            vec![9, 7, 8]
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::partition_labels("eccbbbbdec".to_owned()),
            vec![10]
        )
    }
}
// @leetup=inject:after_code
