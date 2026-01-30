//! Math input component

use leptos::prelude::*;
use crate::grader::preprocess_input;

#[component]
pub fn MathInput(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(default = "Enter your answer...")]
    placeholder: &'static str,
    #[prop(default = false)]
    disabled: bool,
    #[prop(optional)]
    on_submit: Option<Callback<()>>,
) -> impl IntoView {
    let on_input = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        set_value.set(target.value());
    };

    let on_keypress = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            if let Some(callback) = on_submit {
                callback.run(());
            }
        }
    };

    let processed_value = move || preprocess_input(&value.get());

    view! {
        <div class="space-y-2">
            <input
                type="text"
                class="w-full px-4 py-3 border border-gray-300 rounded focus:border-gray-900 focus:outline-none font-mono"
                class:bg-gray-100=disabled
                placeholder=placeholder
                prop:value=value
                prop:disabled=disabled
                on:input=on_input
                on:keypress=on_keypress
            />
            {move || {
                let raw = value.get();
                let processed = processed_value();
                (!raw.is_empty() && raw != processed).then(|| view! {
                    <div class="text-xs text-gray-500">
                        "Parsed: " <code class="bg-gray-100 px-1">{processed}</code>
                    </div>
                })
            }}
        </div>
    }
}
