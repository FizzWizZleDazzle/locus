//! Math input component with inline rendering (like Desmos)

use leptos::prelude::*;
use leptos::html::Div;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, Document, Window};
use crate::grader::preprocess_input;

#[component]
pub fn MathInput(
    #[prop(optional)]
    key: Option<String>,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(default = "Enter your answer...")]
    placeholder: &'static str,
    #[prop(default = false)]
    disabled: bool,
    #[prop(optional)]
    on_submit: Option<Callback<()>>,
) -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();
    let initialized = StoredValue::new(false);

    // Cleanup MathLive when component unmounts
    on_cleanup(move || {
        if let Some(elem) = container_ref.get_untracked() {
            elem.set_inner_html("");  // Destroys MathLive instance
        }
    });

    // Initialize the math field once
    Effect::new(move |_| {
        // Only run once - use get_value() for StoredValue, not get()
        if initialized.get_value() {
            return;
        }

        if let Some(container) = container_ref.get() {
            // Mark as initialized
            initialized.set_value(true);

            let window: Window = web_sys::window().expect("no window");
            let document: Document = window.document().expect("no document");

            // Create math-field element
            let math_field = document
                .create_element("math-field")
                .expect("failed to create math-field");

            let math_field_html = math_field.unchecked_into::<HtmlElement>();

            // Configure MathLive options
            let _ = math_field_html.set_attribute(
                "style",
                "font-size: 18px; \
                 padding: 12px; \
                 border: 1px solid #d1d5db; \
                 border-radius: 4px; \
                 min-height: 48px; \
                 width: 100%;"
            );

            // Disable virtual keyboard to avoid conflicts with typing
            let _ = math_field_html.set_attribute("virtual-keyboard-mode", "off");

            if disabled {
                let _ = math_field_html.set_attribute("disabled", "true");
            }

            // Append to container
            let _ = container.append_child(&math_field_html);

            // Configure MathLive options via JavaScript
            use wasm_bindgen::JsValue;

            // Disable virtual keyboard
            let _ = js_sys::Reflect::set(
                &math_field_html,
                &JsValue::from_str("mathVirtualKeyboardPolicy"),
                &JsValue::from_str("manual")
            );

            // Set an empty menu to hide the menu button
            let empty_array = js_sys::Array::new();
            let _ = js_sys::Reflect::set(
                &math_field_html,
                &JsValue::from_str("menuItems"),
                &empty_array
            );

            // Configure inline shortcuts - keep functions, remove problematic conversions
            let shortcuts = js_sys::Object::new();

            // Trig functions
            let _ = js_sys::Reflect::set(&shortcuts, &"sin".into(), &"\\sin".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"cos".into(), &"\\cos".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"tan".into(), &"\\tan".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"sec".into(), &"\\sec".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"csc".into(), &"\\csc".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"cot".into(), &"\\cot".into());

            // Inverse trig
            let _ = js_sys::Reflect::set(&shortcuts, &"arcsin".into(), &"\\arcsin".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"arccos".into(), &"\\arccos".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"arctan".into(), &"\\arctan".into());

            // Hyperbolic
            let _ = js_sys::Reflect::set(&shortcuts, &"sinh".into(), &"\\sinh".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"cosh".into(), &"\\cosh".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"tanh".into(), &"\\tanh".into());

            // Log functions
            let _ = js_sys::Reflect::set(&shortcuts, &"log".into(), &"\\log".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"ln".into(), &"\\ln".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"exp".into(), &"\\exp".into());

            // Other math functions
            let _ = js_sys::Reflect::set(&shortcuts, &"sqrt".into(), &"\\sqrt".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"lim".into(), &"\\lim".into());

            // Common symbols
            let _ = js_sys::Reflect::set(&shortcuts, &"pi".into(), &"\\pi".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"theta".into(), &"\\theta".into());
            let _ = js_sys::Reflect::set(&shortcuts, &"infty".into(), &"\\infty".into());

            let _ = js_sys::Reflect::set(
                &math_field_html,
                &JsValue::from_str("inlineShortcuts"),
                &shortcuts
            );

            // Set up input event listener
            let set_value_clone = set_value;
            let math_field_clone = math_field_html.clone();
            let input_closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                let new_value = get_math_field_json(&math_field_clone);
                set_value_clone.set(new_value);
            }) as Box<dyn FnMut(_)>);

            let _ = math_field_html.add_event_listener_with_callback(
                "input",
                input_closure.as_ref().unchecked_ref()
            );
            input_closure.forget();

            // Set up Enter key listener
            if let Some(callback) = on_submit {
                let keydown_closure = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                    if e.key() == "Enter" {
                        e.prevent_default();
                        callback.run(());
                    }
                }) as Box<dyn FnMut(_)>);

                let _ = math_field_html.add_event_listener_with_callback(
                    "keydown",
                    keydown_closure.as_ref().unchecked_ref()
                );
                keydown_closure.forget();
            }

            // Set initial value WITHOUT tracking
            let initial = value.get_untracked();
            if !initial.is_empty() {
                set_math_field_value(&math_field_html, &initial);
            }

            // Focus the field initially
            let _ = math_field_html.focus();
        }
    });

    // Separate Effect to sync Leptos signal -> MathLive field
    Effect::new(move |_| {
        let new_value = value.get(); // Track value changes reactively

        // Don't track the container - just check if it exists
        if let Some(container) = container_ref.get_untracked() {
            if let Some(math_field) = container.first_child() {
                let math_field_html = math_field.unchecked_into::<HtmlElement>();

                // Check if MathLive is initialized (has setValue method)
                if !is_mathlive_ready(&math_field_html) {
                    return; // MathLive not ready yet, skip this update
                }

                // Get current value from MathLive
                let current = get_math_field_value(&math_field_html);

                // Only update if different (prevents circular updates)
                if current != new_value {
                    // Use safe setter that checks if setValue exists
                    let _ = try_set_math_field_value(&math_field_html, &new_value);
                }
            }
        }
    });

    let processed_value = move || preprocess_input(&value.get());

    view! {
        <div>
            <div
                node_ref=container_ref
                class="math-input-container"
                data-key=key.unwrap_or_default()
            />

            // Show parsed value if different (debug mode only)
            #[cfg(debug_assertions)]
            {move || {
                let raw = value.get();
                let processed = processed_value();
                (!raw.is_empty() && raw != processed).then(|| view! {
                    <div class="mt-2 text-xs text-gray-500">
                        "Parsed as: " <code class="bg-gray-100 px-1 py-0.5 rounded">{processed}</code>
                    </div>
                })
            }}
        </div>
    }
}

