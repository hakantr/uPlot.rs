# uPlot.rs — uPlot 1.6.32 Rust/GPUI Portu Faz Planı

Bu belge, `../cizelge` projesinde uygulanan kaynak kilidi, özellik envanteri,
kanıta dayalı uyum ve faz kapısı yaklaşımını bu repoya uyarlar. Davranışın
normatif kaynağı yalnız `../uPlot` deposudur; `../cizelge` yalnız süreç ve
altyapı örneğidir.

## 1. Hedef ve sabit kaynaklar

Hedef; uPlot'un küçük, hızlı ve bellek-verimli zaman serisi çizelgesi olma
özelliğini koruyan, yerli Rust çekirdeğine ve GPUI görünümüne sahip bir port
üretmektir. Port, JavaScript sözdizimini kopyalamak yerine aynı veri, ölçek,
çizim, etkileşim ve eklenti semantiğini Rust'a uygun tiplerle sunacaktır.

Hedef repo Apache-2.0 olarak yayımlanacaktır. uPlot'tan uyarlanan MIT kaynaklar
için özgün telif/lisans bildirimi `NOTICE` ve dağıtım içinde eksiksiz korunur;
örnek verileri, fontlar ve diğer varlıklar da lisans envanterine girer.

Kod tabanı Rust 2024 edition kullanır; asgari ve sabit geliştirme araç zinciri
Rust 1.95.0'dır. Tasarım daha eski Rust sürümlerinin sınırlamaları gözetilerek
geriye çekilmez.

| Kaynak | Kilit | Yerel yol | Rol |
|---|---|---|---|
| uPlot | `1.6.32`, commit `0e5812c504430f5c804e0f993376d8999b26cc34` | `../uPlot` | Tek normatif davranış ve algoritma kaynağı |
| uPlot tip yüzeyi | aynı commit, `dist/uPlot.d.ts` | `../uPlot/dist/uPlot.d.ts` | Kamu API envanteri ve varsayılanlar |
| uPlot demoları | aynı commit, 74 HTML + 2 JS giriş | `../uPlot/demos` | Davranış, görsel ve performans senaryoları |
| Cizelge | çalışma ağacındaki mevcut sürüm | `../cizelge` | Yalnız port süreci, test katmanları ve CI örneği |
| GPUI | kilitsiz; mevcut canlı çalışma ağacı | `../gpui` | Pencere, olay, metin ve çizim entegrasyonu |
| GPUI bileşen kütüphanesi | kilitsiz; mevcut canlı çalışma ağacı | `../gpui_kutuphanesi` | Başlık çubuğu ve ortak arayüz bileşenleri |

Normatif uPlot kaynak commit'i yükseltilmedikçe uyum hedefi hareket ettirilmez.
Bu kaynak yükseltmesi ayrı bir değişiklik ve envanter fark raporu gerektirir.
GPUI ile GPUI bileşen kütüphanesi ise sürekli geliştirilen canlı geliştirme
bağımlılıklarıdır; commit pinlenmez ve her derlemede mevcut halleri kullanılır.

## 2. Başlangıç envanteri ve kapsam

Kaynak görüntüsü yaklaşık 8.109 satırlık davranış/API yüzeyinden oluşur:
6.884 satır JavaScript çekirdeği ve 1.225 satır TypeScript bildirimi. Ana
alanlar şunlardır:

- sütunlu hizalı veri ve `mode: 1/2` veri düzenleri;
- seçenek varsayılanları ve çalışma zamanı `setData`/`setScale`/`setSize`
  işlemleri;
- doğrusal, zaman, logaritmik, asinh ve bağımlı ölçekler;
- eksen, çentik, ızgara, başlık, gösterge ve seçim kutusu;
- çizgi, alan, nokta, basamak, spline, monoton kübik, Catmull–Rom ve sütun
  path üreticileri;
- boşluklar, bantlar, kırpma, yön/direction ve piksel hizalama;
- imleç, odak, sürükleyerek yakınlaştırma, seri aç/kapatma ve çizelgeler arası
  senkronizasyon;
- hook yaşam döngüsü, plug-in'ler ve özel path/eksen/değer callback'leri;
- zaman dilimi/DST ve tarih biçimlendirme;
- yüksek nokta sayısında görünür aralık taraması, piksel kovası küçültmesi ve
  akış güncellemeleri.

### Kapsam içi

