// @leetup=custom
// @leetup=info id=134 lang=rust slug=gas-station

/*
* There are `n` gas stations along a circular route, where the amount of gas at
* the `ith` station is `gas[i]`.
*
* You have a car with an unlimited gas tank and it costs `cost[i]` of gas to
* travel from the `ith` station to its next `(i + 1)th` station. You begin the
* journey with an empty tank at one of the gas stations.
*
* Given two integer arrays `gas` and `cost`, return *the starting gas station's
* index if you can travel around the circuit once in the clockwise direction,
* otherwise return* `-1`. If there exists a solution, it is guaranteed to be
* unique
*
*
* Example 1:
*
* Input: gas = [1,2,3,4,5], cost = [3,4,5,1,2]
* Output: 3
* Explanation:
* Start at station 3 (index 3) and fill up with 4 unit of gas. Your tank = 0 + 4 =
*  4
* Travel to station 4. Your tank = 4 - 1 + 5 = 8
* Travel to station 0. Your tank = 8 - 2 + 1 = 7
* Travel to station 1. Your tank = 7 - 3 + 2 = 6
* Travel to station 2. Your tank = 6 - 4 + 3 = 5
* Travel to station 3. The cost is 5. Your gas is just enough to travel back to st
* ation 3.
* Therefore, return 3 as the starting index.
*
* Example 2:
*
* Input: gas = [2,3,4], cost = [3,4,3]
* Output: -1
* Explanation:
* You can't start at station 0 or 1, as there is not enough gas to travel to the n
* ext station.
* Let's start at station 2 and fill up with 4 unit of gas. Your tank = 0 + 4 = 4
* Travel to station 0. Your tank = 4 - 3 + 2 = 3
* Travel to station 1. Your tank = 3 - 3 + 3 = 3
* You cannot travel back to station 2, as it requires 4 unit of gas but you only h
* ave 3.
* Therefore, you can't travel around the circuit once no matter where you start.
*
*
* Constraints:
*
* * `n == gas.length == cost.length`
* * `1 <= n <= 105`
* * `0 <= gas[i], cost[i] <= 104`
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
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let (mut gas_in_tank, mut missing_gas, mut ideal_start_position) = (0, 0, 0);
        for (station_index, (station_gas, station_cost)) in
            gas.into_iter().zip(cost.into_iter()).enumerate()
        {
            gas_in_tank += station_gas - station_cost;
            if gas_in_tank < 0 {
                missing_gas += gas_in_tank;
                gas_in_tank = 0;
                ideal_start_position = (station_index + 1) % n;
            }
        }
        if gas_in_tank + missing_gas >= 0 {
            ideal_start_position as i32
        } else {
            -1
        }
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 3);
    }

    #[test]
    fn example_2() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        assert_eq!(Solution::can_complete_circuit(gas, cost), -1);
    }
}
// @leetup=inject:after_code
