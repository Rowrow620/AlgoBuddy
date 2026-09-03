mod algorithms;
mod app;
mod engine;
mod model;
mod shortcuts;
mod terminal;
mod ui;
mod utils;

use app::VisualizerApp;

// Native entry point.
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

// WebAssembly entry point.
#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    eframe::WebLogger::init(log::LevelFilter::Warn).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let runner = eframe::WebRunner::new();
        if let Err(error) = runner
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Ok(Box::new(VisualizerApp::new(cc)))),
            )
            .await
        {
            log::error!("Failed to start AlgoBuddy WebAssembly app: {error:?}");
        }
    });
}
