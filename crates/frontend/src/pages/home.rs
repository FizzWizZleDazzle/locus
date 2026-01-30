//! Home page

use leptos::prelude::*;
use leptos_router::components::A;

use crate::AuthContext;

#[component]
pub fn Home() -> impl IntoView {
    let auth = expect_context::<AuthContext>();

    view! {
        <div class="max-w-xl mx-auto py-16">
            <h1 class="text-3xl font-semibold text-gray-900 mb-4">
                "Locus"
            </h1>
            <p class="text-gray-600 mb-8">
                "Competitive math platform with ELO-based ranking."
            </p>

            <div class="flex space-x-3">
                <A
                    href="/practice"
                    attr:class="px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800"
                >
                    "Practice"
                </A>
                {move || {
                    if auth.is_logged_in.get() {
                        view! {
                            <A
                                href="/ranked"
                                attr:class="px-4 py-2 border border-gray-300 rounded hover:border-gray-400"
                            >
                                "Ranked"
                            </A>
                        }.into_any()
                    } else {
                        view! {
                            <A
                                href="/register"
                                attr:class="px-4 py-2 border border-gray-300 rounded hover:border-gray-400"
                            >
                                "Sign Up"
                            </A>
                        }.into_any()
                    }
                }}
            </div>

            <div class="mt-16 grid grid-cols-3 gap-8 text-sm">
                <div>
                    <h3 class="font-medium text-gray-900 mb-1">"Practice"</h3>
                    <p class="text-gray-500">"Instant feedback, no account needed"</p>
                </div>
                <div>
                    <h3 class="font-medium text-gray-900 mb-1">"Ranked"</h3>
                    <p class="text-gray-500">"Compete for ELO rating"</p>
                </div>
                <div>
                    <h3 class="font-medium text-gray-900 mb-1">"Topics"</h3>
                    <p class="text-gray-500">"Algebra, Calculus, Linear Algebra"</p>
                </div>
            </div>
        </div>
    }
}
