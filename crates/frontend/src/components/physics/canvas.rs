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

        // Use JS eval to dynamically import the physics-sim WASM module
        // and instantiate the engine on the canvas.
        let init_code = format!(
            r#"
            (async function() {{
                try {{
                    // The physics sim WASM module is built by Trunk as a
                    // separate asset and placed in the dist directory.
                    // For development, we fall back to a placeholder.
                    if (!window.__physics_sim_engine) {{
                        // Placeholder: in production this would be:
                        // const mod = await import('/physics_sim.js');
                        // await mod.default();
                        // window.__physics_sim_engine = new mod.SimulationEngine('{canvas_id}', sceneJson);
                        console.log('[physics-sim] Engine placeholder initialised');
                    }}
                    return true;
                }} catch (e) {{
                    console.error('[physics-sim] Failed to initialise:', e);
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
