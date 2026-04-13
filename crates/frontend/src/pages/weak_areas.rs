//! Weak area analysis page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use locus_common::{SubtopicAccuracy, WeakAreaResponse};

use crate::{AuthContext, api};

#[component]
pub fn WeakAreas() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = leptos_router::hooks::use_navigate();

    let navigate_clone = navigate.clone();
    Effect::new(move |_| {
        if !auth.is_logged_in.get() {
            navigate_clone("/login", Default::default());
        }
    });

    let (data, set_data) = signal(None::<WeakAreaResponse>);
    let (loading, set_loading) = signal(true);

    Effect::new(move |_| {
        spawn_local(async move {
            match api::get_weak_areas().await {
                Ok(resp) => set_data.set(Some(resp)),
                Err(_) => {}
            }
            set_loading.set(false);
        });
    });

    view! {
        <div class="max-w-4xl mx-auto px-4 py-8">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-2xl font-semibold">"Weak Areas"</h1>
                <A href="/stats" attr:class="text-sm text-blue-600 hover:underline">"Back to Stats"</A>
            </div>

            {move || loading.get().then(|| view! {
                <div class="text-gray-500 text-sm">"Loading..."</div>
            })}

            {move || data.get().map(|resp| {
                if resp.all.is_empty() {
                    return view! {
                        <div class="text-center py-12 text-gray-500">
                            <p class="text-lg mb-2">"No data yet!"</p>
                            <p class="text-sm">"Play ranked games to see your weak areas."</p>
                        </div>
                    }.into_any();
                }

                let weakest = resp.weakest.clone();
                let all = resp.all.clone();

                // Group by main_topic
                let mut grouped: std::collections::BTreeMap<String, Vec<SubtopicAccuracy>> = std::collections::BTreeMap::new();
                for item in &all {
                    grouped.entry(item.main_topic.clone()).or_default().push(item.clone());
                }

                view! {
                    <div class="space-y-6">
                        // Weakest areas cards
                        {if !weakest.is_empty() {
                            view! {
                                <div>
                                    <h2 class="font-medium mb-3">"Weakest Areas"</h2>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                                        {weakest.into_iter().map(|item| {
                                            let pct = (item.accuracy * 100.0).round() as i32;
                                            let topic_display = locus_common::MainTopic::from_str(&item.main_topic)
                                                .map(|t| t.display_name().to_string())
                                                .unwrap_or_else(|| item.main_topic.clone());
                                            let subtopic_display = locus_common::subtopic_display_name(&item.subtopic);
                                            let bar_color = if pct < 40 { "bg-red-500" } else if pct < 60 { "bg-yellow-500" } else { "bg-green-500" };
                                            let topic_id = item.main_topic.clone();

                                            view! {
                                                <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
                                                    <div class="text-sm font-medium mb-1">{subtopic_display}</div>
                                                    <div class="text-xs text-gray-500 mb-2">{topic_display}</div>
                                                    <div class="flex items-center gap-2 mb-2">
                                                        <div class="flex-1 bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                                                            <div
                                                                class=format!("{} h-2 rounded-full", bar_color)
                                                                style=format!("width: {}%", pct)
                                                            ></div>
                                                        </div>
                                                        <span class="text-sm font-mono">{format!("{}%", pct)}</span>
                                                    </div>
                                                    <div class="text-xs text-gray-400 mb-2">{format!("{}/{} correct", item.correct, item.total)}</div>
                                                    <A
                                                        href=format!("/practice?main_topic={}", topic_id)
                                                        attr:class="text-xs text-blue-600 hover:underline"
                                                    >"Practice This"</A>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }}

                        // Full subtopic table grouped by topic
                        <div>
                            <h2 class="font-medium mb-3">"All Subtopics"</h2>
                            {grouped.into_iter().map(|(topic_key, items)| {
                                let topic_display = locus_common::MainTopic::from_str(&topic_key)
                                    .map(|t| t.display_name().to_string())
                                    .unwrap_or_else(|| topic_key.clone());

                                view! {
                                    <div class="mb-4">
                                        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">{topic_display}</h3>
                                        <div class="border border-gray-200 dark:border-gray-700 rounded overflow-hidden">
                                            <table class="w-full text-sm">
                                                <thead class="bg-gray-50 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
                                                    <tr>
                                                        <th class="text-left px-4 py-2 font-medium text-gray-600 dark:text-gray-400">"Subtopic"</th>
                                                        <th class="text-right px-4 py-2 font-medium text-gray-600 dark:text-gray-400">"Accuracy"</th>
                                                        <th class="text-right px-4 py-2 font-medium text-gray-600 dark:text-gray-400">"Attempts"</th>
                                                        <th class="text-right px-4 py-2 font-medium text-gray-600 dark:text-gray-400">"Correct"</th>
                                                    </tr>
                                                </thead>
                                                <tbody class="divide-y divide-gray-100 dark:divide-gray-700">
                                                    {items.into_iter().map(|item| {
                                                        let pct = (item.accuracy * 100.0).round() as i32;
                                                        let color = if pct < 40 { "text-red-600" } else if pct < 60 { "text-yellow-600" } else { "text-green-600" };
                                                        view! {
                                                            <tr class="hover:bg-gray-50 dark:hover:bg-gray-800">
                                                                <td class="px-4 py-2">{locus_common::subtopic_display_name(&item.subtopic)}</td>
                                                                <td class=format!("px-4 py-2 text-right font-mono {}", color)>{format!("{}%", pct)}</td>
                                                                <td class="px-4 py-2 text-right">{item.total}</td>
                                                                <td class="px-4 py-2 text-right">{item.correct}</td>
                                                            </tr>
                                                        }
                                                    }).collect_view()}
                                                </tbody>
                                            </table>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                }.into_any()
            })}
        </div>
    }
}
