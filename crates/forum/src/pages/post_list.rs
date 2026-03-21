use leptos::prelude::*;

use crate::api;
use crate::components::post_card::PostCard;

#[component]
pub fn PostListPage() -> impl IntoView {
    let category = RwSignal::new(Option::<String>::None);
    let sort = RwSignal::new("newest".to_string());
    let page = RwSignal::new(1i64);
    let search_input = RwSignal::new(String::new());
    let search = RwSignal::new(String::new());
    let posts = RwSignal::new(Option::<Result<Vec<api::ForumPost>, String>>::None);
    let has_more = RwSignal::new(false);

    // Fetch posts whenever filters change
    Effect::new(move || {
        let cat = category.get();
        let srt = sort.get();
        let pg = page.get();
        let q = search.get();
        leptos::task::spawn_local(async move {
            let result = api::list_posts(
                cat.as_deref(),
                None,
                Some(&srt),
                pg,
                if q.is_empty() { None } else { Some(&q) },
            ).await;
            match result {
                Ok(resp) => {
                    has_more.set(resp.has_more);
                    posts.set(Some(Ok(resp.posts)));
                }
                Err(e) => {
                    has_more.set(false);
                    posts.set(Some(Err(e)));
                }
            }
        });
    });

    let set_category = move |cat: Option<&str>| {
        category.set(cat.map(String::from));
        page.set(1);
    };

    let on_search = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        search.set(search_input.get_untracked());
        page.set(1);
    };

    let tab_class = move |active: bool| {
        if active {
            "px-3 py-1.5 text-sm font-medium rounded bg-gray-100 text-gray-900 border-none cursor-pointer"
        } else {
            "px-3 py-1.5 text-sm font-medium rounded text-gray-500 hover:text-gray-700 hover:bg-gray-50 border-none cursor-pointer bg-transparent"
        }
    };

    view! {
        <div>
            <div class="mb-6">
                <h1 class="text-xl font-medium text-gray-900">"Community Forum"</h1>
                <p class="text-sm text-gray-500 mt-1">"Feature requests and bug reports"</p>
            </div>

            <form class="mb-4" on:submit=on_search>
                <input
                    type="search"
                    placeholder="Search posts..."
                    class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                    prop:value=move || search_input.get()
                    on:input=move |ev| search_input.set(event_target_value(&ev))
                />
            </form>

            <div class="flex justify-between items-center mb-4 gap-4">
                <div class="flex gap-1">
                    <button
                        class=move || tab_class(category.get().is_none())
                        on:click=move |_| set_category(None)
                    >"All"</button>
                    <button
                        class=move || tab_class(category.get().as_deref() == Some("feature_request"))
                        on:click=move |_| set_category(Some("feature_request"))
                    >"Features"</button>
                    <button
                        class=move || tab_class(category.get().as_deref() == Some("bug_report"))
                        on:click=move |_| set_category(Some("bug_report"))
                    >"Bugs"</button>
                </div>
                <div>
                    <select
                        class="px-3 py-1.5 text-sm border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
                        prop:value=move || sort.get()
                        on:change=move |ev| { sort.set(event_target_value(&ev)); page.set(1); }
                    >
                        <option value="newest">"Newest"</option>
                        <option value="top">"Top voted"</option>
                    </select>
                </div>
            </div>

            <div>
                {move || match posts.get() {
                    None => view! { <div class="text-center text-gray-400 py-10">"Loading..."</div> }.into_any(),
                    Some(Err(e)) => view! { <div class="text-red-600 text-sm mb-4">{e}</div> }.into_any(),
                    Some(Ok(list)) => {
                        if list.is_empty() {
                            view! { <div class="text-center text-gray-400 py-10">"No posts yet. Be the first!"</div> }.into_any()
                        } else {
                            let cards: Vec<_> = list.into_iter().map(|p| view! { <PostCard post=p /> }).collect();
                            view! { <div>{cards}</div> }.into_any()
                        }
                    }
                }}
            </div>

            <div class="flex justify-center items-center gap-4 mt-6">
                <button
                    class="text-sm text-gray-500 hover:text-gray-700 disabled:opacity-50"
                    disabled=move || page.get() <= 1
                    on:click=move |_| page.update(|p| *p -= 1)
                >"Previous"</button>
                <span class="text-sm text-gray-400">"Page " {move || page.get()}</span>
                <button
                    class="text-sm text-gray-500 hover:text-gray-700 disabled:opacity-50"
                    disabled=move || !has_more.get()
                    on:click=move |_| page.update(|p| *p += 1)
                >"Next"</button>
            </div>
        </div>
    }
}
