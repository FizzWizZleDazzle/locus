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
        <nav class="border-b bg-white">
            <div class="container mx-auto px-4">
                <div class="flex items-center justify-between h-16">
                    <A href="/" attr:class="text-lg font-medium">
                        "Locus"
                    </A>

                    <div class="flex items-center gap-6">
                        <A href="/practice" attr:class="text-sm hover:text-gray-600">
                            "Practice"
                        </A>
                        <A href="/ranked" attr:class="text-sm hover:text-gray-600">
                            "Ranked"
                        </A>
                        <A href="/leaderboard" attr:class="text-sm hover:text-gray-600">
                            "Leaderboard"
                        </A>

                        {move || {
                            if auth.is_logged_in.get() {
                                view! {
                                    <>
                                        <span class="text-sm text-gray-600">
                                            {move || auth.username.get().unwrap_or_default()}
                                        </span>
                                        <A href="/settings" attr:class="text-sm hover:text-gray-600">
                                            "Settings"
                                        </A>
                                        <button
                                            on:click=logout
                                            class="text-sm hover:text-gray-600"
                                        >
                                            "Logout"
                                        </button>
                                    </>
                                }.into_any()
                            } else {
                                view! {
                                    <>
                                        <A href="/login" attr:class="text-sm hover:text-gray-600">
                                            "Login"
                                        </A>
                                        <A href="/register" attr:class="px-4 py-2 bg-black text-white text-sm hover:bg-gray-800">
                                            "Sign Up"
                                        </A>
                                    </>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>
            </div>
        </nav>
    }
}
