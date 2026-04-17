//! Single physics problem page — simulation + challenge panel + answer input.
//!
//! Layout matches the main app: max-w-5xl container for the wider sim layout,
//! same button/border/text styling as Practice and Daily pages.

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use locus_physics_common::{
    PhysicsProblemResponse, PhysicsSubmitRequest, PhysicsAnswerInput, PhysicsSubmitResponse,
};

use crate::{
    api,
    components::physics::{ChallengePanel, PhysicsCanvas, PhysicsControls},
    components::LatexRenderer,
};

#[component]
pub fn PhysicsProblem() -> impl IntoView {
    let params = use_params_map();

    let (problem, set_problem) = signal(Option::<PhysicsProblemResponse>::None);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);

    // Challenge tracking
    let (hints_used, set_hints_used) = signal(0i32);
    let (fbd_attempts, set_fbd_attempts) = signal(0i32);
    let (stages_completed, set_stages_completed) = signal(0i32);
    let (what_ifs_explored, set_what_ifs_explored) = signal(0i32);
    let (sim_unlocked, set_sim_unlocked) = signal(false);

    // Answer input
    let (answers, set_answers) = signal(Vec::<(String, String)>::new()); // (label, value)
    let (submit_result, set_submit_result) = signal(Option::<PhysicsSubmitResponse>::None);
    let (submitting, set_submitting) = signal(false);

    // Scene JSON for the canvas
    let scene_json = Signal::derive(move || {
        problem.get().map(|p| serde_json::to_string(&p.scene_definition).unwrap_or_default())
    });

    // Fetch problem on mount
    Effect::new(move |_| {
        let id_str = params.read().get("id");
        let Some(id_str) = id_str else {
            set_error.set(Some("No problem ID provided".into()));
            set_loading.set(false);
            return;
        };

        let Ok(id) = uuid::Uuid::parse_str(&id_str) else {
            set_error.set(Some("Invalid problem ID".into()));
            set_loading.set(false);
            return;
        };

        leptos::task::spawn_local(async move {
            match api::get_physics_problem(id).await {
                Ok(p) => {
                    // Initialise answer slots
                    let answer_slots: Vec<(String, String)> = p
                        .answer_spec
                        .parts
                        .iter()
                        .map(|part| (part.label.clone(), String::new()))
                        .collect();
                    set_answers.set(answer_slots);
                    set_problem.set(Some(p));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    });

    // Submit handler
    let on_submit = move |_| {
        let Some(p) = problem.get() else { return };
        set_submitting.set(true);

        let answer_inputs: Vec<PhysicsAnswerInput> = answers
            .get()
            .iter()
            .enumerate()
            .filter_map(|(i, (_, val))| {
                val.parse::<f64>().ok().map(|v| PhysicsAnswerInput {
                    part_index: i,
                    value: v,
                })
            })
            .collect();

        let req = PhysicsSubmitRequest {
            problem_id: p.id,
            answers: answer_inputs,
            parameters_used: serde_json::Value::Object(serde_json::Map::new()),
            hints_used: hints_used.get_untracked(),
            fbd_attempts: fbd_attempts.get_untracked(),
            stages_completed: stages_completed.get_untracked(),
            what_ifs_explored: what_ifs_explored.get_untracked(),
            time_taken_ms: None,
        };

        leptos::task::spawn_local(async move {
            match api::submit_physics_answer(&req).await {
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
    };

    view! {
        <div class="max-w-5xl mx-auto px-4 py-8">
            // Loading
            {move || loading.get().then(|| view! {
                <div class="text-gray-500 text-sm text-center py-16">"Loading problem..."</div>
            })}

            // Error
            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            // Problem content
            {move || problem.get().map(|p| {
                let title = p.title.clone();
                let description = p.description_latex.clone();
                let topic_display = p.physics_topic.clone();
                let subtopic_display = p.physics_subtopic.replace('_', " ");
                let difficulty_label = match p.difficulty {
                    1 => "Easy",
                    2 => "Medium",
                    3 => "Hard",
                    4 => "Expert",
                    _ => "Medium",
                };

                view! {
                    <div>
                        // Header
                        <div class="flex items-center justify-between mb-4">
                            <div>
                                <div class="flex items-center gap-2 text-sm text-gray-500 mb-1">
                                    <a href="/physics" class="hover:text-gray-900 dark:hover:text-white">"Physics"</a>
                                    <span>"/"</span>
                                    <span class="capitalize">{topic_display}</span>
                                    <span>"/"</span>
                                    <span class="capitalize">{subtopic_display}</span>
                                </div>
                                <h1 class="text-2xl font-semibold">{title}</h1>
                            </div>
                            <span class="px-2 py-0.5 text-xs font-medium rounded bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400">
                                {difficulty_label}
                            </span>
                        </div>

                        // Problem description
                        <div class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg mb-4">
                            <LatexRenderer content=description />
                        </div>

                        // Main layout: simulation + challenge panel
                        <div class="grid grid-cols-1 lg:grid-cols-5 gap-4">
                            // Left: simulation canvas (3/5 width)
                            <div class="lg:col-span-3 space-y-3">
                                <PhysicsCanvas
                                    scene_json=scene_json
                                    unlocked=Signal::derive(move || sim_unlocked.get())
                                />

                                <PhysicsControls
                                    unlocked=Signal::derive(move || sim_unlocked.get())
                                    on_play=Callback::new(move |_| {
                                        let _ = js_sys::eval("if(window.__physics_sim_engine) window.__physics_sim_engine.play()");
                                    })
                                    on_pause=Callback::new(move |_| {
                                        let _ = js_sys::eval("if(window.__physics_sim_engine) window.__physics_sim_engine.pause()");
                                    })
                                    on_step=Callback::new(move |_| {
                                        let _ = js_sys::eval("if(window.__physics_sim_engine) window.__physics_sim_engine.step_forward()");
                                    })
                                    on_reset=Callback::new(move |_| {
                                        let _ = js_sys::eval("if(window.__physics_sim_engine) window.__physics_sim_engine.reset()");
                                    })
                                    on_speed=Callback::new(move |s: f32| {
                                        let _ = js_sys::eval(&format!(
                                            "if(window.__physics_sim_engine) window.__physics_sim_engine.set_speed({})", s
                                        ));
                                    })
                                />
                            </div>

                            // Right: challenge panel + answer input (2/5 width)
                            <div class="lg:col-span-2 space-y-4">
                                // Challenge stages
                                <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4 min-h-[300px]">
                                    <ChallengePanel
                                        stages=p.challenge_stages.clone()
                                        what_if_prompts=p.what_if_prompts.clone()
                                        on_unlock_sim=Callback::new(move |_| {
                                            set_sim_unlocked.set(true);
                                        })
                                        on_hint_used=Callback::new(move |_| {
                                            set_hints_used.update(|n| *n += 1);
                                        })
                                        hints_used=hints_used
                                        fbd_attempts=set_fbd_attempts
                                        stages_completed=set_stages_completed
                                        what_ifs_explored=set_what_ifs_explored
                                    />
                                </div>

                                // Answer input
                                <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
                                    <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3">"Your Answer"</h3>
                                    <div class="space-y-2">
                                        {move || answers.get().into_iter().enumerate().map(|(i, (label, val))| {
                                            let unit = p.answer_spec.parts.get(i).map(|p| p.unit.clone()).unwrap_or_default();
                                            view! {
                                                <div class="flex items-center gap-2">
                                                    <label class="text-sm text-gray-600 dark:text-gray-400 w-24 truncate">{label}</label>
                                                    <input
                                                        type="number"
                                                        step="any"
                                                        class="flex-1 px-3 py-1.5 border border-gray-300 dark:border-gray-600 rounded text-sm font-mono focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                                                        placeholder="0.00"
                                                        prop:value=val.clone()
                                                        on:input=move |ev| {
                                                            let new_val = event_target_value(&ev);
                                                            set_answers.update(|a| {
                                                                if let Some(slot) = a.get_mut(i) {
                                                                    slot.1 = new_val;
                                                                }
                                                            });
                                                        }
                                                    />
                                                    <span class="text-xs text-gray-500 font-mono">{unit}</span>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>

                                    // Submit button
                                    <button
                                        class="w-full mt-3 px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 dark:bg-white dark:text-gray-900 dark:hover:bg-gray-100 disabled:opacity-50 transition-colors"
                                        on:click=on_submit
                                        disabled=move || submitting.get() || submit_result.get().is_some()
                                    >
                                        {move || if submitting.get() { "Submitting..." } else { "Submit Answer" }}
                                    </button>

                                    // Result
                                    {move || submit_result.get().map(|res| {
                                        let total = res.score.total();
                                        view! {
                                            <div class=format!(
                                                "mt-3 p-3 rounded-lg border {}",
                                                if res.is_correct {
                                                    "bg-green-50 border-green-200 dark:bg-green-900/20 dark:border-green-800"
                                                } else {
                                                    "bg-red-50 border-red-200 dark:bg-red-900/20 dark:border-red-800"
                                                }
                                            )>
                                                <p class=format!(
                                                    "font-medium {}",
                                                    if res.is_correct { "text-green-700 dark:text-green-400" } else { "text-red-700 dark:text-red-400" }
                                                )>
                                                    {if res.is_correct { "Correct!" } else { "Not quite right." }}
                                                </p>

                                                // Score breakdown
                                                <div class="mt-2 grid grid-cols-2 gap-1 text-xs text-gray-600 dark:text-gray-400">
                                                    <div>{format!("Correctness: {}/40", res.score.correctness)}</div>
                                                    <div>{format!("Process: {}/30", res.score.process)}</div>
                                                    <div>{format!("Prediction: {}/15", res.score.prediction_accuracy)}</div>
                                                    <div>{format!("Independence: {}/15", res.score.independence)}</div>
                                                </div>
                                                <div class="mt-1 text-sm font-semibold text-gray-700 dark:text-gray-300">
                                                    {format!("Total: {}/100", total)}
                                                    {(res.score.exploration_bonus > 0).then(|| format!(" (+{} exploration)", res.score.exploration_bonus))}
                                                </div>
                                            </div>
                                        }
                                    })}
                                </div>
                            </div>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
