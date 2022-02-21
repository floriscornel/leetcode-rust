// @leetup=custom
// @leetup=info id=1288 lang=rust slug=remove-covered-intervals

/*
* Given an array `intervals` where `intervals[i] = [li, ri]` represent the
* interval `[li, ri)`, remove all intervals that are covered by another interval
* in the list.
*
* The interval `[a, b)` is covered by the interval `[c, d)` if and only if `c <=
* a` and `b <= d`.
*
* Return *the number of remaining intervals*.
*
*
* Example 1:
*
* Input: intervals = [[1,4],[3,6],[2,8]]
* Output: 2
* Explanation: Interval [3,6] is covered by [2,8], therefore it is removed.
*
* Example 2:
*
* Input: intervals = [[1,4],[2,3]]
* Output: 1
*
*
* Constraints:
*
* * `1 <= intervals.length <= 1000`
* * `intervals[i].length == 2`
* * `0 <= li <= ri <= 105`
* * All the given intervals are unique.
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
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|interval: &Vec<i32>| (interval[0], -interval[1]));

        let mut count = 0;
        let mut end = 0;

        for interval in intervals.iter() {
            if interval[1] > end {
                count += 1;
                end = interval[1];
            }
        }
        count
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
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]]),
            2
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![2, 3]]),
            1
        )
    }
}
// @leetup=inject:after_code
