// This small FFI boundary is isolated from the main workspace's unsafe-code prohibition.
// The Xcode UIKit host invokes this exported function as its application entry point.
use photocraft_engine::Session;
use photocraft_ui_egui::{PhotocraftApp, Services};

/// Starts PhotoCraft's native eframe/winit application from the iOS host.
#[unsafe(no_mangle)]
pub extern "C" fn start_photocraft_app() {
    if let Err(error) = eframe::run_native(
        "PhotoCraft",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(PhotocraftApp::new(Session::new(), Services::default())))),
    ) {
        log::error!("PhotoCraft failed to start on iPadOS: {error}");
    }
}
