//! Public profile page — also shows detailed stats when viewing own profile

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;
use locus_common::{EloHistoryPoint, PublicProfileResponse, UserStatsResponse};

use crate::{AuthContext, api, components::{ActivityMatrix, BadgeGrid}};

#[component]
pub fn Profile() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let params = use_params_map();
    let username = move || {
        params.read().get("username").unwrap_or_default()
    };

    let (profile, set_profile) = signal(None::<PublicProfileResponse>);
    let (own_stats, set_own_stats) = signal(None::<UserStatsResponse>);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);

    // ELO history state (only used when viewing own profile)
    let (selected_topic, set_selected_topic) = signal(None::<String>);
    let (elo_history, set_elo_history) = signal(Vec::<EloHistoryPoint>::new());
    let (history_loading, set_history_loading) = signal(false);

    // Is this the logged-in user's own profile?
    let is_own = move || {
        auth.username.get()
            .map(|u| u == username())
            .unwrap_or(false)
    };

    // Load profile data
    Effect::new(move |_| {
        let uname = username();
        if uname.is_empty() {
            return;
        }
        set_loading.set(true);
        let own = is_own();
        spawn_local(async move {
            match api::get_public_profile(&uname).await {
                Ok(p) => {
                    set_profile.set(Some(p));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
            // If own profile, also load detailed stats (has badges with earned state)
            if own {
                if let Ok(s) = api::get_user_stats().await {
                    set_own_stats.set(Some(s));
                }
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
            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            {move || loading.get().then(|| view! {
                <div class="text-gray-500 text-sm">"Loading..."</div>
            })}

            {move || profile.get().map(|p| {
                let accuracy = if p.total_attempts > 0 {
                    (p.correct_attempts * 100) / p.total_attempts
                } else {
                    0
                };
                let member_since = p.member_since.format("%B %Y").to_string();
                let own = is_own();

                // Use stats badges when own profile (has earned state), otherwise profile badges
                let badges = if own {
                    own_stats.get().map(|s| s.badges).unwrap_or_else(|| p.badges.clone())
                } else {
                    p.badges.clone()
                };

                view! {
                    <div class="space-y-6">
                        // Header
                        <div>
                            <h1 class="text-2xl font-semibold">{p.username.clone()}</h1>
                            <p class="text-sm text-gray-500 mt-1">{format!("Member since {}", member_since)}</p>
                        </div>

                        // Stats summary
                        <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                            <div class="border border-gray-200 rounded p-4 text-center">
                                <div class="text-2xl font-bold">{p.total_attempts}</div>
                                <div class="text-sm text-gray-500 mt-1">"Problems Attempted"</div>
                            </div>
                            <div class="border border-gray-200 rounded p-4 text-center">
                                <div class="text-2xl font-bold">{format!("{}%", accuracy)}</div>
                                <div class="text-sm text-gray-500 mt-1">"Accuracy"</div>
                            </div>
                            <div class="border border-gray-200 rounded p-4 text-center">
                                <div class="text-2xl font-bold">{p.current_streak}</div>
                                <div class="text-sm text-gray-500 mt-1">"Ranked Streak"</div>
                            </div>
                            <div class="border border-gray-200 rounded p-4 text-center">
                                <div class="text-2xl font-bold">{p.daily_puzzle_streak}</div>
                                <div class="text-sm text-gray-500 mt-1">"Daily Streak"</div>
                            </div>
                        </div>

                        // Activity matrix
                        <div class="border border-gray-200 rounded p-4">
                            <div class="flex items-center justify-between mb-3">
                                <h2 class="font-medium">"Daily Puzzle Activity"</h2>
                                <div class="flex items-center gap-2 text-sm">
                                    <span class="text-lg font-bold">{p.activity.streak}</span>
                                    <span class="text-gray-500 dark:text-gray-400">"day streak"</span>
                                </div>
                            </div>
                            <ActivityMatrix days=p.activity.days.clone() />
                        </div>

                        // Badges
                        <div class="border border-gray-200 dark:border-gray-700 rounded p-4">
                            <h2 class="font-medium mb-3">"Badges"</h2>
                            <BadgeGrid badges=badges />
                        </div>

                        // Per-topic table
                        {if !p.topics.is_empty() {
                            let topics = p.topics.clone();
                            if own {
                                // Own profile: full table with streaks, clickable rows
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
                                                    <th class="text-right px-4 py-3 font-medium text-gray-600">"Best"</th>
                                                </tr>
                                            </thead>
                                            <tbody class="divide-y divide-gray-100">
                                                {topics.into_iter().map(|entry| {
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
                            } else {
                                // Other user: simple table
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
                                                </tr>
                                            </thead>
                                            <tbody class="divide-y divide-gray-100">
                                                {topics.into_iter().map(|entry| {
                                                    let acc = if entry.total > 0 {
                                                        (entry.correct * 100) / entry.total
                                                    } else {
                                                        0
                                                    };
                                                    let display = locus_common::MainTopic::from_str(&entry.topic)
                                                        .map(|t| t.display_name().to_string())
                                                        .unwrap_or_else(|| entry.topic.clone());
                                                    view! {
                                                        <tr class="hover:bg-gray-50">
                                                            <td class="px-4 py-3 text-gray-900">{display}</td>
                                                            <td class="px-4 py-3 text-right font-mono">{entry.elo}</td>
                                                            <td class="px-4 py-3 text-right font-mono text-gray-500">{entry.peak_elo}</td>
                                                            <td class="px-4 py-3 text-right">{format!("{}%", acc)}</td>
                                                            <td class="px-4 py-3 text-right">{entry.total}</td>
                                                        </tr>
                                                    }
                                                }).collect_view()}
                                            </tbody>
                                        </table>
                                    </div>
                                }.into_any()
                            }
                        } else {
                            view! {
                                <div class="text-gray-500 text-sm">"No ranked attempts yet."</div>
                            }.into_any()
                        }}

                        // ELO history graph (own profile only)
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
