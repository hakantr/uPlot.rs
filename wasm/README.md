# uPlot.rs WASM chart listesi

Masaüstü grafik listesiyle aynı `kart tanımı → Grafik → Sahne → SVG` hattını
tarayıcıya açar. Katalog, uyum manifestinde tamamlanan bütün kaynak kartlarını
ve bunların resmî alt grafiklerini içerir; yeni portlar aynı oturum ve kart
seçme sözleşmesine eklenir.

## Derleme ve çalıştırma

```sh
cargo build -p uplot-rs-wasm --release --target wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.120
wasm-bindgen --target web --out-dir wasm/pkg \
  target/wasm32-unknown-unknown/release/uplot_rs_wasm.wasm
python3 -m http.server 8081
```

Ardından `http://localhost:8081/wasm/www/` adresini açın. `Resize` kartındaki
nokta sürgüsü aynı Rust kartını yeniden üretir; diğer katalog girdileri de
resmî demo verilerini ve çekirdeğe port edilen ölçek, yol, dolgu ve etkileşim
davranışlarını görsel olarak doğrular.

Web portunun geliştirme sözleşmesi **8081**'dir. Grafik üzerinde hover canlı
değerleri gösterir; yatay sürükleme X aralığına yakınlaştırır, çift tıklama tam
görünüme döner, tekerlek fare konumunu odak alarak yakınlaştırır, boşluk + sol
sürükleme yakın görünümü taşır. Kartta `dokunma_etkileşimi(true)` ise iki
parmakla X/Y yakınlaştırma ve tek parmakla taşıma da çalışır. Resmî
davranışlarla uPlot.rs uyarlamalarının ayrıntılı ayrımı için
[Resmî depo farklılıkları](../RESMI_DEPO_FARKLILIKLARI.md) belgesini okuyun.

## Çizim yaşam döngüsü

Canlı grafiklerde pointer olayları tarayıcının boya karesiyle birleştirilir.
Cursor, hover noktaları ve seçim alanı SVG içindeki ayrı etkileşim grubunda
yerinde güncellenir. Veri veya ölçek değişip ana grafik yeniden çizildiğinde
yardım, lejant ve tooltip DOM'u korunur; yalnız grafik SVG yüzeyi değiştirilir.
Bu kurallar bütün kartlara ortak adaptör üzerinden uygulanır.
