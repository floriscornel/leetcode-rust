// @leetup=custom
// @leetup=info id=212 lang=rust slug=word-search-ii

/*
* Given an `m x n` `board` of characters and a list of strings `words`, return
* *all words on the board*.
*
* Each word must be constructed from letters of sequentially adjacent cells, where
* adjacent cells are horizontally or vertically neighboring. The same letter
* cell may not be used more than once in a word.
*
*
* Example 1:
*
* []
* Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","
* f","l","v"]], words = ["oath","pea","eat","rain"]
* Output: ["eat","oath"]
*
* Example 2:
*
* []
* Input: board = [["a","b"],["c","d"]], words = ["abcb"]
* Output: []
*
*
* Constraints:
*
* * `m == board.length`
* * `n == board[i].length`
* * `1 <= m, n <= 12`
* * `board[i][j]` is a lowercase English letter.
* * `1 <= words.length <= 3 * 104`
* * `1 <= words[i].length <= 10`
* * `words[i]` consists of lowercase English letters.
* * All the strings of `words` are unique.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]

struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::{HashMap, HashSet};

struct TrieStruct {
    root_node: TrieNode,
}

struct TrieNode {
    is_final: bool,
    child_nodes: HashMap<char, TrieNode>,
}

impl TrieNode {
    pub fn new(is_final: bool) -> TrieNode {
        TrieNode {
            is_final,
            child_nodes: HashMap::new(),
        }
    }
}

impl TrieStruct {
    pub fn new() -> TrieStruct {
        TrieStruct {
            root_node: TrieNode::new(false),
        }
    }
    pub fn insert(&mut self, string_val: String) {
        let mut node = &mut self.root_node;
        for letter in string_val.chars() {
            node = node
                .child_nodes
                .entry(letter)
                .or_insert_with(|| TrieNode::new(false));
        }
        node.is_final = true;
    }
    pub fn remove(&mut self, string_val: &str) {
        let mut node = &mut self.root_node;
        for (idx, letter) in string_val.char_indices() {
            if idx == string_val.len() - 1 {
                if let Some(next) = node.child_nodes.get(&letter) {
                    if next.child_nodes.is_empty() {
                        node.child_nodes.remove(&letter);
                    }
                }
            } else if let Some(next) = node.child_nodes.get_mut(&letter) {
                node = next;
            } else {
                return;
            }
        }
    }
}

type TravelSequence = Vec<(usize, usize)>;

impl Solution {
    fn get_options(sequence: &TravelSequence, m: usize, n: usize) -> TravelSequence {
        let &(i, j) = sequence.last().unwrap();
        let mut options: TravelSequence = Vec::new();
        if i > 0 && !sequence.contains(&(i - 1, j)) {
            options.push((i - 1, j));
        }
        if j > 0 && !sequence.contains(&(i, j - 1)) {
            options.push((i, j - 1));
        }
        if i + 1 < m && !sequence.contains(&(i + 1, j)) {
            options.push((i + 1, j));
        }
        if j + 1 < n && !sequence.contains(&(i, j + 1)) {
            options.push((i, j + 1));
        }
        options
    }

    pub fn follow_sequence(
        board: &[Vec<char>],
        node: &TrieNode,
        mut sequence: TravelSequence,
        mut word: String,
    ) -> Option<Vec<String>> {
        let &(i, j) = sequence.last().unwrap();
        if let Some(next) = node.child_nodes.get(&board[i][j]) {
            let mut words = Vec::<String>::new();
            word.push(board[i][j]);
            if next.is_final {
                words.push(word.clone());
            }
            for option in Self::get_options(&sequence, board.len(), board[0].len()) {
                sequence.push(option);
                if let Some(mut found) =
                    Self::follow_sequence(board, next, sequence.clone(), word.clone())
                {
                    words.append(&mut found);
                }
                sequence.pop();
            }
            Some(words)
        } else {
            None
        }
    }
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let (m, n, mut k) = (board.len(), board[0].len(), 0);
        let (mut trie, mut answer) = (TrieStruct::new(), HashSet::<String>::new());
        for word in words {
            k = k.max(word.len());
            trie.insert(word);
        }
        for (i, row) in board.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if let Some(found) =
                    Self::follow_sequence(&board, &trie.root_node, vec![(i, j)], "".to_owned())
                {
                    for word in found {
                        trie.remove(&word);
                        answer.insert(word);
                    }
                }
            }
        }
        answer.into_iter().collect()
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Solution;

    #[test]
    fn example_1() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = vec![
            "oath".to_owned(),
            "pea".to_owned(),
            "eat".to_owned(),
            "rain".to_owned(),
        ];
        let output: HashSet<String> = vec!["eat".to_owned(), "oath".to_owned()]
            .into_iter()
            .collect();
        let answer: HashSet<String> = Solution::find_words(board, words).into_iter().collect();
        assert_eq!(answer, output)
    }

    #[test]
    fn example_2() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let words = vec!["abcb".to_owned()];
        let output: HashSet<String> = vec![].into_iter().collect();
        let answer: HashSet<String> = Solution::find_words(board, words).into_iter().collect();
        assert_eq!(answer, output)
    }

    #[test]
    fn example_3() {
        let board = vec![vec!['a'], vec!['a']];
        let words = vec!["aaa".to_owned()];
        let output: HashSet<String> = vec![].into_iter().collect();
        let answer: HashSet<String> = Solution::find_words(board, words).into_iter().collect();
        assert_eq!(answer, output)
    }

    #[test]
    fn example_4() {
        let board = vec![
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
        ];
        let words = vec![
            "a".to_owned(),
            "aa".to_owned(),
            "aaa".to_owned(),
            "aaaa".to_owned(),
            "aaaaa".to_owned(),
            "aaaaaa".to_owned(),
            "aaaaaaa".to_owned(),
            "aaaaaaaa".to_owned(),
            "aaaaaaaaa".to_owned(),
            "aaaaaaaaaa".to_owned(),
        ];
        let output: HashSet<String> = vec![
            "a".to_owned(),
            "aa".to_owned(),
            "aaa".to_owned(),
            "aaaa".to_owned(),
            "aaaaa".to_owned(),
            "aaaaaa".to_owned(),
            "aaaaaaa".to_owned(),
            "aaaaaaaa".to_owned(),
            "aaaaaaaaa".to_owned(),
            "aaaaaaaaaa".to_owned(),
        ]
        .into_iter()
        .collect();
        let answer: HashSet<String> = Solution::find_words(board, words).into_iter().collect();
        assert_eq!(answer, output)
    }
}
// @leetup=inject:after_code
