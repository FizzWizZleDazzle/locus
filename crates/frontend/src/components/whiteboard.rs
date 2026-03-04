//! Whiteboard canvas component
//!
//! Full-area drawable canvas using Fabric.js (lazy-loaded).
//! Includes a floating toolbar for pen, text, eraser, color, clear, and brush size.
//! Text tool works like Khan Academy: click anywhere to create an editable text box.
//!
//! The Fabric.js canvas instance is stored on `window.__wb_canvas` so that all
//! interactions go through JS eval — this avoids holding a non-Send JsValue in
//! Rust closures (required by Leptos's reactive system).

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Active whiteboard tool
#[derive(Clone, Copy, PartialEq)]
enum Tool {
    Pen,
    Text,
    Eraser,
}

/// Lazy-loads Fabric.js by injecting a `<script>` tag, then calls `callback` on load.
fn ensure_fabric_loaded(callback: impl FnOnce() + 'static) {
    let window = web_sys::window().unwrap();

    let has_fabric = js_sys::Reflect::get(&window, &"fabric".into())
        .map(|v| !v.is_undefined() && !v.is_null())
        .unwrap_or(false);

    if has_fabric {
        callback();
        return;
    }

    let document = window.document().unwrap();
    let script = document
        .create_element("script")
        .unwrap()
        .unchecked_into::<web_sys::HtmlScriptElement>();
    script.set_src("https://cdn.jsdelivr.net/npm/fabric@5/dist/fabric.min.js");

    let cb = Closure::once_into_js(move || {
        callback();
    });
    script.set_onload(Some(cb.unchecked_ref()));

    document.head().unwrap().append_child(&script).unwrap();
}

/// Run JS code that has access to the canvas via `window.__wb_canvas`.
/// The code string can reference `c` as the canvas.
fn wb_eval(code: &str) {
    let wrapped = format!(
        "(function() {{ var c = window.__wb_canvas; if (!c) return; {} }})()",
        code
    );
    let _ = js_sys::eval(&wrapped);
}

/// Redraw the subtle background grid after a canvas clear.
fn wb_redraw_grid(is_dark: bool) {
    let grid_color = if is_dark {
        "rgba(255,255,255,0.04)"
    } else {
        "rgba(0,0,0,0.04)"
    };
    wb_eval(&format!(
        "var gs=30,w=c.width,h=c.height;\
         for(var x=gs;x<w;x+=gs)c.add(new fabric.Line([x,0,x,h],{{stroke:'{}',strokeWidth:1,selectable:false,evented:false,excludeFromExport:true}}));\
         for(var y=gs;y<h;y+=gs)c.add(new fabric.Line([0,y,w,y],{{stroke:'{}',strokeWidth:1,selectable:false,evented:false,excludeFromExport:true}}));\
         c.renderAll();",
        grid_color, grid_color
    ));
}

/// Initialize the Fabric.js canvas and store it on window.__wb_canvas.
fn init_fabric_canvas(canvas_id: &str, is_dark: bool) {
    let bg = if is_dark { "#030712" } else { "#f9fafb" };
    let grid_color = if is_dark {
        "rgba(255,255,255,0.04)"
    } else {
        "rgba(0,0,0,0.04)"
    };
    let text_color = if is_dark { "#e5e7eb" } else { "#1f2937" };
    let code = format!(
        r#"
        (function() {{
            var c = new fabric.Canvas('{}', {{
                isDrawingMode: true,
                backgroundColor: '{}',
                width: window.innerWidth,
                height: window.innerHeight
            }});
            c.freeDrawingBrush.color = '{}';
            c.freeDrawingBrush.width = 2;

            // Text tool: when __wb_text_mode is true, clicking adds an IText
            window.__wb_text_mode = false;
            window.__wb_text_color = '{}';
            c.on('mouse:down', function(opt) {{
                if (!window.__wb_text_mode) return;
                if (opt.target) return;
                var pointer = c.getPointer(opt.e);
                var text = new fabric.IText('', {{
                    left: pointer.x,
                    top: pointer.y,
                    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", system-ui, sans-serif',
                    fontSize: 18,
                    fill: window.__wb_text_color,
                    editable: true
                }});
                c.add(text);
                c.setActiveObject(text);
                text.enterEditing();
            }});

            // Draw subtle grid
            var gridSize = 30;
            var w = window.innerWidth;
            var h = window.innerHeight;
            for (var x = gridSize; x < w; x += gridSize) {{
                c.add(new fabric.Line([x, 0, x, h], {{
                    stroke: '{}', strokeWidth: 1,
                    selectable: false, evented: false, excludeFromExport: true
                }}));
            }}
            for (var y = gridSize; y < h; y += gridSize) {{
                c.add(new fabric.Line([0, y, w, y], {{
                    stroke: '{}', strokeWidth: 1,
                    selectable: false, evented: false, excludeFromExport: true
                }}));
            }}
            c.renderAll();

            window.__wb_canvas = c;
        }})()
        "#,
        canvas_id, bg, text_color, text_color, grid_color, grid_color
    );
    let _ = js_sys::eval(&code);
}

