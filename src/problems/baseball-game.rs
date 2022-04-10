// @leetup=custom
// @leetup=info id=682 lang=rust slug=baseball-game

/*
* You are keeping score for a baseball game with strange rules. The game consists
* of several rounds, where the scores of past rounds may affect future rounds'
* scores.
*
* At the beginning of the game, you start with an empty record. You are given a
* list of strings `ops`, where `ops[i]` is the `ith` operation you must apply to
* the record and is one of the following:
*
* 1. An integer `x` - Record a new score of `x`.
* 2. `"+"` - Record a new score that is the sum of the previous two scores. It is
*    guaranteed there will always be two previous scores.
* 3. `"D"` - Record a new score that is double the previous score. It is
*    guaranteed there will always be a previous score.
* 4. `"C"` - Invalidate the previous score, removing it from the record. It is
*    guaranteed there will always be a previous score.
*
* Return *the sum of all the scores on the record*.
*
*
* Example 1:
*
* Input: ops = ["5","2","C","D","+"]
* Output: 30
* Explanation:
* "5" - Add 5 to the record, record is now [5].
* "2" - Add 2 to the record, record is now [5, 2].
* "C" - Invalidate and remove the previous score, record is now [5].
* "D" - Add 2 * 5 = 10 to the record, record is now [5, 10].
* "+" - Add 5 + 10 = 15 to the record, record is now [5, 10, 15].
* The total sum is 5 + 10 + 15 = 30.
*
* Example 2:
*
* Input: ops = ["5","-2","4","C","D","9","+","+"]
* Output: 27
* Explanation:
* "5" - Add 5 to the record, record is now [5].
* "-2" - Add -2 to the record, record is now [5, -2].
* "4" - Add 4 to the record, record is now [5, -2, 4].
* "C" - Invalidate and remove the previous score, record is now [5, -2].
* "D" - Add 2 * -2 = -4 to the record, record is now [5, -2, -4].
* "9" - Add 9 to the record, record is now [5, -2, -4, 9].
* "+" - Add -4 + 9 = 5 to the record, record is now [5, -2, -4, 9, 5].
* "+" - Add 9 + 5 = 14 to the record, record is now [5, -2, -4, 9, 5, 14].
* The total sum is 5 + -2 + -4 + 9 + 5 + 14 = 27.
*
* Example 3:
*
* Input: ops = ["1"]
* Output: 1
*
*
* Constraints:
*
* * `1 <= ops.length <= 1000`
* * `ops[i]` is `"C"`, `"D"`, `"+"`, or a string representing an integer in the
*   range `[-3 * 104, 3 * 104]`.
* * For operation `"+"`, there will always be at least two previous scores on the
*   record.
* * For operations `"C"` and `"D"`, there will always be at least one previous
*   score on the record.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut stack = Vec::<i32>::new();
        for op in ops {
            match op.chars().next().unwrap() {
                '+' => {
                    let (last_index, second_last_index) = (stack.len() - 1, stack.len() - 2);
                    stack.push(stack[last_index] + stack[second_last_index]);
                }
                'D' => {
                    let last_index = stack.len() - 1;
                    stack.push(stack[last_index] * 2);
                }
                'C' => {
                    let new_size = stack.len() - 1;
                    stack.truncate(new_size);
                }
                _ => {
                    let score = op.parse::<i32>().unwrap();
                    stack.push(score);
                }
            }
        }
        stack.iter().sum()
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let input = vec!["5", "2", "C", "D", "+"]
            .iter()
            .map(|x| (*x).to_owned())
            .collect();
        assert_eq!(Solution::cal_points(input), 30);
    }

    #[test]
    fn example_2() {
        let input = vec!["5", "-2", "4", "C", "D", "9", "+", "+"]
            .iter()
            .map(|x| (*x).to_owned())
            .collect();
        assert_eq!(Solution::cal_points(input), 27);
    }

    fn example_3() {
        assert_eq!(Solution::cal_points(vec!["1".to_owned()]), 1);
    }
}
// @leetup=inject:after_code
