//! Topic and subtopic selection component

use leptos::prelude::*;
use locus_common::{MainTopic, subtopic_display_name};
use std::collections::HashSet;

#[component]
pub fn TopicSelector(
    /// Callback when user confirms selection
    on_confirm: Callback<(MainTopic, Vec<String>)>,
) -> impl IntoView {
    let (selected_topic, set_selected_topic) = signal(None::<MainTopic>);
    let (selected_subtopics, set_selected_subtopics) = signal(HashSet::<String>::new());

    let toggle_subtopic = move |subtopic: String| {
        set_selected_subtopics.update(|set| {
            if set.contains(&subtopic) {
                set.remove(&subtopic);
            } else {
                set.insert(subtopic);
            }
        });
    };

    let confirm = move || {
        if let Some(topic) = selected_topic.get() {
            let subtopics: Vec<String> = selected_subtopics.get().into_iter().collect();
            on_confirm.run((topic, subtopics));
        }
    };

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-medium text-gray-900 mb-3">"Select Subject"</h2>
                <div class="grid grid-cols-2 gap-2">
                    {MainTopic::all().iter().map(|topic| {
                        let t = topic.clone();
                        let t2 = topic.clone();
                        let is_selected = move || selected_topic.get() == Some(t.clone());

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
                                    set_selected_topic.set(Some(t2.clone()));
                                    set_selected_subtopics.set(HashSet::new());
                                }
                            >
                                {topic.display_name()}
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>

            {move || selected_topic.get().map(|topic| {
                let subtopics = topic.subtopics();

                view! {
                    <div>
                        <h3 class="text-sm font-medium text-gray-700 mb-2">
                            "Select Topics (check at least one)"
                        </h3>
                        <div class="space-y-1">
                            {subtopics.iter().map(|subtopic| {
                                let st = (*subtopic).to_string();
                                let st_check = st.clone();
                                let st_toggle = st.clone();
                                let st_display = st.clone();
                                let is_checked = move || selected_subtopics.get().contains(&st_check.clone());

                                view! {
                                    <label class="flex items-center space-x-2 text-sm cursor-pointer">
                                        <input
                                            type="checkbox"
                                            class="rounded border-gray-300"
                                            prop:checked=is_checked
                                            on:change=move |_| toggle_subtopic(st_toggle.clone())
                                        />
                                        <span>{subtopic_display_name(&st_display)}</span>
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
            })}
        </div>
    }
}
