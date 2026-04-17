//! Phase 2: Free-body diagram builder.
//!
//! Students drag force arrows onto the correct body in the correct direction.
//! Forces are validated against expected directions with a tolerance.

use leptos::prelude::*;
use std::collections::HashMap;

use locus_physics_common::challenge::ForceSpec;

#[component]
pub fn FbdBuilder(
    /// The body on which forces should be placed.
    target_body: String,
    /// Expected forces with directions.
    expected_forces: Vec<ForceSpec>,
    /// Direction tolerance in degrees.
    #[prop(default = 15.0)]
    tolerance_deg: f32,
    /// Per-force hints.
    #[prop(default = HashMap::new())]
    per_force_hints: HashMap<String, String>,
    /// Fired with the number of attempts it took.
    on_complete: Callback<i32>,
) -> impl IntoView {
    let (attempts, set_attempts) = signal(0i32);
    let (placed_forces, set_placed_forces) = signal(Vec::<(String, bool)>::new());
    let (completed, set_completed) = signal(false);
    let (show_hint, set_show_hint) = signal(Option::<String>::None);

    let forces_for_view = expected_forces.clone();
    let hints = per_force_hints.clone();

    let on_check = move |_| {
        set_attempts.update(|a| *a += 1);

        // In a full implementation, this would read the actual placed arrows
        // from the canvas. For now, we simulate a check against expected forces.
        // The student interacts with the canvas to place arrows, and we validate
        // positions/directions via the SimulationEngine.

        // Placeholder: check if all forces have been placed
        let placed = placed_forces.get();
        let all_correct = expected_forces.iter().all(|ef| {
            placed.iter().any(|(id, correct)| id == &ef.id && *correct)
        });

        if all_correct {
            set_completed.set(true);
            on_complete.run(attempts.get_untracked() + 1);
        }
    };

    view! {
        <div class="space-y-3">
            <p class="text-sm text-gray-600 dark:text-gray-300">
                "Drag the force arrows onto the "
                <span class="font-semibold">{target_body.clone()}</span>
                " in the correct directions."
            </p>

            // Available force arrows
            <div class="flex flex-wrap gap-2">
                {forces_for_view.into_iter().map(|force| {
                    let force_id = force.id.clone();
                    let force_id2 = force.id.clone();
                    let hint_text = hints.get(&force.id).cloned();
                    view! {
                        <div
                            class="flex items-center gap-1.5 px-3 py-1.5 rounded-full border-2 cursor-grab active:cursor-grabbing transition-colors"
                            style=format!("border-color: {}; background-color: {}22", force.color, force.color)
                            draggable="true"
                        >
                            // Arrow icon
                            <svg class="w-4 h-4" style=format!("color: {}", force.color) fill="currentColor" viewBox="0 0 24 24">
                                <path d="M12 2l-5 9h3v11h4V11h3z"></path>
                            </svg>
                            <span class="text-xs font-medium">{force.label.clone()}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Check button
            {move || (!completed.get()).then(|| view! {
                <div class="flex items-center gap-2">
                    <button
                        class="py-2 px-4 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-medium text-sm"
                        on:click=on_check.clone()
                    >
                        "Check my FBD"
                    </button>
                    <span class="text-xs text-gray-500">
                        {move || format!("Attempt {}", attempts.get() + 1)}
                    </span>
                </div>
            })}

            // Success message
            {move || completed.get().then(|| view! {
                <div class="flex items-center gap-2 px-3 py-2 bg-green-50 dark:bg-green-900/20 rounded-lg text-green-700 dark:text-green-400">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                    </svg>
                    <span class="text-sm font-medium">
                        {move || format!("Free-body diagram correct! (attempt {})", attempts.get())}
                    </span>
                </div>
            })}

            // Hint display
            {move || show_hint.get().map(|hint| view! {
                <div class="px-3 py-2 bg-amber-50 dark:bg-amber-900/20 rounded-lg text-amber-700 dark:text-amber-400 text-xs">
                    {hint}
                </div>
            })}
        </div>
    }
}
