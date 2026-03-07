//! GitHub-style activity matrix component

use chrono::Datelike;
use leptos::prelude::*;
use locus_common::{DailyActivityDay, DailyDayStatus};

#[component]
pub fn ActivityMatrix(days: Vec<DailyActivityDay>) -> impl IntoView {
    let cells: Vec<(usize, usize, DailyDayStatus)> = days
        .iter()
        .map(|day| {
            let row = day.date.weekday().num_days_from_monday() as usize;
            (row, 0, day.status.clone())
        })
        .collect();

    // Group by week (column)
    let mut week_cols: Vec<Vec<(usize, DailyDayStatus)>> = Vec::new();
    let mut current_week: Vec<(usize, DailyDayStatus)> = Vec::new();
    let mut last_week_num: Option<i32> = None;

    for (i, day) in days.iter().enumerate() {
        let iso_week = day.date.iso_week().week() as i32;
        let year = day.date.year();
        let week_key = year * 100 + iso_week;

        if let Some(last) = last_week_num {
            if week_key != last {
                week_cols.push(std::mem::take(&mut current_week));
            }
        }
        last_week_num = Some(week_key);
        let row = day.date.weekday().num_days_from_monday() as usize;
        current_week.push((row, cells[i].2.clone()));
    }
    if !current_week.is_empty() {
        week_cols.push(current_week);
    }

    let cell_size = 11;
    let gap = 2;
    let step = cell_size + gap;
    let cols = week_cols.len();
    let width = cols * step + 2;
    let height = 7 * step + 2;

    let mut rects = Vec::new();
    for (col_idx, week) in week_cols.iter().enumerate() {
        for &(row, ref status) in week {
            let x = col_idx * step + 1;
            let y = row * step + 1;
            let fill = match status {
                DailyDayStatus::NoPuzzle => "var(--matrix-empty, #ebedf0)",
                DailyDayStatus::Missed => "var(--matrix-missed, #fddf68)",
                DailyDayStatus::SolvedLate => "var(--matrix-late, #7bc96f)",
                DailyDayStatus::SolvedSameDay => "var(--matrix-same-day, #196127)",
            };
            rects.push(format!(
                r#"<rect x="{x}" y="{y}" width="{cell_size}" height="{cell_size}" rx="2" fill="{fill}" />"#
            ));
        }
    }

    let svg_content = format!(
        r#"<svg width="{width}" height="{height}" viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg">{}</svg>"#,
        rects.join("")
    );

    view! {
        <div class="overflow-x-auto">
            <style>
                ".dark { --matrix-empty: #161b22; --matrix-missed: #5a4a00; --matrix-late: #0e4429; --matrix-same-day: #39d353; }"
            </style>
            <div inner_html=svg_content />
            <div class="flex items-center gap-3 mt-2 text-xs text-gray-500 dark:text-gray-400">
                <span class="flex items-center gap-1">
                    <span class="inline-block w-2.5 h-2.5 rounded-sm" style="background: var(--matrix-same-day, #196127)"></span>
                    "Same day"
                </span>
                <span class="flex items-center gap-1">
                    <span class="inline-block w-2.5 h-2.5 rounded-sm" style="background: var(--matrix-late, #7bc96f)"></span>
                    "Solved late"
                </span>
                <span class="flex items-center gap-1">
                    <span class="inline-block w-2.5 h-2.5 rounded-sm" style="background: var(--matrix-missed, #fddf68)"></span>
                    "Missed"
                </span>
                <span class="flex items-center gap-1">
                    <span class="inline-block w-2.5 h-2.5 rounded-sm" style="background: var(--matrix-empty, #ebedf0)"></span>
                    "No puzzle"
                </span>
            </div>
        </div>
    }
}