`dist/uPlot.d.ts` içindeki kamu API'si, `src/` altındaki varsayılan davranış,
74 HTML demo ve bunların kullandığı yerleşik path/etkileşim özellikleri kapsam
içidir. DOM'a özgü bir özellik GPUI'de aynı nesne biçimiyle değil, gözlenebilir
sonucu eşdeğer yerli API ile sunulur.

### Kapsam dışı

Kaynak uPlot'un açıkça “non-feature” saydığı veri ayrıştırma/istatistik,
yerleşik animasyon ve yerleşik pan davranışı porta eklenmez. Demo içindeki
üçüncü taraf yardımcı kitaplıkların genel amaçlı API'leri port kapsamı değildir;
yalnız uPlot davranışını kanıtlamak için gereken fixture/veri dönüşümü test
aracında tutulabilir. React/Vue/Svelte sarmalayıcıları bu portun çekirdeğine
dahil değildir.

## 3. Tam uyum tanımı

Bir özellik ancak aşağıdaki kanıtların tümü varsa tamamlanmış sayılır:

1. **API:** `uPlot.d.ts` sembolü veya iç varsayılanı Rust karşılığına ve kaynak
   konumuna eşlenmiştir.
2. **Sayısal:** ölçek, aralık, tarih, indeks ve koordinat sonuçları sabit
   fixture'larda kaynakla tolerans içinde aynıdır.
3. **Görsel:** aynı veri/seçenek/profil için referans ve Rust render'ı kabul
   eşiğini geçer.
4. **Davranış:** ilgili hook sırası, etkileşim, güncelleme ve senkronizasyon
   senaryosu doğrulanır.
5. **Dayanıklılık:** test, lint, lisans ve ilgili performans bütçesi yeşildir;
   çalışma zamanı kodunda kontrolsüz panik yolu yoktur.

Durumlar: `yok`, `kısmi`, `uygulandı_kanıt_bekliyor`, `tam_kanıtlı`,
`kapsam_dışı`. `kısmi` tamamlanmış kabul edilmez.

## 4. Hedef mimari

```text
Rust seçenekleri + hizalı veri
            |
            v
 doğrulama / varsayılanlar / yaşam döngüsü
            |
            v
 ölçekler -> görünür indeksler -> yerleşim
            |
            v
 path üreticileri -> sahne/çizim komutları
            |                    |
            |                    +-> kayıt yüzeyi (golden test)
            |                    +-> piksel yüzeyi (görsel test)
            |                    +-> GPUI yüzeyi (uygulama)
            v
 imleç / seçim / hook / sync / plug-in
```

Önerilen modül sınırları:

```text
src/
  veri.rs, secenek.rs, hata.rs
  olcek.rs, olcek/{aralik,dogrusal,log,asinh,zaman}.rs
  yerlesim/{eksen,izgara,gosterge}.rs
  yol/{dogrusal,basamak,spline,monoton,catmull_rom,sutun,nokta,bant}.rs
  cizim.rs, cizim/{komut,yuzey,kayit,piksel,gpui}.rs
  etkilesim/{imlec,secim,odak,senkron}.rs
  kanca.rs, eklenti.rs, zaman.rs, grafik.rs, lib.rs
uyum/
  kaynak_kilidi.toml, api_matrisi.json, demo_manifesti.json
  senaryolar/, sapmalar/
testler/
  birim/, altin/, gorsel/, performans/
tools/uyum/
```

Çekirdek, GPUI olmadan derlenebilir olmalıdır. GPUI ve raster çıktı ayrı
özellik bayrakları olur. Dinamik JavaScript callback'leri Rust'ta yaşam süresi
ve iş parçacığı sınırları açık trait/callback türleriyle modellenir.

## 5. Fazlar

### Faz 0 — Kaynak kilidi, lisans ve yürütülebilir uyum envanteri

Amaç, “tamamlandı” kararını ölçülebilir hale getirmektir.

- Cargo çalışma alanı, `LICENSE`/`NOTICE`, README, lint ve CI iskeletini kur.
- MIT kaynak uPlot atfını, taşınan algoritmaları ve demo varlık lisanslarını
  kaydet; bağımlılık lisans kapısını ekle.
- `uPlot.d.ts`, `src/` dışa aktarımları ve demo indeksinden tekrar üretilebilir
  `api_matrisi.json` ile `demo_manifesti.json` oluştur.
