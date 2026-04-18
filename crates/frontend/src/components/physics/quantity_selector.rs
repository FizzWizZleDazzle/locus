//! Phase 1: Identify which physical quantities matter.
//!
//! Students select checkboxes from a list of quantities. Some are relevant,
//! some are distractors. Immediate per-item feedback is given on submit.

use leptos::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::components::LatexRenderer;
use locus_physics_common::challenge::QuantityOption;

#[component]
pub fn QuantitySelector(
    /// The correct quantities for this problem.
    correct: Vec<QuantityOption>,
    /// Plausible distractors.
    distractors: Vec<QuantityOption>,
    /// Per-quantity explanations.
    explanations: HashMap<String, String>,
    /// Fired when the student answers this stage correctly.
    on_complete: Callback<()>,
) -> impl IntoView {
    let (selected, set_selected) = signal(HashSet::<String>::new());
    let (submitted, set_submitted) = signal(false);
    let (all_correct_sig, set_all_correct_sig) = signal(false);
    let (feedback, set_feedback) = signal(Vec::<(String, bool, String)>::new());

    let correct_ids: HashSet<String> = correct.iter().map(|q| q.id.clone()).collect();

    // Merge and shuffle (deterministic order for now)
    let mut all_options: Vec<QuantityOption> = correct.clone();
    all_options.extend(distractors.clone());
    all_options.sort_by(|a, b| a.label.cmp(&b.label));

    let all_options_for_view = all_options.clone();
    let explanations_clone = explanations.clone();
    let correct_ids_clone = correct_ids.clone();

    let on_submit = move |_| {
        let sel = selected.get();
        let mut fb = Vec::new();
        let mut all_correct = true;

        // Check each correct item is selected
        for q in &correct {
            let is_selected = sel.contains(&q.id);
            let expl = explanations_clone
                .get(&q.id)
                .cloned()
                .unwrap_or_default();
            if !is_selected {
                all_correct = false;
            }
            fb.push((q.id.clone(), is_selected, expl));
        }

        // Check no distractors are selected
        for q in &distractors {
            let is_selected = sel.contains(&q.id);
            let expl = explanations_clone
                .get(&q.id)
                .cloned()
                .unwrap_or_default();
            if is_selected {
                all_correct = false;
            }
            fb.push((q.id.clone(), !is_selected, expl));
        }

        set_feedback.set(fb);
        set_submitted.set(true);
        set_all_correct_sig.set(all_correct);

        if all_correct {
            on_complete.run(());
        }
    };

    let on_retry = move |_| {
        set_submitted.set(false);
        set_feedback.set(Vec::new());
        set_all_correct_sig.set(false);
    };

    view! {
        <div class="space-y-3">
            <div class="grid grid-cols-2 gap-2">
                {all_options_for_view.into_iter().map(|q| {
                    let qid = q.id.clone();
                    let qid2 = q.id.clone();
                    let qid3 = q.id.clone();
                    let qid_toggle = q.id.clone();
                    let is_correct_item = correct_ids_clone.contains(&q.id);
                    view! {
                        <label
                            class=move || {
                                let base = "flex items-center gap-2 p-2 rounded-lg border cursor-pointer transition-colors";
                                let sel = selected.get().contains(&qid);
                                if submitted.get() {
                                    let was_selected = selected.get().contains(&qid2);
                                    if (is_correct_item && was_selected) || (!is_correct_item && !was_selected) {
                                        format!("{} border-green-300 bg-green-50 dark:bg-green-900/20", base)
                                    } else {
                                        format!("{} border-red-300 bg-red-50 dark:bg-red-900/20", base)
                                    }
                                } else if sel {
                                    format!("{} border-blue-400 bg-blue-50 dark:bg-blue-900/30", base)
                                } else {
                                    format!("{} border-gray-200 hover:border-gray-300 dark:border-gray-600", base)
                                }
                            }
                        >
                            <input
                                type="checkbox"
                                class="rounded"
                                prop:checked=move || selected.get().contains(&qid3)
                                on:change=move |_| {
                                    set_selected.update(|s| {
                                        if s.contains(&qid_toggle) {
                                            s.remove(&qid_toggle);
                                        } else {
                                            s.insert(qid_toggle.clone());
                                        }
                                    });
                                }
                                disabled=move || submitted.get() && all_correct_sig.get()
                            />
                            <span class="text-sm">{q.label}</span>
                            {(!q.symbol_latex.is_empty()).then(|| view! {
                                <span class="text-xs text-gray-500 ml-auto">
                                    <LatexRenderer content=format!("${}$", q.symbol_latex) />
                                </span>
                            })}
                        </label>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Submit / retry button
            {move || if !submitted.get() {
                view! {
                    <button
                        class="w-full py-2 px-4 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-medium text-sm"
                        on:click=on_submit.clone()
                    >
                        "Check my selections"
                    </button>
                }.into_any()
            } else if !all_correct_sig.get() {
                view! {
                    <button
                        class="w-full py-2 px-4 bg-amber-600 text-white rounded-lg hover:bg-amber-700 transition-colors font-medium text-sm"
                        on:click=on_retry.clone()
                    >
                        "Try again"
                    </button>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Feedback
            {move || submitted.get().then(|| {
                let fb = feedback.get();
                view! {
                    <div class="space-y-1.5 mt-2">
                        {fb.into_iter().map(|(id, is_ok, explanation)| {
                            view! {
                                <div class=format!(
                                    "text-xs px-3 py-1.5 rounded {}",
                                    if is_ok { "text-green-700 bg-green-50" } else { "text-red-700 bg-red-50" }
                                )>
                                    <span class="font-medium">{if is_ok { "Correct" } else { "Incorrect" }}</span>
                                    {(!explanation.is_empty()).then(|| view! {
                                        <span>{format!(" - {}", explanation)}</span>
                                    })}
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }
            })}
        </div>
    }
}
