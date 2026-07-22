# uPlot.rs

**English** · [Türkçe](README.md)

This project is a port of [uPlot](https://github.com/leeoniya/uPlot) 1.6.32's
small, fast, and memory-efficient charting approach to Rust, GPUI, and WASM.
It is not an independently invented charting engine. The normative source is
commit `0e5812c504430f5c804e0f993376d8999b26cc34` of the sibling `../uPlot`
repository; uPlot defines the behavioral, API, and visual compatibility target.

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
hover marker, live legend, and drag-to-zoom interaction on the X axis.

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

## Running locally

```sh
cargo test
cargo run --example ilk_kart
cargo run --example chart_listesi
npm --prefix tools/uyum run denetle
```

The first command runs the tests, the second generates `target/ilk-kart.svg`,
and the third opens the live GPUI chart list. The final command verifies the
locked sibling `../uPlot` commit, version, and file hashes. See
[wasm/README.md](wasm/README.md) for browser instructions.

## Source layout

- `src/veri.rs`: uPlot-compatible aligned column data contract
- `src/olcek.rs`: scale and range mathematics
- `src/cizim.rs`: surface-independent scene commands and SVG output
- `src/grafik.rs`: initial rendering pipeline
- `src/kart.rs`: verifiable card fixtures
- `uyum/`: machine-readable source and evidence inventory
- `tools/uyum/`: reproducibility and verification tooling

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
