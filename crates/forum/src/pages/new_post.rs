use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::api;

#[component]
pub fn NewPostPage() -> impl IntoView {
    let user = expect_context::<RwSignal<Option<api::UserInfo>>>();

    let category = RwSignal::new("feature_request".to_string());
    let title = RwSignal::new(String::new());
    let body = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<String>::None);
    let loading = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        loading.set(true);
        error.set(None);

        leptos::task::spawn_local(async move {
            match api::create_post(
                &category.get_untracked(),
                &title.get_untracked(),
                &body.get_untracked(),
            ).await {
                Ok(post) => {
                    let nav = use_navigate();
                    nav(&format!("/forum/post/{}", post.id), Default::default());
                }
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                }
            }
        });
    };

    view! {
        <div class="max-w-sm mx-auto py-16">
            <Show
                when=move || user.get().is_some()
                fallback=|| view! { <div class="text-red-600 text-sm">"Please login first."</div> }
            >
                <h1 class="text-xl font-medium text-gray-900 mb-6">"New Post"</h1>

                {move || error.get().map(|e| view! {
                    <div class="text-red-600 text-sm mb-4">{e}</div>
                })}

                <form on:submit=on_submit class="space-y-4">
                    <div>
                        <label class="block text-sm text-gray-600 mb-1">"Category"</label>
                        <select
                            class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                            prop:value=move || category.get()
                            on:change=move |ev| category.set(event_target_value(&ev))
                        >
                            <option value="feature_request">"Feature Request"</option>
                            <option value="bug_report">"Bug Report"</option>
                        </select>
                    </div>
                    <div>
                        <label class="block text-sm text-gray-600 mb-1">"Title"</label>
                        <input
                            type="text"
                            placeholder="Brief summary"
                            maxlength=200
                            required=true
                            class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                            prop:value=move || title.get()
                            on:input=move |ev| title.set(event_target_value(&ev))
                        />
                    </div>
                    <div>
                        <label class="block text-sm text-gray-600 mb-1">"Description"</label>
                        <textarea
                            placeholder="Describe in detail..."
                            rows=8
                            required=true
                            class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                            prop:value=move || body.get()
                            on:input=move |ev| body.set(event_target_value(&ev))
                        />
                    </div>

                    <button
                        type="submit"
                        class="w-full px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 disabled:opacity-50"
                        disabled=move || loading.get()
                    >
                        {move || if loading.get() { "Submitting..." } else { "Submit" }}
                    </button>
                </form>
            </Show>
        </div>
    }
}
