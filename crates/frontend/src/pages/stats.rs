//! Stats page

use leptos::prelude::*;
use leptos::task::spawn_local;
use locus_common::{DailyActivityResponse, EloHistoryPoint, UserStatsResponse};

use crate::components::{ActivityMatrix, BadgeGrid};
use crate::{AuthContext, api};

#[component]
pub fn Stats() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = leptos_router::hooks::use_navigate();

    let navigate_clone = navigate.clone();
    Effect::new(move |_| {
        if !auth.is_logged_in.get() {
            navigate_clone("/login", Default::default());
        }
    });

    let (stats, set_stats) = signal(None::<UserStatsResponse>);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);

    let (selected_topic, set_selected_topic) = signal(None::<String>);
    let (elo_history, set_elo_history) = signal(Vec::<EloHistoryPoint>::new());
    let (history_loading, set_history_loading) = signal(false);

    // Activity matrix state
    let (activity, set_activity) = signal(None::<DailyActivityResponse>);

    // Load stats on mount
    Effect::new(move |_| {
        spawn_local(async move {
            match api::get_user_stats().await {
                Ok(s) => {
                    set_stats.set(Some(s));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    });

    // Load activity matrix
    Effect::new(move |_| {
        spawn_local(async move {
            if let Ok(a) = api::get_daily_activity().await {
                set_activity.set(Some(a));
            }
        });
    });

    // Load ELO history when topic is selected
    Effect::new(move |_| {
        if let Some(topic) = selected_topic.get() {
            set_history_loading.set(true);
            spawn_local(async move {
                match api::get_elo_history(&topic).await {
                    Ok(resp) => {
                        set_elo_history.set(resp.history);
                        set_history_loading.set(false);
                    }
                    Err(_) => {
                        set_elo_history.set(Vec::new());
                        set_history_loading.set(false);
                    }
                }
            });
        }
    });

    view! {
        <div class="max-w-4xl mx-auto px-4 py-8">
            <h1 class="text-2xl font-semibold mb-6">"Stats"</h1>

            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            {move || loading.get().then(|| view! {
                <div class="text-gray-500 text-sm">"Loading..."</div>
            })}

            {move || stats.get().map(|s| {
                let accuracy = if s.total_attempts > 0 {
                    (s.correct_attempts * 100) / s.total_attempts
                } else {
                    0
                };

                view! {
                    <div class="space-y-6">
                        // Global summary
                        <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                            <div class="border border-gray-200 rounded p-4 text-center">
                                <div class="text-2xl font-bold">{s.total_attempts}</div>
                                <div class="text-sm text-gray-500 mt-1">"Problems Solved"</div>
                            </div>
                            <div class="border border-gray-200 rounded p-4 text-center">
                                <div class="text-2xl font-bold">{format!("{}%", accuracy)}</div>
                                <div class="text-sm text-gray-500 mt-1">"Overall Accuracy"</div>
                            </div>
                            <div class="border border-gray-200 rounded p-4 text-center">
                                <div class="text-2xl font-bold">{s.correct_attempts}</div>
                                <div class="text-sm text-gray-500 mt-1">"Correct Answers"</div>
                            </div>
                            <div class="border border-gray-200 rounded p-4 text-center">
                                <div class="text-2xl font-bold">{s.current_streak}</div>
                                <div class="text-sm text-gray-500 mt-1">"Day Streak"</div>
                            </div>
                        </div>

                        // Daily puzzle activity matrix
                        {move || activity.get().map(|act| view! {
                            <div class="border border-gray-200 rounded p-4">
                                <div class="flex items-center justify-between mb-3">
                                    <h2 class="font-medium">"Daily Puzzle Activity"</h2>
                                    <div class="flex items-center gap-2 text-sm">
                                        <span class="text-lg font-bold">{act.streak}</span>
                                        <span class="text-gray-500 dark:text-gray-400">"day streak"</span>
                                    </div>
                                </div>
                                <ActivityMatrix days=act.days.clone() />
                            </div>
                        })}

                        // Badges
                        <div class="border border-gray-200 dark:border-gray-700 rounded p-4">
                            <h2 class="font-medium mb-3">"Badges"</h2>
                            <BadgeGrid badges=s.badges.clone() />
                        </div>

                        // Per-topic table
                        {if s.topics.is_empty() {
                            view! {
                                <div class="text-gray-500 text-sm">"No ranked attempts yet. Play some ranked games to see your stats!"</div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="border border-gray-200 rounded overflow-hidden">
                                    <table class="w-full text-sm">
                                        <thead class="bg-gray-50 border-b border-gray-200">
                                            <tr>
                                                <th class="text-left px-4 py-3 font-medium text-gray-600">"Topic"</th>
                                                <th class="text-right px-4 py-3 font-medium text-gray-600">"ELO"</th>
                                                <th class="text-right px-4 py-3 font-medium text-gray-600">"Peak ELO"</th>
                                                <th class="text-right px-4 py-3 font-medium text-gray-600">"Accuracy"</th>
                                                <th class="text-right px-4 py-3 font-medium text-gray-600">"Solved"</th>
                                                <th class="text-right px-4 py-3 font-medium text-gray-600">"Streak"</th>
                                                <th class="text-right px-4 py-3 font-medium text-gray-600">"Best Streak"</th>
                                            </tr>
                                        </thead>
                                        <tbody class="divide-y divide-gray-100">
                                            {s.topics.into_iter().map(|entry| {
                                                let topic_id = entry.topic.clone();
                                                let acc = if entry.total > 0 {
                                                    (entry.correct * 100) / entry.total
                                                } else {
                                                    0
                                                };
                                                let display = locus_common::MainTopic::from_str(&entry.topic)
                                                    .map(|t| t.display_name().to_string())
                                                    .unwrap_or_else(|| entry.topic.clone());
                                                view! {
                                                    <tr
                                                        class="hover:bg-gray-50 cursor-pointer transition-colors"
                                                        on:click=move |_| set_selected_topic.set(Some(topic_id.clone()))
                                                    >
                                                        <td class="px-4 py-3 text-gray-900">{display}</td>
                                                        <td class="px-4 py-3 text-right font-mono">{entry.elo}</td>
                                                        <td class="px-4 py-3 text-right font-mono text-gray-500">{entry.peak_elo}</td>
                                                        <td class="px-4 py-3 text-right">{format!("{}%", acc)}</td>
                                                        <td class="px-4 py-3 text-right">{entry.total}</td>
                                                        <td class="px-4 py-3 text-right">{entry.topic_streak}</td>
                                                        <td class="px-4 py-3 text-right">{entry.peak_topic_streak}</td>
                                                    </tr>
                                                }
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                </div>
                            }.into_any()
                        }}

                        // ELO history graph
                        {move || selected_topic.get().map(|topic| {
                            let display = locus_common::MainTopic::from_str(&topic)
                                .map(|t| t.display_name().to_string())
                                .unwrap_or_else(|| topic.clone());
                            view! {
                                <div class="border border-gray-200 rounded p-4">
                                    <div class="flex items-center justify-between mb-3">
                                        <h2 class="font-medium">{format!("{} — ELO History (last 30 days)", display)}</h2>
                                        <button
                                            class="text-xs text-gray-400 hover:text-gray-600"
                                            on:click=move |_| set_selected_topic.set(None)
                                        >"Close"</button>
                                    </div>
                                    {move || history_loading.get().then(|| view! {
                                        <div class="text-sm text-gray-500">"Loading..."</div>
                                    })}
                                    {move || {
                                        let pts = elo_history.get();
                                        if pts.is_empty() && !history_loading.get() {
                                            view! { <div class="text-sm text-gray-500">"No data for this period."</div> }.into_any()
                                        } else if !pts.is_empty() {
                                            // Build simple SVG sparkline (reversed so oldest left)
                                            let mut sorted = pts.clone();
                                            sorted.reverse();
                                            let min_elo = sorted.iter().map(|p| p.elo).min().unwrap_or(1500);
                                            let max_elo = sorted.iter().map(|p| p.elo).max().unwrap_or(1500);
                                            let range = (max_elo - min_elo).max(1) as f64;
                                            let n = sorted.len();
                                            let w = 600.0f64;
                                            let h = 120.0f64;
                                            let pad = 8.0f64;

                                            let points: Vec<(f64, f64)> = sorted.iter().enumerate().map(|(i, p)| {
                                                let x = pad + (i as f64 / (n - 1).max(1) as f64) * (w - 2.0 * pad);
                                                let y = h - pad - ((p.elo - min_elo) as f64 / range) * (h - 2.0 * pad);
                                                (x, y)
                                            }).collect();

                                            let polyline = points.iter()
                                                .map(|(x, y)| format!("{:.1},{:.1}", x, y))
                                                .collect::<Vec<_>>()
                                                .join(" ");

                                            let last_elo = sorted.last().map(|p| p.elo).unwrap_or(0);
                                            let first_elo = sorted.first().map(|p| p.elo).unwrap_or(0);
                                            let delta = last_elo - first_elo;
                                            let delta_color = if delta >= 0 { "text-green-600" } else { "text-red-600" };
                                            let delta_str = if delta >= 0 { format!("+{}", delta) } else { format!("{}", delta) };

                                            view! {
                                                <div>
                                                    <div class="flex gap-4 text-sm mb-2">
                                                        <span class="text-gray-500">{format!("Current: {}", last_elo)}</span>
                                                        <span class=delta_color>{delta_str}</span>
                                                    </div>
                                                    <svg
                                                        viewBox=format!("0 0 {} {}", w, h)
                                                        class="w-full h-32 overflow-visible"
                                                    >
                                                        <polyline
                                                            points=polyline
                                                            fill="none"
                                                            stroke="#374151"
                                                            stroke-width="1.5"
                                                            stroke-linejoin="round"
                                                            stroke-linecap="round"
                                                        />
                                                        {points.iter().map(|(x, y)| view! {
                                                            <circle cx=x.to_string() cy=y.to_string() r="3" fill="#374151" />
                                                        }).collect_view()}
                                                    </svg>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }}
                                </div>
                            }
                        })}
                    </div>
                }
            })}
        </div>
    }
}
