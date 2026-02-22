//! Locus Frontend - Competitive Math Platform

mod api;
mod components;
mod env;
mod formatters;
mod grader;
mod katex_bindings;
mod oauth;
mod pages;
pub mod problem_queue;
mod utils;

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

use components::{Navbar, Sidebar};
use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};
use pages::{
    ForgotPassword, Home, Leaderboard, Login, Practice, PrivacyPolicy, Ranked, Register,
    ResetPassword, Settings, TermsOfService, VerifyEmail,
};

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

    // Theme state — read initial value from <html> class (set by inline script)
    let initial_dark = leptos::web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
        .map(|el| el.class_list().contains("dark"))
        .unwrap_or(false);

    let (is_dark, set_is_dark) = signal(initial_dark);

    let toggle_theme = Callback::new(move |_: ()| {
        let new_dark = !is_dark.get_untracked();
        set_is_dark.set(new_dark);

        if let Some(doc) = leptos::web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
        {
            let class_list = doc.class_list();
            if new_dark {
                let _ = class_list.add_1("dark");
            } else {
                let _ = class_list.remove_1("dark");
            }
        }

        if let Some(storage) =
            leptos::web_sys::window().and_then(|w| w.local_storage().ok().flatten())
        {
            let _ = storage.set_item("theme", if new_dark { "dark" } else { "light" });
        }
    });

    provide_context(ThemeContext {
        is_dark,
        toggle_theme,
    });

    let auth = expect_context::<AuthContext>();

    view! {
        <Router>
            <div class="min-h-screen flex flex-col">
                <Navbar />
                {move || auth.is_logged_in.get().then(|| view! {
                    <Sidebar />
                })}
                <main class=move || format!(
                    "flex-1 transition-all duration-300 {}",
                    if auth.is_logged_in.get() { "ml-16" } else { "" }
                )>
                    <Routes fallback=|| view! { <p class="text-center mt-12">"Page not found"</p> }>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/practice") view=Practice />
                        <Route path=path!("/ranked") view=Ranked />
                        <Route path=path!("/leaderboard") view=Leaderboard />
                        <Route path=path!("/login") view=Login />
                        <Route path=path!("/register") view=Register />
                        <Route path=path!("/verify-email") view=VerifyEmail />
                        <Route path=path!("/forgot-password") view=ForgotPassword />
                        <Route path=path!("/reset-password") view=ResetPassword />
                        <Route path=path!("/settings") view=Settings />
                        <Route path=path!("/privacy-policy") view=PrivacyPolicy />
                        <Route path=path!("/terms-of-service") view=TermsOfService />
                    </Routes>
                </main>
                <footer class=move || format!(
                    "py-4 px-6 text-xs text-gray-400 border-t transition-all duration-300 flex justify-between items-center {}",
                    if auth.is_logged_in.get() { "ml-16" } else { "" }
                )>
                    <div class="flex space-x-4">
                        <A href="/privacy-policy" attr:class="hover:text-gray-600">"Privacy Policy"</A>
                        <A href="/terms-of-service" attr:class="hover:text-gray-600">"Terms of Service"</A>
                    </div>
                    <button
                        on:click=move |_| toggle_theme.run(())
                        class="p-1.5 rounded hover:bg-gray-100 transition-colors"
                        title=move || if is_dark.get() { "Switch to light mode" } else { "Switch to dark mode" }
                    >
                        {move || if is_dark.get() {
                            view! {
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path>
                                </svg>
                            }
                        } else {
                            view! {
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path>
                                </svg>
                            }
                        }}
                    </button>
                    <span>
                        {if cfg!(debug_assertions) {
                            format!("Locus - {} - {}", env!("CARGO_PKG_VERSION"), env!("BUILD_TIMESTAMP"))
                        } else {
                            format!("Locus - {}", env!("CARGO_PKG_VERSION"))
                        }}
                    </span>
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

/// Global theme context
#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub is_dark: ReadSignal<bool>,
    pub toggle_theme: Callback<()>,
}
