# GPUI geçişi son doğrulama kaydı

**Tarih:** 2026-07-26

**Normatif uPlot kaynağı:** `0e5812c504430f5c804e0f993376d8999b26cc34`

**Doğrulanan uygulama tabanı:** `62151d4`

**Kilitli GPUI:** `18b9b8b4d6f5f8a6397e273f5b0fdbac51581192`

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
- GPUI Web adaptör veya aygıt oluşturma hatasını kaybetmez; renderer seçimi ve
  donanım tanısı `gpui_platform`/`gpui_web` katmanına aittir.
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
- 240 izlenen dosya/129.517 satır yerel `../gpui/yetenek.md` ve `../gpui`
  kaynaklarına göre sorumluluk denetiminden geçirilmiştir. Ayrıntılı sonuç
  `GPUI_YETENEK_DEVIR_DENETIMI.md` içindedir.

## Faz ve commit kanıtları

| Faz | Sonuç | Birincil kanıt |
|---:|---|---|
| 0 | Başlangıç arşivi ve envanter | `codex/pre-gpui-primary-archive` → `5c60cb5` |
| 1 | Stable GPUI Web, touch/pinch ve platform önkoşulları | `c7b8d85`, `17a6bb1` |
| 2 | GPUI-first root API ve diagnostics sınırı | `dcc9dae`, `567befa`, `f6d9545`, `87bd9e3` |
| 3 | İsteğe bağlı gerçek GPUI → SVG vektör kaydı | `cc8631e`, `86a2ec6`, `f6d9545` |
| 4 | Retained ana/etkileşim katmanları ve DPI path cache | `caa5438`, `7499cca`, `f6d9545`, `87bd9e3`; GPUI `18b9b8b` |
| 5 | Tek literal Rust kart kayıt defteri | `f0ee0a0`, `e48cda4` |
| 6 | Native/web ortak GPUI katalog uygulaması | `b6206d0` |
| 7 | Native uygulama ve GPU backend/fallback tanısı | `3e95b90`, `7ae95ba`, `af7fc90`; GPUI `18b9b8b` |
| 8 | GPUI Web/WASM, deep-link ve Blob SVG indirme | `7a133d2`, `66b7773` |
| 9 | Ortak wheel/drag/touch/sync etkileşimleri | `b52dc0c`, `8baed24` |
| 10 | Eski runtime, örnek ve not temizliği | `a31df4a` |
| 11 | Kilitli, çok platformlu CI ve yayın hattı | `1502da3`, `7ae95ba` |
| 12 | Release performans ve son kabul kapıları | `8cdf785`, `6036249` |
| 13 | Yerel GPUI yetenek devri ve ikinci runtime temizliği | `13071b3`…`62151d4`; GPUI `18b9b8b` |

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
- Retained etkileşim tuvali ana yüzeyin `(0, 0)` köşesine sabitlenir ve
  ertelenmiş üst katmanda boyanır. Tarayıcı kabulünde Resize 100 üzerinde
  düşey/yatay crosshair ile imleç noktası görünür; ana sahne revizyonu
  değişmez.
- Multi Bars tarayıcı kabulünde metrikler kendi ölçeklerinde çizilir, değer
  etiketleri bar sınırlarında kalır ve hover yalnız vurulan barın rengini
  değiştirir; seri başına renkli cursor noktaları üretilmez.
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
| Multi Bars | 0,07 ms | 0,21 ms | 0,05 ms |
| Latency Heatmap ~35K | 3,91 ms | 4,10 ms | 4,35 ms |
| Latency Heatmap ~20K | 0,47 ms | 0,43 ms | 0,55 ms |
| Mass Spectrum 41.986 | 0,97 ms | 1,29 ms | 1,31 ms |
| Sparse 13.608 | 0,15 ms | 0,14 ms | 0,26 ms |
| Sync Cursor CPU | 0,16 ms | 0,16 ms | 0,13 ms |

Ek sonuçlar:

- Resize 100/1.000 ve Sine 6×600 `setData + scene` p95 süreleri kare
  bütçesinin çok altındadır.
