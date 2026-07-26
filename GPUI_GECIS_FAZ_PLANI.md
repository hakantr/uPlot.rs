# uPlot.rs Tam GPUI Geçiş Faz Planı

## 1. Amaç

Bu plan uPlot.rs'i birden fazla UI framework'üne uyarlanabilen genel amaçlı
bir grafik motoru olmaktan çıkarıp doğrudan GPUI uygulamalarına hizmet eden,
native ve web üzerinde aynı Rust bileşenini kullanan bir GPUI grafik
kütüphanesine dönüştürür.

Geçiş tamamlandığında:

- dağıtılan birincil API `gpui::GpuiGrafik` ve ilişkili GPUI entity/element
  türlerinden oluşur;
- macOS, Linux ve Windows aynı GPUI bileşenini kullanır;
- web hedefi SVG/DOM tabanlı ikinci bir arayüz yerine `gpui_web` ve WebGPU
  üzerinden aynı GPUI bileşenini çalıştırır;
- kart verisi, ölçek matematiği, path üretimi, hit-test ve etkileşim durum
  makineleri tek Rust çekirdeğinde kalır;
- SVG bir çalışma zamanı renderer'ı değildir; GPUI grafik yüzeyinin
  vektörel dışa aktarım desteğidir;
- eski port fazları, geçici uyum notları, çift katalog tanımları ve kart
  başına SVG CLI örnekleri kaldırılır;
- native ve web katalogları aynı Rust kart kayıt defterini, aynı GPUI
  görünümünü ve aynı etkileşim kodunu çalıştırır.

## 2. Mimari kararlar

### 2.1 GPUI-first, tek interaktif renderer

İnteraktif çizim ve kullanıcı olayı işleme yalnız GPUI üzerinden yapılır.
`Grafik`, `HizalıVeri`, ölçekler ve geometri üreticileri GPUI bileşeninin iç
uygulama ayrıntılarıdır; başka UI framework'leri için bağımsız adaptör
sözleşmesi sunulmaz.

### 2.2 Retained grafik display-list'i korunur

`Komut` ve `Sahne` SVG'ye özgü değildir. Bunlar veri/ölçek hesaplaması ile
GPUI paint çağrıları arasındaki retained grafik display-list'idir. Aşağıdaki
amaçlarla korunur:

- değişmeyen katmanların ve path geometrisinin yeniden kullanılabilmesi;
- GPUI paint ile SVG kaydının aynı semantik kaynaktan üretilebilmesi;
- headless geometri ve etkileşim testlerinin GPU gerektirmeden çalışması;
- platformlar arasında deterministik sonuç alınması.

Bu türler genel renderer eklenti API'si olarak belgelenmez. Kütüphane içi
GPUI çizim sözleşmesi olarak konumlandırılır.

### 2.3 SVG desteğinin kesin sınırı

GPUI'nin genel `gpui::Scene` nesnesi GPU paint aşamasında metni glyph
atlaslarına, görselleri texture atlaslarına ve stroked path'leri tessellated
üçgenlere dönüştürür. Bu aşamadan sonra özgün `<text>`, font ve path
semantiğini eksiksiz geri kurmak mümkün değildir.

Bu nedenle `gpui-svg` desteği ekran görüntüsünü SVG'ye çevirmeye çalışmaz.
`GpuiGrafik`, GPUI paint tarafından kullanılan aynı retained grafik
display-list'ini ve aynı yüzey dönüşümünü yalnız dışa aktarım istendiği anda
vektörel kaydediciye verir:

```text
Grafik durumu
    └─ retained grafik display-list'i
         ├─ normal frame → GPUI paint_path / paint_quad / text_system
         └─ svg_kaydı çağrısı → GPUI SVG kaydedici
              └─ path / rect / circle / text / gradient / clipPath
```

Sonuç raster `<image>` kapsayıcısı değil, düzenlenebilir gerçek SVG şekilleri
ve metinleridir.

`gpui-svg`:

- yalnız `GpuiGrafik` tarafından çizilen grafik yüzeyini kaydeder;
- rastgele bir üçüncü taraf GPUI uygulamasının tamamını SVG'ye dönüştürmeyi
  vaat etmez;
