// @leetup=custom
// @leetup=info id=295 lang=rust slug=find-median-from-data-stream

/*
* The median is the middle value in an ordered integer list. If the size of
* the list is even, there is no middle value, and the median is the mean of the
* two middle values.
*
* * For example, for `arr = [2,3,4]`, the median is `3`.
* * For example, for `arr = [2,3]`, the median is `(2 + 3) / 2 = 2.5`.
*
* Implement the MedianFinder class:
*
* * `MedianFinder()` initializes the `MedianFinder` object.
* * `void addNum(int num)` adds the integer `num` from the data stream to the data
*   structure.
* * `double findMedian()` returns the median of all elements so far. Answers
*   within `10-5` of the actual answer will be accepted.
*
*
* Example 1:
*
* Input
* ["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
* [[], [1], [2], [], [3], []]
* Output
* [null, null, null, 1.5, null, 2.0]
* Explanation
* MedianFinder medianFinder = new MedianFinder();
* medianFinder.addNum(1);    // arr = [1]
* medianFinder.addNum(2);    // arr = [1, 2]
* medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
* medianFinder.addNum(3);    // arr[1, 2, 3]
* medianFinder.findMedian(); // return 2.0
*
*
* Constraints:
*
* * `-105 <= num <= 105`
* * There will be at least one element in the data structure before calling
*   `findMedian`.
* * At most `5 * 104` calls will be made to `addNum` and `findMedian`.
*
*
* Follow up:
*
* * If all integer numbers from the stream are in the range `[0, 100]`, how would
*   you optimize your solution?
* * If `99%` of all integer numbers from the stream are in the range `[0, 100]`,
*   how would you optimize your solution?
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.min_heap.len() == self.max_heap.len() {
            self.max_heap.push(num);
            if let Some(x) = self.max_heap.pop() {
                self.min_heap.push(Reverse(x));
            }
        } else {
            self.min_heap.push(Reverse(num));
            if let Some(Reverse(x)) = self.min_heap.pop() {
                self.max_heap.push(x);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.min_heap.len() > self.max_heap.len() {
            if let Some(Reverse(x)) = self.min_heap.peek() {
                return *x as f64;
            }
        } else if let Some(Reverse(x)) = self.min_heap.peek() {
            if let Some(y) = self.max_heap.peek() {
                return (*x + *y) as f64 / 2_f64;
            }
        }
        -1_f64
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::MedianFinder;

    #[test]
    fn example_1() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        assert_eq!(mf.find_median(), 1.0_f64);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5_f64);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0_f64);
        mf.add_num(4);
        assert_eq!(mf.find_median(), 2.5_f64);
        mf.add_num(5);
        assert_eq!(mf.find_median(), 3.0_f64);
        mf.add_num(6);
        assert_eq!(mf.find_median(), 3.5_f64);
    }

    #[test]
    fn example_2() {
        let mut mf = MedianFinder::new();
        mf.add_num(11);
        assert_eq!(mf.find_median(), 11.0_f64);
        mf.add_num(14);
        assert_eq!(mf.find_median(), 12.5_f64);
        mf.add_num(6);
        assert_eq!(mf.find_median(), 11.0_f64);
    }

    #[test]
    fn example_3() {
        let mut mf = MedianFinder::new();
        mf.add_num(11);
        assert_eq!(mf.find_median(), 11.0_f64);
        mf.add_num(12);
        assert_eq!(mf.find_median(), 11.5_f64);
        mf.add_num(13);
        assert_eq!(mf.find_median(), 12.0_f64);
        mf.add_num(14);
        assert_eq!(mf.find_median(), 12.5_f64);
        mf.add_num(15);
        assert_eq!(mf.find_median(), 13.0_f64);
        mf.add_num(16);
        assert_eq!(mf.find_median(), 13.5_f64);
    }
}
// @leetup=inject:after_code
