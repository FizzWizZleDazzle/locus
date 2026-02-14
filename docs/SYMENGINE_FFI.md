# SymEngine FFI Safety Guide

**CRITICAL:** Improper SymEngine FFI usage can cause segmentation faults and data corruption.

This document is **mandatory reading** before modifying any SymEngine-related code.

## Safety Rules (READ FIRST)

### Rule 1: Type Guards Before number_is_zero()

**NEVER DO THIS:**
```rust
if symengine::number_is_zero(&expr.ptr) { ... }  // SEGFAULT on non-Number types
```

**ALWAYS DO THIS:**
```rust
if symengine::is_a_Number(&expr.ptr) && symengine::number_is_zero(&expr.ptr) { ... }
```

**Why:** `number_is_zero()` assumes the input is a Number type. Calling it on Symbol, Add, Mul, or any other non-Number type causes **immediate segmentation faults** in native builds.

**Detection:** This error is silent in WASM but crashes in production (native builds).

### Rule 2: Thread Safety (Native Only)

**Native builds:** SymEngine is **NOT thread-safe**. All FFI calls are protected by a global Mutex.

```rust
// In symengine.rs
#[cfg(not(target_family = "wasm"))]
static SYMENGINE_LOCK: Mutex<()> = Mutex::new(());

#[cfg(not(target_family = "wasm"))]
macro_rules! with_lock {
    ($expr:expr) => {{
        let _guard = SYMENGINE_LOCK.lock().unwrap();
        $expr
    }};
}

#[cfg(target_family = "wasm")]
macro_rules! with_lock {
    ($expr:expr) => { $expr };  // No-op for WASM
}
```

**WASM builds:** Single-threaded, no mutex needed (macro is a no-op).

**Impact:**
- Correctness: Prevents race conditions and data corruption
- Performance: Mutex contention can cause slowdowns in multi-threaded workloads
- Mitigation: Minimize FFI calls, batch operations, cache results

**Why not build WITH_SYMENGINE_THREAD_SAFE?**
- Native `/usr/local/lib/libsymengine.a` was not compiled with thread-safe flag
- Rebuilding SymEngine with thread-safety would require recompilation
- Current approach (global mutex) is simpler and sufficient for our workload

### Rule 3: Memory Management

- `basic_new_heap()` allocates on heap → must call `basic_free_heap()` eventually
- `Expr` struct wraps the pointer and handles cleanup in `Drop`
- **Don't manually free Expr** - let Rust handle it

```rust
impl Drop for Expr {
    fn drop(&mut self) {
        with_lock!({
            unsafe {
                symengine::basic_free_heap(self.ptr);
            }
        });
    }
}
```

**Memory leak potential:**
- Calling `basic_new_heap()` directly without corresponding `basic_free_heap()`
- Always use `Expr` wrapper which handles cleanup automatically

### Rule 4: Clone Safely

**Implementation:**
```rust
impl Clone for Expr {
    fn clone(&self) -> Self {
        with_lock!({
            let s = Self::to_string(self);  // Single lock acquisition
            Self::parse(&s).unwrap()
        })
    }
}
```

**Why single lock acquisition:**
- Nested locks can deadlock: `with_lock!({ with_lock!({ ... }) })`
- Acquire lock once, perform all operations, release

**Alternative approaches considered:**
1. Deep copy FFI function - not available in SymEngine C API
2. Reference counting - too complex for current use case
3. String round-trip - chosen for simplicity

## Architecture

### Conditional Compilation

```rust
// Detect build target
#[cfg(target_family = "wasm")]
const USES_WASM: bool = true;

#[cfg(not(target_family = "wasm"))]
const USES_WASM: bool = false;
```

**Why conditional compilation:**
- WASM and native builds link different libraries
- Thread safety requirements differ
- Allocator bridge only needed for WASM

### Build System Integration

#### WASM Build (`common/build.rs`)

```rust
#[cfg(target_family = "wasm")]
{
    // Link from symengine.js/dist/wasm-unknown/lib/
    println!("cargo:rustc-link-search=native=../symengine.js/dist/wasm-unknown/lib");
    println!("cargo:rustc-link-lib=static=symengine");
    println!("cargo:rustc-link-lib=static=c++");
    println!("cargo:rustc-link-lib=static=c++abi");
    println!("cargo:rustc-link-lib=static=c");

    // Compile wasi_stub.c for missing WASI functions
    cc::Build::new()
        .file("wasi_stub.c")
        .compile("wasi_stub");
}
```