/// Whiteboard canvas with toolbar overlay.
///
/// - Full-area `<canvas>` behind content (z-index: 0)
/// - Floating toolbar: pen, text, eraser, clear, color, brush size
/// - Text tool: click canvas to create editable text boxes (Khan Academy style)
/// - Resizes on window resize
/// - Clears on problem change (via `problem_id`)
#[component]
pub fn Whiteboard(
    /// Current problem ID — canvas clears when this changes
    #[prop(into)]
    problem_id: Signal<String>,
    /// Whether dark mode is active
    #[prop(into)]
    is_dark: Signal<bool>,
) -> impl IntoView {
    let (loading, set_loading) = signal(true);
    let (canvas_ready, set_canvas_ready) = signal(false);
    let (active_color, set_active_color) = signal(String::from("#1f2937"));
    let (brush_size, set_brush_size) = signal(2u32);
    let (active_tool, set_active_tool) = signal(Tool::Pen);

    let canvas_id = "whiteboard-canvas";

    // Initialize canvas after mount
    Effect::new(move |_| {
        let dark = is_dark.get();
        set_loading.set(true);
        ensure_fabric_loaded(move || {
            init_fabric_canvas("whiteboard-canvas", dark);
            set_canvas_ready.set(true);
            set_loading.set(false);
        });
    });

    // Dispose Fabric.js canvas on unmount — removes wrapper DOM elements,
    // event listeners, and the crosshair cursor that would otherwise persist.
    on_cleanup(move || {
        let _ = js_sys::eval(
            "(function() { if (window.__wb_canvas) { window.__wb_canvas.dispose(); window.__wb_canvas = null; } })()",
        );
    });

    // Resize canvas on window resize
    Effect::new(move |_| {
        if !canvas_ready.get() {
            return;
        }
        let resize = Closure::<dyn FnMut()>::new(move || {
            let window = web_sys::window().unwrap();
            let w = window.inner_width().unwrap().as_f64().unwrap_or(1920.0);
            let h = window.inner_height().unwrap().as_f64().unwrap_or(1080.0);
            wb_eval(&format!(
                "c.setDimensions({{ width: {}, height: {} }}); c.renderAll();",
                w, h
            ));
        });

        if let Some(window) = web_sys::window() {
            let _ =
                window.add_event_listener_with_callback("resize", resize.as_ref().unchecked_ref());
        }
        resize.forget();
    });

    // Clear canvas when problem changes
    Effect::new(move |prev_id: Option<String>| {
        let current_id = problem_id.get();
        if let Some(prev) = prev_id {
            if prev != current_id && canvas_ready.get_untracked() {
                let dark = is_dark.get_untracked();
                let bg = if dark { "#030712" } else { "#f9fafb" };
                wb_eval(&format!("c.clear(); c.backgroundColor = '{}';", bg));
                wb_redraw_grid(dark);
            }
        }
        current_id
    });

    // Update brush color
    Effect::new(move |_| {
        let color = active_color.get();
        if canvas_ready.get() && active_tool.get_untracked() == Tool::Pen {
            wb_eval(&format!("c.freeDrawingBrush.color = '{}';", color));
        }
        // Also update text color for new text objects
        if let Some(window) = web_sys::window() {
            let _ = js_sys::Reflect::set(
                &window,
                &"__wb_text_color".into(),
                &JsValue::from_str(&color),
            );
        }
    });

    // Update brush size
    Effect::new(move |_| {
        let size = brush_size.get();
        if canvas_ready.get() && active_tool.get_untracked() != Tool::Eraser {
            wb_eval(&format!("c.freeDrawingBrush.width = {};", size));
        }
    });

    // Helper to set window.__wb_text_mode
    let set_text_mode = move |enabled: bool| {
        if let Some(window) = web_sys::window() {
            let _ = js_sys::Reflect::set(
                &window,
                &"__wb_text_mode".into(),
                &JsValue::from_bool(enabled),
            );
        }
    };

    // Pen button
    let on_pen = move |_| {
        set_active_tool.set(Tool::Pen);
        set_text_mode(false);
        let color = active_color.get_untracked();
        wb_eval(&format!(
            "c.freeDrawingBrush = new fabric.PencilBrush(c); c.freeDrawingBrush.color = '{}'; c.freeDrawingBrush.width = {}; c.isDrawingMode = true;",
            color,
            brush_size.get_untracked()
        ));
    };

    // Text button
    let on_text = move |_| {
        set_active_tool.set(Tool::Text);
        set_text_mode(true);
        wb_eval("c.isDrawingMode = false;");
    };

    // Eraser
    let on_eraser = move |_| {
        set_active_tool.set(Tool::Eraser);
        set_text_mode(false);
        let eraser_color = if is_dark.get_untracked() {
            "#030712"
        } else {
            "#f9fafb"
        };
        wb_eval(&format!(
            "c.freeDrawingBrush = new fabric.PencilBrush(c); c.freeDrawingBrush.color = '{}'; c.freeDrawingBrush.width = 20; c.isDrawingMode = true;",
            eraser_color
        ));
    };

    // Clear all
    let on_clear = move |_| {
        let dark = is_dark.get_untracked();
        let bg = if dark { "#030712" } else { "#f9fafb" };
        wb_eval(&format!("c.clear(); c.backgroundColor = '{}';", bg));
        wb_redraw_grid(dark);
    };

    let colors: Vec<(&str, &str)> = vec![
        ("#1f2937", "bg-gray-800"),
        ("#2563eb", "bg-blue-600"),
        ("#dc2626", "bg-red-600"),
        ("#16a34a", "bg-green-600"),
    ];

    let dark_colors: Vec<(&str, &str)> = vec![
        ("#e5e7eb", "bg-gray-200"),
        ("#60a5fa", "bg-blue-400"),
        ("#f87171", "bg-red-400"),
        ("#4ade80", "bg-green-400"),
    ];

    view! {
        // Canvas element (full area, behind everything)
        <canvas
            id=canvas_id
            class="absolute inset-0 z-0"
            style="width: 100%; height: 100%;"
        ></canvas>

        // Loading indicator
        {move || loading.get().then(|| view! {
            <div class="absolute inset-0 z-10 flex items-center justify-center bg-gray-50/80 dark:bg-gray-900/80">
                <span class="text-gray-500 text-sm">"Loading whiteboard..."</span>
            </div>
        })}

        // Floating toolbar
        <div
            class="absolute bottom-4 right-4 z-30 flex items-center gap-1.5 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg px-2 py-1.5"
            on:pointerdown=|ev: web_sys::PointerEvent| ev.stop_propagation()
            on:pointermove=|ev: web_sys::PointerEvent| ev.stop_propagation()
        >
            // Pen
            <button
                class=move || format!(
                    "p-1.5 rounded {} text-xs",
                    if active_tool.get() == Tool::Pen { "bg-gray-200 dark:bg-gray-600" } else { "hover:bg-gray-100 dark:hover:bg-gray-700" }
                )
                on:click=on_pen
                title="Draw"
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z">
                    </path>
                </svg>
            </button>

            // Text
            <button
                class=move || format!(
                    "p-1.5 rounded {} text-xs font-bold",
                    if active_tool.get() == Tool::Text { "bg-gray-200 dark:bg-gray-600" } else { "hover:bg-gray-100 dark:hover:bg-gray-700" }
                )
                on:click=on_text
                title="Text — click on canvas to type"
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M4 6h16M4 6v2m16-2v2M7 6v12m0 0h2m-2 0H5m12-12v12m0 0h2m-2 0h-2">
                    </path>
                </svg>
            </button>

            // Eraser
            <button
                class=move || format!(
                    "p-1.5 rounded {} text-xs",
                    if active_tool.get() == Tool::Eraser { "bg-gray-200 dark:bg-gray-600" } else { "hover:bg-gray-100 dark:hover:bg-gray-700" }
                )
                on:click=on_eraser
                title="Eraser"
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16">
                    </path>
                </svg>
            </button>

            // Separator
            <div class="w-px h-5 bg-gray-300 dark:bg-gray-600 mx-0.5"></div>

            // Color dots
            {move || {
                let cs = if is_dark.get() { &dark_colors } else { &colors };
                cs.iter().map(|(hex, _class)| {
                    let hex = hex.to_string();
                    let hex1 = hex.clone();
                    let hex2 = hex.clone();
                    let hex3 = hex.clone();
                    let hex4 = hex.clone();
                    let hex_for_click = hex.clone();
                    view! {
                        <button
                            class="w-5 h-5 rounded-full border-2 transition-all"
                            class:border-gray-400=move || active_color.get() != hex1
                            class:border-gray-900=move || active_color.get() == hex2 && !is_dark.get()
                            class:border-white=move || active_color.get() == hex3 && is_dark.get()
                            class:scale-110=move || active_color.get() == hex4
                            style=format!("background-color: {}", hex)
                            on:click=move |_| {
                                set_active_color.set(hex_for_click.clone());
                                if active_tool.get_untracked() == Tool::Pen {
                                    wb_eval(&format!(
                                        "c.freeDrawingBrush.color = '{}';",
                                        hex_for_click
                                    ));
                                }
                            }
                        ></button>
                    }
                }).collect::<Vec<_>>()
            }}

            // Separator
            <div class="w-px h-5 bg-gray-300 dark:bg-gray-600 mx-0.5"></div>

            // Brush size buttons
            <button
                class=move || format!(
                    "p-1.5 rounded text-xs {}",
                    if brush_size.get() == 1 { "bg-gray-200 dark:bg-gray-600" } else { "hover:bg-gray-100 dark:hover:bg-gray-700" }
                )
                on:click=move |_| set_brush_size.set(1)
                title="Small brush"
            >
                <div class="w-1.5 h-1.5 rounded-full bg-current"></div>
            </button>
            <button
                class=move || format!(
                    "p-1.5 rounded text-xs {}",
                    if brush_size.get() == 2 { "bg-gray-200 dark:bg-gray-600" } else { "hover:bg-gray-100 dark:hover:bg-gray-700" }
                )
                on:click=move |_| set_brush_size.set(2)
                title="Medium brush"
            >
                <div class="w-2.5 h-2.5 rounded-full bg-current"></div>
            </button>
            <button
                class=move || format!(
                    "p-1.5 rounded text-xs {}",
                    if brush_size.get() == 5 { "bg-gray-200 dark:bg-gray-600" } else { "hover:bg-gray-100 dark:hover:bg-gray-700" }
                )
                on:click=move |_| set_brush_size.set(5)
                title="Large brush"
            >
                <div class="w-3.5 h-3.5 rounded-full bg-current"></div>
            </button>

            // Separator
            <div class="w-px h-5 bg-gray-300 dark:bg-gray-600 mx-0.5"></div>

            // Clear all
            <button
                class="p-1.5 rounded hover:bg-red-100 dark:hover:bg-red-900/30 text-red-500"
                on:click=on_clear
                title="Clear all"
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16">
                    </path>
                </svg>
            </button>
        </div>
    }
}
