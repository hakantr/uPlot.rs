# uPlot.rs WASM chart listesi

Masaüstü grafik listesiyle aynı `kart tanımı → Grafik → Sahne → SVG` hattını
tarayıcıya açar. Katalog şu anda `Resize` ve `Area Fill` kaynak kartlarını
içerir; yeni portlar aynı oturum ve kart seçme sözleşmesine eklenir.

## Derleme ve çalıştırma

```sh
cargo build -p uplot-rs-wasm --release --target wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.120
wasm-bindgen --target web --out-dir wasm/pkg \
  target/wasm32-unknown-unknown/release/uplot_rs_wasm.wasm
python3 -m http.server 8081
```

Ardından `http://localhost:8081/wasm/www/` adresini açın. `Resize` kartındaki
nokta sürgüsü aynı Rust kartını yeniden üretir; böylece farklı veri
yoğunlukları görsel olarak doğrulanabilir. `Area Fill`, resmî demonun üç
serisini ve dolgu modelini gösterir.

Web portunun geliştirme sözleşmesi **8081**'dir. Grafik üzerinde hover canlı
değerleri gösterir; yatay sürükleme X aralığına yakınlaştırır, çift tıklama tam
görünüme döner, tekerlek fare konumunu odak alarak yakınlaştırır, boşluk + sol
sürükleme yakın görünümü taşır. Kartta `dokunma_etkileşimi(true)` ise iki
parmakla X/Y yakınlaştırma ve tek parmakla taşıma da çalışır. Resmî
davranışlarla uPlot.rs uyarlamalarının ayrıntılı ayrımı için
[Resmî depo farklılıkları](../RESMI_DEPO_FARKLILIKLARI.md) belgesini okuyun.
