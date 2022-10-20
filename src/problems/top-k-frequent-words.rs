// @leetup=custom
// @leetup=info id=692 lang=rust slug=top-k-frequent-words

/*
* Given an array of strings `words` and an integer `k`, return *the *`k`* most
* frequent strings*.
*
* Return the answer sorted by the frequency from highest to lowest. Sort
* the words with the same frequency by their lexicographical order.
*
*
* Example 1:
*
* Input: words = ["i","love","leetcode","i","love","coding"], k = 2
* Output: ["i","love"]
* Explanation: "i" and "love" are the two most frequent words.
* Note that "i" comes before "love" due to a lower alphabetical order.
*
* Example 2:
*
* Input: words = ["the","day","is","sunny","the","the","the","sunny","is","is"
* ], k = 4
* Output: ["the","is","sunny","day"]
* Explanation: "the", "is", "sunny" and "day" are the four most frequent words
* , with the number of occurrence being 4, 3, 2 and 1 respectively.
*
*
* Constraints:
*
* * `1 <= words.length <= 500`
* * `1 <= words[i].length <= 10`
* * `words[i]` consists of lowercase English letters.
* * `k` is in the range `[1, The number of unique words[i]]`
*
*
* Follow-up: Could you solve it in `O(n log(k))` time and `O(n)` extra space?
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]

struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::HashMap;
use std::iter::FromIterator;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut map = HashMap::<String, u32>::new();

        for word in words {
            map.entry(word).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut vec = Vec::from_iter(map.iter());

        vec.sort_by(|(a_word, a_count), (b_word, b_count)| {
            if a_count == b_count {
                b_word.cmp(a_word)
            } else {
                a_count.cmp(b_count)
            }
        });

        vec.iter()
            .rev()
            .take(k as usize)
            .map(|(word, count)| (*word).clone())
            .collect()
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let words = vec![
            "i", "love", "leetcode", "i", "love", "coding", "aa", "aa", "zz", "zz", "a", "a",
        ];
        let k = 5;
        let output = vec!["a", "aa", "i", "love", "zz"];
        assert_eq!(
            Solution::top_k_frequent(
                words
                    .iter()
                    .map(|x| (*x).to_owned())
                    .collect::<Vec<String>>(),
                k
            ),
            output
        );
    }

    #[test]
    fn example_2() {
        let words = vec![
            "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
        ];
        let k = 4;
        let output = vec!["the", "is", "sunny", "day"];
        assert_eq!(
            Solution::top_k_frequent(
                words
                    .iter()
                    .map(|x| (*x).to_owned())
                    .collect::<Vec<String>>(),
                k
            ),
            output
        );
    }
}
// @leetup=inject:after_code
