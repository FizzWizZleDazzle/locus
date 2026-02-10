// Stub implementations for WASI and C runtime functions required by wasi-libc
// and libc++ when running in the wasm32-unknown-unknown environment.
//
// These allow libsymengine.a (compiled with wasi-sdk) to work without a full
// WASI runtime.  Error paths (exceptions, abort) trap the WASM instance.

typedef long ssize_t;
typedef unsigned long size_t;
typedef unsigned short __wasi_errno_t;
typedef unsigned int __wasi_fd_t;
typedef unsigned long long __wasi_timestamp_t;
typedef unsigned char __wasi_clockid_t;

#define __WASI_ERRNO_NOSYS 52

struct __wasi_ciovec_t {
    const void *buf;
    size_t buf_len;
};

struct __wasi_iovec_t {
    void *buf;
    size_t buf_len;
};

// =============================================================================
// WASI snapshot preview1 stubs
// =============================================================================

__wasi_errno_t __imported_wasi_snapshot_preview1_fd_write(
    __wasi_fd_t fd, const struct __wasi_ciovec_t *iovs, size_t iovs_len,
    size_t *nwritten) {
    (void)fd; (void)iovs; (void)iovs_len;
    if (nwritten) {
        size_t total = 0;
        for (size_t i = 0; i < iovs_len; i++)
            total += iovs[i].buf_len;
        *nwritten = total;
    }
    return 0;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_fd_read(
    __wasi_fd_t fd, const struct __wasi_iovec_t *iovs, size_t iovs_len,
    size_t *nread) {
    (void)fd; (void)iovs; (void)iovs_len;
    if (nread) *nread = 0;
    return 0;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_fd_close(__wasi_fd_t fd) {
    (void)fd;
    return 0;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_fd_seek(
    __wasi_fd_t fd, long long offset, unsigned char whence,
    unsigned long long *newoffset) {
    (void)fd; (void)offset; (void)whence;
    if (newoffset) *newoffset = 0;
    return __WASI_ERRNO_NOSYS;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_fd_fdstat_get(
    __wasi_fd_t fd, void *stat) {
    (void)fd; (void)stat;
    return __WASI_ERRNO_NOSYS;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_fd_prestat_get(
    __wasi_fd_t fd, void *prestat) {
    (void)fd; (void)prestat;
    return __WASI_ERRNO_NOSYS;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_fd_prestat_dir_name(
    __wasi_fd_t fd, void *path, size_t path_len) {
    (void)fd; (void)path; (void)path_len;
    return __WASI_ERRNO_NOSYS;
}

_Noreturn void __imported_wasi_snapshot_preview1_proc_exit(
    unsigned int code) {
    (void)code;
    __builtin_trap();
}

__wasi_errno_t __imported_wasi_snapshot_preview1_environ_get(
    void **environ, void *environ_buf) {
    (void)environ; (void)environ_buf;
    return 0;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_environ_sizes_get(
    size_t *environ_count, size_t *environ_buf_size) {
    if (environ_count) *environ_count = 0;
    if (environ_buf_size) *environ_buf_size = 0;
    return 0;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_clock_time_get(
    __wasi_clockid_t id, __wasi_timestamp_t precision,
    __wasi_timestamp_t *time) {
    (void)id; (void)precision;
    if (time) *time = 0;
    return 0;
}

// =============================================================================
// C++ atexit stub — prevent global destructor registration
// =============================================================================
// wasm-bindgen's "command" pattern calls __wasm_call_ctors/__wasm_call_dtors
// around EVERY export invocation.  If C++ global destructors are registered
// via __cxa_atexit, they'll destroy SymEngine's static constants (BooleanAtom,
// etc.) after the first call, then subsequent calls crash when those objects
// are accessed again.  By making __cxa_atexit a no-op, globals are constructed
// once and never destroyed — which is correct for browser lifetime.

int __cxa_atexit(void (*func)(void *), void *arg, void *dso_handle) {
    (void)func; (void)arg; (void)dso_handle;
    return 0;
}

// =============================================================================
// C++ exception stubs (SymEngine compiled with -fno-exceptions, but libc++
// may still reference these symbols)
// =============================================================================

void *__cxa_allocate_exception(size_t size) {
    (void)size;
    __builtin_trap();
    return 0; // unreachable
}

_Noreturn void __cxa_throw(void *thrown_exception, void *tinfo, void (*dest)(void *)) {
    (void)thrown_exception; (void)tinfo; (void)dest;
    __builtin_trap();
}

// =============================================================================
// Compiler-rt complex arithmetic builtins
// =============================================================================

struct dc { double real; double imag; };

struct dc __muldc3(double a, double b, double c, double d) {
    // (a+bi) * (c+di) = (ac-bd) + (ad+bc)i
    struct dc r;
    r.real = a * c - b * d;
    r.imag = a * d + b * c;
    return r;
}

struct dc __divdc3(double a, double b, double c, double d) {
    // (a+bi) / (c+di) = ((ac+bd) + (bc-ad)i) / (c^2+d^2)
    struct dc r;
    double denom = c * c + d * d;
    if (denom == 0.0) {
        r.real = 0.0;
        r.imag = 0.0;
    } else {
        r.real = (a * c + b * d) / denom;
        r.imag = (b * c - a * d) / denom;
    }
    return r;
}
