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
    let user = RwSignal::new(Option::<api::UserInfo>::None);
    provide_context(user);

    // Check if already logged in
    leptos::task::spawn_local(async move {
        if let Ok(info) = api::get_me().await {
            user.set(Some(info));
        }
    });

    view! {
        <Router>
            <components::nav::Nav />
            <main class="max-w-2xl mx-auto px-4 py-8">
                <Routes fallback=|| view! { <div><h1 class="text-xl font-medium text-gray-900">"404 - Not Found"</h1></div> }>
                    <Route path=path!("/forum") view=pages::post_list::PostListPage />
                    <Route path=path!("/forum/post/:id") view=pages::post_detail::PostDetailPage />
                    <Route path=path!("/forum/new") view=pages::new_post::NewPostPage />
                </Routes>
            </main>
        </Router>
    }
}
