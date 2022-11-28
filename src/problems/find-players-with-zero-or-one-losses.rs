#![allow(dead_code, unused_variables)]
struct Solution {}

use std::collections::BTreeSet;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut always_winners = BTreeSet::<i32>::new();
        let mut losers = BTreeSet::<i32>::new();
        let mut one_loss = BTreeSet::<i32>::new();
        for vec in matches {
            let (winner, loser) = (vec[0], vec[1]);
            if !losers.contains(&winner) {
                always_winners.insert(winner);
            }
            always_winners.remove(&loser);

            if one_loss.contains(&loser) {
                one_loss.remove(&loser);
            } else if !losers.contains(&loser) {
                one_loss.insert(loser);
            }

            losers.insert(loser);
        }
        vec![
            always_winners.into_iter().collect::<_>(),
            one_loss.into_iter().collect::<_>(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        let output = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];
        assert_eq!(Solution::find_winners(matches), output);
    }

    #[test]
    fn example_2() {
        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        let output = vec![vec![1, 2, 5, 6], vec![]];
        assert_eq!(Solution::find_winners(matches), output);
    }
}
