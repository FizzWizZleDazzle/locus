use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};
use crate::{api, AuthContext};

#[component]
pub fn Sidebar() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = use_navigate();
    let (is_expanded, set_is_expanded) = signal(false);

    let on_mouse_enter = move |_| set_is_expanded.set(true);
    let on_mouse_leave = move |_| set_is_expanded.set(false);

    let logout = move |_| {
        api::logout();
        auth.set_logged_in.set(false);
        auth.set_username.set(None);
        navigate("/", Default::default());
    };

    view! {
        <div
            class=move || format!(
                "fixed top-0 left-0 h-full bg-gray-900 text-white shadow-xl z-40 transition-all duration-300 ease-in-out {}",
                if is_expanded.get() { "w-64" } else { "w-16" }
            )
            on:mouseenter=on_mouse_enter
            on:mouseleave=on_mouse_leave
        >
            <div class="flex flex-col h-full py-6">
                // Logo/Title area
                <div class="mb-8 px-4">
                    <div class="flex items-center gap-3">
                        // Logo icon
                        <svg class="w-8 h-8 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                        </svg>
                        // Title (only shown when expanded)
                        <div class=move || format!(
                            "overflow-hidden transition-all duration-300 {}",
                            if is_expanded.get() { "opacity-100 w-auto" } else { "opacity-0 w-0" }
                        )>
                            <h1 class="text-xl font-bold whitespace-nowrap">"Locus"</h1>
                            {move || auth.username.get().map(|name| view! {
                                <p class="text-xs text-gray-400 whitespace-nowrap">{name}</p>
                            })}
                        </div>
                    </div>
                </div>

                // Navigation
                <nav class="flex-1 flex flex-col px-2">
                    <div class="space-y-2">
                        // Practice
                        <A href="/practice" attr:class="flex items-center gap-3 px-3 py-3 rounded-lg hover:bg-gray-800 transition-colors">
                            <svg class="w-6 h-6 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                            </svg>
                            <span class=move || format!(
                                "whitespace-nowrap transition-all duration-300 {}",
                                if is_expanded.get() { "opacity-100" } else { "opacity-0 w-0" }
                            )>"Practice"</span>
                        </A>

                        // Ranked
                        <A href="/ranked" attr:class="flex items-center gap-3 px-3 py-3 rounded-lg hover:bg-gray-800 transition-colors">
                            <svg class="w-6 h-6 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"></path>
                            </svg>
                            <span class=move || format!(
                                "whitespace-nowrap transition-all duration-300 {}",
                                if is_expanded.get() { "opacity-100" } else { "opacity-0 w-0" }
                            )>"Ranked"</span>
                        </A>
                    </div>

                    // Settings and Logout at bottom
                    <div class="mt-auto space-y-2">
                        // Settings
                        <A href="/settings" attr:class="flex items-center gap-3 px-3 py-3 rounded-lg hover:bg-gray-800 transition-colors">
                            <svg class="w-6 h-6 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                            </svg>
                            <span class=move || format!(
                                "whitespace-nowrap transition-all duration-300 {}",
                                if is_expanded.get() { "opacity-100" } else { "opacity-0 w-0" }
                            )>"Settings"</span>
                        </A>

                        // Logout
                        <button
                            on:click=logout
                            class="w-full flex items-center gap-3 px-3 py-3 rounded-lg hover:bg-gray-800 transition-colors text-red-400"
                        >
                            <svg class="w-6 h-6 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path>
                            </svg>
                            <span class=move || format!(
                                "whitespace-nowrap transition-all duration-300 {}",
                                if is_expanded.get() { "opacity-100" } else { "opacity-0 w-0" }
                            )>"Logout"</span>
                        </button>
                    </div>
                </nav>
            </div>
        </div>
    }
}
