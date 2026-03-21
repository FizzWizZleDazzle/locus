mod api;
mod components;
mod pages;

use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <div class="dashboard"><h1>"404"</h1></div> }>
                <Route path=path!("/status") view=pages::dashboard::DashboardPage />
                <Route path=path!("/") view=|| view! { <leptos_router::components::Redirect path="/status" /> } />
            </Routes>
        </Router>
    }
}
