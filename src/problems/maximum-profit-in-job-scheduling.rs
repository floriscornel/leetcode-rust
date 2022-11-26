// @leetup=custom
// @leetup=info id=1235 lang=rust slug=maximum-profit-in-job-scheduling

/*
* We have `n` jobs, where every job is scheduled to be done from `startTime[i]` to
* `endTime[i]`, obtaining a profit of `profit[i]`.
*
* You're given the `startTime`, `endTime` and `profit` arrays, return the maximum
* profit you can take such that there are no two jobs in the subset with
* overlapping time range.
*
* If you choose a job that ends at time `X` you will be able to start another job
* that starts at time `X`.
*
*
* Example 1:
*
* []
*
* Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
* Output: 120
* Explanation: The subset chosen is the first and fourth job.
* Time range [1-3]+[3-6] , we get profit of 120 = 50 + 70.
*
* Example 2:
*
* []
*
* Input: startTime = [1,2,3,4,6], endTime = [3,5,10,6,9], profit = [20,20,100,
* 70,60]
* Output: 150
* Explanation: The subset chosen is the first, fourth and fifth job.
* Profit obtained 150 = 20 + 70 + 60.
*
* Example 3:
*
* []
*
* Input: startTime = [1,1,1], endTime = [2,3,4], profit = [5,6,4]
* Output: 6
*
*
* Constraints:
*
* * `1 <= startTime.length == endTime.length == profit.length <= 5 * 104`
* * `1 <= startTime[i] < endTime[i] <= 109`
* * `1 <= profit[i] <= 104`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

type Job = (i32, i32, i32);
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs: Vec<Job> = start_time
            .into_iter()
            .zip(end_time.into_iter())
            .zip(profit.into_iter())
            .map(|((start, end), profit)| (start, end, profit))
            .collect();
        jobs.sort_unstable_by(|&(start_a, _, _), (start_b, _, _)| start_a.cmp(start_b));

        let mut cache: Vec<Option<i32>> = vec![None; jobs.len()];
        Self::dfs(&jobs, &mut cache, 0)
    }

    fn dfs(jobs: &[Job], cache: &mut [Option<i32>], index: usize) -> i32 {
        if index >= jobs.len() {
            return 0;
        }

        if let Some(cached) = cache[index] {
            return cached;
        }

        let (_, current_end, current_profit) = jobs[index];

        let accept_job_profit: i32 = if let Some((next_index, _)) = jobs
            .iter()
            .enumerate()
            .skip(index + 1)
            .find(|(_, &(next_start, _, _))| next_start >= current_end)
        {
            current_profit + Self::dfs(jobs, cache, next_index)
        } else {
            current_profit
        };

        let ignore_job_profit: i32 = Self::dfs(jobs, cache, index + 1);

        let best_choice = accept_job_profit.max(ignore_job_profit);
        cache[index] = Some(best_choice);
        best_choice
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let start_time = [1, 2, 3, 3];
        let end_time = [3, 4, 5, 6];
        let profit = [50, 10, 40, 70];
        assert_eq!(
            Solution::job_scheduling(start_time.to_vec(), end_time.to_vec(), profit.to_vec()),
            120
        );
    }

    #[test]
    fn example_2() {
        let start_time = [1, 2, 3, 4, 6];
        let end_time = [3, 5, 10, 6, 9];
        let profit = [20, 20, 100, 70, 60];
        assert_eq!(
            Solution::job_scheduling(start_time.to_vec(), end_time.to_vec(), profit.to_vec()),
            150
        );
    }

    #[test]
    fn example_3() {
        let start_time = [1, 1, 1];
        let end_time = [2, 3, 4];
        let profit = [5, 6, 4];
        assert_eq!(
            Solution::job_scheduling(start_time.to_vec(), end_time.to_vec(), profit.to_vec()),
            6
        );
    }
}
// @leetup=inject:after_code
