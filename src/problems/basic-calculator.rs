// @leetup=custom
// @leetup=info id=224 lang=rust slug=basic-calculator

/*
* Given a string `s` representing a valid expression, implement a basic calculator
* to evaluate it, and return *the result of the evaluation*.
*
* Note: You are not allowed to use any built-in function which evaluates
* strings as mathematical expressions, such as `eval()`.
*
*
* Example 1:
*
* Input: s = "1 + 1"
* Output: 2
*
* Example 2:
*
* Input: s = " 2-1 + 2 "
* Output: 3
*
* Example 3:
*
* Input: s = "(1+(4+5+2)-3)+(6+8)"
* Output: 23
*
*
* Constraints:
*
* * `1 <= s.length <= 3 * 105`
* * `s` consists of digits, `'+'`, `'-'`, `'('`, `')'`, and `' '`.
* * `s` represents a valid expression.
* * `'+'` is not used as a unary operation (i.e., `"+1"` and `"+(2 + 3)"` is
*   invalid).
* * `'-'` could be used as a unary operation (i.e., `"-1"` and `"-(2 + 3)"` is
*   valid).
* * There will be no two consecutive operators in the input.
* * Every number and running calculation will fit in a signed 32-bit integer.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
#[derive(Clone, Copy, Debug)]
enum Operator {
    Plus,
    Minus,
}

#[derive(Clone, Debug)]
enum Expression {
    Num(i32),
    Composite(Box<Expression>, Operator, Box<Expression>),
}

impl Expression {
    fn closing_index(string: &[u8], from: usize, to: usize) -> usize {
        let mut count = 1;
        for (idx, char) in string.iter().enumerate().take(to).skip(from + 1) {
            match *char {
                b'(' => count += 1,
                b')' => count -= 1,
                _ => continue,
            };
            if count == 0 {
                return idx;
            }
        }
        unreachable!("Correct input but could not find closing bracket.")
    }

    fn read_number(string: &[u8], from: usize, to: usize) -> (i32, usize) {
        let mut val: i32 = 0;
        let mut count = 0;
        for char in string.iter().take(to).skip(from) {
            if let Some(char_val) = char::from(*char).to_digit(10) {
                val *= 10;
                val += char_val as i32;
                count += 1;
            } else {
                break;
            }
        }
        (val, count)
    }

    fn build(string: &[u8], from: usize, to: usize) -> Self {
        let mut lhs = None;
        let mut operator = None;
        let mut skip = 0;
        for i in from..to {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            match string[i] {
                b' ' | b')' => continue,
                b'+' => {
                    operator = Some(Operator::Plus);
                }
                b'-' => {
                    operator = Some(Operator::Minus);
                    if lhs.is_none() {
                        lhs = Some(Expression::Num(0));
                    }
                }
                b'(' => {
                    let closing_idx = Self::closing_index(string, i, to);
                    if let Some(lhs_val) = lhs {
                        lhs = Some(Expression::Composite(
                            Box::new(lhs_val),
                            operator.unwrap(),
                            Box::new(Expression::build(string, i + 1, closing_idx)),
                        ));
                        operator = None;
                    } else {
                        lhs = Some(Expression::build(string, i + 1, closing_idx));
                    }
                    skip = closing_idx - i;
                }
                x => {
                    let (value, count) = Self::read_number(string, i, to);
                    if let Some(lhs_val) = lhs {
                        lhs = Some(Expression::Composite(
                            Box::new(lhs_val),
                            operator.unwrap(),
                            Box::new(Expression::Num(value)),
                        ));
                        operator = None;
                    } else {
                        lhs = Some(Expression::Num(value));
                    }
                    skip = count - 1;
                }
            };
        }
        lhs.unwrap_or(Expression::Num(0))
    }

    fn evaluate(&self) -> i32 {
        match self {
            Self::Num(x) => *x,
            Self::Composite(lhs, operator, rhs) => match operator {
                Operator::Plus => Self::evaluate(lhs.as_ref()) + Self::evaluate(rhs),
                Operator::Minus => Self::evaluate(lhs) - Self::evaluate(rhs),
            },
        }
    }
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let expr = Expression::build(s.as_bytes(), 0, s.len());
        expr.evaluate()
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::calculate("1 + 1".to_owned()), 2);
        assert_eq!(Solution::calculate(" 2-1 + 2 ".to_owned()), 3);
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_owned()), 23);
        assert_eq!(Solution::calculate("(((1)))-(((2)))".to_owned()), -1);
        assert_eq!(Solution::calculate("(-((1)))-(((2)))".to_owned()), -3);
        assert_eq!(Solution::calculate("(((1)))-(((2)))+((3))".to_owned()), 2);
        assert_eq!(
            Solution::calculate("(((1)))-((((2)))+((3)))".to_owned()),
            -4
        );
        assert_eq!(Solution::calculate("-(2 + 3)".to_owned()), -5);
    }
}
// @leetup=inject:after_code
