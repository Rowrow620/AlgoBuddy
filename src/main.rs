mod algorithms;
mod app;
mod model;

use app::VisualizerApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
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
