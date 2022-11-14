// @leetup=custom
// @leetup=info id=947 lang=rust slug=most-stones-removed-with-same-row-or-column

/*
* On a 2D plane, we place `n` stones at some integer coordinate points. Each
* coordinate point may have at most one stone.
*
* A stone can be removed if it shares either the same row or the same column
* as another stone that has not been removed.
*
* Given an array `stones` of length `n` where `stones[i] = [xi, yi]` represents
* the location of the `ith` stone, return *the largest possible number of stones
* that can be removed*.
*
*
* Example 1:
*
* Input: stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]
* Output: 5
* Explanation: One way to remove 5 stones is as follows:
* 1. Remove stone [2,2] because it shares the same row as [2,1].
* 2. Remove stone [2,1] because it shares the same column as [0,1].
* 3. Remove stone [1,2] because it shares the same row as [1,0].
* 4. Remove stone [1,0] because it shares the same column as [0,0].
* 5. Remove stone [0,1] because it shares the same row as [0,0].
* Stone [0,0] cannot be removed since it does not share a row/column with another
* stone still on the plane.
*
* Example 2:
*
* Input: stones = [[0,0],[0,2],[1,1],[2,0],[2,2]]
* Output: 3
* Explanation: One way to make 3 moves is as follows:
* 1. Remove stone [2,2] because it shares the same row as [2,0].
* 2. Remove stone [2,0] because it shares the same column as [0,0].
* 3. Remove stone [0,2] because it shares the same row as [0,0].
* Stones [0,0] and [1,1] cannot be removed since they do not share a row/column wi
* th another stone still on the plane.
*
* Example 3:
*
* Input: stones = [[0,0]]
* Output: 0
* Explanation: [0,0] is the only stone on the plane, so you cannot remove it.
*
*
* Constraints:
*
* * `1 <= stones.length <= 1000`
* * `0 <= xi, yi <= 104`
* * No two stones are at the same coordinate point.
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

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut removed_count = 0;
        let mut parent = (0..stones.len()).collect::<Vec<usize>>();
        let (mut rows, mut cols) = (HashMap::<i32, usize>::new(), HashMap::<i32, usize>::new());
        let mut union = |i, j| {
            let (ir, jr) = (Self::find(i, &mut parent), Self::find(j, &mut parent));
            parent[ir] = jr;
            i32::from(ir != jr)
        };

        for (idx, (x, y)) in stones.iter().map(|x| (x[0], x[1])).enumerate() {
            if let Some(&j) = rows.get(&x) {
                removed_count += union(idx, j);
            } else {
                rows.insert(x, idx);
            }
            if let Some(&j) = cols.get(&y) {
                removed_count += union(idx, j);
            } else {
                cols.insert(y, idx);
            }
        }
        removed_count
    }

    fn find(x: usize, parent: &mut [usize]) -> usize {
        if x != parent[x] {
            parent[x] = Self::find(parent[x], parent);
        }
        parent[x]
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let stones = vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2],
        ];
        assert_eq!(Solution::remove_stones(stones), 5);
    }

    #[test]
    fn example_2() {
        let stones = vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]];
        assert_eq!(Solution::remove_stones(stones), 3);
    }
}
// @leetup=inject:after_code
