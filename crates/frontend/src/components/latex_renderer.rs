//! Modular LaTeX renderer that handles multiple input formats

use leptos::prelude::*;
use leptos::html::Div;
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
            // Determine format and prepare content
            let processed = prepare_latex_content(&content_clone);

            // Set content
            element.set_inner_html(&processed);

            // Apply KaTeX auto-render
            let options = js_sys::JSON::parse(r#"{
                "delimiters": [
                    {"left": "$$", "right": "$$", "display": true},
                    {"left": "$", "right": "$", "display": false}
                ],
                "throwOnError": false
            }"#).unwrap();

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

/// Prepare LaTeX content for rendering based on its format
fn prepare_latex_content(content: &str) -> String {
    let trimmed = content.trim();

    // Already has delimiters - use as-is (most common case)
    if trimmed.contains('$') {
        return trimmed.to_string();
    }

    // Check if it's pure LaTeX (no spaces/text before first backslash)
    // Examples: "\frac{1}{2}", "\text{Factor: } x^2"
    if is_pure_latex(trimmed) {
        return format!("${}$", trimmed);
    }

    // Plain text word problem - no math delimiters
    trimmed.to_string()
}

/// Detect if content is pure LaTeX (no mixed text)
/// Returns true only if content starts with LaTeX and has no plain text before it
fn is_pure_latex(content: &str) -> bool {
    let trimmed = content.trim();

    // Must start with backslash for pure LaTeX
    // Examples: "\frac{1}{2}", "\text{Find: } x"
    // NOT: "Simplify: \frac{1}{2}" (this is mixed)
    trimmed.starts_with('\\')
}
