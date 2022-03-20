// @leetup=custom
// @leetup=info id=1007 lang=rust slug=minimum-domino-rotations-for-equal-row

/*
* In a row of dominoes, `tops[i]` and `bottoms[i]` represent the top and bottom
* halves of the `ith` domino. (A domino is a tile with two numbers from 1 to 6 -
* one on each half of the tile.)
*
* We may rotate the `ith` domino, so that `tops[i]` and `bottoms[i]` swap values.
*
* Return the minimum number of rotations so that all the values in `tops` are the
* same, or all the values in `bottoms` are the same.
*
* If it cannot be done, return `-1`.
*
*
* Example 1:
*
* []
* Input: tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
* Output: 2
* Explanation:
* The first figure represents the dominoes as given by tops and bottoms: before we
*  do any rotations.
* If we rotate the second and fourth dominoes, we can make every value in the top
* row equal to 2, as indicated by the second figure.
*
* Example 2:
*
* Input: tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
* Output: -1
* Explanation:
* In this case, it is not possible to rotate the dominoes to make one row of value
* s equal.
*
*
* Constraints:
*
* * `2 <= tops.length <= 2 * 104`
* * `bottoms.length == tops.length`
* * `1 <= tops[i], bottoms[i] <= 6`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::{HashSet, VecDeque};
#[derive(Debug, Clone)]
struct State {
    tops: Vec<i32>,
    bottoms: Vec<i32>,
    flipped: HashSet<usize>,
}

impl Solution {
    fn flip(current: &State, choice: usize) -> State {
        let mut new = State {
            tops: current.tops.clone(),
            bottoms: current.bottoms.clone(),
            flipped: current.flipped.clone(),
        };
        new.tops[choice] = current.bottoms[choice];
        new.bottoms[choice] = current.tops[choice];
        new.flipped.insert(choice);
        new
    }

    fn same_elements(list: &[i32]) -> bool {
        for elem in list.iter() {
            if list[0] != *elem {
                return false;
            }
        }
        true
    }

    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let number_of_choices = tops.len();
        let mut states = VecDeque::<State>::new();
        let initial_state = State {
            tops,
            bottoms,
            flipped: HashSet::<usize>::new(),
        };
        states.push_back(initial_state);

        while !states.is_empty() {
            // Pop the current state from the front of the queue.
            let current_state = states.pop_front().unwrap();
            // Itterate over all possible moves
            for choice in 0..number_of_choices {
                // Check if a move should be made
                if !current_state.flipped.contains(&choice) {
                    // Generate a new state by applying the move
                    let new_state = Solution::flip(&current_state, choice);

                    // Check if we have reached completion
                    if Solution::same_elements(&new_state.tops)
                        || Solution::same_elements(&new_state.bottoms)
                    {
                        // Return the number of steps of the solution
                        return new_state.flipped.len() as i32;
                    }

                    // Push the new state to the back of the queue
                    states.push_back(new_state);
                }
            }
        }
        // After visiting all states, return -1
        -1
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let tops = vec![2, 1, 2, 4, 2, 2];
        let bottoms = vec![5, 2, 6, 2, 3, 2];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), 2)
    }

    #[test]
    fn example_2() {
        let tops = vec![3, 5, 1, 2, 3];
        let bottoms = vec![3, 6, 3, 3, 4];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), -1)
    }
}
// @leetup=inject:after_code
