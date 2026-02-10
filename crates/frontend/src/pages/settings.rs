//! Settings page

use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::{api, AuthContext};

#[component]
pub fn Settings() -> impl IntoView {
    let auth = expect_context::<AuthContext>();

    let (password, set_password) = signal(String::new());
    let (confirm, set_confirm) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(None::<String>);
    let (loading, set_loading) = signal(false);
    let (has_password, set_has_password) = signal(None::<bool>);

    // Fetch user profile to check has_password
    spawn_local(async move {
        if let Ok(profile) = api::get_me().await {
            set_has_password.set(Some(profile.has_password));
        }
    });

    let submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_error.set(None);
        set_success.set(None);

        let pw = password.get();
        let cf = confirm.get();

        if pw != cf {
            set_error.set(Some("Passwords do not match".into()));
            return;
        }

        set_loading.set(true);

        spawn_local(async move {
            match api::set_password(&pw).await {
                Ok(_) => {
                    set_success.set(Some("Password set successfully".into()));
                    set_has_password.set(Some(true));
                    set_password.set(String::new());
                    set_confirm.set(String::new());
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
            <h1 class="text-xl font-medium text-gray-900 mb-6">"Settings"</h1>

            {move || {
                if !auth.is_logged_in.get() {
                    return Some(view! {
                        <p class="text-gray-500">"Please log in to access settings."</p>
                    }.into_any());
                }
                None
            }}

            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            {move || success.get().map(|s| view! {
                <div class="text-green-600 text-sm mb-4">{s}</div>
            })}

            <div class="mb-8">
                <h2 class="text-sm font-medium text-gray-700 mb-3">
                    {move || {
                        match has_password.get() {
                            Some(true) => "Change Password",
                            _ => "Set Password",
                        }
                    }}
                </h2>
                <p class="text-xs text-gray-400 mb-4">
                    {move || {
                        match has_password.get() {
                            Some(false) => "Set a password to also sign in with email and password.",
                            _ => "Update your password.",
                        }
                    }}
                </p>

                <form on:submit=submit class="space-y-4">
                    <div>
                        <label class="block text-sm text-gray-600 mb-1">"New Password"</label>
                        <input
                            type="password"
                            class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                            minlength="8"
                            required
                        />
                        <p class="text-xs text-gray-400 mt-1">"At least 8 characters"</p>
                    </div>

                    <div>
                        <label class="block text-sm text-gray-600 mb-1">"Confirm Password"</label>
                        <input
                            type="password"
                            class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                            prop:value=confirm
                            on:input=move |ev| set_confirm.set(event_target_value(&ev))
                            minlength="8"
                            required
                        />
                    </div>

                    <button
                        type="submit"
                        class="w-full px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 disabled:opacity-50"
                        disabled=loading
                    >
                        {move || if loading.get() { "Saving..." } else { "Set Password" }}
                    </button>
                </form>
            </div>
        </div>
    }
}
