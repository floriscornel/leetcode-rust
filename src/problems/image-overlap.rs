// @leetup=custom
// @leetup=info id=835 lang=rust slug=image-overlap

/*
* You are given two images, `img1` and `img2`, represented as binary, square
* matrices of size `n x n`. A binary matrix has only `0`s and `1`s as values.
*
* We translate one image however we choose by sliding all the `1` bits left,
* right, up, and/or down any number of units. We then place it on top of the other
* image. We can then calculate the overlap by counting the number of positions
* that have a `1` in both images.
*
* Note also that a translation does not include any kind of rotation. Any `1`
* bits that are translated outside of the matrix borders are erased.
*
* Return *the largest possible overlap*.
*
*
* Example 1:
*
* []
* Input: img1 = [[1,1,0],[0,1,0],[0,1,0]], img2 = [[0,0,0],[0,1,1],[0,0,1]]
* Output: 3
* Explanation: We translate img1 to right by 1 unit and down by 1 unit.
* []
* The number of positions that have a 1 in both images is 3 (shown in red).
* []
*
* Example 2:
*
* Input: img1 = [[1]], img2 = [[1]]
* Output: 1
*
* Example 3:
*
* Input: img1 = [[0]], img2 = [[0]]
* Output: 0
*
*
* Constraints:
*
* * `n == img1.length == img1[i].length`
* * `n == img2.length == img2[i].length`
* * `1 <= n <= 30`
* * `img1[i][j]` is either `0` or `1`.
* * `img2[i][j]` is either `0` or `1`.
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
    pub fn convert_to_bits(vec: Vec<Vec<i32>>) -> Vec<u32> {
        vec.iter()
            .map(|row| {
                let mut result: u32 = 0;
                row.iter()
                    .map(|y| *y as u32)
                    .collect::<Vec<u32>>()
                    .iter()
                    .for_each(|&bit| {
                        result <<= 1;
                        result ^= bit;
                    });
                result
            })
            .collect::<Vec<u32>>()
    }

    pub fn overlap(a: &[u32], b: &[u32], x_shift: usize, y_shift: usize) -> i32 {
        let (mut top_left, mut top_right, mut bottom_left, mut bottom_right) = (0, 0, 0, 0);
        for i in 0..a.len() {
            let (left_idx, right_idx) = (i as i32 - y_shift as i32, i + y_shift);
            if left_idx >= 0 {
                top_left += (a[i] & b[left_idx as usize] << x_shift).count_ones();
                bottom_left += (a[i] & b[i - y_shift] >> x_shift).count_ones();
            }
            if right_idx < a.len() {
                top_right += (a[i] & b[i + y_shift] << x_shift).count_ones();
                bottom_right += (a[i] & b[i + y_shift] >> x_shift).count_ones();
            }
        }
        top_left.max(top_right).max(bottom_left).max(bottom_right) as i32
    }

    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let (mask1, mask2) = (
            Solution::convert_to_bits(img1),
            Solution::convert_to_bits(img2),
        );
        let mut max_overlap = 0;
        for i in 0..mask1.len() {
            for j in 0..mask2.len() {
                max_overlap = max_overlap.max(Solution::overlap(&mask1, &mask2, i, j));
            }
        }
        max_overlap
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
            Solution::largest_overlap(
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
            ),
            3
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::largest_overlap(vec![vec![1]], vec![vec![1]]), 1)
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::largest_overlap(vec![vec![1]], vec![vec![0]]), 0)
    }
}
// @leetup=inject:after_code
