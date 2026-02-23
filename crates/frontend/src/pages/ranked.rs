//! Ranked mode page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_query_map};
use locus_common::ProblemResponse;

use crate::{
    AuthContext, api,
    components::{ProblemInterface, TopicSelector},
    grader::preprocess_input,
    problem_queue::ProblemQueue,
    utils::{push_url_playing, setup_popstate_listener, update_url},
};

#[derive(Clone)]
struct SessionAttempt {
    is_correct: bool,
    elo_before: i32,
    elo_after: i32,
    time_taken_ms: Option<i32>,
    topic_streak: i32,
}

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

    // Session tracking
    let (session_attempts, set_session_attempts) = signal(Vec::<SessionAttempt>::new());
    let (session_start_elo, set_session_start_elo) = signal(0i32);
    let (show_summary, set_show_summary) = signal(false);

    // Problem queue for batch fetching
    let queue = ProblemQueue::new(false);

    let load_problem = move || {
        set_error.set(None);
        set_answer.set(String::new());
        set_result.set(None);

        let topic = selected_topic.get();
        let subtopics = selected_subtopics.get();

        if let Some(p) = queue.next(topic.clone(), subtopics.clone()) {
            set_problem.set(Some(p));
            set_loading.set(false);
            set_start_time.set(Some(js_sys::Date::now()));
        } else {
            set_loading.set(true);
            queue.fetch(topic, subtopics);
        }
    };

    // When the queue finishes loading and we're still waiting for a problem, pop one
    Effect::new(move |_| {
        if loading.get() && !queue.loading() {
            if let Some(err) = queue.error() {
                set_error.set(Some(err));
                set_loading.set(false);
            } else {
                let topic = selected_topic.get();
                let subtopics = selected_subtopics.get();
                if let Some(p) = queue.next(topic, subtopics) {
                    set_problem.set(Some(p));
                    set_loading.set(false);
                    set_start_time.set(Some(js_sys::Date::now()));
                }
            }
        }
    });

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

        // Clear stale problems from previous topic selection
        queue.clear();

        // Reset session state
        set_session_attempts.set(Vec::new());
        set_show_summary.set(false);

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
                // Parse subtopics from comma-separated string
                let subtopics = if let Some(st) = subtopics_param {
                    st.split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<String>>()
                } else {
                    Vec::new()
                };

                // Set initial values for TopicSelector
                set_initial_topic.set(Some(topic_val.clone()));
                set_initial_subtopics.set(Some(subtopics.clone()));

                // Also set parent state so callbacks work correctly
                set_selected_topic.set(Some(topic_val));
                set_selected_subtopics.set(subtopics);
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
            let time_taken = start_time
                .get()
                .map(|start| (js_sys::Date::now() - start) as i32);

            spawn_local(async move {
                match api::submit_answer(p.id, &user_input, time_taken).await {
                    Ok(resp) => {
                        // Record for session
                        let attempt = SessionAttempt {
                            is_correct: resp.is_correct,
                            elo_before: resp.elo_before,
                            elo_after: resp.elo_after,
                            time_taken_ms: time_taken,
                            topic_streak: resp.topic_streak,
                        };
                        set_session_attempts.update(|v| {
                            // Record session start ELO on first attempt
                            if v.is_empty() {
                                set_session_start_elo.set(resp.elo_before);
                            }
                            v.push(attempt);
                        });

                        set_result.set(Some(SubmitResult {
                            is_correct: resp.is_correct,
                            elo_before: resp.elo_before,
                            elo_after: resp.elo_after,
                            elo_change: resp.elo_change,
                            topic_streak: resp.topic_streak,
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
        set_session_attempts.set(Vec::new());
        set_show_summary.set(false);
        set_session_start_elo.set(0);
    };

    view! {
        <div class="max-w-2xl mx-auto px-4 py-8">
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-2xl font-semibold">"Ranked"</h1>
                <div class="flex items-center gap-3">
                    {move || {
                        let attempts = session_attempts.get();
                        (!attempts.is_empty()).then(|| view! {
                            <button
                                class="text-sm px-3 py-1.5 border border-gray-300 rounded hover:bg-gray-50"
                                on:click=move |_| set_show_summary.set(true)
                            >
                                "Finish Session"
                            </button>
                        })
                    }}
                    {move || problem.get().is_some().then(|| view! {
                        <button
                            class="text-sm text-gray-600 hover:text-gray-900"
                            on:click=move |_| reset_selection()
                        >
                            "Change Topics"
                        </button>
                    })}
                </div>
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
                                <div class="flex items-center gap-2 text-lg mb-2">
                                    {if r.is_correct {
                                        view! {
                                            <svg class="w-5 h-5 text-green-600 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                            </svg>
                                            <span>"Correct"</span>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <svg class="w-5 h-5 text-red-600 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                            </svg>
                                            <span>"Incorrect"</span>
                                        }.into_any()
                                    }}
                                </div>
                                {(r.is_correct && r.topic_streak > 0).then(|| view! {
                                    <div class="flex items-center gap-1 text-sm text-orange-600 mb-2">
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                                        </svg>
                                        {format!("{} correct in a row", r.topic_streak)}
                                    </div>
                                })}
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

            // Session summary modal
            {move || show_summary.get().then(|| {
                let attempts = session_attempts.get();
                let total = attempts.len() as i32;
                let correct = attempts.iter().filter(|a| a.is_correct).count() as i32;
                let accuracy = if total > 0 { correct * 100 / total } else { 0 };
                let start_elo = session_start_elo.get();
                let end_elo = attempts.last().map(|a| a.elo_after).unwrap_or(start_elo);
                let elo_delta = end_elo - start_elo;
                let elo_delta_color = if elo_delta >= 0 { "text-green-600" } else { "text-red-600" };
                let elo_delta_str = if elo_delta >= 0 { format!("+{}", elo_delta) } else { format!("{}", elo_delta) };

                let avg_time_ms = {
                    let timed: Vec<i32> = attempts.iter().filter_map(|a| a.time_taken_ms).collect();
                    if timed.is_empty() {
                        None
                    } else {
                        Some(timed.iter().sum::<i32>() / timed.len() as i32)
                    }
                };

                let best_streak = attempts.iter().map(|a| a.topic_streak).max().unwrap_or(0);

                view! {
                    <div class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4">
                        <div class="bg-white rounded-lg shadow-xl max-w-sm w-full p-6">
                            <h2 class="text-xl font-semibold mb-4">"Session Summary"</h2>

                            <div class="space-y-3 text-sm">
                                <div class="flex justify-between">
                                    <span class="text-gray-600">"Problems attempted"</span>
                                    <span class="font-medium">{total}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-gray-600">"Correct"</span>
                                    <span class="font-medium">{correct}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-gray-600">"Accuracy"</span>
                                    <span class="font-medium">{format!("{}%", accuracy)}</span>
                                </div>
                                <div class="border-t pt-3 flex justify-between">
                                    <span class="text-gray-600">"ELO change"</span>
                                    <span class="font-medium">
                                        {format!("{} → {}", start_elo, end_elo)}
                                        <span class=format!("ml-2 {}", elo_delta_color)>
                                            {elo_delta_str}
                                        </span>
                                    </span>
                                </div>
                                {avg_time_ms.map(|ms| view! {
                                    <div class="flex justify-between">
                                        <span class="text-gray-600">"Avg solve time"</span>
                                        <span class="font-medium">{format!("{:.1}s", ms as f64 / 1000.0)}</span>
                                    </div>
                                })}
                                <div class="flex justify-between">
                                    <span class="text-gray-600">"Best streak"</span>
                                    <span class="font-medium">{best_streak}</span>
                                </div>
                            </div>

                            <div class="mt-6 flex gap-3">
                                <button
                                    class="flex-1 px-4 py-2 border border-gray-300 rounded hover:bg-gray-50 text-sm"
                                    on:click=move |_| set_show_summary.set(false)
                                >
                                    "Keep Playing"
                                </button>
                                <button
                                    class="flex-1 px-4 py-2 bg-black text-white rounded hover:bg-gray-800 text-sm"
                                    on:click=move |_| reset_selection()
                                >
                                    "End Session"
                                </button>
                            </div>
                        </div>
                    </div>
                }
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
    topic_streak: i32,
}
