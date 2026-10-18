//! iPadOS entry point. Desktop shell integrations are intentionally not compiled on iOS.
use photocraft_engine::Session;
use photocraft_ui_egui::{PhotocraftApp, Services};

fn main() -> eframe::Result {
    eframe::run_native(
        "PhotoCraft",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(PhotocraftApp::new(Session::new(), Services::default())))),
    )
}