- Resize 1.000 SVG snapshot'ı yaklaşık 0,20 ms ve 19.947 bayttır.
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
cargo check -p uplot-rs-gpui-ornekler --all-features
cargo check -p uplot-rs-gpui-katalog
cargo check -p uplot-rs-gpui-web --target wasm32-unknown-unknown
cargo check --target wasm32-unknown-unknown --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
RUSTFLAGS='--cfg phase12_allocator --cap-lints allow' \
  cargo test --release -p uplot-rs-gpui-ornekler --test performance_budgets \
  --all-features -- --test-threads=1 --nocapture
npm --prefix tools/uyum run denetle
(cd uygulamalar/web && NO_COLOR=false trunk build --release --public-url /uPlot.rs/)
```

Son release web çıktısı 83.473 bayt JavaScript ve 11.214.029 bayt WASM
üretmiştir.

CI her push'ta Linux test/Clippy/performance/WASM matrisini ve macOS ARM64 ile
Windows x64 native kontrollerini çalıştırır. Nightly yayın aynı kilitli kardeş
commitlerden macOS, Linux, Windows ve GPUI Web artefaktları üretir.

## Karar: gpui'de fork yok, upstream yol gönderimi kabul edildi

**Tarih:** 2026-07-28

Bir dönem `../gpui`, uPlot.rs için iki ek taşıdı: `Path.vertices`'in
`Arc<Vec<_>>` olması ve cihaz ölçeğine çevrilmiş yolu paylaşımlı gönderen
`paint_scaled_path` / `paint_transformed_scaled_path`. İkisi de Zed'de hiç
var olmadı ve gpui'nin kendi parite politikasına (`AGENTS.md`) aykırıydı;
2026-07-28 Zed senkronu bu yüzden onları kaldırdı.

Ekleme geri getirilmedi. Gerekçe ölçüldü, tahmin edilmedi:

| ölçüm | fork'lu | upstream | not |
|---|---:|---:|---|
| Resize kök render p50 | 578 µs | 699 µs | ihmal edilebilir |
| ThinBars (55 yüzey) | 897 µs | 917 µs | ihmal edilebilir |
| TimezonesDst (51 yüzey) | 923 µs | 978 µs | ihmal edilebilir |
| MassSpectrum | 564 µs | 621 µs | ihmal edilebilir |
| **LatencyHeatmap** | **684 µs** | **4,93 ms** | tek anlamlı fark |

LatencyHeatmap yüzey başına ~49K köşe taşıyor; maliyet köşe hacmiyle
süperdoğrusal büyüyor. Kalıcı bütçe testinin p50 sınırı 8,35 ms olduğu için
kart hâlâ bütçenin içinde.

Başsız ölçüm ile canlı davranış ayrışıyor ve bunu bilerek kabul ettik:
bütçe testi her turda `cx.notify()` ile tam yeniden boyama zorluyor, canlı
uygulamada ise veri yüzeyi `cached()` altında olduğundan çoğu karede
`sahneyi_önbellekli_boya` hiç çalışmıyor. Aynı kartın canlı etkileşim
CPU'su %21–27 ölçüldü ve gezinme akıcı bulundu.

Tessellation önbelleği (asıl kazanç) korunuyor; kaldırılan yalnız
ölçeklenmiş kopyanın kareler arası paylaşımıydı.
`uygulamalar/katalog/tests/upstream_yol_butcesi.rs` kabul edilen köşe başına
maliyeti bütçeye bağlar ve sessiz büyümeyi engeller. Zed ileride eşdeğer bir
API eklerse normal senkronla gelir; buraya yeniden eklenmez.

## Sürekli koruma

`tools/uyum/denetle.mjs` eski runtime, bağımsız SVG renderer veya ikinci
katalog tekrar eklenirse CI'yı başarısız yapar. `main` ile `origin/main`
eşliği ve temiz çalışma ağacı son kabulün parçasıdır.

`tools/paket_siniri_denetle.sh` kart listesinin ve örnek kabukların
kütüphane paketine sızmadığını doğrular: paket içeriği, uygulama
crate'lerinin `publish = false` durumu ve kütüphanenin bağımlılık yönü. Sınır
hedeften değil paket içeriğinden geldiği için native ve wasm aynı denetimle
kapsanır.
