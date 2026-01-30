//! Ranked mode page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use locus_common::{MainTopic, ProblemResponse};

use crate::{
    api,
    components::{MathInput, ProblemCard, TopicSelector},
    grader::preprocess_input,
    AuthContext,
};

#[component]
pub fn Ranked() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = use_navigate();

    Effect::new(move |_| {
        if !auth.is_logged_in.get() {
            navigate("/login", Default::default());
        }
    });

    // Topic selection state
    let (selected_topic, set_selected_topic) = signal(None::<MainTopic>);
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

        let topic = selected_topic.get().map(|t| t.as_str().to_string());
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

    let on_topic_confirm = Callback::new(move |(topic, subtopics): (MainTopic, Vec<String>)| {
        set_selected_topic.set(Some(topic));
        set_selected_subtopics.set(subtopics);
        load_problem();
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
        <div class="max-w-2xl mx-auto py-8">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-xl font-medium text-gray-900">"Ranked"</h1>
                {move || selected_topic.get().is_some().then(|| view! {
                    <button
                        class="text-sm text-gray-500 hover:text-gray-900"
                        on:click=move |_| reset_selection()
                    >
                        "Change Topics"
                    </button>
                })}
            </div>

            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            // Show topic selector if no topic selected
            {move || selected_topic.get().is_none().then(|| view! {
                <div class="border border-gray-200 rounded p-6">
                    <TopicSelector on_confirm=on_topic_confirm />
                </div>
            })}

            // Show problem once topic is selected
            {move || selected_topic.get().is_some().then(|| view! {
                <div class="space-y-6">
                    {move || loading.get().then(|| view! {
                        <div class="text-gray-500 text-sm">"Loading..."</div>
                    })}

                    {move || problem.get().map(|p| view! {
                        <ProblemCard problem=p />
                    })}

                    {move || (problem.get().is_some() && result.get().is_none()).then(|| view! {
                        <div class="space-y-4">
                            <MathInput
                                value=answer
                                set_value=set_answer
                                placeholder="Your answer"
                                disabled=submitting.get()
                                on_submit=on_submit
                            />

                            <button
                                class="w-full px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 disabled:opacity-50"
                                on:click=move |_| submit()
                                disabled=move || answer.get().is_empty() || submitting.get()
                            >
                                {move || if submitting.get() { "Submitting..." } else { "Submit" }}
                            </button>
                        </div>
                    })}

                    {move || result.get().map(|r| {
                        let elo_color = if r.elo_change >= 0 { "text-green-600" } else { "text-red-600" };
                        let elo_prefix = if r.elo_change >= 0 { "+" } else { "" };

                        view! {
                            <div class="p-4 border border-gray-200 rounded">
                                <div class="text-lg mb-2">
                                    {if r.is_correct { "Correct" } else { "Incorrect" }}
                                </div>
                                <div class="text-sm text-gray-600">
                                    {format!("{} → {}", r.elo_before, r.elo_after)}
                                    <span class=format!(" ml-2 {}", elo_color)>
                                        {format!("({}{})", elo_prefix, r.elo_change)}
                                    </span>
                                </div>
                                <button
                                    class="mt-4 w-full px-4 py-2 border border-gray-300 rounded hover:border-gray-400"
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
