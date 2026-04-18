//! Phase 2: Free-body diagram builder.
//!
//! Interactive SVG: a target body sits at the centre of an SVG panel; each
//! expected force is represented as an arrow emanating from the body. The
//! student drags the arrow's tip to rotate it to the correct direction.
//! On Check, each picked angle is compared to the expected direction within
//! `tolerance_deg`.

use leptos::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;

use locus_physics_common::challenge::ForceSpec;

const PANEL: f32 = 240.0;
const ARROW_LEN: f32 = 85.0;
const HANDLE_R: f32 = 10.0;

#[component]
pub fn FbdBuilder(
    target_body: String,
    expected_forces: Vec<ForceSpec>,
    #[prop(default = 15.0)] tolerance_deg: f32,
    #[prop(default = HashMap::new())] per_force_hints: HashMap<String, String>,
    on_complete: Callback<i32>,
) -> impl IntoView {
    let (attempts, set_attempts) = signal(0i32);
    // angle in degrees, physics frame (0=right, 90=up, CCW)
    let initial: HashMap<String, f32> = expected_forces
        .iter()
        .enumerate()
        .map(|(i, f)| (f.id.clone(), (i as f32) * (360.0 / expected_forces.len().max(1) as f32)))
        .collect();
    let (angles, set_angles) = signal(initial);
    let (feedback, set_feedback) = signal(HashMap::<String, bool>::new());
    let (completed, set_completed) = signal(false);
    let (dragging, set_dragging) = signal(Option::<String>::None);

    let forces = expected_forces.clone();
    let tol = tolerance_deg;

    let on_check = move |_| {
        set_attempts.update(|a| *a += 1);
        let a = angles.get();
        let mut fb = HashMap::new();
        let mut all_ok = true;
        for f in &forces {
            let picked = a.get(&f.id).copied().unwrap_or(-999.0);
            let d = (picked - f.direction_deg).rem_euclid(360.0);
            let diff = d.min(360.0 - d);
            let ok = diff <= tol;
            if !ok { all_ok = false; }
            fb.insert(f.id.clone(), ok);
        }
        set_feedback.set(fb);
        if all_ok {
            set_completed.set(true);
            on_complete.run(attempts.get_untracked());
        }
    };

    let on_pointer_move = move |ev: web_sys::PointerEvent| {
        let Some(id) = dragging.get() else { return };
        let target = ev.current_target().unwrap().dyn_into::<web_sys::Element>().unwrap();
        let rect = target.get_bounding_client_rect();
        let cx = rect.left() + rect.width() / 2.0;
        let cy = rect.top() + rect.height() / 2.0;
        let dx = ev.client_x() as f64 - cx;
        let dy = ev.client_y() as f64 - cy;
        // physics frame: y-up, angle CCW from +x
        let angle = (-dy).atan2(dx).to_degrees();
        let mut angle = if angle < 0.0 { angle + 360.0 } else { angle } as f32;
        // snap to nearest 5°
        angle = (angle / 5.0).round() * 5.0;
        if angle >= 360.0 { angle -= 360.0; }
        set_angles.update(|map| { map.insert(id.clone(), angle); });
    };

    let on_pointer_up = move |_ev: web_sys::PointerEvent| {
        set_dragging.set(None);
    };

    let forces_for_view = expected_forces.clone();
    let hints = per_force_hints.clone();

    view! {
        <div class="space-y-2">
            <p class="text-xs text-gray-500 dark:text-gray-400">
                "Drag each handle to point the force in its direction."
            </p>

            <div class="flex justify-center">
                <svg
                    width=PANEL
                    height=PANEL
                    viewBox=format!("0 0 {} {}", PANEL, PANEL)
                    class="bg-slate-50 dark:bg-gray-900 rounded border border-gray-200 dark:border-gray-700 touch-none"
                    on:pointermove=on_pointer_move
                    on:pointerup=on_pointer_up
                    on:pointerleave=on_pointer_up
                >
                    // Compass guide
                    <g stroke="#cbd5e1" stroke-width="1" fill="none">
                        <circle cx=PANEL/2.0 cy=PANEL/2.0 r=ARROW_LEN stroke-dasharray="2,3"></circle>
                        <line x1=PANEL/2.0 - 110.0 y1=PANEL/2.0 x2=PANEL/2.0 + 110.0 y2=PANEL/2.0></line>
                        <line x1=PANEL/2.0 y1=PANEL/2.0 - 110.0 x2=PANEL/2.0 y2=PANEL/2.0 + 110.0></line>
                    </g>
                    <g fill="#94a3b8" font-size="10" text-anchor="middle">
                        <text x=PANEL/2.0 y=14>"up"</text>
                        <text x=PANEL/2.0 y=PANEL - 6.0>"down"</text>
                        <text x=8 y=PANEL/2.0 + 4.0 text-anchor="start">"left"</text>
                        <text x=PANEL - 8.0 y=PANEL/2.0 + 4.0 text-anchor="end">"right"</text>
                    </g>

                    <rect
                        x=PANEL/2.0 - 20.0
                        y=PANEL/2.0 - 16.0
                        width=40
                        height=32
                        fill="#e5e7eb"
                        stroke="#6b7280"
                        stroke-width=1
                        rx=2
                    ></rect>

                    // Arrows
                    {forces_for_view.into_iter().map(|force| {
                        let fid = force.id.clone();
                        let fid_line = force.id.clone();
                        let fid_head = force.id.clone();
                        let fid_handle = force.id.clone();
                        let fid_lbl = force.id.clone();
                        let fid_down = force.id.clone();
                        let color = force.color.clone();
                        let color2 = color.clone();
                        let color3 = color.clone();
                        let color4 = color.clone();
                        let label = force.label.clone();

                        // Convert physics angle (CCW from +x, y-up) → SVG coords (y-down)
                        let get_tip = move |id: &str| {
                            let a = angles.get().get(id).copied().unwrap_or(0.0) as f64;
                            let rad = a.to_radians();
                            let tx = PANEL as f64 / 2.0 + rad.cos() * ARROW_LEN as f64;
                            let ty = PANEL as f64 / 2.0 - rad.sin() * ARROW_LEN as f64;
                            (tx, ty)
                        };

                        let line_x2 = {
                            let fid = fid_line.clone();
                            move || { let (x, _) = get_tip(&fid); x }
                        };
                        let line_y2 = {
                            let fid = fid_line.clone();
                            move || { let (_, y) = get_tip(&fid); y }
                        };
                        let head_transform = move || {
                            let a = angles.get().get(&fid_head).copied().unwrap_or(0.0);
                            let svg_rot = -a;
                            let (tx, ty) = {
                                let rad = (a as f64).to_radians();
                                let tx = PANEL as f64 / 2.0 + rad.cos() * ARROW_LEN as f64;
                                let ty = PANEL as f64 / 2.0 - rad.sin() * ARROW_LEN as f64;
                                (tx, ty)
                            };
                            format!("translate({}, {}) rotate({})", tx, ty, svg_rot)
                        };
                        let handle_cx = {
                            let fid = fid_handle.clone();
                            move || { let (x, _) = get_tip(&fid); x }
                        };
                        let handle_cy = {
                            let fid = fid_handle.clone();
                            move || { let (_, y) = get_tip(&fid); y }
                        };
                        let lbl_x = {
                            let fid = fid_lbl.clone();
                            move || { let (x, _) = get_tip(&fid); x + 10.0 }
                        };
                        let lbl_y = {
                            let fid = fid_lbl.clone();
                            move || { let (_, y) = get_tip(&fid); y - 10.0 }
                        };

                        view! {
                            <g>
                                <line
                                    x1=PANEL/2.0 y1=PANEL/2.0
                                    x2=line_x2 y2=line_y2
                                    stroke=color
                                    stroke-width=3
                                    stroke-linecap="round"
                                ></line>
                                <polygon
                                    points="0,0 -10,-5 -10,5"
                                    fill=color2
                                    transform=head_transform
                                ></polygon>
                                <circle
                                    cx=handle_cx cy=handle_cy r=HANDLE_R
                                    fill="white"
                                    stroke=color3
                                    stroke-width=2
                                    style="cursor: grab"
                                    on:pointerdown=move |ev: web_sys::PointerEvent| {
                                        ev.stop_propagation();
                                        set_dragging.set(Some(fid_down.clone()));
                                    }
                                ></circle>
                                <text
                                    x=lbl_x y=lbl_y
                                    fill=color4
                                    font-size="11"
                                    font-weight="600"
                                    pointer-events="none"
                                >{label}</text>
                            </g>
                        }
                    }).collect::<Vec<_>>()}
                </svg>
            </div>

            <div class="grid grid-cols-2 gap-x-3 gap-y-1 text-xs">
                {expected_forces.iter().map(|f| {
                    let fid_fb = f.id.clone();
                    let fid_ang = f.id.clone();
                    let label = f.label.clone();
                    let color = f.color.clone();
                    view! {
                        <div class="flex items-center gap-1.5">
                            <span class="inline-block w-2 h-2 rounded-full" style=format!("background-color: {}", color)></span>
                            <span class="text-gray-700 dark:text-gray-300 truncate">{label}</span>
                            <span class="ml-auto font-mono text-gray-500">
                                {move || format!("{:.0}°", angles.get().get(&fid_ang).copied().unwrap_or(0.0))}
                            </span>
                            {move || feedback.get().get(&fid_fb).map(|ok| {
                                if *ok { view! { <span class="text-green-600">"✓"</span> }.into_any() }
                                else { view! { <span class="text-red-600">"✗"</span> }.into_any() }
                            })}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            {move || if completed.get() {
                view! {
                    <div class="text-xs text-green-700 dark:text-green-400 font-medium">
                        {move || format!("FBD correct ({} attempts)", attempts.get())}
                    </div>
                }.into_any()
            } else {
                view! {
                    <button
                        class="w-full py-1.5 px-3 bg-gray-900 text-white dark:bg-white dark:text-gray-900 rounded text-xs font-medium hover:opacity-90"
                        on:click=on_check.clone()
                    >{move || format!("Check FBD (attempt {})", attempts.get() + 1)}</button>
                }.into_any()
            }}
        </div>
    }
}
