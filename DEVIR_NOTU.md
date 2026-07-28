# Devir notu

**Tarih:** 2026-07-29

Bu not, oturumun bittiği noktadan başka bir ortamda (özellikle macOS)
devam edebilmek içindir. Kalıcı mimari kararlar `GPUI_GECIS_DOGRULAMA.md`
içindedir; burada yalnız devir için gereken durum, açık konular ve
tekrar üretim adımları var.

## Depo durumu

| depo | dal | son commit | durum |
|---|---|---|---|
| `uPlot.rs` | `main` | `b6ca42a` + devir notu | temiz, gönderildi |
| `../gpui` | `main` | `d3a6038` | temiz, gönderildi |
| `../gpui_kutuphanesi` | `main` | `36f1174` | temiz, gönderildi |

Yan yana beklenen dizinler: `uPlot.rs`, `gpui`, `gpui_kutuphanesi`,
`uPlot` (normatif kaynak, `master`), `zed` (parite doğrulaması için
`7b030b5008`).

## Bu oturumda yapılanlar

**gpui katı pariteye döndü.** `paint_scaled_path` /
`paint_transformed_scaled_path` ve `Path.vertices`'in `Arc<Vec<_>>` hâli
geri alındı; kaynak Zed `7b030b5008` ile birebir, yalnız
`EXTRACTION.md`'de belgelenmiş iki test uyarlaması farklı. Karar ölçümle
verildi ve `EXTRACTION.md`'ye "Evaluated and rejected" başlığıyla yazıldı.

**Raster katmanı eklendi** (`src/gpui/raster.rs`). Köşe bütçesini aşan ve
kayıpsız rasterleştirilebilen yüzeyler bir kez BGRA tamponuna çizilip
kare başına tek sprite gönderiyor — uPlot'un canvas modelinin GPUI
karşılığı. LatencyHeatmap kök render 4,93 ms → 666 µs. Politika kart
bazlı değil ölçüm bazlı; eşik `RASTER_NOKTA_EŞİĞİ = 8_000`.

**uPlot paritesi düzeltmeleri:** tekerlek zoomunda Y görünümü otomatik
bırakılıyor, tekerlek yakınlaştırması varsayılan kapalı (uPlot'ta
çekirdekte yok, demo eklentisi), yakınlaştırmada veri katmanı yeniden
kuruluyor, imleç katmanı ızgara sınırına kırpılıyor, kırpma dikdörtgenine
görsel pay eklendi.

**Erişilebilirlik:** kart listesi ikincil satırlarının kontrastı AA
eşiğinin üstüne çıkarıldı; paylaşılan bileşenlere açık tema bağlandı
(`ortak_bileşen_ayarları`), anahtar düğmesinin kontrastı
`gpui_kutuphanesi`'nde düzeltildi.

**Ölçüm altyapısı:** `izleme` feature'ı (varsayılan kapalı,
`UPLOT_IZLEME=1` ile çalışır), `upstream_yol_butcesi`,
`sahne_kompozisyonu` ve uPlot resmî benchmark senaryosu
(`uplot_bench_kartı`, 166.650 nokta).

## Açık konular

**1. Linux'ta web boş ekran veriyor — bu makineye özgü.**
Aynı kod macOS'ta çalışıyor ve kartlar görüntüleniyor. Bu makinede
(Linux, AMD Radeon 780M/RADV, Chrome 150) WebGPU bağlamı kuruluyor ama
ilk karede patlıyor:

```
CopyTextureForBrowser from [Texture (unlabeled 1x1 px, BGRA8Unorm)]
to [Invalid Texture]
```

Canvas doğru boyutta (1265×1398), kopyalanan yüzey dokusu 1×1. Chrome'da
`chrome://flags/#enable-vulkan` açılmadan adaptör hiç gelmiyordu; açınca
adaptör geliyor ama kare başarısız oluyor.

İlgili: senkron, `gpui_web/src/wgpu_context.rs`'ten fork-local **WebGL2
fallback**'ini kaldırdı. Eskiden WebGPU tökezlediğinde WebGL2'ye
düşülüyordu; artık yedek yok. Parite politikası gereği geri konmadı,
ama bu kurulumda değerinin somut olduğu görüldü. Çözüm önce Zed'de
aranmalı.

**2. Kart doğrulaması yapılmadı.** Asıl hedef 73 kartı wasm üzerinden
gezip hataları toplamaktı; yukarıdaki engel yüzünden bu makinede
yapılamadı. macOS'ta yapılmalı.

**3. LAN üzerinden erişim WebGPU vermez.** `http://192.168.0.35:8081`
güvenli bağlam olmadığından `navigator.gpu` hiç tanımlanmıyor
(`isSecureContext: false`). Uzaktan bakmak için SSH tüneli gerekir:

```sh
ssh -L 8081:localhost:8081 <kullanıcı>@<mac>
# sonra http://localhost:8081
```

Ama sayfa yine yerel tarayıcıda render edilir; 1 numaralı engel sürer.

**4. Başlık çubuğu sürüklemesi.** Bir ara çalışmadı, sonra düzeldi;
nedeni doğrulanmadı. Şüphe: `gpui_kutuphanesi/platform_pencere.rs`'e
yeni eklenen `BaslikSuruklemeDurumu` taşımayı basıştan sonraki **ilk
harekette** başlatıyor, Wayland ise `xdg_toplevel::move` için taze giriş
serial'i bekliyor. Tekrarlarsa oradan başlanmalı.

**5. `gpui_web` varsayılan `multithreaded`.** Nightly ve `build-std`
zorunluluğunu bu getiriyor, ama web kabuğumuz `single_threaded_web()`
çağırıyor. Gerçekten kullanılmıyorsa `default-features = false` nightly
bakımını tümüyle kaldırabilir. Şimdilik Zed yaklaşımı korunuyor.

**6. Görsel regresyon otomasyonu yok.** Raster sprite yerleşim hatasını
(`c847866`) hiçbir test yakalamadı; kare bütçesi süreyi ölçüyor,
yerleşimi değil.

## Çalıştırma

```sh
# Masaüstü
cargo run -p uplot-rs-chart-listesi --release

# İzleme günlüğü açık (kart değişimi, kaydırma, fare, kare özetleri)
UPLOT_IZLEME=1 cargo run -p uplot-rs-chart-listesi --release

# Web (kendi workspace'i; nightly + build-std, ilk derleme uzun)
cd uygulamalar/web && trunk serve --release

# Doğrulama
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
./tools/paket_siniri_denetle.sh
npm --prefix tools/uyum run denetle
cargo test --release -p uplot-rs-gpui-katalog --lib kok_render_kare_butcesi -- --nocapture
RUSTFLAGS='--cfg phase12_allocator --cap-lints allow' \
  cargo test --release -p uplot-rs-gpui-ornekler --test performance_budgets \
  --all-features -- --test-threads=1 --nocapture
```

## Son ölçümler (bu makine, uygulama kapalıyken)

| kart | kök render p50 |
|---|---:|
| MassSpectrum | 562 µs |
| LatencyHeatmap | 666 µs |
| Resize | 759 µs |
| TimezonesDst (51 yüzey) | 978 µs |
| ThinBars (55 yüzey) | 987 µs |

uPlot resmî benchmark senaryosu (166.650 nokta): ilk çizim 1,83 ms,
yeniden çizim p50 1,53 ms, 17.773 geometri öğesi, sıfır tahsis.
Karşılaştırmanın sınırları `GPUI_GECIS_DOGRULAMA.md` içinde yazılı.
