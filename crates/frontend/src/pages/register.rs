//! Registration page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::{components::A, hooks::use_navigate};

use crate::{api, oauth, AuthContext};

#[component]
pub fn Register() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = use_navigate();

    let (username, set_username) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (loading, set_loading) = signal(false);
    let (success_email, set_success_email) = signal(None::<String>);
    let (resending, set_resending) = signal(false);

    let submit = StoredValue::new(move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        // Check if passwords match
        if password.get() != confirm_password.get() {
            set_error.set(Some("Passwords do not match".to_string()));
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        let username_val = username.get();
        let email_val = email.get();
        let password_val = password.get();

        spawn_local(async move {
            match api::register(&username_val, &email_val, &password_val).await {
                Ok(resp) => {
                    set_success_email.set(Some(resp.email));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    });

    let nav_google = use_navigate();
    let on_google = StoredValue::new(move |_| {
        set_error.set(None);
        let nav = nav_google.clone();
        oauth::open_oauth_popup(
            "google",
            move |resp| {
                api::store_oauth_auth(&resp.token, &resp.user.username);
                auth.set_logged_in.set(true);
                auth.set_username.set(Some(resp.user.username));
                nav("/", Default::default());
            },
            move |err| set_error.set(Some(err)),
        );
    });

    let nav_github = use_navigate();
    let on_github = StoredValue::new(move |_| {
        set_error.set(None);
        let nav = nav_github.clone();
        oauth::open_oauth_popup(
            "github",
            move |resp| {
                api::store_oauth_auth(&resp.token, &resp.user.username);
                auth.set_logged_in.set(true);
                auth.set_username.set(Some(resp.user.username));
                nav("/", Default::default());
            },
            move |err| set_error.set(Some(err)),
        );
    });

    let resend_click = move |_| {
        set_resending.set(true);
        set_error.set(None);

        let email_val = success_email.get().unwrap_or_default();

        spawn_local(async move {
            match api::resend_verification(&email_val).await {
                Ok(_) => {
                    set_resending.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_resending.set(false);
                }
            }
        });
    };

    view! {
        <div class="max-w-sm mx-auto py-16">
            <Show
                when=move || success_email.get().is_some()
                fallback=move || view! {
                    <>
                        <h1 class="text-xl font-medium text-gray-900 mb-6">"Sign Up"</h1>

                        {move || error.get().map(|e| view! {
                            <div class="text-red-600 text-sm mb-4">{e}</div>
                        })}

                        <div class="space-y-3 mb-6">
                            <button
                                on:click=move |ev| on_google.with_value(|f| f(ev))
                                class="w-full px-4 py-2 border border-gray-300 rounded hover:bg-gray-50 flex items-center justify-center gap-2 text-sm"
                            >
                                "Continue with Google"
                            </button>
                            <button
                                on:click=move |ev| on_github.with_value(|f| f(ev))
                                class="w-full px-4 py-2 border border-gray-300 rounded hover:bg-gray-50 flex items-center justify-center gap-2 text-sm"
                            >
                                "Continue with GitHub"
                            </button>
                        </div>

                        <div class="flex items-center gap-3 mb-6">
                            <div class="flex-1 border-t border-gray-200"></div>
                            <span class="text-xs text-gray-400">"or"</span>
                            <div class="flex-1 border-t border-gray-200"></div>
                        </div>

                        <form on:submit=move |ev| submit.with_value(|f| f(ev)) class="space-y-4">
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

                            <div>
                                <label class="block text-sm text-gray-600 mb-1">"Confirm Password"</label>
                                <input
                                    type="password"
                                    class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                                    prop:value=confirm_password
                                    on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                                    minlength="8"
                                    required
                                />
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
                    </>
                }
            >
                <div class="text-center">
                    <h1 class="text-xl font-medium text-gray-900 mb-3">"Check Your Email"</h1>
                    <p class="text-gray-600 mb-6">
                        "We sent a verification link to "
                        <span class="font-medium">{move || success_email.get().unwrap_or_default()}</span>
                    </p>
                    <p class="text-sm text-gray-500 mb-6">
                        "Click the link in the email to verify your account and log in."
                    </p>

                    {move || error.get().map(|e| view! {
                        <div class="text-red-600 text-sm mb-4">{e}</div>
                    })}

                    <button
                        on:click=resend_click
                        class="text-sm text-gray-600 hover:text-gray-900 disabled:opacity-50"
                        disabled=resending
                    >
                        {move || if resending.get() { "Sending..." } else { "Resend verification email" }}
                    </button>

                    <p class="text-sm text-gray-500 mt-6">
                        <A href="/login" attr:class="text-gray-900 hover:underline">"Back to Login"</A>
                    </p>
                </div>
            </Show>
        </div>
    }
}
