//! Locus Frontend - Competitive Math Platform

mod api;
mod grader;
mod pages;
mod components;
mod katex_bindings;

// ---------------------------------------------------------------------------
// C-compatible allocator bridge
// ---------------------------------------------------------------------------
// wasi-libc's dlmalloc is stripped from the shipped libc.a to avoid a
// dual-allocator conflict with Rust's own dlmalloc.  The C/C++ code
// (SymEngine, libc++, libc) calls malloc/free/calloc/realloc which we
// provide here, delegating to Rust's built-in allocator.
//
// We store the usable size just before the returned pointer so that
// free() can reconstruct the Layout.
// ---------------------------------------------------------------------------

const HEADER: usize = 16; // enough room for a usize, keeps 16-byte alignment

// SAFETY: Symbol name "malloc" is unique within this crate and designed to override
// the standard C library malloc for WASM compatibility with SymEngine FFI
#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    unsafe {
        if size == 0 {
            return core::ptr::null_mut();
        }
        let total = size + HEADER;
        let layout = core::alloc::Layout::from_size_align_unchecked(total, HEADER);
        let raw = std::alloc::alloc(layout);
        if raw.is_null() {
            return raw;
        }
        *(raw as *mut usize) = size;
        raw.add(HEADER)
    }
}

// SAFETY: Symbol name "free" is unique and designed to pair with our malloc
// implementation for WASM compatibility
#[unsafe(no_mangle)]
pub extern "C" fn free(ptr: *mut u8) {
    unsafe {
        if ptr.is_null() {
            return;
        }
        let raw = ptr.sub(HEADER);
        let size = *(raw as *mut usize);
        let total = size + HEADER;
        let layout = core::alloc::Layout::from_size_align_unchecked(total, HEADER);
        std::alloc::dealloc(raw, layout);
    }
}

// SAFETY: Symbol name "calloc" is unique and designed for WASM compatibility
// with SymEngine FFI
#[unsafe(no_mangle)]
pub extern "C" fn calloc(nmemb: usize, size: usize) -> *mut u8 {
    unsafe {
        let total_size = match nmemb.checked_mul(size) {
            Some(s) => s,
            None => return core::ptr::null_mut(),
        };
        if total_size == 0 {
            return core::ptr::null_mut();
        }
        let total = total_size + HEADER;
        let layout = core::alloc::Layout::from_size_align_unchecked(total, HEADER);
        let raw = std::alloc::alloc_zeroed(layout);
        if raw.is_null() {
            return raw;
        }
        *(raw as *mut usize) = total_size;
        raw.add(HEADER)
    }
}

// SAFETY: Symbol name "realloc" is unique and designed for WASM compatibility
// with SymEngine FFI
#[unsafe(no_mangle)]
pub extern "C" fn realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
    unsafe {
        if ptr.is_null() {
            return malloc(new_size);
        }
        if new_size == 0 {
            free(ptr);
            return core::ptr::null_mut();
        }
        let raw = ptr.sub(HEADER);
        let old_size = *(raw as *mut usize);
        let old_total = old_size + HEADER;
        let new_total = new_size + HEADER;
        let layout = core::alloc::Layout::from_size_align_unchecked(old_total, HEADER);
        let new_raw = std::alloc::realloc(raw, layout, new_total);
        if new_raw.is_null() {
            return new_raw;
        }
        *(new_raw as *mut usize) = new_size;
        new_raw.add(HEADER)
    }
}

// Internal libc aliases used by wasi-libc internals
// SAFETY: Unique internal symbol that delegates to our malloc implementation
#[unsafe(no_mangle)]
pub extern "C" fn __libc_malloc(size: usize) -> *mut u8 {
    malloc(size)
}

// SAFETY: Unique internal symbol that delegates to our free implementation
#[unsafe(no_mangle)]
pub extern "C" fn __libc_free(ptr: *mut u8) {
    free(ptr)
}

// SAFETY: Unique internal symbol that delegates to our calloc implementation
#[unsafe(no_mangle)]
pub extern "C" fn __libc_calloc(nmemb: usize, size: usize) -> *mut u8 {
    calloc(nmemb, size)
}

// SAFETY: Unique internal symbol that delegates to our realloc implementation
#[unsafe(no_mangle)]
pub extern "C" fn __libc_realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    realloc(ptr, size)
}

use leptos::prelude::*;
use leptos_router::{
    components::{Router, Route, Routes},
    path,
};

use pages::{Home, Practice, Ranked, Leaderboard, Login, Register};
use components::Navbar;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // Global auth state
    let (is_logged_in, set_logged_in) = signal(api::is_logged_in());
    let (username, set_username) = signal(api::get_stored_username());

    // Provide auth context to all components
    provide_context(AuthContext {
        is_logged_in,
        set_logged_in,
        username,
        set_username,
    });

    view! {
        <Router>
            <div class="min-h-screen flex flex-col">
                <Navbar />
                <main class="flex-1">
                    <Routes fallback=|| view! { <p class="text-center mt-12">"Page not found"</p> }>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/practice") view=Practice />
                        <Route path=path!("/ranked") view=Ranked />
                        <Route path=path!("/leaderboard") view=Leaderboard />
                        <Route path=path!("/login") view=Login />
                        <Route path=path!("/register") view=Register />
                    </Routes>
                </main>
                <footer class="py-6 text-center text-xs text-gray-400 border-t">
                    "Locus"
                </footer>
            </div>
        </Router>
    }
}

/// Global authentication context
#[derive(Clone, Copy)]
pub struct AuthContext {
    pub is_logged_in: ReadSignal<bool>,
    pub set_logged_in: WriteSignal<bool>,
    pub username: ReadSignal<Option<String>>,
    pub set_username: WriteSignal<Option<String>>,
}
