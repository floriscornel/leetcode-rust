#![allow(dead_code, unused_variables)]
struct Solution {}

/** Word Ladder - https://leetcode.com/problems/word-ladder/ */
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

#[cfg(test)]
mod tests {
    #[test]
    fn example_1() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec!["hot", "dot", "dog", "lot", "log", "cog"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(
            super::Solution::ladder_length(begin_word, end_word, word_list),
            5
        );
    }

    #[test]
    fn example_2() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec!["hot", "dot", "dog", "lot", "log"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(
            super::Solution::ladder_length(begin_word, end_word, word_list),
            0
        );
    }

    #[test]
    fn example_3() {
        let begin_word = "hot".to_string();
        let end_word = "dog".to_string();
        let word_list = vec!["hot", "dog"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(
            super::Solution::ladder_length(begin_word, end_word, word_list),
            0
        );
    }
}
