// @leetup=custom
// @leetup=info id=223 lang=rust slug=rectangle-area

/*
* Given the coordinates of two rectilinear rectangles in a 2D plane, return
* *the total area covered by the two rectangles*.
*
* The first rectangle is defined by its bottom-left corner `(ax1, ay1)` and
* its top-right corner `(ax2, ay2)`.
*
* The second rectangle is defined by its bottom-left corner `(bx1, by1)` and
* its top-right corner `(bx2, by2)`.
*
*
* Example 1:
*
* [Rectangle Area]
* Input: ax1 = -3, ay1 = 0, ax2 = 3, ay2 = 4, bx1 = 0, by1 = -1, bx2 = 9, by2
* = 2
* Output: 45
*
* Example 2:
*
* Input: ax1 = -2, ay1 = -2, ax2 = 2, ay2 = 2, bx1 = -2, by1 = -2, bx2 = 2, by
* 2 = 2
* Output: 16
*
*
* Constraints:
*
* * `-104 <= ax1 <= ax2 <= 104`
* * `-104 <= ay1 <= ay2 <= 104`
* * `-104 <= bx1 <= bx2 <= 104`
* * `-104 <= by1 <= by2 <= 104`
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
    #[allow(clippy::too_many_arguments)]
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let a = (ax2 - ax1) * (ay2 - ay1);
        let b = (bx2 - bx1) * (by2 - by1);
        let overlap_x = ax2.min(bx2) - ax1.max(bx1);
        let overlap_y = ay2.min(by2) - ay1.max(by1);
        a + b - (overlap_x.max(0) * overlap_y.max(0))
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::compute_area(-5, -5, -4, 0, -3, -3, 3, 3), 41);
    }
}
// @leetup=inject:after_code
