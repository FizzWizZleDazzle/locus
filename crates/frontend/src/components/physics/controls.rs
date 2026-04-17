//! Simulation playback controls — play/pause/step/reset/speed.
//!
//! Controls are **disabled** until the simulation is unlocked (i.e. the
//! student has committed a prediction).

use leptos::prelude::*;

#[component]
pub fn PhysicsControls(
    /// Whether the simulation is unlocked (prediction committed).
    #[prop(into)]
    unlocked: Signal<bool>,
    /// Callback: play
    on_play: Callback<()>,
    /// Callback: pause
    on_pause: Callback<()>,
    /// Callback: step forward one frame
    on_step: Callback<()>,
    /// Callback: reset to initial state
    on_reset: Callback<()>,
    /// Callback: speed changed
    on_speed: Callback<f32>,
) -> impl IntoView {
    let (is_playing, set_is_playing) = signal(false);
    let (speed, set_speed) = signal(1.0f32);

    let disabled = move || !unlocked.get();

    let play_pause = move |_| {
        if is_playing.get() {
            set_is_playing.set(false);
            on_pause.run(());
        } else {
            set_is_playing.set(true);
            on_play.run(());
        }
    };

    let step = move |_| {
        on_step.run(());
    };

    let reset = move |_| {
        set_is_playing.set(false);
        on_reset.run(());
    };

    let speed_options: Vec<(f32, &str)> = vec![
        (0.25, "0.25x"),
        (0.5, "0.5x"),
        (1.0, "1x"),
        (2.0, "2x"),
    ];

    view! {
        <div class="flex items-center gap-2 p-2 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
            // Play/Pause
            <button
                class=move || format!(
                    "p-2 rounded-lg transition-colors {}",
                    if disabled() {
                        "bg-gray-100 text-gray-400 cursor-not-allowed"
                    } else {
                        "bg-blue-50 text-blue-600 hover:bg-blue-100 dark:bg-blue-900/30 dark:text-blue-400"
                    }
                )
                on:click=play_pause
                disabled=disabled
                title=move || if is_playing.get() { "Pause" } else { "Play" }
            >
                {move || if is_playing.get() {
                    view! {
                        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M6 4h4v16H6zM14 4h4v16h-4z"></path>
                        </svg>
                    }
                } else {
                    view! {
                        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M8 5v14l11-7z"></path>
                        </svg>
                    }
                }}
            </button>

            // Step forward
            <button
                class=move || format!(
                    "p-2 rounded-lg transition-colors {}",
                    if disabled() {
                        "bg-gray-100 text-gray-400 cursor-not-allowed"
                    } else {
                        "hover:bg-gray-100 dark:hover:bg-gray-700"
                    }
                )
                on:click=step
                disabled=disabled
                title="Step forward"
            >
                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M6 4l10 8-10 8V4zM18 4h2v16h-2V4z"></path>
                </svg>
            </button>

            // Reset
            <button
                class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                on:click=reset
                title="Reset"
            >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15">
                    </path>
                </svg>
            </button>

            // Divider
            <div class="w-px h-6 bg-gray-300 dark:bg-gray-600"></div>

            // Speed selector
            <div class="flex items-center gap-1">
                {speed_options.into_iter().map(|(s, label)| {
                    let s_val = s;
                    view! {
                        <button
                            class=move || format!(
                                "px-2 py-1 text-xs rounded transition-colors {}",
                                if (speed.get() - s_val).abs() < 0.01 {
                                    "bg-gray-200 dark:bg-gray-600 font-semibold"
                                } else if disabled() {
                                    "text-gray-400 cursor-not-allowed"
                                } else {
                                    "hover:bg-gray-100 dark:hover:bg-gray-700"
                                }
                            )
                            on:click=move |_| {
                                set_speed.set(s_val);
                                on_speed.run(s_val);
                            }
                            disabled=disabled
                        >
                            {label}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Locked indicator
            {move || (!unlocked.get()).then(|| view! {
                <div class="ml-auto flex items-center gap-1.5 text-amber-600 dark:text-amber-400">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z">
                        </path>
                    </svg>
                    <span class="text-xs font-medium">"Predict first"</span>
                </div>
            })}
        </div>
    }
}
