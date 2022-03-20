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
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    moves: HashSet<usize>,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let number_of_choices = tops.len();
        let mut states = BinaryHeap::new();
        let mut initial_state = State {
            cost: 0,
            moves: HashSet::<usize>::new(),
        };
        if Solution::completed(&initial_state, &tops, &bottoms) {
            return 0;
        }
        initial_state.cost = Solution::get_costs(&initial_state, &tops, &bottoms);
        states.push(initial_state);
        while !states.is_empty() {
            let current_state = states.pop().unwrap();
            for choice in 0..number_of_choices {
                if !current_state.moves.contains(&choice) {
                    let mut new_state = State {
                        cost: 0,
                        moves: current_state.moves.clone(),
                    };
                    new_state.moves.insert(choice);
                    if Solution::completed(&new_state, &tops, &bottoms) {
                        return new_state.moves.len() as i32;
                    }
                    new_state.cost = Solution::get_costs(&new_state, &tops, &bottoms);
                    if new_state.cost < current_state.cost {
                        states.push(new_state);
                    }
                }
            }
        }
        -1
    }

    fn get_costs(state: &State, tops: &[i32], bottoms: &[i32]) -> usize {
        let mut top_map = HashMap::<i32, i32>::new();
        let mut bottom_map = HashMap::<i32, i32>::new();

        for index in 0..tops.len() {
            if state.moves.contains(&index) {
                let top_count = top_map.entry(bottoms[index]).or_insert(0);
                *top_count += 1;
                let bottom_count = bottom_map.entry(tops[index]).or_insert(0);
                *bottom_count += 1;
            } else {
                let top_count = top_map.entry(tops[index]).or_insert(0);
                *top_count += 1;
                let bottom_count = bottom_map.entry(bottoms[index]).or_insert(0);
                *bottom_count += 1;
            }
        }

        let top_most_occurences = *top_map.values().max().unwrap() as usize;
        let bottom_most_occurences = *bottom_map.values().max().unwrap() as usize;

        10000 * (tops.len() - usize::max(top_most_occurences, bottom_most_occurences))
            + state.moves.len()
    }

    fn completed(state: &State, tops: &[i32], bottoms: &[i32]) -> bool {
        let (mut top_complete, mut bottom_complete) = (true, true);
        for index in 0..tops.len() {
            if state.moves.contains(&index) {
                if tops[0] != bottoms[index] {
                    top_complete = false;
                }
                if bottoms[0] != tops[index] {
                    bottom_complete = false;
                }
            } else {
                if tops[0] != tops[index] {
                    top_complete = false;
                }
                if bottoms[0] != bottoms[index] {
                    bottom_complete = false;
                }
            }
            if !top_complete && !bottom_complete {
                return false;
            }
        }
        true
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

    #[test]
    fn example_3() {
        let tops = vec![1, 1, 1];
        let bottoms = vec![1, 1, 1];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), 0)
    }

    #[test]
    fn example_4() {
        let tops = vec![1, 0];
        let bottoms = vec![1, 0];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), -1)
    }

    #[test]
    fn example_5() {
        let tops = vec![1, 0];
        let bottoms = vec![0, 1];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), 1)
    }

    #[test]
    fn example_6() {
        let tops = vec![1, 3, 2, 3, 4, 1, 1, 4, 1];
        let bottoms = vec![5, 1, 5, 5, 2, 3, 5, 3, 1];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), -1)
    }
}
// @leetup=inject:after_code