// Helper functions to interact with math-field

/// Check if MathLive is fully initialized and ready
fn is_mathlive_ready(element: &HtmlElement) -> bool {
    use wasm_bindgen::JsValue;

    // Check if element has setValue method (indicates MathLive is ready)
    if let Ok(set_value_fn) = js_sys::Reflect::get(element, &JsValue::from_str("setValue")) {
        !set_value_fn.is_undefined() && !set_value_fn.is_null()
    } else {
        false
    }
}

fn get_math_field_value(element: &HtmlElement) -> String {
    use wasm_bindgen::JsValue;
    let value = js_sys::Reflect::get(element, &JsValue::from_str("value"))
        .unwrap_or(JsValue::from_str(""));
    value.as_string().unwrap_or_default()
}

fn get_math_field_json(element: &HtmlElement) -> String {
    use wasm_bindgen::JsValue;

    // Try to get MathJSON via getValue('math-json')
    if let Ok(get_value_fn) = js_sys::Reflect::get(element, &JsValue::from_str("getValue")) {
        if let Ok(func) = get_value_fn.dyn_into::<js_sys::Function>() {
            if let Ok(result) = func.call1(element, &JsValue::from_str("math-json")) {
                if let Some(json_str) = result.as_string() {
                    // Check if it's an error response
                    if !json_str.contains("Error") && !json_str.contains("not-available") {
                        return json_str;
                    }
                }
            }
        }
    }

    // Fallback to LaTeX if MathJSON not available
    let value = js_sys::Reflect::get(element, &JsValue::from_str("value"))
        .unwrap_or(JsValue::from_str(""));
    value.as_string().unwrap_or_default()
}

/// Safely set MathLive field value, returns true if successful
fn try_set_math_field_value(element: &HtmlElement, value: &str) -> bool {
    use wasm_bindgen::JsValue;

    // Get setValue method
    let set_value_fn = match js_sys::Reflect::get(element, &JsValue::from_str("setValue")) {
        Ok(fn_val) if !fn_val.is_undefined() && !fn_val.is_null() => fn_val,
        _ => return false, // setValue not available
    };

    // Convert to function
    let set_value_fn = match set_value_fn.dyn_into::<js_sys::Function>() {
        Ok(func) => func,
        Err(_) => return false, // Not a function
    };

    // Call the function
    set_value_fn.call1(element, &JsValue::from_str(value)).is_ok()
}

fn set_math_field_value(element: &HtmlElement, value: &str) {
    let _ = try_set_math_field_value(element, value);
}
