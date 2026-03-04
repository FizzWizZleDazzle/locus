//! AnswerInput — per-type dispatcher that wraps MathField with
//! templates, restrictions, and small UI affordances.

use super::math_field::MathField;
use leptos::html::Div;
use leptos::prelude::*;
use locus_common::AnswerType;
use wasm_bindgen::JsCast;

/// One logical input per problem. Adapts a MathQuill field per `AnswerType`.
#[component]
pub fn AnswerInput(
    answer_type: AnswerType,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    on_submit: Callback<String>,
    #[prop(optional)] key: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let submit_cb = Callback::new(move |()| on_submit.run(value.get()));
    let k = key.unwrap_or_default();

    match answer_type {
        // Boolean — toggle buttons
        AnswerType::Boolean => {
            view! { <BooleanInput set_value on_submit=submit_cb key=k disabled /> }.into_any()
        }

        // Word — plain text input
        AnswerType::Word => {
            view! { <PlainTextInput value set_value on_submit=submit_cb key=k disabled /> }
                .into_any()
        }

        // Interval — MathField + bracket toggle overlays
        AnswerType::Interval => {
            view! { <IntervalInput set_value on_submit=submit_cb key=k disabled /> }.into_any()
        }

        // Inequality — MathField + operator palette
        AnswerType::Inequality => {
            view! { <InequalityInput set_value on_submit=submit_cb key=k disabled /> }.into_any()
        }

        // Matrix — MathField + add/remove row/col buttons
        AnswerType::Matrix => {
            view! { <MatrixInput set_value on_submit=submit_cb key=k disabled /> }.into_any()
        }

        // MultiPart — stacked MathFields
        AnswerType::MultiPart => {
            view! { <MultiPartInput set_value on_submit=submit_cb key=k disabled /> }.into_any()
        }

        // Numeric — restricted input (no letters, no +, *, ^)
        AnswerType::Numeric => {
            view! { <NumericInput set_value on_submit=submit_cb key=k disabled /> }.into_any()
        }

        // Expression, Set, Tuple, List, Equation — template-only variants
        _ => {
            let template = match answer_type {
                AnswerType::Set => "\\left\\{ \\right\\}".to_string(),
                AnswerType::Tuple => "\\left( ,\\right)".to_string(),
                AnswerType::List => "\\left[ ,\\right]".to_string(),
                AnswerType::Equation => "=".to_string(),
                _ => String::new(), // Expression
            };

            view! {
                <MathField
                    set_plain=set_value
                    on_submit=submit_cb
                    template=template
                    key=k
                    disabled=disabled
                />
            }
            .into_any()
        }
    }
}

// ============================================================================
// Plain text input (Word)
// ============================================================================

#[component]
fn PlainTextInput(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    on_submit: Callback<()>,
    #[prop(optional)] key: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <input
            type="text"
            class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded text-lg focus:border-gray-900 dark:focus:border-gray-300 focus:outline-none bg-white dark:bg-gray-800 dark:text-gray-100"
            placeholder="Type your answer"
            prop:value=move || value.get()
            prop:disabled=disabled
            data-key=key.unwrap_or_default()
            on:input=move |ev| {
                let v = event_target_value(&ev);
                set_value.set(v);
            }
            on:keydown=move |ev| {
                if ev.key() == "Enter" {
                    ev.prevent_default();
                    on_submit.run(());
                }
            }
        />
    }
}

// ============================================================================
// Boolean — True / False toggle buttons
// ============================================================================

#[component]
fn BooleanInput(
    set_value: WriteSignal<String>,
    on_submit: Callback<()>,
    #[prop(optional)] key: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let (selected, set_selected) = signal(Option::<bool>::None);
    let _ = (on_submit, key);

    let make_handler = move |val: bool| {
        move |_| {
            if disabled {
                return;
            }
            set_selected.set(Some(val));
            set_value.set(if val {
                "true".to_string()
            } else {
                "false".to_string()
            });
        }
    };

    let btn_class = move |val: bool| {
        let base = "flex-1 px-4 py-3 text-lg font-medium rounded border transition-colors";
        let is_selected = selected.get() == Some(val);
        if is_selected {
            format!(
                "{} bg-gray-900 text-white border-gray-900 dark:bg-gray-100 dark:text-gray-900 dark:border-gray-100",
                base
            )
        } else {
            format!(
                "{} bg-white text-gray-700 border-gray-300 hover:bg-gray-50 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 dark:hover:bg-gray-700",
                base
            )
        }
    };

    view! {
        <div class="flex gap-3">
            <button
                type="button"
                class=move || btn_class(true)
                prop:disabled=disabled
                on:click=make_handler(true)
            >
                "True"
            </button>
            <button
                type="button"
                class=move || btn_class(false)
                prop:disabled=disabled
                on:click=make_handler(false)
            >
                "False"
            </button>
        </div>
    }
}

