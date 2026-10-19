# iPadOS build

## Build workflow

The `iPadOS IPA` workflow runs on a macOS 15 runner. It builds the Rust app as a static library
for `aarch64-apple-ios`, generates an Xcode project, links the library into a native iOS app host,
and packages the resulting `.app` directory into an IPA.

Run it from **Actions → iPadOS IPA → Run workflow**. Changes to the app, shared crates, Cargo
manifests, the iOS host/project, or this workflow also trigger a build on `main`. The output is
uploaded as the `PhotoCraft-iPadOS-unsigned-IPA` artifact for 14 days.

## Signing and current limitations

The artifact is **unsigned** because this repository has not been configured with an Apple signing
certificate and provisioning profile. It is intended for validating the native build and bundle
structure, not direct installation on a physical iPad or distribution through TestFlight/App Store.
Signing should be added only after the matching Apple certificate and provisioning profile are
configured as GitHub Actions secrets; never commit those credentials.

The iPadOS entry point starts the shared PhotoCraft UI with default services. Native document
import/export, share-sheet integration, persistent app-container paths, app icons, and device testing
for touch, lifecycle, and GPU rendering remain follow-up work. A successful CI package is not a
claim that these on-device behaviors have been validated.

The general desktop CI, documentation, packaging-lint, and standalone iOS target-probe workflows
are manual-only; they do not run automatically on pushes or pull requests.
