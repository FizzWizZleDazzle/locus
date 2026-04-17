//! Phase 5: Reflection and error diagnosis.
//!
//! When the student's prediction was wrong, they diagnose their own mistake
//! by selecting from a list of common errors.  The correct diagnosis unlocks
//! a targeted micro-lesson.

use leptos::prelude::*;
use std::collections::HashMap;

use locus_physics_common::challenge::{DiagnosticOption, MicroLesson};

#[component]
pub fn ReflectionPanel(
    /// Diagnostic options the student picks from.
    options: Vec<DiagnosticOption>,
    /// Micro-lessons keyed by option id.
    micro_lessons: HashMap<String, MicroLesson>,
    /// Fired when the student successfully diagnoses their error.
    on_complete: Callback<String>,
) -> impl IntoView {
    let (selected, set_selected) = signal(Option::<String>::None);
    let (submitted, set_submitted) = signal(false);
    let (correct_diagnosis, set_correct_diagnosis) = signal(false);
    let (active_lesson, set_active_lesson) = signal(Option::<MicroLesson>::None);

    let options_for_check = options.clone();
    let lessons = micro_lessons.clone();

    let on_submit = move |_| {
        let Some(sel_id) = selected.get() else {
            return;
        };

        set_submitted.set(true);

        let is_correct = options_for_check
            .iter()
            .any(|opt| opt.id == sel_id && opt.is_correct);

        set_correct_diagnosis.set(is_correct);

        // Show the micro-lesson for the selected diagnosis
        if let Some(lesson) = lessons.get(&sel_id) {
            set_active_lesson.set(Some(lesson.clone()));
        }

        if is_correct {
            on_complete.run(sel_id);
        }
    };

    view! {
        <div class="space-y-3">
            <p class="text-sm font-medium text-gray-700 dark:text-gray-300">
                "What do you think went wrong? Select the most likely cause:"
            </p>

            <div class="space-y-1.5">
                {options.into_iter().map(|opt| {
                    let opt_id = opt.id.clone();
                    let opt_id2 = opt.id.clone();
                    let opt_id3 = opt.id.clone();
                    let is_correct_opt = opt.is_correct;
                    view! {
                        <label
                            class=move || {
                                let base = "flex items-center gap-2.5 p-2.5 rounded-lg border cursor-pointer transition-colors text-sm";
                                let is_sel = selected.get().as_deref() == Some(&opt_id);
                                if submitted.get() {
                                    let was_selected = selected.get().as_deref() == Some(&opt_id2);
                                    if was_selected && is_correct_opt {
                                        format!("{} border-green-300 bg-green-50 dark:bg-green-900/20", base)
                                    } else if was_selected && !is_correct_opt {
                                        format!("{} border-red-300 bg-red-50 dark:bg-red-900/20", base)
                                    } else {
                                        format!("{} border-gray-200 dark:border-gray-600 opacity-60", base)
                                    }
                                } else if is_sel {
                                    format!("{} border-blue-400 bg-blue-50 dark:bg-blue-900/30", base)
                                } else {
                                    format!("{} border-gray-200 dark:border-gray-600 hover:border-gray-300", base)
                                }
                            }
                        >
                            <input
                                type="radio"
                                name="diagnosis"
                                class="text-blue-600"
                                prop:checked=move || selected.get().as_deref() == Some(&opt_id3)
                                on:change=move |_| set_selected.set(Some(opt.id.clone()))
                                disabled=move || submitted.get()
                            />
                            <span>{opt.label.clone()}</span>
                        </label>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Submit button
            {move || (!submitted.get()).then(|| view! {
                <button
                    class="py-2 px-4 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-medium text-sm disabled:opacity-50 disabled:cursor-not-allowed"
                    on:click=on_submit.clone()
                    disabled=move || selected.get().is_none()
                >
                    "Submit diagnosis"
                </button>
            })}

            // Micro-lesson
            {move || active_lesson.get().map(|lesson| view! {
                <div class="p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
                    <p class="text-sm text-blue-800 dark:text-blue-300">
                        {lesson.explanation_latex}
                    </p>
                </div>
            })}

            // Retry prompt if wrong
            {move || (submitted.get() && !correct_diagnosis.get()).then(|| view! {
                <button
                    class="text-sm text-blue-600 hover:underline"
                    on:click=move |_| {
                        set_submitted.set(false);
                        set_selected.set(None);
                        set_active_lesson.set(None);
                    }
                >
                    "Try again"
                </button>
            })}
        </div>
    }
}
