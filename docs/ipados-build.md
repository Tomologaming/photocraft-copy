# iPadOS build status and next steps

## Current status

The iOS target probe in PR #2 checks only `photocraft-ui-egui`. It is not an iPad app build and does not produce an IPA.

The `photocraft` target now selects a dedicated iPadOS entry point and omits desktop-only shell dependencies (native desktop file dialogs, clipboard integration, and desktop automation). The iPadOS target check validates this entry point and the shared PhotoCraft UI/core dependency graph. Desktop behavior remains in the existing entry point, unchanged.

## Why a successful Rust target check is not an IPA

An iPad-installable package requires more than compiling Rust for `aarch64-apple-ios`:

1. A supported iOS application entry point and lifecycle integration for the chosen eframe/winit version.
2. An Xcode app target/bundle with an iOS `Info.plist`, launch configuration, icons, and required capabilities.
3. A signed app bundle. Sideloadly can sign/install a compatible IPA, but the archive still needs to be a valid iOS app.
4. Device testing on physical hardware, including touch, file import/export, lifecycle and GPU rendering.

The iPadOS entry point currently starts the shared PhotoCraft UI with default services. Native iPad document import/export, share-sheet integration, persistent app-container paths, and lifecycle/device validation still need dedicated implementation; this target check does not claim those features are complete.

## Packaging and signing policy

Do not publish a placeholder or renamed Rust binary as an IPA. Do not commit signing certificates, provisioning profiles, or Apple account secrets. Once the iOS entry point and Xcode target are implemented, CI can build an unsigned device archive or a signed IPA using explicitly configured secrets. Local Sideloadly signing remains an option if the generated app bundle is valid.

## Acceptance criteria

- [ ] Full `photocraft` target check succeeds for `aarch64-apple-ios`.
- [ ] A real iOS app entry point and Xcode project/build configuration exist.
- [ ] CI produces a structurally valid IPA, not just a Rust executable or static library.
- [ ] IPA installs and launches on a physical iPad.
- [ ] Image import, editing, save/export, suspend/resume and GPU rendering are verified on device.
