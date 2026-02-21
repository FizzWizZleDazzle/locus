use leptos::prelude::*;
use leptos_use::use_interval_fn;

/// Compact countdown timer. Sits in the problem card header.
/// Purely visual — does NOT block submission after expiry.
#[component]
pub fn Timer(seconds: i32) -> impl IntoView {
    let (remaining, set_remaining) = signal(seconds);

    use_interval_fn(
        move || {
            set_remaining.update(|r| {
                if *r > 0 {
                    *r -= 1
                }
            });
        },
        1000,
    );

    let display = move || {
        let r = remaining.get();
        format!("{}:{:02}", r / 60, r % 60)
    };

    let style = move || {
        let r = remaining.get();
        if r == 0 {
            "color: #ef4444"
        } else if r <= 10 {
            "color: #f97316"
        } else {
            "color: #6b7280"
        }
    };

    view! {
        <span class="tabular-nums" style=style>{display}</span>
    }
}
