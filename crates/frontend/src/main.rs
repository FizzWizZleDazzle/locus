//! Locus Frontend - Competitive Math Platform

mod api;
mod grader;
mod symengine;
mod pages;
mod components;

use leptos::prelude::*;
use leptos_router::{
    components::{Router, Route, Routes},
    path,
};

use pages::{Home, Practice, Ranked, Leaderboard, Login, Register};
use components::Navbar;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // Global auth state
    let (is_logged_in, set_logged_in) = signal(api::is_logged_in());
    let (username, set_username) = signal(api::get_stored_username());

    // Provide auth context to all components
    provide_context(AuthContext {
        is_logged_in,
        set_logged_in,
        username,
        set_username,
    });

    view! {
        <Router>
            <div class="min-h-screen flex flex-col">
                <Navbar />
                <main class="flex-1 container mx-auto px-4 py-8">
                    <Routes fallback=|| view! { <p>"Page not found"</p> }>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/practice") view=Practice />
                        <Route path=path!("/ranked") view=Ranked />
                        <Route path=path!("/leaderboard") view=Leaderboard />
                        <Route path=path!("/login") view=Login />
                        <Route path=path!("/register") view=Register />
                    </Routes>
                </main>
                <footer class="py-8 text-center text-xs text-gray-400">
                    "Locus"
                </footer>
            </div>
        </Router>
    }
}

/// Global authentication context
#[derive(Clone, Copy)]
pub struct AuthContext {
    pub is_logged_in: ReadSignal<bool>,
    pub set_logged_in: WriteSignal<bool>,
    pub username: ReadSignal<Option<String>>,
    pub set_username: WriteSignal<Option<String>>,
}
