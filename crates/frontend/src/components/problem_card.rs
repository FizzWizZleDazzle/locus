//! Problem card component

use leptos::prelude::*;
use locus_common::ProblemResponse;
use crate::katex_bindings::render_plain_math_to_string;
use crate::components::LatexRenderer;

#[component]
pub fn ProblemCard(
    problem: ProblemResponse,
    #[prop(default = None)]
    show_answer: Option<String>,
    #[prop(optional)]
    key: Option<String>,
) -> impl IntoView {
    let _ = key; // Suppress unused warning - key is used by Leptos for identity
    view! {
        <div class="border p-6">
            <div class="flex justify-between text-sm text-gray-500 mb-4">
                <span>{locus_common::subtopic_display_name(&problem.subtopic)}</span>
                <span>{format!("Difficulty: {}", problem.difficulty)}</span>
            </div>

            <div class="text-xl text-center py-4">
                <LatexRenderer
                    content=problem.question_latex.clone()
                    render_key=key.clone().unwrap_or_else(|| problem.id.to_string())
                />
            </div>

            {show_answer.map(|answer| {
                // Convert plain math notation to rendered LaTeX
                let rendered = render_plain_math_to_string(&answer)
                    .unwrap_or_else(|_| format!("<code>{}</code>", answer));

                view! {
                    <div class="mt-4 pt-4 border-t">
                        <span class="text-sm text-gray-500">"Answer: "</span>
                        <span class="text-lg ml-2" inner_html=rendered></span>
                    </div>
                }
            })}
        </div>
    }
}