// ============================================================================
// Numeric — restricted MathField
// ============================================================================

#[component]
fn NumericInput(
    set_value: WriteSignal<String>,
    on_submit: Callback<()>,
    #[prop(optional)] key: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    // Restrict: only digits, decimal, minus, fractions, sqrt, pi.
    // Block variable letters (a-z), +, *, ^.
    let restrict = Callback::new(|latex: String| -> bool {
        if latex.is_empty() {
            return true;
        }

        // Strip known-good LaTeX commands to check remaining chars
        let mut cleaned = latex.clone();
        for cmd in &[
            "\\frac", "\\sqrt", "\\pi", "\\left", "\\right", "\\cdot", "\\times", "\\infty",
        ] {
            cleaned = cleaned.replace(cmd, "");
        }
        cleaned = cleaned.replace('{', "").replace('}', "");
        cleaned = cleaned.replace('(', "").replace(')', "");
        cleaned = cleaned.replace('[', "").replace(']', "");
        cleaned = cleaned.replace(' ', "");

        // Everything remaining should be digits, '.', '-', ','
        cleaned
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == '-' || c == ',')
    });

    view! {
        <MathField
            set_plain=set_value
            on_submit=on_submit
            key=key.unwrap_or_default()
            disabled=disabled
            restrict=restrict
        />
    }
}

// ============================================================================
// Interval — bracket toggle + MathField
// ============================================================================

/// Rewrite interval delimiters in LaTeX based on bracket toggles.
fn rewrite_interval_delimiters(latex: &str, left_closed: bool, right_closed: bool) -> String {
    let mut result = latex.to_string();
    // Replace left delimiter
    if left_closed {
        result = result.replacen("\\left(", "\\left[", 1);
    } else {
        result = result.replacen("\\left[", "\\left(", 1);
    }
    // Replace right delimiter (last occurrence)
    if right_closed {
        if let Some(pos) = result.rfind("\\right)") {
            result.replace_range(pos..pos + 7, "\\right]");
        }
    } else {
        if let Some(pos) = result.rfind("\\right]") {
            result.replace_range(pos..pos + 7, "\\right)");
        }
    }
    result
}