- ana grafik katmanını zorunlu, cursor/seçim/tooltip etkileşim katmanını
  isteğe bağlı kaydeder;
- GPUI yüzeyinin gerçek genişlik, yükseklik, aspect-fit ofseti ve clip
  sınırlarını uygular;
- normal GPUI paint sırasında çalışmaz, SVG metni oluşturmaz ve kayıtçı
  dallanması yapmaz;
- retained komutları yalnız `svg_kaydı(...)`/`svg_dosyasına_yaz(...)`
  çağrıldığında tek seferlik okur;
- native hedefte dosyaya atomik olmayan fakat tipli `std::io::Result` ile
  yazma kolaylığı; tüm hedeflerde SVG `String`/byte çıktısı sağlar.

Bu ayrım bir performans sözleşmesidir: `gpui-svg` feature'ının derlenmiş
olması tek başına frame maliyeti oluşturmaz. Normal çizim yolunda kaydedici
nesnesi, SVG buffer'ı, XML biçimlendirme, komut kopyası veya komut başına
`if recording` kontrolü bulunmaz. Dışa aktarım ayrı, açıkça çağrılan ve
senkron bir snapshot işlemidir.

### 2.4 Web hedefi

Web hedefi `wasm/src/lib.rs → SVG String → JavaScript → DOM` hattını
kullanmaz. Hedef hat:

```text
Ortak Rust katalog entity'si
    → GpuiGrafik
    → gpui_web::WebPlatform
    → gpui_wgpu
    → HTMLCanvasElement / WebGPU
```

JavaScript yalnız WASM modülünü başlatır ve WebGPU başlatma hatasını
kullanıcıya gösterir. Kart, seri, zoom veya tooltip davranışı JavaScript'te
tekrar uygulanmaz.

## 3. Tamamlanma tanımı

Geçiş ancak aşağıdaki koşulların tümü sağlandığında `%100` kabul edilir:

1. Root crate varsayılan olarak GPUI bileşenini derler.
2. GPUI bağımlılığı yalnız `../gpui/crates/gpui` yolundan gelir.
3. Dağıtılan API'de başka framework renderer/adaptör sözleşmesi bulunmaz.
4. Native katalog yalnız `GpuiGrafik` kullanır.
5. Web katalog yalnız `gpui_web` üzerinden aynı `GpuiGrafik` türünü kullanır.
6. Native ve web kart kayıtları tek Rust kaynak dosyasından gelir.
7. Eski SVG/DOM WASM runtime'ı kaldırılır.
8. SVG yalnız `gpui-svg` dışa aktarım feature'ı olarak kalır.
9. GPUI SVG çıktısı ana ve isteğe bağlı etkileşim katmanını gerçek vektör
   öğeleriyle kaydeder.
10. Wheel, Shift+wheel, Ctrl+wheel, drag, seçim, görünüm geçmişi, resize,
    cursor sync ve touch/pinch native ve web kabul testlerinden geçer.
11. Statik hover ana sahne revision'ını değiştirmez.
12. `setData`/`setSeries` yalnız kirlenen katmanları ve serileri yeniler.
13. Linux GPU adaptörü ile yazılım fallback'i tanılanabilir biçimde raporlanır.
14. Stable Rust 1.95 ile native ve `wasm32-unknown-unknown` derlemeleri geçer.
15. Format, Clippy, tüm testler ve performans bütçeleri geçer.
16. `main` ile `origin/main` eşittir ve çalışma ağacı temizdir.

## 4. Faz ve ilerleme ağırlıkları

| Faz | Konu | Toplam ilerleme |
|---|---|---:|
| 0 | Arşiv dalı ve başlangıç envanteri | %5 |
| 1 | GPUI platform/web önkoşulları | %12 |
| 2 | Root crate ve API sınırları | %20 |
| 3 | GPUI → SVG vektör kayıt desteği | %30 |
| 4 | Retained katman ve cache mimarisi | %43 |
| 5 | Tek Rust kart kayıt defteri | %51 |
| 6 | Ortak GPUI katalog uygulaması | %61 |
| 7 | Native uygulama geçişi | %68 |
| 8 | GPUI Web/WASM geçişi | %79 |
| 9 | Etkileşim ve senkronizasyon eşliği | %88 |
| 10 | Eski runtime, örnek ve not temizliği | %93 |
| 11 | Belge, CI ve yayın hattı | %97 |
| 12 | Tam doğrulama ve performans kapıları | %100 |

