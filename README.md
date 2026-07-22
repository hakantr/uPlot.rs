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
lejantını ve X ekseninde sürükle-bırak yakınlaştırmasını da taşır.

### Etkileşim seçenekleri ve kaynak ayrımı

Kart etkileşimleri `EtkileşimSeçenekleri` ile ayrı ayrı açılıp kapatılır:

```rust
.etkileşimler(EtkileşimSeçenekleri::default()
    .tekerlek_etkileşimi(true)
    .tekerlek_ayarları(TekerlekAyarları::default()
        .kip(TekerlekKipi::Otomatik))
    .seçim_yakınlaştır(true)
    .çift_tıkla_tam_görünüm(true)
    .görünüm_geçmişi(true))
```

`seçim_yakınlaştır` ile `çift_tıkla_tam_görünüm`, uPlot çekirdeğinin
davranışlarıdır. `tekerlek_etkileşimi`, uPlot'un resmi
[`wheelZoomPlugin`](https://github.com/leeoniya/uPlot/blob/0e5812c504430f5c804e0f993376d8999b26cc34/demos/zoom-wheel.html)
portudur ve eklenti olduğu için varsayılan olarak kapalıdır.
`görünüm_geçmişi` ise uPlot.rs'e özgü “Geri” uzantısıdır ve o da varsayılan
olarak kapalıdır. İlk kart, görsel ve davranışsal doğrulama için dördünü de
açık tanımlar. WASM ve masaüstü örneklerindeki “Tekerlek eklentisi” anahtarı,
bu kart ayarını canlı olarak `true`/`false` arasında değiştirir.

`TekerlekKipi::Otomatik`, satır tabanlı klasik mouse tekerleğinde resmi
`0.75` adımını korur. Magic Mouse ve trackpad gibi piksel tabanlı hassas
girdilerde delta büyüklüğüyle orantılı yakınlaştırma, küçük hareketler için
ölü bölge, kare başına olay birleştirme ve hareket başına tek geçmiş kaydı
uygular. Bu giriş normalizasyonu resmi eklentinin bir parçası değil, farklı
aygıtlarda eşdeğer kontrol sağlamak için eklenen uPlot.rs uyarlamasıdır.

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
cargo run --example chart_listesi
npm --prefix tools/uyum run denetle
```

## Hata yönetimi

Üretim Rust kodunda `panic!`, `unwrap`, `expect`, kontrolsüz dilim indeksleme,
`todo!`, `unimplemented!` ve `unreachable!` yasaktır. Doğrulama hataları tipli
`UplotHatası` değerleriyle çağırana döner; masaüstü arayüzü hatayı kart üzerinde,
WASM arayüzü ise güvenli bir hata SVG'siyle kullanıcıya bildirir. Bu kural
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
