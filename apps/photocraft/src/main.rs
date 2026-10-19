#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

// Keep the established desktop entry point unchanged while compiling a separate iPadOS shell.
#[cfg(not(target_os = "ios"))]
include!("desktop_main.rs");

#[cfg(target_os = "ios")]
include!("ios_main.rs");

#[cfg(target_os = "ios")]
fn main() {
    start_photocraft_app();
}
