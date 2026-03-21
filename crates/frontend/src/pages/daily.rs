//! Daily puzzle page — clean, focused layout inspired by Wordle/NYT puzzles

use leptos::prelude::*;
use leptos::task::spawn_local;
use locus_common::{DailyPuzzleResponse, DailySubmitRequest, DailySubmitResponse};

use crate::{
    api,
    components::{AnswerInput, LatexRenderer, ProblemCard},
    grader::preprocess_input,
    AuthContext,
};

#[component]
pub fn Daily() -> impl IntoView {
    let _auth = expect_context::<AuthContext>();

    // Puzzle state
    let (puzzle, set_puzzle) = signal(None::<DailyPuzzleResponse>);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);

    // Answer state
    let (answer, set_answer) = signal(String::new());
    let (submit_result, set_submit_result) = signal(None::<DailySubmitResponse>);
    let (submitting, set_submitting) = signal(false);

    // Hints state
    let (hints_revealed, set_hints_revealed) = signal(0_usize);
    let (hints_data, set_hints_data) = signal(Vec::<String>::new());

    // Load today's puzzle
    Effect::new(move |_| {
        spawn_local(async move {
            match api::get_daily_today().await {
                Ok(p) => {
                    set_puzzle.set(Some(p));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    });

    // Load hints when puzzle is available
    Effect::new(move |_| {
        if let Some(p) = puzzle.get() {
            let date = p.puzzle_date.format("%Y-%m-%d").to_string();
            spawn_local(async move {
                if let Ok(detail) = api::get_daily_puzzle(&date).await {
                    set_hints_data.set(detail.hints);
                }
            });

            if let Some(ref status) = p.user_status {
                set_hints_revealed.set(status.hints_revealed as usize);
            }
        }
    });

    let on_submit = Callback::new(move |_raw_input: String| {
        if submitting.get() {
            return;
        }
        if let Some(p) = puzzle.get() {
            let user_input = preprocess_input(&answer.get());
            if user_input.is_empty() {
                return;
            }
            set_submitting.set(true);
            let req = DailySubmitRequest {
                daily_puzzle_id: p.id,
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
        let total = hints_data.get_untracked().len();
        if current < total {
            set_hints_revealed.set(current + 1);
        }
    };

    view! {
        <div class="max-w-2xl mx-auto px-4 py-8">
            // Error
            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            // Loading
            {move || loading.get().then(|| view! {
                <div class="text-gray-400 text-sm py-16 text-center">"Loading today's puzzle..."</div>
            })}

            // Puzzle content
            {move || puzzle.get().map(|p| {
                let is_solved = submit_result.get().map(|r| r.solved).unwrap_or(
                    p.user_status.as_ref().map(|s| s.solved).unwrap_or(false)
                );
                let was_already_solved = p.user_status.as_ref().map(|s| s.solved).unwrap_or(false);
                let same_day = p.user_status.as_ref().map(|s| s.solved_same_day).unwrap_or(false);
                let streak = p.user_status.as_ref().map(|s| s.streak).unwrap_or(0);
                let problem = p.problem.clone();
                let answer_type = problem.answer_type;
                let puzzle_date = p.puzzle_date.format("%B %-d, %Y").to_string();

                view! {
                    // ── Header ──────────────────────────────────────────────
                    <div class="flex items-start justify-between mb-8">
                        <div>
                            <p class="text-sm text-gray-500 dark:text-gray-400">{puzzle_date}</p>
                            {if !p.title.is_empty() {
                                view! { <h1 class="text-2xl font-bold tracking-tight mt-0.5">{p.title.clone()}</h1> }.into_any()
                            } else {
                                view! { <h1 class="text-2xl font-bold tracking-tight mt-0.5">"Daily Puzzle"</h1> }.into_any()
                            }}
                            {(!p.source.is_empty()).then(|| view! {
                                <p class="text-sm text-gray-500 dark:text-gray-400 mt-0.5 italic">{p.source.clone()}</p>
                            })}
                        </div>

                        // Status badges
                        <div class="flex items-center gap-2">
                            {(was_already_solved).then(|| {
                                let label = if same_day { "Solved" } else { "Previously solved" };
                                view! {
                                    <span class="text-xs font-medium px-2.5 py-1 bg-green-50 dark:bg-green-950/40 text-green-600 dark:text-green-400 border border-green-200 dark:border-green-800 rounded-full">
                                        {label}
                                    </span>
                                }
                            })}
                            {(streak > 0).then(|| view! {
                                <div class="flex items-center gap-1 px-2.5 py-1 bg-amber-50 dark:bg-amber-950/40 border border-amber-200 dark:border-amber-800 rounded-full">
                                    <svg class="w-3.5 h-3.5 text-amber-500" fill="currentColor" viewBox="0 0 20 20">
                                        <path d="M12.395 2.553a1 1 0 00-1.45-.385c-.345.23-.614.558-.822.88-.214.33-.403.713-.57 1.116-.334.804-.614 1.768-.84 2.734a31.365 31.365 0 00-.613 3.58 2.64 2.64 0 01-.945-1.067c-.328-.68-.398-1.534-.398-2.654A1 1 0 005.05 6.05 6.981 6.981 0 003 11a7 7 0 1011.95-4.95c-.592-.591-.98-.985-1.348-1.467-.363-.476-.724-1.063-1.207-2.03zM12.12 15.12A3 3 0 017 13s.879.5 2.5.5c0-1 .5-4 1.25-4.5.5 1 .786 1.293 1.371 1.879A2.99 2.99 0 0113 13a2.99 2.99 0 01-.879 2.121z"></path>
                                    </svg>
                                    <span class="text-xs font-bold text-amber-700 dark:text-amber-400">{streak}</span>
                                </div>
                            })}
                        </div>
                    </div>

                    // ── Problem card ────────────────────────────────────────
                    <div class="mb-6">
                        <ProblemCard problem=problem.clone() />
                    </div>

                    // ── Hints ───────────────────────────────────────────────
                    {move || {
                        let hints = hints_data.get();
                        let revealed = hints_revealed.get();
                        let total = hints.len();
                        if total == 0 {
                            return None;
                        }
                        Some(view! {
                            <div class="mb-6">
                                // Revealed hints
                                {(revealed > 0).then(|| view! {
                                    <div class="space-y-2 mb-2">
                                        {(0..revealed).map(|i| {
                                            let hint = hints[i].clone();
                                            view! {
                                                <div class="p-3 bg-amber-50 dark:bg-amber-950/30 border border-amber-200 dark:border-amber-800 rounded-lg text-sm">
                                                    <span class="font-medium text-amber-700 dark:text-amber-400">{format!("Hint {} ", i + 1)}</span>
                                                    <LatexRenderer content=hint />
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                })}

                                // Reveal button
                                {(!is_solved && revealed < total).then(|| view! {
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

                    // ── Answer input + submit (always visible) ───────────
                    {
                        let on_sub = on_submit.clone();
                        view! {
                            <div class="mb-6">
                                // Result feedback
                                {move || submit_result.get().map(|r| {
                                    if r.is_correct {
                                        view! {
                                            <div class="mb-4 p-4 bg-green-50 dark:bg-green-950/30 border border-green-200 dark:border-green-800 rounded-lg text-center">
                                                <p class="text-green-700 dark:text-green-400 font-semibold text-lg">"Correct!"</p>
                                                <p class="text-sm text-green-600 dark:text-green-500 mt-1">
                                                    {format!("Solved in {} attempt{}.", r.attempts, if r.attempts == 1 { "" } else { "s" })}
                                                    {(r.streak > 1).then(|| format!(" {} day streak!", r.streak))}
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
                                                disabled=submitting.get_untracked() || is_solved
                                            />
                                            <button
                                                class="w-full px-4 py-2.5 bg-gray-900 dark:bg-gray-100 text-white dark:text-gray-900 rounded-lg font-medium hover:bg-gray-800 dark:hover:bg-gray-200 disabled:opacity-50 transition-colors"
                                                on:click=move |_| on_submit.run(answer.get_untracked())
                                                disabled=move || answer.get().is_empty() || submitting.get() || is_solved
                                            >
                                                {move || if submitting.get() { "Checking..." } else { "Submit Answer" }}
                                                </button>
                                            </div>
                                        }
                                    }
                            </div>
                        }
                    }

                    // ── Footer link ─────────────────────────────────────────
                    <div class="text-center pt-4 border-t border-gray-100 dark:border-gray-800">
                        <a href="/daily/archive" class="text-sm text-gray-400 dark:text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 transition-colors">
                            "View past puzzles"
                        </a>
                    </div>
                }
            })}
        </div>
    }
}
