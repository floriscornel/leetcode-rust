// @leetup=custom
// @leetup=info id=885 lang=rust slug=spiral-matrix-iii

/*
* You start at the cell `(rStart, cStart)` of an `rows x cols` grid facing east.
* The northwest corner is at the first row and column in the grid, and the
* southeast corner is at the last row and column.
*
* You will walk in a clockwise spiral shape to visit every position in this grid.
* Whenever you move outside the grid's boundary, we continue our walk outside the
* grid (but may return to the grid boundary later.). Eventually, we reach all
* `rows * cols` spaces of the grid.
*
* Return *an array of coordinates representing the positions of the grid in the
* order you visited them*.
*
*
* Example 1:
*
* []
* Input: rows = 1, cols = 4, rStart = 0, cStart = 0
* Output: [[0,0],[0,1],[0,2],[0,3]]
*
* Example 2:
*
* []
* Input: rows = 5, cols = 6, rStart = 1, cStart = 4
* Output: [[1,4],[1,5],[2,5],[2,4],[2,3],[1,3],[0,3],[0,4],[0,5],[3,5],[3,4],[
* 3,3],[3,2],[2,2],[1,2],[0,2],[4,5],[4,4],[4,3],[4,2],[4,1],[3,1],[2,1],[1,1],[0,
* 1],[4,0],[3,0],[2,0],[1,0],[0,0]]
*
*
* Constraints:
*
* * `1 <= rows, cols <= 100`
* * `0 <= rStart < rows`
* * `0 <= cStart < cols`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
#[derive(PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
impl Direction {
    pub fn next(&self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn next_point(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Up => Point {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let n = i32::max(r_start + rows, c_start + cols) * 2;

        let mut output: Vec<Vec<i32>> = vec![];
        let mut position = Point {
            x: c_start,
            y: r_start,
        };
        let mut direction = Direction::Right;
        let (mut step_length, mut step_count) = (1, 0);

        loop {
            if position.x >= 0 && position.x < cols && position.y >= 0 && position.y < rows {
                output.push(vec![position.y, position.x]);
            }
            position = position.next_point(&direction);
            step_count += 1;

            if step_count >= step_length {
                step_count = 0;
                if direction == Direction::Down || direction == Direction::Up {
                    step_length += 1;
                }
                direction = direction.next();
            }
            if step_count > n {
                break;
            }
        }

        output
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let rows = 1;
        let cols = 4;
        let r_start = 0;
        let c_start = 0;
        let expected = vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]];
        assert_eq!(
            Solution::spiral_matrix_iii(rows, cols, r_start, c_start),
            expected
        );
    }

    #[test]
    fn example_2() {
        let rows = 5;
        let cols = 6;
        let r_start = 1;
        let c_start = 4;
        let expected = vec![
            vec![1, 4],
            vec![1, 5],
            vec![2, 5],
            vec![2, 4],
            vec![2, 3],
            vec![1, 3],
            vec![0, 3],
            vec![0, 4],
            vec![0, 5],
            vec![3, 5],
            vec![3, 4],
            vec![3, 3],
            vec![3, 2],
            vec![2, 2],
            vec![1, 2],
            vec![0, 2],
            vec![4, 5],
            vec![4, 4],
            vec![4, 3],
            vec![4, 2],
            vec![4, 1],
            vec![3, 1],
            vec![2, 1],
            vec![1, 1],
            vec![0, 1],
            vec![4, 0],
            vec![3, 0],
            vec![2, 0],
            vec![1, 0],
            vec![0, 0],
        ];
        assert_eq!(
            Solution::spiral_matrix_iii(rows, cols, r_start, c_start),
            expected
        );
    }

    #[test]
    fn example_3() {
        let rows = 8;
        let cols = 5;
        let r_start = 1;
        let c_start = 2;
        let expected = vec![
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![2, 2],
            vec![2, 1],
            vec![1, 1],
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![0, 4],
            vec![1, 4],
            vec![2, 4],
            vec![3, 4],
            vec![3, 3],
            vec![3, 2],
            vec![3, 1],
            vec![3, 0],
            vec![2, 0],
            vec![1, 0],
            vec![0, 0],
            vec![4, 4],
            vec![4, 3],
            vec![4, 2],
            vec![4, 1],
            vec![4, 0],
            vec![5, 4],
            vec![5, 3],
            vec![5, 2],
            vec![5, 1],
            vec![5, 0],
            vec![6, 4],
            vec![6, 3],
            vec![6, 2],
            vec![6, 1],
            vec![6, 0],
            vec![7, 4],
            vec![7, 3],
            vec![7, 2],
            vec![7, 1],
            vec![7, 0],
        ];
        assert_eq!(
            Solution::spiral_matrix_iii(rows, cols, r_start, c_start),
            expected
        );
    }

    #[test]
    fn example_4() {
        let rows = 10;
        let cols = 3;
        let r_start = 0;
        let c_start = 1;
        let expected = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 2],
            vec![1, 1],
            vec![1, 0],
            vec![0, 0],
            vec![2, 2],
            vec![2, 1],
            vec![2, 0],
            vec![3, 2],
            vec![3, 1],
            vec![3, 0],
            vec![4, 2],
            vec![4, 1],
            vec![4, 0],
            vec![5, 2],
            vec![5, 1],
            vec![5, 0],
            vec![6, 2],
            vec![6, 1],
            vec![6, 0],
            vec![7, 2],
            vec![7, 1],
            vec![7, 0],
            vec![8, 2],
            vec![8, 1],
            vec![8, 0],
            vec![9, 2],
            vec![9, 1],
            vec![9, 0],
        ];
        assert_eq!(
            Solution::spiral_matrix_iii(rows, cols, r_start, c_start),
            expected
        );
    }
}
// @leetup=inject:after_code
