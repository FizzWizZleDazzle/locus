//! Practice mode page

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use locus_common::ProblemResponse;

use crate::{
    ThemeContext,
    components::{LatexRenderer, ProblemInterface, TopicSelector, Whiteboard},
    formatters::format_answer_for_display,
    grader::{GradeResult, check_answer, preprocess_input},
    problem_queue::ProblemQueue,
    utils::{push_url_playing, setup_popstate_listener, update_url},
};

// format_answer_for_display moved to crate::formatters module

#[component]
pub fn Practice() -> impl IntoView {
    let query = use_query_map();
    let theme = expect_context::<ThemeContext>();

    // Topic selection state
    let (selected_topic, set_selected_topic) = signal(None::<String>);
    let (selected_subtopics, set_selected_subtopics) = signal(Vec::<String>::new());

    // Problem state
    let (problem, set_problem) = signal(None::<ProblemResponse>);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    // Answer state
    let (answer, set_answer) = signal(String::new());
    let (result, set_result) = signal(None::<GradeResult>);
    let (show_answer, set_show_answer) = signal(false);

    // Whiteboard mode
    let (whiteboard_mode, set_whiteboard_mode) = signal(false);

    // Problem queue for batch fetching
    let queue = ProblemQueue::new(true);

    let load_problem = move || {
        set_error.set(None);
        set_answer.set(String::new());
        set_result.set(None);
        set_show_answer.set(false);

        let topic = selected_topic.get();
        let subtopics = selected_subtopics.get();

        if let Some(p) = queue.next(topic.clone(), subtopics.clone()) {
            set_problem.set(Some(p));
            set_loading.set(false);
        } else {
            // Queue empty — fetch and wait for first result
            set_loading.set(true);
            queue.fetch(topic, subtopics);
        }
    };

    // When the queue finishes loading and we're still waiting for a problem, pop one
    Effect::new(move |_| {
        if loading.get() && !queue.loading() {
            if let Some(err) = queue.error() {
                set_error.set(Some(err));
                set_loading.set(false);
            } else {
                let topic = selected_topic.get();
                let subtopics = selected_subtopics.get();
                if let Some(p) = queue.next(topic, subtopics) {
                    set_problem.set(Some(p));
                    set_loading.set(false);
                }
            }
        }
    });

    let on_topic_change = Callback::new(move |topic: String| {
        set_selected_topic.set(Some(topic.clone()));
        set_selected_subtopics.set(Vec::new());

        // Update URL immediately when topic is selected
        update_url(&format!("/practice?topic={}", topic));
    });

    let on_subtopics_change = Callback::new(move |subtopics: Vec<String>| {
        set_selected_subtopics.set(subtopics.clone());

        // Update URL immediately when subtopics change
        if let Some(topic) = selected_topic.get() {
            let url = if subtopics.is_empty() {
                format!("/practice?topic={}", topic)
            } else {
                let subtopic_str = subtopics.join(",");
                format!("/practice?topic={}&subtopics={}", topic, subtopic_str)
            };
            update_url(&url);
        }
    });

    let on_topic_confirm = Callback::new(move |(topic, subtopics): (String, Vec<String>)| {
        set_selected_topic.set(Some(topic.clone()));
        set_selected_subtopics.set(subtopics.clone());

        // Clear stale problems from previous topic selection
        queue.clear();

        // Create a history entry with 'playing' state so back button can return to topic selector
        let url = if subtopics.is_empty() {
            format!("/practice?topic={}", topic)
        } else {
            let subtopic_str = subtopics.join(",");
            format!("/practice?topic={}&subtopics={}", topic, subtopic_str)
        };
        push_url_playing(&url);

        load_problem();
    });

    // Parse URL params on mount to get initial values
    let (initial_topic, set_initial_topic) = signal(None::<String>);
    let (initial_subtopics, set_initial_subtopics) = signal(None::<Vec<String>>);

    // Watch for URL changes (including browser back/forward)
    Effect::new(move |_| {
        let topic_param = query.read().get("topic");
        let subtopics_param = query.read().get("subtopics");

        if let Some(topic_val) = topic_param {
            if !topic_val.is_empty() {
                // Parse subtopics from comma-separated string
                let subtopics: Vec<String> = if let Some(st) = subtopics_param {
                    st.split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                } else {
                    Vec::new()
                };

                // Set initial values for TopicSelector
                set_initial_topic.set(Some(topic_val.clone()));
                set_initial_subtopics.set(Some(subtopics.clone()));

                // Also set parent state so callbacks work correctly
                set_selected_topic.set(Some(topic_val));
                set_selected_subtopics.set(subtopics);
            }
        } else {
            // No topic in URL - clear everything to show topic selector
            set_initial_topic.set(None);
            set_initial_subtopics.set(None);
            set_selected_topic.set(None);
            set_selected_subtopics.set(Vec::new());
            set_problem.set(None);
        }
    });

    // Listen for popstate events (back/forward navigation)
    Effect::new(move |_| {
        setup_popstate_listener(move || {
            set_problem.set(None);
        });
    });

    // Copy signals for use in closures
    let problem_for_check = problem;
    let answer_for_check = answer;
    let set_result_for_check = set_result;

    let result_for_submit = result;
    let on_submit = Callback::new(move |_| {
        if result_for_submit.get().is_some() {
            return;
        }
        if let Some(p) = problem_for_check.get() {
            let user_input = preprocess_input(&answer_for_check.get());
            if let Some(answer_key) = &p.answer_key {
                let grade = check_answer(&user_input, answer_key, p.grading_mode, p.answer_type);
                set_result_for_check.set(Some(grade));
            }
        }
    });

    let reset_selection = move || {
        set_selected_topic.set(None);
        set_selected_subtopics.set(Vec::new());
        set_problem.set(None);
    };

    // Derive problem ID for whiteboard clearing
    let problem_id_signal = Signal::derive(move || {
        problem.get().map(|p| p.id.to_string()).unwrap_or_default()
    });

    let wb_signal = Signal::derive(move || whiteboard_mode.get());

    view! {
        <div class=move || if whiteboard_mode.get() && problem.get().is_some() {
            "relative w-full h-[calc(100vh-4rem)] overflow-hidden"
        } else {
            "max-w-2xl mx-auto px-4 py-8"
        }>
            // Header — normal bar, or floating elements on canvas in whiteboard mode
            {move || if whiteboard_mode.get() && problem.get().is_some() {
                view! {
                    // Small label top-left
                    <span class="absolute top-3 left-3 z-30 text-xs text-gray-600 dark:text-gray-300 font-medium select-none">"Practice"</span>
                    // Controls top-right
                    <div class="absolute top-3 right-3 z-30 flex items-center gap-2"
                        on:pointerdown=|ev: web_sys::PointerEvent| ev.stop_propagation()
                    >
                        <button
                            class="p-1.5 rounded border bg-gray-900 text-white border-gray-900 dark:bg-gray-100 dark:text-gray-900 dark:border-gray-100 transition-colors"
                            on:click=move |_| set_whiteboard_mode.update(|v| *v = !*v)
                            title="Toggle whiteboard"
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z">
                                </path>
                            </svg>
                        </button>
                        <button
                            class="text-sm text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white"
                            on:click=move |_| reset_selection()
                        >
                            "Change Topics"
                        </button>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="flex items-center justify-between mb-6">
                        <h1 class="text-2xl font-semibold">"Practice"</h1>
                        <div class="flex items-center gap-3">
                            {move || problem.get().is_some().then(|| view! {
                                <button
                                    class="p-1.5 rounded border text-gray-500 border-gray-300 hover:border-gray-400 hover:text-gray-700 transition-colors"
                                    on:click=move |_| set_whiteboard_mode.update(|v| *v = !*v)
                                    title="Toggle whiteboard"
                                >
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                            d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z">
                                        </path>
                                    </svg>
                                </button>
                            })}
                            {move || problem.get().is_some().then(|| view! {
                                <button
                                    class="text-sm text-gray-500 hover:text-gray-900"
                                    on:click=move |_| reset_selection()
                                >
                                    "Change Topics"
                                </button>
                            })}
                        </div>
                    </div>
                }.into_any()
            }}

            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            // Show topic selector if no problem loaded
            {move || problem.get().is_none().then(|| view! {
                <div class="border border-gray-200 rounded p-6">
                    <TopicSelector
                        on_confirm=on_topic_confirm
                        on_topic_change=on_topic_change
                        on_subtopics_change=on_subtopics_change
                        initial_topic=initial_topic.get()
                        initial_subtopics=initial_subtopics.get().unwrap_or_default()
                    />
                </div>
            })}

            // Show problem once loaded
            {move || problem.get().is_some().then(|| view! {
                <div class=move || if whiteboard_mode.get() { "relative w-full h-full" } else { "space-y-6" }>
                    // Whiteboard canvas (behind everything)
                    {move || whiteboard_mode.get().then(|| view! {
                        <Whiteboard problem_id=problem_id_signal is_dark=Signal::derive(move || theme.is_dark.get()) />
                    })}

                    {move || loading.get().then(|| view! {
                        <div class="text-gray-500 text-sm">"Loading..."</div>
                    })}

                    // Show answer key if revealed
                    {move || (show_answer.get() && problem.get().is_some() && !whiteboard_mode.get()).then(|| {
                        problem.get().and_then(|p| {
                            let ans = p.answer_key.clone()?;
                            let answer_type = p.answer_type;

                            // Format answer based on its type
                            let rendered_answer = format_answer_for_display(&ans, answer_type)
                                .unwrap_or_else(|_| format!("<code>{}</code>", ans));

                            let solution = p.solution_latex.clone();
                            Some(view! {
                                <div class="p-4 bg-blue-50 border border-blue-200 rounded">
                                    <div class="text-sm font-medium text-blue-900 mb-1">"Answer:"</div>
                                    <div class="text-blue-800 text-xl" inner_html=rendered_answer></div>
                                </div>
                                {(!solution.is_empty()).then(|| view! {
                                    <div class="p-4 bg-blue-50 border border-blue-200 rounded mt-2">
                                        <div class="text-sm font-medium text-blue-900 mb-1">"Solution:"</div>
                                        <div class="text-blue-800">
                                            <LatexRenderer content=solution />
                                        </div>
                                    </div>
                                })}
                            })
                        })
                    })}

                    <ProblemInterface
                        problem=problem
                        answer=answer
                        set_answer=set_answer
                        on_submit=on_submit
                        whiteboard_mode=wb_signal
                        render_controls=move || {
                            // Inline check logic to avoid capturing closures
                            let check_inline = move || {
                                if let Some(p) = problem.get() {
                                    let user_input = preprocess_input(&answer.get());
                                    if let Some(answer_key) = &p.answer_key {
                                        let grade = check_answer(&user_input, answer_key, p.grading_mode, p.answer_type);
                                        set_result.set(Some(grade));
                                    }
                                }
                            };

                            view! {
                                <div class="flex space-x-2">
                                    <button
                                        class="flex-1 px-4 py-2 bg-gray-900 text-white rounded hover:bg-gray-800 disabled:opacity-50"
                                        on:click=move |_| check_inline()
                                        disabled=move || answer.get().is_empty() || result.get().is_some()
                                    >
                                        "Check"
                                    </button>
                                    <button
                                        class="px-4 py-2 border border-gray-300 rounded hover:border-gray-400"
                                        on:click=move |_| set_show_answer.set(true)
                                    >
                                        "Reveal"
                                    </button>
                                </div>

                                {move || (result.get().map(|r| r.is_correct()).unwrap_or(false) || show_answer.get()).then(|| view! {
                                    <button
                                        class="w-full px-4 py-2 mt-2 border border-gray-300 rounded hover:border-gray-400"
                                        on:click=move |_| load_problem()
                                    >
                                        "Next Problem"
                                    </button>
                                })}
                            }
                        }
                        render_result=move || result.get().map(|r| {
                            let (color, msg): (&str, String) = match r {
                                GradeResult::Correct => ("text-green-600", "Correct".to_string()),
                                GradeResult::Incorrect => ("text-red-600", "Incorrect".to_string()),
                                GradeResult::Invalid(m) => ("text-yellow-600", m),
                                GradeResult::Error(m) => ("text-red-600", m),
                            };
                            view! {
                                <div class=format!("text-sm {}", color)>{msg}</div>
                            }
                        })
                    />
                </div>
            })}
        </div>
    }
}
