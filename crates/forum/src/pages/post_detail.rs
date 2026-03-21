use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::api;
use crate::components::comment::Comment;

#[component]
pub fn PostDetailPage() -> impl IntoView {
    let params = use_params_map();
    let user = expect_context::<RwSignal<Option<api::UserInfo>>>();

    let data = RwSignal::new(Option::<Result<api::PostDetailResponse, String>>::None);
    let comment_body = RwSignal::new(String::new());
    let comment_error = RwSignal::new(Option::<String>::None);
    let voted = RwSignal::new(false);
    let upvotes = RwSignal::new(0i32);

    // Load post data
    Effect::new(move || {
        let id = params.get().get("id")
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);
        leptos::task::spawn_local(async move {
            let result = api::get_post(id).await;
            if let Ok(ref d) = result {
                voted.set(d.user_voted);
                upvotes.set(d.post.upvotes);
            }
            data.set(Some(result));
        });
    });

    let post_id = move || {
        params.get().get("id")
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0)
    };

    let on_vote = move |_| {
        let id = post_id();
        leptos::task::spawn_local(async move {
            if let Ok(resp) = api::toggle_vote(id).await {
                voted.set(resp.voted);
                upvotes.set(resp.upvotes);
            }
        });
    };

    let on_comment = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let id = post_id();
        let body = comment_body.get_untracked();
        comment_error.set(None);

        leptos::task::spawn_local(async move {
            match api::add_comment(id, &body).await {
                Ok(_) => {
                    comment_body.set(String::new());
                    // Reload
                    if let Ok(result) = api::get_post(id).await {
                        voted.set(result.user_voted);
                        upvotes.set(result.post.upvotes);
                        data.set(Some(Ok(result)));
                    }
                }
                Err(e) => comment_error.set(Some(e)),
            }
        });
    };

    view! {
        <div>
            {move || match data.get() {
                None => view! { <div class="text-center text-gray-400 py-10">"Loading..."</div> }.into_any(),
                Some(Err(e)) => view! { <div class="text-red-600 text-sm mb-4">{e}</div> }.into_any(),
                Some(Ok(d)) => {
                    let post = d.post;
                    let comments = d.comments;
                    let category_class = if post.category == "bug_report" { "tag tag-bug" } else { "tag tag-feature" };
                    let category_label = if post.category == "bug_report" { "Bug Report" } else { "Feature Request" };
                    let status_class = format!("tag tag-status-{}", post.status);
                    let status_label = post.status.replace('_', " ");
                    let is_locked = post.locked;
                    let title = post.title.clone();
                    let username = post.username.clone();
                    let body = post.body.clone();
                    let comment_count = comments.len();

                    let comment_views: Vec<_> = comments.into_iter().map(|c| view! { <Comment comment=c /> }).collect();

                    view! {
                        <div>
                            // Tags
                            <div class="flex gap-1.5 mb-3">
                                <span class=category_class>{category_label}</span>
                                <span class=status_class>{status_label}</span>
                                {post.pinned.then(|| view! { <span class="tag tag-pinned">"Pinned"</span> })}
                                {post.locked.then(|| view! { <span class="tag tag-locked">"Locked"</span> })}
                            </div>

                            // Title + meta
                            <h1 class="text-2xl font-bold text-gray-900 mb-2">{title}</h1>
                            <div class="text-sm text-gray-400 mb-6">
                                <span>{username}</span>
                            </div>

                            // Body
                            <div class="p-5 bg-gray-50 border border-gray-200 rounded whitespace-pre-wrap leading-relaxed mb-4">
                                {body}
                            </div>

                            // Vote
                            <div class="mb-8">
                                <button
                                    class=move || {
                                        let base = "px-4 py-2 border rounded font-semibold text-sm transition-colors";
                                        if voted.get() {
                                            format!("{} border-gray-900 text-gray-900", base)
                                        } else {
                                            format!("{} border-gray-300 text-gray-700 hover:bg-gray-50", base)
                                        }
                                    }
                                    on:click=on_vote
                                    disabled=move || user.get().is_none()
                                >
                                    {move || format!("^ {}", upvotes.get())}
                                </button>
                            </div>

                            // Comments
                            <div class="mt-4">
                                <h2 class="text-base font-semibold text-gray-900 mb-4">{format!("Comments ({})", comment_count)}</h2>
                                {comment_views}

                                {move || {
                                    let logged_in = user.get().is_some();
                                    if is_locked {
                                        view! { <div class="text-center text-gray-400 py-5 text-sm">"This post is locked."</div> }.into_any()
                                    } else if logged_in {
                                        view! {
                                            <form class="mt-4" on:submit=on_comment>
                                                {move || comment_error.get().map(|e| view! { <div class="text-red-600 text-sm mb-2">{e}</div> })}
                                                <textarea
                                                    placeholder="Write a comment..."
                                                    rows=3
                                                    class="w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none mb-2"
                                                    prop:value=move || comment_body.get()
                                                    on:input=move |ev| comment_body.set(event_target_value(&ev))
                                                />
                                                <button
                                                    type="submit"
                                                    class="px-4 py-2 bg-gray-900 text-white text-sm rounded hover:bg-gray-800"
                                                >"Post Comment"</button>
                                            </form>
                                        }.into_any()
                                    } else {
                                        view! { <div class="text-center text-gray-400 py-5 text-sm">"Login to comment"</div> }.into_any()
                                    }
                                }}
                            </div>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
