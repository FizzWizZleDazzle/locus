//! MathQuill wrapper component — atomic, reusable math input field.
//!
//! Wraps a MathQuill MathField instance with Leptos signals.
//! On every edit, the LaTeX is converted to plain text via `convert_latex_to_plain()`
//! and pushed to the `set_plain` writer.

use leptos::html::Div;
use leptos::prelude::*;
use locus_common::latex::convert_latex_to_plain;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

/// A single MathQuill editable math field.
///
/// - `set_plain`: receives **plain text** output on every edit (via `convert_latex_to_plain`).
/// - `template`: initial LaTeX to pre-seed (e.g. `"\\left\\{ \\right\\}"`).
/// - `on_submit`: fired on Enter key.
/// - `restrict`: optional closure that receives the current LaTeX after each edit.
///    Return `true` to accept the edit, `false` to revert to the previous value.
#[component]
pub fn MathField(
    /// Write plain-text output on every edit
    set_plain: WriteSignal<String>,
    /// Called on Enter key
    #[prop(optional)]
    on_submit: Option<Callback<()>>,
    /// Initial LaTeX template (e.g., "\\left\\{ \\right\\}")
    #[prop(default = String::new())]
    template: String,
    /// Key for forced remounting (problem ID)
    #[prop(optional)]
    key: Option<String>,
    #[prop(default = false)] disabled: bool,
    /// Optional restriction: returns false to reject the edit (reverts to previous LaTeX)
    #[prop(optional)]
    restrict: Option<Callback<String, bool>>,
) -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();
    let initialized = StoredValue::new(false);

    // Cleanup MathQuill when component unmounts
    on_cleanup(move || {
        if let Some(elem) = container_ref.get_untracked() {
            elem.set_inner_html("");
        }
    });

    Effect::new(move |_| {
        if initialized.get_value() {
            return;
        }

        let Some(container) = container_ref.get() else {
            return;
        };
        initialized.set_value(true);

        let window = web_sys::window().expect("no window");
        let document = window.document().expect("no document");

        // Create the <span> that MathQuill will mount on
        let span = document
            .create_element("span")
            .expect("create span")
            .unchecked_into::<HtmlElement>();
        let _ = container.append_child(&span);

        // Get MQ interface: var MQ = MathQuill.getInterface(2);
        let mq_interface = get_mq_interface();
        if mq_interface.is_undefined() || mq_interface.is_null() {
            web_sys::console::error_1(&"MathQuill not loaded".into());
            return;
        }

        // Build config object
        let config = js_sys::Object::new();

        // Spacebar should just type a space (MathQuill default behavior)
        let _ = js_sys::Reflect::set(
            &config,
            &"spaceBehavesLikeTab".into(),
            &wasm_bindgen::JsValue::FALSE,
        );

        // --- edit handler ---
        let set_plain_clone = set_plain;
        let restrict_clone = restrict;
        // We use a JS-side variable to track the last good LaTeX for restriction reverts
        let prev_latex = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        let prev_latex_edit = prev_latex.clone();

        let edit_closure = Closure::wrap(Box::new(move |mq_field: wasm_bindgen::JsValue| {
            let latex = get_latex(&mq_field);

            // If restriction function provided, check it
            if let Some(ref restrict_fn) = restrict_clone {
                if !restrict_fn.run(latex.clone()) {
                    // Revert to previous good value
                    let prev = prev_latex_edit.borrow().clone();
                    set_latex(&mq_field, &prev);
                    return;
                }
            }

            *prev_latex_edit.borrow_mut() = latex.clone();
            let plain = convert_latex_to_plain(&latex);
            set_plain_clone.set(plain);
        }) as Box<dyn FnMut(wasm_bindgen::JsValue)>);

        let handlers = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&handlers, &"edit".into(), edit_closure.as_ref());
        edit_closure.forget();

        // --- enter handler ---
        if let Some(on_submit_cb) = on_submit {
            let enter_closure = Closure::wrap(Box::new(move |_mq_field: wasm_bindgen::JsValue| {
                on_submit_cb.run(());
            })
                as Box<dyn FnMut(wasm_bindgen::JsValue)>);
            let _ = js_sys::Reflect::set(&handlers, &"enter".into(), enter_closure.as_ref());
            enter_closure.forget();
        }

        let _ = js_sys::Reflect::set(&config, &"handlers".into(), &handlers);

        // Create MathField: MQ.MathField(span, config)
        let math_field_fn =
            js_sys::Reflect::get(&mq_interface, &"MathField".into()).unwrap_or_default();
        let math_field_fn: js_sys::Function = math_field_fn.unchecked_into();
        let mq_instance = math_field_fn
            .call2(&mq_interface, &span, &config)
            .unwrap_or_default();

        // Pre-seed template
        if !template.is_empty() {
            set_latex(&mq_instance, &template);
            *prev_latex.borrow_mut() = template.clone();
            // Also emit the initial plain text
            let plain = convert_latex_to_plain(&template);
            set_plain.set(plain);
        }

        // Focus the field
        let _ = call_method(&mq_instance, "focus");

        // Disable if needed
        if disabled {
            let _ = call_method(&mq_instance, "blur");
            // MathQuill doesn't have a disable API; we make the container inert
            let _ = container.set_attribute("style", "pointer-events:none;opacity:0.5;");
        }
    });

    view! {
        <div
            node_ref=container_ref
            class="mathquill-container"
            data-key=key.unwrap_or_default()
        />
    }
}

