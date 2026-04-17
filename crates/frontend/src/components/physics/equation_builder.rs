//! Phase 3: Equation builder.
//!
//! Students assemble the governing equation by selecting terms from dropdowns.
//! Wrong combinations get targeted feedback.

use leptos::prelude::*;
use std::collections::HashMap;

use locus_physics_common::challenge::EquationTerm;

#[component]
pub fn EquationBuilder(
    /// Axis/context label, e.g. "parallel to the incline".
    axis_label: String,
    /// The correct ordered terms.
    correct_terms: Vec<EquationTerm>,
    /// All available terms (including distractors).
    available_terms: Vec<EquationTerm>,
    /// Error feedback keyed by wrong combinations.
    #[prop(default = HashMap::new())]
    error_feedback: HashMap<String, String>,
    /// Fired when the equation is built correctly.
    on_complete: Callback<()>,
) -> impl IntoView {
    let num_slots = correct_terms.len();
    let (slots, set_slots) = signal(vec![String::new(); num_slots]);
    let (submitted, set_submitted) = signal(false);
    let (is_correct, set_is_correct) = signal(false);
    let (feedback_msg, set_feedback_msg) = signal(Option::<String>::None);

    let correct_for_check = correct_terms.clone();
    let error_fb = error_feedback.clone();
    let available_for_view = available_terms.clone();

    let on_check = move |_| {
        let current_slots = slots.get();
        set_submitted.set(true);

        // Check if the selected terms match the correct terms
        let correct = current_slots
            .iter()
            .zip(correct_for_check.iter())
            .all(|(selected_id, expected)| selected_id == &expected.id);

        set_is_correct.set(correct);

        if correct {
            set_feedback_msg.set(None);
            on_complete.run(());
        } else {
            // Look for targeted feedback
            let combo_key = current_slots.join(",");
            let msg = error_fb
                .get(&combo_key)
                .cloned()
                .unwrap_or_else(|| "Not quite. Check the direction and components of each force.".into());
            set_feedback_msg.set(Some(msg));
            // Allow retry
            set_submitted.set(false);
        }
    };

    view! {
        <div class="space-y-3">
            <p class="text-sm text-gray-600 dark:text-gray-300">
                "Build the net force equation along the axis "
                <span class="font-semibold italic">{axis_label}</span>
                ":"
            </p>

            // Equation assembly area
            <div class="flex items-center gap-1 flex-wrap p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border">
                <span class="text-sm font-mono font-semibold">"&Sigma;F ="</span>
                {(0..num_slots).map(|i| {
                    let available = available_for_view.clone();
                    view! {
                        <div class="flex items-center gap-0.5">
                            {(i > 0).then(|| view! { <span class="text-gray-400 mx-1">"+"</span> })}
                            <select
                                class="px-2 py-1 text-sm border rounded bg-white dark:bg-gray-700 dark:border-gray-600 font-mono"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_slots.update(|s| {
                                        if i < s.len() {
                                            s[i] = value;
                                        }
                                    });
                                }
                            >
                                <option value="">"Select..."</option>
                                {available.iter().map(|term| {
                                    view! {
                                        <option value={term.id.clone()}>
                                            {format!("{}{}", term.sign, term.latex)}
                                        </option>
                                    }
                                }).collect::<Vec<_>>()}
                            </select>
                        </div>
                    }
                }).collect::<Vec<_>>()}
                <span class="text-sm font-mono font-semibold ml-1">"= ma"</span>
            </div>

            // Check button
            {move || (!is_correct.get()).then(|| view! {
                <button
                    class="py-2 px-4 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-medium text-sm"
                    on:click=on_check.clone()
                >
                    "Check equation"
                </button>
            })}

            // Feedback
            {move || feedback_msg.get().map(|msg| view! {
                <div class="px-3 py-2 bg-red-50 dark:bg-red-900/20 rounded-lg text-red-700 dark:text-red-400 text-sm">
                    {msg}
                </div>
            })}

            // Success
            {move || is_correct.get().then(|| view! {
                <div class="flex items-center gap-2 px-3 py-2 bg-green-50 dark:bg-green-900/20 rounded-lg text-green-700 dark:text-green-400">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                    </svg>
                    <span class="text-sm font-medium">"Equation correct!"</span>
                </div>
            })}
        </div>
    }
}
