//! Forgot password page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;

use crate::api;

#[component]
pub fn ForgotPassword() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(None::<String>);
    let (loading, set_loading) = signal(false);

    let submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(None);
        set_success.set(None);

        let email_val = email.get();

        spawn_local(async move {
            match api::forgot_password(&email_val).await {
                Ok(resp) => {
                    set_success.set(Some(resp.message));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <div class="max-w-sm mx-auto py-16">
            <h1 class="text-xl font-medium text-gray-900 mb-2">"Reset Password"</h1>
            <p class="text-sm text-gray-600 mb-6">
                "Enter your email address and we'll send you a link to reset your password."
            </p>

            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4 p-3 bg-red-50 rounded border border-red-200">
                    {e}
                </div>
            })}

            {move || success.get().map(|s| view! {
                <div class="text-green-700 text-sm mb-4 p-3 bg-green-50 rounded border border-green-200">
                    {s}
                </div>
            })}

            <form on:submit=submit class="space-y-4">
                <div>
                    <label class="block text-sm text-gray-600 mb-1">"Email"</label>
                    <input
                        type="email"
                        class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                        prop:value=email
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                        required
                        placeholder="your@email.com"
                    />
                </div>

                <button
                    type="submit"
                    class="w-full px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 disabled:opacity-50"
                    disabled=loading
                >
                    {move || if loading.get() { "Sending..." } else { "Send Reset Link" }}
                </button>
            </form>

            <p class="text-sm text-gray-500 mt-6 text-center">
                <A href="/login" attr:class="text-gray-900 hover:underline">"Back to login"</A>
            </p>
        </div>
    }
}
