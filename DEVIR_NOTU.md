# Devir notu

**Tarih:** 2026-07-29 (macOS oturumu)

Bu not, oturumun bittiği noktadan devam edebilmek içindir. Kalıcı mimari
kararlar `GPUI_GECIS_DOGRULAMA.md` içindedir; burada yalnız devir için
gereken durum, açık konular ve tekrar üretim adımları var.

## Depo durumu

| depo | dal | son commit | durum |
|---|---|---|---|
| `uPlot.rs` | `main` | `7c413ad` | temiz, gönderildi |
| `../gpui` | `main` | `d3a6038` | temiz, gönderildi |
| `../gpui_kutuphanesi` | `main` | `36f1174` | temiz, gönderildi |

Yan yana beklenen dizinler: `uPlot.rs`, `gpui`, `gpui_kutuphanesi`,
`uPlot` (normatif kaynak, `master`), `zed` (parite doğrulaması için
`7b030b5008`; bu makinede `401a0c7e3d` duruyor).

Bu oturumun commit'leri (`4b5fa67` sonrası, hepsi gönderildi):

```
7c413ad test(gpui): yerleşim ve gradyan şeritlerini regresyon testine bağla
2e8f256 docs: devir notunu sparkline ailesinin kapanışıyla güncelle
06807d1 fix(gpui): gradyan maskelerini yol ile aynı koordinat uzayına al
1adaa52 fix(gpui): yüzey taban yüksekliğini ham yüksekliğe bağla
9bae73e docs: devir notunu lejant çalışmasıyla güncelle
6296f82 feat(lejant): seri girdilerini tıklanabilir yap ve konumlandırılabilir kıl
12002a1 docs: devir notuna commit listesi ve öncelikli yapılacaklar ekle
b2427a9 docs: devir notunu eksen ve çubuk düzeltmeleriyle güncelle
a2add2b fix(çubuk): kırpılan değer ucunda köşe yuvarlatmasını kaldır
1fd03a9 fix(çubuk): değer etiketini çubuk kalınlığına göre ölçekle
db5525c fix(gpui): görünüm değişiminde imleç katmanını yeniden çöz
286d064 fix(eksen): Y ekseni başlığını değer etiketlerinin dışına al
2b8a8ec fix(test): raster testlerinde expect ve panic kullanımını kaldır
74c9307 fix(eksen): logaritmik X etiketlerini piksel alanına göre seyrelt
a74ea7b fix(gpui): dağılım yüzeylerini raster yolunda çiz
c03076c test(gpui): toplu daire yolunun kurulum sınırını belgele
eb8a375 fix(katalog): kart yüzeylerini görünür alana sığdır ve kesik içeriği gider
9587523 feat(gpui): yüzeyleri görünür alana uyarlayan yerleşim katmanı ekle
```

Doğrulama durumu: `cargo fmt --all --check` temiz, `cargo clippy --workspace
--all-targets` uyarısız, testler çekirdek 98 / örnekler 262 / katalog 15 +
sahne 2 / resize 13 / svg 6 / area_fill 3 / bütçe 2 geçiyor.

**`upstream_yol_butcesi` yalnız `--release` ile anlamlıdır.** Debug'da
`Path::scale` köşe başına ~19 ns ölçülür ve 4 ns bütçesini aşar; test
gpui'nin yol gönderim maliyetini ölçtüğünden bu bir regresyon değil,
optimizasyonsuz derlemenin sonucudur. Aşağıdaki doğrulama komutlarında
`cargo test --workspace` bu tek testi düşürür; `cargo test --release -p
uplot-rs-gpui-katalog --test upstream_yol_butcesi` ayrı koşulmalıdır.

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

**Lejant uPlot davranışına getirildi** (commit `6296f82`). Üç iş birlikte
yapıldı, çünkü tek tek yapılsa lejant tutarsız kalırdı.

Girdiler seri bazlı: işaret serinin rengini taşır, gizli seri kaynak
`.u-legend .u-off` kuralıyla aynı 0,3 opaklıkta listede kalır, tıklanınca
`seri_görünürlüğünü_ayarla` çağrılır. Eski `filter(|seri| seri.göster)`
elemesi yalnız gizli seriyi saklamıyordu, aynı zamanda değerleri
kaydırıyordu: değer listesi filtrelenmemiş seri sırasıyla geldiğinden gizli
serinin ardındaki her girdi komşusunun değerini gösteriyordu.

