// @leetup=custom
// @leetup=info id=1029 lang=rust slug=two-city-scheduling

/*
* A company is planning to interview `2n` people. Given the array `costs` where
* `costs[i] = [aCosti, bCosti]`, the cost of flying the `ith` person to city `a`
* is `aCosti`, and the cost of flying the `ith` person to city `b` is `bCosti`.
*
* Return *the minimum cost to fly every person to a city* such that exactly `n`
* people arrive in each city.
*
*
* Example 1:
*
* Input: costs = [[10,20],[30,200],[400,50],[30,20]]
* Output: 110
* Explanation:
* The first person goes to city A for a cost of 10.
* The second person goes to city A for a cost of 30.
* The third person goes to city B for a cost of 50.
* The fourth person goes to city B for a cost of 20.
* The total minimum cost is 10 + 30 + 50 + 20 = 110 to have half the people interv
* iewing in each city.
*
* Example 2:
*
* Input: costs = [[259,770],[448,54],[926,667],[184,139],[840,118],[577,469]]
* Output: 1859
*
* Example 3:
*
* Input: costs = [[515,563],[451,713],[537,709],[343,819],[855,779],[457,60],[
* 650,359],[631,42]]
* Output: 3086
*
*
* Constraints:
*
* * `2 * n == costs.length`
* * `2 <= costs.length <= 100`
* * `costs.length` is even.
* * `1 <= aCosti, bCosti <= 1000`
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
    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        //Sort costs by absolute difference in descending order  e.g. [[1,100], [50, 60], [20, 22]]
        costs.sort_unstable_by(|a, b| {
            (i32::abs(b[0] - b[1]))
                .partial_cmp(&i32::abs(a[0] - a[1]))
                .unwrap()
        });
        let (n, mut a, mut b, mut result) = (costs.len() / 2, 0, 0, 0);
        for cost in costs {
            let put_in_a = if a < n && b < n {
                // If both a and b have space, check which has lower cost
                cost[0] < cost[1]
            } else {
                // If one of them is full, put it in the other
                a < n
            };
            if put_in_a {
                a += 1;
                result += cost[0];
            } else {
                b += 1;
                result += cost[1];
            }
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
        let costs = vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]];
        assert_eq!(Solution::two_city_sched_cost(costs), 110);
    }

    #[test]
    fn example_2() {
        let costs = vec![
            vec![259, 770],
            vec![448, 54],
            vec![926, 667],
            vec![184, 139],
            vec![840, 118],
            vec![577, 469],
        ];
        assert_eq!(Solution::two_city_sched_cost(costs), 1859);
    }

    #[test]
    fn example_3() {
        let costs = vec![
            vec![515, 563],
            vec![451, 713],
            vec![537, 709],
            vec![343, 819],
            vec![855, 779],
            vec![457, 60],
            vec![650, 359],
            vec![631, 42],
        ];
        assert_eq!(Solution::two_city_sched_cost(costs), 3086);
    }
}
// @leetup=inject:after_code
