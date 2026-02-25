//! Navigation bar component

use leptos::prelude::*;
use leptos_router::components::A;

use crate::{AuthContext, ThemeContext};

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let theme = expect_context::<ThemeContext>();

    // Only show navbar for logged-out users
    view! {
        {move || (!auth.is_logged_in.get()).then(|| view! {
            <nav class="border-b bg-white">
                <div class="container mx-auto px-4">
                    <div class="flex items-center justify-between h-16">
                        <A href="/" attr:class="text-lg font-medium">"Locus"</A>
                        <div class="flex items-center gap-4">
                            <A href="/login" attr:class="text-sm hover:text-gray-600">"Login"</A>
                            /* <button
                                on:click=move |_| theme.toggle_theme.run(())
                                class="p-2 rounded hover:bg-gray-100 transition-colors"
                                title=move || if theme.is_dark.get() { "Switch to light mode" } else { "Switch to dark mode" }
                            >
                                {move || if theme.is_dark.get() {
                                    view! {
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path>
                                        </svg>
                                    }
                                } else {
                                    view! {
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path>
                                        </svg>
                                    }
                                }} 
                            </button> */
                            <A href="/register" attr:class="px-4 py-2 bg-black text-white text-sm hover:bg-gray-800">
                                "Sign Up"
                            </A>
                        </div>
                    </div>
                </div>
            </nav>
        })}
    }
}
