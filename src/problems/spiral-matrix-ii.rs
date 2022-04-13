// @leetup=custom
// @leetup=info id=59 lang=rust slug=spiral-matrix-ii

/*
* Given a positive integer `n`, generate an `n x n` `matrix` filled with elements
* from `1` to `n2` in spiral order.
*
*
* Example 1:
*
* []
* Input: n = 3
* Output: [[1,2,3],[8,9,4],[7,6,5]]
*
* Example 2:
*
* Input: n = 1
* Output: [[1]]
*
*
* Constraints:
*
* * `1 <= n <= 20`
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
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n == 1 {
            return vec![vec![1]];
        }

        // Setup a 2D n*n matrix initialized with 0's
        let mut output: Vec<Vec<i32>> = Vec::with_capacity(n as usize);
        for i in 0..n {
            let mut inner: Vec<i32> = Vec::with_capacity(n as usize);
            for j in 0..(n as usize) {
                inner.push(0);
            }
            output.push(inner);
        }

        let out_of_bounds = |p: &Point| p.x < 0 || p.y < 0 || p.x >= n || p.y >= n;
        let greater_zero = |p: &Point, arr: &Vec<Vec<i32>>| arr[p.y as usize][p.x as usize] > 0;

        // Fill in the array
        let mut direction = Direction::Right;
        let mut position: Point = Point { x: 0, y: 0 };
        let mut counter = 1;

        // Initialize beginning point
        output[position.x as usize][position.y as usize] = counter;
        counter += 1;

        loop {
            // Find the next point in current position
            let mut next = position.next_point(&direction);

            // If the x/y are out of bounds or the point is filled in already
            if out_of_bounds(&next) || greater_zero(&next, &output) {
                // Change the direction
                direction = direction.next();
                // next point is from current position in the new direction
                next = position.next_point(&direction);
            }
            // If this is already filled we are finished
            if greater_zero(&next, &output) {
                break;
            }
            // Update our position
            position = next;
            output[position.y as usize][position.x as usize] = counter;
            counter += 1;
        }

        output
    }
}

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
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let output = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        assert_eq!(Solution::generate_matrix(3), output);
    }

    #[test]
    fn example_2() {
        let output = vec![vec![1]];
        assert_eq!(Solution::generate_matrix(1), output);
    }

    #[test]
    fn example_3() {
        let output = vec![vec![1, 2], vec![4, 3]];
        assert_eq!(Solution::generate_matrix(2), output);
    }
}
// @leetup=inject:after_code
