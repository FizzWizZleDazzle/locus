//! Wrong answer review page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use locus_common::{ProblemResponse, WrongAnswerReviewItem};

use crate::components::{LatexRenderer, ProblemCard};
use crate::katex_bindings::render_plain_math_to_string;
use crate::utils::escape_html;
use crate::{AuthContext, api};

#[component]
pub fn WrongAnswers() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = leptos_router::hooks::use_navigate();

    let navigate_clone = navigate.clone();
    Effect::new(move |_| {
        if !auth.is_logged_in.get() {
            navigate_clone("/login", Default::default());
        }
    });

    let (items, set_items) = signal(Vec::<WrongAnswerReviewItem>::new());
    let (total, set_total) = signal(0i64);
    let (loading, set_loading) = signal(true);
    let (offset, set_offset) = signal(0i64);
    let (topic_filter, set_topic_filter) = signal(None::<String>);
    let (expanded, set_expanded) = signal(None::<uuid::Uuid>);

    let load = move || {
        set_loading.set(true);
        let topic = topic_filter.get_untracked();
        let off = offset.get_untracked();
        spawn_local(async move {
            match api::get_wrong_answers(topic.as_deref(), 20, off).await {
                Ok(resp) => {
                    set_items.set(resp.items);
                    set_total.set(resp.total);
                }
                Err(_) => {}
            }
            set_loading.set(false);
        });
    };

    Effect::new(move |_| {
        let _ = offset.get();
        let _ = topic_filter.get();
        load();
    });

    let topics = locus_common::MainTopic::all();
    let topic_options: Vec<(String, String)> = topics
        .iter()
        .filter(|t| !matches!(t, locus_common::MainTopic::Test))
        .map(|t| (t.as_str().to_string(), t.display_name().to_string()))
        .collect();

    view! {
        <div class="max-w-4xl mx-auto px-4 py-8">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-2xl font-semibold">"Wrong Answers"</h1>
                <A href="/stats" attr:class="text-sm text-blue-600 hover:underline">"Back to Stats"</A>
            </div>

            <div class="mb-4">
                <select
                    class="border border-gray-300 rounded px-3 py-2 text-sm"
                    on:change=move |ev| {
                        let val = event_target_value(&ev);
                        set_offset.set(0);
                        if val.is_empty() {
                            set_topic_filter.set(None);
                        } else {
                            set_topic_filter.set(Some(val));
                        }
                    }
                >
                    <option value="">"All Topics"</option>
                    {topic_options.into_iter().map(|(id, name)| view! {
                        <option value=id>{name}</option>
                    }).collect_view()}
                </select>
            </div>

            {move || loading.get().then(|| view! {
                <div class="text-gray-500 text-sm">"Loading..."</div>
            })}

            {move || {
                let current_items = items.get();
                if !loading.get() && current_items.is_empty() {
                    Some(view! {
                        <div class="text-center py-12 text-gray-500">
                            <p class="text-lg mb-2">"No wrong answers found!"</p>
                            <p class="text-sm">"Keep practicing to improve your skills."</p>
                        </div>
                    })
                } else {
                    None
                }
            }}

            <div class="space-y-4">
                {move || items.get().into_iter().map(|item| {
                    let item_id = item.attempt_id;
                    let is_expanded = move || expanded.get() == Some(item_id);
                    let topic_id = item.main_topic.clone();
                    let date_str = item.attempted_at.format("%b %d, %Y").to_string();
                    let solution = item.solution_latex.clone();

                    // Render user's wrong answer and correct answer as math
                    let user_rendered = render_plain_math_to_string(&item.user_input)
                        .unwrap_or_else(|_| format!("<code>{}</code>", escape_html(&item.user_input)));
                    let correct_rendered = render_plain_math_to_string(&item.answer_key)
                        .unwrap_or_else(|_| format!("<code>{}</code>", escape_html(&item.answer_key)));

                    // Build a ProblemResponse for ProblemCard
                    let prob = ProblemResponse {
                        id: item.problem_id,
                        question_latex: item.question_latex.clone(),
                        difficulty: item.difficulty,
                        main_topic: item.main_topic.clone(),
                        subtopic: item.subtopic.clone(),
                        grading_mode: locus_common::GradingMode::Equivalent,
                        answer_type: locus_common::AnswerType::Expression,
                        calculator_allowed: String::new(),
                        answer_key: None,
                        solution_latex: String::new(),
                        question_image: String::new(),
                        time_limit_seconds: None,
                    };

                    view! {
                        <div class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden">
                            <ProblemCard problem=prob />

                            <div class="px-6 pb-4 space-y-3">
                                <div class="flex items-center gap-1 text-xs text-gray-400">
                                    <span>{date_str}</span>
                                </div>

                                <div class="flex gap-6 text-sm">
                                    <div>
                                        <span class="text-gray-500">"Your answer: "</span>
                                        <span class="text-red-600" inner_html=user_rendered></span>
                                    </div>
                                    <div>
                                        <span class="text-gray-500">"Correct: "</span>
                                        <span class="text-green-600" inner_html=correct_rendered></span>
                                    </div>
                                </div>

                                <div class="flex gap-2">
                                    {if !solution.is_empty() {
                                        view! {
                                            <button
                                                class="text-xs text-blue-600 hover:underline"
                                                on:click=move |_| {
                                                    if is_expanded() {
                                                        set_expanded.set(None);
                                                    } else {
                                                        set_expanded.set(Some(item_id));
                                                    }
                                                }
                                            >
                                                {move || if is_expanded() { "Hide Solution" } else { "Show Solution" }}
                                            </button>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }}
                                    <A
                                        href=format!("/practice?main_topic={}", topic_id)
                                        attr:class="text-xs text-blue-600 hover:underline"
                                    >"Practice This Topic"</A>
                                </div>

                                {move || is_expanded().then(|| {
                                    let sol = solution.clone();
                                    view! {
                                        <div class="mt-3 pt-3 border-t border-gray-100 dark:border-gray-700">
                                            <LatexRenderer content=sol />
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            // Pagination
            {move || {
                let t = total.get();
                let off = offset.get();
                if t > 20 {
                    Some(view! {
                        <div class="flex justify-between items-center mt-6">
                            <button
                                class="px-4 py-2 text-sm border rounded disabled:opacity-50"
                                disabled=move || off == 0
                                on:click=move |_| set_offset.set((off - 20).max(0))
                            >"Previous"</button>
                            <span class="text-sm text-gray-500">{format!("Showing {}-{} of {}", off + 1, (off + 20).min(t), t)}</span>
                            <button
                                class="px-4 py-2 text-sm border rounded disabled:opacity-50"
                                disabled=move || off + 20 >= t
                                on:click=move |_| set_offset.set(off + 20)
                            >"Next"</button>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
