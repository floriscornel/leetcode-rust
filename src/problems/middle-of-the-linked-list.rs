// @leetup=custom
// @leetup=info id=876 lang=rust slug=middle-of-the-linked-list

/*
* Given the `head` of a singly linked list, return *the middle node of the linked
* list*.
*
* If there are two middle nodes, return the second middle node.
*
*
* Example 1:
*
* []
* Input: head = [1,2,3,4,5]
* Output: [3,4,5]
* Explanation: The middle node of the list is node 3.
*
* Example 2:
*
* []
* Input: head = [1,2,3,4,5,6]
* Output: [4,5,6]
* Explanation: Since the list has two middle nodes with values 3 and 4, we ret
* urn the second one.
*
*
* Constraints:
*
* * The number of nodes in the list is in the range `[1, 100]`.
* * `1 <= Node.val <= 100`
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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // If the linked list is empty, return None.
        head.as_ref()?;

        // Initialize the first and second pointers to the head of the linked list.
        let mut first = &head;
        let mut second = &head;

        // Loop until the second pointer reaches the end of the linked list.
        // This loop will stop when the second pointer reaches the end of the linked list,
        // which means that the first pointer will be pointing to the middle node.
        while let Some(ref node) = second {
            second = &node.next;
            if let Some(ref node) = second {
                if let Some(ref node) = first {
                    first = &node.next;
                }
                second = &node.next;
            }
        }

        // Return the middle node that the first pointer is pointing to.
        (*first).clone()
    }
}
// This implementation uses the "two pointers" technique, where we have two pointers that traverse the linked list at different speeds. In this case, the first pointer moves one node at a time, while the second pointer moves two nodes at a time. This means that when the second pointer reaches the end of the linked list, the first pointer will be pointing to the middle node.

// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::middle_node(linked![1, 2, 3, 4, 5]),
            linked![3, 4, 5]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::middle_node(linked![1, 2, 3, 4, 5, 6]),
            linked![4, 5, 6]
        );
    }
}
// @leetup=inject:after_code
