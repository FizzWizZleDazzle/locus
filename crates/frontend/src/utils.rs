//! Utility functions

use serde::Serialize;

/// Escape a string for safe embedding in HTML content.
pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

#[derive(Serialize)]
struct HistoryState {
    mode: String,
}

/// Update the browser URL without triggering navigation
/// Uses replaceState to avoid creating history entries during topic selection
pub fn update_url(url: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(history) = window.history().ok() {
            let state = HistoryState {
                mode: "selecting".to_string(),
            };
            if let Ok(state_js) = serde_wasm_bindgen::to_value(&state) {
                let _ = history.replace_state_with_url(&state_js, "", Some(url));
            }
        }
    }
}

/// Push a new entry to browser history for playing mode
/// Used when transitioning to problem-solving (clicking "Start")
pub fn push_url_playing(url: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(history) = window.history().ok() {
            let state = HistoryState {
                mode: "playing".to_string(),
            };
            if let Ok(state_js) = serde_wasm_bindgen::to_value(&state) {
                let _ = history.push_state_with_url(&state_js, "", Some(url));
            }
        }
    }
}

/// Set up a popstate listener that clears problem state when navigating back to selection mode
/// Returns a cleanup function that should be called on unmount
pub fn setup_popstate_listener<F>(on_back_to_selecting: F)
where
    F: Fn() + 'static,
{
    let window = web_sys::window().expect("window");

    let closure = Closure::wrap(Box::new(move |_event: web_sys::PopStateEvent| {
        // Check history state to determine if we should show problem or selector
        if let Some(window) = web_sys::window() {
            if let Ok(history) = window.history() {
                if let Ok(state) = history.state() {
                    // Try to parse the state object
                    if let Ok(state_str) = js_sys::JSON::stringify(&state) {
                        let state_str = state_str.as_string().unwrap_or_default();
                        // If state contains "selecting" mode, clear the problem
                        if state_str.contains("selecting") {
                            on_back_to_selecting();
                        }
                    }
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    window
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .expect("add popstate listener");

    // Keep closure alive
    closure.forget();
}
