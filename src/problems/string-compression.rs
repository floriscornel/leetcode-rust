// @leetup=custom
// @leetup=info id=443 lang=rust slug=string-compression

/*
* Given an array of characters `chars`, compress it using the following algorithm:
*
* Begin with an empty string `s`. For each group of consecutive repeating
* characters in `chars`:
*
* * If the group's length is `1`, append the character to `s`.
* * Otherwise, append the character followed by the group's length.
*
* The compressed string `s` should not be returned separately, but instead, be
* stored in the input character array `chars`. Note that group lengths that
* are `10` or longer will be split into multiple characters in `chars`.
*
* After you are done modifying the input array, return *the new length of the
* array*.
*
* You must write an algorithm that uses only constant extra space.
*
*
* Example 1:
*
* Input: chars = ['a','a','b','b','c','c','c']
* Output: Return 6, and the first 6 characters of the input array should be: [
* 'a','2','b','2','c','3']
* Explanation: The groups are 'aa', 'bb', and 'ccc'. This compresses to 'a2b2c
* 3'.
*
* Example 2:
*
* Input: chars = ['a']
* Output: Return 1, and the first character of the input array should be: ['a'
* ]
* Explanation: The only group is 'a', which remains uncompressed since it's a
* single character.
*
* Example 3:
*
* Input: chars = ['a','b','b','b','b','b','b','b','b','b','b','b','b']
* Output: Return 4, and the first 4 characters of the input array should be: [
* 'a','b','1','2'].
* Explanation: The groups are 'a' and 'bbbbbbbbbbbb'. This compresses to 'ab12
* '.
*
*
* Constraints:
*
* * `1 <= chars.length <= 2000`
* * `chars[i]` is a lowercase English letter, uppercase English letter, digit, or
*   symbol.
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
    pub fn write(chars: &mut [char], write_idx: &mut usize, char: char, count: i32) {
        chars[*write_idx] = char;
        *write_idx += 1;
        if count > 1 {
            for c in count.to_string().as_bytes() {
                chars[*write_idx] = *c as char;
                *write_idx += 1;
            }
        }
    }

    pub fn compress(chars: &mut [char]) -> i32 {
        let (mut read_idx, mut write_idx, mut last_char, mut count) = (0, 0, chars[0], 0);
        while read_idx < chars.len() {
            if last_char == chars[read_idx] {
                count += 1;
            } else {
                Self::write(chars, &mut write_idx, last_char, count);
                last_char = chars[read_idx];
                count = 1;
            }
            read_idx += 1;
        }
        Self::write(chars, &mut write_idx, last_char, count);
        write_idx as i32
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut input = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(Solution::compress(&mut input), 6);
        assert_eq!(input[0..6], vec!['a', '2', 'b', '2', 'c', '3']);
    }

    #[test]
    fn example_2() {
        let mut input = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        assert_eq!(Solution::compress(&mut input), 4);
        assert_eq!(input[0..4], vec!['a', 'b', '1', '2']);
    }

    #[test]
    fn example_3() {
        let mut input = vec!['a'];
        assert_eq!(Solution::compress(&mut input), 1);
        assert_eq!(input, vec!['a',]);
    }
}
// @leetup=inject:after_code
