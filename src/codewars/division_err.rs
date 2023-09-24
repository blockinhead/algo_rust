#[cfg_attr(debug_assertions, allow(unused_imports, unused_variables, dead_code))]
#[allow(dead_code)]

use crate::division_err::DivisionError::{DivideByZero, NotDivisible};

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    pub dividend: i32,
    pub divisor: i32,
}

// This function should calculate `a` divided by `b` if `a` is evenly divisible by b.
// Otherwise, it should return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    match a % b == 0 {
        true => Ok(a/b),
        false => Err(DivideByZero)
    }
}