Her faz ayrı commit edilir ve `main` dalına pushlanır. Faz geçişinde kullanıcıya
toplam ilerleme yüzdesi bildirilir.

### 4.1 Uygulama durumu

2026-07-26 itibarıyla Faz 0–12 tamamlanmıştır. Normatif kabul kanıtları,
ölçümler ve commit eşlemesi `GPUI_GECIS_DOGRULAMA.md` dosyasındadır.

| Faz | Durum | Son kanıt |
|---:|---|---|
| 0 | Tamamlandı | arşiv `5c60cb5` |
| 1 | Tamamlandı | `c7b8d85`, `17a6bb1` |
| 2 | Tamamlandı | `dcc9dae`, `567befa`, `87bd9e3` |
| 3 | Tamamlandı | `86a2ec6`, `f6d9545` |
| 4 | Tamamlandı | `7499cca`, `f6d9545`, `87bd9e3`; GPUI `bd12656` |
| 5 | Tamamlandı | `e48cda4` |
| 6 | Tamamlandı | `b6206d0` |
| 7 | Tamamlandı | `3e95b90`, `7ae95ba` |
| 8 | Tamamlandı | `66b7773` |
| 9 | Tamamlandı | `8baed24` |
| 10 | Tamamlandı | `a31df4a` |
| 11 | Tamamlandı | `7ae95ba` |
| 12 | Tamamlandı | `8cdf785`, `6036249`, `87bd9e3` |

## 5. Faz 0 — Arşiv ve envanter

### Durum

Tamamlandı. `main` başlangıç durumu
`codex/pre-gpui-primary-archive` dalına taşınmış ve uzak depoya gönderilmiştir.

### Çıktılar

- geri dönüş için değiştirilemez başlangıç referansı;
- feature, renderer, WASM, katalog, örnek, belge ve CI envanteri;
- root crate'in GPUI feature'ıyla native ve wasm32 derlenebilirliğinin
  başlangıç kontrolü.

### Kabul

- arşiv dalı `5c60cb5` commit'ini gösterir;
- yeni geliştirme `main` üzerinde yapılır;
- başlangıç çalışma ağacı temizdir.

## 6. Faz 1 — GPUI platform ve web önkoşulları

### İşler

1. `../gpui` sürümünde `gpui`, `gpui_wgpu`, `gpui_web` ve
   `gpui_platform` API'lerini sabitle.
2. Root crate'in `gpui` feature olmadan derlenmesi ihtiyacını kaldır.
3. Web uygulamasında `gpui_platform::application()` kullanma; tek iş
   parçacıklı stable yol için doğrudan:
   - `gpui_web::init_logging()`;
   - `gpui_web::WebPlatform::new(false)`;
   - `Application::with_platform(...)`
   kullan.
4. `gpui_web` bağımlılığını `default-features = false` tanımlayarak
   `wasm_thread` nightly zorunluluğunu kaldır.
5. WebGPU bulunamadığında boş canvas yerine okunabilir başlatma hatası üret.
6. DPR, `ResizeObserver`, requestAnimationFrame ve canvas boyut akışını smoke
   testlerle doğrula.
7. GPUI Web pointer olaylarında pointer kimliği, çoklu temas,
   `TouchEvent`/`PinchEvent` ve gesture bitiş akışını doğrula; eksikse
   `../gpui` için ayrı, testli düzeltme hazırla.

### Değişecek alanlar

- `Cargo.toml`
- yeni ortak katalog/web crate Cargo tanımları
- gerektiğinde `../gpui/crates/gpui_web`
- gerektiğinde `../gpui/crates/gpui_platform`

### Kabul

- stable Rust ile GPUI Web minimal uygulaması derlenir;
- WebGPU hata yolu test edilir;
- tek ve iki parmak olay dizileri GPUI event API'sine ulaşır;
- root crate native ve wasm32 GPUI hedeflerinde derlenir.

## 7. Faz 2 — Root crate ve dağıtılan API

### İşler

