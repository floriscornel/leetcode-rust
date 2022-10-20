// @leetup=custom
// @leetup=info id=12 lang=rust slug=integer-to-roman

/*
* Roman numerals are represented by seven different symbols: `I`, `V`, `X`, `L`,
* `C`, `D` and `M`.
*
* Symbol       Value
* I             1
* V             5
* X             10
* L             50
* C             100
* D             500
* M             1000
*
* For example, `2` is written as `II` in Roman numeral, just two one"s added
* together. `12` is written as `XII`, which is simply `X + II`. The number `27` is
* written as `XXVII`, which is `XX + V + II`.
*
* Roman numerals are usually written largest to smallest from left to right.
* However, the numeral for four is not `IIII`. Instead, the number four is written
* as `IV`. Because the one is before the five we subtract it making four. The same
* principle applies to the number nine, which is written as `IX`. There are six
* instances where subtraction is used:
*
* * `I` can be placed before `V` (5) and `X` (10) to make 4 and 9.
* * `X` can be placed before `L` (50) and `C` (100) to make 40 and 90.
* * `C` can be placed before `D` (500) and `M` (1000) to make 400 and 900.
*
* Given an integer, convert it to a roman numeral.
*
*
* Example 1:
*
* Input: num = 3
* Output: "III"
* Explanation: 3 is represented as 3 ones.
*
* Example 2:
*
* Input: num = 58
* Output: "LVIII"
* Explanation: L = 50, V = 5, III = 3.
*
* Example 3:
*
* Input: num = 1994
* Output: "MCMXCIV"
* Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
*
*
* Constraints:
*
* * `1 <= num <= 3999`
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
    pub fn int_to_roman(mut num: i32) -> String {
        let mut output = String::new();
        while num > 0 {
            let (substract, append) = match num {
                1000..=3999 => (1000, "M"),
                900..=999 => (900, "CM"),
                500..=899 => (500, "D"),
                400..=499 => (400, "CD"),
                100..=399 => (100, "C"),
                90..=99 => (90, "XC"),
                50..=89 => (50, "L"),
                40..=49 => (40, "XL"),
                10..=39 => (10, "X"),
                9 => (9, "IX"),
                5..=8 => (5, "V"),
                4 => (4, "IV"),
                1..=3 => (1, "I"),
                _ => panic!("Unaccepted input value"),
            };
            num -= substract;
            output.push_str(append);
        }
        output
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::int_to_roman(3), "III".to_owned())
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_owned())
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_owned())
    }
}
// @leetup=inject:after_code
