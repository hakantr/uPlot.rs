# uPlot.rs

**English** · [Türkçe](README.md)

This project is a port of [uPlot](https://github.com/leeoniya/uPlot) 1.6.32's
small, fast, and memory-efficient charting approach to Rust, GPUI, and WASM.
It is not an independently invented charting engine. The normative source is
[commit `0e5812c` in the uPlot repository](https://github.com/leeoniya/uPlot/commit/0e5812c504430f5c804e0f993376d8999b26cc34);
uPlot defines the behavioral, API, and visual compatibility target.

The codebase uses Rust 2024 edition and requires Rust 1.95 or newer. New
modules use `foo.rs` and, when needed, `foo/submodule.rs` instead of `mod.rs`.

`gpui` and `gpui_kutuphanesi` are intentionally not pinned to commits. Local
builds use the current sibling worktrees through path dependencies, while CI
uses the current default branches of both repositories. Only the normative
uPlot source is commit-locked.

The port currently contains the Phase 0 foundation and the first vertical
compatibility card:

- validated aligned/columnar data model;
- numeric X scales and fixed/automatic Y ranges;
- deterministic, GPUI-independent scene commands;
- dependency-free SVG output;
- GPUI desktop chart list using the `../gpui_kutuphanesi` title bar and buttons;
- interactive WASM chart list served on development port 8081;
- one shared Rust card-definition example shown in desktop and WASM UIs;
- source lock, API matrix, demo manifest, and scenario record;
- first card: a 100-point `sin(x)` line based on `demos/resize.html`.

The first card also ports the source demo's conditional hollow points, filled
hover marker, live legend, numeric grid aligned to the visible range, and
drag-to-zoom interaction on the X axis.

The GPUI chart list is not part of the distributed `uplot-rs` library. It is a
separate, unpublished verification application under `uygulamalar/masaustu`.
Selection, wheel zoom, touch zoom/pan, desktop pan, full-view reset, and view history are implemented in
the core. Library users only provide data, colors, and feature switches;
unspecified features retain their core defaults.

## Rendering-surface features

Cargo features are additive and may be enabled together:

```toml
uplot-rs = {
    version = "0.1.0",
    default-features = false,
    features = ["gpui", "svg"]
}
```

The Cargo package name is the lowercase `uplot-rs`; Rust exposes it in code as
`uplot_rs` because hyphens become underscores. The source repository is
[hakantr/uPlot.rs](https://github.com/hakantr/uPlot.rs).

- `svg`: the default dependency-free SVG output;
- `wasm`: enables `svg` support for the WASM surface;
- `gpui`: enables the ready-to-use `uplot_rs::gpui::GpuiGrafik` component.

The `gpui` feature follows the unpinned `../gpui/crates/gpui` worktree. A
feature enables compilation but does not import Rust names automatically:

```rust
use uplot_rs::gpui::GpuiGrafik;

let chart = Grafik::yeni(options, data)?;
let surface = cx.new(|_| GpuiGrafik::yeni(chart));
```

The GPUI catalog uses this component but is not included in the library package.

## Chart interactions

Optional upstream plugin behaviors are switched per chart:

```rust
let interactions = EtkileşimSeçenekleri::default()
    .tekerlek_etkileşimi(true)
    .dokunma_etkileşimi(true)
    .seçim_yakınlaştır(true);
```

`dokunma_etkileşimi(true)` enables the two-finger X/Y zoom and single-finger
pan ported from `demos/zoom-touch.html`. Once a desktop chart is zoomed,
Space + left drag pans automatically and requires no additional chart option.
Optional behaviors set to `false` are disabled; omitted settings keep their
`Default` values.

## Behavior that differs from upstream

Required port changes, API adaptations, and uPlot.rs-specific extensions live
in a separate inventory to keep this README compact. See
[Differences from the official uPlot repository](RESMI_DEPO_FARKLILIKLARI.md#differences-from-the-official-uplot-repository)
for details and provenance.

## Live demo and automated builds

The interactive WASM chart list is published with GitHub Pages:

**[Open the live uPlot.rs WASM demo](https://hakantr.github.io/uPlot.rs/)**

Every day at 18:00 UTC (21:00 in Türkiye), the WASM package is rebuilt and
deployed to Pages together with the following downloadable workflow artifacts:

- macOS ARM64;
- Linux ARM64;
- Linux x86_64;
- Windows x86_64;
- WASM web package.

See the
[nightly-artifacts workflow](https://github.com/hakantr/uPlot.rs/actions/workflows/nightly-builds.yml)
for scheduled builds and manual runs.

When a new nightly run starts, any queued or older in-progress run is
cancelled. Only the latest nightly run's artifacts and the latest two Pages
deployment records are retained; GitHub Release versions are not affected by
this cleanup.

## Application icon

<img src="assets/app-icon.svg" width="128" alt="uPlot.rs application icon">

One SVG source produces the web favicon, Linux PNG desktop icon, macOS ICNS
application/dock icon, and Windows ICO/EXE icon. Nightly artifacts contain a
macOS `.app` bundle, a portable Linux directory with a `.desktop` entry, and a
Windows `uplot-rs.exe` with the icon embedded.

## Running locally

```sh
cargo test
cargo run --example ilk_kart
cargo run -p uplot-rs-chart-listesi
npm --prefix tools/uyum run denetle
```

## Error handling

Production Rust code forbids `panic!`, `unwrap`, `expect`, unchecked slice
indexing, `todo!`, `unimplemented!`, and `unreachable!`. Validation failures
are returned to callers as typed `UplotHatası` values; the desktop and WASM
verification UIs show errors on the chart card. Workspace
lints and the CI Clippy step enforce this policy on every change.

The first command runs the tests, the second generates `target/ilk-kart.svg`,
and the third opens the live GPUI chart list. The final command verifies the
commit, version, and file hashes in a local checkout of the
[uPlot source repository](https://github.com/leeoniya/uPlot), cloned as `uPlot`
beside this repository. See
[wasm/README.md](wasm/README.md) for browser instructions.

## Source layout

- `src/veri.rs`: uPlot-compatible aligned column data contract
- `src/olcek.rs`: scale and range mathematics
- `src/cizim.rs`: surface-independent scene commands and SVG output
- `src/grafik.rs`: initial rendering pipeline
- `src/etkilesim.rs`: chart interaction state, zooming, and view history
- `src/gpui.rs`: ready GPUI chart component behind the `gpui` feature
- `src/svg.rs`: SVG surface used by the `svg` and `wasm` features
- `src/kart.rs`: verifiable card fixtures
- `uygulamalar/masaustu/`: GPUI verification app excluded from distribution
- `uyum/`: machine-readable source and evidence inventory
- `tools/uyum/`: reproducibility and verification tooling
- `RESMI_DEPO_FARKLILIKLARI.md`: direct-port versus uPlot.rs-extension inventory

See [UPLOT_TAM_UYUM_FAZ_PLANI.md](UPLOT_TAM_UYUM_FAZ_PLANI.md) for the detailed
roadmap.

## Attribution and thanks

The original chart-engine design, performance approach, API ideas, algorithms,
default behaviors, and demo scenarios belong to the
[uPlot repository](https://github.com/leeoniya/uPlot). The Rust code in this
repository adapts that work to different runtimes and user interfaces, tests
its equivalence, and documents the port.

Our sincere thanks go to uPlot creator Leon Sorokin and everyone who has
contributed code, bug reports, reviews, documentation, and feedback to the
upstream project. The functionality and correctness achieved by uPlot.rs are
possible because they shared their work as open source.

## License

This repository is licensed under Apache-2.0. The normative uPlot source is
licensed under MIT; its original copyright and license notice are retained in
[NOTICE](NOTICE).