- Kaynak JS referans çalıştırıcısını kur: JSON fixture alıp sayısal sonuç,
  hook izi ve ekran görüntüsü üretsin.
- Sabit render profili belirle: pencere boyutu, DPR, font, saat dilimi, locale,
  renk alanı ve toleranslar.
- Çekirdek benchmark fixture'larını sabitle: ilk render, yeniden ölçekleme,
  imleç hareketi, `setData`, bellek ve akış güncellemesi.

**Kapı:** Kaynak hash'i doğrulanır; tüm kamu sembolleri ve 76 demo girişi
manifesttedir; referans çalıştırıcı en az bir çizgi demosunda sayısal, görsel
ve hook izi üretir.

### Faz 1 — Güvenli çekirdek tipleri, veri sözleşmesi ve çizim yüzeyi

- Hizalı sütun verisini ödünç alınabilen `f64`/nullable temsille modelle;
  eşit uzunluk, sıralama ve boş veri hatalarını tipli döndür.
- Seçenek varsayılanları, renk/stil, ölçü, bbox ve yön tiplerini kur.
- Kayıt, piksel ve GPUI uygulamalarını besleyen ortak çizim komutlarını kur:
  path, clip, stroke, fill, dash, alpha, dönüşüm ve metin.
- Deterministik metin ölçümü, DPR/piksel hizalama ve iç içe kırpma ekle.
- Hook kayıt/çalıştırma sırasının ve tanı/hata kanalının temelini kur.

**Kapı:** Headless çekirdek GPUI'siz derlenir; çizim komutu golden testleri
deterministiktir; boş/geçersiz veri panik üretmez; temel yüzeyler aynı sahne
komutlarını tüketir.

### Faz 2 — Ölçek, aralık, tarih ve koordinat matematiği

- Doğrusal, zaman, log2/log10, asinh, bağımlı ve özel dağılım davranışlarını
  taşı.
- `rangeNum`, soft/hard min-max, padding, clamp, auto-range ve yön tersleme
  kurallarını eşle.
- Değer↔konum dönüşümleri, görünür indeks araması ve `mode: 2` facet veri
  seçimini uygula.
- Tarih çentikleri, biçim şablonları, timezone/DST ve `DateZoned` eşdeğerini
  bağımsız testlenebilir hale getir.

**Kapı:** Kaynak çalıştırıcıya karşı sınır/değer/konum ve tarih fixture'ları
yeşildir; NaN, sonsuz, tek nokta, sabit seri ve ters aralık durumları güvenli
sonuç verir.

### Faz 3 — Yerleşim, eksen, ızgara, başlık ve gösterge

- Çoklu ve bağımlı ölçek/eksen yerleşimini, dört kenarı ve yatay/dikey yönü
  taşı.
- Çentik artışı/ayrımı, filtre, değer formatter'ı, döndürme, otomatik eksen
  boyutu, padding ve döngüsel ölçümü eşle.
- Izgara/tick/border çizimini; başlık, gösterge marker/değerleri ve seri
  görünürlük durumunu ekle.
- Boyut değişikliğinde yerleşim ve hook sırasını doğrula.

**Kapı:** `axis-*`, `scales-*`, `timezones-*`, `months-*`, `legend` ve
`resize` demo kümeleri sayısal/görsel kapıyı geçer.

### Faz 4 — Path üreticileri, boşluklar ve bantlar

- Doğrusal çizgi ve görünür piksel başına min/max küçültmeyi önce taşı;
  bu yol performans tabanıdır.
- Nokta, basamak, spline, monoton kübik, Catmull–Rom ve sütun/OHLC path
  üreticilerini ekle.
- `spanGaps`, boşluk kırpması, fill-to, high/low band, yön, `dir`, `ori`,
  özel radius/facet ve path cache geçersizleştirmesini eşle.
- Özel path builder trait'ini kamu API'sine aç.

**Kapı:** Her yerleşik path için komut golden'ı ve görsel fixture yeşildir;
`line-paths`, `missing-data`, `path-gap-clip`, `bars-*`, `candlestick-*`,
`box-whisker` ve `high-low-bands` demo aileleri kapanır.

### Faz 5 — Grafik yaşam döngüsü ve mutasyon API'si

- Grafik kurulum/sökümünü ve `init`, `ready`, `draw*`, `set*`, `destroy`
  hook sırasını taşı.
