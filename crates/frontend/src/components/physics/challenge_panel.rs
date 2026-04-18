//! Challenge panel — orchestrates the Predict-Test-Reflect flow.
//!
//! Displays the active challenge stage and manages progression through
//! stages.  The simulation is locked until the Prediction stage is completed.

use leptos::prelude::*;

use locus_physics_common::challenge::{ChallengeStage, StageData, WhatIfPrompt};

use super::{
    EquationBuilder, FbdBuilder, PredictionInput, QuantitySelector, ReflectionPanel,
    WhatIfExplorer,
};

#[component]
pub fn ChallengePanel(
    /// The challenge stages for the current problem.
    stages: Vec<ChallengeStage>,
    /// Post-solve exploration prompts.
    what_if_prompts: Vec<WhatIfPrompt>,
    /// Fired when the sim should be unlocked (prediction committed).
    on_unlock_sim: Callback<()>,
    /// Fired when a hint is used.
    on_hint_used: Callback<()>,
    /// Tracks total hints used.
    hints_used: ReadSignal<i32>,
    /// Tracks FBD attempts.
    fbd_attempts: WriteSignal<i32>,
    /// Tracks stages completed.
    stages_completed: WriteSignal<i32>,
    /// Tracks what-ifs explored.
    what_ifs_explored: WriteSignal<i32>,
) -> impl IntoView {
    let (active_stage, set_active_stage) = signal(0usize);
    let (show_hint, set_show_hint) = signal(false);
    let (all_complete, set_all_complete) = signal(false);

    let total_stages = stages.len();
    let stages_for_nav = stages.clone();
    let stages_for_render = stages.clone();
    let what_ifs = what_if_prompts.clone();

    let advance_stage = move || {
        let current = active_stage.get_untracked();
        stages_completed.set((current + 1) as i32);
        if current + 1 < total_stages {
            set_active_stage.set(current + 1);
            set_show_hint.set(false);
        } else {
            set_all_complete.set(true);
        }
    };

    view! {
        <div class="flex flex-col h-full">
            // Stage navigator (horizontal dots)
            <div class="flex items-center gap-1.5 mb-4 pb-3 border-b border-gray-200 dark:border-gray-700">
                {stages_for_nav.iter().enumerate().map(|(i, stage)| {
                    let title = stage.title.clone();
                    view! {
                        <button
                            class=move || format!(
                                "flex items-center gap-1 px-2 py-1 rounded text-xs transition-colors {}",
                                if i == active_stage.get() {
                                    "bg-blue-100 text-blue-700 dark:bg-blue-900/40 dark:text-blue-400 font-semibold"
                                } else if i < active_stage.get() {
                                    "text-green-600 dark:text-green-400"
                                } else {
                                    "text-gray-400"
                                }
                            )
                            on:click=move |_| {
                                if i <= active_stage.get_untracked() {
                                    set_active_stage.set(i);
                                }
                            }
                        >
                            <span class=move || format!(
                                "w-5 h-5 flex items-center justify-center rounded-full text-[10px] font-bold {}",
                                if i < active_stage.get() {
                                    "bg-green-500 text-white"
                                } else if i == active_stage.get() {
                                    "bg-blue-500 text-white"
                                } else {
                                    "bg-gray-200 dark:bg-gray-600 text-gray-500"
                                }
                            )>
                                {move || if i < active_stage.get() {
                                    "\u{2713}".to_string()
                                } else {
                                    format!("{}", i + 1)
                                }}
                            </span>
                            <span class="hidden sm:inline">{title.clone()}</span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Active stage content
            <div class="flex-1 overflow-y-auto">
                {move || {
                    if all_complete.get() {
                        return view! {
                            <div class="space-y-4">
                                <div class="flex items-center gap-2 px-3 py-2 bg-green-50 dark:bg-green-900/20 rounded-lg text-green-700 dark:text-green-400">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                    </svg>
                                    <span class="text-sm font-medium">"All stages complete! Now submit your answer."</span>
                                </div>

                                // What-if exploration
                                {(!what_ifs.is_empty()).then(|| {
                                    let prompts = what_ifs.clone();
                                    view! {
                                        <WhatIfExplorer
                                            prompts=prompts
                                            on_explore=Callback::new(move |_idx| {
                                                what_ifs_explored.update(|n| *n += 1);
                                            })
                                        />
                                    }
                                })}
                            </div>
                        }.into_any();
                    }

                    let idx = active_stage.get();
                    if idx >= stages_for_render.len() {
                        return view! { <p>"No stages available."</p> }.into_any();
                    }

                    let stage = stages_for_render[idx].clone();
                    let prompt = stage.prompt_text.clone();
                    let hint = stage.hint_text.clone();

                    let on_advance = {
                        let advance = advance_stage;
                        Callback::new(move |_| advance())
                    };
                    let on_fbd_done = {
                        let advance = advance_stage;
                        Callback::new(move |n: i32| {
                            fbd_attempts.set(n);
                            advance();
                        })
                    };
                    let on_predict = Callback::new(move |_: (f64, bool)| {
                        on_unlock_sim.run(());
                    });
                    let on_reflect = {
                        let advance = advance_stage;
                        Callback::new(move |_: String| advance())
                    };

                    let widget = match stage.stage_data.clone() {
                        StageData::IdentifyQuantities { correct, distractors, explanations } => {
                            view! {
                                <QuantitySelector
                                    correct=correct
                                    distractors=distractors
                                    explanations=explanations
                                    on_complete=on_advance
                                />
                            }.into_any()
                        }
                        StageData::FreebodyDiagram { target_body, expected_forces, direction_tolerance_deg, per_force_hints } => {
                            view! {
                                <FbdBuilder
                                    target_body=target_body
                                    expected_forces=expected_forces
                                    tolerance_deg=direction_tolerance_deg
                                    per_force_hints=per_force_hints
                                    on_complete=on_fbd_done
                                />
                            }.into_any()
                        }
                        StageData::EquationBuilder { axis_label, correct_terms, available_terms, error_feedback } => {
                            view! {
                                <EquationBuilder
                                    axis_label=axis_label
                                    correct_terms=correct_terms
                                    available_terms=available_terms
                                    error_feedback=error_feedback
                                    on_complete=on_advance
                                />
                            }.into_any()
                        }
                        StageData::Prediction { question, answer, unit, tolerance_pct, .. } => {
                            view! {
                                <PredictionInput
                                    question=question
                                    unit=unit
                                    answer=answer
                                    tolerance_pct=tolerance_pct
                                    on_predict=on_predict
                                    on_complete=on_advance
                                />
                            }.into_any()
                        }
                        StageData::Reflection { diagnostic_options, micro_lessons, .. } => {
                            view! {
                                <ReflectionPanel
                                    options=diagnostic_options
                                    micro_lessons=micro_lessons
                                    on_complete=on_reflect
                                />
                            }.into_any()
                        }
                    };

                    view! {
                        <div class="space-y-3">
                            <p class="text-sm text-gray-700 dark:text-gray-300">{prompt}</p>

                            {hint.map(|h| {
                                let hint_text = h.clone();
                                view! {
                                    <div>
                                        {move || if show_hint.get() {
                                            view! {
                                                <div class="px-3 py-2 bg-amber-50 dark:bg-amber-900/20 rounded-lg text-amber-700 dark:text-amber-400 text-xs border border-amber-200 dark:border-amber-800">
                                                    <span class="font-medium">"Hint: "</span>
                                                    {hint_text.clone()}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <button
                                                    class="text-xs text-amber-600 hover:underline flex items-center gap-1"
                                                    on:click=move |_| {
                                                        set_show_hint.set(true);
                                                        on_hint_used.run(());
                                                    }
                                                >
                                                    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"></path>
                                                    </svg>
                                                    {move || format!("Show hint ({} used)", hints_used.get())}
                                                </button>
                                            }.into_any()
                                        }}
                                    </div>
                                }
                            })}

                            <div class="min-h-[100px]">{widget}</div>
                        </div>
                    }.into_any()
                }}
            </div>

            // Progress bar
            <div class="mt-3 pt-3 border-t border-gray-200 dark:border-gray-700">
                <div class="flex items-center justify-between text-xs text-gray-500 mb-1">
                    <span>{move || format!("Stage {} of {}", active_stage.get() + 1, total_stages)}</span>
                    <span>{move || format!("Hints used: {}", hints_used.get())}</span>
                </div>
                <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-1.5">
                    <div
                        class="bg-blue-500 h-1.5 rounded-full transition-all duration-300"
                        style=move || format!(
                            "width: {}%",
                            if total_stages == 0 { 0 } else { ((active_stage.get() + 1) * 100) / total_stages }
                        )
                    ></div>
                </div>
            </div>
        </div>
    }
}
