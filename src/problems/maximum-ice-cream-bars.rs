// @leetup=custom
// @leetup=info id=1833 lang=rust slug=maximum-ice-cream-bars

/*
* It is a sweltering summer day, and a boy wants to buy some ice cream bars.
*
* At the store, there are `n` ice cream bars. You are given an array `costs` of
* length `n`, where `costs[i]` is the price of the `ith` ice cream bar in coins.
* The boy initially has `coins` coins to spend, and he wants to buy as many ice
* cream bars as possible.
*
* Return *the maximum number of ice cream bars the boy can buy with *`coins`*
* coins.*
*
* Note: The boy can buy the ice cream bars in any order.
*
*
* Example 1:
*
* Input: costs = [1,3,2,4,1], coins = 7
* Output: 4
* Explanation: The boy can buy ice cream bars at indices 0,1,2,4 for a total p
* rice of 1 + 3 + 2 + 1 = 7.
*
* Example 2:
*
* Input: costs = [10,6,8,7,7,8], coins = 5
* Output: 0
* Explanation: The boy cannot afford any of the ice cream bars.
*
* Example 3:
*
* Input: costs = [1,6,3,1,2,5], coins = 20
* Output: 6
* Explanation: The boy can buy all the ice cream bars for a total price of 1 +
*  6 + 3 + 1 + 2 + 5 = 18.
*
*
* Constraints:
*
* * `costs.length == n`
* * `1 <= n <= 105`
* * `1 <= costs[i] <= 105`
* * `1 <= coins <= 108`
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
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable();
        let mut count = 0;
        for cost in costs {
            if coins < cost {
                break;
            }
            coins -= cost;
            count += 1;
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
        let costs = vec![1, 3, 2, 4, 1];
        let coins = 7;
        let expected = 4;
        assert_eq!(Solution::max_ice_cream(costs, coins), expected);
    }

    #[test]
    fn example_2() {
        let costs = vec![10, 6, 8, 7, 7, 8];
        let coins = 5;
        let expected = 0;
        assert_eq!(Solution::max_ice_cream(costs, coins), expected);
    }

    #[test]
    fn example_3() {
        let costs = vec![1, 6, 3, 1, 2, 5];
        let coins = 20;
        let expected = 6;
        assert_eq!(Solution::max_ice_cream(costs, coins), expected);
    }
}
// @leetup=inject:after_code
