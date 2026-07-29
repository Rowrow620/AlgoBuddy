#![allow(
    dead_code,
    unused_imports,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::field_reassign_with_default,
    clippy::useless_vec,
    clippy::unnecessary_map_or,
    clippy::needless_range_loop,
    clippy::vec_init_then_push,
    clippy::single_char_add_str,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::useless_format,
    clippy::manual_div_ceil,
    clippy::unnecessary_min_or_max,
    clippy::int_plus_one,
    clippy::implicit_saturating_sub
)]

mod algorithms;
mod app;
mod model;
mod utils;

use app::VisualizerApp;

// ── Native Desktop Entry Point ──
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let force_all = args.iter().any(|a| a == "--all" || a == "--dev" || a == "-a") || cfg!(feature = "all-problems");

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
        Box::new(move |cc| {
            let mut app = VisualizerApp::new(cc);
            if force_all {
                app.set_show_unaudited(true);
            }
            Ok(Box::new(app))
        }),
    )
}

// ── WebAssembly (WASM) Entry Point ──
#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let runner = eframe::WebRunner::new();
        let _ = runner
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Ok(Box::new(VisualizerApp::new(cc)))),
            )
            .await;
    });
}




