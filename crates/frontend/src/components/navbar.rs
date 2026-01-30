//! Navigation bar component

use leptos::prelude::*;
use leptos_router::components::A;

use crate::{api, AuthContext};

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = expect_context::<AuthContext>();

    let logout = move |_| {
        api::logout();
        auth.set_logged_in.set(false);
        auth.set_username.set(None);
    };

    view! {
        <nav class="border-b border-gray-200 bg-white">
            <div class="container mx-auto px-4">
                <div class="flex items-center justify-between h-14">
                    <A href="/" attr:class="text-lg font-semibold text-gray-900">
                        "Locus"
                    </A>

                    <div class="flex items-center space-x-6">
                        <A href="/practice" attr:class="text-sm text-gray-600 hover:text-gray-900">
                            "Practice"
                        </A>
                        <A href="/ranked" attr:class="text-sm text-gray-600 hover:text-gray-900">
                            "Ranked"
                        </A>
                        <A href="/leaderboard" attr:class="text-sm text-gray-600 hover:text-gray-900">
                            "Leaderboard"
                        </A>

                        {move || {
                            if auth.is_logged_in.get() {
                                view! {
                                    <div class="flex items-center space-x-4">
                                        <span class="text-sm text-gray-700">
                                            {move || auth.username.get().unwrap_or_default()}
                                        </span>
                                        <button
                                            on:click=logout
                                            class="text-sm text-gray-500 hover:text-gray-900"
                                        >
                                            "Logout"
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="flex items-center space-x-4">
                                        <A href="/login" attr:class="text-sm text-gray-600 hover:text-gray-900">
                                            "Login"
                                        </A>
                                        <A
                                            href="/register"
                                            attr:class="text-sm px-3 py-1.5 bg-gray-900 text-white rounded hover:bg-gray-800"
                                        >
                                            "Sign Up"
                                        </A>
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>
            </div>
        </nav>
    }
}
