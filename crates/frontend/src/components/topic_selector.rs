//! Topic and subtopic selection component

use crate::api::{self, Topic};
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::collections::HashSet;

#[component]
pub fn TopicSelector(
    /// Callback when user confirms selection
    on_confirm: Callback<(String, Vec<String>)>,
    /// Optional callback when topic changes
    #[prop(optional)]
    on_topic_change: Option<Callback<String>>,
    /// Optional callback when subtopics change
    #[prop(optional)]
    on_subtopics_change: Option<Callback<Vec<String>>>,
    /// Optional initial topic to pre-select
    #[prop(into, default = None)]
    initial_topic: Option<String>,
    /// Optional initial subtopics to pre-select
    #[prop(into, default = Vec::new())]
    initial_subtopics: Vec<String>,
) -> impl IntoView {
    let (selected_topic_id, set_selected_topic_id) = signal(initial_topic);
    let (selected_subtopics, set_selected_subtopics) =
        signal::<HashSet<String>>(initial_subtopics.into_iter().collect());

    // Fetch topics from API
    let (topics, set_topics) = signal(None::<Vec<Topic>>);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);

    // Load topics on mount
    Effect::new(move |_| {
        spawn_local(async move {
            match api::get_topics().await {
                Ok(t) => {
                    set_topics.set(Some(t));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    });

    let toggle_subtopic = move |subtopic: String| {
        set_selected_subtopics.update(|set| {
            if set.contains(&subtopic) {
                set.remove(&subtopic);
            } else {
                set.insert(subtopic);
            }
        });

        // Notify parent of subtopic change
        if let Some(callback) = on_subtopics_change {
            let subtopics: Vec<String> = selected_subtopics.get().into_iter().collect();
            callback.run(subtopics);
        }
    };

    let confirm = move || {
        if let Some(topic_id) = selected_topic_id.get() {
            let subtopics: Vec<String> = selected_subtopics.get().into_iter().collect();
            on_confirm.run((topic_id, subtopics));
        }
    };

    view! {
        <div class="space-y-6">
            {move || {
                if loading.get() {
                    view! {
                        <div class="text-center py-8 text-gray-500">"Loading subjects..."</div>
                    }.into_any()
                } else if let Some(err) = error.get() {
                    view! {
                        <div class="text-center py-8 text-red-600">
                            "Failed to load subjects: " {err}
                        </div>
                    }.into_any()
                } else if let Some(topics_list) = topics.get() {
                    view! {
                        <div>
                            <h2 class="text-lg font-medium text-gray-900 mb-3">"Select Subject"</h2>
                            <div class="grid grid-cols-2 gap-2">
                                {topics_list.iter().map(|topic| {
                                    let topic_id = topic.id.clone();
                                    let topic_id_check = topic.id.clone();
                                    let display_name = topic.display_name.clone();
                                    let is_selected = move || selected_topic_id.get().as_ref() == Some(&topic_id_check);

                                    view! {
                                        <button
                                            class=move || format!(
                                                "px-4 py-2 border rounded text-sm transition-colors {}",
                                                if is_selected() {
                                                    "border-gray-900 bg-gray-900 text-white"
                                                } else {
                                                    "border-gray-300 hover:border-gray-400"
                                                }
                                            )
                                            on:click=move |_| {
                                                set_selected_topic_id.set(Some(topic_id.clone()));
                                                set_selected_subtopics.set(HashSet::new());

                                                // Notify parent of topic change
                                                if let Some(callback) = on_topic_change {
                                                    callback.run(topic_id.clone());
                                                }
                                                // Notify parent that subtopics were cleared
                                                if let Some(callback) = on_subtopics_change {
                                                    callback.run(Vec::new());
                                                }
                                            }
                                        >
                                            {display_name}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>

                            {move || {
                                let selected_id = selected_topic_id.get();
                                selected_id.and_then(|id| {
                                    topics_list.iter().find(|t| t.id == id).map(|topic| {
                                        view! {
                                            <div>
                                                <h3 class="text-sm font-medium text-gray-700 mb-2">
                                                    "Select Topics (check at least one)"
                                                </h3>
                                                <div class="space-y-1">
                                                    {topic.subtopics.iter().map(|subtopic| {
                                                        let st_id_check = subtopic.id.clone();
                                                        let st_id_toggle = subtopic.id.clone();
                                                        let st_display = subtopic.display_name.clone();
                                                        let is_checked = move || selected_subtopics.get().contains(&st_id_check);

                                                        view! {
                                                            <label class="flex items-center space-x-2 text-sm cursor-pointer">
                                                                <input
                                                                    type="checkbox"
                                                                    class="rounded border-gray-300"
                                                                    prop:checked=is_checked
                                                                    on:change=move |_| toggle_subtopic(st_id_toggle.clone())
                                                                />
                                                                <span>{st_display}</span>
                                                            </label>
                                                        }
                                                    }).collect_view()}
                                                </div>

                                                <button
                                                    class="mt-4 w-full px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 disabled:opacity-50"
                                                    on:click=move |_| confirm()
                                                    disabled=move || selected_subtopics.get().is_empty()
                                                >
                                                    "Start"
                                                </button>
                                            </div>
                                        }
                                    })
                                })
                            }}
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}
