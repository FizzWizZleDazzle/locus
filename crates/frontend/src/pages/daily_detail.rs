//! Daily puzzle detail page — lets users attempt any puzzle (past or present),
//! with answer/editorial revealed only after solving or explicitly requesting.

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::{hooks::use_params, params::Params};
use locus_common::{DailyPuzzleDetailResponse, DailySubmitRequest, DailySubmitResponse};

use crate::{
    api,
    components::{AnswerInput, LatexRenderer, ProblemCard},
    grader::preprocess_input,
    AuthContext,
};

#[derive(Params, PartialEq, Clone, Debug)]
struct DailyPuzzleParams {
    date: Option<String>,
}

#[component]
pub fn DailyPuzzleDetail() -> impl IntoView {
    let _auth = expect_context::<AuthContext>();
    let params = use_params::<DailyPuzzleParams>();

    let (detail, set_detail) = signal(None::<DailyPuzzleDetailResponse>);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);

    // Answer/interaction state
    let (answer, set_answer) = signal(String::new());
    let (submit_result, set_submit_result) = signal(None::<DailySubmitResponse>);
    let (submitting, set_submitting) = signal(false);
    let (hints_revealed, set_hints_revealed) = signal(0_usize);
    // User explicitly chose to reveal the answer (give up)
    let (show_answer, set_show_answer) = signal(false);

    // Load puzzle detail
    Effect::new(move |_| {
        let date = params
            .get()
            .ok()
            .and_then(|p| p.date)
            .unwrap_or_default();
        if date.is_empty() {
            set_error.set(Some("No date specified".into()));
            set_loading.set(false);
            return;
        }
        spawn_local(async move {
            match api::get_daily_puzzle(&date).await {
                Ok(d) => {
                    if let Some(ref status) = d.user_status {
                        set_hints_revealed.set(status.hints_revealed as usize);
                    }
                    set_detail.set(Some(d));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    });

    let on_submit = Callback::new(move |_raw_input: String| {
        if submitting.get() {
            return;
        }
        if let Some(d) = detail.get() {
            let user_input = preprocess_input(&answer.get());
            if user_input.is_empty() {
                return;
            }
            set_submitting.set(true);
            let req = DailySubmitRequest {
                daily_puzzle_id: d.id,
                user_input,
                hints_used: hints_revealed.get_untracked() as i32,
                time_taken_ms: None,
            };
            spawn_local(async move {
                match api::submit_daily(&req).await {
                    Ok(resp) => {
                        set_submit_result.set(Some(resp));
                        set_submitting.set(false);
                    }
                    Err(e) => {
                        set_error.set(Some(e.message));
                        set_submitting.set(false);
                    }
                }
            });
        }
    });

    let on_reveal_hint = move |_| {
        let current = hints_revealed.get_untracked();
        if let Some(d) = detail.get_untracked() {
            if current < d.hints.len() {
                set_hints_revealed.set(current + 1);
            }
        }
    };

    view! {
        <div class="max-w-2xl mx-auto px-4 py-8">
            // Back nav
            <a href="/daily/archive" class="inline-flex items-center gap-1.5 text-sm text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 transition-colors mb-8">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                </svg>
                "Archive"
            </a>

            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            {move || loading.get().then(|| view! {
                <div class="text-gray-400 text-sm py-8 text-center">"Loading puzzle..."</div>
            })}

            {move || detail.get().map(|d| {
                let puzzle_date = d.puzzle_date.format("%B %-d, %Y").to_string();
                // Whether user has solved this puzzle (either previously or just now)
                let previously_solved = d.user_status.as_ref().map(|s| s.solved).unwrap_or(false);
                let solved_same_day = d.user_status.as_ref().map(|s| s.solved_same_day).unwrap_or(false);
                let just_solved = submit_result.get().map(|r| r.solved).unwrap_or(false);
                // Answer/editorial visible only when solved THIS session or user clicked "Show answer"
                let answer_visible = just_solved || show_answer.get();

                let problem = d.problem.clone();
                let answer_type = problem.answer_type;
                let has_editorial = !d.editorial_latex.is_empty();
                let editorial = d.editorial_latex.clone();
                let hints = d.hints.clone();
                let hints2 = hints.clone();
                let stats = d.stats.clone();
                let answer_key = d.problem.answer_key.clone();
                let is_past = answer_key.is_some();

                view! {
                    // ── Header ──────────────────────────────────────────
                    <div class="flex items-start justify-between mb-8">
                        <div>
                            <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">{puzzle_date}</p>
                            {if !d.title.is_empty() {
                                view! { <h1 class="text-2xl font-bold tracking-tight">{d.title.clone()}</h1> }.into_any()
                            } else {
                                view! { <h1 class="text-2xl font-bold tracking-tight">"Daily Puzzle"</h1> }.into_any()
                            }}
                            {(!d.source.is_empty()).then(|| view! {
                                <p class="text-sm text-gray-500 dark:text-gray-400 mt-0.5 italic">{d.source.clone()}</p>
                            })}
                        </div>
                        {previously_solved.then(|| {
                            let label = if solved_same_day { "Solved" } else { "Solved late" };
                            let colors = if solved_same_day {
                                "bg-green-50 dark:bg-green-950/40 text-green-600 dark:text-green-400 border-green-200 dark:border-green-800"
                            } else {
                                "bg-yellow-50 dark:bg-yellow-950/40 text-yellow-600 dark:text-yellow-400 border-yellow-200 dark:border-yellow-800"
                            };
                            view! {
                                <span class=format!("text-xs font-medium px-2.5 py-1 border rounded-full {}", colors)>
                                    {label}
                                </span>
                            }
                        })}
                    </div>

                    // ── Stats ────────────────────────────────────────────
                    <div class="flex items-center gap-6 text-sm text-gray-500 dark:text-gray-400 pb-6 mb-6 border-b border-gray-200 dark:border-gray-700">
                        <div>
                            <span class="font-semibold text-gray-900 dark:text-gray-100">{format!("{:.0}%", stats.solve_rate * 100.0)}</span>
                            " solve rate"
                        </div>
                        <div>
                            <span class="font-semibold text-gray-900 dark:text-gray-100">{stats.total_solves}</span>
                            " solves"
                        </div>
                        <div>
                            <span class="font-semibold text-gray-900 dark:text-gray-100">{stats.total_attempts}</span>
                            " attempts"
                        </div>
                    </div>

                    // ── Problem ──────────────────────────────────────────
                    <div class="mb-6">
                        <ProblemCard problem=problem.clone() />
                    </div>

                    // ── Hints (progressive reveal) ──────────────────────
                    {move || {
                        let h = hints.clone();
                        let revealed = hints_revealed.get();
                        let total = h.len();
                        if total == 0 {
                            return None;
                        }
                        Some(view! {
                            <div class="mb-6">
                                {(revealed > 0).then(|| view! {
                                    <div class="space-y-2 mb-2">
                                        {(0..revealed).map(|i| {
                                            let hint = h[i].clone();
                                            view! {
                                                <div class="p-3 bg-amber-50 dark:bg-amber-950/30 border border-amber-200 dark:border-amber-800 rounded-lg text-sm">
                                                    <span class="font-medium text-amber-700 dark:text-amber-400">{format!("Hint {} ", i + 1)}</span>
                                                    <LatexRenderer content=hint />
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                })}

                                {(!answer_visible && revealed < total).then(|| view! {
                                    <button
                                        class="text-sm text-amber-600 dark:text-amber-400 hover:text-amber-800 dark:hover:text-amber-300 transition-colors"
                                        on:click=on_reveal_hint
                                    >
                                        {if revealed == 0 {
                                            format!("Need a hint? ({} available)", total)
                                        } else {
                                            format!("Next hint ({}/{})", revealed, total)
                                        }}
                                    </button>
                                })}
                            </div>
                        })
                    }}

                    // ── Answer input + submit (always visible) ──────────
                    {
                        let on_sub = on_submit.clone();
                        view! {
                            <div class="mb-6">
                                // Feedback from current session
                                {move || submit_result.get().map(|r| {
                                    if r.is_correct {
                                        view! {
                                            <div class="mb-4 p-4 bg-green-50 dark:bg-green-950/30 border border-green-200 dark:border-green-800 rounded-lg text-center">
                                                <p class="text-green-700 dark:text-green-400 font-semibold text-lg">"Correct!"</p>
                                                <p class="text-sm text-green-600 dark:text-green-500 mt-1">
                                                    {format!("Solved in {} attempt{}.", r.attempts, if r.attempts == 1 { "" } else { "s" })}
                                                </p>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="mb-4 p-3 bg-red-50 dark:bg-red-950/30 border border-red-200 dark:border-red-800 rounded-lg">
                                                <span class="text-red-700 dark:text-red-400 text-sm">
                                                    {format!("Incorrect — try again ({} attempt{})", r.attempts, if r.attempts == 1 { "" } else { "s" })}
                                                </span>
                                            </div>
                                        }.into_any()
                                    }
                                })}

                                // Input + button
                                {
                                    let on_sub2 = on_sub.clone();
                                    view! {
                                        <div class="space-y-3">
                                            <AnswerInput
                                                answer_type=answer_type
                                                value=answer
                                                set_value=set_answer
                                                on_submit=on_sub2
                                                disabled=submitting.get_untracked()
                                            />
                                            <button
                                                class="w-full px-4 py-2.5 bg-gray-900 dark:bg-gray-100 text-white dark:text-gray-900 rounded-lg font-medium hover:bg-gray-800 dark:hover:bg-gray-200 disabled:opacity-50 transition-colors"
                                                on:click=move |_| on_submit.run(answer.get_untracked())
                                                disabled=move || answer.get().is_empty() || submitting.get()
                                            >
                                                {move || if submitting.get() { "Checking..." } else { "Submit Answer" }}
                                            </button>
                                        </div>
                                    }
                                }

                                // "Show answer" for past puzzles (only when answer not yet visible)
                                {(!answer_visible && is_past).then(|| view! {
                                    <div class="mt-4 text-center">
                                        <button
                                            class="text-sm text-gray-400 dark:text-gray-500 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
                                            on:click=move |_| set_show_answer.set(true)
                                        >
                                            "Show answer"
                                        </button>
                                    </div>
                                })}
                            </div>
                        }
                    }

                    // ── Answer + editorial (shown after solve or "show answer") ──
                    {answer_visible.then(|| {
                        view! {
                            <div class="space-y-4 mb-6">
                                // Answer
                                {answer_key.as_ref().map(|key| view! {
                                    <div class="p-4 bg-gray-50 dark:bg-gray-800/50 border border-gray-200 dark:border-gray-700 rounded-lg">
                                        <p class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-2">"Answer"</p>
                                        <div class="text-lg">
                                            <LatexRenderer content=key.clone() />
                                        </div>
                                    </div>
                                })}

                                // User status badge
                                {d.user_status.as_ref().filter(|s| s.attempts > 0).map(|s| {
                                    let (label, bg) = if s.solved_same_day {
                                        ("Solved on the day", "bg-green-50 dark:bg-green-950/30 border-green-200 dark:border-green-800 text-green-700 dark:text-green-400")
                                    } else if s.solved {
                                        ("Solved later", "bg-yellow-50 dark:bg-yellow-950/30 border-yellow-200 dark:border-yellow-800 text-yellow-700 dark:text-yellow-400")
                                    } else {
                                        ("Not solved", "bg-gray-50 dark:bg-gray-800/50 border-gray-200 dark:border-gray-700 text-gray-500 dark:text-gray-400")
                                    };
                                    view! {
                                        <div class=format!("p-3 border rounded-lg text-sm font-medium {}", bg)>
                                            {label}
                                            {format!(" ({} attempt{})", s.attempts, if s.attempts == 1 { "" } else { "s" })}
                                        </div>
                                    }
                                })}

                                // Editorial
                                {has_editorial.then(|| view! {
                                    <div class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden">
                                        <details>
                                            <summary class="cursor-pointer px-4 py-3 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors select-none">
                                                "Solution & Editorial"
                                            </summary>
                                            <div class="px-4 pb-4 text-sm leading-relaxed border-t border-gray-200 dark:border-gray-700 pt-3">
                                                <LatexRenderer content=editorial.clone() />
                                            </div>
                                        </details>
                                    </div>
                                })}

                                // Show all hints when answer is visible
                                {move || {
                                    let h = hints2.clone();
                                    let total = h.len();
                                    (total > 0).then(|| view! {
                                        <div class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden">
                                            <details>
                                                <summary class="cursor-pointer px-4 py-3 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors select-none">
                                                    {format!("Hints ({})", total)}
                                                </summary>
                                                <div class="px-4 pb-4 space-y-2 border-t border-gray-200 dark:border-gray-700 pt-3">
                                                    {(0..total).map(|i| {
                                                        let hint = h[i].clone();
                                                        view! {
                                                            <div class="p-3 bg-amber-50 dark:bg-amber-950/30 border border-amber-200 dark:border-amber-800 rounded text-sm">
                                                                <span class="font-medium text-amber-700 dark:text-amber-400">{format!("{}. ", i + 1)}</span>
                                                                <LatexRenderer content=hint />
                                                            </div>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            </details>
                                        </div>
                                    })
                                }}
                            </div>
                        }
                    })}
                }
            })}
        </div>
    }
}
