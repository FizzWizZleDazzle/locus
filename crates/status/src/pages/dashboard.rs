use leptos::prelude::*;

use crate::api;
use crate::components::chart::{ResponseTimeChart, UptimeBar};
use crate::components::status_indicator::StatusIndicator;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let current = RwSignal::new(Option::<Result<api::CurrentStatus, String>>::None);
    let history = RwSignal::new(Option::<Result<Vec<api::StatusCheck>, String>>::None);
    let uptime = RwSignal::new(Option::<Result<api::UptimeResponse, String>>::None);

    leptos::task::spawn_local(async move {
        let r = api::get_current().await;
        current.set(Some(r));
    });
    leptos::task::spawn_local(async move {
        let r = api::get_history().await;
        history.set(Some(r));
    });
    leptos::task::spawn_local(async move {
        let r = api::get_uptime().await;
        uptime.set(Some(r));
    });

    view! {
        <div class="max-w-2xl mx-auto py-12 px-4">
            <div class="text-center mb-10">
                <h1 class="text-2xl font-bold text-gray-900">"Locus Status"</h1>
                <p class="text-sm text-gray-500 mt-1">"System health and uptime"</p>
            </div>

            // Current status
            <div class="mb-8">
                {move || match current.get() {
                    None => view! { <div class="text-center text-gray-400 py-10">"Checking status..."</div> }.into_any(),
                    Some(Err(e)) => view! { <div class="text-red-600 text-sm">{e}</div> }.into_any(),
                    Some(Ok(status)) => {
                        let healthy = status.is_healthy;
                        let checked = status.checked_at
                            .map(|t| t.format("%Y-%m-%d %H:%M UTC").to_string())
                            .unwrap_or_else(|| "Never".to_string());
                        let resp_time = status.response_time_ms
                            .map(|ms| format!("{}ms", ms))
                            .unwrap_or_else(|| "-".to_string());

                        view! {
                            <div class="border border-gray-200 rounded p-6">
                                <div class="flex items-center gap-3 mb-5">
                                    <StatusIndicator healthy=healthy />
                                    <span class="text-base font-medium text-gray-900">"api.locusmath.org"</span>
                                </div>
                                <div class="flex gap-8">
                                    <div class="flex flex-col">
                                        <span class="text-xs text-gray-400 uppercase tracking-wide">"Response Time"</span>
                                        <span class="text-lg font-semibold text-gray-900 mt-0.5">{resp_time}</span>
                                    </div>
                                    <div class="flex flex-col">
                                        <span class="text-xs text-gray-400 uppercase tracking-wide">"Last Check"</span>
                                        <span class="text-lg font-semibold text-gray-900 mt-0.5">{checked}</span>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Response time chart
            <div class="mb-8">
                <h2 class="text-base font-semibold text-gray-900 mb-3">"Response Time (24h)"</h2>
                <div class="chart-container">
                    {move || match history.get() {
                        None => view! { <div class="text-center text-gray-400 py-10">"Loading chart..."</div> }.into_any(),
                        Some(Err(e)) => view! { <div class="text-red-600 text-sm">{e}</div> }.into_any(),
                        Some(Ok(checks)) => view! { <ResponseTimeChart checks=checks /> }.into_any(),
                    }}
                </div>
            </div>

            // 30-day uptime
            <div class="mb-8">
                {move || match uptime.get() {
                    None => view! { <div class="text-center text-gray-400 py-10">"Loading uptime..."</div> }.into_any(),
                    Some(Err(e)) => view! { <div class="text-red-600 text-sm">{e}</div> }.into_any(),
                    Some(Ok(data)) => {
                        let pct = format!("{:.2}%", data.uptime_30d_percent);
                        view! {
                            <div class="border border-gray-200 rounded p-6">
                                <div class="flex justify-between items-center mb-4">
                                    <h2 class="text-base font-semibold text-gray-900">"30-Day Uptime"</h2>
                                    <span class="text-2xl font-bold text-green-600">{pct}</span>
                                </div>
                                <div class="uptime-bar-container">
                                    <UptimeBar days=data.daily_breakdown />
                                </div>
                                <div class="flex justify-between text-xs text-gray-400 mt-2">
                                    <span>"30 days ago"</span>
                                    <span>"Today"</span>
                                </div>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
