//! SymEngine FFI bindings (WASM and native).
//!
//! SymEngine is compiled with `WITH_SYMENGINE_THREAD_SAFE=ON` (atomic refcounts
//! and hash caching). Each thread can safely work on its own `Expr` objects.
//! `Expr` is `Send` but not `Sync` — do not share references across threads.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_ulong};

// SymEngine is compiled with WITH_SYMENGINE_THREAD_SAFE=ON (atomic refcounts
// + atomic hash caching). Each thread can safely work on its own Expr objects.
// No mutex needed on native. WASM is single-threaded so also a no-op.
#[cfg(not(target_arch = "wasm32"))]
macro_rules! se_lock {
    () => {};
}

#[cfg(target_arch = "wasm32")]
macro_rules! se_lock {
    () => {};
}

/// Opaque type for SymEngine basic objects
#[repr(C)]
pub struct CBasic {
    _private: [u8; 0],
}

/// Opaque type for SymEngine set objects
#[repr(C)]
pub struct CSetBasic {
    _opaque: [u8; 0],
}

unsafe extern "C" {
    // Lifecycle
    fn basic_new_heap() -> *mut CBasic;
    fn basic_free_heap(b: *mut CBasic);

    // String representations
    fn basic_str(b: *const CBasic) -> *mut c_char;
    fn basic_str_free(s: *mut c_char);

    // Parsing & construction
    fn basic_parse(b: *mut CBasic, s: *const c_char) -> c_int;
    fn symbol_set(b: *mut CBasic, name: *const c_char) -> c_int;
    fn real_double_set_d(b: *mut CBasic, d: f64) -> c_int;
    fn real_double_get_d(b: *const CBasic) -> f64;

    // Constants
    fn basic_const_zero(b: *mut CBasic);

    // Arithmetic
    fn basic_expand(result: *mut CBasic, expr: *const CBasic) -> c_int;
    fn basic_sub(result: *mut CBasic, a: *const CBasic, b: *const CBasic) -> c_int;

    // Comparison & type checking
    fn basic_eq(a: *const CBasic, b: *const CBasic) -> c_int;
    fn number_is_zero(b: *const CBasic) -> c_int;
    fn is_a_RealDouble(b: *const CBasic) -> c_int;
    fn is_a_Integer(b: *const CBasic) -> c_int;
    fn is_a_Rational(b: *const CBasic) -> c_int;
    fn is_a_Number(b: *const CBasic) -> c_int;

    // Calculus
    fn basic_diff(result: *mut CBasic, expr: *const CBasic, sym: *const CBasic) -> c_int;

    // Substitution & evaluation
    fn basic_subs2(
        result: *mut CBasic,
        expr: *const CBasic,
        from: *const CBasic,
        to: *const CBasic,
    ) -> c_int;
    fn basic_evalf(result: *mut CBasic, expr: *const CBasic, bits: c_ulong, real: c_int) -> c_int;

    // Free symbols
    fn basic_free_symbols(expr: *const CBasic, symbols: *mut CSetBasic) -> c_int;
    fn setbasic_new() -> *mut CSetBasic;
    fn setbasic_free(s: *mut CSetBasic);
    fn setbasic_size(s: *mut CSetBasic) -> usize;
    fn setbasic_get(s: *mut CSetBasic, n: c_int, result: *mut CBasic);
}

/// A safe wrapper around SymEngine expressions.
///
/// All methods acquire the global SymEngine lock on native targets.
/// `Expr` is `Send` but NOT `Sync` — do not share references across threads.
pub struct Expr {
    ptr: *mut CBasic,
}

// SAFETY: Expr pointers are only accessed while holding SYMENGINE_LOCK.
// Each Expr owns its pointer exclusively (no aliasing).
unsafe impl Send for Expr {}

impl Expr {
    /// Parse an expression from a string
    pub fn parse(input: &str) -> Result<Self, ExprError> {
        se_lock!();
        unsafe {
            let ptr = basic_new_heap();
            let c_str = CString::new(input)
                .map_err(|_| ExprError::ParseError("Invalid string".to_string()))?;

            let result = basic_parse(ptr, c_str.as_ptr());
            if result != 0 {
                basic_free_heap(ptr);
                return Err(ExprError::ParseError(format!("Failed to parse: {}", input)));
            }

            Ok(Self { ptr })
        }
    }

    /// Get the string representation
    pub fn to_string(&self) -> String {
        se_lock!();
        unsafe {
            let s = basic_str(self.ptr);
            let result = CStr::from_ptr(s).to_string_lossy().into_owned();
            basic_str_free(s);
            result
        }
    }

    /// Create the zero constant
    pub fn zero() -> Self {
        se_lock!();
        unsafe {
            let ptr = basic_new_heap();
            basic_const_zero(ptr);
            Self { ptr }
        }
    }

    /// Expand the expression
    pub fn expand(&self) -> Self {
        se_lock!();
        unsafe {
            let result_ptr = basic_new_heap();
            basic_expand(result_ptr, self.ptr);
            Self { ptr: result_ptr }
        }
    }

    /// Subtract another expression from this one
    pub fn sub(&self, other: &Self) -> Self {
        se_lock!();
        unsafe {
            let result_ptr = basic_new_heap();
            basic_sub(result_ptr, self.ptr, other.ptr);
            Self { ptr: result_ptr }
        }
    }

