// @leetup=custom
// @leetup=info id=2136 lang=rust slug=earliest-possible-day-of-full-bloom

/*
* You have `n` flower seeds. Every seed must be planted first before it can begin
* to grow, then bloom. Planting a seed takes time and so does the growth of a
* seed. You are given two 0-indexed integer arrays `plantTime` and `growTime`,
* of length `n` each:
*
* * `plantTime[i]` is the number of full days it takes you to plant the
*   `ith` seed. Every day, you can work on planting exactly one seed. You do
*   not have to work on planting the same seed on consecutive days, but the
*   planting of a seed is not complete until you have worked `plantTime[i]`
*   days on planting it in total.
* * `growTime[i]` is the number of full days it takes the `ith` seed to grow
*   after being completely planted. After the last day of its growth, the
*   flower blooms and stays bloomed forever.
*
* From the beginning of day `0`, you can plant the seeds in any order.
*
* Return *the earliest possible day where all seeds are blooming*.
*
*
* Example 1:
*
* []
* Input: plantTime = [1,4,3], growTime = [2,3,1]
* Output: 9
* Explanation: The grayed out pots represent planting days, colored pots repre
* sent growing days, and the flower represents the day it blooms.
* One optimal way is:
* On day 0, plant the 0th seed. The seed grows for 2 full days and blooms on day 3
* .
* On days 1, 2, 3, and 4, plant the 1st seed. The seed grows for 3 full days and b
* looms on day 8.
* On days 5, 6, and 7, plant the 2nd seed. The seed grows for 1 full day and bloom
* s on day 9.
* Thus, on day 9, all the seeds are blooming.
*
* Example 2:
*
* []
* Input: plantTime = [1,2,3,2], growTime = [2,1,2,1]
* Output: 9
* Explanation: The grayed out pots represent planting days, colored pots repre
* sent growing days, and the flower represents the day it blooms.
* One optimal way is:
* On day 1, plant the 0th seed. The seed grows for 2 full days and blooms on day 4
* .
* On days 0 and 3, plant the 1st seed. The seed grows for 1 full day and blooms on
*  day 5.
* On days 2, 4, and 5, plant the 2nd seed. The seed grows for 2 full days and bloo
* ms on day 8.
* On days 6 and 7, plant the 3rd seed. The seed grows for 1 full day and blooms on
*  day 9.
* Thus, on day 9, all the seeds are blooming.
*
* Example 3:
*
* Input: plantTime = [1], growTime = [1]
* Output: 2
* Explanation: On day 0, plant the 0th seed. The seed grows for 1 full day and
*  blooms on day 2.
* Thus, on day 2, all the seeds are blooming.
*
*
* Constraints:
*
* * `n == plantTime.length == growTime.length`
* * `1 <= n <= 105`
* * `1 <= plantTime[i], growTime[i] <= 104`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

struct Plant {
    plant_time: i32,
    grow_time: i32,
}

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut plants = plant_time
            .iter()
            .enumerate()
            .map(|(i, time)| Plant {
                plant_time: *time,
                grow_time: grow_time[i],
            })
            .collect::<Vec<Plant>>();

        plants.sort_by(|a, b| b.grow_time.cmp(&a.grow_time));

        let mut current_time = 0;
        plants.iter().fold(0, |answer, plant| {
            current_time += plant.plant_time;
            answer.max(current_time + plant.grow_time)
        })
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
            Solution::earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1]),
            9
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::earliest_full_bloom(vec![1, 2, 3, 2], vec![2, 1, 2, 1]),
            9
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::earliest_full_bloom(vec![1], vec![1]), 2);
    }
}
// @leetup=inject:after_code
