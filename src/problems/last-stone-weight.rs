// @leetup=custom
// @leetup=info id=1046 lang=rust slug=last-stone-weight

/*
* You are given an array of integers `stones` where `stones[i]` is the weight of
* the `ith` stone.
*
* We are playing a game with the stones. On each turn, we choose the heaviest
* two stones and smash them together. Suppose the heaviest two stones have
* weights `x` and `y` with `x <= y`. The result of this smash is:
*
* * If `x == y`, both stones are destroyed, and
* * If `x != y`, the stone of weight `x` is destroyed, and the stone of weight `y`
*   has new weight `y - x`.
*
* At the end of the game, there is at most one stone left.
*
* Return *the smallest possible weight of the left stone*. If there are no stones
* left, return `0`.
*
*
* Example 1:
*
* Input: stones = [2,7,4,1,8,1]
* Output: 1
* Explanation:
* We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then,
* we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then,
* we combine 2 and 1 to get 1 so the array converts to [1,1,1] then,
* we combine 1 and 1 to get 0 so the array converts to [1] then that's the value o
* f the last stone.
*
* Example 2:
*
* Input: stones = [1]
* Output: 1
*
*
* Constraints:
*
* * `1 <= stones.length <= 30`
* * `1 <= stones[i] <= 1000`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        while heap.len() > 1 {
            let y = heap.pop().unwrap();
            let x = heap.pop().unwrap();
            if x != y {
                heap.push(y - x);
            }
        }
        heap.pop().unwrap_or(0)
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::last_stone_weight(vec![1]), 1)
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::last_stone_weight(vec![1, 3]), 2)
    }
}
// @leetup=inject:after_code
