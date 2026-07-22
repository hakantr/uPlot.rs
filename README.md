# uPlot.rs

[English](readme_en.md) · **Türkçe**

Bu proje, [uPlot](https://github.com/leeoniya/uPlot) 1.6.32'nin küçük, hızlı ve
bellek-verimli çizim yaklaşımını Rust'a, GPUI'ye ve WASM'e taşıyan bir porttur.
Bağımsız olarak ortaya çıkarılmış yeni bir grafik motoru değildir. Normatif
kaynak, [uPlot deposundaki `0e5812c` commit'idir](https://github.com/leeoniya/uPlot/commit/0e5812c504430f5c804e0f993376d8999b26cc34);
davranış, API ve görsel uyum kararlarında uPlot esas alınır.

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
lejantını, görünür aralığa göre yeniden hizalanan sayısal ızgarasını ve X
ekseninde sürükle-bırak yakınlaştırmasını da taşır.

GPUI chart listesi dağıtılan `uplot-rs` kütüphanesinin parçası değildir;
`uygulamalar/masaustu` altındaki ayrı, yayınlanmayan bir doğrulama uygulamasıdır.
Kartın seçim, tekerlek, dokunma, taşıma, tam görünüm ve geçmiş davranışları çekirdekte çözülür.
Kütüphane kullanıcısı yalnız veriyi, renk düzenini ve açık/kapalı özellikleri
tanımlar; belirtilmeyen özellikler çekirdek varsayılanlarını kullanır.

## Çizim yüzeyi feature'ları

Cargo feature'ları toplamsaldır ve birbiriyle birlikte açılabilir:

```toml
uplot-rs = {
    path = "../uPlot.rs",
    default-features = false,
    features = ["gpui", "svg"]
}
```

- `svg`: varsayılan, bağımlılıksız SVG çıktısı;
- `wasm`: WASM yüzeyi için `svg` desteğini açar;
- `gpui`: hazır `uplot_rs::gpui::GpuiGrafik` bileşenini açar.

`gpui` feature'ı pinsiz `../gpui/crates/gpui` çalışma ağacına bağlıdır. Feature
yalnız modülü derlemeye açar; kullanımda açık ad alanı korunur:

```rust
use uplot_rs::gpui::GpuiGrafik;

let grafik = Grafik::yeni(seçenekler, veri)?;
let yüzey = cx.new(|_| GpuiGrafik::yeni(grafik));
```

GPUI katalog uygulaması bu bileşeni kullanır fakat kütüphane paketine girmez.

## Kart etkileşimleri

İsteğe bağlı resmî eklenti davranışları kart tanımında açılıp kapatılır:

```rust
let etkileşimler = EtkileşimSeçenekleri::default()
    .tekerlek_etkileşimi(true)
    .dokunma_etkileşimi(true)
    .seçim_yakınlaştır(true);
```

`dokunma_etkileşimi(true)`, `demos/zoom-touch.html` kaynaklı iki parmakla X/Y
yakınlaştırmayı ve yakınlaştırılmış görünümde tek parmakla taşımayı açar.
Masaüstünde grafik yakınlaştırıldıktan sonra boşluk + sol sürükleme otomatik
olarak taşıma yapar; bunun için ikinci bir kart seçeneği gerekmez. `false`
verilen isteğe bağlı davranışlar kapanır, hiç belirtilmeyenler `Default`
değerleriyle çalışır.

## Resmî depodan farklı işleyişler

Port zorunlulukları, API uyarlamaları ve uPlot.rs'e özgü uzantılar ana README'yi
büyütmemek için ayrı bir envanterde tutulur. Ayrıntılar ve kaynak ayrımı için
[Resmî uPlot deposundan farklılıklar](RESMI_DEPO_FARKLILIKLARI.md) belgesini
okuyun.

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

## Uygulama ikonu

<img src="assets/app-icon.svg" width="128" alt="uPlot.rs uygulama ikonu">

Tek SVG ana kaynaktan web faviconu, Linux PNG masaüstü ikonu, macOS ICNS
uygulama/dock ikonu ve Windows ICO/EXE ikonu üretilir. Gece artefaktı macOS'ta
`.app` paketi, Linux'ta `.desktop` kaydıyla taşınabilir dizin ve Windows'ta
ikonu gömülü `uplot-rs.exe` içerir.

## Çalıştırma

```sh
cargo test
cargo run --example ilk_kart
cargo run -p uplot-rs-chart-listesi
npm --prefix tools/uyum run denetle
```

## Hata yönetimi

Üretim Rust kodunda `panic!`, `unwrap`, `expect`, kontrolsüz dilim indeksleme,
`todo!`, `unimplemented!` ve `unreachable!` yasaktır. Doğrulama hataları tipli
`UplotHatası` değerleriyle çağırana döner; masaüstü ve WASM doğrulama
arayüzleri hatayı kart üzerinde kullanıcıya bildirir. Bu kural
workspace lintleri ve CI Clippy adımıyla her değişiklikte denetlenir.

İlk komut testleri, ikinci komut `target/ilk-kart.svg` çıktısını, üçüncü komut
canlı GPUI listesini açar. Son komut, [uPlot kaynak deposunun](https://github.com/leeoniya/uPlot)
aynı üst dizine `uPlot` adıyla klonlanmış yerel kopyasında commit/sürüm/dosya
hash kilidini doğrular. Tarayıcı listesi için
[wasm/README.md](wasm/README.md) yönergelerini kullanın.

## Kaynak düzeni

- `src/veri.rs`: uPlot hizalı sütun veri sözleşmesi
- `src/olcek.rs`: ölçek ve aralık matematiği
- `src/cizim.rs`: yüzeyden bağımsız sahne komutları ve SVG çıktısı
- `src/grafik.rs`: ilk çizim hattı
- `src/etkilesim.rs`: kartın etkileşim durumu, yakınlaştırma ve görünüm geçmişi
- `src/gpui.rs`: `gpui` feature'ıyla açılan hazır GPUI grafik bileşeni
- `src/svg.rs`: `svg`/`wasm` feature'larının SVG yüzeyi
- `src/kart.rs`: kanıtlanabilir kart fixture'ları
- `uygulamalar/masaustu/`: dağıtıma girmeyen GPUI doğrulama uygulaması
- `uyum/`: makine-okunur kaynak ve kanıt envanteri
- `tools/uyum/`: yeniden üretim/denetim araçları
- `RESMI_DEPO_FARKLILIKLARI.md`: resmî port ile uPlot.rs uzantılarının ayrımı

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
