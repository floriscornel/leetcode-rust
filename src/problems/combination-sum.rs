// @leetup=custom
// @leetup=info id=39 lang=rust slug=combination-sum

/*
* Given an array of distinct integers `candidates` and a target integer
* `target`, return *a list of all unique combinations of *`candidates`* where
* the chosen numbers sum to *`target`*.* You may return the combinations in any
* order.
*
* The same number may be chosen from `candidates` an unlimited number of
* times. Two combinations are unique if the frequency of at least one of the
* chosen numbers is different.
*
* It is guaranteed that the number of unique combinations that sum up to
* `target` is less than `150` combinations for the given input.
*
*
* Example 1:
*
* Input: candidates = [2,3,6,7], target = 7
* Output: [[2,2,3],[7]]
* Explanation:
* 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple time
* s.
* 7 is a candidate, and 7 = 7.
* These are the only two combinations.
*
* Example 2:
*
* Input: candidates = [2,3,5], target = 8
* Output: [[2,2,2,2],[2,3,3],[3,5]]
*
* Example 3:
*
* Input: candidates = [2], target = 1
* Output: []
*
*
* Constraints:
*
* * `1 <= candidates.length <= 30`
* * `1 <= candidates[i] <= 200`
* * All elements of `candidates` are distinct.
* * `1 <= target <= 500`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables, clippy::comparison_chain)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut states: Vec<Vec<i32>> = Vec::new();
        for num in &candidates {
            if *num < target {
                states.push(vec![*num]);
            } else if *num == target {
                result.push(vec![*num]);
            }
        }

        while let Some(state) = states.pop() {
            let sum: i32 = state.iter().sum();
            let last = state.iter().last().unwrap();
            for num in &candidates {
                if *num < *last {
                    continue;
                }
                let new_sum = sum + num;
                if new_sum > target {
                    continue;
                }
                let mut new_state = state.clone();
                new_state.push(*num);
                if new_sum == target {
                    result.push(new_state);
                } else if new_sum < target {
                    states.push(new_state);
                }
            }
        }

        result
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
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![7], vec![2, 2, 3]]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![3, 5], vec![2, 3, 3], vec![2, 2, 2, 2]]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::combination_sum(vec![2], 1),
            Vec::<Vec<i32>>::new()
        );
    }
}
// @leetup=inject:after_code
