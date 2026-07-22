# Resmî uPlot deposundan farklılıklar

[English](#differences-from-the-official-uplot-repository) · **Türkçe**

uPlot.rs bir uPlot portudur; normatif kaynak
[uPlot `0e5812c` commit'idir](https://github.com/leeoniya/uPlot/commit/0e5812c504430f5c804e0f993376d8999b26cc34).
Bu belge, kaynak davranışın doğrudan port edildiği yerlerle uPlot.rs'e özgü
uyarlama ve uzantıların birbirine karışmaması için tutulur.

## Davranış ve mimari farkları

| Alan | Resmî uPlot | uPlot.rs | Tür |
|---|---|---|---|
| Dil ve API | JavaScript seçenek nesneleri | Rust 2024 tipleri ve oluşturucu zincirleri | Zorunlu port uyarlaması |
| Çizim yüzeyi | Canvas tabanlı tarayıcı çizimi | Ortak sahne komutlarından SVG/WASM ve GPUI çizimi | Zorunlu port uyarlaması |
| Hata modeli | JavaScript çalışma zamanı davranışı | Tipli `UplotHatası`; üretim kodunda panic yasakları | uPlot.rs güvenlik ilkesi |
| Etkileşim tanımı | Bazı cursor davranışları örtük varsayılandır; eklentiler ayrıca kurulur | Etkileşimler kart başına açık `true`/`false` seçenekleridir | API uyarlaması |
| Tekerlek yakınlaştırma | `demos/zoom-wheel.html` içindeki isteğe bağlı `wheelZoomPlugin` | Aynı eklentinin kart seçeneğiyle açılan portu ve canlı anahtarı | Resmî eklenti portu + kontrol uyarlaması |
| Dokunma yakınlaştırma/taşıma | `demos/zoom-touch.html` içinde iki parmakla X/Y yakınlaştırma ve tek dokunuşla taşıma | `dokunma_etkileşimi(true)` ile yüzey adaptörlerinin kullandığı çekirdek davranışı | Resmî eklenti portu + API uyarlaması |
| Masaüstü taşıma bağı | `zoom-wheel.html` orta tuşla X taşıma yapar; `zoom-touch.html` X/Y taşıma matematiğini içerir | Yakın görünümde boşluk + sol sürükleme aynı ölçek matematiğiyle X/Y taşır; ayrı seçenek gerekmez | Resmî matematik portu + giriş uyarlaması |
| Hassas tekerlek girdisi | Her `wheel` olayına sabit katsayı uygular | Ayrık mouse tekerleği ile Magic Mouse/trackpad piksel akışını otomatik ayırır | uPlot.rs uzantısı |
| Tam görünüme dönüş | Çekirdekte çift tıklama `autoScaleX()` çağırır | Çift tıklamaya ek olarak görünür “Tam görünüm” düğmesi sunar | Keşfedilebilirlik uyarlaması |
| Görünüm geçmişi | Adımlı geri alma geçmişi bulunmaz | Hareket başına kayıt tutan “Geri” kontrolü vardır | uPlot.rs uzantısı |
| Örnek kataloğu | Bağımsız HTML demoları | Aynı kart tanımını kullanan masaüstü ve WASM chart listeleri | Port sunumu |
| Örnek/çekirdek sınırı | Demolar dağıtımdan ayrı HTML girişleridir | GPUI kataloğu ayrı ve yayınlanmayan workspace uygulamasıdır; bütün davranış çekirdektedir | Rust paket uyarlaması |
| Yüzey seçimi | Canvas tarayıcı ortamına gömülüdür | `svg`, `wasm` ve `gpui` toplamsal Cargo feature'larıyla seçilir | Rust paket uyarlaması |

### Hassas tekerlek normalizasyonu

`TekerlekKipi::Otomatik` aşağıdaki uPlot.rs davranışlarını ekler:

- satır tabanlı klasik mouse tekerleğinde resmî `0.75` adımı korunur;
- piksel tabanlı girdide 100 piksel toplam hareket bir resmî adıma eşlenir;
- 1.5 piksel altındaki küçük hareketler ölü bölgede tutulur;
- masaüstü ve WASM aynı çekirdek denetleyicisine yalnız normalize olay girdisi iletir;
- tekerlek geçmişi olay başına değil, 140 ms ile gruplanan hareket başına yazılır.

Bu normalizasyon resmî `wheelZoomPlugin`in parçası değildir. Eklentinin fare
odağını göreli konumunda tutma ve tam veri aralığına sıkıştırma yaklaşımı ise
korunur.

## Fark olmayan, doğrudan taşınan davranışlar

Aşağıdakiler uPlot.rs'e özgü özellik olarak değerlendirilmez:

- koşullu boş seri noktaları ve hover sırasında dolu nokta;
- fareyi kesintisiz izleyen cursor çizgileri ve canlı lejant;
- X ekseninde sürükleyerek seçim/yakınlaştırma;
- çift tıklamayla tam X aralığına dönme;
- resmî `wheelZoomPlugin`in `0.75` temel katsayısı, fare odağı ve aralık sınırı.
- resmî `zoom-touch` eklentisinin X/Y ölçek yakınlaştırma ve taşıma matematiği.

Kaynaklar:

- [`demos/resize.html`](https://github.com/leeoniya/uPlot/blob/0e5812c504430f5c804e0f993376d8999b26cc34/demos/resize.html)
- [`demos/zoom-wheel.html`](https://github.com/leeoniya/uPlot/blob/0e5812c504430f5c804e0f993376d8999b26cc34/demos/zoom-wheel.html)
- [`demos/zoom-touch.html`](https://github.com/leeoniya/uPlot/blob/0e5812c504430f5c804e0f993376d8999b26cc34/demos/zoom-touch.html)
- [`src/uPlot.js` çift tıklama davranışı](https://github.com/leeoniya/uPlot/blob/0e5812c504430f5c804e0f993376d8999b26cc34/src/uPlot.js#L3364)

## Kaynak ve bağımlılık politikası

Normatif uPlot kaynağı commit ve dosya hash'leriyle kilitlidir. `gpui` ile
`gpui_kutuphanesi` sürekli geliştiği için commit pinlenmez; yerel derlemeler
kardeş çalışma ağaçlarının, CI ise güncel varsayılan dalların durumunu kullanır.
Bu politika bir uPlot davranış farkı değil, portun geliştirme politikasıdır.

---

# Differences from the official uPlot repository

**English** · [Türkçe](#resmî-uplot-deposundan-farklılıklar)

uPlot.rs is a port whose normative source is
[uPlot commit `0e5812c`](https://github.com/leeoniya/uPlot/commit/0e5812c504430f5c804e0f993376d8999b26cc34).
This inventory separates direct ports from uPlot.rs-specific adaptations.

## Behavioral and architectural differences

| Area | Official uPlot | uPlot.rs | Classification |
|---|---|---|---|
| Language and API | JavaScript option objects | Rust 2024 types and builder chains | Required port adaptation |
| Rendering surface | Browser Canvas rendering | Shared scene commands rendered by SVG/WASM and GPUI | Required port adaptation |
| Error model | JavaScript runtime behavior | Typed `UplotHatası` and production panic prohibitions | uPlot.rs safety policy |
| Interaction declaration | Some cursor behaviors are implicit defaults; plugins are installed separately | Explicit per-card `true`/`false` interaction options | API adaptation |
| Wheel zoom | Optional `wheelZoomPlugin` in `demos/zoom-wheel.html` | Port enabled per card, with a live switch | Official plugin port plus control adaptation |
| Touch zoom/pan | Two-finger X/Y zoom and single-touch pan in `demos/zoom-touch.html` | Core behavior enabled per chart with `dokunma_etkileşimi(true)` | Official plugin port plus API adaptation |
| Desktop pan binding | Middle-button X pan in `zoom-wheel.html`; X/Y pan mathematics in `zoom-touch.html` | Space + left drag pans X/Y after zoom, without a separate option | Official mathematics plus input adaptation |
| Precise wheel input | Fixed factor per `wheel` event | Automatic separation of discrete wheels and Magic Mouse/trackpad pixel streams | uPlot.rs extension |
| Full-range reset | Core double-click calls `autoScaleX()` | Also exposes a visible Full view button | Discoverability adaptation |
| View history | No stepwise view undo | Gesture-level Back history | uPlot.rs extension |
| Demo catalog | Independent HTML demos | Desktop and WASM chart lists sharing one card definition | Port presentation |
| Demo/core boundary | Demos are separate HTML entry points | The GPUI catalog is a separate unpublished workspace app; all behavior lives in core | Rust packaging adaptation |
| Surface selection | Canvas is built into the browser runtime | Additive `svg`, `wasm`, and `gpui` Cargo features select surfaces | Rust packaging adaptation |

In automatic wheel mode, traditional line-based wheels keep the official
`0.75` step. Precise input maps 100 pixels to one official step, applies a
1.5-pixel dead zone, and stores one history entry per 140 ms gesture. Desktop
and WASM pass only normalized event input to the same core controller. This normalization is not part of the
official plugin; cursor anchoring and full-range clamping remain ported
behavior.

Conditional hollow points, filled hover markers, continuous cursor lines,
live legends, drag selection, double-click reset, and the official wheel
plugin's base mathematics are direct ports and are not claimed as uPlot.rs
extensions. The same applies to the X/Y scale mathematics ported from the
official `zoom-touch` demo; only the Space + left-drag desktop binding is an
input adaptation.

The normative uPlot checkout is commit/hash locked. GPUI dependencies remain
live by project policy; this is a development-policy distinction rather than
a chart-behavior difference.
