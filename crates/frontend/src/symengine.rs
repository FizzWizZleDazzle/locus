//! SymEngine FFI bindings for WebAssembly
//!
//! This module provides bindings to the SymEngine C API for symbolic computation.
//! In the browser, SymEngine is loaded as a WASM side module and linked dynamically.

#![allow(dead_code)]

/// Opaque type for SymEngine basic objects
#[repr(C)]
pub struct CBasic {
    _private: [u8; 0],
}

// FFI declarations - these will be linked from symengine.wasm
// For MVP, we use a simplified implementation that doesn't require actual linking
/*
extern "C" {
    pub fn basic_new_heap() -> *mut CBasic;
    pub fn basic_free_heap(b: *mut CBasic);
    pub fn basic_str(b: *const CBasic) -> *mut c_char;
    pub fn basic_str_free(s: *mut c_char);
    pub fn basic_parse(b: *mut CBasic, s: *const c_char) -> c_int;
    pub fn basic_eq(a: *const CBasic, b: *const CBasic) -> c_int;
    pub fn basic_expand(result: *mut CBasic, expr: *const CBasic) -> c_int;
    pub fn basic_diff(result: *mut CBasic, expr: *const CBasic, sym: *const CBasic) -> c_int;
}
*/

/// A safe wrapper around SymEngine expressions
pub struct Expr {
    raw: String,
}

impl Expr {
    /// Parse an expression from a string
    pub fn parse(input: &str) -> Result<Self, ExprError> {
        // For MVP: just store the normalized string
        let normalized = normalize_input(input);
        Ok(Self { raw: normalized })
    }

    /// Get the string representation
    pub fn to_string(&self) -> String {
        self.raw.clone()
    }

    /// Expand the expression
    pub fn expand(&self) -> Self {
        // For MVP: return as-is (actual expansion requires SymEngine)
        Self { raw: self.raw.clone() }
    }

    /// Check equality with another expression
    pub fn equals(&self, other: &Self) -> bool {
        // For MVP: normalized string comparison
        self.raw == other.raw
    }

    /// Check if expression is a product (for factoring verification)
    pub fn is_mul(&self) -> bool {
        // Simple heuristic: contains '*' at top level (outside parentheses)
        let mut depth = 0;
        for c in self.raw.chars() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                '*' if depth == 0 => return true,
                _ => {}
            }
        }
        false
    }

    /// Differentiate with respect to a variable
    pub fn diff(&self, _var: &str) -> Result<Self, ExprError> {
        // For MVP: not implemented
        Err(ExprError::NotImplemented)
    }
}

/// Normalize mathematical input for comparison
fn normalize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
}

#[derive(Debug, Clone)]
pub enum ExprError {
    ParseError(String),
    NotImplemented,
}

impl std::fmt::Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ExprError::NotImplemented => write!(f, "Not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let expr = Expr::parse("x^2 + 1").unwrap();
        assert_eq!(expr.to_string(), "x^2+1");
    }

    #[test]
    fn test_equals_normalized() {
        let a = Expr::parse("x^2 + 1").unwrap();
        let b = Expr::parse("x^2+1").unwrap();
        assert!(a.equals(&b));
    }

    #[test]
    fn test_is_mul() {
        let product = Expr::parse("(x+1)*(x-1)").unwrap();
        assert!(product.is_mul());

        let sum = Expr::parse("x^2 + 1").unwrap();
        assert!(!sum.is_mul());
    }
}
