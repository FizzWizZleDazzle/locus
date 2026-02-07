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
