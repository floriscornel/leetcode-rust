// @leetup=custom
// @leetup=info id=38 lang=rust slug=count-and-say

/*
* The count-and-say sequence is a sequence of digit strings defined by the
* recursive formula:
*
* * `countAndSay(1) = "1"`
* * `countAndSay(n)` is the way you would "say" the digit string from
*   `countAndSay(n-1)`, which is then converted into a different digit string.
*
* To determine how you "say" a digit string, split it into the minimal number
* of substrings such that each substring contains exactly one unique digit.
* Then for each substring, say the number of digits, then say the digit. Finally,
* concatenate every said digit.
*
* For example, the saying and conversion for digit string `"3322251"`:
*
* []
*
* Given a positive integer `n`, return *the *`nth`* term of the count-and-say
* sequence*.
*
*
* Example 1:
*
* Input: n = 1
* Output: "1"
* Explanation: This is the base case.
*
* Example 2:
*
* Input: n = 4
* Output: "1211"
* Explanation:
* countAndSay(1) = "1"
* countAndSay(2) = say "1" = one 1 = "11"
* countAndSay(3) = say "11" = two 1's = "21"
* countAndSay(4) = say "21" = one 2 + one 1 = "12" + "11" = "1211"
*
*
* Constraints:
*
* * `1 <= n <= 30`
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
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_owned();
        }

        // Convert to sequence of digits
        let digit_vector = Solution::count_and_say(n - 1)
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        // Reduce same digits
        let (mut output, mut current, mut counter) = (Vec::new(), 0, 0);
        for digit in digit_vector {
            if digit == current {
                counter += 1;
            } else {
                if counter > 0 {
                    output.push(counter);
                    output.push(current);
                }
                current = digit;
                counter = 1;
            }
        }
        output.push(counter);
        output.push(current);

        // Return output as string
        output
            .into_iter()
            .map(|i| i.to_string())
            .collect::<String>()
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::count_and_say(1), "1".to_owned())
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::count_and_say(4), "1211".to_owned())
    }
}
// @leetup=inject:after_code
