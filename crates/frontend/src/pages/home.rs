//! Home page

use leptos::prelude::*;
use leptos_router::components::A;

use crate::AuthContext;

#[component]
pub fn Home() -> impl IntoView {
    let auth = expect_context::<AuthContext>();

    view! {
        <div class="max-w-2xl mx-auto px-4 py-20">
            <h1 class="text-4xl font-semibold mb-4">
                "Locus"
            </h1>
            <p class="text-gray-600 mb-8">
                "Competitive math platform with ELO-based ranking."
            </p>

            <div class="flex gap-3 mb-16">
                <A href="/practice" attr:class="px-6 py-3 bg-black text-white hover:bg-gray-800">
                    "Practice"
                </A>
                {move || {
                    if auth.is_logged_in.get() {
                        view! {
                            <A href="/ranked" attr:class="px-6 py-3 border hover:bg-gray-50">
                                "Ranked"
                            </A>
                        }.into_any()
                    } else {
                        view! {
                            <A href="/register" attr:class="px-6 py-3 border hover:bg-gray-50">
                                "Sign Up"
                            </A>
                        }.into_any()
                    }
                }}
            </div>

            <div class="grid grid-cols-3 gap-8">
                <div>
                    <h3 class="font-medium mb-1">"Practice"</h3>
                    <p class="text-sm text-gray-500">"Instant feedback, no account needed"</p>
                </div>
                <div>
                    <h3 class="font-medium mb-1">"Ranked"</h3>
                    <p class="text-sm text-gray-500">"Compete for ELO rating"</p>
                </div>
                <div>
                    <h3 class="font-medium mb-1">"Topics"</h3>
                    <p class="text-sm text-gray-500">"Algebra, Calculus, Linear Algebra"</p>
                </div>
            </div>
        </div>
    }
}
