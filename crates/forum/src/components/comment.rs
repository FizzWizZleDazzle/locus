use leptos::prelude::*;

use crate::api::ForumComment;

#[component]
pub fn Comment(comment: ForumComment) -> impl IntoView {
    let time_ago = format_time_ago(&comment.created_at);
    let username = comment.username.clone();
    let body = comment.body.clone();

    view! {
        <div class="border border-gray-200 rounded p-4 mb-2">
            <div class="flex gap-2 items-center mb-2">
                <span class="font-semibold text-sm text-gray-900">{username}</span>
                <span class="text-xs text-gray-400">{time_ago}</span>
            </div>
            <div class="text-sm text-gray-700 whitespace-pre-wrap">{body}</div>
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
