//! Phase 4: Prediction input.
//!
//! The student commits a quantitative prediction before the simulation unlocks.
//! Once submitted, the value is **locked** — no changes allowed.  After the
//! sim runs, the actual result is shown alongside their prediction.

use leptos::prelude::*;

#[component]
pub fn PredictionInput(
    /// The question, e.g. "What will the acceleration be?"
    question: String,
    /// Unit label, e.g. "m/s^2".
    unit: String,
    /// Correct answer (revealed after sim runs).
    answer: f64,
    /// Tolerance percentage.
    #[prop(default = 5.0)]
    tolerance_pct: f32,
    /// Fired with (user_prediction, is_close_enough) when locked in.
    on_predict: Callback<(f64, bool)>,
    /// Fired when this stage is complete (after reflection if wrong).
    on_complete: Callback<()>,
) -> impl IntoView {
    let (input_value, set_input_value) = signal(String::new());
    let (locked, set_locked) = signal(false);
    let (prediction, set_prediction) = signal(Option::<f64>::None);
    let (sim_finished, set_sim_finished) = signal(false);

    let answer_val = answer;
    let tol = tolerance_pct;

    let submit = move || {
        let Ok(val) = input_value.get().parse::<f64>() else {
            return;
        };

        set_prediction.set(Some(val));
        set_locked.set(true);

        let error_pct = if answer_val.abs() > 1e-9 {
            ((val - answer_val).abs() / answer_val.abs()) * 100.0
        } else {
            (val - answer_val).abs() * 100.0
        };
        let is_close = error_pct <= tol as f64;

        on_predict.run((val, is_close));

        set_sim_finished.set(true);
        on_complete.run(());
    };

    view! {
        <div class="space-y-3">
            <p class="text-sm font-medium text-gray-700 dark:text-gray-300">
                {question}
            </p>

            <div class="flex items-center gap-2">
                <input
                    type="number"
                    step="any"
                    class=move || format!(
                        "flex-1 px-3 py-2 border rounded-lg text-sm font-mono {}",
                        if locked.get() {
                            "bg-gray-100 dark:bg-gray-700 cursor-not-allowed border-gray-300"
                        } else {
                            "border-gray-300 dark:border-gray-600 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                        }
                    )
                    placeholder="Enter your prediction..."
                    prop:value=move || input_value.get()
                    on:input=move |ev| set_input_value.set(event_target_value(&ev))
                    disabled=move || locked.get()
                    on:keydown=move |ev: web_sys::KeyboardEvent| {
                        if ev.key() == "Enter" && !locked.get() {
                            submit();
                        }
                    }
                />
                <span class="text-sm text-gray-500 font-mono">{unit.clone()}</span>
                {move || (!locked.get()).then(|| view! {
                    <button
                        class="px-4 py-2 bg-amber-600 text-white rounded-lg hover:bg-amber-700 transition-colors font-medium text-sm whitespace-nowrap"
                        on:click=move |_| submit()
                    >
                        "Lock in prediction"
                    </button>
                })}
            </div>

            // Lock confirmation
            {move || locked.get().then(|| view! {
                <div class="flex items-center gap-1.5 text-xs text-gray-500">
                    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z">
                        </path>
                    </svg>
                    <span>"Prediction locked. Simulation unlocked!"</span>
                </div>
            })}

            // Result comparison (after sim finishes)
            {move || (locked.get() && sim_finished.get()).then(|| {
                let pred = prediction.get().unwrap_or(0.0);
                let error_pct = if answer_val.abs() > 1e-9 {
                    ((pred - answer_val).abs() / answer_val.abs()) * 100.0
                } else {
                    (pred - answer_val).abs() * 100.0
                };
                let is_close = error_pct <= tol as f64;

                view! {
                    <div class=format!(
                        "p-3 rounded-lg border {}",
                        if is_close {
                            "bg-green-50 border-green-200 dark:bg-green-900/20 dark:border-green-800"
                        } else {
                            "bg-amber-50 border-amber-200 dark:bg-amber-900/20 dark:border-amber-800"
                        }
                    )>
                        <div class="grid grid-cols-2 gap-2 text-sm">
                            <div>
                                <span class="text-gray-500">"Your prediction: "</span>
                                <span class="font-mono font-semibold">{format!("{:.2}", pred)}</span>
                            </div>
                            <div>
                                <span class="text-gray-500">"Actual result: "</span>
                                <span class="font-mono font-semibold">{format!("{:.2}", answer_val)}</span>
                            </div>
                        </div>
                        <div class=format!(
                            "mt-1 text-xs font-medium {}",
                            if is_close { "text-green-700 dark:text-green-400" } else { "text-amber-700 dark:text-amber-400" }
                        )>
                            {if is_close {
                                format!("Within {:.0}% - excellent prediction!", error_pct)
                            } else {
                                format!("Off by {:.0}% - let's figure out why.", error_pct)
                            }}
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
