// @leetup=custom
// @leetup=info id=402 lang=rust slug=remove-k-digits

/*
* Given string num representing a non-negative integer `num`, and an integer `k`,
* return *the smallest possible integer after removing* `k` *digits from* `num`.
*
*
* Example 1:
*
* Input: num = "1432219", k = 3
* Output: "1219"
* Explanation: Remove the three digits 4, 3, and 2 to form the new number 1219
*  which is the smallest.
*
* Example 2:
*
* Input: num = "10200", k = 1
* Output: "200"
* Explanation: Remove the leading 1 and the number is 200. Note that the outpu
* t must not contain leading zeroes.
*
* Example 3:
*
* Input: num = "10", k = 2
* Output: "0"
* Explanation: Remove all the digits from the number and it is left with nothi
* ng which is 0.
*
*
* Constraints:
*
* * `1 <= k <= num.length <= 105`
* * `num` consists of only digits.
* * `num` does not have any leading zeros except for the zero itself.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables, clippy::comparison_chain)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

#[derive(Debug, Clone)]
struct State<'a> {
    nums: &'a Vec<u8>,
    ignored: Vec<usize>,
    remainer: i32,
}

impl<'a> State<'a> {
    fn is_smaller(&self, state: &State) -> bool {
        let (mut i, mut j) = (0, 0);
        while i < self.nums.len() && j < state.nums.len() {
            while self.ignored.contains(&i) {
                i += 1;
            }
            while state.ignored.contains(&j) {
                j += 1;
            }
            if self.nums[i] < state.nums[j] {
                return true;
            } else if self.nums[i] > state.nums[j] {
                return false;
            }
            i += 1;
            j += 1;
        }
        false
    }

    fn clone_removed(&self, index: usize) -> State {
        let mut ignored = self.ignored.clone();
        ignored.push(index);
        State {
            nums: self.nums,
            ignored,
            remainer: self.remainer - 1,
        }
    }
}

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        //Convert "1234" -> vec![1, 2, 3, 4]
        let nums: Vec<u8> = num.into_bytes().iter().map(|x| x - 48).collect();

        // Setup array of states and best state
        let mut states: Vec<State> = Vec::new();
        let mut best_state: State = State {
            nums: &nums,
            ignored: vec![0],
            remainer: k,
        };
        states.push(best_state.clone());

        // Loop over all states
        while !states.is_empty() {
            let state = states.pop().unwrap();
            if state.remainer == 0 {
                if state.is_smaller(&best_state) {
                    best_state = state;
                }
            } else {
                for i in 0..state.nums.len() {
                    states.push(state.clone_removed(i));
                }
            }
        }

        let result: Vec<u8> = best_state.nums.iter().map(|x| x + 48).collect();
        String::from_utf8(result).expect("Found invalid encoding")
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
            Solution::remove_kdigits("1432219".to_string(), 3),
            "1219".to_string()
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::remove_kdigits("10200".to_string(), 1),
            "200".to_string()
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::remove_kdigits("10".to_string(), 2),
            "0".to_string()
        )
    }
}
// @leetup=inject:after_code
