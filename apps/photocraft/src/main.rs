// Keep the established desktop entry point unchanged while compiling a separate iPadOS shell.
#[cfg(not(target_os = "ios"))]
include!("desktop_main.rs");

#[cfg(target_os = "ios")]
include!("ios_main.rs");
