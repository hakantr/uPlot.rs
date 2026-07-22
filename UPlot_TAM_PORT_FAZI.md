# uPlot.rs tam port faz planı

Hedef, normatif kaynak olarak kilitlenen uPlot `1.6.32` / `0e5812c` çalışma
ağacındaki çekirdek ve genel API'nin tamamını; 73 HTML demosunun çizim, veri,
ölçek, etkileşim ve eklenti davranışlarıyla birlikte Rust çekirdeğine taşımaktır.
Masaüstü ve WASM katalogları ürün
API'si değildir; aynı çekirdeği görsel ve davranışsal olarak doğrulayan iki
ayrı yüzeydir.

Kapsam yalnız demolarla belirlenmez. `src/**/*.js` uygulaması ve
`dist/uPlot.d.ts` genel API'si birincil yetenek envanteridir; demoda karşılığı
olmayan kaynak yeteneği de port edilir ve sentetik bir uyum testi alır.

Bir demo ancak kaynak hash'i, özgün veri/veri üreteci, Rust kart tanımı, sayısal test, masaüstü/WASM
görünümü ve gerekiyorsa etkileşim kanıtı tamamlandığında `tam_kanıtlı` sayılır.
Kart kodunda davranış tekrarlanmaz: yeni bir ihtiyaç önce çekirdek seçenek ve
çizim modeline eklenir, yüzeyler yalnız olay iletir ve sahne komutlarını boyar.

## Değiştirilemez kurallar

- Rust 2024 ve en düşük Rust `1.95` korunur; `mod.rs` kullanılmaz.
- Üretim kodunda `panic!`, `unwrap`, `expect`, kontrolsüz dizinleme ve eşdeğer
  panik yolları kullanılmaz; hatalar `UplotHatası` ile raporlanır.
- Normatif davranış yalnız kilitli `../uPlot` kaynağından port edilir. uPlot.rs
  uzantıları resmî port davranışından ayrı kaydedilir.
- GPUI katalog uygulaması dağıtılan çekirdeğin içine sızmaz. Geliştirici yalnız
  kart verisini, görünüm seçeneklerini ve açık/kapalı yetenekleri tanımlar.
- Her faz önceki fazların test, uyum ve görsel kanıt kapılarını yeniden çalıştırır.
- Her kart [ortak kart davranışları](ORTAK_KART_DAVRANISLARI.md) sözleşmesindeki
  bütün maddeler için manifest kararı taşır. Eksik karar veya gerekçesiz istisna
  uyum denetimini ve CI'ı kırar.
- Kaynakta etkileşim varsa port kartında da bulunur; statik benzerlik tek başına
  kabul sayılmaz.
- Demo içindeki sabit diziler ve veri varlıkları aynen kullanılır. Rastgele veri
  üreteçlerinde aynı algoritma ve parametreler korunur; görsel karşılaştırmada
  referans ve Rust aynı açık tohumla çalıştırılır.
- Ortak sahne modeli GPU kullanımını engellemez: `gpui` yüzeyi komutları
  GPUI'nin GPU destekli boyama hattına verir. Büyük veri performansında uPlot'un
  piksel kovası decimation, yol önbelleği ve toplu boyama davranışları ayrıca
  kaynak eşdeğerliği kapsamındadır.

## Faz 0 — Envanter ve ortak sözleşme

Kaynak JS dosyaları, genel API sembolleri ve 73 demo için makine-okunur
envanter; kaynak/veri hash'i, sahip fazı, bağımlılıkları ve kanıt durumu tutulur.
`Resize` kartıyla kurulan hizalı veri, dinamik eksen,
çizim alanı kırpması, seçim, tekerlek, dokunma, taşıma ve görünüm geçmişi bu
fazın başlangıç tabanıdır.

Kabul: envanter kaynak ağacıyla bire bir eşleşir; eksik, fazla veya hash'i
değişmiş demo uyum denetimini kırar.

## Faz 1 — Doğrusal yollar, alanlar ve veri boşlukları

Alan dolgusu ve sıfır tabanı, seri bantları, null boşlukları, span/clip
davranışları, nokta görünürlüğü, doğrusal/stepped/spline/bar path üreticileri,
stack ve sparkline temelleri uygulanır. İlk teslim `area-fill.html` kartıdır.

Kabul: faz envanterindeki her kart ortak path/dolgu API'sini kullanır; SVG ve
GPUI aynı sahne semantiğini üretir.

## Faz 2 — Ölçekler, eksenler ve zaman

Log2/log10, arcsinh ve özel dağılımlar; ters/yönü değiştirilmiş ölçekler;
bağımlı ölçekler; yumuşak/sabit aralıklar; eksen konumu, ölçümü ve otomatik
boyutu; UTC/saat dilimi/DST ve değişken ay genişlikleri taşınır.

## Faz 3 — Özel çizim ve seri eklentileri

Gruplu/yığılmış/floating bar, candlestick/OHLC, box-whisker, scatter/bubble,
heatmap, timeline/discrete, annotations, datum, trendline ve wind renderer
eklentileri çekirdek çizim protokolü üzerinden uygulanır.

## Faz 4 — Cursor, tooltip ve yakınlaştırma eklentileri

Cursor bind/snap/focus, nearest-non-null, tooltip yerleşimi, axis indicator,
zoom varyasyonları, fetch zoom ve ranger/grip bileşimleri taşınır. Var olan
wheel/touch portları kendi kaynak kartlarında ayrıca kanıtlanır.

## Faz 5 — Çoklu grafik, senkronizasyon ve canlı veri

Seri ekleme/silme, veri akışı, sine stream, cursor/scroll/y-zero senkronu ve
resize sırasında cursor/seçim korunması uygulanır. Katalog, birden fazla canlı
kartı durum çakışması olmadan çalıştırır.

## Faz 6 — Tam katalog ve kanıt kapısı

Masaüstü ve WASM katalogları `uyum/demo_envanteri.json` üzerinden üretilen aynı
sıra/kategori bilgisini kullanır. Her kart kaynak bağlantısı, varsayılan kapalı
Rust tanım kutusu, etkileşim açıklaması ve kanıt durumunu gösterir.

Kabul: 73/73 demo `tam_kanıtlı`; workspace testleri, tüm feature'larla Clippy,
WASM release derlemesi ve görsel/davranış regresyonları yeşildir.
