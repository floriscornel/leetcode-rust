// @leetup=custom
// @leetup=info id=328 lang=rust slug=odd-even-linked-list

/*
* Given the `head` of a singly linked list, group all the nodes with odd indices
* together followed by the nodes with even indices, and return *the reordered
* list*.
*
* The first node is considered odd, and the second node is even,
* and so on.
*
* Note that the relative order inside both the even and odd groups should remain
* as it was in the input.
*
* You must solve the problem in `O(1)` extra space complexity and `O(n)` time
* complexity.
*
*
* Example 1:
*
* []
* Input: head = [1,2,3,4,5]
* Output: [1,3,5,2,4]
*
* Example 2:
*
* []
* Input: head = [2,1,3,5,6,4,7]
* Output: [2,3,6,7,1,5,4]
*
*
* Constraints:
*
* * The number of nodes in the linked list is in the range `[0, 104]`.
* * `-106 <= Node.val <= 106`
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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut even_head = head.as_mut()?.next.take();
        let mut odd = &mut head;
        let mut even = &mut even_head;

        fn next_as_mutable(node: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
            &mut node.as_mut().unwrap().next
        }

        while even.is_some() && next_as_mutable(even).is_some() {
            *next_as_mutable(odd) = next_as_mutable(even).take();
            odd = next_as_mutable(odd);
            *next_as_mutable(even) = next_as_mutable(odd).take();
            even = next_as_mutable(even);
        }
        *next_as_mutable(odd) = even_head;
        head
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
            Solution::odd_even_list(linked![1, 2, 3, 4, 5]),
            linked![1, 3, 5, 2, 4]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::odd_even_list(linked![2, 1, 3, 5, 6, 4, 7]),
            linked![2, 3, 6, 7, 1, 5, 4]
        );
    }
}
// @leetup=inject:after_code
