//! Email verification page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::{
    components::A,
    hooks::{use_navigate, use_query_map},
};

use crate::api;

#[component]
pub fn VerifyEmail() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    let (loading, set_loading) = signal(true);
    let (success, set_success) = signal(false);
    let (error, set_error) = signal(None::<String>);

    // Extract token from URL on mount
    Effect::new(move || {
        let token = query.read().get("token").unwrap_or_default();

        if token.is_empty() {
            set_error.set(Some("No verification token provided".to_string()));
            set_loading.set(false);
            return;
        }

        let token_clone = token.clone();
        spawn_local(async move {
            match api::verify_email(&token_clone).await {
                Ok(_) => {
                    set_success.set(true);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    });

    view! {
        <div class="max-w-md mx-auto py-16 text-center">
            {move || {
                let nav = navigate.clone();
                if loading.get() {
                    view! {
                        <div>
                            <h1 class="text-xl font-medium text-gray-900 mb-3">"Verifying your email..."</h1>
                            <p class="text-gray-600">"Please wait a moment."</p>
                        </div>
                    }.into_any()
                } else if success.get() {
                    view! {
                        <div>
                            <h1 class="text-xl font-medium text-gray-900 mb-3">"Email Verified!"</h1>
                            <p class="text-gray-600 mb-6">"Your email has been successfully verified. You can now log in to your account."</p>
                            <button
                                on:click=move |_| nav("/login", Default::default())
                                class="px-6 py-2 bg-gray-900 text-white rounded hover:bg-gray-800"
                            >
                                "Go to Login"
                            </button>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div>
                            <h1 class="text-xl font-medium text-gray-900 mb-3">"Verification Failed"</h1>
                            <p class="text-red-600 mb-6">{error.get().unwrap_or_default()}</p>
                            <div class="space-y-3">
                                <A
                                    href="/register"
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
                }
            }}
        </div>
    }
}
