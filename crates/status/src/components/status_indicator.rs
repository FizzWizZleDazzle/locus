use leptos::prelude::*;

#[component]
pub fn StatusIndicator(healthy: Option<bool>) -> impl IntoView {
    let (class, label) = match healthy {
        Some(true) => ("status-badge healthy", "Operational"),
        Some(false) => ("status-badge unhealthy", "Degraded"),
        None => ("status-badge unknown", "Unknown"),
    };

    view! {
        <span class=class>{label}</span>
    }
}
