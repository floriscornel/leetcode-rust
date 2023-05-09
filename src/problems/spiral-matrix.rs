// @leetup=custom
// @leetup=info id=54 lang=rust slug=spiral-matrix

/*
* Given an `m x n` `matrix`, return *all elements of the* `matrix` *in spiral
* order*.
*
*
* Example 1:
*
* []
* Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
* Output: [1,2,3,6,9,8,7,4,5]
*
* Example 2:
*
* []
* Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
* Output: [1,2,3,4,8,12,11,10,9,5,6,7]
*
*
* Constraints:
*
* * `m == matrix.length`
* * `n == matrix[i].length`
* * `1 <= m, n <= 10`
* * `-100 <= matrix[i][j] <= 100`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

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

const VISITED_VALUE: i32 = -101;

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (n, m) = (matrix.len() as i32, matrix[0].len() as i32);
        let out_of_bounds = |p: &Point| p.x < 0 || p.y < 0 || p.x >= m || p.y >= n;

        let mut result = vec![];
        let mut direction = Direction::Right;
        let mut position: Point = Point { x: 0, y: 0 };
        loop {
            result.push(matrix[position.y as usize][position.x as usize]);
            matrix[position.y as usize][position.x as usize] = VISITED_VALUE;
            let mut next = position.next_point(&direction);
            if out_of_bounds(&next) || matrix[next.y as usize][next.x as usize] == VISITED_VALUE {
                direction = direction.next();
                next = position.next_point(&direction);
            }
            if out_of_bounds(&next) || matrix[next.y as usize][next.x as usize] == VISITED_VALUE {
                break;
            }
            position = next;
        }

        result
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let output = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(Solution::spiral_order(matrix), output)
    }

    #[test]
    fn example_2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let output = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(Solution::spiral_order(matrix), output)
    }

    #[test]
    fn example_3() {
        let matrix = vec![vec![1, 2, 3, 4]];
        let output = vec![1, 2, 3, 4];
        assert_eq!(Solution::spiral_order(matrix), output)
    }

    #[test]
    fn example_4() {
        let matrix = vec![vec![1], vec![2], vec![3], vec![4]];
        let output = vec![1, 2, 3, 4];
        assert_eq!(Solution::spiral_order(matrix), output)
    }

    #[test]
    fn example_5() {
        let matrix = vec![vec![1, 2], vec![3, 4]];
        let output = vec![1, 2, 4, 3];
        assert_eq!(Solution::spiral_order(matrix), output)
    }
}
// @leetup=inject:after_code
