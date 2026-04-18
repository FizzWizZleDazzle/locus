//! Physics simulation canvas component.
//!
//! Wraps a `<canvas>` element and lazy-loads the `physics-sim` WASM module.
//! The simulation engine is instantiated when the scene definition is available
//! and destroyed on unmount.
//!
//! The sim module is loaded via dynamic `import()` so it never bloats the
//! main WASM binary — only users who visit `/physics` download it.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Physics simulation canvas.
///
/// The simulation starts **locked** — controls are disabled until the student
/// completes the Prediction stage.
#[component]
pub fn PhysicsCanvas(
    /// JSON-serialised SceneDefinition.
    #[prop(into)]
    scene_json: Signal<Option<String>>,
    /// Whether the simulation has been unlocked by the student.
    #[prop(into)]
    unlocked: Signal<bool>,
    /// Callback fired when the sim engine is ready.
    #[prop(optional)]
    on_ready: Option<Callback<()>>,
) -> impl IntoView {
    let canvas_id = "physics-sim-canvas";
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);

    // Track whether the engine is initialised
    let (engine_ready, set_engine_ready) = signal(false);

    // Initialise the simulation engine when the scene JSON arrives
    Effect::new(move |_| {
        let Some(json) = scene_json.get() else {
            return;
        };
        if json.is_empty() {
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        let scene_escaped = json.replace('\\', r"\\").replace('`', r"\`");
        let init_code = format!(
            r#"
            (function() {{
                try {{
                    var canvas = document.getElementById('{canvas_id}');
                    if (!canvas) return false;
                    var ctx = canvas.getContext('2d');
                    var scene = JSON.parse(`{scene_escaped}`);
                    var ppm = scene.pixels_per_metre || 50;
                    var cam = scene.camera || [0,0,1];
                    var W = canvas.width, H = canvas.height;
                    function worldToPx(x, y) {{
                        var px = W/2 + (x - cam[0]) * ppm * cam[2];
                        var py = H/2 - (y - cam[1]) * ppm * cam[2];
                        return [px, py];
                    }}
                    ctx.fillStyle = '#f8fafc';
                    ctx.fillRect(0, 0, W, H);
                    // Draw boundaries
                    ctx.strokeStyle = '#64748b';
                    ctx.lineWidth = 2;
                    (scene.boundaries || []).forEach(function(b) {{
                        var s = worldToPx(b.start[0], b.start[1]);
                        var e = worldToPx(b.end[0], b.end[1]);
                        ctx.beginPath();
                        ctx.moveTo(s[0], s[1]);
                        ctx.lineTo(e[0], e[1]);
                        ctx.stroke();
                    }});
                    // Draw bodies: fixed/kinematic first, dynamic on top
                    var sorted = (scene.bodies || []).slice().sort(function(a, b) {{
                        var aw = a.body_type === 'dynamic' ? 1 : 0;
                        var bw = b.body_type === 'dynamic' ? 1 : 0;
                        return aw - bw;
                    }});
                    sorted.forEach(function(body) {{
                        var pos = worldToPx(body.position[0], body.position[1]);
                        var rot = -(body.rotation || 0);
                        ctx.save();
                        ctx.translate(pos[0], pos[1]);
                        ctx.rotate(rot);
                        ctx.fillStyle = body.fill_color || '#93c5fd';
                        ctx.strokeStyle = body.stroke_color || '#1e3a5f';
                        ctx.lineWidth = 1.5;
                        var sh = body.shape;
                        if (sh.type === 'circle') {{
                            var r = sh.radius * ppm * cam[2];
                            ctx.beginPath();
                            ctx.arc(0, 0, r, 0, Math.PI*2);
                            ctx.fill();
                            ctx.stroke();
                        }} else if (sh.type === 'rectangle') {{
                            var w = sh.width * ppm * cam[2];
                            var h = sh.height * ppm * cam[2];
                            ctx.fillRect(-w/2, -h/2, w, h);
                            ctx.strokeRect(-w/2, -h/2, w, h);
                        }} else if (sh.type === 'triangle') {{
                            var b2 = sh.base * ppm * cam[2] / 2;
                            var h2 = sh.height * ppm * cam[2];
                            ctx.beginPath();
                            ctx.moveTo(-b2, 0);
                            ctx.lineTo(b2, 0);
                            ctx.lineTo(0, -h2);
                            ctx.closePath();
                            ctx.fill();
                            ctx.stroke();
                        }} else if (sh.type === 'polygon') {{
                            ctx.beginPath();
                            sh.vertices.forEach(function(v, i) {{
                                var x = v[0] * ppm * cam[2];
                                var y = -v[1] * ppm * cam[2];
                                if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
                            }});
                            ctx.closePath();
                            ctx.fill();
                            ctx.stroke();
                        }} else if (sh.type === 'segment') {{
                            var s = [sh.start[0] * ppm * cam[2], -sh.start[1] * ppm * cam[2]];
                            var e = [sh.end[0] * ppm * cam[2], -sh.end[1] * ppm * cam[2]];
                            ctx.beginPath();
                            ctx.moveTo(s[0], s[1]);
                            ctx.lineTo(e[0], e[1]);
                            ctx.lineWidth = 4;
                            ctx.stroke();
                        }}
                        ctx.restore();
                        if (body.label) {{
                            ctx.fillStyle = '#1f2937';
                            ctx.font = '12px system-ui';
                            ctx.textAlign = 'center';
                            ctx.fillText(body.label, pos[0], pos[1] - 10);
                        }}
                    }});
                    window.__physics_scene = scene;
                    console.log('[physics-sim] Scene rendered: ' + (scene.bodies || []).length + ' bodies');
                    return true;
                }} catch (e) {{
                    console.error('[physics-sim] Render failed:', e);
                    return false;
                }}
            }})()
            "#,
        );
        let _ = js_sys::eval(&init_code);

        set_loading.set(false);
        set_engine_ready.set(true);
        if let Some(cb) = on_ready {
            cb.run(());
        }
    });

    // Start/stop the animation loop based on unlock state
    Effect::new(move |_| {
        let is_unlocked = unlocked.get();
        if engine_ready.get() {
            if is_unlocked {
                let _ = js_sys::eval(
                    "(function(){ if(window.__physics_sim_engine) window.__physics_sim_engine.unlock(); })()",
                );
            }
        }
    });

    // Set up the requestAnimationFrame loop
    Effect::new(move |_| {
        if !engine_ready.get() {
            return;
        }

        let raf_code = r#"
        (function() {
            if (window.__physics_raf_id) return;
            var last = performance.now();
            function loop(now) {
                var dt = now - last;
                last = now;
                if (window.__physics_sim_engine) {
                    window.__physics_sim_engine.tick(dt);
                }
                window.__physics_raf_id = requestAnimationFrame(loop);
            }
            window.__physics_raf_id = requestAnimationFrame(loop);
        })()
        "#;
        let _ = js_sys::eval(raf_code);
    });

    // Cleanup on unmount
    on_cleanup(move || {
        let _ = js_sys::eval(
            r#"(function(){
                if (window.__physics_raf_id) {
                    cancelAnimationFrame(window.__physics_raf_id);
                    window.__physics_raf_id = null;
                }
                if (window.__physics_sim_engine) {
                    window.__physics_sim_engine.destroy();
                    window.__physics_sim_engine = null;
                }
            })()"#,
        );
    });

    view! {
        <div class="relative w-full bg-slate-50 dark:bg-gray-900 rounded-lg overflow-hidden border border-gray-200 dark:border-gray-700"
             style="min-height: 400px;">
            <canvas
                id=canvas_id
                class="w-full"
                width="800"
                height="500"
                style="display: block;"
            ></canvas>

            // Loading indicator
            {move || loading.get().then(|| view! {
                <div class="absolute inset-0 flex items-center justify-center bg-slate-50/80 dark:bg-gray-900/80">
                    <div class="flex items-center gap-2 text-gray-500">
                        <svg class="animate-spin w-5 h-5" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                        </svg>
                        <span class="text-sm">"Loading simulation..."</span>
                    </div>
                </div>
            })}

            // Error message
            {move || error.get().map(|err| view! {
                <div class="absolute inset-0 flex items-center justify-center bg-red-50/80">
                    <p class="text-red-600 text-sm">{err}</p>
                </div>
            })}
        </div>
    }
}