// ============================================================================
// JS interop helpers
// ============================================================================

/// Get MathQuill interface: `MathQuill.getInterface(2)`
pub(crate) fn get_mq_interface() -> wasm_bindgen::JsValue {
    let window = web_sys::window().unwrap();
    let mq_global = js_sys::Reflect::get(&window, &"MathQuill".into()).unwrap_or_default();
    if mq_global.is_undefined() || mq_global.is_null() {
        return wasm_bindgen::JsValue::UNDEFINED;
    }
    let get_interface =
        js_sys::Reflect::get(&mq_global, &"getInterface".into()).unwrap_or_default();
    if let Ok(func) = get_interface.dyn_into::<js_sys::Function>() {
        func.call1(&mq_global, &wasm_bindgen::JsValue::from_f64(2.0))
            .unwrap_or_default()
    } else {
        wasm_bindgen::JsValue::UNDEFINED
    }
}

/// `mqField.latex()` — get current LaTeX
fn get_latex(mq_field: &wasm_bindgen::JsValue) -> String {
    let latex_fn = js_sys::Reflect::get(mq_field, &"latex".into()).unwrap_or_default();
    if let Ok(func) = latex_fn.dyn_into::<js_sys::Function>() {
        if let Ok(result) = func.call0(mq_field) {
            return result.as_string().unwrap_or_default();
        }
    }
    String::new()
}

/// `mqField.latex(value)` — set LaTeX
fn set_latex(mq_field: &wasm_bindgen::JsValue, value: &str) {
    let latex_fn = js_sys::Reflect::get(mq_field, &"latex".into()).unwrap_or_default();
    if let Ok(func) = latex_fn.dyn_into::<js_sys::Function>() {
        let _ = func.call1(mq_field, &wasm_bindgen::JsValue::from_str(value));
    }
}

/// Call a no-arg method on a MathQuill instance
fn call_method(mq_field: &wasm_bindgen::JsValue, method: &str) -> wasm_bindgen::JsValue {
    let func_val = js_sys::Reflect::get(mq_field, &method.into()).unwrap_or_default();
    if let Ok(func) = func_val.dyn_into::<js_sys::Function>() {
        func.call0(mq_field).unwrap_or_default()
    } else {
        wasm_bindgen::JsValue::UNDEFINED
    }
}
