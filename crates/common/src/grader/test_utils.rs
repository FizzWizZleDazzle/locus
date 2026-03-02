//! Shared test utilities for grader tests.
//!
//! Provides a minimal `ExprEngine` that parses f64 values,
//! sufficient for testing grading logic without SymEngine FFI.

use super::ExprEngine;
use std::fmt;

#[derive(Clone, Debug)]
pub struct NumExpr(pub f64);

#[derive(Debug)]
pub struct NumError(pub String);

impl fmt::Display for NumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ExprEngine for NumExpr {
    type Error = NumError;

    fn parse(input: &str) -> Result<Self, Self::Error> {
        input
            .trim()
            .parse::<f64>()
            .map(NumExpr)
            .map_err(|e| NumError(e.to_string()))
    }

    fn expand(&self) -> Self {
        self.clone()
    }

    fn sub(&self, other: &Self) -> Self {
        NumExpr(self.0 - other.0)
    }

    fn equals(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 1e-10
    }

    fn is_zero(&self) -> bool {
        self.0.abs() < 1e-10
    }

    fn free_symbols(&self) -> Vec<String> {
        vec![]
    }

    fn subs_float(&self, _: &str, _: f64) -> Self {
        self.clone()
    }

    fn to_float(&self) -> Option<f64> {
        Some(self.0)
    }
}
