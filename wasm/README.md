# uPlot.rs WASM chart listesi

Masaüstü chart listesiyle aynı `sinüs_kartı → Grafik → Sahne → SVG` hattını
tarayıcıya açar.

## Derleme ve çalıştırma

```sh
cargo build -p uplot-rs-wasm --release --target wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.120
wasm-bindgen --target web --out-dir wasm/pkg \
  target/wasm32-unknown-unknown/release/uplot_rs_wasm.wasm
python3 -m http.server 8081
```

Ardından `http://localhost:8081/wasm/www/` adresini açın. Nokta sürgüsü aynı
Rust kartını yeniden üretir; böylece farklı veri yoğunlukları görsel olarak
doğrulanabilir.

Web portunun geliştirme sözleşmesi **8081**'dir. Grafik üzerinde hover canlı
değerleri gösterir; yatay sürükleme X aralığına yakınlaştırır, çift tıklama
ölçeği sıfırlar.
