use leptos::prelude::*;
use leptos_router::components::A;

use crate::api::ForumPost;

#[component]
pub fn PostCard(post: ForumPost) -> impl IntoView {
    let category_class = if post.category == "bug_report" { "tag tag-bug" } else { "tag tag-feature" };
    let category_label = if post.category == "bug_report" { "Bug" } else { "Feature" };
    let status_class = format!("tag tag-status-{}", post.status);
    let status_label = post.status.replace('_', " ");
    let time_ago = format_time_ago(&post.created_at);
    let href = format!("/forum/post/{}", post.id);
    let title = post.title.clone();
    let username = post.username.clone();
    let comment_count = post.comment_count;
    let upvotes = post.upvotes;
    let pinned = post.pinned;
    let locked = post.locked;

    view! {
        <div class="flex gap-4 px-4 py-3 border border-gray-200 rounded mb-2 hover:border-gray-300 transition-colors" class:pinned-border=pinned>
            <div class="flex flex-col items-center min-w-[48px] pt-1">
                <span class="text-lg font-bold text-gray-900">{upvotes}</span>
                <span class="text-xs text-gray-400 uppercase">"votes"</span>
            </div>
            <div class="flex-1 min-w-0">
                <div class="flex gap-1.5 mb-1.5 flex-wrap">
                    {pinned.then(|| view! { <span class="tag tag-pinned">"Pinned"</span> })}
                    <span class=category_class>{category_label}</span>
                    <span class=status_class>{status_label}</span>
                    {locked.then(|| view! { <span class="tag tag-locked">"Locked"</span> })}
                </div>
                <A href=href attr:class="text-base font-semibold text-gray-900 hover:text-gray-600 block mb-1">{title}</A>
                <div class="text-sm text-gray-400 flex gap-1.5">
                    <span>{username}</span>
                    <span class="opacity-50">" · "</span>
                    <span>{time_ago}</span>
                    <span class="opacity-50">" · "</span>
                    <span>{format!("{} comments", comment_count)}</span>
                </div>
            </div>
        </div>
    }
}

fn format_time_ago(dt: &chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now - *dt;

    if diff.num_days() > 30 {
        format!("{}mo ago", diff.num_days() / 30)
    } else if diff.num_days() > 0 {
        format!("{}d ago", diff.num_days())
    } else if diff.num_hours() > 0 {
        format!("{}h ago", diff.num_hours())
    } else if diff.num_minutes() > 0 {
        format!("{}m ago", diff.num_minutes())
    } else {
        "just now".to_string()
    }
}
