//! KaTeX JavaScript bindings for rendering LaTeX in WASM

use wasm_bindgen::prelude::*;

/// External binding to KaTeX's render function
/// Renders LaTeX to a DOM element
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = katex, catch)]
    pub fn render(tex: &str, element: &web_sys::Element, options: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(js_namespace = katex, catch)]
    pub fn renderToString(tex: &str, options: &JsValue) -> Result<String, JsValue>;
}

/// External binding to Nerdamer symbolic math library
#[wasm_bindgen]
extern "C" {
    /// Nerdamer function - parses and evaluates symbolic math expressions
    #[wasm_bindgen(js_name = nerdamer, catch)]
    pub fn nerdamer_parse(expression: &str) -> Result<NerdamerExpression, JsValue>;

    /// Nerdamer expression type
    pub type NerdamerExpression;

    /// Convert a Nerdamer expression to LaTeX string
    #[wasm_bindgen(method, js_name = toTeX, catch)]
    pub fn to_tex(this: &NerdamerExpression) -> Result<String, JsValue>;
}

/// External binding to JSON.parse for creating options object
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = JSON, js_name = parse)]
    pub fn json_parse(s: &str) -> JsValue;
}

/// Safely render LaTeX to a DOM element
/// Catches errors and fails gracefully
pub fn render_math_safe(tex: &str, element: &web_sys::Element, display_mode: bool) {
    let options = if display_mode {
        json_parse(r#"{"throwOnError": false, "displayMode": true}"#)
    } else {
        json_parse(r#"{"throwOnError": false, "displayMode": false}"#)
    };

    if let Err(_e) = render(tex, element, &options) {
        // On error, display the raw LaTeX with an error indicator
        let error_html = format!(
            r#"<span style="color: #dc2626; font-family: monospace;">Error: {}</span>"#,
            tex
        );
        element.set_inner_html(&error_html);
    }
}

/// Render LaTeX to an HTML string
pub fn render_math_to_string(tex: &str, display_mode: bool) -> Result<String, String> {
    let options = if display_mode {
        json_parse(r#"{"throwOnError": false, "displayMode": true}"#)
    } else {
        json_parse(r#"{"throwOnError": false, "displayMode": false}"#)
    };

    renderToString(tex, &options)
        .map_err(|e| format!("KaTeX error: {:?}", e))
}

/// Convert plain math notation (SymPy format) to LaTeX and render it
/// Uses Nerdamer library to parse plain math like "sin(x)" or "10*x + 3"
/// and convert it to proper LaTeX format
pub fn render_plain_math_to_string(plain_math: &str) -> Result<String, String> {
    // First, convert plain math notation to LaTeX using Nerdamer
    let expr = nerdamer_parse(plain_math)
        .map_err(|e| format!("Failed to parse math expression: {:?}", e))?;

    let mut latex = expr.to_tex()
        .map_err(|e| format!("Failed to convert to LaTeX: {:?}", e))?;

    // Remove explicit multiplication symbols for cleaner display
    // Nerdamer outputs things like "2 \cdot x" but we want "2x"
    latex = latex.replace(r" \cdot ", "");

    // Then render the LaTeX with KaTeX
    render_math_to_string(&latex, false)
}
