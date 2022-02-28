// @leetup=custom
// @leetup=info id=171 lang=rust slug=excel-sheet-column-number

/*
* Given a string `columnTitle` that represents the column title as appear in an
* Excel sheet, return *its corresponding column number*.
*
* For example:
*
* A -> 1
* B -> 2
* C -> 3
* ...
* Z -> 26
* AA -> 27
* AB -> 28
* ...
*
*
* Example 1:
*
* Input: columnTitle = "A"
* Output: 1
*
* Example 2:
*
* Input: columnTitle = "AB"
* Output: 28
*
* Example 3:
*
* Input: columnTitle = "ZY"
* Output: 701
*
*
* Constraints:
*
* * `1 <= columnTitle.length <= 7`
* * `columnTitle` consists only of uppercase English letters.
* * `columnTitle` is in the range `["A", "FXSHRXW"]`.
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
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result: i32 = 0;
        for char in column_title.chars() {
            result *= 26;
            result += ((char as u8) - 64) as i32;
        }
        result
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
        assert_eq!(Solution::title_to_number("B".to_string()), 2);
        assert_eq!(Solution::title_to_number("Z".to_string()), 26);
        assert_eq!(Solution::title_to_number("AA".to_string()), 27);
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
        assert_eq!(Solution::title_to_number("ZZ".to_string()), 702);
        assert_eq!(Solution::title_to_number("FXSHRXW".to_string()), 2147483647);
    }
}
// @leetup=inject:after_code
