/**
 * Implementation by aylei and mapx on github.com
 * Source: https://github.com/aylei/leetcode-rust/blob/master/src/util/linked_list.rs
 *
 * Macro Usage: linked![1,4,5]
 */
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

#[macro_export]
macro_rules! linked {
    ($($e:expr),*) => {crate::util::linked_list::to_list(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {crate::util::linked_list::to_list(vec![$($e.to_owned()), *])};
}
