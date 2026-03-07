//! Badge grid component — shows all badges with earned/locked states using AI-generated images.

use leptos::prelude::*;
use locus_common::badges::BadgeDisplay;

#[component]
pub fn BadgeGrid(badges: Vec<BadgeDisplay>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-3 sm:grid-cols-6 gap-3">
            {badges.into_iter().map(|badge| {
                let img_src = format!("/badges/{}.png", badge.id);
                if badge.earned {
                    let name = badge.name;
                    let desc = badge.description;
                    view! {
                        <div class="flex flex-col items-center rounded-lg p-3 bg-white border border-gray-200 shadow-sm">
                            <img src=img_src class="w-12 h-12 mb-1.5" alt=name.clone() />
                            <div class="text-xs font-semibold text-center text-gray-900">{name}</div>
                            <div class="text-[10px] text-gray-500 text-center mt-0.5">{desc}</div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="flex flex-col items-center rounded-lg p-3 bg-white border border-gray-100">
                            <img src=img_src class="w-12 h-12 mb-1.5 grayscale opacity-30" alt="Locked badge" />
                            <div class="text-xs font-semibold text-center text-gray-400">"???"</div>
                            <div class="text-[10px] text-gray-300 text-center mt-0.5">"Keep going..."</div>
                        </div>
                    }.into_any()
                }
            }).collect_view()}
        </div>
    }
}
