# GPUI yetenek devir denetimi

**Tarih:** 2026-07-26

**uPlot.rs kod tabanı:** `62151d4`

**Yerel GPUI kod tabanı:** `18b9b8b4d6f5f8a6397e273f5b0fdbac51581192`

**Tek GPUI yetenek kaynağı:** `../gpui/yetenek.md` ve `../gpui` çalışma ağacı

Bu kayıt, uPlot.rs'in GPUI'de zaten bulunan platform ve UI işlerini ikinci kez
uygulayıp uygulamadığını denetler. GPUI yetenekleri için yayımlanmış eski web
belgeleri kullanılmamıştır. `../gpui/yetenek.md` kullanıcının yerel ve izlenmeyen
dosyası olarak korunmuş, değiştirilmemiştir.

## Kapsam

Denetim sırasında Git tarafından izlenen 240 dosyanın tamamı sınıflandırıldı:

| Alan | Dosya | Satır | Denetlenen sorumluluk |
|---|---:|---:|---|
| `src/` | 24 | 20.331 | dağıtılan kütüphane, GPUI adaptörü, grafik çekirdeği, SVG kaydı |
| `uygulamalar/` | 105 | 83.747 | yayınlanmayan kartlar, katalog, native ve web girişleri |
| `uyum/` | 79 | 11.217 | normatif uPlot envanteri ve kanıt matrisi |
| `tools/` | 9 | 813 | uyum ve tekrar-runtime korumaları |
| `assets/` | 8 | 3.813 | uygulama/dağıtım görselleri |
| `.github/` | 2 | 346 | test, performans, WASM, Pages ve nightly iş akışları |

Toplam izlenen içerik 129.517 satırdır. Bunun 111 dosya/54.304 satırı Rust,
8 dosya/1.590 satırı belgedir. Kalan içerik normatif fixture, envanter,
manifest, kilit ve uygulama varlıklarıdır.

## Devir sonucu

### GPUI'ye devredilen işler

| Sorumluluk | Önceki tekrar | Nihai sahip | Kanıt |
|---|---|---|---|
| Web platformu ve renderer başlatma | uPlot.rs içinde WebGPU preflight ve platform seçimi | `gpui_platform::web_init` + `single_threaded_web` | `13071b3`; GPUI `18b9b8b` |
| Browser event/resize/DPR/RAF çalışma zamanı | ayrı `wasm/` DOM/SVG uygulaması | GPUI Web platformu | `131d463` |
| Uzun kart listesi | bütün kartları her render'da üretme | GPUI `uniform_list` | `aa8c3c4` |
| Klavye komutları | ham Escape/1/2 kararları | GPUI `Action` + `KeyBinding` | `e93d2d7` |
| Kart odak/erişilebilirlik | click odaklı özel davranış | GPUI focus, key context ve AccessKit | `f14aa41` |
| Wheel zamanı | platformdan bağımsız yerde doğrudan saat okuma | GPUI background executor saati | `65f706f` |
| Ana ve etkileşim yüzeyi yaşam döngüsü | kök view içinde iki paint closure/cache | iki GPUI `Entity` + `cached` yüzey | `1718525` |
| Fiziksel piksel oranı | sabit 1× logical piksel varsayımı | GPUI `Window::scale_factor` | `4f6357b` |
| GPUI yaşam döngüsü testi | yalnız saf Rust sahne testleri | GPUI `TestAppContext` sentetik input | `62151d4` |

GPUI renk tipi yalnız hex çözümünü doğrudan sağladığı için uPlot'un CSS renk
sözleşmesi çekirdekte tutuldu. Boya yolunda tekrar parse maliyeti
`GpuiYolÖnbelleği` içine alınarak her benzersiz renk bir kez çözülür
(`3eef957`). Bu, GPUI'nin desteklemediği veri biçimini koruyan bir uyum
adaptörüdür; ikinci bir tema veya renk state sistemi değildir.

### Kütüphanede bilinçli olarak kalan işler

- Hizalı sütun veri modeli, `null`/hizalama eksiği ayrımı, `setData`,
  `setSeries`, seri yaşam döngüsü ve veri paylaşımı uPlot alan mantığıdır.
- Ölçek, range, zoom, pan, seçim, senkron grup, en yakın veri, hit-test,
  seyrekleştirme, gap clipping, bar/bant/ısı haritası ve path üretimi grafik
  çekirdeğinin görevidir. GPUI bunların semantiğini sağlamaz.