**Libraries:**
- `libsymengine.a` - Core SymEngine library
- `libc++.a` - C++ standard library (LLVM)
- `libc++abi.a` - C++ ABI library
- `libc.a` - WASI libc (with dlmalloc stripped)

**wasi_stub.c:**
Provides stub implementations for WASI functions not available in browser environment.

#### Native Build (`common/build.rs`)

```rust
#[cfg(not(target_family = "wasm"))]
{
    // Link static SymEngine library
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=static=symengine");

    // Link dynamic system libraries
    println!("cargo:rustc-link-lib=dylib=gmp");    // GNU Multi-Precision library
    println!("cargo:rustc-link-lib=dylib=stdc++"); // C++ standard library
}
```

**Why static SymEngine, dynamic gmp/stdc++?**
- SymEngine: Static linking ensures version consistency
- gmp/stdc++: System libraries, dynamic linking reduces binary size

### Allocator Bridge (WASM Only)

**Critical:** WASM builds **MUST** provide C allocator functions in `frontend/src/main.rs`:

```rust
use std::alloc::{alloc, dealloc, realloc, Layout};

#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    if size == 0 {
        return std::ptr::null_mut();
    }
    let layout = Layout::from_size_align(size, 8).unwrap();
    unsafe { alloc(layout) }
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    // Note: We don't know the original size, so we can't call dealloc properly
    // This is a known limitation of the allocator bridge approach
}

#[no_mangle]
pub extern "C" fn calloc(nmemb: usize, size: usize) -> *mut u8 {
    let total_size = nmemb * size;
    let ptr = malloc(total_size);
    if !ptr.is_null() {
        unsafe {
            std::ptr::write_bytes(ptr, 0, total_size);
        }
    }
    ptr
}

#[no_mangle]
pub extern "C" fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    if ptr.is_null() {
        return malloc(size);
    }
    if size == 0 {
        free(ptr);
        return std::ptr::null_mut();
    }

    // Allocate new memory, copy old data, free old memory
    let new_ptr = malloc(size);
    if !new_ptr.is_null() && !ptr.is_null() {
        // Note: We don't know old size, so we can't safely copy
        // This is a limitation - may cause bugs if old size < new size
        unsafe {
            std::ptr::copy_nonoverlapping(ptr, new_ptr, size);
        }
        free(ptr);
    }
    new_ptr
}
```

**Why this is needed:**
- SymEngine C++ code calls `malloc`, `free`, `calloc`, `realloc`
- WASM environment expects these to come from "env" module
- Without bridge: **"module 'env' not found" errors**
- wasi-libc's dlmalloc is stripped from libc.a to prevent dual-allocator conflict

**Known limitations:**
- `free()` doesn't know original allocation size (can't call dealloc properly)
- `realloc()` doesn't know old size (may copy incorrect amount)
- This works for SymEngine's usage patterns but is not a general solution

**Alternative approaches considered:**
1. **wasm-bindgen allocator** - doesn't integrate with C code
2. **Custom allocator** - too complex, error-prone
3. **Current approach** - pragmatic, works for our use case

## Common Patterns

### Safe Expression Checking

```rust
pub fn check_zero_safe(expr: &Expr) -> bool {
    with_lock!({
        if symengine::is_a_Number(expr.ptr) {
            // Safe: we checked type first
            symengine::number_is_zero(expr.ptr) != 0
        } else {
            // Expand and check symbolically
            let expanded = expand(expr);
            symengine::is_a_Number(expanded.ptr)
                && symengine::number_is_zero(expanded.ptr) != 0
        }
    })
}
```

### Parsing and Error Handling

```rust
pub fn safe_parse(s: &str) -> Result<Expr, String> {
    with_lock!({
        let expr = Expr::new();
        let c_str = CString::new(s).map_err(|_| "Invalid string (contains null byte)")?;

        unsafe {
            symengine::basic_parse(&mut expr.ptr, c_str.as_ptr());
        }

        // Verify parsing succeeded by checking if result is valid
        if expr.ptr.is_null() {
            return Err(format!("Failed to parse: {}", s));
        }

        Ok(expr)
    })
}
```

### Substitution with Multiple Variables

```rust
pub fn substitute_multiple(expr: &Expr, subs: &[(Expr, Expr)]) -> Expr {
    with_lock!({
        let mut result = expr.clone();
        for (var, val) in subs {
            result = subs2(&result, var, val);
        }
        result
    })
}
```

