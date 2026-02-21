//! Reset password page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::{
    components::A,
    hooks::{use_navigate, use_query_map},
};

use crate::api;

#[component]
pub fn ResetPassword() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    let (token, set_token) = signal(String::new());
    let (new_password, set_new_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());

    let (validating, set_validating) = signal(true);
    let (token_valid, set_token_valid) = signal(false);
    let (loading, set_loading) = signal(false);
    let (success, set_success) = signal(false);
    let (error, set_error) = signal(None::<String>);

    // Validate token on mount
    Effect::new(move || {
        let token_val = query.read().get("token").unwrap_or_default();

        if token_val.is_empty() {
            set_error.set(Some("No reset token provided".to_string()));
            set_validating.set(false);
            return;
        }

        set_token.set(token_val.clone());

        spawn_local(async move {
            match api::validate_reset_token(&token_val).await {
                Ok(resp) => {
                    if resp.valid {
                        set_token_valid.set(true);
                        set_validating.set(false);
                    } else {
                        set_error.set(resp.message.or(Some("Invalid reset token".to_string())));
                        set_validating.set(false);
                    }
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_validating.set(false);
                }
            }
        });
    });

    let submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        // Client-side password match validation
        if new_password.get() != confirm_password.get() {
            set_error.set(Some("Passwords do not match".to_string()));
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        let token_val = token.get();
        let password_val = new_password.get();
        let nav = navigate.clone();

        spawn_local(async move {
            match api::reset_password(&token_val, &password_val).await {
                Ok(_) => {
                    set_success.set(true);
                    set_loading.set(false);
                    // Redirect to login after 2 seconds
                    set_timeout(
                        move || {
                            nav("/login", Default::default());
                        },
                        std::time::Duration::from_secs(2),
                    );
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    };

    let submit_handler = StoredValue::new(submit);

    view! {
        <div class="max-w-sm mx-auto py-16">
            {move || {
                if validating.get() {
                    view! {
                        <div class="text-center">
                            <h1 class="text-xl font-medium text-gray-900 mb-3">"Validating reset link..."</h1>
                            <p class="text-gray-600">"Please wait a moment."</p>
                        </div>
                    }.into_any()
                } else if success.get() {
                    view! {
                        <div class="text-center">
                            <h1 class="text-xl font-medium text-gray-900 mb-3">"Password Reset Successful!"</h1>
                            <p class="text-gray-600 mb-6">"Your password has been updated. Redirecting to login..."</p>
                            <A href="/login" attr:class="px-6 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 inline-block">
                                "Go to Login"
                            </A>
                        </div>
                    }.into_any()
                } else if !token_valid.get() {
                    view! {
                        <div class="text-center">
                            <h1 class="text-xl font-medium text-gray-900 mb-3">"Invalid Reset Link"</h1>
                            <p class="text-red-600 mb-6">{error.get().unwrap_or_else(|| "This reset link is invalid or has expired.".to_string())}</p>
                            <div class="space-y-3">
                                <A
                                    href="/forgot-password"
                                    attr:class="block px-6 py-2 bg-gray-900 text-white rounded hover:bg-gray-800"
                                >
                                    "Request New Link"
                                </A>
                                <A
                                    href="/login"
                                    attr:class="block text-sm text-gray-600 hover:text-gray-900"
                                >
                                    "Back to Login"
                                </A>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div>
                            <h1 class="text-xl font-medium text-gray-900 mb-2">"Set New Password"</h1>
                            <p class="text-sm text-gray-600 mb-6">
                                "Enter your new password below."
                            </p>

                            {move || error.get().map(|e| view! {
                                <div class="text-red-600 text-sm mb-4 p-3 bg-red-50 rounded border border-red-200">
                                    {e}
                                </div>
                            })}

                            <form on:submit=move |ev| submit_handler.with_value(|f| f(ev)) class="space-y-4">
                                <div>
                                    <label class="block text-sm text-gray-600 mb-1">"New Password"</label>
                                    <input
                                        type="password"
                                        class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                                        prop:value=new_password
                                        on:input=move |ev| set_new_password.set(event_target_value(&ev))
                                        required
                                        minlength="8"
                                        placeholder="At least 8 characters"
                                    />
                                </div>

                                <div>
                                    <label class="block text-sm text-gray-600 mb-1">"Confirm Password"</label>
                                    <input
                                        type="password"
                                        class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                                        prop:value=confirm_password
                                        on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                                        required
                                        minlength="8"
                                        placeholder="Repeat your password"
                                    />
                                </div>

                                <button
                                    type="submit"
                                    class="w-full px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 disabled:opacity-50"
                                    disabled=loading
                                >
                                    {move || if loading.get() { "Resetting..." } else { "Reset Password" }}
                                </button>
                            </form>

                            <p class="text-sm text-gray-500 mt-4 text-center">
                                <A href="/login" attr:class="text-gray-900 hover:underline">"Back to login"</A>
                            </p>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
