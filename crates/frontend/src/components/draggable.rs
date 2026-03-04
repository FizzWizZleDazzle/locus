//! Generic draggable wrapper component
//!
//! Wraps children in an absolutely-positioned div with a drag handle.
//! Only the handle initiates dragging; content inside remains interactive.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;

/// Draggable wrapper that positions children via CSS transform.
///
/// - Drag handle at the top (grip bar) initiates drag
/// - `stopPropagation` on pointer events prevents canvas interference
/// - Boundary clamping keeps the card within the viewport
#[component]
pub fn Draggable(
    /// Content to render inside the draggable container
    children: Children,
    /// Whether the card is collapsed (shows collapse/expand toggle in handle)
    #[prop(into)]
    collapsed: ReadSignal<bool>,
    /// Callback to toggle collapsed state
    on_toggle_collapse: Callback<()>,
    /// Initial X position (px from left)
    #[prop(default = 32.0)]
    initial_x: f64,
    /// Initial Y position (px from top)
    #[prop(default = 32.0)]
    initial_y: f64,
) -> impl IntoView {
    let (pos_x, set_pos_x) = signal(initial_x);
    let (pos_y, set_pos_y) = signal(initial_y);
    let (dragging, set_dragging) = signal(false);
    let (offset_x, set_offset_x) = signal(0.0);
    let (offset_y, set_offset_y) = signal(0.0);

    // Mouse down on handle: start drag
    let on_handle_mousedown = move |ev: MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        set_dragging.set(true);
        set_offset_x.set(ev.client_x() as f64 - pos_x.get_untracked());
        set_offset_y.set(ev.client_y() as f64 - pos_y.get_untracked());
    };

    // Global mousemove: update position with boundary clamping
    let on_mousemove = Closure::<dyn FnMut(MouseEvent)>::new(move |ev: MouseEvent| {
        if !dragging.get_untracked() {
            return;
        }
        ev.prevent_default();

        let new_x = ev.client_x() as f64 - offset_x.get_untracked();
        let new_y = ev.client_y() as f64 - offset_y.get_untracked();

        // Clamp within viewport
        let window = web_sys::window().unwrap();
        let vw = window.inner_width().unwrap().as_f64().unwrap_or(1920.0);
        let vh = window.inner_height().unwrap().as_f64().unwrap_or(1080.0);

        set_pos_x.set(new_x.max(0.0).min(vw - 100.0));
        set_pos_y.set(new_y.max(0.0).min(vh - 60.0));
    });

    // Global mouseup: stop drag
    let on_mouseup = Closure::<dyn FnMut(MouseEvent)>::new(move |_: MouseEvent| {
        set_dragging.set(false);
    });

    // Attach global listeners
    if let Some(window) = web_sys::window() {
        let _ = window
            .add_event_listener_with_callback("mousemove", on_mousemove.as_ref().unchecked_ref());
        let _ =
            window.add_event_listener_with_callback("mouseup", on_mouseup.as_ref().unchecked_ref());
    }
    on_mousemove.forget();
    on_mouseup.forget();

    // Touch support
    let on_handle_touchstart = move |ev: web_sys::TouchEvent| {
        ev.stop_propagation();
        if let Some(touch) = ev.touches().get(0) {
            set_dragging.set(true);
            set_offset_x.set(touch.client_x() as f64 - pos_x.get_untracked());
            set_offset_y.set(touch.client_y() as f64 - pos_y.get_untracked());
        }
    };

    let on_touchmove =
        Closure::<dyn FnMut(web_sys::TouchEvent)>::new(move |ev: web_sys::TouchEvent| {
            if !dragging.get_untracked() {
                return;
            }
            ev.prevent_default();
            if let Some(touch) = ev.touches().get(0) {
                let new_x = touch.client_x() as f64 - offset_x.get_untracked();
                let new_y = touch.client_y() as f64 - offset_y.get_untracked();

                let window = web_sys::window().unwrap();
                let vw = window.inner_width().unwrap().as_f64().unwrap_or(1920.0);
                let vh = window.inner_height().unwrap().as_f64().unwrap_or(1080.0);

                set_pos_x.set(new_x.max(0.0).min(vw - 100.0));
                set_pos_y.set(new_y.max(0.0).min(vh - 60.0));
            }
        });

    let on_touchend =
        Closure::<dyn FnMut(web_sys::TouchEvent)>::new(move |_: web_sys::TouchEvent| {
            set_dragging.set(false);
        });

    if let Some(window) = web_sys::window() {
        let _ = window
            .add_event_listener_with_callback("touchmove", on_touchmove.as_ref().unchecked_ref());
        let _ = window
            .add_event_listener_with_callback("touchend", on_touchend.as_ref().unchecked_ref());
    }
    on_touchmove.forget();
    on_touchend.forget();

    view! {
        <div
            class="absolute z-20"
            style=move || format!(
                "transform: translate({}px, {}px); will-change: transform;",
                pos_x.get(), pos_y.get()
            )
            on:pointerdown=|ev: web_sys::PointerEvent| ev.stop_propagation()
            on:pointermove=|ev: web_sys::PointerEvent| ev.stop_propagation()
        >
            <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg shadow-xl overflow-hidden"
                style="min-width: 340px; max-width: 560px;">
                // Drag handle
                <div
                    class="flex items-center justify-between px-3 py-1.5 bg-gray-100 dark:bg-gray-800 cursor-grab active:cursor-grabbing select-none border-b border-gray-200 dark:border-gray-700"
                    on:mousedown=on_handle_mousedown
                    on:touchstart=on_handle_touchstart
                >
                    // Grip icon
                    <div class="flex gap-0.5">
                        <div class="flex flex-col gap-0.5">
                            <div class="w-1 h-1 rounded-full bg-gray-400"></div>
                            <div class="w-1 h-1 rounded-full bg-gray-400"></div>
                            <div class="w-1 h-1 rounded-full bg-gray-400"></div>
                        </div>
                        <div class="flex flex-col gap-0.5">
                            <div class="w-1 h-1 rounded-full bg-gray-400"></div>
                            <div class="w-1 h-1 rounded-full bg-gray-400"></div>
                            <div class="w-1 h-1 rounded-full bg-gray-400"></div>
                        </div>
                    </div>

                    // Collapse/expand toggle
                    <button
                        class="p-1 hover:bg-gray-200 dark:hover:bg-gray-700 rounded text-gray-500 text-xs"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            on_toggle_collapse.run(());
                        }
                        title=move || if collapsed.get() { "Expand" } else { "Collapse" }
                    >
                        {move || if collapsed.get() {
                            view! { <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg> }.into_any()
                        } else {
                            view! { <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7"></path></svg> }.into_any()
                        }}
                    </button>
                </div>

                // Content
                <div class="p-4">
                    {children()}
                </div>
            </div>
        </div>
    }
}
