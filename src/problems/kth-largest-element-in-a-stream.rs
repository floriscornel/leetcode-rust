// @leetup=custom
// @leetup=info id=703 lang=rust slug=kth-largest-element-in-a-stream

/*
* Design a class to find the `kth` largest element in a stream. Note that it is
* the `kth` largest element in the sorted order, not the `kth` distinct element.
*
* Implement `KthLargest` class:
*
* * `KthLargest(int k, int[] nums)` Initializes the object with the integer `k`
*   and the stream of integers `nums`.
* * `int add(int val)` Appends the integer `val` to the stream and returns the
*   element representing the `kth` largest element in the stream.
*
*
* Example 1:
*
* Input
* ["KthLargest", "add", "add", "add", "add", "add"]
* [[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
* Output
* [null, 4, 5, 5, 8, 8]
* Explanation
* KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
* kthLargest.add(3);   // return 4
* kthLargest.add(5);   // return 5
* kthLargest.add(10);  // return 5
* kthLargest.add(9);   // return 8
* kthLargest.add(4);   // return 8
*
*
* Constraints:
*
* * `1 <= k <= 104`
* * `0 <= nums.length <= 104`
* * `-104 <= nums[i] <= 104`
* * `-104 <= val <= 104`
* * At most `104` calls will be made to `add`.
* * It is guaranteed that there will be at least `k` elements in the array when
*   you search for the `kth` element.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
// @leetup=inject:before_code_ex

// @leetup=code

struct KthLargest {
    k: usize,
    heap: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut new = KthLargest {
            k: k as usize,
            heap: BinaryHeap::with_capacity(k as usize + 1),
        };
        for num in nums {
            new.add(num);
        }
        new
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(-val);
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        -self.heap.peek().unwrap()
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::KthLargest;

    #[test]
    fn example_1() {
        let mut x = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(x.add(3), 4);
        assert_eq!(x.add(5), 5);
        assert_eq!(x.add(10), 5);
        assert_eq!(x.add(9), 8);
        assert_eq!(x.add(4), 8);
    }
}
// @leetup=inject:after_code

use std::collections::BinaryHeap;
