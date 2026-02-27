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
        // Boolean/Word — plain text input, no MathQuill
        AnswerType::Boolean | AnswerType::Word => {
            view! { <PlainTextInput value set_value on_submit=submit_cb key=k disabled answer_type /> }
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

        // Matrix — MathField + add row/col buttons
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
                AnswerType::Set => "\\left\\{ \\right\\}",
                AnswerType::Tuple => "\\left( ,\\right)",
                AnswerType::List => "\\left[ ,\\right]",
                AnswerType::Equation => "=",
                _ => "", // Expression
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
// Plain text input (Boolean / Word)
// ============================================================================

#[component]
fn PlainTextInput(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    on_submit: Callback<()>,
    #[prop(optional)] key: Option<String>,
    #[prop(default = false)] disabled: bool,
    answer_type: AnswerType,
) -> impl IntoView {
    let placeholder = match answer_type {
        AnswerType::Boolean => "true or false",
        AnswerType::Word => "Type your answer",
        _ => "Your answer",
    };

    view! {
        <input
            type="text"
            class="w-full px-4 py-3 border border-gray-300 rounded text-lg focus:border-gray-900 focus:outline-none"
            placeholder=placeholder
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
// Interval — bracket toggle overlays
// ============================================================================

#[component]
fn IntervalInput(
    set_value: WriteSignal<String>,
    on_submit: Callback<()>,
    #[prop(optional)] key: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let (left_closed, set_left_closed) = signal(false); // ( by default
    let (right_closed, set_right_closed) = signal(false); // ) by default

    // Plain text from MathField (includes template delimiters after conversion)
    let (inner_plain, set_inner_plain) = signal(String::new());

    // Assemble the full interval string whenever parts change
    Effect::new(move |_| {
        let left = if left_closed.get() { "[" } else { "(" };
        let right = if right_closed.get() { "]" } else { ")" };
        let inner = inner_plain.get();

        // Strip any leading/trailing parens/brackets from the converted template,
        // then re-wrap with the user-selected bracket types.
        let stripped = inner
            .trim()
            .trim_start_matches(['(', '['])
            .trim_end_matches([')', ']'])
            .trim();
        let assembled = format!("{}{}{}", left, stripped, right);
        set_value.set(assembled);
    });

    view! {
        <div class="relative">
            <button
                type="button"
                class="absolute left-0 top-0 bottom-0 z-10 w-8 flex items-center justify-center
                       text-lg font-mono text-gray-500 hover:text-gray-900 hover:bg-gray-100
                       rounded-l transition-colors border-r border-gray-200"
                on:click=move |_| set_left_closed.update(|v| *v = !*v)
                title="Toggle open/closed bracket"
            >
                {move || if left_closed.get() { "[" } else { "(" }}
            </button>

            <div style="padding-left: 32px; padding-right: 32px;">
                <MathField
                    set_plain=set_inner_plain
                    on_submit=on_submit
                    template="\\left( ,\\right)"
                    key=key.unwrap_or_default()
                    disabled=disabled
                />
            </div>

            <button
                type="button"
                class="absolute right-0 top-0 bottom-0 z-10 w-8 flex items-center justify-center
                       text-lg font-mono text-gray-500 hover:text-gray-900 hover:bg-gray-100
                       rounded-r transition-colors border-l border-gray-200"
                on:click=move |_| set_right_closed.update(|v| *v = !*v)
                title="Toggle open/closed bracket"
            >
                {move || if right_closed.get() { "]" } else { ")" }}
            </button>
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
                        if let Ok(mq_instance) =
                            mq_fn.call1(&wasm_bindgen::JsValue::NULL, &mq_span)
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
// Matrix — template + add row/col
// ============================================================================

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
                view! {
                    <MathField
                        set_plain=set_value
                        on_submit=on_submit
                        template="\\begin{pmatrix} & \\\\ & \\end{pmatrix}"
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
                    on:click=move |_| set_cols.update(|c| *c += 1)
                >
                    "+Col"
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