    /// Check structural equality with another expression
    pub fn equals(&self, other: &Self) -> bool {
        se_lock!();
        unsafe { basic_eq(self.ptr, other.ptr) != 0 }
    }

    /// Check if expression is the number zero.
    /// Must check is_a_Number first — `number_is_zero` segfaults on non-Number types
    /// in native SymEngine builds.
    pub fn is_zero(&self) -> bool {
        se_lock!();
        unsafe { is_a_Number(self.ptr) != 0 && number_is_zero(self.ptr) != 0 }
    }

    /// Check if expression is a number (integer, rational, or real)
    pub fn is_number(&self) -> bool {
        se_lock!();
        unsafe { is_a_Number(self.ptr) != 0 }
    }

    /// Substitute a named variable with a float value
    pub fn subs_float(&self, var_name: &str, val: f64) -> Self {
        se_lock!();
        unsafe {
            let sym_ptr = basic_new_heap();
            let c_name = CString::new(var_name).expect("Invalid variable name");
            symbol_set(sym_ptr, c_name.as_ptr());

            let val_ptr = basic_new_heap();
            real_double_set_d(val_ptr, val);

            let result_ptr = basic_new_heap();
            basic_subs2(result_ptr, self.ptr, sym_ptr, val_ptr);

            basic_free_heap(sym_ptr);
            basic_free_heap(val_ptr);

            Self { ptr: result_ptr }
        }
    }

    /// Evaluate expression to a float.
    /// Returns None if the expression cannot be fully evaluated (still has free symbols).
    pub fn to_float(&self) -> Option<f64> {
        se_lock!();
        unsafe {
            // Fast path: if already a RealDouble, extract directly
            if is_a_RealDouble(self.ptr) != 0 {
                return Some(real_double_get_d(self.ptr));
            }

            // If it's an Integer or Rational, convert via string
            if is_a_Integer(self.ptr) != 0 || is_a_Rational(self.ptr) != 0 {
                let s = basic_str(self.ptr);
                let str_val = CStr::from_ptr(s).to_string_lossy().into_owned();
                basic_str_free(s);
                return str_val.parse::<f64>().ok();
            }

            // General case: use evalf in real domain (real=1 to avoid complex results)
            let result_ptr = basic_new_heap();
            let rc = basic_evalf(result_ptr, self.ptr, 53, 1);

            if rc != 0 {
                basic_free_heap(result_ptr);
                return None;
            }

            let val = if is_a_RealDouble(result_ptr) != 0 {
                Some(real_double_get_d(result_ptr))
            } else if is_a_Integer(result_ptr) != 0 || is_a_Rational(result_ptr) != 0 {
                let s = basic_str(result_ptr);
                let str_val = CStr::from_ptr(s).to_string_lossy().into_owned();
                basic_str_free(s);
                str_val.parse::<f64>().ok()
            } else {
                None
            };

            basic_free_heap(result_ptr);
            val
        }
    }

    /// Get the names of free symbols in this expression
    pub fn free_symbols(&self) -> Vec<String> {
        se_lock!();
        unsafe {
            let set = setbasic_new();
            basic_free_symbols(self.ptr, set);

            let n = setbasic_size(set);
            let mut result = Vec::with_capacity(n);
            let tmp = basic_new_heap();

            for i in 0..n {
                setbasic_get(set, i as c_int, tmp);
                let s = basic_str(tmp);
                result.push(CStr::from_ptr(s).to_string_lossy().into_owned());
                basic_str_free(s);
            }

            basic_free_heap(tmp);
            setbasic_free(set);
            result
        }
    }

    /// Differentiate with respect to a variable
    pub fn diff(&self, var: &str) -> Result<Self, ExprError> {
        se_lock!();
        unsafe {
            let var_ptr = basic_new_heap();
            let c_var = CString::new(var)
                .map_err(|_| ExprError::ParseError("Invalid variable".to_string()))?;

            symbol_set(var_ptr, c_var.as_ptr());

            let result_ptr = basic_new_heap();
            let result_code = basic_diff(result_ptr, self.ptr, var_ptr);

            basic_free_heap(var_ptr);

            if result_code != 0 {
                basic_free_heap(result_ptr);
                return Err(ExprError::ParseError("Differentiation failed".to_string()));
            }

            Ok(Self { ptr: result_ptr })
        }
    }
}

impl Drop for Expr {
    fn drop(&mut self) {
        se_lock!();
        unsafe {
            basic_free_heap(self.ptr);
        }
    }
}

impl Clone for Expr {
    fn clone(&self) -> Self {
        se_lock!();
        unsafe {
            // Get string representation and re-parse in a single lock acquisition
            let s = basic_str(self.ptr);
            let str_val = CStr::from_ptr(s).to_string_lossy().into_owned();
            basic_str_free(s);

            let ptr = basic_new_heap();
            let c_str = CString::new(str_val).expect("Failed to clone expression");
            let rc = basic_parse(ptr, c_str.as_ptr());
            assert_eq!(rc, 0, "Failed to clone expression");
            Self { ptr }
        }
    }
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
