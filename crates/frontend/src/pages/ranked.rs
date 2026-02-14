//! Ranked mode page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_query_map};
use locus_common::{MainTopic, ProblemResponse};

use crate::{
    api,
    components::{ProblemInterface, TopicSelector},
    grader::preprocess_input,
    utils::{update_url, push_url_playing, setup_popstate_listener},
    AuthContext,
};

#[component]
pub fn Ranked() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = use_navigate();
    let query = use_query_map();

    let navigate_clone = navigate.clone();
    Effect::new(move |_| {
        if !auth.is_logged_in.get() {
            navigate_clone("/login", Default::default());
        }
    });

    // Topic selection state
    let (selected_topic, set_selected_topic) = signal(None::<String>);
    let (selected_subtopics, set_selected_subtopics) = signal(Vec::<String>::new());

    // Problem state
    let (problem, set_problem) = signal(None::<ProblemResponse>);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    // Answer state
    let (answer, set_answer) = signal(String::new());
    let (submitting, set_submitting) = signal(false);
    let (result, set_result) = signal(None::<SubmitResult>);
    let (start_time, set_start_time) = signal(None::<f64>);

    let load_problem = move || {
        set_loading.set(true);
        set_error.set(None);
        set_answer.set(String::new());
        set_result.set(None);

        let topic = selected_topic.get();
        let subtopics = selected_subtopics.get();

        spawn_local(async move {
            match api::get_problem(false, topic.as_deref(), Some(&subtopics)).await {
                Ok(p) => {
                    set_problem.set(Some(p));
                    set_loading.set(false);
                    set_start_time.set(Some(js_sys::Date::now()));
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    };

    let on_topic_change = Callback::new(move |topic: String| {
        set_selected_topic.set(Some(topic.clone()));
        set_selected_subtopics.set(Vec::new());

        // Update URL immediately when topic is selected
        update_url(&format!("/ranked?topic={}", topic));
    });

    let on_subtopics_change = Callback::new(move |subtopics: Vec<String>| {
        set_selected_subtopics.set(subtopics.clone());

        // Update URL immediately when subtopics change
        if let Some(topic) = selected_topic.get() {
            let url = if subtopics.is_empty() {
                format!("/ranked?topic={}", topic)
            } else {
                let subtopic_str = subtopics.join(",");
                format!("/ranked?topic={}&subtopics={}", topic, subtopic_str)
            };
            update_url(&url);
        }
    });

    let on_topic_confirm = Callback::new(move |(topic, subtopics): (String, Vec<String>)| {
        set_selected_topic.set(Some(topic.clone()));
        set_selected_subtopics.set(subtopics.clone());

        // Create a history entry with 'playing' state so back button can return to topic selector
        let url = if subtopics.is_empty() {
            format!("/ranked?topic={}", topic)
        } else {
            let subtopic_str = subtopics.join(",");
            format!("/ranked?topic={}&subtopics={}", topic, subtopic_str)
        };
        push_url_playing(&url);

        load_problem();
    });

    // Parse URL params on mount to get initial values
    let (initial_topic, set_initial_topic) = signal(None::<String>);
    let (initial_subtopics, set_initial_subtopics) = signal(None::<Vec<String>>);

    // Watch for URL changes (including browser back/forward)
    Effect::new(move |_| {
        let topic_param = query.read().get("topic");
        let subtopics_param = query.read().get("subtopics");

        if let Some(topic_val) = topic_param {
            if !topic_val.is_empty() {
                // Validate topic exists - silently ignore if invalid
                let main_topic = match MainTopic::from_str(&topic_val) {
                    Some(t) => t,
                    None => return, // Invalid topic, ignore params
                };

                // Parse subtopics from comma-separated string
                let subtopics = if let Some(st) = subtopics_param {
                    st.split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<String>>()
                } else {
                    Vec::new()
                };

                // Validate subtopics belong to topic - filter out invalid ones
                let valid_subtopic_list = main_topic.subtopics();
                let filtered_subtopics: Vec<String> = subtopics.into_iter()
                    .filter(|st| valid_subtopic_list.contains(&st.as_str()))
                    .collect();

                // Set initial values for TopicSelector
                set_initial_topic.set(Some(topic_val.clone()));
                set_initial_subtopics.set(Some(filtered_subtopics.clone()));

                // Also set parent state so callbacks work correctly
                set_selected_topic.set(Some(topic_val));
                set_selected_subtopics.set(filtered_subtopics);
            }
        } else {
            // No topic in URL - clear everything to show topic selector
            set_initial_topic.set(None);
            set_initial_subtopics.set(None);
            set_selected_topic.set(None);
            set_selected_subtopics.set(Vec::new());
            set_problem.set(None);
            set_result.set(None);
        }
    });

    // Listen for popstate events (back/forward navigation)
    Effect::new(move |_| {
        setup_popstate_listener(move || {
            set_problem.set(None);
            set_result.set(None);
        });
    });

    let submit = move || {
        if let Some(p) = problem.get() {
            set_submitting.set(true);
            set_error.set(None);

            let user_input = preprocess_input(&answer.get());
            let time_taken = start_time.get().map(|start| {
                (js_sys::Date::now() - start) as i32
            });

            spawn_local(async move {
                match api::submit_answer(p.id, &user_input, time_taken).await {
                    Ok(resp) => {
                        set_result.set(Some(SubmitResult {
                            is_correct: resp.is_correct,
                            elo_before: resp.elo_before,
                            elo_after: resp.elo_after,
                            elo_change: resp.elo_change,
                        }));
                        set_submitting.set(false);
                    }
                    Err(e) => {
                        set_error.set(Some(e.message));
                        set_submitting.set(false);
                    }
                }
            });
        }
    };

    let on_submit = Callback::new(move |_| {
        if !submitting.get() && !answer.get().is_empty() {
            submit();
        }
    });

    let reset_selection = move || {
        set_selected_topic.set(None);
        set_selected_subtopics.set(Vec::new());
        set_problem.set(None);
        set_result.set(None);
    };

    view! {
        <div class="max-w-2xl mx-auto px-4 py-8">
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-2xl font-semibold">"Ranked"</h1>
                {move || problem.get().is_some().then(|| view! {
                    <button
                        class="text-sm text-gray-600 hover:text-gray-900"
                        on:click=move |_| reset_selection()
                    >
                        "Change Topics"
                    </button>
                })}
            </div>

            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            // Show topic selector if no problem loaded
            {move || problem.get().is_none().then(|| view! {
                <div class="border border-gray-200 rounded p-6">
                    <TopicSelector
                        on_confirm=on_topic_confirm
                        on_topic_change=on_topic_change
                        on_subtopics_change=on_subtopics_change
                        initial_topic=initial_topic.get()
                        initial_subtopics=initial_subtopics.get().unwrap_or_default()
                    />
                </div>
            })}

            // Show problem once loaded
            {move || problem.get().is_some().then(|| view! {
                <div class="space-y-6">
                    {move || loading.get().then(|| view! {
                        <div class="text-gray-500 text-sm">"Loading..."</div>
                    })}

                    {move || result.get().is_none().then(|| view! {
                        <ProblemInterface
                            problem=problem
                            answer=answer
                            set_answer=set_answer
                            on_submit=on_submit
                            render_controls=move || view! {
                                <button
                                    class="w-full px-4 py-3 bg-black text-white hover:bg-gray-800 disabled:opacity-50"
                                    on:click=move |_| submit()
                                    disabled=move || answer.get().is_empty() || submitting.get()
                                >
                                    {move || if submitting.get() { "Submitting..." } else { "Submit" }}
                                </button>
                            }
                            render_result=|| ()
                        />
                    })}

                    {move || result.get().map(|r| {
                        let elo_color = if r.elo_change >= 0 { "text-green-600" } else { "text-red-600" };
                        let elo_prefix = if r.elo_change >= 0 { "+" } else { "" };

                        view! {
                            <div class="p-6 border rounded">
                                <div class="text-lg mb-2">
                                    {if r.is_correct { "✓ Correct" } else { "✗ Incorrect" }}
                                </div>
                                <div class="text-sm text-gray-600 mb-4">
                                    {format!("{} → {}", r.elo_before, r.elo_after)}
                                    <span class=format!("ml-2 font-medium {}", elo_color)>
                                        {format!("({}{})", elo_prefix, r.elo_change)}
                                    </span>
                                </div>
                                <button
                                    class="w-full px-4 py-3 border hover:bg-gray-50 rounded"
                                    on:click=move |_| load_problem()
                                >
                                    "Next Problem"
                                </button>
                            </div>
                        }
                    })}
                </div>
            })}
        </div>
    }
}

#[derive(Clone)]
struct SubmitResult {
    is_correct: bool,
    elo_before: i32,
    elo_after: i32,
    elo_change: i32,
}
