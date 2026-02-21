//! OAuth popup helper

use locus_common::AuthResponse;
use wasm_bindgen::prelude::*;
use web_sys::Window;

// Import centralized environment configuration
// CRITICAL: Never hardcode production URLs - always use crate::env functions
use crate::env;

/// Open an OAuth popup and listen for the result via postMessage
///
/// `url` should be the full OAuth URL (e.g., "{API_BASE}/auth/oauth/google")
pub fn open_oauth_popup(
    url: &str,
    on_success: impl Fn(AuthResponse) + 'static,
    on_error: impl Fn(String) + 'static,
) {
    let window: Window = web_sys::window().expect("no global window");
    let popup = window.open_with_url_and_target_and_features(
        url,
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
    let closure =
        Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |event: web_sys::MessageEvent| {
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
                                if let Ok(auth) =
                                    serde_json::from_value::<AuthResponse>(data.clone())
                                {
                                    on_success(auth);
                                }
                            }
                        }
                        "oauth_error" => {
                            let error = msg
                                .get("error")
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

/// Open an OAuth popup for login (shortcut for provider name)
pub fn open_oauth_login_popup(
    provider: &str,
    on_success: impl Fn(AuthResponse) + 'static,
    on_error: impl Fn(String) + 'static,
) {
    let url = format!("{}/auth/oauth/{}", env::api_base(), provider);
    open_oauth_popup(&url, on_success, on_error);
}

/// Open an OAuth popup for linking (shortcut for provider name)
pub fn open_oauth_link_popup(
    provider: &str,
    on_success: impl Fn(AuthResponse) + 'static,
    on_error: impl Fn(String) + 'static,
) {
    use gloo_storage::{LocalStorage, Storage};
    use wasm_bindgen::JsValue;

    // Get the current auth token
    let token = match LocalStorage::get::<String>("locus_token") {
        Ok(token) => token,
        Err(_) => {
            on_error("Not authenticated. Please log in first.".to_string());
            return;
        }
    };

    // URL encode the token using JavaScript's encodeURIComponent
    let encoded_token = js_sys::encode_uri_component(&token);
    let url = format!(
        "{}/auth/oauth/link/{}?token={}",
        env::api_base(),
        provider,
        encoded_token
    );
    open_oauth_popup(&url, on_success, on_error);
}