#[component]
fn IntervalInput(
    set_value: WriteSignal<String>,
    on_submit: Callback<()>,
    #[prop(optional)] key: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();
    let (left_closed, set_left_closed) = signal(false);
    let (right_closed, set_right_closed) = signal(false);

    let update_delimiters = move |new_left: bool, new_right: bool| {
        if let Some(container) = container_ref.get_untracked() {
            let container_el: &web_sys::HtmlElement = &container;
            if let Some(mq_span) = container_el
                .query_selector(".mq-editable-field")
                .ok()
                .flatten()
            {
                let mq_span: web_sys::HtmlElement = mq_span.unchecked_into();
                let mq_interface = super::math_field::get_mq_interface();
                if !mq_interface.is_undefined() {
                    if let Ok(mq_fn) = mq_interface.dyn_ref::<js_sys::Function>().ok_or(()) {
                        if let Ok(mq_instance) = mq_fn.call1(&wasm_bindgen::JsValue::NULL, &mq_span)
                        {
                            // Get current LaTeX
                            let latex_fn = js_sys::Reflect::get(&mq_instance, &"latex".into())
                                .unwrap_or_default();
                            if let Ok(func) = latex_fn.dyn_into::<js_sys::Function>() {
                                if let Ok(result) = func.call0(&mq_instance) {
                                    let current = result.as_string().unwrap_or_default();
                                    let updated =
                                        rewrite_interval_delimiters(&current, new_left, new_right);
                                    // Set updated LaTeX
                                    let set_fn =
                                        js_sys::Reflect::get(&mq_instance, &"latex".into())
                                            .unwrap_or_default();
                                    if let Ok(func) = set_fn.dyn_into::<js_sys::Function>() {
                                        let _ = func.call1(
                                            &mq_instance,
                                            &wasm_bindgen::JsValue::from_str(&updated),
                                        );
                                    }
                                    // Re-focus
                                    let focus_fn =
                                        js_sys::Reflect::get(&mq_instance, &"focus".into())
                                            .unwrap_or_default();
                                    if let Ok(func) = focus_fn.dyn_into::<js_sys::Function>() {
                                        let _ = func.call0(&mq_instance);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    let toggle_left = move |_| {
        let new_val = !left_closed.get_untracked();
        set_left_closed.set(new_val);
        update_delimiters(new_val, right_closed.get_untracked());
    };

    let toggle_right = move |_| {
        let new_val = !right_closed.get_untracked();
        set_right_closed.set(new_val);
        update_delimiters(left_closed.get_untracked(), new_val);
    };

    let left_label = move || if left_closed.get() { "[" } else { "(" };
    let right_label = move || if right_closed.get() { "]" } else { ")" };

    let bracket_btn = "px-3 py-2 text-lg font-mono border border-gray-300 dark:border-gray-600 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors select-none";

    view! {
        <div node_ref=container_ref>
            <div class="flex items-stretch gap-1">
                <button
                    type="button"
                    class=bracket_btn
                    prop:disabled=disabled
                    on:click=toggle_left
                    title="Toggle open/closed left bracket"
                >
                    {left_label}
                </button>
                <div class="flex-1">
                    <MathField
                        set_plain=set_value
                        on_submit=on_submit
                        template="\\left( ,\\right)".to_string()
                        key=key.unwrap_or_default()
                        disabled=disabled
                    />
                </div>
                <button
                    type="button"
                    class=bracket_btn
                    prop:disabled=disabled
                    on:click=toggle_right
                    title="Toggle open/closed right bracket"
                >
                    {right_label}
                </button>
            </div>
        </div>
    }
}

// ============================================================================
// Inequality — operator palette
// ============================================================================

#[component]
fn InequalityInput(
    set_value: WriteSignal<String>,
    on_submit: Callback<()>,
    #[prop(optional)] key: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();

    let insert_latex = move |latex_cmd: &'static str| {
        if let Some(container) = container_ref.get_untracked() {
            let container_el: &web_sys::HtmlElement = &container;
            if let Some(mq_span) = container_el
                .query_selector(".mq-editable-field")
                .ok()
                .flatten()
            {
                let mq_span: web_sys::HtmlElement = mq_span.unchecked_into();
                // MQ(span) retrieves the existing MathQuill instance
                let mq_interface = super::math_field::get_mq_interface();
                if !mq_interface.is_undefined() {
                    if let Ok(mq_fn) = mq_interface.dyn_ref::<js_sys::Function>().ok_or(()) {
                        if let Ok(mq_instance) = mq_fn.call1(&wasm_bindgen::JsValue::NULL, &mq_span)
                        {
                            // .write(latex) inserts LaTeX at cursor
                            let write_fn = js_sys::Reflect::get(&mq_instance, &"write".into())
                                .unwrap_or_default();
                            if let Ok(func) = write_fn.dyn_into::<js_sys::Function>() {
                                let _ = func.call1(
                                    &mq_instance,
                                    &wasm_bindgen::JsValue::from_str(latex_cmd),
                                );
                            }
                            // Re-focus the field
                            let focus_fn = js_sys::Reflect::get(&mq_instance, &"focus".into())
                                .unwrap_or_default();
                            if let Ok(func) = focus_fn.dyn_into::<js_sys::Function>() {
                                let _ = func.call0(&mq_instance);
                            }
                        }
                    }
                }
            }
        }
    };

    view! {
        <div node_ref=container_ref>
            <MathField
                set_plain=set_value
                on_submit=on_submit
                key=key.unwrap_or_default()
                disabled=disabled
            />
            <div class="flex gap-2 mt-1">
                <button
                    type="button"
                    class="px-3 py-1 text-sm border border-gray-300 rounded hover:bg-gray-100 font-mono transition-colors"
                    on:click=move |_| insert_latex("\\gt ")
                >
                    ">"
                </button>
                <button
                    type="button"
                    class="px-3 py-1 text-sm border border-gray-300 rounded hover:bg-gray-100 font-mono transition-colors"
                    on:click=move |_| insert_latex("\\ge ")
                >
                    "\u{2265}"
                </button>
                <button
                    type="button"
                    class="px-3 py-1 text-sm border border-gray-300 rounded hover:bg-gray-100 font-mono transition-colors"
                    on:click=move |_| insert_latex("\\lt ")
                >
                    "<"
                </button>
                <button
                    type="button"
                    class="px-3 py-1 text-sm border border-gray-300 rounded hover:bg-gray-100 font-mono transition-colors"
                    on:click=move |_| insert_latex("\\le ")
                >
                    "\u{2264}"
                </button>
            </div>
        </div>
    }
}

// ============================================================================
// Matrix — template + add/remove row/col
// ============================================================================

/// Generate a MathQuill LaTeX template for an NxM matrix.
fn matrix_template(rows: u32, cols: u32) -> String {
    let row: String = (0..cols).map(|_| " ").collect::<Vec<_>>().join("& ");
    let rows_str: String = (0..rows)
        .map(|_| row.as_str())
        .collect::<Vec<_>>()
        .join("\\\\ ");
    format!("\\begin{{pmatrix}} {} \\end{{pmatrix}}", rows_str)
}

#[component]
fn MatrixInput(
    set_value: WriteSignal<String>,
    on_submit: Callback<()>,
    #[prop(optional)] key: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let (rows, set_rows) = signal(2u32);
    let (cols, set_cols) = signal(2u32);

    // Force remount key includes dimensions
    let field_key = Memo::new(move |_| {
        format!(
            "{}-{}x{}",
            key.as_deref().unwrap_or("m"),
            rows.get(),
            cols.get()
        )
    });

    view! {
        <div>
            {move || {
                let k = field_key.get();
                let r = rows.get();
                let c = cols.get();
                let tmpl = matrix_template(r, c);
                view! {
                    <MathField
                        set_plain=set_value
                        on_submit=on_submit
                        template=tmpl
                        key=k
                        disabled=disabled
                    />
                }
            }}
            <div class="flex gap-2 mt-1">
                <button
                    type="button"
                    class="px-3 py-1 text-xs border border-gray-300 rounded hover:bg-gray-100 transition-colors"
                    on:click=move |_| set_rows.update(|r| *r += 1)
                >
                    "+Row"
                </button>
                <button
                    type="button"
                    class="px-3 py-1 text-xs border border-gray-300 rounded hover:bg-gray-100 transition-colors"
                    on:click=move |_| set_rows.update(|r| if *r > 1 { *r -= 1 })
                >
                    "-Row"
                </button>
                <button
                    type="button"
                    class="px-3 py-1 text-xs border border-gray-300 rounded hover:bg-gray-100 transition-colors"
                    on:click=move |_| set_cols.update(|c| *c += 1)
                >
                    "+Col"
                </button>
                <button
                    type="button"
                    class="px-3 py-1 text-xs border border-gray-300 rounded hover:bg-gray-100 transition-colors"
                    on:click=move |_| set_cols.update(|c| if *c > 1 { *c -= 1 })
                >
                    "-Col"
                </button>
                <span class="text-xs text-gray-400 self-center">
                    {move || format!("{}x{}", rows.get(), cols.get())}
                </span>
            </div>
        </div>
    }
}

// ============================================================================
// MultiPart — stacked fields
// ============================================================================

#[component]
fn MultiPartInput(
    set_value: WriteSignal<String>,
    on_submit: Callback<()>,
    #[prop(optional)] key: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let (part_count, set_part_count) = signal(2usize);
    let (parts, set_parts) = signal(vec![String::new(), String::new()]);

    // Assemble parts into "p1|||p2|||p3" on change
    Effect::new(move |_| {
        let assembled = parts.get().join("|||");
        set_value.set(assembled);
    });

    view! {
        <div class="space-y-3">
            {move || {
                let count = part_count.get();
                (0..count)
                    .map(|i| {
                        let part_key =
                            format!("{}-part{}", key.as_deref().unwrap_or("mp"), i);

                        // Each part gets its own signal pair
                        let (part_read, set_part_signal) = signal(String::new());

                        // Forward this part's value to the parts vec
                        Effect::new(move |_| {
                            let val = part_read.get();
                            set_parts.update(|p| {
                                if i < p.len() {
                                    p[i] = val;
                                }
                            });
                        });

                        view! {
                            <div class="flex items-start gap-2">
                                <span class="text-xs text-gray-400 pt-3 min-w-[48px]">
                                    {format!("Part {}:", i + 1)}
                                </span>
                                <div class="flex-1">
                                    <MathField
                                        set_plain=set_part_signal
                                        on_submit=on_submit
                                        key=part_key
                                        disabled=disabled
                                    />
                                </div>
                            </div>
                        }
                    })
                    .collect_view()
            }}
            <button
                type="button"
                class="px-3 py-1 text-xs border border-gray-300 rounded hover:bg-gray-100 transition-colors"
                on:click=move |_| {
                    set_part_count.update(|c| *c += 1);
                    set_parts.update(|p| p.push(String::new()));
                }
            >
                "+ Add Part"
            </button>
        </div>
    }
}
