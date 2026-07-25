mod algorithms;
mod app;
mod model;

use app::VisualizerApp;

// ── Native Desktop Entry Point ──
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("AlgoBuddy — NeetCode Roadmap Visualizer")
            .with_inner_size([1400.0, 850.0])
            .with_min_inner_size([1000.0, 650.0]),
        ..Default::default()
    };

    eframe::run_native(
        "AlgoBuddy — NeetCode Roadmap Visualizer",
        native_options,
        Box::new(|cc| Ok(Box::new(VisualizerApp::new(cc)))),
    )
}

// ── WebAssembly (WASM) Entry Point ──
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect panics to browser console
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let start_result = eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Ok(Box::new(VisualizerApp::new(cc)))),
            )
            .await;

        let _ = start_result;
    });
}

