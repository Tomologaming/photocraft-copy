// The Xcode iPadOS host links this library and calls the exported app entry point.
#[cfg(target_os = "ios")]
include!("ios_main.rs");