1. `gpui` bağımlılığını opsiyonel olmaktan çıkar.
2. Varsayılan feature kümesini GPUI çalışma zamanı olarak tanımla.
3. Eski `wasm = ["svg"]` zincirini kaldır.
4. `svg` genel renderer feature'ını `gpui-svg` dışa aktarım feature'ına
   dönüştür.
5. `src/lib.rs` crate belgesini GPUI bileşeni sözleşmesine göre yaz.
6. `GpuiGrafik`, olaylar, SVG kayıt seçenekleri ve kullanıcıya gereken veri/
   seçenek türlerini üst düzeyde yeniden export et.
7. `Komut` ve `Sahne` türlerini genel framework backend API'si olarak
   tanıtmaktan vazgeç; gerekli debug/test erişimini ayrı modüle al.
8. Kart oluşturucularının `GrafikSeçenekleri + HizalıVeri` yerine doğrudan
   `Grafik` veya `GpuiGrafik` üretmesini sağlayacak geçiş API'si ekle.
9. Eski API için geçici `deprecated` köprü kullanma; arşiv dalı geri dönüş
   noktasıdır.

### Kabul

- README'deki en kısa kullanım örneği doğrudan GPUI entity oluşturur;
- default build `GpuiGrafik` içerir;
- SVG dışa aktarım kapalıyken SVG serializer kodu derlenmez;
- public API taramasında başka framework adaptörü yoktur.

## 8. Faz 3 — GPUI → SVG vektör kayıt desteği

### API

Planlanan temel API:

```rust
let kayıt = GpuiSvgKayıtAyarları::yeni(1_200, 600)?
    .etkileşim_katmanı(false);

let svg = grafik.read(cx).svg_kaydı(kayıt);

#[cfg(not(target_family = "wasm"))]
grafik.read(cx).svg_dosyasına_yaz("grafik.svg", kayıt)?;
```

Nihai adlar Rust/Türkçe API düzeniyle uyumlu tutulur.

### İşler

1. GPUI paint yüzeyinin aspect-fit dönüşümünü tek bir
   `GpuiYüzeyDönüşümü` türüne çıkar.
2. GPUI paint ve SVG kaydedicinin aynı dönüşümü kullanmasını sağla.
3. Aşağıdaki komutların vektör kaydını uygula:
   - arka plan;
   - düz/kesikli çizgi;
   - line/spline/stepped path parçaları;
   - alan ve gradyan alan;
   - sabit/değişken daire grupları;
   - dikdörtgen ve bağımsız köşe yarıçaplı dikdörtgen;
   - yatay, çok satırlı ve döndürülmüş metin;
   - clip path;
   - opacity ve renk;
   - ana/etkileşim katman grupları.
4. Gradyan kimliklerini belge içinde deterministik ve çakışmasız üret.
5. XML metin/öznitelik kaçışını ve sonlu sayı doğrulamasını uygula.
6. Native dosya yazma ve tüm platformlarda string/byte API'si ekle.
7. SVG kaydı sırasında GPUI scene revision'ını veya grafik durumunu değiştirme.
8. Kaydın yeni geometri hesaplaması yapmamasını; mevcut retained sahneyi
   kullanmasını sağla.
9. SVG kaydediciyi GPUI frame/paint çağrı zincirine bağlama; yalnız açık
   dışa aktarım API'sinden çağır.
10. Feature açık fakat dışa aktarım çağrılmamış durumda SVG allocation ve
    serializer çağrı sayısının sıfır olduğunu sayaçlı test/benchmark ile
    doğrula.

### Test matrisi

- Resize: path ve eksen metinleri
- Area Fill: üç alan ve gradyan/clip
- Multi Bars: yuvarlatılmış rect, değer etiketi, renk
- Scatter: toplu daire path'i
- Timezones: çok satırlı ve rollover metni
- Y ekseni: döndürülmüş metin
- Cursor Snap: isteğe bağlı etkileşim katmanı
- Retina/aspect-fit: kaynak boyuttan farklı hedef yüzey
- Determinizm: aynı revision iki kez byte-eşit SVG
- Durum güvenliği: kayıttan önce/sonra zoom, cursor ve history eşit
- Sıfır normal-yol maliyeti: 1.000 paint çağrısında SVG serializer çağrısı
  ve SVG buffer allocation sayısı `0`

