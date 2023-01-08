// @leetup=custom
// @leetup=info id=149 lang=rust slug=max-points-on-a-line

/*
* Given an array of `points` where `points[i] = [xi, yi]` represents a point on
* the X-Y plane, return *the maximum number of points that lie on the same
* straight line*.
*
*
* Example 1:
*
* []
* Input: points = [[1,1],[2,2],[3,3]]
* Output: 3
*
* Example 2:
*
* []
* Input: points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
* Output: 4
*
*
* Constraints:
*
* * `1 <= points.length <= 300`
* * `points[i].length == 2`
* * `-104 <= xi, yi <= 104`
* * All the `points` are unique.
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
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        type Point = (i32, i32);
        let mut max = 0;

        let points = points
            .into_iter()
            .map(|p| (p[0], p[1]))
            .collect::<Vec<Point>>();

        for from in &points {
            let mut angle_intersection_count = std::collections::HashMap::new();
            let mut local_max = 1;
            for to in &points {
                if from != to {
                    let (dx, dy) = (to.0 - from.0, to.1 - from.1);
                    let angle = (dx as f64).atan2(dy as f64);
                    let intersection_count =
                        angle_intersection_count.entry(angle.to_bits()).or_insert(1);
                    *intersection_count += 1;
                    local_max = std::cmp::max(local_max, *intersection_count);
                }
            }
            max = std::cmp::max(max, local_max);
        }
        max
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::max_points(points), 3);
    }

    #[test]
    fn example_2() {
        let points = vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ];
        assert_eq!(Solution::max_points(points), 4);
    }

    #[test]
    fn example_3() {
        let points = vec![vec![0, 0]];
        assert_eq!(Solution::max_points(points), 1);
    }
}
// @leetup=inject:after_code
