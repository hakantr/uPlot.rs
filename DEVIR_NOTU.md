# Devir notu

**Tarih:** 2026-07-29 (macOS oturumu)

Bu not, oturumun bittiği noktadan devam edebilmek içindir. Kalıcı mimari
kararlar `GPUI_GECIS_DOGRULAMA.md` içindedir; burada yalnız devir için
gereken durum, açık konular ve tekrar üretim adımları var.

## Depo durumu

| depo | dal | son commit | durum |
|---|---|---|---|
| `uPlot.rs` | `main` | `a74ea7b` | temiz, **gönderilmedi** |
| `../gpui` | `main` | `d3a6038` | temiz, gönderildi |
| `../gpui_kutuphanesi` | `main` | `36f1174` | temiz, gönderildi |

Yan yana beklenen dizinler: `uPlot.rs`, `gpui`, `gpui_kutuphanesi`,
`uPlot` (normatif kaynak, `master`), `zed` (parite doğrulaması için
`7b030b5008`; bu makinede `401a0c7e3d` duruyor).

## Önceki oturumun açık konuları — kapanış

**Linux'ta web boş ekran (eski #1): macOS'ta sorun yok.** WebGPU bağlamı
kuruluyor, katalog render ediliyor, konsol temiz. Boş ekran o makineye
(RADV) özgü.

