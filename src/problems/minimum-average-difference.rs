#![allow(dead_code, unused_variables)]

struct Solution {}

impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let (mut prefix_sum, mut infix_sum) = (vec![nums[0] as i64], vec![nums[n - 1] as i64]);
        for i in 1..n {
            prefix_sum.push(prefix_sum[i - 1] + nums[i] as i64);
            prefix_sum[i - 1] = f64::floor(prefix_sum[i - 1] as f64 / i as f64) as i64;
            infix_sum.push(infix_sum[i - 1] + nums[n - 1 - i] as i64);
            infix_sum[i - 1] = f64::floor(infix_sum[i - 1] as f64 / i as f64) as i64;
        }
        prefix_sum[n - 1] = f64::floor(prefix_sum[n - 1] as f64 / n as f64) as i64;
        infix_sum[n - 1] = f64::floor(infix_sum[n - 1] as f64 / n as f64) as i64;
        infix_sum.reverse();
        infix_sum.push(0);

        let (mut min_val, mut min_idx) = (i32::MAX, 0);
        for i in 0..n {
            let abs_diff = i64::abs(prefix_sum[i] - infix_sum[i + 1]) as i32;
            if abs_diff < min_val {
                min_val = abs_diff;
                min_idx = i;
            }
        }
        min_idx as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::minimum_average_difference(vec![2, 5, 3, 9, 5, 3]),
            3
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::minimum_average_difference(vec![0]), 0)
    }
}
