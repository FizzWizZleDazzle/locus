//! Problem card component

use leptos::prelude::*;
use locus_common::ProblemResponse;

#[component]
pub fn ProblemCard(
    problem: ProblemResponse,
    #[prop(default = None)]
    show_answer: Option<String>,
) -> impl IntoView {
    view! {
        <div class="border p-6">
            <div class="flex justify-between text-sm text-gray-500 mb-4">
                <span>{locus_common::subtopic_display_name(&problem.subtopic)}</span>
                <span>{format!("Difficulty: {}", problem.difficulty)}</span>
            </div>

            <div class="text-xl text-center py-4">
                <span inner_html=problem.question_latex.clone()></span>
            </div>

            {show_answer.map(|answer| view! {
                <div class="mt-4 pt-4 border-t text-sm">
                    <span class="text-gray-500">"Answer: "</span>
                    <code>{answer}</code>
                </div>
            })}
        </div>
    }
}
