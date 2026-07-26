# GPUI geçişi son doğrulama kaydı

**Tarih:** 2026-07-26

**Normatif uPlot kaynağı:** `0e5812c504430f5c804e0f993376d8999b26cc34`

**Doğrulanan uygulama tabanı:** `e48cda4`

**Kilitli GPUI:** `1b8f324169f7cdf81d3567650fb6704327d4d2f4`

**Kilitli ortak bileşenler:** `c97926c3c15ab99091b51f7ed88bd98f357de409`

Bu kayıt `GPUI_GECIS_FAZ_PLANI.md` içindeki on üç uygulama fazının
tamamlanma kanıtıdır. uPlot.rs'in tek interaktif renderer'ı GPUI'dir. Native
ve web uygulamaları aynı `GpuiGrafik` bileşenini, aynı Rust kart kayıt
defterini ve aynı etkileşim durum makinelerini kullanır.

## Son mimari

- Root crate GPUI-first'tür; `GpuiGrafik` ve `GpuiGrafikOlayı` üst düzeyde
  sunulur.
- Native giriş `uygulamalar/masaustu`, web girişi `uygulamalar/web`,
  paylaşılan uygulama `uygulamalar/katalog` altındadır.
- Web çizimi `gpui_web → gpui_wgpu → HTMLCanvasElement/WebGPU` hattındadır.
  Seri veya path başına DOM/SVG düğümü ve davranışları kopyalayan JavaScript
  registry yoktur.
- Ana grafik ve cursor/seçim/hover ayrı retained GPUI yüzeyleridir. İkisinin
  kalıcı revizyonu ve GPUI yol önbelleği vardır.
- Komut geometrisi oluşturulurken bir kez kimliklendirilir. Sahne geçişinde
  büyük `Vec` içerikleri yeniden karşılaştırılmaz; değişmeyen yollar ve
  fiziksel DPI path'leri paylaşılır. Renk değişimi tessellation'ı bozmaz.
- `gpui-svg` ikinci bir interaktif renderer değildir. Serializer yalnız
  `GpuiGrafik::svg_kaydı` veya `svg_dosyasına_yaz` açıkça çağrıldığında
  retained yüzeyi gerçek vektör öğelerine kaydeder.
- Eski `examples/`, `wasm/`, `src/svg.rs`, kart başına SVG CLI ve geçici port
  notları arşiv dalında kalmıştır; aktif `main` hattında yoktur.

## Faz ve commit kanıtları

| Faz | Sonuç | Birincil kanıt |
|---:|---|---|
| 0 | Başlangıç arşivi ve envanter | `codex/pre-gpui-primary-archive` → `5c60cb5` |
| 1 | Stable GPUI Web, touch/pinch ve platform önkoşulları | `c7b8d85`, `17a6bb1` |
| 2 | GPUI-first root API ve diagnostics sınırı | `dcc9dae`, `567befa`, `f6d9545` |
| 3 | İsteğe bağlı gerçek GPUI → SVG vektör kaydı | `cc8631e`, `86a2ec6`, `f6d9545` |
| 4 | Retained ana/etkileşim katmanları ve DPI path cache | `caa5438`, `7499cca`, `f6d9545`; GPUI `1b8f324` |
| 5 | Tek literal Rust kart kayıt defteri | `f0ee0a0`, `e48cda4` |
| 6 | Native/web ortak GPUI katalog uygulaması | `b6206d0` |
| 7 | Native uygulama ve GPU backend/fallback tanısı | `3e95b90`, `7ae95ba` |
| 8 | GPUI Web/WASM, deep-link ve Blob SVG indirme | `7a133d2`, `66b7773` |
| 9 | Ortak wheel/drag/touch/sync etkileşimleri | `b52dc0c`, `8baed24` |
| 10 | Eski runtime, örnek ve not temizliği | `a31df4a` |
| 11 | Kilitli, çok platformlu CI ve yayın hattı | `1502da3`, `7ae95ba` |
| 12 | Release performans ve son kabul kapıları | `8cdf785`, `6036249` |

## Katalog ve davranış kabulü

- Normatif uyum envanteri: 73 kaynak demo/senaryo, 203 API satırı ve 19 ortak
  davranış.
- Ortak UI kayıt defteri: 66 ana kart ve sayfa içi ilişkili/varyant yüzeyler.
- Multi Bars tek ana karttır; dört varyantı tek fabrika üretir. Eski dört
  varyant slug'ı doğru sayfa içi seçimi açmaya devam eder.
