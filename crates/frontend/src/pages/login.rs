//! Login page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::{
    components::A,
    hooks::{use_navigate, use_query_map},
};

use crate::{AuthContext, api, components::EmailInput, oauth};

/// Validate that a redirect URL is safe (prevents open redirect attacks).
fn is_safe_redirect(url: &str) -> bool {
    // Relative paths starting with / (but not //)
    if let Some(rest) = url.strip_prefix('/') {
        return !rest.starts_with('/');
    }

    // Parse as absolute URL
    if let Ok(parsed) = web_sys::Url::new(url) {
        let hostname = parsed.hostname();
        // Allow locusmath.org and subdomains
        if hostname == "locusmath.org" || hostname.ends_with(".locusmath.org") {
            return true;
        }
        // Allow localhost/127.0.0.1 for dev
        if hostname == "localhost" || hostname == "127.0.0.1" {
            return true;
        }
    }

    false
}

/// Perform post-login navigation: redirect to safe URL or fall back to home.
fn do_post_login_redirect(redirect: &Option<String>) -> bool {
    if let Some(url) = redirect.as_ref().filter(|u| is_safe_redirect(u)) {
        if url.starts_with("http://") || url.starts_with("https://") {
            if let Some(window) = web_sys::window() {
                let _ = window.location().set_href(url);
                return true;
            }
        }
    }
    false
}

#[component]
pub fn Login() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = use_navigate();
    let query = use_query_map();

    let redirect_url = query.read_untracked().get("redirect").unwrap_or_default();
    let redirect_url = if redirect_url.is_empty() {
        None
    } else {
        Some(redirect_url)
    };

    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (loading, set_loading) = signal(false);
    let (email_valid, set_email_valid) = signal(false);
    let (show_resend, set_show_resend) = signal(false);
    let (resending, set_resending) = signal(false);

    let redirect_for_submit = redirect_url.clone();
    let submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(None);

        let email_val = email.get();
        let password_val = password.get();
        let nav = navigate.clone();
        let redirect = redirect_for_submit.clone();

        spawn_local(async move {
            match api::login(&email_val, &password_val).await {
                Ok(resp) => {
                    auth.set_logged_in.set(true);
                    auth.set_username.set(Some(resp.user.username));
                    if !do_post_login_redirect(&redirect) {
                        // Relative path redirect or no redirect
                        let path = redirect
                            .as_ref()
                            .filter(|u| is_safe_redirect(u) && u.starts_with('/'))
                            .map(|u| u.as_str())
                            .unwrap_or("/");
                        nav(path, Default::default());
                    }
                }
                Err(e) => {
                    let msg = e.message.clone();
                    set_error.set(Some(msg.clone()));
                    set_show_resend
                        .set(msg.contains("verify your email") || msg.contains("verification"));
                    set_loading.set(false);
                }
            }
        });
    };

    let redirect_for_google = redirect_url.clone();
    let nav_google = use_navigate();
    let on_google = move |_| {
        set_error.set(None);
        let nav = nav_google.clone();
        let redirect = redirect_for_google.clone();
        oauth::open_oauth_login_popup(
            "google",
            move |resp| {
                api::store_username(&resp.user.username);
                auth.set_logged_in.set(true);
                auth.set_username.set(Some(resp.user.username));
                if !do_post_login_redirect(&redirect) {
                    let path = redirect
                        .as_ref()
                        .filter(|u| is_safe_redirect(u) && u.starts_with('/'))
                        .map(|u| u.as_str())
                        .unwrap_or("/");
                    nav(path, Default::default());
                }
            },
            move |err| set_error.set(Some(err)),
        );
    };

    let redirect_for_github = redirect_url.clone();
    let nav_github = use_navigate();
    let on_github = move |_| {
        set_error.set(None);
        let nav = nav_github.clone();
        let redirect = redirect_for_github.clone();
        oauth::open_oauth_login_popup(
            "github",
            move |resp| {
                api::store_username(&resp.user.username);
                auth.set_logged_in.set(true);
                auth.set_username.set(Some(resp.user.username));
                if !do_post_login_redirect(&redirect) {
                    let path = redirect
                        .as_ref()
                        .filter(|u| is_safe_redirect(u) && u.starts_with('/'))
                        .map(|u| u.as_str())
                        .unwrap_or("/");
                    nav(path, Default::default());
                }
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
                    set_error.set(Some(
                        "Verification email sent! Check your inbox.".to_string(),
                    ));
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
                <EmailInput value=email set_value=set_email valid=set_email_valid />

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
                    disabled=move || loading.get() || !email_valid.get()
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
