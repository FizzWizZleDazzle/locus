//! Settings page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::{AuthContext, api, oauth};
use locus_common::{SuccessResponse, UserProfile};

#[component]
pub fn Settings() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = use_navigate();

    // State for user profile
    let (profile, set_profile) = signal(None::<UserProfile>);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(None::<String>);

    // Password change state
    let (old_password, set_old_password) = signal(String::new());
    let (new_password, set_new_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());

    // Username change state
    let (show_username_form, set_show_username_form) = signal(false);
    let (new_username, set_new_username) = signal(String::new());

    // Delete account state
    let (show_delete_modal, set_show_delete_modal) = signal(false);
    let (delete_password, set_delete_password) = signal(String::new());
    let (delete_confirmation, set_delete_confirmation) = signal(String::new());

    // Fetch user profile and redirect if not authenticated
    let nav_check = navigate.clone();
    spawn_local(async move {
        match api::get_me().await {
            Ok(p) => {
                set_new_username.set(p.username.clone());
                set_profile.set(Some(p));
            }
            Err(_) => {
                // Not authenticated, redirect to login
                nav_check("/login", Default::default());
            }
        }
    });

    // Password change handler
    let submit_password = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_error.set(None);
        set_success.set(None);

        let old_pw = old_password.get();
        let new_pw = new_password.get();
        let confirm_pw = confirm_password.get();

        if new_pw != confirm_pw {
            set_error.set(Some("Passwords do not match".into()));
            return;
        }

        set_loading.set(true);

        spawn_local(async move {
            let result = if profile.get().map(|p| p.has_password).unwrap_or(false) {
                api::change_password(&old_pw, &new_pw).await
            } else {
                api::set_password(&new_pw).await.map(|_| SuccessResponse {
                    success: true,
                    message: Some("Password set successfully".to_string()),
                })
            };

            match result {
                Ok(_) => {
                    set_success.set(Some("Password updated successfully".into()));
                    set_old_password.set(String::new());
                    set_new_password.set(String::new());
                    set_confirm_password.set(String::new());
                    // Refresh profile
                    if let Ok(p) = api::get_me().await {
                        set_profile.set(Some(p));
                    }
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                }
            }
            set_loading.set(false);
        });
    };

    // Username change handler
    let submit_username = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_error.set(None);
        set_success.set(None);

        let username = new_username.get();
        set_loading.set(true);

        spawn_local(async move {
            match api::change_username(&username).await {
                Ok(updated_profile) => {
                    set_success.set(Some("Username updated successfully".into()));
                    set_profile.set(Some(updated_profile));
                    set_show_username_form.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                }
            }
            set_loading.set(false);
        });
    };

    // Delete account handler
    let confirm_delete = move |_| {
        set_error.set(None);
        set_success.set(None);

        let has_password = profile.get().map(|p| p.has_password).unwrap_or(false);
        let password = if has_password {
            Some(delete_password.get())
        } else {
            None
        };
        let confirmation = if !has_password {
            Some(delete_confirmation.get())
        } else {
            None
        };

        set_loading.set(true);

        spawn_local(async move {
            match api::delete_account(password.as_deref(), confirmation.as_deref()).await {
                Ok(_) => {
                    // Auth cleared by API client, navigate to home
                    auth.set_logged_in.set(false);
                    auth.set_username.set(None);
                    web_sys::window().unwrap().location().set_href("/").unwrap();
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                    set_show_delete_modal.set(false);
                }
            }
        });
    };

    // Unlink OAuth handler
    let unlink_provider = move |provider: String| {
        set_error.set(None);
        set_success.set(None);

        spawn_local(async move {
            match api::unlink_oauth(&provider).await {
                Ok(updated_profile) => {
                    set_success.set(Some(format!("{} account unlinked", provider)));
                    set_profile.set(Some(updated_profile));
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                }
            }
        });
    };

    // Link OAuth handler
    let link_provider = move |provider: String| {
        let provider_clone = provider.clone();
        oauth::open_oauth_link_popup(
            &provider,
            move |auth_response| {
                set_success.set(Some(format!(
                    "{} account linked successfully",
                    provider_clone
                )));
                set_profile.set(Some(auth_response.user));
            },
            move |error| {
                set_error.set(Some(error));
            },
        );
    };

    view! {
        <div class="max-w-2xl mx-auto py-8 px-4">
            <h1 class="text-2xl font-semibold text-gray-900 mb-8">"Account Settings"</h1>

            {move || error.get().map(|e| view! {
                <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-4">
                    {e}
                </div>
            })}

            {move || success.get().map(|s| view! {
                <div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded mb-4">
                    {s}
                </div>
            })}

            // Account Information Section
            <div class="bg-white border border-gray-200 rounded-lg p-6 mb-6">
                <h2 class="text-lg font-medium text-gray-900 mb-4">"Account Information"</h2>

                {move || profile.get().map(|p| view! {
                    <div class="space-y-3">
                        <div class="flex justify-between items-center">
                            <div>
                                <p class="text-sm text-gray-500">"Username"</p>
                                <p class="text-gray-900">{p.username.clone()}</p>
                            </div>
                            {move || {
                                if !show_username_form.get() {
                                    Some(view! {
                                        <button
                                            on:click=move |_| set_show_username_form.set(true)
                                            class="text-sm text-blue-600 hover:text-blue-700"
                                        >
                                            "Change"
                                        </button>
                                    }.into_any())
                                } else {
                                    None
                                }
                            }}
                        </div>

                        {move || {
                            if show_username_form.get() {
                                Some(view! {
                                    <form on:submit=submit_username class="space-y-3 pt-2">
                                        <div>
                                            <label class="block text-sm text-gray-600 mb-1">"New Username"</label>
                                            <input
                                                type="text"
                                                class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                                                prop:value=new_username
                                                on:input=move |ev| set_new_username.set(event_target_value(&ev))
                                                minlength="3"
                                                maxlength="20"
                                                required
                                            />
                                        </div>
                                        <div class="flex gap-2">
                                            <button
                                                type="submit"
                                                class="px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 disabled:opacity-50"
                                                disabled=loading
                                            >
                                                {move || if loading.get() { "Saving..." } else { "Save" }}
                                            </button>
                                            <button
                                                type="button"
                                                on:click=move |_| {
                                                    set_show_username_form.set(false);
                                                    set_new_username.set(profile.get().map(|p| p.username).unwrap_or_default());
                                                }
                                                class="px-4 py-2 border border-gray-300 rounded hover:bg-gray-50"
                                            >
                                                "Cancel"
                                            </button>
                                        </div>
                                    </form>
                                }.into_any())
                            } else {
                                None
                            }
                        }}

                        <div>
                            <p class="text-sm text-gray-500">"Email"</p>
                            <p class="text-gray-900">
                                {p.email.clone()}
                                {move || {
                                    if p.email_verified {
                                        Some(view! {
                                            <span class="ml-2 text-green-600 text-sm">"✓ Verified"</span>
                                        }.into_any())
                                    } else {
                                        Some(view! {
                                            <span class="ml-2 text-yellow-600 text-sm">"⚠ Not verified"</span>
                                        }.into_any())
                                    }
                                }}
                            </p>
                        </div>

                        <div>
                            <p class="text-sm text-gray-500">"Member since"</p>
                            <p class="text-gray-900">{p.created_at.format("%B %d, %Y").to_string()}</p>
                        </div>
                    </div>
                })}
            </div>

            // Password Management Section
            <div class="bg-white border border-gray-200 rounded-lg p-6 mb-6">
                <h2 class="text-lg font-medium text-gray-900 mb-2">
                    {move || {
                        if profile.get().map(|p| p.has_password).unwrap_or(false) {
                            "Change Password"
                        } else {
                            "Set Password"
                        }
                    }}
                </h2>
                <p class="text-sm text-gray-500 mb-4">
                    {move || {
                        if profile.get().map(|p| p.has_password).unwrap_or(false) {
                            "Update your password to keep your account secure."
                        } else {
                            "Set a password to also sign in with email and password."
                        }
                    }}
                </p>

                <form on:submit=submit_password class="space-y-4">
                    {move || {
                        if profile.get().map(|p| p.has_password).unwrap_or(false) {
                            Some(view! {
                                <div>
                                    <label class="block text-sm text-gray-600 mb-1">"Current Password"</label>
                                    <input
                                        type="password"
                                        class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                                        prop:value=old_password
                                        on:input=move |ev| set_old_password.set(event_target_value(&ev))
                                        required
                                    />
                                </div>
                            }.into_any())
                        } else {
                            None
                        }
                    }}

                    <div>
                        <label class="block text-sm text-gray-600 mb-1">"New Password"</label>
                        <input
                            type="password"
                            class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                            prop:value=new_password
                            on:input=move |ev| set_new_password.set(event_target_value(&ev))
                            minlength="8"
                            required
                        />
                        <p class="text-xs text-gray-400 mt-1">"At least 8 characters"</p>
                    </div>

                    <div>
                        <label class="block text-sm text-gray-600 mb-1">"Confirm New Password"</label>
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
                        class="px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 disabled:opacity-50"
                        disabled=loading
                    >
                        {move || if loading.get() { "Saving..." } else if profile.get().map(|p| p.has_password).unwrap_or(false) { "Change Password" } else { "Set Password" }}
                    </button>
                </form>
            </div>

            // Connected Accounts Section
            <div class="bg-white border border-gray-200 rounded-lg p-6 mb-6">
                <h2 class="text-lg font-medium text-gray-900 mb-2">"Connected Accounts"</h2>
                <p class="text-sm text-gray-500 mb-4">
                    "Link social accounts for easy sign-in. You must have either a password or at least one connected account."
                </p>

                {move || profile.get().map(|p| {
                    let providers = p.oauth_providers.clone();
                    let has_google = providers.contains(&"google".to_string());
                    let has_github = providers.contains(&"github".to_string());

                    view! {
                        <div class="space-y-3">
                            // Google
                            <div class="flex items-center justify-between py-2">
                                <div class="flex items-center gap-3">
                                    <div class="w-10 h-10 bg-white border border-gray-300 rounded flex items-center justify-center">
                                        <svg class="w-5 h-5" viewBox="0 0 24 24">
                                            <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
                                            <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
                                            <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
                                            <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="font-medium text-gray-900">"Google"</p>
                                        <p class="text-xs text-gray-500">
                                            {if has_google { "Connected" } else { "Not connected" }}
                                        </p>
                                    </div>
                                </div>
                                {if has_google {
                                    view! {
                                        <button
                                            on:click=move |_| unlink_provider("google".to_string())
                                            class="text-sm text-red-600 hover:text-red-700"
                                        >
                                            "Unlink"
                                        </button>
                                    }.into_any()
                                } else {
                                    view! {
                                        <button
                                            on:click=move |_| link_provider("google".to_string())
                                            class="text-sm text-blue-600 hover:text-blue-700"
                                        >
                                            "Link Google"
                                        </button>
                                    }.into_any()
                                }}
                            </div>

                            // GitHub
                            <div class="flex items-center justify-between py-2">
                                <div class="flex items-center gap-3">
                                    <div class="w-10 h-10 bg-gray-900 rounded flex items-center justify-center">
                                        <svg class="w-5 h-5" fill="white" viewBox="0 0 24 24">
                                            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="font-medium text-gray-900">"GitHub"</p>
                                        <p class="text-xs text-gray-500">
                                            {if has_github { "Connected" } else { "Not connected" }}
                                        </p>
                                    </div>
                                </div>
                                {if has_github {
                                    view! {
                                        <button
                                            on:click=move |_| unlink_provider("github".to_string())
                                            class="text-sm text-red-600 hover:text-red-700"
                                        >
                                            "Unlink"
                                        </button>
                                    }.into_any()
                                } else {
                                    view! {
                                        <button
                                            on:click=move |_| link_provider("github".to_string())
                                            class="text-sm text-blue-600 hover:text-blue-700"
                                        >
                                            "Link GitHub"
                                        </button>
                                    }.into_any()
                                }}
                            </div>
                        </div>
                    }
                })}
            </div>

            // Danger Zone Section
            <div class="bg-white border-2 border-red-200 rounded-lg p-6">
                <h2 class="text-lg font-medium text-red-900 mb-2">"Danger Zone"</h2>
                <p class="text-sm text-gray-600 mb-4">
                    "Once you delete your account, there is no going back. This action cannot be undone."
                </p>
                <button
                    on:click=move |_| set_show_delete_modal.set(true)
                    class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700"
                >
                    "Delete Account"
                </button>
            </div>

            // Delete Account Modal
            {move || {
                if show_delete_modal.get() {
                    Some(view! {
                        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" on:click=move |_| set_show_delete_modal.set(false)>
                            <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4" on:click=move |ev| ev.stop_propagation()>
                                <h3 class="text-lg font-semibold text-gray-900 mb-4">"Delete Account?"</h3>
                                <p class="text-sm text-gray-600 mb-4">
                                    "This action cannot be undone. All your data, including your problems, scores, and account information will be permanently deleted."
                                </p>

                                {move || {
                                    if profile.get().map(|p| p.has_password).unwrap_or(false) {
                                        Some(view! {
                                            <div class="mb-4">
                                                <label class="block text-sm text-gray-600 mb-1">"Enter your password to confirm"</label>
                                                <input
                                                    type="password"
                                                    class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                                                    prop:value=delete_password
                                                    on:input=move |ev| set_delete_password.set(event_target_value(&ev))
                                                    placeholder="Password"
                                                />
                                            </div>
                                        }.into_any())
                                    } else {
                                        Some(view! {
                                            <div class="mb-4">
                                                <label class="block text-sm text-gray-600 mb-1">"Type your username to confirm"</label>
                                                <input
                                                    type="text"
                                                    class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                                                    prop:value=delete_confirmation
                                                    on:input=move |ev| set_delete_confirmation.set(event_target_value(&ev))
                                                    placeholder="Username"
                                                />
                                            </div>
                                        }.into_any())
                                    }
                                }}

                                <div class="flex gap-3">
                                    <button
                                        on:click=confirm_delete
                                        class="flex-1 px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 disabled:opacity-50"
                                        disabled=loading
                                    >
                                        {move || if loading.get() { "Deleting..." } else { "Delete My Account" }}
                                    </button>
                                    <button
                                        on:click=move |_| {
                                            set_show_delete_modal.set(false);
                                            set_delete_password.set(String::new());
                                            set_delete_confirmation.set(String::new());
                                        }
                                        class="flex-1 px-4 py-2 border border-gray-300 rounded hover:bg-gray-50"
                                    >
                                        "Cancel"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_any())
                } else {
                    None
                }
            }}
        </div>
    }
}