- Aynı kaynak sayfasının ilişkili grafikleri aynı sayfada gösterilir.
  Sync Cursor, Line Paths, Latency Heatmap ve benzeri grupların bireysel ve
  ortak etkileşim sözleşmeleri testlidir.
- Wheel varsayılan olarak XY, Shift yalnız X, Ctrl yalnız Y eksenini
  yakınlaştırır. Seçim, pan, geçmiş, yüzey dışı mouse-up ve touch/pinch aynı
  çekirdeğe gider.
- Açıklama ve kod panelleri GPUI bileşenidir ve varsayılan kapalıdır.

## SVG kabulü

- Resize, Area Fill, Multi Bars, Scatter, Timezones, döndürülmüş Y etiketi,
  Cursor Snap, retina/aspect-fit ve deterministik kayıt testleri geçer.
- Çıktı raster `<image>` içermez; path, rect, circle, text, gradient ve clip
  düzenlenebilir vektör olarak kalır.
- Kayıt grafik state'ini ve ana sahne revizyonunu değiştirmez.
- Serializer sayacı gerçek `Sahne::svg_içeriği` girişindedir.
- Feature açıkken 1.000 sıcak retained boya hazırlığında serializer çağrısı
  `0`, tahsis sayısı `0`, toplam tahsis `0 bayt` ölçülür.
- Tarayıcı kabulünde `SVG'yi indir` düğmesi
  `uplot-rs-gpui-svg-export.svg` dosyasını üretmiştir: 800×400, 2.686 bayt,
  gerçek `<path>`, `<text>` ve `<rect>` içerir.

## Release performans kabulü

Performans kapısı optimize derlemede tek iş parçacığıyla p50/p95/p99, ilk
çizim, zoom, komut/geometri sayısı, tahsis ve Linux `VmRSS` sınırlarını
denetler. Bu ölçümler çekirdek sahne üretimi ve retained hazırlık maliyetidir;
GPU sürücü süresi katalogdaki 180 karelik gerçek `on_next_frame` tanısı ile
ayrıca görünür kılınır.

2026-07-26 yerel aarch64 macOS örnek sonucu:

| Senaryo | İlk çizim | Yeniden çizim p95 | Zoom p95 |
|---|---:|---:|---:|
| Multi Bars | 0,06 ms | 0,03 ms | 0,03 ms |
| Latency Heatmap ~35K | 4,10 ms | 3,93 ms | 4,07 ms |
| Latency Heatmap ~20K | 0,47 ms | 0,51 ms | 0,45 ms |
| Mass Spectrum 41.986 | 0,86 ms | 1,08 ms | 1,12 ms |
| Sparse 13.608 | 0,16 ms | 0,28 ms | 0,15 ms |
| Sync Cursor CPU | 0,14 ms | 0,14 ms | 0,36 ms |

Ek sonuçlar:

- Resize 100/1.000 ve Sine 6×600 `setData + scene` p95 süreleri kare
  bütçesinin çok altındadır.
- Resize 1.000 SVG snapshot'ı yaklaşık 0,24 ms ve 19.947 bayttır.
- 1.000 sıcak retained boya hazırlığında GPUI path tahsisi `0`dır.
- Linux'ta aynı kapı `/proc/self/status` üzerinden mutlak ve büyüme `VmRSS`
  sınırını uygular.

## Son doğrulama komutları

```sh
rustc --version
cargo fmt --all --check
cargo check -p uplot-rs --lib --no-default-features
cargo check -p uplot-rs --lib --no-default-features --features gpui-svg
cargo check -p uplot-rs --lib --all-features
cargo check -p uplot-rs-gpui-katalog
cargo check -p uplot-rs-gpui-web --target wasm32-unknown-unknown
cargo check --target wasm32-unknown-unknown --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
RUSTFLAGS='--cfg phase12_allocator --cap-lints allow' \
  cargo test --release -p uplot-rs --test performance_budgets \
  --all-features -- --test-threads=1 --nocapture
npm --prefix tools/uyum run denetle
(cd uygulamalar/web && NO_COLOR=false trunk build --release --public-url /uPlot.rs/)
```

CI her push'ta Linux test/Clippy/performance/WASM matrisini ve macOS ARM64 ile
Windows x64 native kontrollerini çalıştırır. Nightly yayın aynı kilitli kardeş
commitlerden macOS, Linux, Windows ve GPUI Web artefaktları üretir.

## Sürekli koruma

`tools/uyum/denetle.mjs` eski runtime, bağımsız SVG renderer veya ikinci
katalog tekrar eklenirse CI'yı başarısız yapar. `main` ile `origin/main`
eşliği ve temiz çalışma ağacı son kabulün parçasıdır.
