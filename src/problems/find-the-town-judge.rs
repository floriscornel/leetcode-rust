// @leetup=custom
// @leetup=info id=997 lang=rust slug=find-the-town-judge

/*
* In a town, there are `n` people labeled from `1` to `n`. There is a rumor that
* one of these people is secretly the town judge.
*
* If the town judge exists, then:
*
* 1. The town judge trusts nobody.
* 2. Everybody (except for the town judge) trusts the town judge.
* 3. There is exactly one person that satisfies properties 1 and 2.
*
* You are given an array `trust` where `trust[i] = [ai, bi]` representing that the
* person labeled `ai` trusts the person labeled `bi`.
*
* Return *the label of the town judge if the town judge exists and can be
* identified, or return *`-1`* otherwise*.
*
*
* Example 1:
*
* Input: n = 2, trust = [[1,2]]
* Output: 2
*
* Example 2:
*
* Input: n = 3, trust = [[1,3],[2,3]]
* Output: 3
*
* Example 3:
*
* Input: n = 3, trust = [[1,3],[2,3],[3,1]]
* Output: -1
*
*
* Constraints:
*
* * `1 <= n <= 1000`
* * `0 <= trust.length <= 104`
* * `trust[i].length == 2`
* * All the pairs of `trust` are unique.
* * `ai != bi`
* * `1 <= ai, bi <= n`
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

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut trusting_people_set = HashSet::<usize>::new();
        let mut trusted_array = vec![0; (n + 1) as usize];

        for (truster, trusted) in trust
            .into_iter()
            .map(|pair| (pair[0] as usize, pair[1] as usize))
        {
            trusting_people_set.insert(truster);
            trusted_array[trusted] += 1;
        }

        for (person, score) in trusted_array.into_iter().enumerate().skip(1) {
            if score == (n - 1) && !trusting_people_set.contains(&person) {
                return person as i32;
            }
        }
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
        let n = 2;
        let trust = vec![vec![1, 2]];
        let expected = 2;
        assert_eq!(Solution::find_judge(n, trust), expected);
    }

    #[test]
    fn example_2() {
        let n = 3;
        let trust = vec![vec![1, 3], vec![2, 3]];
        let expected = 3;
        assert_eq!(Solution::find_judge(n, trust), expected);
    }

    #[test]
    fn example_3() {
        let n = 3;
        let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
        let expected = -1;
        assert_eq!(Solution::find_judge(n, trust), expected);
    }

    #[test]
    fn example_4() {
        let n = 3;
        let trust = vec![vec![1, 2], vec![2, 3]];
        let expected = -1;
        assert_eq!(Solution::find_judge(n, trust), expected);
    }
}
// @leetup=inject:after_code
