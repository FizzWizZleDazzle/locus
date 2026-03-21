use leptos::prelude::*;
use leptos_router::components::A;

use crate::api;

const MAIN_APP_URL: &str = env!("LOCUS_FRONTEND_URL");

#[component]
pub fn Nav() -> impl IntoView {
    let user = expect_context::<RwSignal<Option<api::UserInfo>>>();

    let on_logout = move |_| {
        leptos::task::spawn_local(async move {
            let _ = api::logout().await;
            user.set(None);
        });
    };

    let login_href = move || {
        let current_url = web_sys::window()
            .and_then(|w| w.location().href().ok())
            .unwrap_or_default();
        let encoded = js_sys::encode_uri_component(&current_url);
        format!("{}/login?redirect={}", MAIN_APP_URL, encoded)
    };

    view! {
        <nav class="border-b border-gray-200 bg-white">
            <div class="max-w-4xl mx-auto px-4 flex items-center justify-between h-14">
                <A href="/forum" attr:class="text-sm font-semibold text-gray-900">"Locus Forum"</A>
                <div class="flex items-center gap-3">
                    {move || {
                        if let Some(u) = user.get() {
                            view! {
                                <span class="text-sm text-gray-500">{u.username}</span>
                                <A href="/forum/new" attr:class="px-3 py-1.5 bg-gray-900 text-white text-sm rounded hover:bg-gray-800">"New Post"</A>
                                <button class="text-sm text-gray-500 hover:text-gray-700" on:click=on_logout>"Logout"</button>
                            }.into_any()
                        } else {
                            let href = login_href();
                            view! {
                                <a href=href class="px-3 py-1.5 bg-gray-900 text-white text-sm rounded hover:bg-gray-800">"Login"</a>
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        </nav>
    }
}
