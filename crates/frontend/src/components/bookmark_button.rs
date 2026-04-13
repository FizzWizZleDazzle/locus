//! Reusable bookmark toggle button

use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api;

#[component]
pub fn BookmarkButton(
    problem_id: uuid::Uuid,
    #[prop(optional, into)]
    initial_bookmarked: Option<bool>,
) -> impl IntoView {
    let (bookmarked, set_bookmarked) = signal(initial_bookmarked.unwrap_or(false));
    let (loading, set_loading) = signal(false);

    let toggle = move |_| {
        if loading.get_untracked() {
            return;
        }
        set_loading.set(true);
        let is_bookmarked = bookmarked.get_untracked();
        spawn_local(async move {
            let result = if is_bookmarked {
                api::remove_bookmark(problem_id).await
            } else {
                api::add_bookmark(problem_id).await
            };
            if result.is_ok() {
                set_bookmarked.set(!is_bookmarked);
            }
            set_loading.set(false);
        });
    };

    view! {
        <button
            class=move || format!(
                "p-1 rounded transition-colors {}",
                if bookmarked.get() { "text-yellow-500 hover:text-yellow-600" } else { "text-gray-400 hover:text-gray-600" }
            )
            title=move || if bookmarked.get() { "Remove bookmark" } else { "Bookmark this problem" }
            on:click=toggle
            disabled=move || loading.get()
        >
            <svg class="w-5 h-5" viewBox="0 0 24 24" fill=move || if bookmarked.get() { "currentColor" } else { "none" } stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z"></path>
            </svg>
        </button>
    }
}
