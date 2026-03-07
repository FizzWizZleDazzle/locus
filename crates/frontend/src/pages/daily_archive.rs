//! Daily puzzle archive — paginated list of past puzzles

use leptos::prelude::*;
use leptos::task::spawn_local;
use locus_common::DailyArchiveEntry;

use crate::{api, AuthContext};

#[component]
pub fn DailyArchive() -> impl IntoView {
    let auth = expect_context::<AuthContext>();

    let (entries, set_entries) = signal(Vec::<DailyArchiveEntry>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    let (page, set_page) = signal(0_i64);
    let (has_more, set_has_more) = signal(true);

    let page_size: i64 = 30;

    let load_entries = move |p: i64| {
        set_loading.set(true);
        spawn_local(async move {
            match api::get_daily_archive(page_size, p * page_size).await {
                Ok(data) => {
                    set_has_more.set(data.len() as i64 >= page_size);
                    set_entries.set(data);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e.message));
                    set_loading.set(false);
                }
            }
        });
    };

    Effect::new(move |_| {
        let p = page.get();
        load_entries(p);
    });

    view! {
        <div class="max-w-2xl mx-auto px-4 py-8">
            // Header
            <div class="flex items-center justify-between mb-8">
                <div>
                    <h1 class="text-2xl font-bold tracking-tight">"Puzzle Archive"</h1>
                    <p class="text-sm text-gray-500 dark:text-gray-400 mt-0.5">"Browse and revisit past daily puzzles"</p>
                </div>
                <a href="/daily" class="text-sm text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 transition-colors">
                    "Today's Puzzle"
                </a>
            </div>

            {move || error.get().map(|e| view! {
                <div class="text-red-600 text-sm mb-4">{e}</div>
            })}

            {move || loading.get().then(|| view! {
                <div class="text-gray-400 text-sm py-8 text-center">"Loading..."</div>
            })}

            {move || (!loading.get()).then(|| {
                let data = entries.get();
                if data.is_empty() {
                    return view! {
                        <div class="text-gray-400 text-sm py-8 text-center">"No past puzzles yet."</div>
                    }.into_any();
                }

                view! {
                    <div class="divide-y divide-gray-100 dark:divide-gray-800">
                        {data.into_iter().map(|entry| {
                            let date_str = entry.puzzle_date.format("%Y-%m-%d").to_string();
                            let display_date = entry.puzzle_date.format("%b %-d").to_string();
                            let display_year = entry.puzzle_date.format("%Y").to_string();
                            let solve_pct = format!("{:.0}%", entry.solve_rate * 100.0);
                            let href = format!("/daily/puzzle/{}", date_str);

                            let badge = if entry.user_solved_same_day == Some(true) {
                                Some(("Solved", "text-green-600 dark:text-green-400"))
                            } else if entry.user_solved == Some(true) {
                                Some(("Late", "text-yellow-600 dark:text-yellow-400"))
                            } else if auth.is_logged_in.get() {
                                Some(("Unsolved", "text-gray-400 dark:text-gray-500"))
                            } else {
                                None
                            };

                            view! {
                                <a href=href class="flex items-center gap-4 py-3.5 hover:bg-gray-50 dark:hover:bg-gray-800/30 -mx-2 px-2 rounded transition-colors">
                                    // Date column
                                    <div class="w-16 flex-shrink-0">
                                        <span class="text-sm font-semibold">{display_date}</span>
                                        <span class="text-xs text-gray-400 ml-1">{display_year}</span>
                                    </div>

                                    // Title + meta
                                    <div class="flex-1 min-w-0">
                                        <p class="text-sm font-medium truncate">
                                            {if entry.title.is_empty() { "Daily Puzzle".to_string() } else { entry.title }}
                                        </p>
                                        <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
                                            {entry.main_topic.replace('_', " ")}
                                            " · "
                                            {format!("Difficulty {}", entry.difficulty)}
                                            " · "
                                            {format!("{} solve rate", solve_pct)}
                                        </p>
                                    </div>

                                    // Badge
                                    {badge.map(|(label, color)| view! {
                                        <span class=format!("text-xs font-medium flex-shrink-0 {}", color)>{label}</span>
                                    })}

                                    // Chevron
                                    <svg class="w-4 h-4 text-gray-300 dark:text-gray-600 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                    </svg>
                                </a>
                            }
                        }).collect::<Vec<_>>()}
                    </div>

                    // Pagination
                    <div class="flex items-center justify-between mt-8 pt-4 border-t border-gray-100 dark:border-gray-800">
                        <button
                            class="text-sm text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 disabled:opacity-30 disabled:cursor-default transition-colors"
                            on:click=move |_| set_page.update(|p| *p = (*p).saturating_sub(1))
                            disabled=move || page.get() == 0
                        >
                            "Previous"
                        </button>
                        <span class="text-xs text-gray-400">{move || format!("Page {}", page.get() + 1)}</span>
                        <button
                            class="text-sm text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 disabled:opacity-30 disabled:cursor-default transition-colors"
                            on:click=move |_| set_page.update(|p| *p += 1)
                            disabled=move || !has_more.get()
                        >
                            "Next"
                        </button>
                    </div>
                }.into_any()
            })}
        </div>
    }
}
