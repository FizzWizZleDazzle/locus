//! Navigation bar component

use leptos::prelude::*;
use leptos_router::components::A;

use crate::AuthContext;

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = expect_context::<AuthContext>();

    // Only show navbar for logged-out users
    view! {
        {move || (!auth.is_logged_in.get()).then(|| view! {
            <nav class="border-b bg-white">
                <div class="container mx-auto px-4">
                    <div class="flex items-center justify-between h-16">
                        <A href="/" attr:class="text-lg font-medium">"Locus"</A>
                        <div class="flex items-center gap-4">
                            <A href="/login" attr:class="text-sm hover:text-gray-600">"Login"</A>
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
