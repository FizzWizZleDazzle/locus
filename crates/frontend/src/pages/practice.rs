//! Practice mode page

use leptos::prelude::*;
use leptos::task::spawn_local;
use locus_common::{MainTopic, ProblemResponse};

use crate::{
    api,
    components::{MathInput, ProblemCard, TopicSelector},
    grader::{check_answer, preprocess_input, GradeResult},
};

#[component]
pub fn Practice() -> impl IntoView {
    // Topic selection state
    let (selected_topic, set_selected_topic) = signal(None::<MainTopic>);
    let (selected_subtopics, set_selected_subtopics) = signal(Vec::<String>::new());

    // Problem state
    let (problem, set_problem) = signal(None::<ProblemResponse>);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    // Answer state
    let (answer, set_answer) = signal(String::new());
    let (result, set_result) = signal(None::<GradeResult>);
    let (show_answer, set_show_answer) = signal(false);

    let load_problem = move || {
        set_loading.set(true);
        set_error.set(None);
        set_answer.set(String::new());
        set_result.set(None);
        set_show_answer.set(false);

        let topic = selected_topic.get().map(|t| t.as_str().to_string());
        let subtopics = selected_subtopics.get();

        spawn_local(async move {
            match api::get_problem(
                true,
                topic.as_deref(),
                Some(&subtopics),
            ).await {
                Ok(p) => {
                    set_problem.set(Some(p));
                    set_loading.set(false);
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

    let check = move || {
        if let Some(p) = problem.get() {
            let user_input = preprocess_input(&answer.get());
            if let Some(answer_key) = &p.answer_key {
                let grade = check_answer(&user_input, answer_key, p.grading_mode);
                set_result.set(Some(grade));
            }
        }
    };

    let on_submit = Callback::new(move |_| {
        check();
    });

    let reset_selection = move || {
        set_selected_topic.set(None);
        set_selected_subtopics.set(Vec::new());
        set_problem.set(None);
    };

    view! {
        <div class="max-w-2xl mx-auto py-8">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-xl font-medium text-gray-900">"Practice"</h1>
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

                    {move || problem.get().map(|p| {
                        let answer_key = if show_answer.get() {
                            p.answer_key.clone()
                        } else {
                            None
                        };
                        view! {
                            <ProblemCard problem=p show_answer=answer_key />
                        }
                    })}

                    {move || problem.get().is_some().then(|| view! {
                        <div class="space-y-4">
                            <MathInput
                                value=answer
                                set_value=set_answer
                                placeholder="Your answer"
                                on_submit=on_submit
                            />

                            <div class="flex space-x-2">
                                <button
                                    class="flex-1 px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 disabled:opacity-50"
                                    on:click=move |_| check()
                                    disabled=move || answer.get().is_empty()
                                >
                                    "Check"
                                </button>
                                <button
                                    class="px-4 py-2 border border-gray-300 rounded hover:border-gray-400"
                                    on:click=move |_| set_show_answer.set(true)
                                >
                                    "Reveal"
                                </button>
                            </div>

                            {move || result.get().map(|r| {
                                let (color, msg): (&str, String) = match r {
                                    GradeResult::Correct => ("text-green-600", "Correct".to_string()),
                                    GradeResult::Incorrect => ("text-red-600", "Incorrect".to_string()),
                                    GradeResult::Invalid(m) => ("text-yellow-600", m),
                                    GradeResult::Error(m) => ("text-red-600", m),
                                };
                                view! {
                                    <div class=format!("text-sm {}", color)>{msg}</div>
                                }
                            })}

                            {move || (result.get().map(|r| r.is_correct()).unwrap_or(false) || show_answer.get()).then(|| view! {
                                <button
                                    class="w-full px-4 py-2 border border-gray-300 rounded hover:border-gray-400"
                                    on:click=move |_| load_problem()
                                >
                                    "Next Problem"
                                </button>
                            })}
                        </div>
                    })}
                </div>
            })}
        </div>
    }
}
