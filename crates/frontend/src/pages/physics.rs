//! Physics topic browser and problem list page.
//!
//! Matches the styling of the Practice page: max-w-2xl centered layout,
//! same button styles, same card patterns.

use leptos::prelude::*;
use leptos_router::components::A;

use locus_physics_common::{PhysicsProblemSummary, PhysicsTopicInfo};

use crate::api;

#[component]
pub fn Physics() -> impl IntoView {
    let (topics, set_topics) = signal(Vec::<PhysicsTopicInfo>::new());
    let (selected_topic, set_selected_topic) = signal(Option::<String>::None);
    let (selected_subtopic, set_selected_subtopic) = signal(Option::<String>::None);
    let (problems, set_problems) = signal(Vec::<PhysicsProblemSummary>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);

    // Fetch topics on mount
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            match api::get_physics_topics().await {
                Ok(t) => {
                    set_topics.set(t);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    });

    // Fetch problems when topic/subtopic changes
    Effect::new(move |_| {
        let topic = selected_topic.get();
        let subtopic = selected_subtopic.get();

        if topic.is_none() {
            set_problems.set(vec![]);
            return;
        }

        set_loading.set(true);
        leptos::task::spawn_local(async move {
            match api::get_physics_problems(topic.as_deref(), subtopic.as_deref(), 20).await {
                Ok(p) => {
                    set_problems.set(p);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    });

    view! {
        <div class="max-w-2xl mx-auto px-4 py-8">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-2xl font-semibold">"Physics"</h1>
                {move || selected_topic.get().is_some().then(|| view! {
                    <button
                        class="text-sm text-gray-500 hover:text-gray-900 dark:hover:text-white"
                        on:click=move |_| {
                            set_selected_topic.set(None);
                            set_selected_subtopic.set(None);
                        }
                    >
                        "All Topics"
                    </button>
                })}
            </div>

            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            // Topic selector (when no topic is selected)
            {move || selected_topic.get().is_none().then(|| view! {
                <div class="space-y-3">
                    {move || topics.get().into_iter().filter(|t| t.enabled).map(|topic| {
                        let topic_id = topic.id.clone();
                        let topic_id2 = topic.id.clone();
                        view! {
                            <button
                                class="w-full text-left p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-gray-400 dark:hover:border-gray-500 transition-colors"
                                on:click=move |_| set_selected_topic.set(Some(topic_id.clone()))
                            >
                                <div class="flex items-center justify-between">
                                    <div>
                                        <h2 class="font-medium text-gray-900 dark:text-white">{topic.display_name.clone()}</h2>
                                        <p class="text-sm text-gray-500 mt-0.5">
                                            {format!("{} subtopics", topic.subtopics.len())}
                                        </p>
                                    </div>
                                    <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                    </svg>
                                </div>
                            </button>
                        }
                    }).collect::<Vec<_>>()}

                    {move || (loading.get() && topics.get().is_empty()).then(|| view! {
                        <div class="text-gray-500 text-sm text-center py-8">"Loading topics..."</div>
                    })}
                </div>
            })}

            // Subtopic filter (when topic is selected)
            {move || selected_topic.get().map(|topic_id| {
                let topic_info = topics.get().into_iter().find(|t| t.id == topic_id);
                view! {
                    <div class="mb-4">
                        {topic_info.map(|info| view! {
                            <div class="flex flex-wrap gap-2 mb-4">
                                <button
                                    class=move || format!(
                                        "px-3 py-1 text-sm rounded-full border transition-colors {}",
                                        if selected_subtopic.get().is_none() {
                                            "bg-gray-900 text-white border-gray-900 dark:bg-white dark:text-gray-900 dark:border-white"
                                        } else {
                                            "border-gray-300 dark:border-gray-600 hover:border-gray-400"
                                        }
                                    )
                                    on:click=move |_| set_selected_subtopic.set(None)
                                >
                                    "All"
                                </button>
                                {info.subtopics.iter().filter(|st| st.enabled).map(|st| {
                                    let st_id = st.id.clone();
                                    let st_id2 = st.id.clone();
                                    let st_id3 = st.id.clone();
                                    view! {
                                        <button
                                            class=move || format!(
                                                "px-3 py-1 text-sm rounded-full border transition-colors {}",
                                                if selected_subtopic.get().as_deref() == Some(&st_id) {
                                                    "bg-gray-900 text-white border-gray-900 dark:bg-white dark:text-gray-900 dark:border-white"
                                                } else {
                                                    "border-gray-300 dark:border-gray-600 hover:border-gray-400"
                                                }
                                            )
                                            on:click=move |_| set_selected_subtopic.set(Some(st_id2.clone()))
                                        >
                                            {st.display_name.clone()}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        })}
                    </div>
                }
            })}

            // Problem list
            {move || selected_topic.get().is_some().then(|| view! {
                <div class="space-y-2">
                    {move || if loading.get() {
                        view! {
                            <div class="text-gray-500 text-sm text-center py-8">"Loading problems..."</div>
                        }.into_any()
                    } else if problems.get().is_empty() {
                        view! {
                            <div class="text-gray-500 text-sm text-center py-8">"No problems available for this topic yet."</div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-2">
                                {move || problems.get().into_iter().map(|p| {
                                    let href = format!("/physics/problem/{}", p.id);
                                    let difficulty_label = match p.difficulty {
                                        1 => "Easy",
                                        2 => "Medium",
                                        3 => "Hard",
                                        4 => "Expert",
                                        _ => "Medium",
                                    };
                                    let difficulty_color = match p.difficulty {
                                        1 => "text-green-600 bg-green-50",
                                        2 => "text-amber-600 bg-amber-50",
                                        3 => "text-red-600 bg-red-50",
                                        4 => "text-purple-600 bg-purple-50",
                                        _ => "text-gray-600 bg-gray-50",
                                    };
                                    let solved = p.user_solved.unwrap_or(false);

                                    view! {
                                        <A href=href attr:class="block p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-gray-400 dark:hover:border-gray-500 transition-colors">
                                            <div class="flex items-center justify-between">
                                                <div class="flex-1 min-w-0">
                                                    <div class="flex items-center gap-2">
                                                        <h3 class="font-medium text-gray-900 dark:text-white truncate">
                                                            {p.title.clone()}
                                                        </h3>
                                                        {solved.then(|| view! {
                                                            <svg class="w-4 h-4 text-green-500 flex-shrink-0" fill="currentColor" viewBox="0 0 24 24">
                                                                <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                                            </svg>
                                                        })}
                                                    </div>
                                                    <p class="text-sm text-gray-500 mt-0.5 truncate">
                                                        {p.description_latex.chars().take(80).collect::<String>()}
                                                        {(p.description_latex.len() > 80).then(|| "...")}
                                                    </p>
                                                </div>
                                                <span class=format!("ml-3 px-2 py-0.5 text-xs font-medium rounded {}", difficulty_color)>
                                                    {difficulty_label}
                                                </span>
                                            </div>
                                        </A>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }}
                </div>
            })}
        </div>
    }
}