### Kabul

- çıktı raster `<image>` içermez;
- path, rect, circle, text, gradient ve clip öğeleri düzenlenebilir vektördür;
- GPUI'de görülen grafik geometrisi hedef SVG pikselinde eşleşir;
- feature kapalıyken dosya boyutu ve bağımlılık yüzeyi SVG kodunu içermez.
- feature açıkken fakat kayıt istenmemişken GPUI paint akışı feature kapalı
  yapı ile aynı çağrı ve allocation davranışını korur.

## 9. Faz 4 — Retained katman ve cache mimarisi

### Hedef katmanlar

1. arka plan;
2. eksen/grid;
3. seri geometrileri;
4. annotation/hook;
5. cursor/seçim/hover;
6. tooltip/lejant UI.

### İşler

- ana ve etkileşim yüzeyine ayrı kalıcı revision ver;
- her retained komutun geometrisini oluşturulurken bir kez kimliklendir;
- cache anahtarını komut geometri kimliği, yüzey sınırı ve fiziksel DPI
  ölçeğiyle kur;
- komutların derin `Vec` eşitliğiyle cache koruma kararını kaldır; sahne
  geçişini sabit boyutlu kimliklerle karşılaştır;
- `setData`, `setSeries`, resize ve zoom sonrası yalnız kimliği değişen
  komut/path yuvalarını kirlet; yalnız cursor değişikliğini ayrı retained
  etkileşim yüzeyine yönlendir;
- geometri `Vec` kapasitelerini tekrar kullan;
- görünür X dilimini ve dış komşuları path üretiminden önce belirle;
- statik hover'da ana sahne ve path revision'ının değişmediğini test et;
- GPUI paint ve isteğe bağlı SVG recorder aynı revision'lı retained katmanları
  birbirinden bağımsız çağrılarda tüketir.

### Kabul

- pointer hareketi ana path üretmez;
- yalnız bir seri değiştiğinde diğer seri path'leri korunur;
- yalnız renk değiştiğinde geometri/tessellation korunur;
- resize gerekli katmanları yeniler fakat veri deposunu kopyalamaz.

## 10. Faz 5 — Tek Rust kart kayıt defteri

### İşler

- kart kimliği, başlık, kaynak, açıklama, grup, varyant ve fabrika fonksiyonunu
  tek `KartTanımı` kayıt defterine taşı;
- desktop `KartKimliği` match tekrarlarını kaldır;
- JavaScript kart registry'sini kaldırılabilir hale getir;
- aynı kaynak sayfasındaki ilişkili yüzeyleri tek grup olarak tanımla;
- `No Data` gibi yalnız girdiyle değişen örnekleri tek kart + seçenek modeli
  olarak koru;
- zoom/touch özelliklerini ortak grafik seçeneği yap; tekrar zoom kartlarını
  katalogdan çıkar;
- açıklama ve kod örneği GPUI açılır panellerini varsayılan kapalı tut.

### Kabul

- native ve web aynı kart sayısını, sırasını ve grup ilişkisini raporlar;
- kart meta verisi tek Rust dosyasında bulunur;
- kart seçimi grafik entity'sini gereksiz yeniden kurmaz.

## 11. Faz 6 — Ortak GPUI katalog uygulaması

### Yapı

```text
uygulamalar/katalog/
├─ src/lib.rs              # Ortak ChartListesi ve kart registry kullanımı
├─ src/native.rs           # Native'e özgü pencere/ikon işlemleri
└─ src/web.rs              # wasm_bindgen başlangıç ve WebGPU hata köprüsü
```

### İşler

- mevcut masaüstü `ChartListesi`ni ortak lib'e taşı;
- native olmayan filesystem/ikon/resource işlemlerini ayır;
- ortak bileşenlerin web feature'ını hedef bazlı seç;
- tek `GpuiGrafik` entity'sini kart yaşam döngüsünde koru;
- açıklama, kaynak kodu, kontroller, legend ve metrik alanlarını GPUI
  bileşenleri yap;
- kart geçişinde görev/zamanlayıcı iptalini garanti et.

### Kabul

