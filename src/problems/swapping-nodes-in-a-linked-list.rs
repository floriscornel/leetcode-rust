// @leetup=custom
// @leetup=info id=1721 lang=rust slug=swapping-nodes-in-a-linked-list

/*
* You are given the `head` of a linked list, and an integer `k`.
*
* Return *the head of the linked list after swapping the values of the *`kth`
* *node from the beginning and the *`kth` *node from the end (the list is
* 1-indexed).*
*
*
* Example 1:
*
* []
* Input: head = [1,2,3,4,5], k = 2
* Output: [1,4,3,2,5]
*
* Example 2:
*
* Input: head = [7,9,6,6,7,8,3,0,9,5], k = 5
* Output: [7,9,6,6,8,7,3,0,9,5]
*
*
* Constraints:
*
* * The number of nodes in the list is `n`.
* * `1 <= k <= n <= 105`
* * `0 <= Node.val <= 100`
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

impl Solution {
    pub fn swap_nodes(head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        let (mut left, mut right, mut second_pos) = (-1, -1, -1);
        // Loop over the list and the values and second_position of k
        Solution::get_positions(head.clone(), 0, k, &mut left, &mut right, &mut second_pos);
        // If k > n/2, then then second_pos < k. We will swap the variables.
        if second_pos < k {
            std::mem::swap(&mut second_pos, &mut k);
            std::mem::swap(&mut left, &mut right);
        }
        // Build new list
        Solution::build_list(head, 1, k, &mut left, &mut right, &mut second_pos)
    }

    fn get_positions(
        head: Option<Box<ListNode>>,
        offset: i32,
        k: i32,
        left: &mut i32,
        right: &mut i32,
        second_pos: &mut i32,
    ) -> (i32, i32) {
        let left_pos = offset + 1;
        if let Some(x) = head {
            let (next_left, next_right) =
                Solution::get_positions(x.next, left_pos, k, left, right, second_pos);
            let right_pos = next_right + 1;
            if left_pos == k {
                *left = x.val;
            }
            if right_pos == k {
                *right = x.val;
                *second_pos = left_pos;
            }
            (left_pos, right_pos)
        } else {
            (left_pos, 0)
        }
    }

    fn build_list(
        head: Option<Box<ListNode>>,
        offset: i32,
        k: i32,
        left: &mut i32,
        right: &mut i32,
        second_pos: &mut i32,
    ) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            if offset == *second_pos {
                node.val = *left;
                // After replacing the second value we can stop itterating
                return Some(node);
            }
            if offset == k {
                node.val = *right;
            }
            node.next = Solution::build_list(node.next, offset + 1, k, left, right, second_pos);
            Some(node)
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
        assert_eq!(
            Solution::swap_nodes(linked![1, 2, 3, 4, 5], 2),
            linked![1, 4, 3, 2, 5]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::swap_nodes(linked![7, 9, 6, 6, 7, 8, 3, 0, 9, 5], 5),
            linked![7, 9, 6, 6, 8, 7, 3, 0, 9, 5]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::swap_nodes(linked![1, 2, 3, 4, 5], 4),
            linked![1, 4, 3, 2, 5]
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::swap_nodes(linked![1, 2, 3, 4, 5], 1),
            linked![5, 2, 3, 4, 1]
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            Solution::swap_nodes(linked![1, 2, 3, 4, 5], 5),
            linked![5, 2, 3, 4, 1]
        );
    }
}
// @leetup=inject:after_code
