# PhotoCraft roadmap

**Stage: alpha** · next: beta, ~30 points (ready for real work ~45% → ~75%, with reliable PSD) and ~1,100–1,800 h away

> **Last reviewed:** 2026-10-11 · **Last updated:** 2026-10-13 · **Change:** minor (oracle floor 145 of 258 and 953 open issues, as in gaps.md and on GitHub; toolbox count 56 of 68, 12 missing) · **Target:** Adobe Photoshop 2026

PhotoCraft's one-page summary of how close it is to Adobe Photoshop 2026 (27.11.0, measured from
the installed app on 2026-10-10) at version 0.6.0. Detail: [target-app parity](docs/target-app-parity.md)
(methodology, weights, evidence), [gaps](docs/gaps.md) (the ranked work list),
[roadmap](docs/roadmap.md) (milestones, current focus), [scorecard](docs/scorecard.md) and
[menu checklist](docs/parity-checklist.md) (both generated and measured),
[architecture](docs/architecture.md).

Hours are Opus 5.5 agent wall-clock hours for one agent working sequentially, calibrated from this
repository's 1,004 merged PRs (see [calibration](docs/target-app-parity.md#remaining-effort-and-calibration)).
About 80% of the work parallelizes.

## Headline numbers

| | Value | Kind |
|---|---|---|
| **Feature breadth** | **~76%** | partly measured: menus 628/628, tools 56/68, panels 30/35, formats 13+4 of 31 |
| **Ready for real work** | **~45%** (40–50%) | estimated: weighted by dimension below |
| Mainstream practitioner | ~43% (38–48%) | estimated: weekly areas of a typical pro, with discounts ([method](docs/target-app-parity.md#mainstream-practitioner-43)) |
| Essentials user | ~61% (55–67%) | estimated: core features only ([method](docs/target-app-parity.md#essentials-user-61)) |
| Remaining to **beta** | **~1,100–1,800 h** | estimated |
| Remaining to **full parity** | **~2,300–4,100 h** | estimated; AI and plug-ins need owner decisions |

### Readiness by audience

| Audience | Ready % | Opus 5.5 agent-hours to ~95% | Work that dominates |
|---|---:|---:|---|
| Full target (ready for real work) | ~45% (additive weighted sum: 46%) | 2,300–4,100 | Everything below, plus AI tier, plug-in/scripting ecosystem, video, localization, hardware |
| Mainstream practitioner | ~43% | 1,700–2,800 | Depth in the 11 weekly areas (1,180–2,000), performance budgets, stability on real machines, PSD exchange with Photoshop |
| Essentials user | ~61% | 400–700 | Startup and GPU robustness, brush and transform responsiveness, Type tool, discoverability (flyouts, commit buttons), AVIF/PDF open |

Hours calibrated as in [Remaining effort](docs/target-app-parity.md#remaining-effort-and-calibration) (2–9 h per checklist
item, 8–20 h per format or subsystem, from the repo's 1,004 merged PRs); each tier is a subset of
the one above. About 80% parallelizes (independent crates and command modules); performance
architecture and the PSD writer are the serial parts.

**Why alpha.** Core workflows (open, layer, select, paint, adjust, filter, type, save) work end to
end, and breadth is high, but 953 issues are open (2026-10-13), 22 of 25 performance budgets
are missed, and PSDs we write are not yet reliably accepted by Photoshop (#1281, #2469). Beta
needs ~75% ready and no blocking PSD gap. The [alpha gate](docs/roadmap.md#alpha-gate) passes: all six core workflows
(retouch, composite, cut-out, design, paint, PSD exchange) complete end to end and save and reopen;
four are partial in depth, none blocked.

## By dimension

| Dimension | % ready | Hours to full | Doc |
|---|---:|---:|---|
| Features (depth) | 55% (breadth 76%) | 600–1,000 | [target-app-parity](docs/target-app-parity.md#feature-areas) |
| UI/UX fidelity (tools, handles, modifiers, brush feel, panels) | 40% | 350–600 | [ui-parity](docs/ui-parity.md), [brush-parity](docs/brush-parity.md), [context-menu-parity](docs/context-menu-parity.md) |
| File formats | 60% (presence ~80%) | 300–500 | [file-format-parity](docs/file-format-parity.md) |
| Hardware (GPU, pen, displays) | 50% | 80–150 | [hardware-parity](docs/hardware-parity.md) |
| Localization | 55% | 80–140 | [localization-parity](docs/localization-parity.md) |
| Performance | 25% (3/25 budgets met, measured) | 200–350 | [scorecard](docs/scorecard.md#performance) |
| Stability | 45% | 150–250 | [gaps](docs/gaps.md#g3-user-reported-bug-backlog) |
| Platforms | 80% | 30–60 | [scorecard](docs/scorecard.md#distribution) |
| Ecosystem / plug-ins / scripting | 15% | 250–450 | [gaps](docs/gaps.md#g16-plug-in-compatibility) |
| AI features | 10% | 300–600 | [gaps](docs/gaps.md#g15-ai-and-generative-features) |

## Features

| Area | % ready | Hours |
|---|---:|---:|
| Layers, masks, blend modes, smart objects, styles | 65% | 150–250 |
| Selections, Select and Mask, channels | 55% | 120–200 |
| Painting and brush engine | 55% | 150–250 |
| Retouching (healing, clone, patch, remove, content-aware) | 50% | 100–180 |
| Transform, warp, puppet, liquify | 55% | 100–180 |
| Adjustments and filters | 70% | 120–200 |
| Type | 40% | 120–200 |
| Vector shapes and paths | 55% | 80–140 |
| Colour management and modes | 70% | 60–100 |
| Camera Raw and computational photo | 45% | 120–200 |
| Automation, actions, scripting | 45% | 120–200 |
| File I/O | 60% | 300–500 |
| Print, export, Save for Web | 50% | 60–100 |
| Video / timeline, data-driven graphics | 30% | 100–200 |
| Workspace, panels, preferences | 40% | 150–250 |
| AI / generative | 10% | 300–600 |

Rows and evidence: [target-app-parity › Feature areas](docs/target-app-parity.md#feature-areas).

## Languages

Measured with the `cargo xtask i18n-coverage` key universe (1,608 keys). `partial` everywhere
because engine messages are still English. Detail: [localization-parity](docs/localization-parity.md).

| Language | Code | UI strings | Status |
|---|---|---:|---|
| English | `en` | 100% | full |
| Simplified Chinese | `zh-hans` | 100% | partial |
| Spanish | `es` | 100% | partial |
| Hindi | `hi` | 0% | none |
| Arabic | `ar` | 0% (RTL shell only) | none |
| French | `fr` | 100% | partial |
| Portuguese (Brazil) | `pt-br` | 100% | partial |
| Indonesian | `id` | 100% | partial |
| Japanese | `ja` | 100% | partial |
| German | `de` | 100% | partial |
| Korean | `ko` | 100% | partial |
| Vietnamese | `vi` | 0% | none |

Also shipped: Traditional Chinese, Russian, Ukrainian, Czech, Polish, Greek, Dutch, Italian (8).

## Upcoming

| Rank | Milestone | Estimate |
|---|---|---:|
| 1 | PSD that Photoshop trusts: Save As corruption, Photoshop re-open check, Photoshop-oracle floor 145 → 200+ of 258 | 150–250 h |
| 2 | Performance budgets: the 22 over-budget scenarios and the 150-layer GPU crash | 200–350 h |
| 3 | User bug backlog to under 100 open | 250–400 h |
| 4 | Brush and transform feel (brush-parity, ui-parity rows) | 160–290 h |
| 5 | The 12 missing tools | 60–100 h |

Detail and the next releases: [docs/roadmap.md](docs/roadmap.md).

## Progress log

| Date | Entry |
|---|---|
| 2026-10-10 | Full re-measure against Photoshop 2026 27.11.0 (installed bundle inspected): breadth ~76%, ready ~45%, alpha. Progress docs reorganized to the craftrules standard (`parity.md` → `parity-checklist.md`, `parity-estimate.md` merged into `target-app-parity.md`). 0.6.0 released. |
| 2026-10-09 | Remove tool, Rotate View, crop rotation; OpenRaster and Paint.NET; 39 Affinity documents in the corpus |
| 2026-10-08 | 0.5.0. Affinity import; Magnetic Lasso, Pattern Stamp, Mixer Brush, Pencil, Patch, Vertical Type on the toolbar |
| 2026-10-07 | 0.3.0. 10 UI languages; shortcut audit 214 → 0 failures; TIFF decode 10× faster |
| 2026-10-05 | 0.2.0, first signed and notarized release. Honest assessment: ready for real work 25–35% |
| 2026-10-03 | Menu breadth 625/625 |
| 2026-10-01 | Menu breadth 224 → 532 of 625 in a day |
| 2026-09-30 | First commit |

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-13 | minor | Focus item 1 quoted the oracle floor as 133 of 256; it is 145 of 258 (`crates/io/tests/corpus.rs`, gaps.md G1). Open issues 579 → 953 (2026-10-13) |
| 2026-10-11 | minor | Toolbox count 56 of 68 (12 missing) with the Convert Point tool (#3204) |
| 2026-10-10 | minor | Readiness by audience with hours per tier |
| 2026-10-10 | minor | Mainstream-practitioner and essentials-user numbers added |
| 2026-10-10 | minor | Alpha gate result (passes; stays alpha) in the stage explanation |
| 2026-10-10 | major | First version, per craftrules `standards/progress-docs.md` |


---

## iPadOS port (iPad A16, Sideloadly)

**Status: feasibility probe added; iOS target not yet verified** · Build environment: GitHub Actions macOS runners · Distribution target: sideloadable IPA

This is a separate platform port, not a claim that the current desktop UI already works on iPad. The existing app is Rust/egui/eframe and the browser build is Rust compiled to WebAssembly. The repository's `AGENTS.md` prohibits webview shells and JavaScript/TypeScript UI; respect that rule and resolve any proposed architecture change explicitly before coding. Feature parity with the desktop app is a goal, not an assumption. PhotoCraft is currently alpha, so port-specific compatibility and limitations must be recorded honestly.

### Phase 0 — feasibility and architecture

- [ ] Run the new `iPadOS target probe` workflow and record the actual `aarch64-apple-ios` compile result for `photocraft-ui-egui`.
- [ ] Audit `eframe`, `egui`, `wgpu`, `rfd`, codecs, fonts, and native dependencies for the `aarch64-apple-ios` target.
- [ ] Decide and document the approved iPad UI architecture without silently violating the Rust-first/no-webview rules.
- [ ] Define the minimum supported iPadOS version and test strategy for the iPad A16.
- [ ] Document supported file import/export, app sandbox access, persistence, memory pressure, and GPU constraints.
- [ ] Define IPA signing/provisioning requirements for Sideloadly; keep all signing secrets out of git.

### Phase 1 — build pipeline and app launch

- [ ] Add a dedicated GitHub Actions workflow using a macOS runner to build and validate the iPadOS target.
- [ ] Add an app bundle/project and required iPadOS metadata, icons, launch experience, and permissions.
- [ ] Produce a reproducible unsigned or appropriately signed IPA artifact compatible with the chosen Sideloadly workflow.
- [ ] Upload the IPA and build logs as workflow artifacts; clearly label experimental builds.
- [ ] Verify install, launch, relaunch, and uninstall/reinstall on a physical iPad A16.

### Phase 2 — usable editing baseline

- [ ] Make app navigation and tool targets comfortable for touch in portrait and landscape.
- [ ] Implement document open/import using the iPad document picker and Files provider locations.
- [ ] Implement Save, Save As, export, and recovery-safe handling of interrupted edits.
- [ ] Verify canvas pan, pinch zoom, selection, undo/redo, and keyboard/trackpad support where available.
- [ ] Verify the core workflow end to end: open an image, edit, save/export, close, and reopen.

### Phase 3 — iPad interaction and visual polish

- [ ] Add Apple Pencil input with pressure/tilt only where the current input pipeline can support it reliably.
- [ ] Add appropriate haptic feedback for discrete UI actions, with a setting to disable it; do not vibrate continuously while painting.
- [ ] Design adaptive toolbars, layer panels, popovers, and floating controls for iPad screen sizes.
- [ ] Add Liquid Glass styling where supported by the selected iPadOS SDK, with readable fallbacks for older versions.
- [ ] Respect Dynamic Type where practical, accessibility labels, Reduce Motion, and light/dark appearance.

### Phase 4 — compatibility and release quality

- [ ] Audit every desktop feature and classify it as working, adapted, unsupported, or unverified on iPadOS.
- [ ] Test PSD and other supported formats, large canvases, memory pressure, backgrounding, and recovery on device.
- [ ] Add automated checks for the iPad build and smoke-test launch; document manual device checks.
- [ ] Publish clear install/refresh instructions for Sideloadly, including signing expiry and re-signing caveats.
- [ ] Update the README platform claims only after the port has been built and verified; document known limitations and release status.

### Definition of done

- [ ] A clean GitHub Actions run produces a downloadable IPA artifact.
- [ ] That artifact installs and launches on a physical iPad A16 through the documented Sideloadly workflow.
- [ ] The documented core editing workflow passes on device and does not lose a saved document.
- [ ] Completed/unsupported features and test evidence are documented; `CHANGELOG.md`, `ROADMAP.md`, and, when applicable, `README.md` are updated.

### iPadOS implementation log

| Date | Change | Evidence / remaining work |
|---|---|---|
| 2026-10-17 | Added `.github/workflows/ios-target-probe.yml` to compile-check the shared Rust UI dependency graph on a macOS runner for `aarch64-apple-ios`. | Workflow is not yet run in this change; this is a feasibility probe, not an IPA or app-shell build. Desktop-only dependencies and the app entry point still need a target-specific audit. |
