#[macro_export]
macro_rules! matrix {
    ($($row:expr),*) => {{
        vec![$($row.to_vec()),*]
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn matrix_macro_creates_2d_vec() {
        assert_eq!(
            matrix![[2, 1, 3], [6, 5, 4], [7, 8, 9]],
            vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]]
        );
    }
}
