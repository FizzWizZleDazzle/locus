//! Modular LaTeX renderer that handles multiple input formats

use leptos::html::Div;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = renderMathInElement)]
    fn render_math_in_element(element: &web_sys::Element, options: &JsValue);
}

/// Smart LaTeX renderer component that handles:
/// - Pure LaTeX (wraps in $ if needed)
/// - Mixed text/math (processes $ delimiters)
/// - Plain text (no processing)
#[component]
pub fn LatexRenderer(
    /// The LaTeX/text content to render
    content: String,
    /// Unique key to force re-render when content changes
    #[prop(optional)]
    render_key: Option<String>,
) -> impl IntoView {
    let content_ref = NodeRef::<Div>::new();
    let content_clone = content.clone();

    // Re-render when content changes
    Effect::new(move |_| {
        if let Some(element) = content_ref.get() {
            // Prepare content using shared validation/fix logic
            let processed = locus_common::katex_validate::prepare_for_rendering(&content_clone);

            // Set content
            element.set_inner_html(&processed);

            // Apply KaTeX auto-render
            let options = js_sys::JSON::parse(
                r#"{
                "delimiters": [
                    {"left": "$$", "right": "$$", "display": true},
                    {"left": "$", "right": "$", "display": false},
                    {"left": "\\(", "right": "\\)", "display": false},
                    {"left": "\\[", "right": "\\]", "display": true}
                ],
                "throwOnError": false
            }"#,
            )
            .unwrap();

            render_math_in_element(&element, &options);
        }
    });

    view! {
        <div
            node_ref=content_ref
            data-render-key={render_key.unwrap_or_default()}
        ></div>
    }
}
