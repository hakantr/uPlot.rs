# GPUI geçişi son doğrulama kaydı

**Tarih:** 2026-07-26  
**Normatif uPlot kaynağı:** `0e5812c504430f5c804e0f993376d8999b26cc34`  
**uPlot.rs doğrulanan tabanı:** `1502da391f03d7d9dfea15adc422a700cd75d614`

Bu kayıt, `GPUI_GECIS_FAZ_PLANI.md` içindeki on iki fazın tamamlanma
kanıtlarını tek yerde toplar. Kütüphanenin tek interaktif renderer'ı GPUI'dir.
Native ve web uygulamaları aynı `GpuiGrafik` bileşenini ve aynı yayınlanmayan
kart kataloğunu kullanır.

## Son mimari

- `gpui`, kök paketin isteğe bağlı olmayan çalışma zamanı bağımlılığıdır.
- Native giriş `uygulamalar/masaustu`, web girişi `uygulamalar/web` altındadır.
- İki giriş de `uygulamalar/katalog::ChartListesi` entity'sini açar.
- Eski `examples/`, `wasm/`, `src/svg.rs` ve `[[example]]` çalışma yolları
  kaldırılmıştır.
- Retained grafik komutları normal karelerde doğrudan GPUI
  `paint_path`/`paint_quad` hattına verilir.
- SVG ikinci bir interaktif renderer değildir. `gpui-svg` açıldığında
  `GpuiGrafik::svg_kaydı` ile yalnız açık kullanıcı/API isteğinde retained
  sahneden gerçek vektör kayıt üretilir.
- Normal GPUI paint yolunda SVG kayıt bayrağı, serializer çağrısı, `String`
  üretimi veya SVG tahsisatı bulunmaz.

## Faz ve commit kanıtları

| Faz | Sonuç | Commit |
|---|---|---|
| 0–1 | Ayrıntılı plan ve isteğe bağlı vektör kayıt sözleşmesi | `3806d88`, `6762411` |
| 2 | GPUI Web başlangıç uygulaması | `c7b8d85` |
| 3 | Web touch pan/pinch ve ortak etkileşim akışı | `17a6bb1` |
| 4 | Kök API'nin GPUI-first hale gelmesi | `dcc9dae` |
| 5 | GPUI SVG vektör kayıt kabul testleri | `cc8631e` |
| 6 | Sürümlü retained path önbelleği | `caa5438` |
| 7 | Native/web ortak GPUI kataloğu | `b6206d0` |
| 8 | Eski web gösteriminin GPUI Web ile değiştirilmesi | `7a133d2` |
| 9 | Yüzey sınırı dışı pointer/touch etkileşimleri | `b52dc0c` |
| 10 | Eski SVG runtime ve CLI örneklerinin kaldırılması | `a31df4a` |
| 11 | GPUI-only mimariyi koruyan CI/uyum kapıları | `1502da3` |
| 12 | Tam feature, test, lint, release ve canlı web doğrulaması | bu kayıt |

Kardeş `../gpui` deposunda GPUI Web için kullanılan tamamlayıcı commitler:
`bcde5c9`, `cb5a82f`, `774be43`.

## 2026-07-26 son kabul çalıştırması

Başarıyla tamamlanan komutlar:

```text
cargo fmt --all -- --check
cargo check -p uplot-rs --lib --no-default-features
cargo check -p uplot-rs --lib --no-default-features --features gpui-svg
cargo check -p uplot-rs --lib --all-features
cargo check -p uplot-rs-gpui-katalog
cargo check -p uplot-rs-gpui-web --target wasm32-unknown-unknown
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
npm --prefix tools/uyum run denetle
NO_COLOR=false trunk build --release --public-url /uPlot.rs/
```

Test sonucu:

- 318 çekirdek birim testi;
- 14 entegrasyon testi;
- 73 kart, 203 API satırı ve 19 ortak davranış uyum denetimi;
- yarım milyon nokta, 41.986 Mass Spectrum örneği, 34.110 heatmap hücresi,
  13.608 sparse X değeri ve 60 FPS setData akışı için veri/retained-sahne
  regresyonları;
- hover'ın ana sahne geometrisini değiştirmemesi;
- yalnız değişen serinin GPUI yolunun geçersizleşmesi;
- path önbelleği kapasitesinin yeniden kullanılması;
- SVG kaydının belirlenimli olması ve grafik durumunu değiştirmemesi;
- etkileşim katmanının yalnız açıkça istendiğinde SVG'ye alınması.

Release GPUI Web çıktısı:

- `index.html`: yaklaşık 2,4 KB;
- GPUI Web JavaScript başlatıcısı: yaklaşık 77 KB;
- optimize wasm32 uygulaması: yaklaşık 11 MB.

Canlı tarayıcı kabulünde tek WebGPU canvas açıldı. `GPUI SVG Export` kartı
kayıttan önce “kayıt yalnız düğmeye basıldığında çalışır” durumunu gösterdi.
`SVG'yi panoya kopyala` eylemi çağrıldıktan sonra 800×400 gerçek vektör kayıt
üretildi ve arayüz 2.686 bayt raporladı. Bu, kayıt akışının normal GPUI
karelerinden ayrık olduğunun uçtan uca kabul kanıtıdır.

## Sürekli koruma

`tools/uyum/denetle.mjs` aşağıdakiler yeniden eklenirse CI'yı başarısız yapar:

- eski `examples/` ve `wasm/` dizinleri;
- eski `src/svg.rs`;
- kök Cargo paketinde `[[example]]`;
- zorunlu olmayan veya kaybolmuş GPUI kök bağımlılığı;
- yeniden dışa açılmış bağımsız `svg` renderer modülü.

CI ayrıca SVG feature kapalı/açık kütüphane derlemelerini, native kataloğu,
wasm32 GPUI Web hedefini, Clippy'yi ve bütün testleri her değişiklikte yeniden
çalıştırır.
