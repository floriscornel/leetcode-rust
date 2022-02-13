#![allow(dead_code, unused_variables)]
struct Solution {}

/** Search a 2D Matrix - https://leetcode.com/problems/search-a-2d-matrix/ */

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut x, mut y) = (0, 0);
        let (m, n) = (matrix.len(), matrix[0].len());
        while x < n && y < m {
            if matrix[y][x] == target {
                return true;
            } else if y + 1 < m && matrix[y + 1][x] <= target {
                y += 1;
            } else if x + 1 < n && matrix[y][x + 1] <= target {
                x += 1;
            } else {
                return false;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn search_matrix() {
        assert!(super::Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ));
        assert!(!super::Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        ));
    }
}
