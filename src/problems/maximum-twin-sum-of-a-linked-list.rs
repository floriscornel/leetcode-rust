// @leetup=custom
// @leetup=info id=2130 lang=rust slug=maximum-twin-sum-of-a-linked-list

/*
* In a linked list of size `n`, where `n` is even, the `ith` node
* (0-indexed) of the linked list is known as the twin of the `(n-1-i)th`
* node, if `0 <= i <= (n / 2) - 1`.
*
* * For example, if `n = 4`, then node `0` is the twin of node `3`, and node `1`
*   is the twin of node `2`. These are the only nodes with twins for `n = 4`.
*
* The twin sum is defined as the sum of a node and its twin.
*
* Given the `head` of a linked list with even length, return *the maximum twin
* sum of the linked list*.
*
*
* Example 1:
*
* []
* Input: head = [5,4,2,1]
* Output: 6
* Explanation:
* Nodes 0 and 1 are the twins of nodes 3 and 2, respectively. All have twin sum =
* 6.
* There are no other nodes with twins in the linked list.
* Thus, the maximum twin sum of the linked list is 6.
*
* Example 2:
*
* []
* Input: head = [4,2,2,3]
* Output: 7
* Explanation:
* The nodes with twins present in this linked list are:
* - Node 0 is the twin of node 3 having a twin sum of 4 + 3 = 7.
* - Node 1 is the twin of node 2 having a twin sum of 2 + 2 = 4.
* Thus, the maximum twin sum of the linked list is max(7, 4) = 7.
*
* Example 3:
*
* []
* Input: head = [1,100000]
* Output: 100001
* Explanation:
* There is only one node with a twin in the linked list having twin sum of 1 + 100
* 000 = 100001.
*
*
* Constraints:
*
* * The number of nodes in the list is an even integer in the range `[2,
*   105]`.
* * `1 <= Node.val <= 105`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}

use crate::util::linked_list::ListNode;

// @leetup=inject:before_code_ex

// @leetup=code

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use std::collections::VecDeque;
use std::iter::FromIterator;

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut itt = ListNode::new(0);
        itt.next = head;

        let mut queue = VecDeque::<i32>::from_iter(itt);
        let mut max = 0;
        while let (Some(a), Some(b)) = (queue.pop_front(), queue.pop_back()) {
            max = max.max(a + b);
        }
        max
    }
}

impl Iterator for ListNode {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next.take() {
            self.next = next.next;
            Some(next.val)
        } else {
            None
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
        let head = linked![5, 4, 2, 1];
        let result = 6;

        assert_eq!(Solution::pair_sum(head), result);
    }

    #[test]
    fn example_2() {
        let head = linked![4, 2, 2, 3];
        let result = 7;

        assert_eq!(Solution::pair_sum(head), result);
    }

    #[test]
    fn example_3() {
        let head = linked![1, 100000];
        let result = 100001;

        assert_eq!(Solution::pair_sum(head), result);
    }

    #[test]
    fn example_4() {
        let head = linked![1, 2, 3, 4, 5, 6];
        let result = 7;

        assert_eq!(Solution::pair_sum(head), result);
    }
}
// @leetup=inject:after_code
