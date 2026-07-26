# uPlot.rs

[English](readme_en.md) · **Türkçe**

Bu proje, [uPlot](https://github.com/leeoniya/uPlot) 1.6.32'nin küçük, hızlı ve
bellek-verimli çizim yaklaşımını doğrudan GPUI'ye taşıyan bir Rust portudur.
Bağımsız olarak ortaya çıkarılmış yeni bir grafik motoru değildir. Normatif
kaynak, [uPlot deposundaki `0e5812c` commit'idir](https://github.com/leeoniya/uPlot/commit/0e5812c504430f5c804e0f993376d8999b26cc34);
davranış, API ve görsel uyum kararlarında uPlot esas alınır.

Kod tabanı Rust 2024 edition kullanır ve en az Rust 1.95 gerektirir. Yeni
modüller `mod.rs` yerine `foo.rs` + gerektiğinde `foo/alt_modul.rs` düzenini
izler.

`gpui` ve `gpui_kutuphanesi` commit pinlenmez. Path bağımlılıkları her yerel
derlemede kardeş çalışma ağaçlarının mevcut durumunu, CI ise depoların güncel
varsayılan dallarını kullanır. Yalnız normatif uPlot kaynağı commit kilitlidir.

Kütüphanenin tek interaktif renderer'ı GPUI'dir. Native uygulamalar ve web
hedefi aynı `GpuiGrafik` bileşenini kullanır; web yüzeyi `gpui_web` + WebGPU
üzerinde çalışır. SVG ikinci bir runtime renderer değildir, yalnız istendiğinde
retained GPUI grafik yüzeyinden üretilen vektör dışa aktarımdır.

Portun ortak altyapısı şunları içerir:

- doğrulanmış sütunlu/hizalı veri modeli;
- sayısal x ve sabit/otomatik y aralığı;
- GPUI paint ve isteğe bağlı SVG kaydı tarafından tüketilen retained çizim
  komutları;
- `../gpui_kutuphanesi` title bar/düğmelerini kullanan GPUI masaüstü chart listesi;
- `gpui_web` ile WebGPU canvas üzerinde çalışan GPUI Web uygulaması;
- native ve GPUI Web'de aynı Rust katalog kaynağından üretilen bütün kartlar;
- 18 kaynak dosyası, 304 genel API üyesi, 28 veri varlığı ve 73 demonun
  hash-kilitli envanteri;
- `Resize` kartı: `demos/resize.html` tabanlı 100 noktalı `sin(x)` çizgisi.
- `Area Fill` kartı: kaynaktaki 1…30 x dizisi, −10…10 değer havuzu, üç seri,
  sıfıra dolgu ve çoklu cursor/lejant davranışı.

`Resize` kartı, kaynak demonun koşullu boş noktalarını, dolu hover noktasını, canlı
lejantını, görünür aralığa göre yeniden hizalanan sayısal ızgarasını ve X
ekseninde sürükle-bırak yakınlaştırmasını da taşır.

GPUI chart listesi dağıtılan `uplot-rs` kütüphanesinin parçası değildir.
`uygulamalar/katalog` içindeki tek yayınlanmayan Rust entity'si hem
`uygulamalar/masaustu` hem `uygulamalar/web` girişleri tarafından kullanılır.
Kartın seçim, tekerlek, dokunma, taşıma, tam görünüm ve geçmiş davranışları çekirdekte çözülür.
Kütüphane kullanıcısı yalnız veriyi, renk düzenini ve açık/kapalı özellikleri
tanımlar; belirtilmeyen özellikler çekirdek varsayılanlarını kullanır.

## Kullanım

GPUI zorunlu ana bağımlılıktır; ayrıca bir `gpui` feature'ı açılmaz:

```toml
uplot-rs = { version = "0.1.0" }
```

Bu registry tanımı `0.1.0` sürümü crates.io'da yayımlandıktan sonra doğrudan
çalışır; kaynak depo aşağıda ayrıca verilmiştir.

Cargo paket adı küçük harfli `uplot-rs`, Rust kodundaki crate adı ise tirelerin
alt çizgiye çevrilmesi nedeniyle `uplot_rs` olur. Kaynak depo
[hakantr/uPlot.rs](https://github.com/hakantr/uPlot.rs) adresindedir.

Kullanımda açık GPUI ad alanı korunur:

```rust
use uplot_rs::{Grafik, gpui::GpuiGrafik};

let grafik = Grafik::yeni(seçenekler, veri)?;
let yüzey = cx.new(|_| GpuiGrafik::yeni(grafik));
```

GPUI katalog uygulaması bu bileşeni kullanır fakat kütüphane paketine girmez.
Retained sahne modeli GPUI'nin GPU hızlandırmasını kaldırmaz; komutlar
GPUI'nin GPU destekli `paint_path`/`paint_quad` hattına verilir.
Retained komut listesi genel amaçlı ikinci bir renderer backend'i değildir.
Normal entegrasyon `Grafik` + `gpui::GpuiGrafik` API'sini kullanır; test,
profil ve özel doğrulama araçlarının inceleme ihtiyacı için
`diagnostics::{Komut, Sahne}` ad alanında ayrılır.

### İsteğe bağlı GPUI → SVG kaydı

`gpui-svg` yalnız dışa aktarım API'sini derler:

```toml
uplot-rs = { version = "0.1.0", features = ["gpui-svg"] }
```

```rust
use uplot_rs::GpuiSvgKayıtAyarları;

let ayarlar = GpuiSvgKayıtAyarları::yeni(1_200, 600)?;
let svg = yüzey.read(cx).svg_kaydı(ayarlar);
std::fs::write("grafik.svg", svg.byte_değeri())?;
```

Serializer normal GPUI frame/paint yolunda çalışmaz. Yalnız `svg_kaydı`
çağrıldığında mevcut retained sahneyi okur; geometriyi yeniden hesaplamaz,
grafik durumunu değiştirmez ve raster `<image>` yerine düzenlenebilir vektör
öğeleri üretir.

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

Etkileşimli GPUI Web chart listesi GitHub Pages üzerinde yayınlanır:

**[uPlot.rs canlı GPUI Web örneğini aç](https://hakantr.github.io/uPlot.rs/)**

Her gün Türkiye saatiyle 21:00'de GPUI Web paketi yeniden derlenip Pages ortamına
yayınlanır ve şu indirilebilir workflow artefaktları oluşturulur:

- macOS ARM64;
- Linux ARM64;
- Linux x86_64;
- Windows x86_64;
- GPUI Web/WebGPU paketi.

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
cargo run -p uplot-rs-chart-listesi
cd uygulamalar/web && NO_COLOR=false trunk serve
npm --prefix tools/uyum run envanter
npm --prefix tools/uyum run denetle
```

## Hata yönetimi

Üretim Rust kodunda `panic!`, `unwrap`, `expect`, kontrolsüz dilim indeksleme,
`todo!`, `unimplemented!` ve `unreachable!` yasaktır. Doğrulama hataları tipli
`UplotHatası` değerleriyle çağırana döner; GPUI masaüstü ve web doğrulama
arayüzleri hatayı kart üzerinde kullanıcıya bildirir. Bu kural
workspace lintleri ve CI Clippy adımıyla her değişiklikte denetlenir.

İlk komut testleri çalıştırır. Masaüstü komutu canlı GPUI listesini, Trunk
komutu GPUI Web/WGPU uygulamasını açar. Envanter komutu kaynak/API/demo dökümlerini
yeniden üretir; denetim komutu [uPlot kaynak deposunun](https://github.com/leeoniya/uPlot)
aynı üst dizine `uPlot` adıyla klonlanmış yerel kopyasında commit/sürüm/dosya
hash kilidini doğrular. Tarayıcı kataloğu `uygulamalar/web/Trunk.toml`
üzerinden aynı ortak GPUI entity'sini açar.

## Kaynak düzeni

- `src/veri.rs`: uPlot hizalı sütun veri sözleşmesi
- `src/olcek.rs`: ölçek ve aralık matematiği
- `src/cizim.rs` + `src/cizim/`: crate içi retained GPUI çizim komutları ve kırpma;
  yalnız tanılama görünümü `diagnostics` ad alanından açılır
- `src/grafik.rs`: ilk çizim hattı
- `src/etkilesim.rs`: kartın etkileşim durumu, yakınlaştırma ve görünüm geçmişi
- `src/gpui.rs`: her normal build'de bulunan hazır GPUI grafik bileşeni
- `src/gpui/svg_kaydi.rs`: yalnız `gpui-svg` ile derlenen vektör dışa aktarım
- `src/secenek.rs` + `src/secenek/`: ilişkili seçenek türleri
- `uygulamalar/ornekler/`: yayınlanmayan kart yapılandırmaları ve kaynak veri
  fixture'ları; `uplot-rs` kullanıcılarının bağımlılıklarına girmez
- `uygulamalar/katalog/`: native ve web'in paylaştığı tek GPUI kart kayıt
  defteri, ilişkili yüzey grupları ve açıklama UI'si
- `uygulamalar/masaustu/`: ortak kataloğu native GPUI penceresinde açan giriş
- `uygulamalar/web/`: ortak kataloğu `gpui_web`/WebGPU üzerinde açan giriş
- `uyum/`: makine-okunur kaynak ve kanıt envanteri
- `tools/uyum/`: yeniden üretim/denetim araçları
- `RESMI_DEPO_FARKLILIKLARI.md`: resmî port ile uPlot.rs uzantılarının ayrımı
- `ORTAK_KART_DAVRANISLARI.md`: her yeni portta zorunlu ve CI tarafından
  denetlenen ortak görsel/etkileşim sözleşmesi

Ayrıntılı yol haritası için
[Tam GPUI geçiş faz planına](GPUI_GECIS_FAZ_PLANI.md) bakın.
Tamamlanan fazların commit, test ve release kanıtları
[GPUI geçişi son doğrulama kaydında](GPUI_GECIS_DOGRULAMA.md) tutulur.

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
