// @leetup=custom
// @leetup=info id=991 lang=rust slug=broken-calculator

/*
* There is a broken calculator that has the integer `startValue` on its display
* initially. In one operation, you can:
*
* * multiply the number on display by `2`, or
* * subtract `1` from the number on display.
*
* Given two integers `startValue` and `target`, return *the minimum number of
* operations needed to display *`target`* on the calculator*.
*
*
* Example 1:
*
* Input: startValue = 2, target = 3
* Output: 2
* Explanation: Use double operation and then decrement operation {2 -> 4 -> 3}
* .
*
* Example 2:
*
* Input: startValue = 5, target = 8
* Output: 2
* Explanation: Use decrement and then double {5 -> 4 -> 8}.
*
* Example 3:
*
* Input: startValue = 3, target = 10
* Output: 3
* Explanation: Use double, decrement and double {3 -> 6 -> 5 -> 10}.
*
*
* Constraints:
*
* * `1 <= x, y <= 109`
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
    pub fn broken_calc(start_value: i32, mut target: i32) -> i32 {
        let mut counter = 0;
        while target > start_value {
            counter += 1;
            if target % 2 == 0 {
                target /= 2;
            } else {
                target += 1;
            }
        }
        counter + start_value - target
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::broken_calc(2, 3), 2)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::broken_calc(5, 8), 2)
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::broken_calc(3, 10), 3)
    }
}
// @leetup=inject:after_code