- `setData`, `setScale`, `setSize`, seri ekle/sil, göster/gizle, odak ve
  seçim durumlarını ekle.
- Batch/defer çizim ve cache geçersizleştirmesini, hatada mevcut durumu
  koruyan işlem yaklaşımıyla kur.
- Veri ve seçenek sahipliği için kopyasız sıcak yolu doğrula.

**Kapı:** Güncelleme ve streaming demoları davranış izinde eşdeğerdir;
başarısız mutasyon grafiği yarım durumda bırakmaz; kaynakla aynı yeniden çizim
ve hook sırası gözlenir.

### Faz 6 — İmleç, seçim, yakınlaştırma, odak ve senkronizasyon

- En yakın indeks/nokta, hover bias, focus prox, imleç noktaları ve gösterge
  canlı değerlerini taşı.
- Sürükleme seçimi, x/y yakınlaştırma, seçim kutusu resize ve seri/eksen
  kontrollerini ekle.
- Çoklu grafik sync pub/sub, filtre ve scale/series eşleme davranışını kur;
  abonelik ömrünü sızıntısız yönet.
- Fare ve dokunma girdisini GPUI olaylarına eşle; DOM'a özel bind noktalarını
  yerli giriş adaptörüyle sun.

**Kapı:** `cursor-*`, `focus-*`, `zoom-*`, `sync-*`, `select-*` ve touch
demolarının kayıtlı etkileşim senaryoları hook izi ve son görüntü açısından
eşdeğerdir.

### Faz 7 — Plug-in ve özelleştirme yüzeyi

- `Options`, `Series`, `Scale`, `Axis`, `Cursor`, `Legend`, `Band`, `Hooks`
  ve `Plugin` yüzeylerindeki tüm callback/override noktalarını matrise göre
  tamamla.
- Çizim hook'ları, özel ölçek/aralık, eksen değerleri, imleç bind/refiner,
  özel path ve overlay kullanımını güvenli Rust trait'leriyle aç.
- Kaynak demolarındaki zoom wheel/touch, annotation, tooltip, smoothing,
  stacked/grouped helper ve ranger plug-in örneklerini taşı.

**Kapı:** API matrisinde callback/plug-in yollarında `yok` veya `kısmi`
kalmaz; seçilmiş üçüncü taraf plug-in demoları çekirdeği çatallamadan çalışır.

### Faz 8 — Performans, bellek ve uzun süreli dayanıklılık

- 10M nokta, 600 seri, 3.600 nokta/60 fps akış ve sık imleç hareketi
  senaryolarını Rust benchmarklarına çevir.
- Tahsis, kopya, path cache, görünür aralık araması, küçültme ve metin ölçüm
  sıcak yollarını profil et.
- Kaynak uPlot ölçümlerini aynı donanım/tarayıcıyla yeniden baseline al;
  GPUI farkı nedeniyle mutlak README sayılarını doğrudan kapı yapma.
- Port için ayrı bütçeler koy: kaynak baseline'a göre ilk render, güncelleme,
  imleç gecikmesi, tepe/kalıcı bellek ve çıktı komutu sayısı. Regresyon eşiği
  Faz 0 ölçümünden sonra sayısallaştırılır ve gevşetilmesi ayrı karar ister.
- Saatler süren streaming/sync yaşam döngüsünde sızıntı ve durum bozulmasını
  denetle.

**Kapı:** Tüm bütçeler CI'da tekrarlanabilir ölçülür; temel doğrusal path'in
asemptotik davranışı ve piksel-kovası küçültmesi kaynakla eşdeğerdir; performans
raporunda açıklanmamış gerileme yoktur.

### Faz 9 — Demo kapanışı, dokümantasyon ve sürüm adayı

- 76 demo girişinin her birini `tam_kanıtlı` veya gerekçeli `kapsam_dışı`
  duruma getir; kapsam dışı girdiler başarı oranını yükseltmez.
- Rust API rehberi, JS→Rust eşleme rehberi, örnek galerisi ve migration notu
  yaz.
- Açık/koyu profil, farklı DPR, saat dilimi ve locale matrisiyle gecelik tam
  görsel/etkileşim koşusunu çalıştır.
- Public API semver incelemesi, dokümantasyon testi, WASM/headless derleme,
  lisans/NOTICE ve paket içeriği denetimini tamamla.

