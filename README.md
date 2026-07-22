# uPlot.rs

[English](readme_en.md) · **Türkçe**

Bu proje, [uPlot](https://github.com/leeoniya/uPlot) 1.6.32'nin küçük, hızlı ve
bellek-verimli çizim yaklaşımını Rust'a, GPUI'ye ve WASM'e taşıyan bir porttur.
Bağımsız olarak ortaya çıkarılmış yeni bir grafik motoru değildir. Normatif
kaynak `../uPlot` deposunun `0e5812c504430f5c804e0f993376d8999b26cc34`
commit'idir; davranış, API ve görsel uyum kararlarında uPlot esas alınır.

Kod tabanı Rust 2024 edition kullanır ve en az Rust 1.95 gerektirir. Yeni
modüller `mod.rs` yerine `foo.rs` + gerektiğinde `foo/alt_modul.rs` düzenini
izler.

`gpui` ve `gpui_kutuphanesi` commit pinlenmez. Path bağımlılıkları her yerel
derlemede kardeş çalışma ağaçlarının mevcut durumunu, CI ise depoların güncel
varsayılan dallarını kullanır. Yalnız normatif uPlot kaynağı commit kilitlidir.

Port şu anda Faz 0 altyapısı ve ilk dikey uyum kartını içerir:

- doğrulanmış sütunlu/hizalı veri modeli;
- sayısal x ve sabit/otomatik y aralığı;
- GPUI'den bağımsız deterministik sahne komutları;
- bağımlılıksız SVG yüzeyi;
- `../gpui_kutuphanesi` title bar/düğmelerini kullanan GPUI masaüstü chart listesi;
- 8081 portunda çalışan etkileşimli WASM chart listesi;
- masaüstü ve WASM'de ortak Rust kart tanımı örneği;
- kaynak kilidi, API matrisi, demo manifesti ve senaryo kaydı;
- ilk kart: `demos/resize.html` tabanlı 100 noktalı `sin(x)` çizgisi.

İlk kart, kaynak demonun koşullu boş noktalarını, dolu hover noktasını, canlı
lejantını ve X ekseninde sürükle-bırak yakınlaştırmasını da taşır.

## Canlı örnek ve otomatik derlemeler

Etkileşimli WASM chart listesi GitHub Pages üzerinde yayınlanır:

**[uPlot.rs canlı WASM örneğini aç](https://hakantr.github.io/uPlot.rs/)**

Her gün Türkiye saatiyle 21:00'de WASM paketi yeniden derlenip Pages ortamına
yayınlanır ve şu indirilebilir workflow artefaktları oluşturulur:

- macOS ARM64;
- Linux ARM64;
- Linux x86_64;
- Windows x86_64;
- WASM web paketi.

Gece derlemeleri ve elle çalıştırma için
[nightly-artifacts workflow'una](https://github.com/hakantr/uPlot.rs/actions/workflows/nightly-builds.yml)
bakın.

Yeni bir gece koşusu başladığında bekleyen/eski koşu iptal edilir. Yalnız son
gece koşusunun artefaktları ve son iki Pages dağıtım kaydı tutulur; GitHub
Release sürümleri bu temizlikten etkilenmez.

## Çalıştırma

```sh
cargo test
cargo run --example ilk_kart
cargo run --example chart_listesi
npm --prefix tools/uyum run denetle
```

İlk komut testleri, ikinci komut `target/ilk-kart.svg` çıktısını, üçüncü komut
canlı GPUI listesini açar. Son komut kardeş `../uPlot` deposunun
commit/sürüm/dosya hash kilidini doğrular. Tarayıcı listesi için
[wasm/README.md](wasm/README.md) yönergelerini kullanın.

## Kaynak düzeni

- `src/veri.rs`: uPlot hizalı sütun veri sözleşmesi
- `src/olcek.rs`: ölçek ve aralık matematiği
- `src/cizim.rs`: yüzeyden bağımsız sahne komutları ve SVG çıktısı
- `src/grafik.rs`: ilk çizim hattı
- `src/kart.rs`: kanıtlanabilir kart fixture'ları
- `uyum/`: makine-okunur kaynak ve kanıt envanteri
- `tools/uyum/`: yeniden üretim/denetim araçları

Ayrıntılı yol haritası için [UPLOT_TAM_UYUM_FAZ_PLANI.md](UPLOT_TAM_UYUM_FAZ_PLANI.md)
dosyasına bakın.

## Atıf ve teşekkür

Grafik motorunun özgün tasarımı, performans yaklaşımı, API fikirleri,
algoritmaları, varsayılan davranışları ve demo senaryoları
[uPlot deposuna](https://github.com/leeoniya/uPlot) aittir. Bu depodaki Rust
kodu; söz konusu çalışmayı farklı çalışma zamanı ve arayüzlere uyarlamak,
eşdeğerliğini sınamak ve belgelemek amacıyla geliştirilir.

uPlot'un yaratıcısı Leon Sorokin'e ve bugüne kadar kaynak projeye kod, hata
raporu, inceleme, dokümantasyon ve geri bildirimle katkıda bulunan bütün uPlot
katkıcılarına içtenlikle teşekkür ederiz. uPlot.rs'nin ulaşabildiği işlevsellik
ve doğruluk, onların açık kaynak olarak paylaştığı çalışma sayesinde mümkündür.

## Lisans

Bu repo Apache-2.0 lisanslıdır. Normatif kaynak uPlot MIT lisanslıdır; özgün
telif ve lisans bildirimi [NOTICE](NOTICE) içinde korunur.
