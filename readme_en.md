# uPlot.rs

**English** · [Türkçe](README.md)

This project ports [uPlot](https://github.com/leeoniya/uPlot) 1.6.32's small,
fast, and memory-efficient charting approach directly to GPUI in Rust.
It is not an independently invented charting engine. The normative source is
[commit `0e5812c` in the uPlot repository](https://github.com/leeoniya/uPlot/commit/0e5812c504430f5c804e0f993376d8999b26cc34);
uPlot defines the behavioral, API, and visual compatibility target.

The codebase uses Rust 2024 edition and requires Rust 1.95 or newer. New
modules use `foo.rs` and, when needed, `foo/submodule.rs` instead of `mod.rs`.

`gpui` and `gpui_kutuphanesi` are intentionally not pinned to commits. Local
builds use the current sibling worktrees through path dependencies, while CI
uses the current default branches of both repositories. Only the normative
uPlot source is commit-locked.

GPUI is the library's only interactive renderer. Native applications and the
web target run the same `GpuiGrafik` component; the web surface uses
`gpui_web` and WebGPU. SVG is not a second runtime renderer. It is an optional
vector export generated from the retained GPUI chart surface on demand. The
GitHub Pages catalogue runs that same GPUI Web renderer. When a WebGPU adapter
is unavailable, GPUI owns renderer selection and diagnostics; uPlot.rs does
not silently switch to a separate SVG/DOM runtime.

The shared foundation contains:

- validated aligned/columnar data model;
- numeric X scales and fixed/automatic Y ranges;
- retained drawing commands consumed by GPUI paint and optional SVG export;
- GPUI desktop chart list using the `../gpui_kutuphanesi` title bar and buttons;
- a GPUI Web application drawing to a WebGPU canvas through `gpui_web`;
- every card produced from the same Rust catalogue source on native and GPUI Web;
- hash-locked inventories for 18 source files, 304 public API members, 28 data
  assets, and all 73 demos;
- `Resize` card: a 100-point `sin(x)` line based on `demos/resize.html`.
- `Area Fill` card: the source 1…30 X values, −10…10 value pool, three sampled
  series, zero-baseline fills, and multi-series cursor/legend behavior.

The `Resize` card also ports the source demo's conditional hollow points, filled
hover marker, live legend, numeric grid aligned to the visible range, and
drag-to-zoom interaction on the X axis.

The GPUI chart list is not part of the distributed `uplot-rs` library. A single
unpublished Rust entity under `uygulamalar/katalog` is consumed by both the
`uygulamalar/masaustu` and `uygulamalar/web` entry points.
Selection, wheel zoom, touch zoom/pan, desktop pan, full-view reset, and view history are implemented in
the core. Library users only provide data, colors, and feature switches;
unspecified features retain their core defaults.

## Usage

GPUI is a required dependency; there is no separate `gpui` feature:

```toml
uplot-rs = { version = "0.1.0" }
```

This registry declaration becomes directly usable after version `0.1.0` is
published on crates.io; the source repository is also linked below.

The Cargo package name is the lowercase `uplot-rs`; Rust exposes it in code as
`uplot_rs` because hyphens become underscores. The source repository is
[hakantr/uPlot.rs](https://github.com/hakantr/uPlot.rs).

The ready component remains in an explicit GPUI namespace:

```rust
use uplot_rs::{Grafik, gpui::{GpuiGrafik, başlat}};

başlat(cx); // once during GPUI application setup
let chart = Grafik::yeni(options, data)?;
let surface = cx.new(|_| GpuiGrafik::yeni(chart));
```

`başlat` registers chart keys through GPUI's Action/KeyBinding system; the
application can remap those actions in its own GPUI keymap layer.

The GPUI catalog uses this component but is not included in the library package.
The retained scene model does not disable GPUI GPU acceleration: commands are
submitted through GPUI's GPU-backed `paint_path`/`paint_quad` pipeline.
Zoom and pan do not rebuild the full-scale data layer: GPUI applies a late GPU
transformation matrix to the same shared path vertices and clips the result to
the plot mask. Only the small axis/grid/label layer is regenerated for the
visible range. `setData`, `setSeries`, and resize still invalidate retained
geometry because they actually change it.
Use `GpuiGrafikGrubu` when multiple surfaces must behave as one. It propagates
cursor, wheel, selection, pan, axis zoom, full-view reset, and series
visibility using normalized surface ratios rather than raw pixels or data
values, so members may have different dimensions, ranges, and scales.
The retained command list is not a general-purpose second renderer backend.
Normal integrations use `Grafik` + `gpui::GpuiGrafik`; inspection for tests,
profiling, and custom verification tools is separated under
`diagnostics::{Komut, Sahne}`.

### Optional GPUI → SVG export

Enable only the export API:

```toml
uplot-rs = { version = "0.1.0", features = ["gpui-svg"] }
```

```rust
use uplot_rs::GpuiSvgKayıtAyarları;

let settings = GpuiSvgKayıtAyarları::yeni(1_200, 600)?;
let svg = surface.read(cx).svg_kaydı(settings);
std::fs::write("chart.svg", svg.byte_değeri())?;
```

The serializer never runs in the normal GPUI frame/paint path. It reads the
existing retained scene only when `svg_kaydı` is called, does not rebuild
geometry or mutate chart state, and emits editable vectors rather than a
raster `<image>`.

## Chart interactions

Optional upstream plugin behaviors are switched per chart:

```rust
let interactions = EtkileşimSeçenekleri::default()
    .tekerlek_etkileşimi(true)
    .dokunma_etkileşimi(true)
    .seçim_yakınlaştır(true);
```

`dokunma_etkileşimi(true)` enables the two-finger X/Y zoom and single-finger
pan ported from `demos/zoom-touch.html`. Wheel zoom requires the chart to be
focused by click or keyboard by default, allowing an overflowing dashboard or
catalog page to scroll first. Applications can explicitly opt into hover-only
wheel zoom with `.tekerlek_odaksız_etkileşim(true)`. Once a desktop chart is zoomed,
Space + left drag pans automatically and requires no additional chart option.
Optional behaviors set to `false` are disabled; omitted settings keep their
`Default` values.

## Behavior that differs from upstream

Required port changes, API adaptations, and uPlot.rs-specific extensions live
in a separate inventory to keep this README compact. See
[Differences from the official uPlot repository](RESMI_DEPO_FARKLILIKLARI.md#differences-from-the-official-uplot-repository)
for details and provenance.

## Live demo and automated builds

The interactive GPUI Web chart list is published with GitHub Pages:

**[Open the live uPlot.rs GPUI Web demo](https://hakantr.github.io/uPlot.rs/)**

Every day at 18:00 UTC (21:00 in Türkiye), the GPUI Web package is rebuilt and
deployed to Pages. The current GPUI Web layer owns renderer and platform
selection; uPlot.rs does not run a second browser runtime. The workflow also
produces these downloadable artifacts:

- macOS ARM64;
- Linux ARM64;
- Linux x86_64;
- Windows x86_64;
- GPUI Web/WebGPU package.

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

Keep the repositories under the same parent directory. `uPlot.rs` consumes the
current local GPUI capabilities directly from the sibling `gpui` and
`gpui_kutuphanesi` worktrees:

```sh
git clone https://github.com/hakantr/gpui.git
git clone https://github.com/hakantr/gpui_kutuphanesi.git
git clone https://github.com/hakantr/uPlot.rs.git
cd uPlot.rs
```

```sh
cargo test
cargo run -p uplot-rs-chart-listesi
cd uygulamalar/web && NO_COLOR=false trunk serve
npm --prefix tools/uyum run envanter
npm --prefix tools/uyum run denetle
```

If a Linux browser cannot create a WebGPU adapter, the Pages catalogue does not
switch to another renderer. Verify the GPU/Vulkan chain with
`vulkaninfo --summary` and `glxinfo -B`, check browser hardware acceleration,
or run the native GPUI app with
`cargo run -p uplot-rs-chart-listesi --release`.

## Error handling

Production Rust code forbids `panic!`, `unwrap`, `expect`, unchecked slice
indexing, `todo!`, `unimplemented!`, and `unreachable!`. Validation failures
are returned to callers as typed `UplotHatası` values; the GPUI desktop and web
verification UIs show errors on the chart card. Workspace
lints and the CI Clippy step enforce this policy on every change.

The first command runs the tests. The desktop command opens the live GPUI
chart list, and Trunk opens the GPUI Web/WGPU application. The inventory
command regenerates the source/API/demo
inventories, and the verification command checks the commit, version, and file
hashes in a local checkout of the
[uPlot source repository](https://github.com/leeoniya/uPlot), cloned as `uPlot`
beside this repository. The browser catalogue opens the same shared GPUI entity
through `uygulamalar/web/Trunk.toml`.

See the [detailed GPUI transition plan](GPUI_GECIS_FAZ_PLANI.md) and the
[final GPUI transition verification record](GPUI_GECIS_DOGRULAMA.md) for the
phase commits, test matrix, and release evidence.

## Source layout

- `src/veri.rs`: uPlot-compatible aligned column data contract
- `src/olcek.rs`: scale and range mathematics
- `src/cizim.rs` + `src/cizim/`: crate-internal retained GPUI drawing commands
  and clipping; only the diagnostics view is exposed under `diagnostics`
- `src/grafik.rs`: initial rendering pipeline
- `src/etkilesim.rs`: chart interaction state, zooming, and view history
- `src/gpui.rs`: ready GPUI chart component included in every normal build
- `src/gpui/svg_kaydi.rs`: vector export compiled only with `gpui-svg`
- `src/secenek.rs` + `src/secenek/`: grouped option types
- `uygulamalar/ornekler/`: unpublished card configurations and source-data
  fixtures; they are not dependencies of `uplot-rs` consumers
- `uygulamalar/katalog/`: the single GPUI card registry, related-surface groups,
  and explanation UI shared by native and web
- `uygulamalar/masaustu/`: native GPUI entry opening the shared catalogue
- `uygulamalar/web/`: `gpui_web`/WebGPU entry opening the shared catalogue
- `uyum/`: machine-readable source and evidence inventory
- `tools/uyum/`: reproducibility and verification tooling
- `RESMI_DEPO_FARKLILIKLARI.md`: direct-port versus uPlot.rs-extension inventory
- `ORTAK_KART_DAVRANISLARI.md`: shared visual/interaction contract enforced for
  every newly ported card by CI

See the [complete GPUI migration phase plan](GPUI_GECIS_FAZ_PLANI.md) for the
detailed roadmap.
The file-by-file ownership audit against the local GPUI worktree is recorded in
the [GPUI capability delegation audit](GPUI_YETENEK_DEVIR_DENETIMI.md).

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
