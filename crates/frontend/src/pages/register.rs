//! Registration page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::{components::A, hooks::use_navigate};

use crate::{api, AuthContext};

#[component]
pub fn Register() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = use_navigate();

    let (username, set_username) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (loading, set_loading) = signal(false);

    let submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(None);

        let username_val = username.get();
        let email_val = email.get();
        let password_val = password.get();
        let nav = navigate.clone();

        spawn_local(async move {
            match api::register(&username_val, &email_val, &password_val).await {
                Ok(resp) => {
                    auth.set_logged_in.set(true);
                    auth.set_username.set(Some(resp.user.username));
                    nav("/", Default::default());
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
            <h1 class="text-xl font-medium text-gray-900 mb-6">"Sign Up"</h1>

            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            <form on:submit=submit class="space-y-4">
                <div>
                    <label class="block text-sm text-gray-600 mb-1">"Username"</label>
                    <input
                        type="text"
                        class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                        prop:value=username
                        on:input=move |ev| set_username.set(event_target_value(&ev))
                        minlength="3"
                        maxlength="50"
                        required
                    />
                </div>

                <div>
                    <label class="block text-sm text-gray-600 mb-1">"Email"</label>
                    <input
                        type="email"
                        class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                        prop:value=email
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                        required
                    />
                </div>

                <div>
                    <label class="block text-sm text-gray-600 mb-1">"Password"</label>
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

                <button
                    type="submit"
                    class="w-full px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 disabled:opacity-50"
                    disabled=loading
                >
                    {move || if loading.get() { "Loading..." } else { "Create Account" }}
                </button>
            </form>

            <p class="text-sm text-gray-500 mt-4">
                "Have an account? "
                <A href="/login" attr:class="text-gray-900 hover:underline">"Login"</A>
            </p>
        </div>
    }
}
