// @leetup=custom
// @leetup=info id=901 lang=rust slug=online-stock-span

/*
* Design an algorithm that collects daily price quotes for some stock and returns
* the span of that stock's price for the current day.
*
* The span of the stock's price today is defined as the maximum number of
* consecutive days (starting from today and going backward) for which the stock
* price was less than or equal to today's price.
*
* * For example, if the price of a stock over the next `7` days were
*   `[100,80,60,70,60,75,85]`, then the stock spans would be `[1,1,1,2,1,4,6]`.
*
* Implement the `StockSpanner` class:
*
* * `StockSpanner()` Initializes the object of the class.
* * `int next(int price)` Returns the span of the stock's price given that
*   today's price is `price`.
*
*
* Example 1:
*
* Input
* ["StockSpanner", "next", "next", "next", "next", "next", "next", "next"]
* [[], [100], [80], [60], [70], [60], [75], [85]]
* Output
* [null, 1, 1, 1, 2, 1, 4, 6]
* Explanation
* StockSpanner stockSpanner = new StockSpanner();
* stockSpanner.next(100); // return 1
* stockSpanner.next(80);  // return 1
* stockSpanner.next(60);  // return 1
* stockSpanner.next(70);  // return 2
* stockSpanner.next(60);  // return 1
* stockSpanner.next(75);  // return 4, because the last 4 prices (including today'
* s price of 75) were less than or equal to today's price.
* stockSpanner.next(85);  // return 6
*
*
* Constraints:
*
* * `1 <= price <= 105`
* * At most `104` calls will be made to `next`.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
// @leetup=inject:before_code_ex

// @leetup=code

struct StockSpanner {
    store: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        Self { store: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        self.store.push(price);
        let mut count = 1;
        for idx in (0..self.store.len() - 1).rev() {
            if self.store[idx] <= price {
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::StockSpanner;

    #[test]
    fn example_1() {
        let mut stock_spanner = StockSpanner::new();
        assert_eq!(stock_spanner.next(100), 1);
        assert_eq!(stock_spanner.next(80), 1);
        assert_eq!(stock_spanner.next(60), 1);
        assert_eq!(stock_spanner.next(70), 2);
        assert_eq!(stock_spanner.next(60), 1);
        assert_eq!(stock_spanner.next(75), 4);
        assert_eq!(stock_spanner.next(85), 6);
    }
}
// @leetup=inject:after_code
