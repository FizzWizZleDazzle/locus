use leptos::prelude::*;
use locus_common::ProblemResponse;
use crate::components::{MathInput, ProblemCard};

/// Unified problem display component used by both Practice and Ranked modes.
///
/// This component encapsulates:
/// - Problem rendering (ProblemCard)
/// - Answer input (MathInput with forced remounting via key prop)
/// - Mode-specific behavior via callback props
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
) -> impl IntoView
where
    ControlsView: Fn() -> ControlsViewOutput + Copy + Send + Sync + 'static,
    ControlsViewOutput: IntoView + 'static,
    ResultView: Fn() -> ResultViewOutput + Copy + Send + Sync + 'static,
    ResultViewOutput: IntoView + 'static,
{
    view! {
        <Show when=move || problem.get().is_some()>
            {move || problem.get().map(|p| {
                let problem_id = p.id.to_string();
                let problem_id_clone = problem_id.clone();
                view! {
                    <div class="space-y-6">
                        // Problem card with key for forced remounting
                        // CRITICAL: key={problem.id} ensures complete destruction/recreation
                        <ProblemCard key=problem_id_clone problem=p.clone() />

                        // Answer input with forced remounting via key
                        // CRITICAL: key={problem.id} ensures complete destruction/recreation
                        <MathInput
                            key=problem_id
                            value=answer
                            set_value=set_answer
                            on_submit=Callback::new(move |_| on_submit.run(answer.get()))
                            placeholder="Your answer"
                        />

                        // Mode-specific controls (passed as prop)
                        {(render_controls)()}

                        // Mode-specific result display (passed as prop)
                        {(render_result)()}
                    </div>
                }
            })}
        </Show>
    }
}
