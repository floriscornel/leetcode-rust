// @leetup=custom
// @leetup=info id=127 lang=rust slug=word-ladder

/*
* A transformation sequence from word `beginWord` to word `endWord` using a
* dictionary `wordList` is a sequence of words `beginWord -> s1 -> s2 -> ... ->
* sk` such that:
*
* * Every adjacent pair of words differs by a single letter.
* * Every `si` for `1 <= i <= k` is in `wordList`. Note that `beginWord` does not
*   need to be in `wordList`.
* * `sk == endWord`
*
* Given two words, `beginWord` and `endWord`, and a dictionary `wordList`, return
* *the number of words in the shortest transformation sequence from*
* `beginWord` *to* `endWord`*, or *`0`* if no such sequence exists.*
*
*
* Example 1:
*
* Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lo
* t","log","cog"]
* Output: 5
* Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot"
*  -> "dog" -> cog", which is 5 words long.
*
* Example 2:
*
* Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lo
* t","log"]
* Output: 0
* Explanation: The endWord "cog" is not in wordList, therefore there is no val
* id transformation sequence.
*
*
* Constraints:
*
* * `1 <= beginWord.length <= 10`
* * `endWord.length == beginWord.length`
* * `1 <= wordList.length <= 5000`
* * `wordList[i].length == beginWord.length`
* * `beginWord`, `endWord`, and `wordList[i]` consist of lowercase English
*   letters.
* * `beginWord != endWord`
* * All the words in `wordList` are unique.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    fn neighbor(a: &[u8], b: &[u8]) -> bool {
        let mut difference_found = false;
        for i in 0..a.len() {
            if a[i] != b[i] {
                if difference_found {
                    return false;
                } else {
                    difference_found = true;
                }
            }
        }
        difference_found
    }

    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut neighbors: HashMap<&String, Vec<&String>> = HashMap::with_capacity(word_list.len());
        for (i, a) in word_list.iter().enumerate() {
            for b in word_list.iter().skip(i) {
                if Solution::neighbor(a.as_bytes(), b.as_bytes()) {
                    let na = neighbors.entry(a).or_insert_with(Vec::new);
                    na.push(b);
                    let nb = neighbors.entry(b).or_insert_with(Vec::new);
                    nb.push(a);
                }
            }
            if Solution::neighbor(begin_word.as_bytes(), a.as_bytes()) {
                let begin = neighbors.entry(&begin_word).or_insert_with(Vec::new);
                begin.push(a);
            }
        }
        let mut q: VecDeque<(i32, &String)> = VecDeque::from(vec![(1, &begin_word)]);
        let mut visited: HashSet<&String> = HashSet::new();
        while !q.is_empty() {
            let (mut len, word) = q.pop_front().unwrap();
            len += 1;
            if let Some(found_neighbors) = neighbors.get(&word) {
                for neighbor in found_neighbors {
                    if **neighbor == end_word {
                        return len;
                    }
                    if !visited.contains(*neighbor) {
                        q.push_back((len, *neighbor));
                        visited.insert(*neighbor);
                    }
                }
            }
        }
        0
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec!["hot", "dot", "dog", "lot", "log", "cog"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);
    }

    #[test]
    fn example_2() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec!["hot", "dot", "dog", "lot", "log"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
    }

    #[test]
    fn example_3() {
        let begin_word = "hot".to_string();
        let end_word = "dog".to_string();
        let word_list = vec!["hot", "dog"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
    }
}
// @leetup=inject:after_code
