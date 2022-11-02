// @leetup=custom
// @leetup=info id=433 lang=rust slug=minimum-genetic-mutation

/*
* A gene string can be represented by an 8-character long string, with choices
* from `'A'`, `'C'`, `'G'`, and `'T'`.
*
* Suppose we need to investigate a mutation from a gene string `start` to a gene
* string `end` where one mutation is defined as one single character changed in
* the gene string.
*
* * For example, `"AACCGGTT" --> "AACCGGTA"` is one mutation.
*
* There is also a gene bank `bank` that records all the valid gene mutations. A
* gene must be in `bank` to make it a valid gene string.
*
* Given the two gene strings `start` and `end` and the gene bank `bank`, return
* *the minimum number of mutations needed to mutate from *`start`* to *`end`. If
* there is no such a mutation, return `-1`.
*
* Note that the starting point is assumed to be valid, so it might not be included
* in the bank.
*
*
* Example 1:
*
* Input: start = "AACCGGTT", end = "AACCGGTA", bank = ["AACCGGTA"]
* Output: 1
*
* Example 2:
*
* Input: start = "AACCGGTT", end = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","
* AAACGGTA"]
* Output: 2
*
* Example 3:
*
* Input: start = "AAAAACCC", end = "AACCCCCC", bank = ["AAAACCCC","AAACCCCC","
* AACCCCCC"]
* Output: 3
*
*
* Constraints:
*
* * `start.length == 8`
* * `end.length == 8`
* * `0 <= bank.length <= 10`
* * `bank[i].length == 8`
* * `start`, `end`, and `bank[i]` consist of only the characters `['A', 'C', 'G',
*   'T']`.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]

struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::HashSet;

type Gene = [u8; 8];
impl Solution {
    pub fn str_to_gene(str: &String) -> Gene {
        let bytes = str.as_bytes();
        let mut res = [0; 8];
        res.clone_from_slice(bytes);
        res
    }

    pub fn is_neighbor(a: &Gene, b: &Gene) -> bool {
        let mut counter = 0;
        for i in 0..a.len() {
            if a[i] != b[i] {
                counter += 1;
                if counter > 1 {
                    return false;
                }
            }
        }
        counter == 1
    }

    pub fn neighbors(target: &Gene, gbank: &HashSet<Gene>) -> HashSet<Gene> {
        let mut neighbors = HashSet::new();
        for potential_neigbor in gbank {
            let is_neighbor = Self::is_neighbor(target, potential_neigbor);
            if is_neighbor {
                neighbors.insert(*potential_neigbor);
            }
        }
        neighbors
    }

    pub fn min_distance(
        current: Gene,
        target: &Gene,
        gbank: HashSet<Gene>,
        current_distance: i32,
    ) -> i32 {
        if current == *target {
            return current_distance;
        }
        let mut min = i32::MAX;
        let neighbors = Self::neighbors(&current, &gbank);
        for neigbor in neighbors {
            let mut new_gbank = gbank.clone();
            new_gbank.remove(&neigbor);
            min = min.min(Self::min_distance(
                neigbor,
                target,
                new_gbank,
                current_distance + 1,
            ))
        }
        min
    }

    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut gbank = HashSet::new();
        for gene in bank {
            gbank.insert(Self::str_to_gene(&gene));
        }
        let min = Self::min_distance(
            Self::str_to_gene(&start),
            &Self::str_to_gene(&end),
            gbank,
            0,
        );
        if min == i32::MAX {
            -1
        } else {
            min
        }
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
            Solution::min_mutation(
                "AACCGGTT".to_owned(),
                "AACCGGTA".to_owned(),
                vec!["AACCGGTA".to_owned()]
            ),
            1
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_owned(),
                "AAACGGTA".to_owned(),
                vec![
                    "AACCGGTA".to_owned(),
                    "AACCGCTA".to_owned(),
                    "AAACGGTA".to_owned()
                ]
            ),
            2
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::min_mutation(
                "AAAAACCC".to_owned(),
                "AACCCCCC".to_owned(),
                vec![
                    "AAAACCCC".to_owned(),
                    "AAACCCCC".to_owned(),
                    "AACCCCCC".to_owned()
                ]
            ),
            3
        )
    }
}
// @leetup=inject:after_code
