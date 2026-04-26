use leptos::prelude::*;

use crate::api::StatusCheck;

#[component]
pub fn ResponseTimeChart(checks: Vec<StatusCheck>) -> impl IntoView {
    if checks.is_empty() {
        return view! { <div class="chart-empty">"No data yet"</div> }.into_any();
    }

    let width = 720;
    let height = 200;
    let padding_left = 50;
    let padding_bottom = 30;
    let padding_top = 20;
    let chart_width = width - padding_left - 10;
    let chart_height = height - padding_top - padding_bottom;

    let max_ms = checks
        .iter()
        .filter_map(|c| c.response_time_ms)
        .max()
        .unwrap_or(1000)
        .max(100) as f64;

    let n = checks.len();
    let bar_width = if n > 0 {
        (chart_width as f64 / n as f64).max(2.0).min(8.0)
    } else {
        4.0
    };

    let mut bars = Vec::new();
    for (i, check) in checks.iter().enumerate() {
        let x = padding_left as f64 + (i as f64 / n as f64) * chart_width as f64;
        let ms = check.response_time_ms.unwrap_or(0) as f64;
        let bar_h = (ms / max_ms) * chart_height as f64;
        let y = (padding_top + chart_height) as f64 - bar_h;
        let color = if check.is_healthy {
            "#22c55e"
        } else {
            "#ef4444"
        };

        bars.push(view! {
            <rect
                x=format!("{:.1}", x)
                y=format!("{:.1}", y)
                width=format!("{:.1}", bar_width)
                height=format!("{:.1}", bar_h.max(1.0))
                fill=color
                rx="1"
            />
        });
    }

    // Y-axis labels
    let y_labels: Vec<_> = (0..=4)
        .map(|i| {
            let val = (max_ms * i as f64 / 4.0) as i32;
            let y = (padding_top + chart_height) as f64 - (i as f64 / 4.0) * chart_height as f64;
            view! {
                <text x="45" y=format!("{:.0}", y + 4.0) text-anchor="end" class="axis-label">
                    {format!("{}ms", val)}
                </text>
            }
        })
        .collect();

    view! {
        <svg viewBox=format!("0 0 {} {}", width, height) class="chart-svg">
            // Grid lines
            {(0..=4).map(|i| {
                let y = (padding_top + chart_height) as f64 - (i as f64 / 4.0) * chart_height as f64;
                view! {
                    <line
                        x1=format!("{}", padding_left)
                        y1=format!("{:.0}", y)
                        x2=format!("{}", width - 10)
                        y2=format!("{:.0}", y)
                        class="grid-line"
                    />
                }
            }).collect::<Vec<_>>()}
            {y_labels}
            {bars}
        </svg>
    }.into_any()
}

#[component]
pub fn UptimeBar(days: Vec<crate::api::DayUptime>) -> impl IntoView {
    if days.is_empty() {
        return view! { <div class="chart-empty">"No data"</div> }.into_any();
    }

    let bars: Vec<_> = days
        .iter()
        .enumerate()
        .map(|(i, day)| {
            let color = if day.uptime_percent >= 99.9 {
                "#22c55e"
            } else if day.uptime_percent >= 95.0 {
                "#f59e0b"
            } else {
                "#ef4444"
            };
            let x = i as f64 * (100.0 / days.len() as f64);
            let w = (100.0 / days.len() as f64) - 0.5;

            view! {
                <rect
                    x=format!("{:.2}%", x)
                    y="0"
                    width=format!("{:.2}%", w.max(0.5))
                    height="32"
                    fill=color
                    rx="2"
                >
                    <title>{format!("{}: {:.1}%", day.date, day.uptime_percent)}</title>
                </rect>
            }
        })
        .collect();

    view! {
        <svg viewBox="0 0 100 32" preserveAspectRatio="none" class="uptime-bar-svg">
            {bars}
        </svg>
    }
    .into_any()
}
