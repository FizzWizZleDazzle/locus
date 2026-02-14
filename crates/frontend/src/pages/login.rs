//! Login page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::{components::A, hooks::use_navigate};

use crate::{api, oauth, AuthContext};

#[component]
pub fn Login() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = use_navigate();

    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (loading, set_loading) = signal(false);
    let (show_resend, set_show_resend) = signal(false);
    let (resending, set_resending) = signal(false);

    let submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(None);

        let email_val = email.get();
        let password_val = password.get();
        let nav = navigate.clone();

        spawn_local(async move {
            match api::login(&email_val, &password_val).await {
                Ok(resp) => {
                    auth.set_logged_in.set(true);
                    auth.set_username.set(Some(resp.user.username));
                    nav("/", Default::default());
                }
                Err(e) => {
                    let msg = e.message.clone();
                    set_error.set(Some(msg.clone()));
                    // Show resend button if error is about email verification
                    set_show_resend.set(msg.contains("verify your email") || msg.contains("verification"));
                    set_loading.set(false);
                }
            }
        });
    };

    let nav_google = use_navigate();
    let on_google = move |_| {
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
    };

    let nav_github = use_navigate();
    let on_github = move |_| {
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
    };

    let resend_verification = move |_| {
        set_resending.set(true);
        set_error.set(None);

        let email_val = email.get();

        spawn_local(async move {
            match api::resend_verification(&email_val).await {
                Ok(_) => {
                    set_error.set(Some("Verification email sent! Check your inbox.".to_string()));
                    set_show_resend.set(false);
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
            <h1 class="text-xl font-medium text-gray-900 mb-6">"Login"</h1>

            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            {move || show_resend.get().then(|| view! {
                <div class="mb-4">
                    <button
                        on:click=resend_verification
                        class="text-sm text-blue-600 hover:text-blue-800 disabled:opacity-50"
                        disabled=resending
                    >
                        {move || if resending.get() { "Sending..." } else { "Resend verification email" }}
                    </button>
                </div>
            })}

            <div class="space-y-3 mb-6">
                <button
                    on:click=on_google
                    class="w-full px-4 py-2 border border-gray-300 rounded hover:bg-gray-50 flex items-center justify-center gap-2 text-sm"
                >
                    "Continue with Google"
                </button>
                <button
                    on:click=on_github
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

            <form on:submit=submit class="space-y-4">
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
                    <div class="flex justify-between items-center mb-1">
                        <label class="block text-sm text-gray-600">"Password"</label>
                        <A href="/forgot-password" attr:class="text-xs text-gray-500 hover:text-gray-700">"Forgot?"</A>
                    </div>
                    <input
                        type="password"
                        class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                        prop:value=password
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                        required
                    />
                </div>

                <button
                    type="submit"
                    class="w-full px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 disabled:opacity-50"
                    disabled=loading
                >
                    {move || if loading.get() { "Loading..." } else { "Login" }}
                </button>
            </form>

            <p class="text-sm text-gray-500 mt-4">
                "No account? "
                <A href="/register" attr:class="text-gray-900 hover:underline">"Sign up"</A>
            </p>
        </div>
    }
}
