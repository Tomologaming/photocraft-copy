# Changelog

Notable changes to the PhotoCraft iPadOS port and its build/install workflow are recorded here. This file supplements the main PhotoCraft release notes; entries describe implemented changes, not planned work.

## Unreleased

- Kept Linux-only Wayland/X11 eframe backends out of the iOS dependency graph and excluded desktop file-dialog/clipboard/open-with dependencies from iOS targets, addressing the first full-app target-check blocker.
- Added a macOS workflow that checks the complete `photocraft` app dependency graph for `aarch64-apple-ios`; this is a diagnostic target check, not an IPA build.
- Documented the concrete iPadOS packaging blockers and acceptance criteria, including a real iOS app entry point, Xcode bundle configuration, signing, and physical-device verification.
- Added a macOS GitHub Actions workflow to compile-check PhotoCraft's shared Rust UI dependency graph for `aarch64-apple-ios`.
- Fixed a compile error found by the first checks: preset loading now initializes the persisted `export_presets` field. The checks need to rerun to verify the fix; no IPA is produced by this probe.

- Added an initial checkbox-based iPadOS port roadmap for GitHub Actions macOS builds and Sideloadly installation.
- Added contributor/agent instructions requiring changelog and roadmap updates after each completed task, with README updates when user-facing documentation changes.
