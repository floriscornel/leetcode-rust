// @leetup=custom
// @leetup=info id=923 lang=rust slug=3sum-with-multiplicity

/*
* Given an integer array `arr`, and an integer `target`, return the number of
* tuples `i, j, k` such that `i < j < k` and `arr[i] + arr[j] + arr[k] == target`.
*
* As the answer can be very large, return it modulo `109 + 7`.
*
*
* Example 1:
*
* Input: arr = [1,1,2,2,3,3,4,4,5,5], target = 8
* Output: 20
* Explanation:
* Enumerating by the values (arr[i], arr[j], arr[k]):
* (1, 2, 5) occurs 8 times;
* (1, 3, 4) occurs 8 times;
* (2, 2, 4) occurs 2 times;
* (2, 3, 3) occurs 2 times.
*
* Example 2:
*
* Input: arr = [1,1,2,2,2,2], target = 5
* Output: 12
* Explanation:
* arr[i] = 1, arr[j] = arr[k] = 2 occurs 12 times:
* We choose one 1 from [1,1] in 2 ways,
* and two 2s from [2,2,2,2] in 6 ways.
*
*
* Constraints:
*
* * `3 <= arr.length <= 3000`
* * `0 <= arr[i] <= 100`
* * `0 <= target <= 300`
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
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let mut hist = vec![0_i64; 101];
        arr.iter().for_each(|&n| hist[n as usize] += 1);
        let mut answer = 0;
        for i in 0..hist.len() {
            for j in i..hist.len() {
                let k = target - i as i32 - j as i32;
                if (0..=100).contains(&k) {
                    if i == j && j == k as usize {
                        answer += (hist[i] * (hist[i] - 1) * (hist[i] - 2)) / 6;
                    } else if i == j && j != k as usize {
                        answer += ((hist[i] * (hist[i] - 1)) / 2) * hist[k as usize]
                    } else if j < k as usize {
                        answer += hist[i] * hist[j] * hist[k as usize];
                    }
                }
            }
        }
        (answer % 1_000_000_007) as i32
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
            Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8),
            20
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::three_sum_multi(vec![1, 1, 2, 2, 2, 2], 5), 12);
    }
}
// @leetup=inject:after_code
