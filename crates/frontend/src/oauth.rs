//! OAuth popup helper

use locus_common::AuthResponse;
use wasm_bindgen::prelude::*;
use web_sys::Window;

// API base URL - must match api.rs
const API_BASE: &str = match option_env!("LOCUS_API_URL") {
    Some(url) => url,
    None => "/api",
};

/// Open an OAuth popup and listen for the result via postMessage
pub fn open_oauth_popup(
    provider: &str,
    on_success: impl Fn(AuthResponse) + 'static,
    on_error: impl Fn(String) + 'static,
) {
    let window: Window = web_sys::window().expect("no global window");

    let url = format!("{}/auth/oauth/{}", API_BASE, provider);
    let popup = window.open_with_url_and_target_and_features(
        &url,
        "_blank",
        "width=500,height=600,popup=yes",
    );

    match popup {
        Ok(Some(_)) => {}
        _ => {
            on_error("Failed to open popup. Please allow popups for this site.".into());
            return;
        }
    }

    // Listen for postMessage from the popup
    let closure = Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |event: web_sys::MessageEvent| {
        // Only process messages with the expected oauth type (CSRF is validated server-side)
        let data = event.data();
        let js_string = match js_sys::JSON::stringify(&data) {
            Ok(s) => String::from(s),
            Err(_) => return,
        };

        // Parse the message
        if let Ok(msg) = serde_json::from_str::<serde_json::Value>(&js_string) {
            if let Some(msg_type) = msg.get("type").and_then(|t| t.as_str()) {
                match msg_type {
                    "oauth_success" => {
                        if let Some(data) = msg.get("data") {
                            if let Ok(auth) = serde_json::from_value::<AuthResponse>(data.clone()) {
                                on_success(auth);
                            }
                        }
                    }
                    "oauth_error" => {
                        let error = msg.get("error")
                            .and_then(|e| e.as_str())
                            .unwrap_or("OAuth sign-in failed")
                            .to_string();
                        on_error(error);
                    }
                    _ => {}
                }
            }
        }
    });

    window
        .add_event_listener_with_callback("message", closure.as_ref().unchecked_ref())
        .expect("failed to add message listener");

    // Leak the closure so it lives for the duration of the page
    // (one per OAuth attempt; acceptable since users don't spam OAuth logins)
    closure.forget();
}