### Expansion and Simplification

```rust
pub fn expand_and_simplify(expr: &Expr) -> Expr {
    with_lock!({
        let expanded = expand(expr);
        // SymEngine automatically simplifies during expansion
        expanded
    })
}
```

## Debugging Tips

### Segfault Checklist

1. **Did you guard `number_is_zero()` with `is_a_Number()`?**
   - Search codebase: `git grep -n "number_is_zero"`
   - Verify each call has type guard

2. **Are you using `with_lock!` in native builds?**
   - All FFI calls in `common/src/symengine.rs` should use macro
   - Check for direct unsafe calls outside macro

3. **Is the allocator bridge present in WASM builds?**
   - Verify `frontend/src/main.rs` has `malloc`, `free`, `calloc`, `realloc`
   - Check for "env" module errors in browser console

4. **Are you calling `basic_free_heap()` manually?**
   - Don't - use `Expr` wrapper
   - Search for manual free calls: `git grep -n "basic_free_heap"`

### Performance Issues

**Symptom:** Slow performance in multi-threaded backend

**Cause:** Mutex contention on `SYMENGINE_LOCK`

**Diagnosis:**
```rust
// Add logging to measure lock contention
with_lock!({
    let start = std::time::Instant::now();
    let result = /* ... operation ... */;
    let elapsed = start.elapsed();
    if elapsed > Duration::from_millis(100) {
        eprintln!("Slow SymEngine operation: {:?}", elapsed);
    }
    result
});
```

**Solutions:**
1. **Minimize FFI calls** - Cache results where possible
2. **Batch operations** - Combine multiple operations into one lock acquisition
3. **Async awareness** - Don't hold lock across await points
4. **Consider thread pool** - Dedicate threads for SymEngine operations

### WASM Import Errors

**Symptom:** Browser console shows "module 'env' not found" or "undefined symbol: malloc"

**Cause:** Missing allocator bridge

**Solution:**
1. Verify allocator functions in `frontend/src/main.rs`
2. Check build output for linker errors
3. Inspect WASM binary: `wasm-objdump -x frontend.wasm | grep malloc`

**Debugging commands:**
```bash
# Check if symbols are exported
wasm-objdump -x target/wasm32-unknown-unknown/release/frontend.wasm | grep -E "malloc|free"

# Inspect imports
wasm-objdump -x target/wasm32-unknown-unknown/release/frontend.wasm | grep -A 10 "Import"
```

### Memory Leaks

**Symptom:** Growing memory usage over time

**Diagnosis:**
```rust
// Add Drop logging to track Expr lifecycle
impl Drop for Expr {
    fn drop(&mut self) {
        eprintln!("Dropping Expr: {:p}", self.ptr);
        with_lock!({
            unsafe {
                symengine::basic_free_heap(self.ptr);
            }
        });
    }
}
```

**Common causes:**
1. Storing Expr in long-lived data structures
2. Circular references (Rust prevents these, but good to check)
3. Manual memory management bypassing Drop