- katalog lib'i native ve wasm32 hedeflerinde derlenir;
- UI içinde SVG veya raw DOM düğümü yoktur;
- aynı kart state akışı iki platformda paylaşılır.

## 12. Faz 7 — Native uygulama

### İşler

- macOS/Linux/Windows girişlerini ince wrapper'lara dönüştür;
- title bar, tema ve ikon entegrasyonunu koru;
- Linux WGPU adapter/backend bilgisini tanılama paneline ekle;
- yazılım renderer fallback'ini görünür uyarı olarak göster;
- timer ve stream akışlarını GPUI executor/frame callback'leriyle birleştir;
- native resize ve scale-factor değişimini retained yüzeye uygula.

### Kabul

- üç native hedef derlenir;
- Linux gerçek GPU ve yazılım fallback koşulları ayırt edilir;
- hover/stream sırasında frame pacing ölçülür;
- mevcut katalog kartlarının tamamı açılır.

## 13. Faz 8 — GPUI Web/WASM

### İşler

- ortak katalogdan `cdylib` web girişini üret;
- `gpui_web` tek iş parçacıklı stable yapılandırmayı kullan;
- `wasm-bindgen`/Trunk veya eşdeğer minimal bootstrap oluştur;
- eski 11K+ satır inline JavaScript'i işlevsel kaynak olarak kullanma;
- WebGPU başlatma, resize, DPR, clipboard/download ve hata UI'sini ekle;
- SVG dışa aktarımı browser download'u için byte/string API'sine bağla;
- web deployment base path ve deep-link davranışını doğrula.

### Kabul

- webde grafik `<canvas>`/WebGPU üzerinde GPUI tarafından çizilir;
- DOM'da seri/path başına SVG düğümü bulunmaz;
- kart davranışı JavaScript'te tekrar uygulanmaz;
- GitHub Pages doğrudan GPUI Web artefaktını yayınlar.

## 14. Faz 9 — Etkileşim, sync ve erişilebilirlik

### İşler

- wheel varsayılan XY;
- Shift+wheel yalnız X;
- Ctrl+wheel yalnız Y;
- seçim, drag, space+pan, çift tıklama ve history;
- pointer capture ve yüzey dışı mouse-up;
- tek parmak pan, iki parmak pinch;
- sync-cursor grup yayın/abonelik;
- ilişkili çoklu yüzey setScale/setSeries davranışları;
- keyboard focus ve temel accessibility node'ları;
- tooltip/lejant güncellemesinin ana sahneden ayrılması.

### Kabul

- ortak platform-bağımsız olay senaryoları native ve webde geçer;
- touch browser doğal scroll'u yanlışlıkla engellemez veya gesture kaybetmez;
- sync grupları dışındaki grafikler birbirini değiştirmez.

## 15. Faz 10 — Eski runtime ve not temizliği

### Silinecek/dönüştürülecek alanlar

- eski `wasm/src/lib.rs` SVG oturumu;
- eski `wasm/www/index.html` kart ve etkileşim uygulaması;
- generated `wasm/pkg`;
- kart başına SVG üreten CLI örneklerinin çoğu;
- `svg-image` PoC kartı; yerine GPUI SVG Export kartı;
- eski `UPLOT_TAM_UYUM_FAZ_PLANI.md`;
- eski `UPlot_TAM_PORT_FAZI.md`;
- geçici tamamlanma/faz notları;
- README'deki framework-bağımsız renderer iddiaları;
- CI'daki eski SVG/WASM feature matrisi.

Kaynak uyumunu kanıtlayan veri fixture'ları ve davranış senaryoları geçici not
değildir; GPUI testlerine taşınır ve korunur.

### Kabul

- `rg` ile eski SVG runtime, DOM patch veya ikinci kart registry bulunmaz;
- tek güncel faz planı bu belgedir;
- arşiv dalı eski uygulamayı gerektiğinde erişilebilir tutar.

## 16. Faz 11 — Belgeler, CI ve yayın

### İşler

- Türkçe ve İngilizce README'leri GPUI kullanımına göre yaz;
- API örneklerini `GpuiGrafik` ve `gpui-svg` üzerinden güncelle;
- native ve web build yönergelerini ayır;
- CI:
  - fmt;
  - clippy `-D warnings`;
  - unit/integration;
  - native GPUI check;
  - wasm32 GPUI Web check;
  - SVG export golden;
  - kart registry parity;
