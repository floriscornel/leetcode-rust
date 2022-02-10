# LeetCode challenges in Rust

These are solutions in Rust for LeetCode challenges. 

## Setup
Make sure to have rust installed and updated to at least version `1.5.60`. To install and update rust check out [rustup](https://rustup.rs/).

I am using [Visual Studio Code](https://code.visualstudio.com/) with the [rust-analyer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer&ssr=true) extension.

## Usage
To add a solution for a new problem like [two-sum](https://leetcode.com/problems/two-sum/) copy `example_problem.rs` into a new file and add the module name to `lib.rs`:
```bash
cp src/example_problem.rs src/two_sum.rs
echo "mod two_sum;" >> src/lib.rs
```

Edit `src/two_sum.rs`:
```rust
#![allow(dead_code, unused_variables)]
struct Solution {}

/* Two Sum - https://leetcode.com/problems/two-sum/ */

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Solution goes in here
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn two_sum() {
        assert_eq!(super::Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(super::Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(super::Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
```
You can run it with:
```bash
cargo test two_sum
```