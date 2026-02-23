//! Leaderboard page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_query_map;
use locus_common::LeaderboardEntry;

use crate::{api, api::Topic, utils::update_url};

#[component]
pub fn Leaderboard() -> impl IntoView {
    let query = use_query_map();

    // Initialize topic from URL or default to calculus
    let initial_topic = query
        .read()
        .get("topic")
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| "calculus".to_string());

    let (selected_topic, set_selected_topic) = signal(initial_topic);
    let (topics_list, set_topics_list) = signal(Vec::<Topic>::new());
    let (entries, set_entries) = signal(Vec::<LeaderboardEntry>::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    // Fetch topics from API on mount
    Effect::new(move |_| {
        spawn_local(async move {
            if let Ok(topics) = api::get_topics().await {
                set_topics_list.set(topics);
            }
        });
    });

    let load_leaderboard = move || {
        set_loading.set(true);
        set_error.set(None);

        let topic = selected_topic.get();

        spawn_local(async move {
            match api::get_leaderboard(&topic).await {
                Ok(resp) => {
                    set_entries.set(resp.entries);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    };

    // Load on mount and when topic changes
    Effect::new(move |_| {
        let _ = selected_topic.get();
        load_leaderboard();
    });

    view! {
        <div class="max-w-2xl mx-auto py-8">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-xl font-medium text-gray-900">"Leaderboard"</h1>

                <select
                    class="text-sm border border-gray-300 rounded px-3 py-1.5"
                    on:change=move |ev| {
                        let new_topic = event_target_value(&ev);
                        set_selected_topic.set(new_topic.clone());

                        // Update URL so users can copy and share it
                        update_url(&format!("/leaderboard?topic={}", new_topic));
                    }
                    prop:value=selected_topic
                >
                    {move || topics_list.get().into_iter().map(|topic| {
                        view! {
                            <option value=topic.id.clone()>{topic.display_name.clone()}</option>
                        }
                    }).collect_view()}
                </select>
            </div>

            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            {move || loading.get().then(|| view! {
                <div class="text-gray-500 text-sm">"Loading..."</div>
            })}

            {move || (!loading.get() && error.get().is_none()).then(|| {
                let current_entries = entries.get();
                if current_entries.is_empty() {
                    view! {
                        <div class="text-gray-500 text-sm">"No players yet for this topic"</div>
                    }.into_any()
                } else {
                    view! {
                        <div class="border border-gray-200 rounded divide-y divide-gray-100">
                            {current_entries.into_iter().map(|entry| {
                                view! {
                                    <div class="flex items-center justify-between px-4 py-3">
                                        <div class="flex items-center space-x-3">
                                            <span class="text-sm text-gray-500 w-8 text-right">{entry.rank}</span>
                                            <span class="text-gray-900">{entry.username}</span>
                                        </div>
                                        <span class="font-mono text-gray-700">{entry.elo}</span>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                }
            })}
        </div>
    }
}
