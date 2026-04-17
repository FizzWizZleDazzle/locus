use crate::AuthContext;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Sidebar() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let (is_expanded, set_is_expanded) = signal(false);

    let on_mouse_enter = move |_| set_is_expanded.set(true);
    let on_mouse_leave = move |_| set_is_expanded.set(false);

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
                        // Daily Puzzle
                        <A href="/daily" attr:class="flex items-center gap-3 px-3 py-3 rounded-lg hover:bg-gray-800 transition-colors">
                            <svg class="w-6 h-6 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"></path>
                            </svg>
                            <span class=move || format!(
                                "whitespace-nowrap transition-all duration-300 {}",
                                if is_expanded.get() { "opacity-100" } else { "opacity-0 w-0" }
                            )>"Daily"</span>
                        </A>

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

                        // Physics
                        <A href="/physics" attr:class="flex items-center gap-3 px-3 py-3 rounded-lg hover:bg-gray-800 transition-colors">
                            <svg class="w-6 h-6 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m-7-9H4m16 0h1M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707m12.728 0l-.707-.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path>
                            </svg>
                            <span class=move || format!(
                                "whitespace-nowrap transition-all duration-300 {}",
                                if is_expanded.get() { "opacity-100" } else { "opacity-0 w-0" }
                            )>"Physics"</span>
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

                        // Profile
                        {move || auth.username.get().map(|name| {
                            let href = format!("/profile/{}", name);
                            view! {
                                <A href=href attr:class="flex items-center gap-3 px-3 py-3 rounded-lg hover:bg-gray-800 transition-colors">
                                    <svg class="w-6 h-6 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                                    </svg>
                                    <span class=move || format!(
                                        "whitespace-nowrap transition-all duration-300 {}",
                                        if is_expanded.get() { "opacity-100" } else { "opacity-0 w-0" }
                                    )>"Profile"</span>
                                </A>
                            }
                        })}
                    </div>

                    // Settings at bottom
                    <div class="mt-auto">
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
                    </div>
                </nav>
            </div>
        </div>
    }
}