- `Sahne`/`Komut`, GPUI paint ile isteğe bağlı SVG kaydının ortak retained
  grafik display-list'idir. GPUI'nin özel `Scene` içini tersine çevirmek için
  kullanılmaz.
- Gerçek metin glyph şekillendirme ve boyama GPUI
  `text_system().shape_line()` hattındadır. Çekirdekte kalan yaklaşık etiket
  ölçüleri uPlot eksen payı, vektör SVG yerleşimi ve platformdan bağımsız
  hit-test içindir. Bunları GPUI font sistemine bağlamak SVG'yi ve headless
  çekirdek testlerini bir pencere/font ortamına bağımlı kılardı.
- `GpuiAnaYüzey` ve `GpuiEtkileşimYüzeyi` içindeki `Rc<RefCell>` alanları UI
  state sahibi değildir. GPUI `canvas` paint callback'inin `'static` ömrü ile
  kendi `Entity` varlığının yol önbelleği arasındaki tek iş parçacıklı
  erişim köprüsüdür. Grafik ve yüzey yaşam döngüsünün sahibi GPUI `Entity`dir.
- `uygulamalar/katalog/src/web_koprusu.rs` yalnız derin bağlantı URL'si ve
  kullanıcı isteğiyle SVG indirme Blob'u gibi tarayıcı kabuğu işlerini yapar.
  Grafik çizimi, input, resize, DPR veya frame scheduling içermez.

## Dağıtım sınırı

`cargo package --list` sonucu dağıtılan paketin yalnız şunları içerdiğini
doğrular:

- `src/` altındaki 24 kütüphane dosyası;
- `Cargo.toml`/`Cargo.lock`;
- `README.md`, `readme_en.md`, `LICENSE`, `NOTICE`;
- Cargo'nun ürettiği paket metadata dosyaları.

`uygulamalar/`, katalog kartları, fixture'lar, performans testleri, `uyum/`,
`tools/`, web/native girişleri ve uygulama ikonları geliştiricinin dependency
graph'ına girmez. GPUI `test-support` yalnız `dev-dependency`dir; normal
kütüphane tüketicisine proptest veya test platformu taşımaz.

## SVG sınırı

`gpui-svg` bir runtime fallback değildir. Normal kare:

```text
Grafik → retained Sahne → GPUI Entity/canvas → GPUI paint
```

Yalnız açık dışa aktarım isteği:

```text
GpuiGrafik::svg_kaydı → aynı retained Sahne → vektör serializer
```

Bu nedenle GPUI frame yolunda XML, SVG buffer'ı veya kayıt dallanması yoktur.
Tarayıcıda WebGPU adaptörü yoksa renderer kararını ve hatayı GPUI verir;
uPlot.rs ikinci bir SVG uygulamasına sessizce geçmez.

## Stable/nightly kararı

Yerel GPUI'nin tek iş parçacıklı web yolu stable Rust ile kullanılabilir.
uPlot.rs için nightly derleyiciye geçmek ölçülmüş bir çalışma zamanı kazancı
sağlamaz; buna karşılık araç zinciri ve tüketici uyumluluğunu daraltır.
Sonuç olarak Rust 1.95 stable korunmuştur. Nightly ancak ileride yerel GPUI
API'si zorunlu bir nightly özelliği kullanır ve aynı yetenek stable yolda
bulunmazsa ayrı bir kanıt kartıyla değerlendirilecektir.

## Koruma ve kabul

- CI ve nightly aynı GPUI commit'ini kullanır (`08f9844`).
- `tools/uyum/denetle.mjs`, eski `wasm/` runtime'ının veya bağımsız ikinci
  renderer'ın geri gelmesini reddeder.
- GPUI `TestAppContext` testi, hover sırasında ana retained sahnenin sabit
  kaldığını ve yalnız etkileşim sahnesinin ilerlediğini doğrular.
- Core/örnek/entegrasyon/SVG/performance testleri, tüm-feature Clippy,
  workspace WASM ve üretim Trunk derlemesi son kabul kapılarıdır.

Denetim sonucu: GPUI'nin verebildiği platform, layout-listesi, input,
erişilebilirlik, zaman, DPR, render yaşam döngüsü ve test kararları GPUI'ye
devredilmiştir. Kalan kod ya uPlot alan semantiği ya da GPUI'nin sağlamadığı
isteğe bağlı vektör kayıt/uyum adaptörüdür.