Konum çekirdeğe `GrafikSeçenekleri::lejant_konumu` olarak eklendi
(`LejantKonumu::{Alt, Üst, Sol, Sağ}`, varsayılan `Alt` = kaynak yerleşimi).
Yan konumlar tek sütunda listeler. Kart tanımlarının hepsi varsayılanı
kullandığından dördü de ancak denetim çubuğundaki döngü düğmesiyle
görülebilir.

13 tooltip kartının ayrı düğme satırı ve Timeseries Discrete'in yüzey içi
düğmeleri kaldırıldı. Birleşik indeks artık yüzey başına sabit seri sayısı
varsaymıyor; `lejant_hedefini_çöz` girdileri üreten sayımın tersini alıyor.
Kırılgan iki karar (değer kayması, birleşik indeks) saf fonksiyona çıkarılıp
üç birim testine bağlandı.

**Sparkline ailesi kapandı** (commit'ler `1adaa52`, `06807d1`). İki kart
bozuktu ve kök nedenleri ayrıydı; ikisi de önceki oturumun şüphelendiği
yol/atlas kapasitesi değil, boyama aşamasında koordinat hatasıydı.

*10×2 tablo:* grafik kökü sabit `min_h(px(120.0))` dayatıyordu, tablo
hücreleri ise 150×30. Her yüzey kendi hücresinden 90 px taşıp sonraki üç
satırın üstüne yazıyor, sonra çizilen üstte kaldığı için yalnız son satır
görünüyordu. Öndeki satırlardaki "boş `ArkaPlan`" da buydu: üstlerini
örten yüzeyin sparkline geometrisi kendi alt kısmında kalıyordu. Taban
artık ham yüksekliği aşamıyor (`en_az_yüzey_yüksekliği`). Ölçüm: yüzey
sınırları 150×120 → 150×30.

*Floating Bars:* gradyan ekseni `yolu_dönüştür` ile yüzey-yerel
hesaplanırken yol önbellekten `hedef_köken`e ötelenmiş çıkıyor ve
`mantıksal_sınırlar` mutlak oluyordu. `gradyan_yolunu_boya` iki uzayı
karıştırıp maske aralıklarını üst üste düşürüyordu; son boyanan yeşil
kırmızıyı örtüyordu. Ölçüm (800×400, köken 305,352): eksen 392..152
yerel, sınır 360..696 mutlak. Aynı karışım `gradients` kartında da
vardı — üç duraklı gradyanın yalnız ilk ikisi görünüyor, çizgi erken
kesiliyordu; o da düzeldi.

Önceki oturumun elediği hipotezler doğru çıktı: `Entity::cached` bu
oturumda da denendi (cache kapatıldı, resize ile bypass edildi), belirti
değişmedi.

**Yerleşim ve gradyan invaryantları teste bağlandı** (commit `7c413ad`).
Bu oturumdaki üç kusurun hiçbirini sahne komutu testleri yakalamadı;
komutlar doğruydu, hata yerleşim ve boyama aşamasındaydı. İki invaryant
o sınıfı kapsıyor.

`kart_yüzeyleri_üst_üste_yerleşmez` 66 kartı GPUI test bağlamında render
edip yüzeylerin ölçülen alanlarını (`GpuiGrafik::ölçülen_alan`)
karşılaştırıyor. Hücre boyutu bilmeye gerek yok. Testin gerçekten
koruduğu doğrulandı: `min_h` düzeltmesi geri alındığında 150×90 px
örtüşme raporluyor. Sanallaştırılmış kartlarda görünür alana girmemiş
yüzeyler ölçüm vermediği için atlanıyor; kartların yarısından azı ölçüm
verirse test ölçüm yolunun bozulduğunu söylüyor.

`gradyan_şeritleri` maske hesabını `gradyan_yolunu_boya` içinden saf bir
fonksiyona çıkarıyor; şeritler sınırlara kısılıyor ve aralıklar sınırları
boşluksuz, örtüşmesiz kaplıyor. Test karışık koordinat uzayını taklit
ediyor: o durumda son durak hiç şerit almıyor — yüzeyde ayrık gradyanın
bir dalının kaybolması olarak görünen belirti buydu.

**Eksen ve çubuk düzeltmeleri.** Logaritmik X ekseninde etiketler yüzey
boyutundan bağımsız üretiliyordu; Y'de var olan `log_etiketi_göster`
seyreltmesi X'e de uygulandı. Y ekseni başlığı eksen payının ortasına
konduğu için değer etiketleriyle çakışıyordu, kenara alındı. Çubuk ucundaki
değer yazısı sabit 10 px'ti, çubuk kalınlığına göre ölçekleniyor. Çubuk
çizim alanına kırpıldığında köşe yuvarlatması kavisi sınıra yapıştırıyordu,
kırpılan uçta köşe düz bırakılıyor. Görünüm değişiminde imleç katmanı eski
piksel konumunda kalıyordu, aynı fare konumundan yeniden çözülüyor.

## Açık konular

**1. Görsel regresyon kapsamı kısmi.** Yerleşim ve gradyan şeridi
invaryantları kuruldu (aşağıda), ama boyanan primitive'ler hâlâ
görülmüyor: `Window::rendered_frame` gpui'de `pub(crate)` ve gpui'ye
dokunulmuyor. Yakalanamayan sınıf: doğru yere doğru boyutta çizilen ama
yanlış renk/şekil taşıyan sahneler. SVG kaydı (`src/gpui/svg_kaydi.rs`)
sahne komutlarından ürediği için bu boşluğu kapatmaz; kapatmak için ya
gpui'de sahneyi okuyan bir test kancası ya da gerçek GPU karesinin
piksel karşılaştırması gerekir.

**2. Çoklu yüzeyli kartlarda lejant yalnız ilk yüzeyi gösteriyor.** Ortak
lejant `self.grafik`'i okur; Stacked Series gibi 16 yüzeyli kartlarda imleç
hangi yüzeydeyse onun değil, ilk yüzeyin değerleri listelenir. Yüzeylerin
kendi düğmeleri korunmalı — onlar yalnız görünürlük değil, bant yeniden
kurulumu da yapıyor (`stacked_series` açıklaması). Timeseries Discrete'in
birleşik lejantı bu işin nasıl genelleştirileceğini gösteriyor: lejant
`lejant_yüzeyleri` listesini gezip indeksi yüzey sınırının ötesine taşır.

Ayrıca `stacked-series` serilerinin bir kısmı kaynakta etiketsiz; lejantta
`● : --` olarak çıkıyorlar. uPlot da etiketsiz seriyi boş hücre gösterdiği
için parite doğru, ama katalogda tıklama hedefi ayırt edilemiyor.

**3. Tarayıcı önbelleği doğrulamayı yanıltıyor.** Chrome saatlerce eski
wasm'ı servis etti ve düzeltmeler "etkisiz" göründü. Doğrulama yaparken
URL'ye sürüm parametresi ekleyin (`?kart=...&v=N`) ve yüklenen dosya
adını `dist/` içindekiyle karşılaştırın.

**4. Chrome MCP birden çok tarayıcıya bağlı.** Bu makinede yerel Chrome'un
yanında uzak bir Linux Chrome da kayıtlı; komutlar varsayılan olarak
yanlış olana gidip `127.0.0.1:8081` için "error page" verebiliyor.
`list_connected_browsers` ile yerel olanı (`isLocal: true`) seçin.

## Yapılacaklar — öncelik sırası

**1. Çoklu yüzeyli kartlarda lejant (açık konu 2).**

**2. Görsel regresyonun kalan boşluğu (açık konu 1).** Yerleşim ve şerit
invaryantları kuruldu; kalan sınıf renk/şekil hataları. Bunun için gpui
tarafında sahneyi okuyan bir test kancası gerekiyor ve gpui katı Zed
paritesinde tutuluyor — bu yüzden sırada arkada duruyor.

Sırada olmayan ama not düşülmeye değer: `gpui_web` varsayılan
`multithreaded` özelliği hâlâ çekiliyor (yukarıda), kapatılırsa nightly ve
`build-std` bakımı tümüyle kalkabilir.

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
| ThinBars (55 yüzey) | 559 µs |
| TimezonesDst (51 yüzey) | 536 µs |
| LatencyHeatmap | 406 µs |
| Resize (tek yüzey) | 352 µs |
| MassSpectrum | 359 µs |

Raster katmanına daire desteği ve seri bazlı lejant eklendikten sonra da
bütçe korunuyor.