## Testing SymEngine Code

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_guard_safety() {
        let symbol = Expr::parse("x").unwrap();
        assert!(!is_a_Number(&symbol.ptr));

        let number = Expr::parse("42").unwrap();
        assert!(is_a_Number(&number.ptr));
    }

    #[test]
    fn test_parse_valid_expression() {
        let expr = Expr::parse("x^2 + 2*x + 1").unwrap();
        let s = expr.to_string();
        assert!(s.contains("x"));
    }

    #[test]
    fn test_parse_invalid_expression() {
        // SymEngine is lenient, but this documents behavior
        let result = Expr::parse("invalid!!!");
        // May parse or return error - check actual behavior
    }

    #[test]
    fn test_expand() {
        let expr = Expr::parse("(x+1)*(x-1)").unwrap();
        let expanded = expand(&expr);
        let s = expanded.to_string();
        assert!(s.contains("x**2") || s.contains("x^2"));
    }

    #[test]
    fn test_substitute() {
        let expr = Expr::parse("x + y").unwrap();
        let x = Expr::parse("x").unwrap();
        let five = Expr::parse("5").unwrap();

        let result = subs2(&expr, &x, &five);
        assert_eq!(result.to_string(), "y + 5"); // or "5 + y"
    }

    #[test]
    fn test_numerical_evaluation() {
        let expr = Expr::parse("sin(0)").unwrap();
        let evaled = evalf(&expr);

        if is_a_Number(&evaled.ptr) && number_is_zero(&evaled.ptr) != 0 {
            // sin(0) should be 0
        } else {
            panic!("sin(0) should evaluate to 0");
        }
    }
}
```

### Integration Tests

```rust
// Test grading system that uses SymEngine
#[test]
fn test_grading_with_symengine() {
    use common::grader::{check_answer_expr, GradingMode};

    // Equivalent expressions
    assert!(check_answer_expr("x^2 - 1", "(x+1)*(x-1)", GradingMode::Equivalent).unwrap());

    // Factor mode
    assert!(check_answer_expr("(x+1)*(x-1)", "(x+1)*(x-1)", GradingMode::Factor).unwrap());
    assert!(check_answer_expr("x^2 - 1", "(x+1)*(x-1)", GradingMode::Factor).is_err());

    // Expand mode
    assert!(check_answer_expr("x^2 - 1", "x^2 - 1", GradingMode::Expand).unwrap());
    assert!(check_answer_expr("(x+1)*(x-1)", "x^2 - 1", GradingMode::Expand).is_err());
}
```

### Fuzz Testing (Future)

```rust
#[test]
#[ignore] // Run manually with: cargo test --ignored
fn fuzz_random_expressions() {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let symbols = vec!["x", "y", "z"];
    let operators = vec!["+", "-", "*", "/", "^"];

    for _ in 0..1000 {
        // Generate random expression
        let expr_str = /* ... random generation ... */;

        // Should not crash
        let _ = Expr::parse(&expr_str);
    }
}
```

## Further Reading

### SymEngine Documentation
- **C API Reference:** https://github.com/symengine/symengine/blob/master/symengine/cwrapper.h
- **Core concepts:** https://github.com/symengine/symengine/wiki
- **Examples:** https://github.com/symengine/symengine/tree/master/benchmarks

### Rust FFI
- **Nomicon (FFI chapter):** https://doc.rust-lang.org/nomicon/ffi.html
- **Safety guidelines:** https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
- **std::ffi module:** https://doc.rust-lang.org/std/ffi/index.html

### WASM Integration
- **symengine.js:** https://github.com/symengine/symengine.js
- **Rust WASM book:** https://rustwasm.github.io/docs/book/
- **wasm-bindgen:** https://rustwasm.github.io/docs/wasm-bindgen/

### Memory Management
- **Rust allocator:** https://doc.rust-lang.org/std/alloc/index.html
- **Custom allocators:** https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html
- **WASM memory model:** https://webassembly.github.io/spec/core/syntax/modules.html#memories

## Quick Reference Card

| Operation | Safe Pattern | Common Mistake |
|-----------|--------------|----------------|
| Check if zero | `is_a_Number(e) && number_is_zero(e)` | `number_is_zero(e)` |
| FFI call (native) | `with_lock!({ unsafe { ... } })` | `unsafe { ... }` |
| FFI call (WASM) | `unsafe { ... }` (no lock) | N/A |
| Parse | `Expr::parse(s)?` | Unwrapping errors |
| Memory mgmt | Use `Expr` wrapper | Manual `basic_free_heap` |
| Clone | `expr.clone()` | Manual string round-trip |
| Substitution | `subs2(&e, &var, &val)` | Forgetting `with_lock!` |

## Version Compatibility

| Component | Version | Notes |
|-----------|---------|-------|
| SymEngine (native) | 0.11+ | At `/usr/local/lib/libsymengine.a` |
| SymEngine (WASM) | 0.9.0 | From symengine.js |
| Rust | 1.70+ | Requires `Edition 2024` for frontend |
| WASM target | wasm32-unknown-unknown | Standard WASM target |
| GMP (native) | System version | Dynamic linking |

**Compatibility notes:**
- WASM version (0.9.0) is older than native (0.11+) - may have feature differences
- Test both WASM and native builds when adding SymEngine functionality
- Some functions may not be available in WASM build

## Change Log

### 2024-01-XX: Initial Documentation
- Documented 4 safety rules
- Added common patterns
- Created debugging checklist
- Added testing examples

### Future Additions
- [ ] Benchmark SymEngine performance
- [ ] Document all available FFI functions
- [ ] Add migration guide for SymEngine upgrades
- [ ] Create SymEngine cheat sheet poster
