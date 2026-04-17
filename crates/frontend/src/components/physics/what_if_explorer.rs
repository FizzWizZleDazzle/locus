//! Phase 6: "What if?" post-solve exploration.
//!
//! After solving, the student explores parameter variations to build intuition.
//! Each prompt suggests a parameter change and asks for a prediction.

use leptos::prelude::*;

use locus_physics_common::challenge::WhatIfPrompt;

#[component]
pub fn WhatIfExplorer(
    /// The exploration prompts.
    prompts: Vec<WhatIfPrompt>,
    /// Fired each time a what-if is explored (with the prompt index).
    on_explore: Callback<usize>,
) -> impl IntoView {
    let (explored, set_explored) = signal(Vec::<usize>::new());
    let (active_prompt, set_active_prompt) = signal(Option::<usize>::None);
    let (user_prediction, set_user_prediction) = signal(String::new());
    let (show_insight, set_show_insight) = signal(false);

    let prompts_for_view = prompts.clone();

    view! {
        <div class="space-y-3">
            <div class="flex items-center gap-2">
                <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300">"Explore further"</h3>
                <span class="text-xs text-gray-400">
                    {move || {
                        let count = explored.get().len();
                        format!("{}/{} explored", count, prompts_for_view.len())
                    }}
                </span>
            </div>

            {prompts.clone().into_iter().enumerate().map(|(i, prompt)| {
                let prompt_q = prompt.question.clone();
                let prompt_insight = prompt.expected_insight.clone();
                let prompt_param = prompt.parameter_key.clone();
                let prompt_val = prompt.suggested_value;

                view! {
                    <div class=move || format!(
                        "p-3 rounded-lg border transition-colors {}",
                        if explored.get().contains(&i) {
                            "border-green-200 bg-green-50/50 dark:bg-green-900/10"
                        } else if active_prompt.get() == Some(i) {
                            "border-blue-300 bg-blue-50/50 dark:bg-blue-900/10"
                        } else {
                            "border-gray-200 dark:border-gray-600"
                        }
                    )>
                        <p class="text-sm font-medium text-gray-700 dark:text-gray-300">
                            {prompt_q.clone()}
                        </p>

                        {move || if active_prompt.get() == Some(i) && !explored.get().contains(&i) {
                            Some(view! {
                                <div class="mt-2 space-y-2">
                                    <input
                                        type="text"
                                        class="w-full px-3 py-1.5 border rounded text-sm border-gray-300 dark:border-gray-600"
                                        placeholder="What do you think will happen?"
                                        prop:value=move || user_prediction.get()
                                        on:input=move |ev| set_user_prediction.set(event_target_value(&ev))
                                    />
                                    <div class="flex items-center gap-2">
                                        <button
                                            class="px-3 py-1 bg-blue-600 text-white rounded text-xs hover:bg-blue-700"
                                            on:click=move |_| {
                                                set_show_insight.set(true);
                                                set_explored.update(|e| e.push(i));
                                                on_explore.run(i);
                                            }
                                        >
                                            "Try it"
                                        </button>
                                        <span class="text-xs text-gray-400">
                                            {format!("Set {} = {}", prompt_param, prompt_val)}
                                        </span>
                                    </div>
                                    {move || show_insight.get().then(|| view! {
                                        <div class="px-3 py-2 bg-purple-50 dark:bg-purple-900/20 rounded text-purple-700 dark:text-purple-400 text-xs">
                                            {prompt_insight.clone()}
                                        </div>
                                    })}
                                </div>
                            })
                        } else if !explored.get().contains(&i) {
                            Some(view! {
                                <button
                                    class="mt-1 text-xs text-blue-600 hover:underline"
                                    on:click=move |_| {
                                        set_active_prompt.set(Some(i));
                                        set_show_insight.set(false);
                                        set_user_prediction.set(String::new());
                                    }
                                >
                                    "Explore this"
                                </button>
                            })
                        } else {
                            Some(view! {
                                <span class="text-xs text-green-600 mt-1 flex items-center gap-1">
                                    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                    </svg>
                                    "Explored"
                                </span>
                            })
                        }}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