**Kapı:** Kapsam içi API satırlarının ve demoların tamamı kanıtlıdır; açık
sapma feragati, kararsız test, panik yolu, lisans belirsizliği veya ölçülmemiş
performans gerilemesi yoktur.

## 6. CI ve değişiklik işleyişi

Her özellik dikey bir dilim olarak teslim edilir: Rust API + uygulama + kaynak
eşleme + birim/golden test + gerekiyorsa görsel/etkileşim kanıtı + doküman.
Yalnız uygulama kodu içeren özellik tamamlanmış sayılmaz.

### Her değişiklikte

- `cargo fmt --check`, clippy (uyarılar hata), birim ve golden testler;
- kaynak kilidi, üretilmiş manifest ve API matrisi fark denetimi;
- değişen yeteneğin sayısal ve görsel fixture'ları;
- çalışma zamanı panik yolu ve bağımlılık lisans denetimi;
- ilgili mikrobenchmarkta regresyon uyarısı.

### Gecelik

- tüm demo shard'ları için sabit profilde kaynak/Rust görsel farkı;
- kaydedilmiş etkileşim ve hook izi senaryoları;
- tüm performans/bellek profilleri ve uzun süreli streaming testi;
- tek HTML uyum raporu: referans, gerçek, fark, metrik ve durum.

## 7. Ana riskler ve erken deneyler

| Risk | Erken karar/deney |
|---|---|
| GPUI path/clip/metin semantiği Canvas 2D ile farklı | Faz 1'de çizgi + döndürülmüş eksen etiketi + gap clip spike'ı |
| Dinamik JS callback yüzeyi Rust yaşam süreleriyle çatışır | Faz 1'de hook, Faz 2'de scale callback prototipi |
| DST/timezone sonuçları platforma göre değişir | Faz 0'da sabit tz/locale fixture, Faz 2'de platformlar arası matris |
| Görsel eşitlik font ve DPR yüzünden kararsızlaşır | Font dosyası/hash'i ve DPR'yi kaynak kilidine al |
| Rust portu doğru ama uPlot'tan yavaş olabilir | İlk doğrusal path benchmarkı Faz 4 sonuna bırakılmaz; Faz 0'dan izlenir |
| Demo sayısı API kapsamını olduğundan yüksek gösterir | Demo manifesti ve `uPlot.d.ts` API matrisi ayrı kapanır |

## 8. Model ve düşünme seviyesi kararı

Ana uygulama modeli: **`gpt-5.6-sol`**.

Ana düşünme seviyesi: **`max`**. Bu port; iki dil arasında davranış çıkarımı,
sayısal algoritma eşleme, yaşam süresi/sahiplik tasarımı, performans profilleme
ve görsel/etkileşim doğrulamasını birlikte gerektiren uzun ufuklu bir iştir.
Mevcut oturumda sunulan modeller içinde `sol` frontier kodlama modelidir ve
`max`, desteklenen en yüksek standart düşünme seviyesidir.

Kullanım politikası:

- Faz 0 mimari kararları, Faz 2 ölçek/tarih matematiği, Faz 4 path eşdeğerliği,
  Faz 6 etkileşim/sync ve Faz 8 performans çalışmaları: `gpt-5.6-sol`, `max`.
- Mekanik fixture/demo taşıma ve iyi tanımlı dokümantasyon işleri:
  `gpt-5.6-terra`, `high` veya `xhigh` maliyet/latans için yeterli olabilir.
- Faz kapısı incelemesi ve kaynakla son fark denetimi yeniden
  `gpt-5.6-sol`, `max` ile yapılır.

En yüksek seviyeyi her küçük değişiklikte kullanmak zorunlu değildir; fakat
tek bir model/seviye seçilecekse doğruluk ve uzun görev sürekliliği nedeniyle
karar `gpt-5.6-sol` + `max`tır.

## 9. Uygulama sırası özeti

```text
F0 envanter/kanıt
  -> F1 çekirdek/yüzey
    -> F2 ölçek/zaman
      -> F3 yerleşim
        -> F4 path
          -> F5 yaşam döngüsü
            -> F6 etkileşim/sync
              -> F7 plug-in/API tamlığı
                -> F8 performans
                  -> F9 kapanış
```

Fazlar sıralı kapatılır; ancak performans ölçümü Faz 0'da, görsel karşılaştırma
Faz 1'de ve API matrisi denetimi Faz 0'da başlayıp tüm çalışma boyunca sürekli
kapı olarak kalır.