- nightly artefaktları yeni GPUI katalog binary/web paketinden üret;
- Pages'i GPUI Web paketine geçir.

### Kabul

- belgelerde eski SVG/DOM web hattı anlatılmaz;
- CI temiz checkout'ta kardeş GPUI bağımlılığını doğru konuma getirir;
- indirilebilir native ve web artefaktları açılır.

## 17. Faz 12 — Performans ve son doğrulama

### Temsilci kartlar

- Resize 100 ve 1.000 nokta
- Multi Bars
- Latency Heatmap 20K/35K
- Mass Spectrum 41.986
- Sine Stream 6×600
- Sparse büyük veri
- Sync Cursor grubu

### Ölçümler

- ilk çizim süresi;
- p50/p95/p99 frame süresi;
- pointer başına ana sahne revision sayısı;
- zoom başına geometri ve paint süresi;
- `setData` ve `setSeries` allocation sayısı;
- resident bellek;
- WASM indirme ve ilk etkileşime hazır olma süresi;
- SVG export süresi ve çıktı boyutu;
- SVG feature açık/kapalı normal paint frame süresi ve allocation farkı;
- Linux GPU adapter/backend bilgisi.

### Bütçeler

- statik hover: ana geometri yeniden üretimi `0`;
- pointer hareketi: kare başına en fazla `1` overlay güncellemesi;
- sürekli 60 Hz örnekleri: p95 frame `< 16,7 ms` hedefi;
- UI ana thread'inde görünür uzun takılma olmaması;
- SVG kaydı grafik state'ini değiştirmemesi;
- SVG istenmeyen normal çalışmada serializer çağrısı ve SVG allocation'ı `0`;
- native'in aynı veri yükünde mevcut arşiv sürümünden yavaşlamaması;
- GPUI Web yoğun kartların eski SVG/DOM sürümüne göre p95 frame süresinde
  ölçülebilir iyileşme göstermesi.

### Son komutlar

```sh
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo check --target wasm32-unknown-unknown --workspace
```

Ek olarak native smoke testleri, browser WebGPU/touch senaryoları, SVG golden
testleri ve git uzak dal eşliği doğrulanır.

## 18. Riskler ve geri dönüş

### GPUI/Web API kırılması

GPUI pre-1.0'dır. Path bağımlılığı güncel kardeş çalışma ağacını kullandığı
için API değişikliği CI'yı kırabilir. Geçiş boyunca kullanılan GPUI API yüzeyi
küçük tutulmalı ve uyumluluk kontrolü CI'da yapılmalıdır.

### WebGPU bulunmaması

GPUI Web için Canvas2D fallback yoktur. Desteklenmeyen tarayıcıda açık hata ve
sistem gereksinimi gösterilir. SVG dışa aktarım çalışma zamanı fallback'i
değildir.

### Genel GPUI sahnesini SVG'ye çevirme beklentisi

Bu özellik `GpuiGrafik` için kayıpsız vektör kaydıdır. Genel GPUI
uygulamasındaki arbitrary texture, video, shader veya üçüncü taraf elementleri
SVG'ye dönüştürmez. API ve belgeler bu sınırı açıkça belirtir.

### Geçiş sırasında davranış kaybı

Eski SVG/JS web runtime ancak aynı kartın GPUI Web kabul senaryosu geçtiğinde
kaldırılır. Başlangıç sürümü ayrıca
`codex/pre-gpui-primary-archive` dalında korunur.

## 19. Commit ve raporlama sözleşmesi

- Her faz veya bağımsız kabul kapısı tek amaçlı commit olur.
- Committen önce ilgili testler çalıştırılır.
- Commit doğrudan `main` dalına pushlanır.
- Faz geçişinde bu plandaki toplam yüzde kullanıcıya bildirilir.
- Başarısız kabul kapısı gizlenmez; kök neden düzeltilmeden sonraki faz
  tamamlanmış sayılmaz.
- `%100`, yalnız Bölüm 3'teki bütün tamamlanma koşulları sağlandığında
  bildirilir.
