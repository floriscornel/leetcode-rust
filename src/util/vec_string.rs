/**
 * Implementation by aylei and mapx on github.com
 * Source: https://github.com/aylei/leetcode-rust/blob/master/src/util/vec_string.rs
 *
 * Macro Usage: vec_string!["a", "b", "c"]
 */

#[macro_export]
macro_rules! vec_string {
    ($($e:expr),*) => {vec![$($e.to_owned()), *]};
    ($($e:expr,)*) => {vec![$($e.to_owned()), *]};
}