**73 kart doğrulaması (eski #2): yapıldı.** Katalog 66 kart tanımı taşıyor
(`multi-bars` 4 varyantlı). Hepsi `?kart=<slug>` ile tek tek gezildi.

**LAN üzerinden WebGPU (eski #3):** değişmedi, güvenli bağlam gerekir.

**`gpui_web` varsayılan `multithreaded` (eski #5):** konsolda
`SharedArrayBuffer not available; falling back to single-threaded
dispatcher` uyarısı `crossOriginIsolated: true` iken bile düşüyor. Kabuk
zaten `single_threaded_web()` çağırdığından `default-features = false`
argümanı güçlendi.

## Bu oturumda yapılanlar

**Yüzey yerleşimi çekirdeğe alındı** (`src/gpui/yerlesim.rs`, commit
`9587523`). Uygulama yalnız resmî sayfadaki ham boyutu bildirir; yüzey
kendini görünür alana uyarlar. `otomatik_uyarla` açıkken en boy oranı
korunarak dikeyde sığdırılır, kapalıyken ham boyutta kalır. Yüzey asla
büyütülmez. Ölçek `EN_KÜÇÜK_ÖLÇEK = 0.5` altına inecekse sığdırma
uygulanmaz — 800×2300 yatay çubuk yüzeyi sığdırılınca eksen etiketleri
okunamıyordu. `uyarlanan_alan` `Styled` döndürür; başlık/açıklama blokları
ölçüm dışında bırakılabilir. 7 birim testi.

**Katalog buna bağlandı** (commit `eb8a375`). 38 kart `çizim_tabanı`'nı
`.flex_none().h(px(N))` ile eziyordu; kapsayıcı pencere yüksekliğinden
bağımsız kalıyor, dış hat ekranı kullanmıyordu. Sabit yükseklikler
kaldırıldı, sabit yüzey boyutu sıfıra indi. Ölçüm iki yoldan gelir:
kapanışta kurulabilen kartlar doğrudan `uyarlanan_alan`dan, `Context`
isteyen kartlar bir önceki karede ölçülen alandan (canvas ölçer, güncelleme
`cx.defer` ile ertelenir — çizim sırasında `Entity::update` etkisiz).
Sync Cursor panelleri grup olduğu için tek ölçekle birlikte sığdırılır.

**Dağılım yüzeyleri raster yoluna alındı** (commit `a74ea7b`). 40.000
noktalı scatter tamamen boştu: GPUI yol kurucusu tek yolda 4.000 daireyi
kurabiliyor, 8.000'i kuramıyor ve `Komut::Daireler` çizimindeki
`build().ok()` hatayı yutuyor. Raster katmanı yalnız `ArkaPlan`, `Alan` ve
dolu `Dikdörtgen` tanıdığından bu sahneler eşiği aştıkları hâlde
rasterlenebilir sayılmıyordu. Vuruşsuz daireler artık kapsamda. Kare
bütçesi etkilenmedi (LatencyHeatmap p50 374 µs).

**İmleç çizgisi fareyi izliyor.** Kesik çizgi `imleç.veri_x` üzerinden
konumlanıyor, yani en yakın örneğe yapışıyordu. Yapışma Ctrl'e alındı;
lejant ve odak değerleri her iki durumda da en yakın örnekten çözülür.

**Küçük görsel düzeltmeler.** Kart listesinde iki satıra saran başlıklarda
kaynak satırı kırpılıyordu (`uniform_list` tek yükseklik uygular) → satır
118 px. Web fontunun kapsamadığı semboller (`▾ ▸ ＋ ▶ □`) kapsadıklarıyla
değiştirildi (`− + → ●`). latency-heatmap X ekseni etiketleri geldi.

## Açık konular

**1. Sparkline ailesinde iki kart bozuk.** "Sparklines · 10×2 tablo"da 20
yüzeyin yalnız sonuncusu çiziliyor, diğerleri boş `ArkaPlan` gösteriyor;
"Sparkline + Floating Bars" tek parça yeşil şekil veriyor.

Ölçüm: **2 satır (4 yüzey) → hepsi çizilir. 5 satır (10 yüzey) → yalnız
sonuncusu.** Belirti yüzey sayısıyla ortaya çıkıyor.

Elenenler: çekirdek sahnesi doğru (testler 15/15, `ArkaPlan`/`Alan`
komutları doğrulanıyor); entity eşleşmesi doğru (`TÜMÜ` 20, `SATIRLAR` 10
çift); raster/vektör ayrımı değil (eşik 10'a düşürülüp denendi);
`Entity::cached` değil; yol önbelleği paylaşımı değil (her yüzey kendi
`Rc<RefCell<GpuiYolÖnbelleği>>`'ini alır); ilk kare boyut ölçümü değil
(hover/tıklama ile yeniden çizim tetiklendi).

Kalan şüphe GPUI'nin kare başına yol/atlas kapasitesi. Scatter'da komut
parçalama denendiğinde `wgpu_atlas.rs:79 index out of bounds` alınmıştı;
burada hata verilmeden sessizce çizilmiyor. Sonraki adım `gpui_wgpu` atlas
ayırma yolunu okuyup kapasite aşımının nasıl raporlandığına bakmak. gpui
katı Zed paritesinde tutulduğundan çözüm katalog/çekirdek tarafında
aranmalı.

**2. Zoom sırasında kavisli sütun uçları yanlış kırpılıyor.** Sütun çizim
alanı sınırının dışına taştığında kavisli ucun kırpılması gerekirken kavis
yeniden çizilip sınıra yapışıyor.

**3. Sütun değer etiketleri zoom'da ölçeklenmiyor.** Sütun büyürken
ucundaki değer yazısı aynı boyutta kalıyor.

**4. Zoom sırasında hover vurgusu eski konumda donuyor.** Fare hareket
edene kadar yeni boyuta göre yeniden konumlanmıyor.

**5. Legend uPlot davranışından uzak.** Resmî sayfa değerleri daha okunaklı
gösteriyor ve değere tıklayınca seriyi gizleyip geri getiriyor. Ayrıca
legend tanım sırasına göre alt/sol/üst/sağ konumlandırılabilmeli ve
bulunduğu alana uygun listelenmeli.

**6. Y ekseni etiketleri yer yer çakışıyor** (mass-spectrum, custom-scales
log yüzeyi). Y bölmeleri `eksen_bölmeleri(aralık, çizim_y, 30.0)` ile
üretiliyor; log ölçekli eksende seyreltme yetersiz kalıyor.

**7. Görsel regresyon otomasyonu hâlâ yok.** Bu oturumdaki kusurların
hiçbirini test yakalamadı; hepsi tarayıcıda gözle bulundu.

**8. Tarayıcı önbelleği doğrulamayı yanıltıyor.** Chrome saatlerce eski
wasm'ı servis etti ve düzeltmeler "etkisiz" göründü. Doğrulama yaparken
URL'ye sürüm parametresi ekleyin (`?kart=...&v=N`) ve yüklenen dosya
adını `dist/` içindekiyle karşılaştırın.

## Çalıştırma

```sh
# Masaüstü
cargo run -p uplot-rs-chart-listesi --release

# İzleme günlüğü açık
UPLOT_IZLEME=1 cargo run -p uplot-rs-chart-listesi --release

# Web — katalog ve örnekler de izlenmeli, yoksa değişiklik yayına girmez
cd uygulamalar/web && trunk serve --release \
  --watch . --watch ../katalog/src --watch ../ornekler/src --watch ../../src

# Doğrulama
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
./tools/paket_siniri_denetle.sh
npm --prefix tools/uyum run denetle
cargo test --release -p uplot-rs-gpui-katalog --lib kok_render_kare_butcesi -- --nocapture
```

## Son ölçümler (bu makine)

| kart | kök render p50 |
|---|---:|
| ThinBars (55 yüzey) | 548 µs |
| TimezonesDst (51 yüzey) | 520 µs |
| LatencyHeatmap | 374 µs |
| MassSpectrum | 330 µs |

Raster katmanına daire desteği eklendikten sonra da bütçe korunuyor.
