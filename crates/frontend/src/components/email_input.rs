//! Email input component with typo suggestion (mailcheck.js) and strict regex validation.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// RFC 5322 email regex (99.99% coverage) — validated on blur, not per-keystroke.
const EMAIL_REGEX: &str = r#"^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$"#;

/// Lazy-loads mailcheck.js by injecting a `<script>` tag, then calls `callback` on load.
fn ensure_mailcheck_loaded(callback: impl FnOnce() + 'static) {
    let window = web_sys::window().unwrap();

    let has_mailcheck = js_sys::Reflect::get(&window, &"Mailcheck".into())
        .map(|v| !v.is_undefined() && !v.is_null())
        .unwrap_or(false);

    if has_mailcheck {
        callback();
        return;
    }

    let document = window.document().unwrap();
    let script = document
        .create_element("script")
        .unwrap()
        .unchecked_into::<web_sys::HtmlScriptElement>();
    script.set_src("https://cdn.jsdelivr.net/npm/mailcheck@1.1.1/src/mailcheck.min.js");

    let cb = Closure::once_into_js(move || {
        callback();
    });
    script.set_onload(Some(cb.unchecked_ref()));

    document.head().unwrap().append_child(&script).unwrap();
}

/// Run `Mailcheck.run()` on the given email. Returns suggested full address or None.
fn run_mailcheck(email: &str) -> Option<String> {
    let code = format!(
        r#"
        (function() {{
            var result = null;
            Mailcheck.run({{
                email: "{}",
                suggested: function(s) {{ result = s.full; }},
                empty: function() {{}}
            }});
            return result;
        }})()
        "#,
        email.replace('\\', "\\\\").replace('"', "\\\"")
    );
    let val = js_sys::eval(&code).ok()?;
    val.as_string()
}

/// Validated email input with typo suggestions and strict format checking.
#[component]
pub fn EmailInput(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional)] placeholder: &'static str,
    valid: WriteSignal<bool>,
) -> impl IntoView {
    let (suggestion, set_suggestion) = signal(None::<String>);
    let (format_error, set_format_error) = signal(false);
    let (mailcheck_ready, set_mailcheck_ready) = signal(false);
    let (touched, set_touched) = signal(false);

    // Load mailcheck.js on mount
    Effect::new(move || {
        ensure_mailcheck_loaded(move || {
            set_mailcheck_ready.set(true);
        });
    });

    let regex = js_sys::RegExp::new(EMAIL_REGEX, "");
    let regex2 = regex.clone();

    let on_blur = move |_| {
        set_touched.set(true);
        let val = value.get();

        if val.is_empty() {
            set_format_error.set(false);
            set_suggestion.set(None);
            valid.set(false);
            return;
        }

        // Layer 2: strict regex
        let is_valid = regex.test(&val);
        set_format_error.set(!is_valid);
        valid.set(is_valid);

        // Layer 1: typo suggestion (only if mailcheck loaded)
        if mailcheck_ready.get_untracked() {
            set_suggestion.set(run_mailcheck(&val));
        }
    };

    let on_input = move |ev: web_sys::Event| {
        let val = event_target_value(&ev);
        set_value.set(val.clone());
        // Clear suggestion on new input
        set_suggestion.set(None);
        // Re-validate if already touched
        if touched.get_untracked() {
            if val.is_empty() {
                set_format_error.set(false);
                valid.set(false);
            } else {
                let is_valid = regex2.test(&val);
                set_format_error.set(!is_valid);
                valid.set(is_valid);
            }
        }
    };

    let accept_suggestion = move |_| {
        if let Some(suggested) = suggestion.get_untracked() {
            set_value.set(suggested);
            set_suggestion.set(None);
            set_format_error.set(false);
            valid.set(true);
        }
    };

    let border_class = move || {
        if touched.get() && format_error.get() {
            "w-full px-3 py-2 border border-red-400 rounded focus:border-gray-900 focus:outline-none"
        } else {
            "w-full px-3 py-2 border border-gray-300 rounded focus:border-gray-900 focus:outline-none"
        }
    };

    view! {
        <div>
            <label class="block text-sm text-gray-600 mb-1">"Email"</label>
            <input
                type="email"
                class=border_class
                prop:value=value
                on:input=on_input
                on:blur=on_blur
                required
                placeholder=placeholder
            />
            <Show when=move || suggestion.get().is_some()>
                <p class="text-sm text-blue-600 mt-1">
                    "Did you mean "
                    <button
                        type="button"
                        class="underline cursor-pointer font-medium"
                        on:click=accept_suggestion
                    >
                        {move || suggestion.get().unwrap_or_default()}
                    </button>
                    "?"
                </p>
            </Show>
            <Show when=move || touched.get() && format_error.get()>
                <p class="text-sm text-red-500 mt-1">"Please enter a valid email address"</p>
            </Show>
        </div>
    }
}
