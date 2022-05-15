// @leetup=custom
// @leetup=info id=47 lang=rust slug=permutations-ii

/*
* Given a collection of numbers, `nums`, that might contain duplicates, return
* *all possible unique permutations in any order.*
*
*
* Example 1:
*
* Input: nums = [1,1,2]
* Output:
* [[1,1,2],
*  [1,2,1],
*  [2,1,1]]
*
* Example 2:
*
* Input: nums = [1,2,3]
* Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
*
*
* Constraints:
*
* * `1 <= nums.length <= 8`
* * `-10 <= nums[i] <= 10`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::HashSet;

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    permutation: Vec<i32>,
    choices: Vec<i32>,
}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let initial_state = State {
            permutation: Vec::new(),
            choices: nums,
        };
        let mut states: Vec<State> = vec![initial_state];
        let mut visited: HashSet<State> = HashSet::new();

        let mut results: Vec<Vec<i32>> = Vec::new();
        while !states.is_empty() {
            let current_state = states.pop().unwrap();
            for choice in current_state.choices.clone() {
                let mut new_permutation = current_state.permutation.clone();
                new_permutation.push(choice);
                let mut new_choices = current_state.choices.clone();
                new_choices.remove(
                    new_choices
                        .iter()
                        .position(|x| *x == choice)
                        .expect("choice not found"),
                );
                let finished = new_choices.is_empty();
                let new_state = State {
                    permutation: new_permutation,
                    choices: new_choices,
                };
                if visited.contains(&new_state) {
                    continue;
                }
                visited.insert(new_state.clone());
                if finished {
                    results.push(new_state.permutation);
                } else {
                    states.push(new_state);
                }
            }
        }

        results
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Solution;

    fn unordered_eq(output: Vec<Vec<i32>>, expected_output: Vec<Vec<i32>>) {
        let mut a = HashSet::<Vec<i32>>::new();
        for x in output {
            a.insert(x);
        }
        let mut b = HashSet::<Vec<i32>>::new();
        for x in expected_output {
            b.insert(x);
        }
        assert_eq!(a, b);
    }

    #[test]
    fn example_1() {
        let input = vec![1, 1, 2];
        let expected_output = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
        let output = Solution::permute_unique(input);
        unordered_eq(output, expected_output);
    }

    #[test]
    fn example_2() {
        let input = vec![1, 2, 3];
        let expected_output = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        let output = Solution::permute_unique(input);

        unordered_eq(output, expected_output);
    }
}
// @leetup=inject:after_code
