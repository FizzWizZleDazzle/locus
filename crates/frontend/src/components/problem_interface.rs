use crate::components::{AnswerInput, Draggable, LatexRenderer, ProblemCard};
use leptos::prelude::*;
use locus_common::ProblemResponse;

/// Unified problem display component used by both Practice and Ranked modes.
///
/// This component encapsulates:
/// - Problem rendering (ProblemCard)
/// - Answer input (AnswerInput with per-type adaptations, forced remounting via key prop)
/// - Mode-specific behavior via callback props
///
/// When `whiteboard_mode` is true, the card becomes collapsible and is wrapped
/// in a `<Draggable>` container that floats above the whiteboard canvas.
#[component]
pub fn ProblemInterface<ControlsView, ControlsViewOutput, ResultView, ResultViewOutput>(
    /// The current problem to display
    problem: ReadSignal<Option<ProblemResponse>>,

    /// Answer input signal (managed by parent)
    answer: ReadSignal<String>,
    set_answer: WriteSignal<String>,

    /// Callback for when user presses Enter or clicks action button
    on_submit: Callback<String>,

    /// Custom controls renderer (mode-specific buttons)
    render_controls: ControlsView,

    /// Custom result renderer (GradeResult for Practice, ELO for Ranked)
    render_result: ResultView,

    /// Whether whiteboard mode is active
    #[prop(optional, into)]
    whiteboard_mode: Option<Signal<bool>>,
) -> impl IntoView
where
    ControlsView: Fn() -> ControlsViewOutput + Copy + Send + Sync + 'static,
    ControlsViewOutput: IntoView + 'static,
    ResultView: Fn() -> ResultViewOutput + Copy + Send + Sync + 'static,
    ResultViewOutput: IntoView + 'static,
{
    let wb_mode = whiteboard_mode.unwrap_or(Signal::derive(|| false));
    // Collapsed by default in whiteboard mode
    let (collapsed, set_collapsed) = signal(true);

    let on_toggle_collapse = Callback::new(move |_: ()| {
        set_collapsed.update(|v| *v = !*v);
    });

    // When whiteboard mode toggles, set collapsed accordingly
    Effect::new(move |_| {
        if wb_mode.get() {
            set_collapsed.set(true);
        } else {
            set_collapsed.set(false);
        }
    });

    view! {
        <Show when=move || problem.get().is_some()>
            {move || problem.get().map(|p| {
                let problem_id = p.id.to_string();
                let problem_id_clone = problem_id.clone();
                let answer_type = p.answer_type;
                let answer_hint = p.answer_type.hint();
                let question_latex = p.question_latex.clone();
                let time_limit = p.time_limit_seconds;

                // Clone p for the two branches
                let p_for_wb = p.clone();

                if wb_mode.get() {
                    let collapsed_content = {
                        let question_latex = question_latex.clone();
                        move || view! {
                            <div class="text-sm truncate max-w-md">
                                <LatexRenderer content=question_latex.clone() render_key="collapsed".to_string() />
                            </div>
                        }
                    };

                    view! {
                        <Draggable collapsed=collapsed on_toggle_collapse=on_toggle_collapse>
                            <Show
                                when=move || !collapsed.get()
                                fallback=collapsed_content
                            >
                                <div class="space-y-6">
                                    <ProblemCard key=problem_id_clone.clone() problem=p_for_wb.clone() time_limit_seconds=time_limit />
                                    <AnswerInput
                                        answer_type=answer_type
                                        value=answer
                                        set_value=set_answer
                                        on_submit=on_submit
                                        key=problem_id.clone()
                                    />
                                    {answer_hint.map(|hint| view! {
                                        <p class="text-xs text-gray-400 -mt-4 italic">{hint}</p>
                                    })}
                                    {(render_controls)()}
                                    {(render_result)()}
                                </div>
                            </Show>
                        </Draggable>
                    }.into_any()
                } else {
                    view! {
                        <div class="space-y-6">
                            <ProblemCard key=problem_id_clone problem=p time_limit_seconds=time_limit />
                            <AnswerInput
                                answer_type=answer_type
                                value=answer
                                set_value=set_answer
                                on_submit=on_submit
                                key=problem_id
                            />
                            {answer_hint.map(|hint| view! {
                                <p class="text-xs text-gray-400 -mt-4 italic">{hint}</p>
                            })}
                            {(render_controls)()}
                            {(render_result)()}
                        </div>
                    }.into_any()
                }
            })}
        </Show>
    }
}
