//! Spaced repetition review page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use locus_common::{ProblemResponse, ReviewQueueItem};

use crate::{AuthContext, api};
use crate::components::{AnswerInput, LatexRenderer, ProblemCard};
use crate::grader::{GradeResult, check_answer, preprocess_input};
use crate::katex_bindings::render_plain_math_to_string;
use crate::utils::escape_html;

#[component]
pub fn Review() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = leptos_router::hooks::use_navigate();

    let navigate_clone = navigate.clone();
    Effect::new(move |_| {
        if !auth.is_logged_in.get() {
            navigate_clone("/login", Default::default());
        }
    });

    let (queue, set_queue) = signal(Vec::<ReviewQueueItem>::new());
    let (due_count, set_due_count) = signal(0i64);
    let (upcoming_count, set_upcoming_count) = signal(0i64);
    let (loading, set_loading) = signal(true);
    let (current_index, set_current_index) = signal(0usize);
    let (answer, set_answer) = signal(String::new());
    let (show_result, set_show_result) = signal(false);
    let (is_correct, set_is_correct) = signal(false);
    let (submitting, set_submitting) = signal(false);

    let load_queue = move || {
        set_loading.set(true);
        spawn_local(async move {
            match api::get_review_queue().await {
                Ok(resp) => {
                    set_due_count.set(resp.due_count);
                    set_upcoming_count.set(resp.upcoming_count);
                    set_queue.set(resp.items);
                    set_current_index.set(0);
                    set_show_result.set(false);
                    set_answer.set(String::new());
                }
                Err(_) => {}
            }
            set_loading.set(false);
        });
    };

    Effect::new(move |_| {
        load_queue();
    });

    let current_item = move || {
        let q = queue.get();
        let idx = current_index.get();
        q.get(idx).cloned()
    };

    let handle_check = move |_input: String| {
        if let Some(item) = current_item() {
            let user_input = preprocess_input(&answer.get());
            let result = check_answer(&user_input, &item.answer_key, item.grading_mode, item.answer_type);
            set_is_correct.set(matches!(result, GradeResult::Correct));
            set_show_result.set(true);
        }
    };

    let handle_rate = move |quality: i32| {
        if let Some(item) = current_item() {
            set_submitting.set(true);
            spawn_local(async move {
                let _ = api::complete_review(item.review_id, quality).await;
                set_submitting.set(false);

                // Move to next item
                let idx = current_index.get_untracked();
                let len = queue.get_untracked().len();
                if idx + 1 < len {
                    set_current_index.set(idx + 1);
                    set_show_result.set(false);
                    set_answer.set(String::new());
                    set_is_correct.set(false);
                } else {
                    // Queue exhausted, reload
                    load_queue();
                }
            });
        }
    };

    view! {
        <div class="max-w-3xl mx-auto px-4 py-8">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-2xl font-semibold">"Review"</h1>
                <div class="flex items-center gap-4">
                    <span class="text-sm text-gray-500">{move || format!("{} due", due_count.get())}</span>
                    <span class="text-sm text-gray-400">{move || format!("{} upcoming", upcoming_count.get())}</span>
                </div>
            </div>

            {move || loading.get().then(|| view! {
                <div class="text-gray-500 text-sm">"Loading..."</div>
            })}

            {move || {
                if !loading.get() && queue.get().is_empty() {
                    Some(view! {
                        <div class="text-center py-16">
                            <div class="text-4xl mb-4">"&#10003;"</div>
                            <p class="text-lg text-gray-700 dark:text-gray-300 mb-2">"Nothing to review!"</p>
                            <p class="text-sm text-gray-500 mb-4">
                                {move || {
                                    let upcoming = upcoming_count.get();
                                    if upcoming > 0 {
                                        format!("{} items coming up for review later.", upcoming)
                                    } else {
                                        "Get wrong answers in ranked mode to build your review queue.".to_string()
                                    }
                                }}
                            </p>
                            <A href="/ranked" attr:class="text-blue-600 hover:underline">"Play Ranked"</A>
                        </div>
                    })
                } else {
                    None
                }
            }}

            {move || current_item().map(|item| {
                let queue_len = queue.get().len();
                let idx = current_index.get();
                let answer_key = item.answer_key.clone();
                let solution = item.solution_latex.clone();
                let answer_type = item.answer_type;
                let problem_id = item.problem_id.to_string();

                // Build ProblemResponse for ProblemCard
                let prob = ProblemResponse {
                    id: item.problem_id,
                    question_latex: item.question_latex.clone(),
                    difficulty: item.difficulty,
                    main_topic: item.main_topic.clone(),
                    subtopic: item.subtopic.clone(),
                    grading_mode: item.grading_mode,
                    answer_type: item.answer_type,
                    calculator_allowed: String::new(),
                    answer_key: None,
                    solution_latex: String::new(),
                    question_image: item.question_image.clone(),
                    time_limit_seconds: None,
                };

                let on_submit = Callback::new(handle_check);

                view! {
                    <div class="space-y-4">
                        // Progress
                        <div class="flex items-center justify-between text-sm text-gray-500">
                            <span>{format!("Problem {} of {}", idx + 1, queue_len)}</span>
                        </div>

                        // Problem card
                        <ProblemCard problem=prob key=problem_id.clone() />

                        // Answer input or result
                        {move || if !show_result.get() {
                            let hint = answer_type.hint();
                            view! {
                                <div class="space-y-2">
                                    {hint.map(|h| view! {
                                        <p class="text-xs text-gray-400 italic">{h}</p>
                                    })}
                                    <AnswerInput
                                        answer_type=answer_type
                                        value=answer
                                        set_value=set_answer
                                        on_submit=on_submit
                                        key=problem_id.clone()
                                    />
                                    <button
                                        class="w-full px-4 py-2 bg-gray-900 dark:bg-gray-100 text-white dark:text-gray-900 rounded text-sm hover:bg-gray-700 dark:hover:bg-gray-300"
                                        on:click=move |_| on_submit.run(answer.get())
                                    >"Check"</button>
                                </div>
                            }.into_any()
                        } else {
                            let ak = answer_key.clone();
                            let sol = solution.clone();
                            let correct_rendered = render_plain_math_to_string(&ak)
                                .unwrap_or_else(|_| format!("<code>{}</code>", escape_html(&ak)));
                            view! {
                                <div class="space-y-4">
                                    // Result
                                    <div class=move || format!(
                                        "rounded-lg p-4 text-sm {}",
                                        if is_correct.get() { "bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-400" }
                                        else { "bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-400" }
                                    )>
                                        {move || if is_correct.get() {
                                            view! { <span>"Correct!"</span> }.into_any()
                                        } else {
                                            view! {
                                                <div>
                                                    <span>"Incorrect. "</span>
                                                    <span>"Correct answer: "</span>
                                                    <span class="font-bold" inner_html=correct_rendered.clone()></span>
                                                </div>
                                            }.into_any()
                                        }}
                                    </div>

                                    // Solution
                                    {if !sol.is_empty() {
                                        let s = sol.clone();
                                        view! {
                                            <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
                                                <div class="text-xs text-gray-500 mb-2">"Solution"</div>
                                                <LatexRenderer content=s />
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }}

                                    // Self-assessment buttons
                                    <div>
                                        <div class="text-sm text-gray-500 mb-2">"How well did you know this?"</div>
                                        <div class="flex gap-2">
                                            <button
                                                class="flex-1 px-3 py-2 border border-red-300 text-red-700 rounded text-sm hover:bg-red-50 disabled:opacity-50"
                                                disabled=move || submitting.get()
                                                on:click=move |_| handle_rate(1)
                                            >"Again"</button>
                                            <button
                                                class="flex-1 px-3 py-2 border border-yellow-300 text-yellow-700 rounded text-sm hover:bg-yellow-50 disabled:opacity-50"
                                                disabled=move || submitting.get()
                                                on:click=move |_| handle_rate(3)
                                            >"Hard"</button>
                                            <button
                                                class="flex-1 px-3 py-2 border border-green-300 text-green-700 rounded text-sm hover:bg-green-50 disabled:opacity-50"
                                                disabled=move || submitting.get()
                                                on:click=move |_| handle_rate(4)
                                            >"Good"</button>
                                            <button
                                                class="flex-1 px-3 py-2 border border-blue-300 text-blue-700 rounded text-sm hover:bg-blue-50 disabled:opacity-50"
                                                disabled=move || submitting.get()
                                                on:click=move |_| handle_rate(5)
                                            >"Easy"</button>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }}
                    </div>
                }
            })}
        </div>
    }
}
