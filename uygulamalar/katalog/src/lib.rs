//! Native ve GPUI Web'in paylaştığı tek Rust grafik kataloğu.

#[cfg(not(target_family = "wasm"))]
use gpui::ClipboardItem;
use gpui::{
    AccessibleAction, App, Bounds, ClickEvent, Context, Entity, Focusable, FontWeight, Hsla,
    IntoElement, KeyBinding, ListAlignment, ListState, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, Pixels, Render, Role, ScrollStrategy, ScrollWheelEvent, SharedString, Size,
    StyleRefinement, Task, UniformListScrollHandle, WeakEntity, Window, canvas, div, list,
    prelude::*, px, rgb, rgba, uniform_list,
};
use ortak_bilesenler::{
    Anahtar, AnahtarOlayi, CubukAyarlari, Dugme, DugmeBoyutu, DugmeTuru, MetinAlani,
    MetinAlaniOlayi, PlatformPencere,
};
use std::collections::HashSet;
use std::ops::Range;
use std::time::Duration;
use uplot_rs::LejantKonumu;
use uplot_rs::gpui::{
    GpuiGrafik, GpuiGrafikGrubu, GpuiGrafikGrupAyarları, GpuiGrafikOlayı, GpuiSeriEşleme,
    GörünürAlan, renk_çöz, uyarlanan_alan,
};
use uplot_rs::izleme;
use uplot_rs_gpui_ornekler::{
    ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ, ALIGN_DATA_KART_TANIM_ÖRNEĞİ, ANNOTATIONS_KART_TANIM_ÖRNEĞİ,
    ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ, AREA_FILL_KART_TANIM_ÖRNEĞİ, AXIS_AUTOSIZE_ARALIK_MS,
    AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ, AXIS_CONTROL_KART_TANIM_ÖRNEĞİ,
    AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ, AlignDataÖrneği, AxisAutosizeAkışı,
    BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ, BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    BOX_WHISKER_KART_TANIM_ÖRNEĞİ, BoyutSenkronAkışı, CANDLESTICK_KART_TANIM_ÖRNEĞİ,
    CURSOR_BIND_KART_TANIM_ÖRNEĞİ, CURSOR_SNAP_KART_TANIM_ÖRNEĞİ, CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ,
    CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ, CustomScaleÖrneği, DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ,
    DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ, DRAW_HOOKS_KART_TANIM_ÖRNEĞİ, EtkileşimSeçenekleri,
    FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ, FocusÖrneği, GPUI_SVG_EXPORT_KART_TANIM_ÖRNEĞİ,
    GRADIENTS_KART_TANIM_ÖRNEĞİ, GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ, GpuiSvgKayıtAyarları,
    GradientÖrneği, Grafik, HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ, HighLowBandsÖrneği,
    LATENCY_HEATMAP_KART_TANIM_ÖRNEĞİ, LINE_PATHS_KART_TANIM_ÖRNEĞİ, LOG_SCALES_KART_TANIM_ÖRNEĞİ,
    LOG_SCALES2_KART_TANIM_ÖRNEĞİ, LatencyHeatmapÖrneği, LinePathsÖrneği, LogScales2Örneği,
    LogScalesÖrneği, MASS_SPECTRUM_KART_TANIM_ÖRNEĞİ, MEASURE_DATUMS_KART_TANIM_ÖRNEĞİ,
    MISSING_DATA_KART_TANIM_ÖRNEĞİ, MONTHS_KART_TANIM_ÖRNEĞİ, MONTHS_RU_KART_TANIM_ÖRNEĞİ,
    MULTI_BARS_KART_TANIM_ÖRNEĞİ, MissingDataÖrneği, MultiBarsÖrneği,
    NEAREST_NON_NULL_KART_TANIM_ÖRNEĞİ, NICE_SCALE_KART_TANIM_ÖRNEĞİ, NO_DATA_KART_TANIM_ÖRNEĞİ,
    NearestNonNullÖrneği, NoDataÖrneği, PATH_GAP_CLIP_KART_TANIM_ÖRNEĞİ,
    PIXEL_ALIGN_KART_TANIM_ÖRNEĞİ, POINTS_KART_TANIM_ÖRNEĞİ, PathGapClipÖrneği, PixelAlignAkışı,
    PixelAlignÖrneği, PointsÖrneği, RESIZE_KART_TANIM_ÖRNEĞİ, SCALE_PADDING_KART_TANIM_ÖRNEĞİ,
    SCALES_DIR_ORI_KART_TANIM_ÖRNEĞİ, SCATTER_KART_TANIM_ÖRNEĞİ, SCROLL_SYNC_KART_TANIM_ÖRNEĞİ,
    SINE_STREAM_KART_TANIM_ÖRNEĞİ, SOFT_MINMAX_KART_TANIM_ÖRNEĞİ,
    SPARKLINES_BARS_KART_TANIM_ÖRNEĞİ, SPARKLINES_KART_TANIM_ÖRNEĞİ, SPARSE_KART_TANIM_ÖRNEĞİ,
    STACKED_SERIES_KART_TANIM_ÖRNEĞİ, STREAM_DATA_ARALIK_MS, STREAM_DATA_KART_TANIM_ÖRNEĞİ,
    SYNC_CURSOR_KART_TANIM_ÖRNEĞİ, SYNC_Y_ZERO_KART_TANIM_ÖRNEĞİ, ScalesDirOriÖrneği,
    ScatterÖrneği, SineAkışı, SmoothingÖrneği, SoftMinMaxAkışı, SoftMinMaxÖrneği,
    SparklinesBarsÖrneği, SparklineÖrneği, SparseÖrneği, StackedSeriesÖrneği, StreamDataGrubu,
    StreamDataÖrneği, SyncCursorGrubu, SyncCursorÖrneği, SyncYZeroAşaması,
    THIN_BARS_STROKE_FILL_KART_TANIM_ÖRNEĞİ, TIME_PERIODS_KART_TANIM_ÖRNEĞİ,
    TIMELINE_DISCRETE_KART_TANIM_ÖRNEĞİ, TIMESERIES_DISCRETE_KART_TANIM_ÖRNEĞİ,
    TIMEZONES_DST_KART_TANIM_ÖRNEĞİ, TOOLTIPS_CLOSEST_KART_TANIM_ÖRNEĞİ,
    TOOLTIPS_KART_TANIM_ÖRNEĞİ, TRENDLINES_KART_TANIM_ÖRNEĞİ, ThinBarsÖrneği, TimePeriodsÖrneği,
    TimelineDiscreteÖrneği, TimeseriesDiscreteÖrneği, TimezonesDstÖrneği,
    UPDATE_CURSOR_SELECT_RESIZE_ARALIK_MS, UPDATE_CURSOR_SELECT_RESIZE_KART_TANIM_ÖRNEĞİ,
    UplotHatası, WIND_DIRECTION_KART_TANIM_ÖRNEĞİ, Y_SCALE_DRAG_KART_TANIM_ÖRNEĞİ,
    Y_SHIFTED_SERIES_ARALIK_MS, Y_SHIFTED_SERIES_KART_TANIM_ÖRNEĞİ, YShiftedSeriesAkışı,
    add_del_series_ek_seçeneği, add_del_series_ek_verisi, add_del_series_kartı,
    align_data_kartları, align_data_maliyet_kartı, annotations_kartı, arcsinh_scales_kartı,
    area_fill_kartı, axis_autosize_kartı, axis_control_kartı, axis_indicators_kartı,
    bars_grouped_stacked_kartları, bars_grouped_stacked_kartı, bars_values_autosize_kartları,
    bars_values_autosize_kartı, box_whisker_kartları, box_whisker_kartı, candlestick_ohlc_kartı,
    cursor_bind_kartı, cursor_snap_kartı, cursor_tooltip_kartı, custom_scales_kartları,
    custom_scales_kartı, data_smoothing_kartı, dependent_scale_kartı, draw_hooks_kartı,
    focus_cursor_kartları, focus_cursor_kartı, gpui_svg_export_kartı, gradients_kartları,
    gradients_kartı, grid_over_series_kartı, high_low_bands_kartları, high_low_bands_kartı,
    latency_heatmap_kartları, latency_heatmap_kartı, line_paths_kartları, line_paths_kartı,
    log_scales_kartları, log_scales_kartı, log_scales2_kartları, log_scales2_kartı,
    mass_spectrum_kartı, measure_datums_kartı, missing_data_kartları, missing_data_null_kartı,
    months_artık_yılsız_kartı, months_kartları, months_rusça_kartı, multi_bars_kartı,
    nearest_non_null_kartı, nice_scale_kartı, no_data_kartı, ortak_kart_etkileşimleri,
    path_gap_clip_kartları, path_gap_clip_kartı, pixel_align_kartları, pixel_align_kartı,
    points_kartları, points_kartı, resize_kartı, scale_padding_kartı, scales_dir_ori_kartları,
    scales_dir_ori_kartı, scatter_kartı, scroll_sync_kartı, sine_stream_kartı,
    soft_minmax_kartları, soft_minmax_kartı, sparklines_bars_kartları, sparklines_bars_kartı,
    sparklines_kartları, sparklines_kartı, sparse_kartları, sparse_kartı, stacked_series_kartları,
    stacked_series_kartı, stacked_series_kartı_görünür, stream_data_kartı, sync_cursor_kartı,
    sync_y_zero_aralıkları, sync_y_zero_kartı, thin_bars_stroke_fill_kartları,
    thin_bars_stroke_fill_kartı, time_periods_kartları, time_periods_kartı,
    timeline_discrete_kartları, timeline_discrete_kartı, timeseries_discrete_kartları,
    timeseries_discrete_kartı, timezones_dst_kartları, timezones_dst_kartı, tooltips_closest_kartı,
    tooltips_kartı, trendlines_kartı, update_cursor_select_resize_kartı, wind_direction_kartı,
    y_scale_drag_kartı, y_shifted_series_kartı, ÇubukYönü, ÇubukÖrneği, İmleçBağSeçenekleri,
};
use web_time::Instant;

#[path = "web_koprusu.rs"]
mod web_köprüsü;

gpui::actions!(uplot_katalog, [KartıEtkinleştir]);

/// Katalog kökü lejant veya denetim metni için yenilendiğinde değişmemiş
/// grafik alt ağaçlarını tekrar render etmez. Grafik varlığının kendi
/// `notify()` çağrısı bu önbelleği gerektiğinde geçersiz kılar.
fn önbellekli_grafik(grafik: Entity<GpuiGrafik>) -> impl IntoElement {
    grafik.cached(StyleRefinement::default().size_full())
}

/// Kart yüzeylerinin görünür alana uyarlanıp uyarlanmayacağı.
///
/// Açıkken yüzeyler resmî sayfadaki ham boyutlarının en boy oranını koruyarak
/// görünür alana dikeyde sığdırılır; kapatıldığında ham boyutlarıyla çizilir ve
/// alana sığmayan kısım kaydırmayla gezilir. Uyarlama davranışının kendisi
/// çekirdekte `uplot_rs::gpui::uyarlanan_alan` altında tanımlıdır.
const OTOMATİK_UYARLA: bool = true;

/// Yüzeyin üstündeki başlık, açıklama kutusu ve alt yazı satırlarının
/// sığdırma alanından düşülen ortak payı.
///
/// Ölçüm çizim tabanının tamamını kapsadığından, yüzeye kalan alan bu metin
/// bloklarının yüksekliği kadar azdır. Pay düşülmezse yüzey kalan alandan
/// büyük hesaplanır ve alt kenardan kırpılır.
const AÇIKLAMA_PAYI: f32 = 190.0;

/// Sync Cursor panelinde lejant satırına ayrılan yükseklik.
const SYNC_LEJANT_PAYI: f32 = 30.0;

/// Sync Cursor panelinin ham yüksekliği: grafik yüzeyi ve lejant satırı.
const SYNC_PANEL_YÜKSEKLİĞİ: f32 = 236.0;

/// Üç panel satırının ve aralarındaki iki boşluğun ham toplamı.
///
/// Grup üyeleri imleç, seçim ve zoom durumunu paylaştığından tek ölçekle
/// birlikte sığdırılır; panel başına ayrı ölçek uygulanmaz.
const SYNC_TOPLAM_YÜKSEKLİK: f32 = SYNC_PANEL_YÜKSEKLİĞİ * 3.0 + 16.0;

/// Canlı lejantın tek seri girdisi.
///
/// `indeks`, sahibin görünürlük yolunda kullandığı birleşik seri numarasıdır:
/// tek yüzeyli kartlarda doğrudan seri indeksi, birleşik lejant kuran
/// Timeseries Discrete'te yüzeyler boyunca ilerleyen sıralı numaradır.
#[derive(Clone, PartialEq)]
struct LejantGirdisi {
    indeks: usize,
    etiket: SharedString,
    değer: SharedString,
    renk: Hsla,
    görünür: bool,
}

/// Canlı lejantı taşıyan bağımsız yüzey.
///
/// Lejant imleç en yakın veri indeksini her değiştirdiğinde tazelenir. Yoğun
/// serilerde bu neredeyse her pointer olayında olur; içeriği katalog kökünün
/// içinde tutmak o olayların hepsini ~3.500 satırlık `render`'a bağlıyordu.
/// Ayrı bir varlık yalnız bu listeyi yeniden kurar.
struct KatalogLejantı {
    /// Tıklama görünürlüğü kartın sahibinde değiştirir; lejant hangi yüzeyin
    /// hangi serisi olduğunu bilmez. Zayıf tutulur, aksi hâlde kökle karşılıklı
    /// sahiplik oluşur.
    sahip: WeakEntity<ChartListesi>,
    x_metni: SharedString,
    girdiler: Vec<LejantGirdisi>,
    konum: LejantKonumu,
}

impl KatalogLejantı {
    fn yeni(sahip: WeakEntity<ChartListesi>) -> Self {
        Self {
            sahip,
            x_metni: SharedString::default(),
            girdiler: Vec::new(),
            konum: LejantKonumu::Alt,
        }
    }

    fn içeriği_ayarla(
        &mut self,
        x_metni: SharedString,
        girdiler: Vec<LejantGirdisi>,
        konum: LejantKonumu,
        cx: &mut Context<Self>,
    ) {
        if self.x_metni == x_metni && self.girdiler == girdiler && self.konum == konum {
            return;
        }
        self.x_metni = x_metni;
        self.girdiler = girdiler;
        self.konum = konum;
        cx.notify();
    }
}

impl Render for KatalogLejantı {
    fn render(&mut self, _pencere: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let dikey = self.konum.dikey_mi();
        let sahip = self.sahip.clone();
        div()
            .flex()
            .when(dikey, |öğe| öğe.flex_col().gap_1())
            .when(!dikey, |öğe| öğe.flex_wrap().gap_x_3().gap_y_1())
            .items_start()
            .text_xs()
            .child(
                div()
                    .flex_none()
                    .text_color(rgb(LEJANT_METİN_RENGİ))
                    .child(self.x_metni.clone()),
            )
            .children(self.girdiler.iter().map(|girdi| {
                let indeks = girdi.indeks;
                let tıklama_sahibi = sahip.clone();
                let odak_sahibi = sahip.clone();
                // uPlot gizli seriyi lejanttan kaldırmaz, `.u-off` ile
                // soluklaştırır; girdi yerinde kalmazsa hangi serinin
                // kapatıldığı ve nereden geri açılacağı görünmez olur.
                let (işaret, metin) = if girdi.görünür {
                    (girdi.renk, rgb(LEJANT_METİN_RENGİ).into())
                } else {
                    (
                        girdi.renk.opacity(GİZLİ_SERİ_SOLUKLUĞU),
                        Hsla::from(rgb(LEJANT_METİN_RENGİ)).opacity(GİZLİ_SERİ_SOLUKLUĞU),
                    )
                };
                div()
                    .id(("lejant-seri", indeks))
                    .flex()
                    .flex_none()
                    .items_center()
                    .gap_1()
                    .cursor_pointer()
                    .text_color(metin)
                    .child(div().flex_none().size(px(8.0)).rounded_full().bg(işaret))
                    .child(format!("{}: {}", girdi.etiket, girdi.değer))
                    .on_click(move |_, _, cx| {
                        tıklama_sahibi
                            .update(cx, |bu, cx| bu.lejant_serisini_değiştir(indeks, cx))
                            .ok();
                    })
                    // uPlot lejant satırında `setSeries(i, {focus: true})`
                    // uygular: odaklanan seri kendi rengini korur, diğerleri
                    // `focus.alpha` ile soluklaşır. Etki yalnız `cursor.focus`
                    // kurulmuş kartlarda görünür.
                    .on_hover(move |üzerinde: &bool, _, cx| {
                        let hedef = üzerinde.then_some(indeks);
                        odak_sahibi
                            .update(cx, |bu, cx| bu.lejant_serisini_odakla(hedef, cx))
                            .ok();
                    })
            }))
    }
}

trait KatalogKaydırmaUzantısı: Styled {
    /// Dikey katalog içinde yatay yüzeyin normal wheel hareketini çalmasını
    /// önler; yatay hareket gerçek yatay delta veya Shift+wheel ile kalır.
    fn yalnız_tekerlek_ekseninde_kaydır(mut self) -> Self {
        self.style().restrict_scroll_to_axis = Some(true);
        self
    }
}

impl<T: Styled> KatalogKaydırmaUzantısı for T {}

/// Ortak grafik ve katalog GPUI eylemlerini uygulamaya bir kez kaydeder.
/// Paylaşılan bileşen kütüphanesinin katalog kromuyla uyumlu ayarları.
///
/// Katalog kendi kromunu açık paletle çiziyor (`panel` beyaz, `zemin`
/// `#f3f4f6`). `ortak_bilesenler` tema sağlayıcı verilmediğinde koyu temayı
/// kuruyor (`ortak_bilesenler/src/tema.rs`, `VarsayilanTema::koyu()`); bu da
/// anahtar etiketlerini `#DCE0E5` ile açık zeminde 1,20:1 kontrasta
/// düşürüp okunmaz hâle getiriyordu. Açık temayı açıkça bağlıyoruz:
/// aynı etiket `#24292F` ile 13,31:1 veriyor.
pub fn ortak_bileşen_ayarları() -> ortak_bilesenler::OrtakBilesenAyarlari {
    ortak_bilesenler::OrtakBilesenAyarlari {
        tema_saglayici: Some(std::sync::Arc::new(ortak_tema::VarsayilanTema::acik())),
        ..ortak_bilesenler::OrtakBilesenAyarlari::default()
    }
}

pub fn başlat(cx: &mut App) {
    izleme::başlat();
    uplot_rs::gpui::başlat(cx);
    cx.bind_keys([
        KeyBinding::new("enter", KartıEtkinleştir, Some("uplot_katalog_kartı")),
        KeyBinding::new("space", KartıEtkinleştir, Some("uplot_katalog_kartı")),
    ]);
}

const PERFORMANS_KARE_SAYISI: usize = 180;
const KARE_P95_BÜTÇESİ_MS: f64 = 16.7;
/// Lejant etiket ve değer metninin rengi.
///
/// İşaret kutusu serinin kendi rengini taşıdığından metin nötr kalır; aksi
/// hâlde açık renkli serilerde satır beyaz zeminde okunmuyordu. `#374151`
/// beyaz üzerinde 9,73:1 veriyor.
const LEJANT_METİN_RENGİ: u32 = 0x374151;
/// Gizli serinin lejant satırına uygulanan opaklık.
///
/// uPlot `.u-legend .u-off > *` kuralıyla aynı 0,3 değeridir; girdi
/// okunabilir kalmalı ama kapalı olduğu ilk bakışta ayırt edilmelidir.
const GİZLİ_SERİ_SOLUKLUĞU: f32 = 0.3;
/// Lejant yan konumdayken sütuna ayrılan genişlik.
///
/// Sınır olmadan uzun seri etiketleri yüzeyin payını yiyor, kısa etiketlerde
/// ise sütun gereksiz daralıp değerleri sarıyordu.
const LEJANT_YAN_GENİŞLİĞİ: f32 = 180.0;

/// Bir yüzeyin serilerini lejant girdisi olarak listeye ekler.
///
/// `değerler`, serilerle aynı sırada gelir ve gizli seriler de listede kalır:
/// girdi düşürülseydi ardındaki her seri bir kayarak komşusunun değerini
/// gösterirdi. `indeks` liste boyunca artar, böylece birleşik lejant kuran
/// kartlarda yüzey sınırını geçen tek bir numaralandırma oluşur.
fn lejant_girdilerini_ekle(
    girdiler: &mut Vec<LejantGirdisi>,
    seriler: &[uplot_rs::SeriSeçenekleri],
    değerler: &[Option<f64>],
    boşta: bool,
    tam_sayı_değerler: bool,
) {
    for (seri_indeksi, seri) in seriler.iter().enumerate() {
        let değer = değerler.get(seri_indeksi).copied().flatten().map_or_else(
            || "--".to_string(),
            |y| {
                let değer = if tam_sayı_değerler && seri.etiket.starts_with("DEV") {
                    format!("{y:.0}")
                } else {
                    format!("{y:.3}")
                };
                format!("{değer}{}", if boşta { " (last)" } else { "" })
            },
        );
        // uPlot etiketsiz seriyi lejantta boş hücre gösterir. Katalog bir
        // doğrulama aracı olduğundan burada sıra numarası veriliyor: aksi
        // hâlde `stacked-series` gibi etiketsiz kartlarda girdiler `● : --`
        // olarak çıkıyor ve hangi seriye tıklandığı ayırt edilemiyor.
        let etiket = if seri.etiket.trim().is_empty() {
            format!("Seri {}", seri_indeksi.saturating_add(1))
        } else {
            seri.etiket.clone()
        };
        girdiler.push(LejantGirdisi {
            indeks: girdiler.len(),
            etiket: SharedString::from(etiket),
            değer: SharedString::from(değer),
            renk: renk_çöz(&seri.renk),
            görünür: seri.göster,
        });
    }
}

/// Birleşik lejant indeksini (yüzey sırası, yüzey içi seri) çiftine çözer.
///
/// `lejant_girdilerini_ekle` yüzeyleri sırayla numaralandırdığından tıklanan
/// girdi de aynı sayımla geri çözülmelidir; yüzey başına sabit seri sayısı
/// varsaymak seri eklenip silinen kartlarda yanlış seriyi kapatır.
fn lejant_hedefini_çöz(
    seri_sayıları: &[usize],
    birleşik_indeks: usize,
) -> Option<(usize, usize)> {
    let mut kalan = birleşik_indeks;
    for (yüzey, seri_sayısı) in seri_sayıları.iter().enumerate() {
        if kalan < *seri_sayısı {
            return Some((yüzey, kalan));
        }
        kalan -= *seri_sayısı;
    }
    None
}

/// Lejant konumunun denetim düğmesinde gösterilen adı.
const fn lejant_konumu_başlığı(konum: LejantKonumu) -> &'static str {
    match konum {
        LejantKonumu::Alt => "alt",
        LejantKonumu::Üst => "üst",
        LejantKonumu::Sol => "sol",
        LejantKonumu::Sağ => "sağ",
    }
}
/// Kart listesindeki ikincil satırların rengi.
///
/// Önceki `#6b7280`, beyaz zeminde 4,83:1 veriyordu ama seçili kartın
/// `#fef2f2` zemininde 4,42:1'e düşüp WCAG AA eşiğinin (4,5:1) altına
/// iniyordu — üstelik bu satırlar `text_xs`. `#4b5563` aynı zeminlerde
/// 7,56:1 ve 6,91:1 veriyor.
const LİSTE_İKİNCİL_RENGİ: u32 = 0x4b5563;
/// Kart listesindeki kaynak etiketinin rengi. Vurgu kırmızısı (`#dc2626`)
/// seçili zeminde 4,42:1'de kalıyordu; `#b91c1c` 5,91:1 veriyor. Kenarlık
/// ve seçim vurgusu için `vurgu` olduğu gibi kullanılmaya devam ediyor.
const LİSTE_KAYNAK_RENGİ: u32 = 0xb91c1c;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Kareİstatistiği {
    p50_ms: f64,
    p95_ms: f64,
    p99_ms: f64,
    azami_ms: f64,
}

#[derive(Default)]
struct KareÖlçer {
    örnekler_ms: Vec<f64>,
    son_kare: Option<Instant>,
    sonuç: Option<Kareİstatistiği>,
    çalışıyor: bool,
}

impl KareÖlçer {
    fn başlat(&mut self) {
        self.örnekler_ms.clear();
        self.örnekler_ms.reserve(PERFORMANS_KARE_SAYISI);
        self.son_kare = None;
        self.sonuç = None;
        self.çalışıyor = true;
    }

    fn kareyi_kaydet(&mut self, şimdi: Instant) {
        if !self.çalışıyor {
            return;
        }
        if let Some(önceki) = self.son_kare {
            self.örnekler_ms
                .push(şimdi.duration_since(önceki).as_secs_f64() * 1_000.0);
        }
        self.son_kare = Some(şimdi);
        if self.örnekler_ms.len() >= PERFORMANS_KARE_SAYISI {
            self.sonuç = kare_istatistiği(&self.örnekler_ms);
            self.çalışıyor = false;
        }
    }

    fn ilerleme(&self) -> usize {
        self.örnekler_ms.len()
    }
}

fn kare_istatistiği(örnekler_ms: &[f64]) -> Option<Kareİstatistiği> {
    if örnekler_ms.is_empty() || örnekler_ms.iter().any(|değer| !değer.is_finite()) {
        return None;
    }
    let mut sıralı = örnekler_ms.to_vec();
    sıralı.sort_by(f64::total_cmp);
    let yüzdelik = |oran: f64| {
        let son = sıralı.len().saturating_sub(1);
        let indeks = ((son as f64) * oran).round() as usize;
        sıralı.get(indeks.min(son)).copied()
    };
    Some(Kareİstatistiği {
        p50_ms: yüzdelik(0.50)?,
        p95_ms: yüzdelik(0.95)?,
        p99_ms: yüzdelik(0.99)?,
        azami_ms: *sıralı.last()?,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KartKimliği {
    AddDelSeries,
    AlignDataCost,
    Resize,
    Annotations,
    AreaFill,
    ScalePadding,
    Months,
    MonthsRussian,
    NiceScale,
    NoData,
    PathGapClip,
    PixelAlign,
    Points,
    ScalesDirOri,
    Scatter,
    ScrollSync,
    SineStream,
    SoftMinMax(SoftMinMaxÖrneği),
    SparklinesBars(SparklinesBarsÖrneği),
    Sparklines(SparklineÖrneği),
    Sparse(SparseÖrneği),
    StackedSeries(StackedSeriesÖrneği),
    StreamData(StreamDataÖrneği),
    GpuiSvgExport,
    SyncCursor,
    SyncYZero(SyncYZeroAşaması),
    ThinBars(ThinBarsÖrneği),
    TimePeriods(TimePeriodsÖrneği),
    TimelineDiscrete(TimelineDiscreteÖrneği),
    TimeseriesDiscrete,
    TimezonesDst,
    TooltipsClosest,
    Tooltips,
    Trendlines,
    UpdateCursorSelectResize,
    WindDirection,
    YScaleDrag,
    YShiftedSeries,
    CursorBind,
    CursorSnap,
    CursorTooltip,
    CustomScales,
    DataSmoothing,
    DrawHooks,
    FocusCursor,
    Gradients,
    GridOverSeries,
    HighLowBands,
    LatencyHeatmap,
    LinePaths,
    LogScales,
    LogScales2,
    MassSpectrum,
    MeasureDatums,
    MultiBars(MultiBarsÖrneği),
    NearestNonNull,
    MissingData,
    DependentScale,
    ArcSinhScales,
    AxisControl,
    AxisAutosize,
    AxisIndicators,
    Bars(ÇubukÖrneği),
    BarsValuesAutosize(ÇubukYönü),
    BoxWhisker(&'static str),
    Candlestick,
}

/// Katalog arayüzü, derin bağlantı, açıklama paneli ve grafik fabrikasının
/// paylaştığı tek literal kart kaydı.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KatalogKartGrubu {
    Tek,
    İlişkiliYüzeyler,
}

type KatalogKartÇıktısı = Result<(uplot_rs::GrafikSeçenekleri, uplot_rs::HizalıVeri), UplotHatası>;
type KatalogKartFabrikası = fn(KatalogFabrikaGirdisi) -> KatalogKartÇıktısı;
type KatalogVaryantSlugÇözücü = fn(&str) -> Option<KartKimliği>;

#[derive(Clone, Copy)]
struct KatalogKartTanımı {
    kimlik: KartKimliği,
    slug: &'static str,
    başlık: &'static str,
    kaynak: &'static str,
    açıklama: Option<&'static str>,
    tanım: &'static str,
    tanım_yolu: &'static str,
    grup: KatalogKartGrubu,
    varyant_grubu: Option<&'static str>,
    eski_sluglar: &'static [&'static str],
    varyant_slugdan: KatalogVaryantSlugÇözücü,
    fabrika: KatalogKartFabrikası,
}

#[derive(Clone, Copy)]
struct KatalogFabrikaGirdisi {
    kart: KartKimliği,
    no_data_örneği: NoDataÖrneği,
    nokta_sayısı: usize,
    autosize_kuvvet: i32,
    latency_kova: u8,
    latency_ofset: u8,
    pixel_align_adımı: usize,
}

macro_rules! katalog_kartı {
    (
        $kimlik:expr,
        $slug:expr,
        $başlık:expr,
        $kaynak:expr,
        $açıklama:expr,
        $tanım:expr,
        $tanım_yolu:expr,
        $grup:expr,
        $varyant_grubu:expr,
        $eski_sluglar:expr,
        $varyant_slugdan:expr,
        $fabrika:expr $(,)?
    ) => {
        KatalogKartTanımı {
            kimlik: $kimlik,
            slug: $slug,
            başlık: $başlık,
            kaynak: $kaynak,
            açıklama: $açıklama,
            tanım: $tanım,
            tanım_yolu: $tanım_yolu,
            grup: $grup,
            varyant_grubu: $varyant_grubu,
            eski_sluglar: $eski_sluglar,
            varyant_slugdan: $varyant_slugdan,
            fabrika: $fabrika,
        }
    };
}

fn yanlış_kart_fabrikası(kart: KartKimliği) -> KatalogKartÇıktısı {
    Err(UplotHatası::BilinmeyenKart {
        kimlik: format!("{kart:?}"),
    })
}

fn varyant_slugı_yok(_slug: &str) -> Option<KartKimliği> {
    None
}

fn multi_bars_slugdan(slug: &str) -> Option<KartKimliği> {
    MultiBarsÖrneği::TÜMÜ
        .into_iter()
        .find(|örnek| örnek.kimlik() == slug)
        .map(KartKimliği::MultiBars)
}

/// Yan menü, native ve web derin bağlantıları ile grafik üretiminin tek kayıt
/// defteri. İlişkili yüzeyler ve girdi varyantları burada yalnız bir kez yer alır.
const KATALOG_KARTLARI: &[KatalogKartTanımı] = &[
    katalog_kartı!(
        KartKimliği::AddDelSeries,
        "add-del-series",
        "Add/Delete Series",
        "add-del-series.html · addSeries/delSeries/setData · kaynak Y indeksi 1",
        Some(
            "Amaç: aynı grafik örneğinde çalışma zamanında seri ekleme/silme, hizalı veri  sütunlarını koruma ve setData ölçek sıfırlamasını gösterir. API: Grafik::seri_ekle  ve seri_sil doğrulanmış işlemlerdir; SeriYaşamDöngüsüOlayı X'i sayan resmî  seriesIdx ile addSeries/delSeries olayını setData olayından önce taşır. İlk  ekleme kaynak turuncusudur; sonraki eklemeler geliştiricinin serileri ayırt  edebilmesi için belirlenimci paletten renk alır. İzleme: çalışan bir panele yeni  sensör, CPU veya metrik eklerken grafik ve etkileşim kimliğini korumak için  uygundur. Maliyet: sütun üretimi O(N), hizalı yapı doğrulaması ve yeniden çizim  O(N×S)'dir; GPUI Entity ve yüzey kimliği değişmez.",
        ),
        ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/add_del_series.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::AddDelSeries => add_del_series_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::HighLowBands,
        "high-low-bands",
        "High/Low Bands · 12 related surfaces",
        "high-low-bands.html · 12 ilişkili line/step/spline/bar yüzeyi",
        Some(
            "Amaç: resmî high-low-bands.html sayfasındaki 12 bağımsız yüzeyi kaynak  sırasıyla birlikte gösterir; farklı path builder, yönlü/kesişen bant, null  koşusu, unaligned ve milisaniyelik ince bar farkları aynı sayfada görülebilir.  API: high_low_bands_kartları tüm yüzeyleri tek grupta döndürür; Differing  Paths/Bars, inverted lines/bars ve iki unaligned yüzey immutable HizalıVeri  depolarını paylaşır. SeriBandı yön ve dolgu sınırlarını; SeriSeçenekleri point  görünürlüğü, bar genişliği, azami genişlik ve değer ucu yarıçapını taşır.  İzleme: min/max/ortalama sıcaklık, güven aralığı ve alt-üst telemetri  sınırlarını boşlukları yanlış köprülemeden okumak içindir. Maliyet: örneklenen  eğri bant dilimleri komşu dörtgenler yerine sürekli çokgen koşularında  birleştirilir; pointer yalnız etkin yüzeyin hafif cursor/lejant katmanını  günceller, ana geometri yalnız veri/ölçek/boyut/görünürlük değişiminde çizilir.",
        ),
        HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/high_low_bands.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::HighLowBands => high_low_bands_kartı(HighLowBandsÖrneği::YıllıkSıcaklık),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::LatencyHeatmap,
        "latency-heatmap",
        "Latency Heatmap · 5 related surfaces",
        "latency-heatmap.html · rand.js · draw hook, mode-2 ve histogram kovaları",
        Some(
            "Amaç: resmî latency-heatmap.html sayfasındaki ham olay yoğunluğu, 5 ms  kovalanmış yoğunluk, mode-2 facet hücreleri ve iki histogram path ayarını  kaynak sırasıyla birlikte gösterir. API: latency_heatmap_kartları beş  bağımsız Grafik döndürür; raw/aggregate aynı immutable min-max HizalıVeri  deposunu, iki histogram ilk snapshot'ı paylaşır. IsıHücresi piksel/veri  boyutunu ve kaynak piksel ofsetini; histogram seri seçeneği align=1 ile sabit  0/3 CSS piksel gap'i taşır. Slider yalnız collapsed histogramda aynı grafik  örneğine setData uygular; gapped snapshot değişmez. İzleme: istek gecikmesi,  trace süresi ve olay yoğunluğunu zaman dağılımı ile toplu histogram arasında  karşılaştırmak içindir. Maliyet: 34.110 ham hücre tek dev yol yerine en çok  1.024 hücrelik retained path parçalarında boyanır; wheel sırasında yalnız  görünür ölçek geometrisi yeniden çözülür ve pointer hafif katmanda kalır.",
        ),
        LATENCY_HEATMAP_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/latency_heatmap.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::LatencyHeatmap => latency_heatmap_kartı(
                LatencyHeatmapÖrneği::Ham,
                f64::from(girdi.latency_kova),
                f64::from(girdi.latency_ofset),
            ),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::LinePaths,
        "line-paths",
        "Line Paths · 8 synced surfaces",
        "line-paths.html · 8 cursor-synced null/linear/spline/stepped/bars surfaces",
        Some(
            "Amaç: resmî line-paths.html sayfasındaki points-only, linear, monotone  cubic, iki stepped ve üç bar path builder sonucunu aynı veri ve aynı  etkileşim bağlamında karşılaştırır. API: line_paths_kartları sekiz Grafik  tanımını kaynak sırasıyla, tek Arc-backed immutable 101 noktalı veri deposuyla  döndürür; 22..25 arası gerçek null koşusu bütün yol türlerinde aynı girdidir.  Masaüstü grup adaptörü kaynak cursor.sync.key=0 ilişkisini veri X/Y değerleriyle  sekiz retained yüzeye taşır; seçim, wheel, pan ve görünüm geçmişi yüzey başına  bağımsız kalır. İzleme: aynı telemetri dizisinde boşluk, nokta, eğri, basamak  ve hizalı bar sunumunun operasyonel farkını tek sayfada görmeye uygundur.  Maliyet: veri sekiz kez kopyalanmaz; pointer ana yol geometrisini yeniden  üretmeden hafif cursor katmanlarını günceller. Ana sahne yalnız veri, görünür  ölçek, seri seçimi veya boyut değiştiğinde yeniden boyanır.",
        ),
        LINE_PATHS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/line_paths.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::LinePaths => line_paths_kartı(LinePathsÖrneği::YalnızNoktalar),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::LogScales,
        "log-scales",
        "Log Scales · 2 independent surfaces",
        "log-scales.html · 12 Minecraft sunucusu · log10 ve doğrusal Y ölçeği",
        Some(
            "Amaç: aynı 1.440 zaman damgası ve 12 Minecraft sunucu serisini log10 ve  doğrusal Y ölçeklerinde kaynak sırasıyla karşılaştırır. API:  log_scales_kartları iki bağımsız Grafik tanımını tek Arc-backed HizalıVeri  deposuyla döndürür; kaynak sıfırları log güvenliği için bir kez 1'e çevrilir.  Y eksenleri kaynak axis.size=60 ve axis.space=15 geometrisini kullanır.  Çekirdek log10 bölmelerini her büyüklükte 1..9 üretir, yalnız etiketleri  kullanılabilir piksel alanına göre all / 12357 / 125 / 1 kümelerine  seyreltir. İzleme: yüksek dinamik aralıklı oyuncu, istek veya kaynak  telemetrisinde hem oran değişimini hem mutlak farkı yan yana okumak içindir.  Maliyet: veri iki kez çözülmez veya kopyalanmaz; cursor ve zoom kaynak gibi  bağımsızdır, pointer yalnız etkin yüzeyin hafif katmanını günceller.",
        ),
        LOG_SCALES_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/log_scales.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::LogScales => log_scales_kartı(LogScalesÖrneği::Logaritmik),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::LogScales2,
        "log-scales2",
        "Log Scales 2 · 12 source surfaces",
        "log-scales2.html · log2/log10, ters yön, null ve kısmi büyüklükler",
        Some(
            "Amaç: resmî log-scales2.html içindeki doğrusal, log10, log2, ters  yön, pozitif filtre, skip-tick, tümü-null, çok küçük ve kısmi büyüklük  durumlarını kaynak sırasıyla tek sayfada gösterir. API:  log_scales2_kartları on iki Grafik tanımını döndürür; ilk üç yüzey aynı  127 noktalı Arc-backed veriyi, In/Out çifti aynı dört zaman noktasını  paylaşır. İlk dört geniş yüzey kaynak axis.size=80 geometrisindedir.  In/Out çifti cursor.sync.key=\"moo\" karşılığı olarak X cursor ve görünümünü  eşler; yatay cursor kapalıdır, üst X ekseni gizlidir ve iki seri birleşik  kompozisyon olarak sunulur. İzleme: çok geniş değer aralıklarının eksen  okunabilirliği, ters giriş/çıkış akışı ve eksik/null telemetri köşelerini  değerlendirmek içindir. Maliyet: ortak veriler yeniden üretilmez; log  ızgarası tüm 1..9 bölmelerini üretirken metinler piksel yoğunluğuna göre  seyreltilir. Pointer ana yolları yeniden çizmeden yalnız etkin veya bağlı  In/Out cursor katmanlarını günceller.",
        ),
        LOG_SCALES2_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/log_scales2.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::LogScales2 => log_scales2_kartı(LogScales2Örneği::GenişDoğrusal),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::MassSpectrum,
        "mass-spectrum",
        "Mass Spectrum",
        "mass-spectrum.html · 41.986 kaynak CSV noktası · özel düz Y aralığı",
        Some(
            "41.986 kütle spektrumu örneğinde görünür X dilimini ve retained yoğun yol performansını gösterir."
        ),
        MASS_SPECTRUM_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/mass_spectrum.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::MassSpectrum => mass_spectrum_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::MeasureDatums,
        "measure-datums",
        "Measure / Datums",
        "measure-datums.html · 1/2 datum · Esc temizle",
        Some(
            "Bir veya iki datum seçerek ölçüm aralığı oluşturma ve Escape ile temizleme davranışını gösterir."
        ),
        MEASURE_DATUMS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/measure_datums.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::MeasureDatums => measure_datums_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::MultiBars(MultiBarsÖrneği::KitaplıklarDikey),
        "multi-bars",
        "Multi Bars · 4 varyant",
        "multi-bars.html · benchmark grupları · negatif ve durum renkli çubuklar",
        Some(
            "Aynı kaynak sayfasındaki dört Multi Bars varyantını tek kartta; grup ölçekleri, değer etiketleri, renkler ve isteğe bağlı çizgiyle karşılaştırır."
        ),
        MULTI_BARS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/multi_bars.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("multi-bars"),
        &[],
        multi_bars_slugdan,
        |girdi| match girdi.kart {
            KartKimliği::MultiBars(örnek) => multi_bars_kartı(örnek),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::NearestNonNull,
        "nearest-non-null",
        "Nearest Non-Null · 5 davranış",
        "nearest-non-null.html · 5 bağımsız yüzeyde null/proximity/cursor karşılaştırması",
        Some(
            "Null boşluklarında en yakın gerçek örneğin beş farklı proximity/cursor politikasıyla nasıl seçildiğini karşılaştırır."
        ),
        NEAREST_NON_NULL_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/nearest_non_null.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::NearestNonNull => {
                nearest_non_null_kartı(NearestNonNullÖrneği::XDeğerineGöre)
            }
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::FocusCursor,
        "focus-cursor",
        "Focus Cursor · 4 related surfaces",
        "focus-cursor.html · cursor.focus + setSeries",
        Some(
            "Amaç: resmî focus-cursor.html sayfasındaki bias, 30 px proximity, dinamik  setSeries stili ve 300 seri performans yüzeylerini aynı sayfada kaynak sırasıyla  karşılaştırır. API: focus_cursor_kartları dört bağımsız Grafik döndürür; ilk iki  yüzey aynı immutable HizalıVeri Arc deposunu paylaşır. seri_odak_sunumu,  odak değiştiğinde yalnız stroke/fill/width boya sonucunu verir. İzleme: yoğun  CPU/RAM zaman serilerinde imlece en yakın seriyi ayrıntılandırıp diğerlerini  soluklaştırmak için uygundur. Maliyet: 130K veri ikinci kez tahsis edilmez; GPUI  retained ana yolları korur, pointer yalnız etkileşim katmanı ile seri boya  durumunu günceller. Ana geometri ancak veri, ölçek, resize veya zoom değişince  yeniden kurulur.",
        ),
        FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/focus_cursor.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::FocusCursor => focus_cursor_kartı(FocusÖrneği::İmleç),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::Gradients,
        "gradients",
        "Gradients · 5 related surfaces",
        "gradients.html · scaleGradient + cursor point colors",
        Some(
            "Amaç: resmî gradients.html sayfasındaki yatay/dikey ayrık stroke, ArcSinh  koordinatı, iki basınç dolgusu ve görünür min/orta/max dolgusunu kaynak  sırasıyla tek kartta karşılaştırır. API: gradients_kartları beş bağımsız  Grafik döndürür; dikey çift aynı data2, dolgu çifti aynı data4 HizalıVeri Arc  deposunu paylaşır. ÖlçekGradyanı değer, ±sonsuz ve görünür_veri_oranı  duraklarını; seri_imleç_rengi cursor point callback sonucunu taşır. İzleme:  eşik bölgelerini çizgi/dolgu rengiyle vurgulamak ve görünür pencerenin basınç  dağılımını okumak için uygundur. Maliyet: veri kopyalanmaz; pointer yalnız  etkin yüzeyin cursor/lejant katmanını günceller. Gradyan ve ana geometri yalnız  veri, ölçek, görünürlük, zoom/pan veya boyut değişiminde yeniden çözülür.",
        ),
        GRADIENTS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/gradients.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::Gradients => gradients_kartı(GradientÖrneği::YatayÇizgi),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::GridOverSeries,
        "grid-over-series",
        "Grid Over Series",
        "grid-over-series.html · drawOrder: series, axes",
        Some(
            "Amaç: üç opak dolgulu serinin kesişimlerinde ızgara, çentik ve eksen bilgisini seri boyasının altında kaybetmeden gösterir. API: GrafikSeçenekleri::katman_sırası, özel bir kart tanımı olmadan arka plan, veri, ızgara/eksen ve bilgi katmanlarının tamamını geliştiricinin sıralamasını sağlar; ızgara, X/Y çentik ve eksen/etiket renkleri CSS olmadan ayrı ayrı ayarlanabilir. Otomatik Y aralığı görünür X verisinden yeniden hesaplanır. İzleme: yoğun ve üst üste binen CPU, bellek veya ağ alanlarında ortak eşik düzlemini her serinin üzerinde okunabilir tutmak için uygundur. Maliyet: üç 30 noktalı seri retained veri yüzeyinde, ızgara ise bağımsız retained katmanda çizilir; pointer yalnız hafif bilgi katmanını günceller.",
        ),
        GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/grid_over_series.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::GridOverSeries => grid_over_series_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::TimezonesDst,
        "timezones-dst",
        "Timezones & DST",
        "timezones-dst.html · tzDate · 51 etkin UTC/London/Chicago yüzeyi",
        Some(
            "Aynı UTC veri aralığını farklı saat dilimleri ve DST geçişleriyle ilişkili yüzeyler halinde gösterir."
        ),
        TIMEZONES_DST_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/timezones_dst.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::TimezonesDst => {
                let örnek =
                    TimezonesDstÖrneği::yeni(0).ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
                timezones_dst_kartı(örnek)
            }
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::TooltipsClosest,
        "tooltips-closest",
        "Summary-opt",
        "tooltips-closest.html · rustc-perf.json · en yakın seri ve commit karşılaştırması",
        Some(
            "Amaç: dört rustc-perf çalışma kipinden imlece beş CSS piksel içinde en yakın  görünür seriyi bulur; commit, değer ve başlangıca göre değişimi gerçek veri  noktasına bağlı tek kutuda gösterir. API: OdakDüzeni yakınlık/alfa kararını,  EnYakınTooltipDüzeni commit dizisini, 100 interpolasyon indeksini ve perf URL  istatistiğini çekirdekte tutar; lejant setSeries, alan seçimi X+Y çalışır.  İzleme: derleyici, servis veya sürüm regresyonunda aynı commit anındaki çalışma  kiplerini karşılaştırmak için uygundur; yerinde plot tıklaması karşılaştırma  bağlantısını açar, sürükleme bağlantı açmaz. Maliyet: 234×4 çizgi noktası ve  100 dikey kılavuz vardır; kılavuzlar tek path komutunda boyanır, pointer araması  O(log N + görünür seri sayısı) ve kutu ana yolları yeniden çizmeden taşınır.  Tarih metni platformlar arası belirlenim için UTC'dir; kaynak browser-local  Date kullandığından bu bilinçli, belgeli tek sunum farkıdır.",
        ),
        TOOLTIPS_CLOSEST_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/tooltips_closest.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::TooltipsClosest => tooltips_closest_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::Tooltips,
        "tooltips",
        "Tooltips",
        "tooltips.html · imleç ve görünür seri kutuları · 2 sn imleç durum koruması",
        Some(
            "Amaç: ham imleç X/Y konumu ile en yakın veri indeksindeki görünür seri  noktalarını ayrı, hafif bilgi kutularında gösterir; kaynak örneğin her iki  saniyede destroy/new uPlot yaşam döngüsünde imleç konumunu koruma sınamasını  sürdürür. API: TooltipDüzeni imleç ve seri kutularını, yeniden_kurma_ms yaşam  döngüsünü ve dış cursor memo kararını tanımlar; lejant setSeries ile One ve  varsayılan gizli Two serisini aynı yüzeyde açıp kapatır. İzleme: cursor  konumu ile örneklenmiş ölçümün farklı olduğunu geliştiriciye açıkça göstermek  ve panel yeniden kurulurken inceleme bağlamının kaybolmamasını sınamak için  uygundur. Maliyet: veri yalnız 7×2'dir; ana yollar yalnız setSeries, ölçek veya  kasıtlı iki saniyelik kaynak yeniden kurulumunda boyanır. Normal pointer  hareketi önbellekli ana yüzeye dokunmaz, yalnız mevcut tooltip katmanlarını  ve cursor çizgilerini taşır.",
        ),
        TOOLTIPS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/tooltips.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::Tooltips => tooltips_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::Trendlines,
        "trendlines",
        "Trendlines",
        "trendlines.html · drawSeries uç trendleri · veri değerlerine yapışan X aralığı",
        Some(
            "Amaç: her görünür serinin ekrandaki ilk ve son gerçek veri indeksini kaynak  drawSeries kancası gibi 5/5 kesik bir uç çizgisiyle bağlar; normal path'in  kırpma için görünüm dışı komşu noktaları kullanabilmesi bu i0/i1 kararını  değiştirmez. API: ÇizimKancasıDüzeni::seri_uç_trendleri kesik aralığını,  x_aralığını_veriye_yapıştır ise seçim ve wheel uçlarının valToIdx eşdeğeriyle  gerçek X değerlerine oturmasını sağlar; lejant setSeries ana yol, dolgu ve  trendi birlikte açıp kapatır. İzleme: seçili zaman penceresindeki genel  başlangıç-son eğilimini ham dalgalanmanın üzerinde okumak için uygundur;  regresyon değildir ve ara noktaları modellemez. Maliyet: iki 100 noktalı yol  ve seri başına tek ek çizgi O(görünür N)'dir. Pointer yalnız cursor/lejant  katmanını taşır; uçlar yalnız ölçek, resize veya setSeries sonrasında yeniden  hesaplanır. Kaynak points.space=10 ve tek-piksel yarım-piksel hizası korunur.",
        ),
        TRENDLINES_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/trendlines.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::Trendlines => trendlines_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::UpdateCursorSelectResize,
        "update-cursor-select-resize",
        "Maintain loc of cursor/select/hoverPts",
        "update-cursor-select-resize.html · setSize sırasında seçim, kilitli imleç ve hover noktası oranları",
        Some(
            "Amaç: setCursor, cursor._lock ve setSelect ile kurulmuş kalıcı etkileşim  durumunun setSize sırasında çizim alanı oranlarında kalmasını gösterir. API:  BoyutSenkronDüzeni yalnız başlangıç cursor/select/hover oranlarını taşır;  Grafik::boyutu_ayarla veri ve ölçeği koruyarak ana sahneyi yeniden boyar. Native  ve web GPUI adaptörleri ana veri sahnesinden ayrı hafif etkileşim katmanında  durumu saklar. Lejant  setSeries kırmızı yolu ve hover noktasını birlikte gizler. İzleme: panel veya  pencere boyutu değişirken kullanıcının kilitli inceleme konumunu kaybetmemek  içindir. Maliyet: kaynak gibi setSize ana yolları yeniden çizer; cursor, seçim  ve hover için ikinci bir ana yol üretmez, yalnız hafif katman koordinatları  güncellenir. 100 ms zamanlayıcı karttan çıkıldığında durdurulur.",
        ),
        UPDATE_CURSOR_SELECT_RESIZE_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/update_cursor_select_resize.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::UpdateCursorSelectResize => update_cursor_select_resize_kartı(800),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::WindDirection,
        "wind-direction",
        "Wind Direction",
        "wind-direction.html · 143 saatlik kaynak veri · 15 px özel yön vektörleri",
        Some(
            "Amaç: sıcaklık çizgisini ve sabit 0…30 m/s ölçekli rüzgâr hızını aynı hizalı  zaman dizisinde gösterir; üçüncü seri, hız konumlarından derece yönüne uzanan  15 CSS piksellik vektörleri özel path olarak üretir. API:  RüzgarYönüDüzeni::yeni hız/yön serisini ve ölçeği bağlar; stil ile vektör  uzunluğu, rengi ve kalınlığı CSS olmadan geliştirici tarafından seçilebilir.  Direction serisinin auto=false kararı dereceleri Y aralığından çıkarır;  lejant setSeries ile üç katman bağımsız açılıp kapanır. İzleme: sıcaklık,  rüzgâr hızı ve yönü gibi aynı zamanlı fakat farklı birimli telemetriyi tek  inceleme yüzeyinde ilişkilendirmek içindir. Maliyet: 139 vektör kaynak gibi  tek beginPath/stroke eşdeğeri Yol komutunda toplu boyanır; görünüm sınırındaki  dış komşular getOuterIdxs eşdeğeriyle korunur. Pointer yalnız hafif cursor  katmanını taşır; ana yollar setSeries, ölçek veya resize ile yeniden hesaplanır.",
        ),
        WIND_DIRECTION_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/wind_direction.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::WindDirection => wind_direction_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::YScaleDrag,
        "y-scale-drag",
        "Draggable x & y scales",
        "y-scale-drag.html · bağımsız X/Y eksen sürükleme · Shift ile büyüt/daralt",
        Some(
            "Amaç: sayısal X ile meter ve km/h adlı iki bağımsız Y ölçeğini doğrudan eksen  üzerinden kaydırır; Shift basılıyken iki uç ters yönde hareket ederek aralığı  büyütür veya daraltır. API: eksen_vuruşu_boyutta gerçek çizim payından hedef  ölçeği seçer; eksen_sürüklemeyi_başlat/sürükle/bitir kaynak setScale yaşam  döngüsünü taşır. Otomatik Y ekseni hesabı kaynak callback'indeki  25 + en_uzun_etiket × 6 piksel formülünü her aralıkta yeniden uygular; lejant  setSeries ilgili elle sürüklenmiş ölçeği otomatik aralığa döndürür. İzleme:  farklı birimli metriklerin ayrıntı düzeyini paneli yeniden kurmadan ayrı ayrı  ayarlamak için uygundur. Maliyet: hareketler ekran karesiyle birleştirilir;  setScale eksen, grid ve iki kısa yolu yeniden boyar, cursor katmanı yerinde  kalır. GPUI Web pointer capture, native GPUI dışarıda mouse-up temizliğiyle sürüklemeyi  yüzey sınırının dışında da güvenle tamamlar.",
        ),
        Y_SCALE_DRAG_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/y_scale_drag.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::YScaleDrag => y_scale_drag_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::YShiftedSeries,
        "y-shifted-series",
        "Y-shifted Series",
        "y-shifted-series.html · aynı ham veriyle 2 sn normal/kaydırılmış kip",
        Some(
            "Amaç: aynı 30×3 ham ölçümü iki saniyede bir normal 0…10 düzlemi ile  Core #1/#2/#3 için 0/+10/+20 kaydırılmış şerit düzlemi arasında değiştirir.  Kırmızı ve yeşil alanların fillTo tabanları 0/10, mavi bars Path2D tabanı  20'dir; lejant series.value gibi her zaman ham 0…10 değerini gösterirken  hover noktası gerçek kaydırılmış geometride kalır. API:  YShiftedSeriesAkışı::ilerlet_güncellemesi yalnız yeni veri, range, axis values  ve fillTo tabanlarını üretir; Grafik::veriyi_y_sunumunda_ayarla aynı Grafik  örneğinde atomik setData uygular. Lejant setSeries görünürlüğü kip geçişinde  korunur. İzleme: aynı ölçekli çekirdek, pod veya kuyruk metriklerini üst üste  binmeden ayrı şeritlerde izleyip ham değerlerini karşılaştırmak içindir.  Maliyet: seçenek ağacı, GPUI entity'si, retained sahne ve etkileşim bağları yeniden  kurulmaz; 30 mavi çubuk tek dolgu ve tek stroke yolunda toplanır. Timer karttan  çıkıldığında iptal edilir, cursor hafif katmanda aynı konumdan yeniden çözülür.",
        ),
        Y_SHIFTED_SERIES_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/y_shifted_series.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::YShiftedSeries => y_shifted_series_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::AlignDataCost,
        "align-data",
        "Align Data · 2 related surfaces",
        "align-data.html · NULL_EXPAND maliyeti + aligned line/bars",
        Some(
            "Hizalama maliyeti ile aynı hizalı verinin line ve bars sunumlarını tek kaynak bağlamında karşılaştırır."
        ),
        ALIGN_DATA_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/align_data.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &["align-data-cost"],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::AlignDataCost => align_data_maliyet_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::Resize,
        "resize",
        "Resize · sayısal x ölçeği",
        "resize.html + zoom-wheel.html + zoom-touch.html",
        Some(
            "Duyarlı GPUI yüzeyinde sayısal X ölçeği, görünür dilim ve piksel-kova çizgisinin boyut değişiminde korunmasını gösterir."
        ),
        RESIZE_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/resize.rs",
        KatalogKartGrubu::Tek,
        None,
        &["line-resize"],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::Resize => resize_kartı(girdi.nokta_sayısı),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::Annotations,
        "annotations",
        "Annotations",
        "annotations.html · X çizgisi/aralığı · üst/alt etiket · görünürlük kırpması",
        Some(
            "X çizgisi ve aralık annotationlarının etiket, kırpma ve görünürlük davranışını gösterir."
        ),
        ANNOTATIONS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/annotations.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::Annotations => annotations_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::AreaFill,
        "area-fill",
        "Area Fill",
        "area-fill.html · kaynakla aynı veri üreteci · ortak Resize etkileşim profili",
        Some(
            "Üç serinin kaynakla aynı veri üretimi, çizgi ve alan dolgularını ortak ölçekte gösterir."
        ),
        AREA_FILL_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/area_fill.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::AreaFill => area_fill_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::ScalePadding,
        "scale-padding",
        "Scale Padding · Flat",
        "scale-padding.html · 13 düz seri · kaynakla aynı değer düzeyleri",
        Some(
            "Amaç: farklı büyüklüklerdeki düz eşik ve taban çizgilerini tek Y ölçeğinde  uçlara değmeden gösterir; kaynak rangeNum hesabı %10 payı dışa doğru uygun  artıma yapıştırarak −13000…13000 üretir. API: YÖlçekSeçenekleri::sayısal_aralık  alt/üst payı ve soft sınır kipini tanımlar; okunabilirliği ayrılması gereken  metrik aileleri adlandırılmış farklı ölçeklere atanabilir. İzleme: alarm ve  kapasite eşikleri için uygundur; ±0.1 ile ±10500 aynı ölçekteyse küçük değerlerin  sıfıra yakın görünmesi doğrudur. Maliyet: 13×10 hizalı nokta O(S×N), imleç  O(log N + S); cursor ve lejant ana yolları yeniden üretmeden güncellenir.",
        ),
        SCALE_PADDING_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/scale_padding.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::ScalePadding => scale_padding_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::Months,
        "months",
        "Months · calendar ticks",
        "months.html · 2 kaynak yüzeyi · UTC ayları ve artık yıl · sabit kanıt tohumu",
        Some(
            "Amaç: gerçek UTC ay sınırlarını normal ve artık yıllarda karşılaştırır. API:  x tarih ölçeği ve kaynak 28 günlük axes.space karşılığı takvim-ay bölmelerini  belirler. İzleme:  aylık faturalama, SLO ve kapasite raporlarında sabit 30 gün yerine gerçek ay  sınırlarını kullanın. Maliyet: iki bağımsız yüzeyde toplam 72 nokta; çizim  O(N+T), imleç O(log N). Resize bölmeleri yeniden hesaplar, veriyi üretmez.",
        ),
        MONTHS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/months.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::Months => months_artık_yılsız_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::MonthsRussian,
        "months-ru",
        "Months · Russian locale",
        "months-ru.html · tek 1920×600 yüzey · UTC ayları ve Rusça fmtDate",
        Some(
            "Amaç: 36 UTC ay başlangıcını veri saat dilimini değiştirmeden Rusça eksen  adlarıyla sunar. API: months_rusça_kartı tek 1920×600 Grafik tanımını ve  TarihAdları::rusça formatter sözlüğünü döndürür; Y aralığı gelen değerlerden  otomatik türetilir. İzleme: aylık telemetri ve kapasite panellerinde depolama  zamanını UTC tutup locale'i yalnız sunum katmanında uygulayın. Maliyet: tek  yüzey 36 nokta taşır; çizim O(N+T), imleç O(log N).",
        ),
        MONTHS_RU_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/months.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::MonthsRussian => months_rusça_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::NiceScale,
        "nice-scale",
        "Nice Scale & Ticks",
        "nice-scale.html · pencere/panel boyutuna bağlı niceScale/niceNum Y aralığı ve artımı",
        Some(
            "Amaç: panel yüksekliğine sığan okunabilir Y bölmelerini ve bu bölmelere tam  oturan yuvarlak sınırları otomatik seçer. API: GüzelÖlçekDüzeni::yeni(30.0),  kaynak niceNum eşiklerini (1/2/2,5/5/10), uçlarda %2 payı ve ArtımaGöre  etiket biçimini birleştirir. İzleme: pencere veya panel boyutu değiştiğinde  sabit tick sayısı yerine en az 30 piksel aralık korunur. Maliyet: altı X  noktası ve üç seri değişmeden kalır; yalnız ölçek, ızgara ve yollar  O(S×N+T) maliyetle yeniden boyanır.",
        ),
        NICE_SCALE_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/nice_scale.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::NiceScale => nice_scale_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::NoData,
        "no-data",
        "No Data · 33 seçenek",
        "no-data.html · tek kartta 33 boş, tek noktalı, düz ve hassas ölçek seçeneği",
        Some(
            "Amaç: boş veri, tek nokta, neredeyse düz ve tam düz serilerde otomatik  sayısal aralığın kararlı kalmasını karşılaştırır. API: NoDataÖrneği::TÜMÜ  kaynak 33 durumu tipli seçenekler olarak sunar; no_data_kartı seçili durumun  zaman kipini, özel boş aralıklarını ve rangeNum eşdeğerini kurar. İzleme:  veri gelmeden önce anlamlı bir aralık; tek veya sabit değer geldiğinde sıfır  genişlikli olmayan güvenli bir ölçek gösterin. Maliyet: 33 eşzamanlı yüzey  yerine seçili tanım aynı GPUI yüzeyinde değiştirilir ve yalnız eksenlerle en  fazla iki nokta yeniden kurulur.",
        ),
        NO_DATA_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/no_data.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::NoData => no_data_kartı(girdi.no_data_örneği),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::PathGapClip,
        "path-gap-clip",
        "Path & Gap Clipping · 15 yüzey",
        "path-gap-clip.html · 15 null/undefined, band, stepped ve piksel yüzeyi",
        Some(
            "Amaç: gerçek null, join sırasında oluşan undefined/hizalama eksiği, band kırpması,  stepped before/after ve tek-piksel gap sınırlarını kaynak sayfadaki 15 yüzeyle  karşılaştırır. API: HizalıDeğer::{Değer, Boş, Tanımsız}, NULL_RETAIN/NULL_EXPAND  join kipleri, linear/stepped/spline yolları ve spanGaps mutasyonu çekirdekte  tanımlıdır; kaynakta setData/setScale yoktur. İzleme: scrape eksiğini gerçek  ölçüm null'u gibi boyamayın; bridge açıldığında çizginin yalnız görsel olarak  bağlandığını kullanıcıya belirtin. Maliyet: path/gap taraması O(N), sıralı imleç  O(log N); pointer yalnız hafif overlay'i günceller, bir saniyelik animasyon yalnız  dört kaynak yüzeyin ana yollarını yeniden kurar.",
        ),
        PATH_GAP_CLIP_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/path_gap_clip.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::PathGapClip =>
                path_gap_clip_kartı(PathGapClipÖrneği::VeriDışınaTaşanÖlçek),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::PixelAlign,
        "pixel-align",
        "Pixel Align · canlı A/B",
        "pixel-align.html · boş başlayan 2 yüzey · tek 1 Hz halka veri + animation-frame X saati",
        Some(
            "Amaç: aynı canlı telemetriyi aynı kayan 120 saniyelik pencerede tam piksel ve  alt-piksel rasterizasyonuyla A/B karşılaştırır. API: grafik piksel_hizası eksen  ve grid varsayılanını, seri piksel_hizası path/point override'ını belirler;  PixelAlignAkışı 1 Hz örnek eklerken frame saati yalnız X ölçeğini ilerletir.  İzleme: hizalama veriyi değiştirmez; pxAlign=1 keskin ve hızlı fakat tırtıllı,  pxAlign=0 daha yumuşak fakat 1 px çizgilerde daha bulanık olabilir. Maliyet:  halka ekleme O(1), her frame çizim O(görünür N×S); grafik örnekleri yeniden  kurulmaz, yakınlaştırılmış görünüm canlı tam aralık ilerlerken sabit kalır.",
        ),
        PIXEL_ALIGN_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/pixel_align.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::PixelAlign => {
                pixel_align_kartı(PixelAlignÖrneği::Varsayılan, girdi.pixel_align_adımı)
            }
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::Points,
        "points",
        "Points · 4 yüzey",
        "points.html · 4 eşzamanlı yüzey · randomWalk.js · points.space, paths:null ve points.filter",
        Some(
            "Amaç: varsayılan nokta yoğunluğu, space=0 zorlaması, yalnız-nokta yolu ve  gerçek boşluklar arasındaki tekil ölçümleri tek kaynak sayfasında karşılaştırır.  API: points.space görünür piksel kapasitesini, paths:null yalnız marker çizimini,  NoktaFiltreKipi::BoşlukArasındakiTekiller ise path gap sınırlarından seçilen  indeksleri tanımlar. İzleme: seyrek olayları çizgiyle birleştirip süreklilik  izlenimi vermeden gösterin; yoğun telemetride marker'ları otomatik gizleyerek  ana eğriyi okunur tutun. Maliyet: dört statik yüzey toplam 3.321 X konumu tarar;  yoğunluk testi O(1), gap filtresi O(N+G×99), çizilen marker sayısı O(k).  Yakınlaştırma ve boyut değişimi filtreyi görünür piksel düzleminde yeniden hesaplar.",
        ),
        POINTS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/points.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::Points => points_kartı(PointsÖrneği::Karma),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::ScalesDirOri,
        "scales-dir-ori",
        "Scales Direction & Orientation · 16 yüzey",
        "scales-dir-ori.html · 16 eşzamanlı yüzey · scale.dir, scale.ori ve axis.side",
        Some(
            "Amaç: aynı iki serinin dört yön kombinasyonunu, karşı eksen taraflarını ve X/Y  yönelim değişimini tek matriste karşılaştırır. API: scale.dir veri yönünü,  scale.ori fiziksel eksen yönelimini, axis.side eksenin top/right/bottom/left  tarafını belirler. Direction Inversion sekiz 600×300; Orientation Inversion  sekiz 320×600 yüzeydir. İzleme: ters akan süreçleri veya dikey zaman eksenini  sunarken veri değerlerini dönüştürmeden fiziksel okumayı değiştirin. Maliyet:  16 statik yüzeyin her biri aynı 10 X konumu ve iki seriyi O(S×N) çizer; timer  yoktur. Cursor yalnız hafif etkileşim katmanlarını taşır; ölçek değişiminde  senkron grubun 16 ana yüzeyi birlikte yeniden boyanır.",
        ),
        SCALES_DIR_ORI_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/scales_dir_ori.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::ScalesDirOri =>
                scales_dir_ori_kartı(ScalesDirOriÖrneği::XArtıAltYArtıSol),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::Scatter,
        "scatter",
        "Scatter & Bubble · 2 bağımsız yüzey",
        "scatter.html · 2 bağımsız mode:2 yüzey · toplu scatter yolu ve uzamsal bubble vuruşu",
        Some(
            "Amaç: sabit boyutlu yoğun scatter ile üçüncü metriği alanla anlatan bubble  yaklaşımını aynı kaynak bağlamında karşılaştırır. API: mode:2 facet serileri  bağımsız X/Y dizileri taşır; bubble size/label facet'leri ve Region A için sağ  y2 ölçeği ekler. İki yüzey veri, cursor ve ölçek bakımından bağımsızdır. İzleme:  korelasyon kümeleri, kapasite/gelir ve nüfus yoğunluğu gibi çok boyutlu  telemetri için uygundur. Maliyet: 40.000 scatter noktası seri başına tek toplu  çizim komutuna iner; bubble hover yalnız ölçek veya boyut değişince yenilenen  uzamsal dizinin aday hücresini sorgular ve ana sahneyi yeniden boyamaz.",
        ),
        SCATTER_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/scatter.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::Scatter => scatter_kartı(ScatterÖrneği::Scatter),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::ScrollSync,
        "scroll-sync",
        "Scroll syncRect()",
        "scroll-sync.html · syncRect() · kaydırmada istemci/sahne eşlemesi",
        Some(
            "Amaç: kaydırılabilir panel içinde grafiğin pencere konumu değiştiğinde cursor,  seçim ve zoom koordinatlarının görsel noktadan kopmamasını gösterir. API:  adaptör güncel yüzey sınırını iletir; YüzeyDikdörtgeni istemci koordinatını  aspect-fit sahneye dönüştürür. İzleme: sanallaştırılmış liste, kayan dashboard,  sabit başlık veya yeniden yerleşen widget içindeki grafikler için gereklidir.  Maliyet: sınır yenileme tek yerleşim ölçümü ve O(1) dönüşümdür; kaydırma ana  veri sahnesini yeniden çizmez. Kaynak davranışını korumak için doğal kapsayıcı  kaydırması varsayılandır; wheel/touch eklentileri ortak API'den açılabilir.",
        ),
        SCROLL_SYNC_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/scroll_sync.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::ScrollSync => scroll_sync_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::SineStream,
        "sine-stream",
        "6 series x 600 points @ 60fps",
        "sine-stream.html · Box–Muller yürüyüşü · requestAnimationFrame",
        Some(
            "Amaç: tek grafik yüzeyinde 600 örnekli altı seriyi ekranın boya ritminde  kaydırarak canlı izleme yükünü gösterir. API: SineAkışı::ilerlet yalnız bir  örnek ilerletir; Grafik::veriyi_ayarla aynı Grafik ve GpuiGrafik örneğinde  uPlot setData ölçek sıfırlamasını uygular. İzleme: telemetri, log oranı ve  kaynak ölçümleri gibi sabit uzunluklu canlı pencereler için uygundur.  Başlıktaki 60 FPS kaynak adıdır; gerçek hız ekran yenileme hızıdır. Maliyet:  VecDeque pencere kaydırması O(1), veri aktarımı ve altı yolun çizimi  O(seri×600); sabit eksen/grid yolları önbellekte, cursor/seçim katmanı  güncellemeler arasında korunur.",
        ),
        SINE_STREAM_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/sine_stream.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::SineStream => sine_stream_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::SoftMinMax(SoftMinMaxÖrneği::MinKip0),
        "soft-minmax",
        "Soft Min/Max · 5 ilişkili yüzey",
        "soft-minmax.html · rangeNum soft/hard/pad/mode · kaynak dataMax++",
        Some(
            "Amaç: aynı iki noktalı verinin soft min mode 0/1/2/3 kararlarını yan yana  karşılaştırır; beşinci yüzey düz sıfır veride iki taraflı −1…1 soft sınırını  gösterir. API: soft_minmax_kartları tek kaynak sayfasının beş yüzeyini kaynak  sırasıyla kurar; SayısalAralıkParçası pad, soft ve mode alanlarını tipli  tanımlar. İzleme: sıfır tabanını sabit tutan oranlar ile küçük değişimlerde  dikey çözünürlüğü koruyan telemetri politikalarını seçmek için uygundur.  Maliyet: tek dataMax adımı yalnız ikişer noktalı dört grafiğe atomik setData  uygular; düz-sıfır yüzeyi değişmez. Tekrarlanan başlatmalar engellenir; bu,  kaynak örnekteki üst üste interval açabilme durumuna karşı kasıtlı güvenliktir.",
        ),
        SOFT_MINMAX_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/soft_minmax.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("soft-minmax"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::SoftMinMax(örnek) => soft_minmax_kartı(örnek, 12.0),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::SparklinesBars(SparklinesBarsÖrneği::GradyanÇubuklar),
        "sparklines-bars",
        "Sparkline + Floating Bars · 2 ilişkili yüzey",
        "sparklines-bars.html · sparkline + yüzen çubuklar + ölçek gradyanı",
        Some(
            "Amaç: aynı sparkline ve yüzen low/high çubuklarını yalnız renk stratejisini  değiştirerek kontrollü A/B karşılaştırır. API: sparklines_bars_kartları iki  yüzeyi birlikte kurar; Floating Bars low değerlerini,  yüzen_çubuk_üst_serisi özel high veri taşıyıcısını kullanır ve bu taşıyıcı  otomatik ölçeğe katılmaz. İzleme: pozitif/negatif bölgeleri kesen sapma ve  bütçe aralıklarında gradyan; kategorik eşiklerde açık nokta renkleri uygundur.  Kaynak cursor/select/legend kapalıdır; ortak wheel/touch/drag yalnız geliştirici  etkinleştirirse adaptör uzantısıdır. Maliyet: her yüzey 16 noktayı O(N) tarar;  gradyan tek toplu alan komutudur, açık renk yolu 16 dikdörtgen üretir.",
        ),
        SPARKLINES_BARS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/sparklines_bars.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("sparklines-bars"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::SparklinesBars(örnek) => sparklines_bars_kartı(örnek),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::Sparklines(SparklineÖrneği::İLK),
        "sparklines",
        "Sparklines · 10×2 tablo",
        "sparklines.html · kaynak CSV · 150×30 eksensiz kompakt yüzey",
        Some(
            "Amaç: yoğun bir izleme tablosunda 10 varlığın iki küçük zaman serisini tek  bakışta karşılaştırır; kaynak sayfanın ilişkili 20 yüzeyi ayrı katalog  kartlarına bölünmez. API: sparklines_kartları kaynak satır sırasıyla 20  (örnek, seçenekler, veri) üçlüsü döndürür; SparklineÖrneği::SATIRLAR  Hacim/Kapanış çiftlerini tanımlar ve her yüzey bağımsız  rangeNum(min,max,.1,true) Y aralığı kullanır. İzleme: hisse yerine servis,  pod veya sensör; sütunlara trafik, hata, gecikme ya da son değer konabilir.  Maliyet: kaynak Promise.all ile 10 CSV ve 20 canvas kurar; port doğrulanmış  440 değeri binary içine gömerek fetch/parser yaşam döngüsünü kaldırır.  Kaynak cursor/select/legend kapalıdır; ortak wheel/touch/drag yalnız  geliştirici etkinleştirirse çekirdek uzantısıdır.",
        ),
        SPARKLINES_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/sparklines.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("sparklines"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::Sparklines(örnek) => sparklines_kartı(örnek),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::Sparse(SparseÖrneği::YerleşikDoğrusal),
        "sparse",
        "Sparse · 3 pathBuilder",
        "sparse.html · sparse.json · yerleşik/özel nokta/saf moveTo yolları",
        Some(
            "Amaç: aynı seyrek telemetride optimize native linear, tek toplu özel kare  noktalar ve naif moveTo/lineTo yolunun görünüm ve maliyet farkını karşılaştırır.  API: sparse_kartları tek decode sonrası üç yüzeyi kaynak sırasıyla üretir;  saf_doğrusal_yol native piksel kovasını atlar, kare points tek Alan/Path2D  komutunda batch edilir. İzleme: uzun null koşularında native yol genel  seçimdir; olay yoğunluğunda points, algoritma kıyasında naive kullanılır.  Maliyet: native piksel başına giriş/min/max/çıkışı koruyup null koşularını  tek kırılmaya indirir; points 4.430 kareyi tek fill path'te taşır; naive  13.608 girdiyi tarayıp dolu noktalarla sınır kırpma kesişimlerini çizer.",
        ),
        SPARSE_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/sparse.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("sparse"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::Sparse(örnek) => sparse_kartı(örnek),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::StackedSeries(StackedSeriesÖrneği::Stacked1),
        "stacked-series",
        "Stacked Series · 16 yüzey",
        "stacked-series.html · stack.js · yığma, yüzde, grup ve karma veri",
        Some(
            "Amaç: tek kaynak sayfasındaki 16 bağımsız yüzeyi birlikte göstererek seri  sırasının algıya etkisini, normal/yüzde/gruplu yığmayı ve null/undefined/zero  ayrımını karşılaştırır. API: stacked_series_kartları kaynak DOM sırasıyla 16  (örnek, seçenekler, veri) üçlüsü döndürür; yalnız ilk dört yüzeyin lejant  görünürlüğü kaynak setSeries hook'u gibi bantları yeniden kurup aynı grafik  örneğinde setData uygular, kalan 12 yüzey yalnız görünürlüğü değiştirir.  İzleme: toplam kapasite bileşenleri, pozitif/negatif bütçeler ve eksik örnek  semantiğinin karşılaştırılması için uygundur; ilişkili varyasyonlar ayrı  katalog kartlarına bölünmez. Maliyet: başlangıç aralıkları kaynak  rangeNum(min,max,.1,true) ile sabittir; lejant güncellemesi yüzeyi yeniden  yaratmaz. Kaynak yüzeyler arasında cursor/ölçek senkronu yoktur ve port da  onları bağımsız tutar. Rastgele çubuk verisi tekrarlanabilir test için  belgelenmiş tohuma bağlanır.",
        ),
        STACKED_SERIES_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/stacked_series.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("stacked-series"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::StackedSeries(örnek) => stacked_series_kartı(örnek),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::StreamData(StreamDataÖrneği::SabitUzunluk),
        "stream-data",
        "Data Stream · 3 yüzey",
        "stream-data.html · bench/data.json · setData canlı akışı",
        Some(
            "Amaç: sabit uzunlukta kayan pencere, sürekli büyüyen veri ve sabit ölçekli  büyüyen veri akışlarını aynı kaynak bağlamında karşılaştırır. API:  StreamDataGrubu tek decode edilmiş Arc kaynağı paylaşır; kartları() üç  bağımsız Grafik üretir, canlı_veriyi_ayarla seçenek ve yüzey ağacını koruyan  setData karşılığıdır. İzleme: CPU/RAM/ağ yerine servis telemetrisi, log oranı  veya sensör değerleri geçirilebilir. Maliyet: kaynak üç ayrı 100 ms timer  kullanır; port aynı tikte tek scheduler ile üç yüzeyi günceller ve veri  sonunda gereksiz kopya/çizimi durdurur. Cursor ve ölçekler yüzeyler arasında  senkronlanmaz; wheel/touch/drag kaynak dışı isteğe bağlı çekirdek uzantısıdır.",
        ),
        STREAM_DATA_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/stream_data.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("stream-data"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::StreamData(örnek) => stream_data_kartı(örnek),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::GpuiSvgExport,
        "gpui-svg-export",
        "GPUI SVG Export · isteğe bağlı kayıt",
        "svg-image.html · GPUI retained yüzey → açık istekle gerçek vektör kayıt",
        Some(
            "Amaç: canlı grafik ile rapor veya olay eki olarak saklanabilen bağımsız  vektör anlık görüntüsünü ayırır. API: GpuiGrafik::svg_kaydı yalnız açıkça  çağrıldığında retained ana sahneyi ve istenirse etkileşim katmanını SVG  komutlarına kaydeder; native hedefte svg_dosyasına_yaz da bulunur. İzleme:  dashboard panelini rapora, olay ekine veya panoya taşımak için uygundur.  Normal GPUI paint/frame yolunda kayıt bayrağı, String, Blob veya komut başına  dal yoktur. Bu karttaki düğme dışa aktarımı o anda çalıştırır ve gerçek vektör  metnini panoya kopyalar; grafik ekranda GPUI ile boyanmaya devam eder.",
        ),
        GPUI_SVG_EXPORT_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/gpui_svg_export.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::GpuiSvgExport => gpui_svg_export_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::SyncCursor,
        "sync-cursor",
        "Sync Cursor",
        "sync-cursor.html · sync.js · bench/data.json · 5 eşzamanlı yüzey",
        Some(
            "Amaç: ayrı CPU, RAM ve TCP yüzeyleriyle farklı seri sıralı iki karşılaştırma yüzeyini çekirdek grup ilişkileri içinde gösterir. API: GpuiGrafikGrubu cursor, wheel, seçim, pan, eksen zoomu, tam görünüm ve setSeries olaylarını normalize fiziksel oranlarla bütün üyelere taşır; üyelerin genişlik, yükseklik, veri aralığı ve Y birimi aynı olmak zorunda değildir. İndeks veya etiket tabanlı seri eşleme grup ayarıdır. İzleme: farklı birim ve boyutlardaki servis telemetrisinde aynı oransal zamanı ve dikey konumu birlikte incelemek içindir. Sync kapatmak yerel cursor/kilit durumunu silmez; ikinci grup kaynak gibi cursor kilidi kullanmaz. Maliyet: beş retained yüzey bağımsız geometrisini korur; pointer yalnız hafif etkileşim katmanlarını, zoom ise paylaşılan görünüm matrislerini günceller.",
        ),
        SYNC_CURSOR_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/sync_cursor.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::SyncCursor => sync_cursor_kartı(SyncCursorÖrneği::Cpu),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::SyncYZero(SyncYZeroAşaması::Ham),
        "sync-y-zero",
        "Sync Y Zero",
        "sync-y-zero.html · ham → simetrik → ortak sıfır pikseli · 3 sol Y ekseni",
        Some(
            "Amaç: farklı büyüklüklerdeki üç Y ölçeğinin sıfırını ham değer sınırlarını  kaybetmeden aynı fiziksel piksele hizalar. API: sync_y_zero_aralıkları ham,  simetrik ve valToPos/posToVal eşdeğeri final aralıklarını üretir;  Grafik::y_ölçek_aralıklarını_ayarla üç adlandırılmış scale.range sonucunu  atomik uygular. İzleme: pozitif/negatif sapmaları farklı birimlerle tek ortak  X ekseninde karşılaştırmak için uygundur. Kaynak zaman çizelgesi seçimden  3 saniye sonra simetrik, 6 saniye sonra 1/11 ortak sıfır oranına geçer.  Maliyet: her aşama O(3) dönüşüm ve tek sahne boyamasıdır; veri, seçenek ağacı,  Grafik ve GPUI entity yeniden kurulmaz. Cursor, legend, X zoom ve ortak  wheel/touch uzantılarının görünüm durumu korunur.",
        ),
        SYNC_Y_ZERO_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/sync_y_zero.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("sync-y-zero"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::SyncYZero(aşama) => sync_y_zero_kartı(aşama),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::ThinBars(ThinBarsÖrneği::Yoğunluk(
            uplot_rs_gpui_ornekler::ThinBarsYoğunluk::Normal30,
        )),
        "thin-bars-stroke-fill",
        "Thin bar stroke & fill",
        "thin-bars-stroke-fill.html · paths/bars.js · 55 vuruş/dolgu geometrisi",
        Some(
            "Amaç: ince çubuklarda vuruşun ne zaman dolguya düştüğünü ve align, width,  gap, dir, stroke birleşimlerinin geometriyi nasıl değiştirdiğini yan yana  karşılaştırır. API: thin_bars_stroke_fill_kartları kaynak sırasıyla 7  yoğunluk ve 48 geometri yüzeyini tek grup olarak döndürür; her Grafik kendi  cursor, seçim ve geçmişini bağımsız tutar. İzleme: yoğun histogram veya  sütun telemetrisinde panel genişliğine göre okunabilir vuruş/dolgu seçmek ve  ters X/hizalama kararlarını doğrulamak için uygundur. Maliyet: kaynak gibi  55 yüzey ve toplam 1.422 çubuk kurulur; bar başına element ağı kurulmaz.  Pointer yalnız ilgili GpuiGrafik etkileşim katmanını, zoom yalnız ilgili  sahneyi günceller. Noktalar görünür X piksel açıklığı yeterli olduğunda  otomatik açılır; wheel/touch/drag isteğe bağlı çekirdek uzantısıdır.",
        ),
        THIN_BARS_STROKE_FILL_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/thin_bars_stroke_fill.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("thin-bars-stroke-fill"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::ThinBars(örnek) => thin_bars_stroke_fill_kartı(örnek),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::TimePeriods(TimePeriodsÖrneği::SaatlikKullanıcılar),
        "time-periods",
        "Time Periods",
        "time-periods.html · traffic.json · saatlik/aylık/günlük dönem karşılaştırması",
        Some(
            "Amaç: aynı trafik kaynağını saatlik yıllar, iki ay ve günlük toplamlar  biçiminde yan yana karşılaştırır. API: time_periods_kartları üç bağımsız  Grafik döndürür; Hourly seri bazlı geçmiş-yıl lejant tarihleri, Feb–Jan  görünür birincil ölçekten türetilen ikinci X ekseni ve Daily ortak UTC  tarihini kullanır. İzleme: aynı ölçümün dönem ve çözünürlük farklarını  Grafana benzeri panellerde karşılaştırmak için uygundur. Maliyet: traffic.json  bir kez ayrıştırılır; her yüzey kendi cursor, seçim, wheel/touch/drag ve  görünüm geçmişini tutar; etkileşim yalnız ilgili GpuiGrafik sahnesini yeniler.",
        ),
        TIME_PERIODS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/time_periods.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("time-periods"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::TimePeriods(örnek) => time_periods_kartı(örnek),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::TimelineDiscrete(TimelineDiscreteÖrneği::DurumZamanÇizelgesi),
        "timeline-discrete",
        "Timeline / Discrete",
        "timeline-discrete.html · distr.js · quadtree.js · null/undefined şeritleri",
        Some(
            "Amaç: gerçek süreli durum geçişlerini, sabit örnek hücrelerini ve yinelenen  değer birleştirmesini aynı kaynak bağlamında karşılaştırır. API:  timeline_discrete_kartları dört bağımsız Grafik döndürür; null/undefined  ayrımı, şerit dağılımı, renk/etiket, sağ kenara uzanan son durum ve 100px  sınırlı matrix vuruşu çekirdektedir. timeline_verisini_ayarla setData ile  hücre dizinini atomik yeniler; setSeries görünürlüğü özel timeline katmanını  değiştirir. İzleme: cihaz duty-cycle ve servis durum geçmişi için uygundur.  Maliyet: hücreler element ağı değil tek sahne boyamasıdır; hover yalnız gerçek  boyalı hücreyi ve hafif vurgu katmanını günceller.",
        ),
        TIMELINE_DISCRETE_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/timeline_discrete.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("timeline-discrete"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::TimelineDiscrete(örnek) => timeline_discrete_kartı(örnek),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::TimeseriesDiscrete,
        "timeseries-discrete",
        "TimeSeries + Discrete",
        "timeseries-discrete.html · iki yüzey · ortak X imleci · birleşik lejant",
        Some(
            "Amaç: aynı zaman eksenindeki sürekli telemetriyi ve ayrık cihaz durumlarını  iki yükseklikte fakat tek etkileşim bağlamında karşılaştırır. API:  timeseries_discrete_kartları üst float ve alt stepped yüzeyi birlikte döndürür;  TimeseriesDiscreteGrubu ortak X imlecini, seçim/zoom görünümünü ve birleşik  lejantı koordine eder, setSeries yalnız sahibi olan yüzeyi değiştirir. İzleme:  CPU/yük gibi sürekli ölçümlerle servis, alarm veya cihaz açık-kapalı durumlarını  aynı zaman noktasında okumak için uygundur. Maliyet: iki ana yüzey yalnız veri  ya da ölçek değiştiğinde boyanır; cursor çizgileri ve birleşik lejant hafif  katmanda güncellenir, veri yolları pointer hareketinde yeniden kurulmaz.",
        ),
        TIMESERIES_DISCRETE_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/timeseries_discrete.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::TimeseriesDiscrete => {
                timeseries_discrete_kartı(TimeseriesDiscreteÖrneği::ZamanSerisi)
            }
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::CursorSnap,
        "cursor-snap",
        "Cursor Snap · 10×10 grid",
        "cursor-snap.html · çekirdek 10×10 piksel imleç ızgarası",
        Some(
            "Amaç: cursor çizgilerini ve alan seçiminin iki ucunu kaynak cursor.move  callback'i gibi aynı 10×10 CSS piksel ızgarasına oturtur; hover noktaları  dönüştürülmüş X'e en yakın gerçek veri örneğinde kalır. API:  GrafikSeçenekleri::imleç_ızgara_adımı dönüşüm sahipliğini çekirdeğe taşır;  native ve web GPUI cursor, seçim başlangıcı ve seçim bitişinde aynı sonucu kullanır.  Lejant setSeries ile üç dolu çizgi serisini ayrı açıp kapatır. İzleme:  gürültülü zaman serilerinde tekrarlanabilir piksel adımlarıyla karşılaştırma  ve zoom penceresi seçmek içindir. Maliyet: snap O(1), hizalı en yakın X  araması O(log N)'dir; normal pointer hareketi ana üç yolu yeniden çizmez,  yalnız hafif cursor/hover/lejant katmanını günceller.",
        ),
        CURSOR_SNAP_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/cursor_snap.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::CursorSnap => cursor_snap_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::CursorTooltip,
        "cursor-tooltip",
        "Cursor Tooltip w/placement.js",
        "cursor-tooltip.html · sınırlara duyarlı canlı bilgi kutusu",
        Some(
            "Amaç: tek yeşil serideki en yakın X/Y örneğini ve plot alanına göre CSS  piksel cursor konumunu hafif bir bilgi kutusunda gösterir. API:  bilgi_kutusunu_yerleştir kaynak placement.js right/start kuralını gerçek  biçimlendirilmiş metin genişliği, plot sınırı ve 12 piksellik boşlukla  çekirdekte hesaplar; sağ alan yetmezse kutu imlecin soluna döner. İzleme:  bir telemetri örneğinin zaman ve değerini ana çizimi değiştirmeden hızlıca  okumak içindir. Maliyet: en yakın X araması O(log N), yerleşim O(1)'dir;  GPUI ana yol önbelleğini korur ve yalnız hafif etkileşim katmanı ile overlay'i  yeniler; GPUI Web pointer olaylarını aynı retained sahne durumuna uygular.",
        ),
        CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/cursor_tooltip.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::CursorTooltip => cursor_tooltip_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::CustomScales,
        "custom-scales",
        "Custom Scales · 3 independent surfaces",
        "custom-scales.html · aynı sayfada doğrusal, log-log ve özel Weibull ölçeği",
        Some(
            "Amaç: aynı 199×4 kaynak veriyi ve 20 siyah draw noktasını resmî sayfadaki  sırayla üç bağımsız 800×800 yüzeyde karşılaştırır: doğrusal, log10/log10 ve  log10 X + özel Weibull Y. API: custom_scales_kartları aynı veri/seri/bant  tanımından üç Grafik üretir; YÖlçekSeçenekleri::özel adlandırılmış fwd/bwd  fonksiyonlarını, y_sabit_bölmeler ile y_özel_etiketler kaynak axis callback  sonuçlarını taşır. İzleme: olasılık ve güven aralığı verisinde ham, log ve  dağılıma özgü görünümün aynı örnekleri nasıl ayırdığını kıyaslamak içindir.  Üç yüzeyin cursor, zoom, pan ve geçmiş durumları paylaşılmaz. Maliyet: ilk  kurulum üç retained sahnede O(3N)'dir; pointer yalnız hafif etkileşim katmanını  günceller, ana band/path yalnız ölçek, resize veya görünürlük değişiminde  yeniden üretilir.",
        ),
        CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/custom_scales.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::CustomScales => custom_scales_kartı(CustomScaleÖrneği::Doğrusal),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::DataSmoothing,
        "data-smoothing",
        "Data Smoothing · 4 independent surfaces",
        "data-smoothing.html · taxi-trips + SGG + ASAP FFT + Moving Avg 300",
        Some(
            "Amaç: resmî Taxi Trips verisinin ham halini Savitzky–Golay, ASAP FFT ve  300 örneklik hareketli ortalama sonuçlarıyla aynı sayfada, kaynak sırasıyla  karşılaştırır. Dört 1920×300 yüzey bağımsız Grafik örnekleridir; cursor, zoom,  pan ve geçmiş durumlarını paylaşmaz. API: data_smoothing_kartları dört yüzeyi  tek grupta döndürür; savitzky_golay, asap_yumuşat ve hareketli_ortalama sabit  demo parametrelerinin hesaplama API'leridir. Y aralıkları kaynak gibi sabit,  sol eksen 60 pikseldir. İzleme: yoğun zaman serisindeki genel eğilimi korurken  gürültünün farklı yöntemlerle ne ölçüde bastırıldığını ve tepe davranışını  kıyaslamak içindir. Maliyet: algoritmalar yalnız grup kurulurken bir kez  çalıştırılır ve süreleri ayrı ölçülür; toplam 10.937 çizgi örneği retained  sahnelere alınır. Pointer en yakın X'i bulup yalnız etkin yüzeyin hafif  cursor/lejant katmanını günceller; yumuşatma ve ana yollar yeniden hesaplanmaz.",
        ),
        DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/data_smoothing.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::DataSmoothing => data_smoothing_kartı(SmoothingÖrneği::Ham),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::DrawHooks,
        "draw-hooks",
        "Draw Hooks",
        "draw-hooks.html · drawClear/drawSeries/draw plugin hooks",
        Some(
            "Amaç: uPlot yaşam döngüsünün drawClear, drawSeries, özel points.show ve draw  aşamalarının tek yüzeyde hangi sırayla birleştiğini gösterir. API:  ÇizimKancasıDüzeni çok duraklı sürekli arka plan gradyanı, setData sırasında  önbelleklenen seri medyanları, altı uçlu yıldız geometrisi ve gerçek sahne  kurulum süresi stilini tanımlar. Siyah 10px eksen çentikleri ve kaynak veri  birebir korunur; yorum satırındaki grid blur eklentisi bilinçli olarak etkin  değildir. İzleme: Grafana benzeri zaman serilerinde eşik/medyan vurgusu, özel  veri işareti ve çizim maliyeti telemetrisi eklemek için uygundur. Maliyet:  medyan sıralaması yalnız ilk kurulum ve setData sırasında O(S·N logN) çalışır;  drawSeries önbelleği O(S) tüketir. Pointer ana yolları, yıldızları, gradyanı  veya medyanları yeniden üretmeden yalnız cursor/lejant katmanını taşır.",
        ),
        DRAW_HOOKS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/draw_hooks.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::DrawHooks => draw_hooks_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::MissingData,
        "missing-data",
        "Missing Data · 2 related surfaces",
        "missing-data.html · resmî veri ve iki kaynak alt grafiği",
        Some(
            "Amaç: aynı resmî sayfadaki iki bağımsız yüzeyi birlikte karşılaştırır. İlk  yüzey gerçek null CPU/RAM örneklerinin yolu nasıl böldüğünü ve TCP Out'un  bağımsız MB ölçeğini; ikinci yüzey dolu değerlerde komşu X farkı 1'i aşınca  series.gaps ile oluşan boşluğu gösterir. API: missing_data_kartları iki ayrı  Grafik örneğini tek kaynak grubunda döndürür; görünüm ve cursor durumları  bilinçli olarak senkronlanmaz. Seri anahtarları setSeries görünürlüğünü ve  autoscale'ı yüzeyinde günceller. İzleme: veri gerçekten yokken oluşan null  kesintisini, örnekleme zamanındaki büyük aralıktan ayırmak içindir. Maliyet:  yollar yalnız setSeries, ölçek veya resize sırasında O(N) yeniden kurulur;  pointer yalnız hafif cursor/lejant katmanını günceller.",
        ),
        MISSING_DATA_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/missing_data.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::MissingData => missing_data_null_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::DependentScale,
        "dependent-scale",
        "Derived Scale · °F / °C",
        "dependent-scale.html · Fahrenheit'tan türetilen Celsius ekseni",
        Some(
            "Amaç: tek Fahrenheit veri yolunu iki birimde okumayı sağlar; Celsius ekseni  ikinci bir seri veya ikinci çizim yolu değildir. API:  YÖlçekSeçenekleri::sayısal_aralık resmî rangeNum(40,80,.1,true) sonucunu,  kaynak_dönüşümü z.from=y ilişkisini ve eksen_en_az_etiket_boşluğu sağ  axis.space=20 davranışını taşır. Lejant setSeries ile aynı Grafik örneğindeki  blah serisini açıp kapatır. İzleme: sıcaklık, hız veya kapasite gibi doğrusal  dönüştürülebilen aynı telemetriyi iki birim sisteminde gösterin; X ya da Y  görünümü değiştiğinde türetilmiş eksen kaynak ölçeğin min/max dönüşümünü  korur. Maliyet: yalnız bir 7 noktalı çizgi O(N) üretilir; ikinci eksen  dönüşümü ve bölmeleri O(1) ek maliyettir. Pointer yalnız hafif cursor/lejant  katmanını taşır; ana yol setSeries, görünüm veya boyut değişiminde yenilenir.",
        ),
        DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/dependent_scale.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::DependentScale => dependent_scale_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::ArcSinhScales,
        "arcsinh-scales",
        "ArcSinh Y Scale",
        "arcsinh-scales.html · değiştirilebilir doğrusal merkez eşiği",
        Some(
            "Amaç: sıfır çevresindeki küçük değişimleri doğrusal, büyük pozitif ve negatif  büyüklükleri logaritmik okunabilirlikle aynı eksende gösterir. API:  YÖlçekSeçenekleri::arcsinh doğrusal merkez eşiğini tanımlar;  y_arcsinh_eşiği_ayarla aynı Grafik örneğinde ham aralığı ve görünüm geçmişini  koruyarak geometriyi yeniler. Lejant setSeries ile Value serisini açıp  kapatır. İzleme: artı ve eksi yönde birkaç mertebeye yayılan sapma, gecikme  farkı veya bilanço telemetrisi için uygundur; wheel, seçim, pan ve touch ters  ArcSinh dönüşümünü çekirdekte uygular. Maliyet: 111 noktalı tek yol ve  decade/multiple ızgarası yalnız eşik, veri, görünüm veya boyut değişiminde  O(N + tick) yenilenir; pointer ana yolu yeniden üretmez.",
        ),
        ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/arcsinh_scales.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::ArcSinhScales => arcsinh_scales_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::AxisControl,
        "axis-control",
        "Axis Control",
        "axis-control.html · 500.001 nokta ve sağ Y ekseni",
        Some(
            "Amaç: yarım milyon örnekte eksen yerleşimi ve sabit −50…50 Y düzlemini kaynak  sinyal ayrıntısını kaybetmeden doğrular. API:  YÖlçekSeçenekleri::eksen_en_az_etiket_boşluğu axis.space=50'yi;  birincil_y_sağda, eksen rengi/genişliği ve X/Y etiketleri resmî eksen  yapılandırmasını taşır. Lejant setSeries ile sin(x) yolunu açıp kapatır.  İzleme: yoğun ve sabit sınırla karşılaştırılması gereken telemetri içindir;  wheel/seçim görünür X dilimini daralttığında kovalar yalnız o dilimde kurulur.  Maliyet: 500.001 değer bellekte korunur; her piksel kovasında ilk/min/maks/son  adaylarıyla sahne O(plot width) noktaya iner, pointer ana yolu yeniden kurmaz.",
        ),
        AXIS_CONTROL_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/axis_control.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::AxisControl => axis_control_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::AxisAutosize,
        "axis-autosize",
        "Axis AutoSize",
        "axis-autosize.html · 501 nokta ve 1…10⁹ dinamik eksen ölçümü",
        Some(
            "Amaç: aynı 501 noktalı sinyal 500 ms aralıklarla 10 kat büyürken X son etiketi  ile Y değerleri için gereken eksen payının kendini yeniden ölçmesini gösterir.  API: AxisAutosizeAkışı kaynak 1…10⁹ yaşam döngüsünü yürütür;  Grafik::canlı_veriyi_x_etiket_çarpanında_ayarla aynı Grafik örneğinde setData  ve X values çarpanını atomik yeniler. Lejant setSeries görünürlüğü tikler  boyunca korunur. İzleme: büyüklüğü çalışma anında birkaç mertebe değişebilen  sayaç, kapasite ve finans telemetrisinde etiket kırpılmasını önlemek içindir;  ortak wheel, seçim, pan ve touch görünümü veri güncellenirken kaybolmaz.  Maliyet: her tikte 501 yeni değer O(N) üretilir; grafik, olay katmanları ve  seçenek ağacı yeniden kurulmaz. Y etiketi genişliği ölçülür, sağ pay son gerçek  X split'inde en fazla üç çevrimde yakınsar; görev 10⁹'da veya karttan çıkışta  bırakılır.",
        ),
        AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/axis_autosize.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::AxisAutosize => axis_autosize_kartı(10_f64.powi(girdi.autosize_kuvvet)),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::AxisIndicators,
        "axis-indicators",
        "Axis indicators",
        "axis-indicators.html · üç renkli eksen ve imleç göstergeleri",
        Some(
            "Amaç: aynı X örneğindeki üç bağımsız Y ölçeğini, ana grafik yollarını yeniden  çizmeden renkli eksen rozetleri ve kılavuzlarla birlikte okumayı sağlar. API:  her YÖlçekSeçenekleri kendi 50 px eksen dilimini, rengini ve aralığını taşır;  axisIndicsPlugin karşılığı genel yatay cursor çizgisini kapatır ve yalnız  görünür/dolu serilerin rozetlerini günceller. Lejant setSeries ile seri yolunu,  noktasını ve rozetini birlikte açıp kapatır. İzleme: aynı zaman noktasındaki  farklı birim veya büyüklüklerdeki CPU, bellek ve ağ metriklerini bağımsız  ölçeklerde karşılaştırın; kırmızı serinin null aralığında yalnız kırmızı  gösterge gizlenir. Maliyet: 30×3 ana yol yalnız veri, görünüm, boyut veya  setSeries değişiminde üretilir; pointer dört hafif rozeti ve üç kılavuzu  O(seri) konumlandırır, karta özel zamanlayıcı bırakılmaz.",
        ),
        AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/axis_indicators.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::AxisIndicators => axis_indicators_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::Bars(ÇubukÖrneği::ÇokGrupÇokSeriDikeyGruplu),
        "bars-grouped-stacked",
        "Bars · Grouped / Stacked · 10 yüzey",
        "bars-grouped-stacked.html · 10 bağımsız grouped/stacked yüzey ve setSeries",
        Some(
            "Amaç: kaynak sayfanın grouped/stacked, dikey/yatay ve tek grup/tek seri sınır  durumlarını on bağımsız yüzeyde birlikte karşılaştırır. API:  bars_grouped_stacked_kartları yüzeyleri kaynak DOM sırasında döndürür;  ÇubukDüzeni yön, yığma ve ters ekseni tanımlar. setSeries grouped serinin  yuvasını, önceden yığılmış serinin kümülatif boşluğunu korur; yeniden yığma  yapmaz. Hover yalnız vurulan barı vurgular ve stacked değerini kümülatif tepe  olarak verir. İzleme: kategorik kapasite, sürüm ya da bölge metriklerini  karşılaştırırken düzen sınırlarını tek sayfada doğrulamak için uygundur.  Maliyet: her yüzey yalnız kendi barlarını tek sahne geçişinde O(grup×seri)  çizer; on Grafik veri ve görünüm geçmişi bakımından bağımsızdır. Kaynak seçim  ve wheel kapalıdır; ortak wheel/touch/drag profili geliştiricinin açabildiği  port uzantısıdır.",
        ),
        BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/bars_grouped_stacked.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("bars-grouped-stacked"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::Bars(örnek) => bars_grouped_stacked_kartı(örnek),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::BarsValuesAutosize(ÇubukYönü::Dikey),
        "bars-values-autosize",
        "Bars Values AutoSize · 2 yüzey",
        "bars-values-autosize.html · dikey/yatay otomatik kompakt değer yazısı",
        Some(
            "Amaç: aynı rastgele değer dizisini dikey ve yatay çubuk yönünde gösterirken  değer yazısının bar ucuyla grafik kenarı arasındaki kullanılabilir alana  otomatik sığmasını karşılaştırır. API: bars_values_autosize_kartları kaynak  sırasıyla iki bağımsız Grafik döndürür; değer_etiketi_otomatik tek çizim  geçişinde kompakt metinleri, bar dikdörtgenlerini ve boşlukları ölçer. Dikey  yüzey metin genişliği, yüksekliği ve bar genişliğinin %80'inden; yatay yüzey  en dar bar yüksekliğinin %80'inden bütün etiketler için ortak 10–25 px boyut  seçer. 10 px altına düşerse etiketlerin tamamı gizlenir. İzleme: dinamik  pozitif/negatif kapasite veya fark metriklerinde etiket taşmasını engellemek  için uygundur. Maliyet: kompakt metin ölçüleri setData'da O(N), kullanılabilir  alan ve çizim O(N) hesaplanır; yüzey yeniden kurulmaz. Kaynakta yorumlu  setData/setSize akışları aynı önbellek ve yeniden ölçüm yaşam döngüsünü  kanıtlar; ortak wheel/touch/drag port uzantısıdır.",
        ),
        BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/bars_values_autosize.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("bars-values-autosize"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::BarsValuesAutosize(yön) => bars_values_autosize_kartı(yön),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::BoxWhisker("01_run1k"),
        "box-whisker",
        "Box & Whisker · 17 yüzey",
        "box-whisker.html · 17 bağımsız yüzey · results.json ve stats.js",
        Some(
            "Amaç: 17 benchmarkın framework dağılımlarını kaynak sayfadaki aynı bağlamda  karşılaştırır. API: box_whisker_kartları kaynak sırasıyla 17 bağımsız Grafik  döndürür; results.json yalnız bir kez ayrıştırılıp özetlenir. stats.js  medyan/q1/q3 değerlerini iki ondalığa yuvarladıktan sonra 1,5×IQR ile bıyık ve  ayrık değer sınıflaması yapılır; rangeNum bütün ayrık değerlerin global  sınırını kapsar. Tam framework adları -90° eksende korunur. Hover ana sahneyi  yeniden çizmeden mavi sütun vurgusunu ve sarı Lib/Median/q1/q3/min/max bilgi  kutusunu hafif katmanda taşır. İzleme: gecikme, bellek ve başlangıç  ölçümlerinde merkezi eğilim kadar varyansı ve kararsız koşuları görmek için  uygundur. Maliyet: ilk özetleme toplam ölçüm sayısıyla O(N), her yüzey çizimi  en çok 30 kutu ve ayrık değer sayısıyla O(N)'dir; ortak wheel/touch/drag ürün  uzantısıdır.",
        ),
        BOX_WHISKER_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/box_whisker.rs",
        KatalogKartGrubu::İlişkiliYüzeyler,
        Some("box-whisker"),
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::BoxWhisker(benchmark) => box_whisker_kartı(benchmark),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::Candlestick,
        "candlestick-ohlc",
        "Candlestick Chart · Gold",
        "candlestick-ohlc.html · Gold OHLC ve hacim",
        Some(
            "Amaç: Gold için tek hizalı tarih sütunundaki Open/High/Low/Close ve hacmi  kaynak demodaki aynı mum + hacim yüzeyinde gösterir. API: MumDüzeni UTC  zamanlarını OHLC sütunlarından ayrı yan veri olarak taşır. Beş seri bağımsız  çizgiler değil tek mum geometrisinin zorunlu alanlarıdır; kaynak özel çizicisi  setSeries/legend toggle sunmaz. Hover ana sahneyi yeniden çizmeden mavi sütun  vurgusunu ve sarı Date/Open/High/Low/Close/Volume bilgi kutusunu hafif katmanda  taşır. Fiyatlar kaynak fmtUSD biçiminde, tarih UTC YYYY-MM-DD olarak gösterilir.  İzleme: piyasa fiyatı veya OHLC pencere özetlerinde yönü, aralığı ve hacmi aynı  zaman sütununda incelemek için uygundur. Maliyet: 218 kaynak satırı gömülüdür;  ana sahne yalnız görünür mum aralığını O(V) çizer, sütun vuruşu sıralı X üzerinde  O(log N)'dir. Ortak wheel/touch/drag davranışları ürün uzantısıdır.",
        ),
        CANDLESTICK_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/candlestick_ohlc.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::Candlestick => candlestick_ohlc_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
    katalog_kartı!(
        KartKimliği::CursorBind,
        "cursor-bind",
        "Cursor Bind (try Ctrl + drag)",
        "cursor-bind.html · Ctrl+sürükle sarı açıklama seçimi · yakınlaştırma yok",
        Some(
            "Amaç: bir grafik olayının varsayılan işleyicisini koruyup çevresine uygulama  politikası eklemeyi gösterir; normal sürükleme zoom, Ctrl sürükleme açıklama  istemidir. API: İmleçBağSeçenekleri birincil tuş filtresi, Ctrl sırasında  setScale durdurma, gerçek Annotation Text istemi ve sürüklemesiz click  iletimini tek deklaratif sözleşmede tanımlar. Kaynaktaki gibi sarı seçim yalnız  dolgu taşır; metin İptal/Tamam/Enter sonrasında kalıcı çizime eklenmez. İzleme:  Grafana benzeri yüzeylerde seçim zoomunu korurken Ctrl ile olay/incident notu  istemek veya normal tıklamayı üst uygulamaya iletmek için uygundur. Maliyet:  30×3 kaynak seri O(N) çizilir; bind kararı ve click iletimi O(1), Ctrl seçiminde  yalnız hafif seçim katmanı ve modal güncellenir.",
        ),
        CURSOR_BIND_KART_TANIM_ÖRNEĞİ,
        "uygulamalar/ornekler/src/cursor_bind.rs",
        KatalogKartGrubu::Tek,
        None,
        &[],
        varyant_slugı_yok,
        |girdi| match girdi.kart {
            KartKimliği::CursorBind => cursor_bind_kartı(),
            _ => yanlış_kart_fabrikası(girdi.kart),
        }
    ),
];

impl KatalogKartTanımı {
    fn slugdan(&self, slug: &str) -> Option<KartKimliği> {
        if self.slug == slug || self.eski_sluglar.contains(&slug) {
            Some(self.kimlik)
        } else {
            (self.varyant_slugdan)(slug)
        }
    }

    fn grafiği_oluştur(&self, girdi: KatalogFabrikaGirdisi) -> KatalogKartÇıktısı {
        (self.fabrika)(girdi)
    }
}

impl KartKimliği {
    fn ana_kart(self) -> Self {
        match self {
            Self::SoftMinMax(_) => Self::SoftMinMax(SoftMinMaxÖrneği::MinKip0),
            Self::SparklinesBars(_) => Self::SparklinesBars(SparklinesBarsÖrneği::GradyanÇubuklar),
            Self::Sparklines(_) => Self::Sparklines(SparklineÖrneği::İLK),
            Self::Sparse(_) => Self::Sparse(SparseÖrneği::YerleşikDoğrusal),
            Self::StackedSeries(_) => Self::StackedSeries(StackedSeriesÖrneği::Stacked1),
            Self::StreamData(_) => Self::StreamData(StreamDataÖrneği::SabitUzunluk),
            Self::SyncYZero(_) => Self::SyncYZero(SyncYZeroAşaması::Ham),
            Self::ThinBars(_) => Self::ThinBars(ThinBarsÖrneği::Yoğunluk(
                uplot_rs_gpui_ornekler::ThinBarsYoğunluk::Normal30,
            )),
            Self::TimePeriods(_) => Self::TimePeriods(TimePeriodsÖrneği::SaatlikKullanıcılar),
            Self::TimelineDiscrete(_) => {
                Self::TimelineDiscrete(TimelineDiscreteÖrneği::DurumZamanÇizelgesi)
            }
            Self::MultiBars(_) => Self::MultiBars(MultiBarsÖrneği::KitaplıklarDikey),
            Self::Bars(_) => Self::Bars(ÇubukÖrneği::ÇokGrupÇokSeriDikeyGruplu),
            Self::BarsValuesAutosize(_) => Self::BarsValuesAutosize(ÇubukYönü::Dikey),
            Self::BoxWhisker(_) => Self::BoxWhisker("01_run1k"),
            kart => kart,
        }
    }

    fn tanımlayıcı(self) -> &'static KatalogKartTanımı {
        let ana_kart = self.ana_kart();
        KATALOG_KARTLARI
            .iter()
            .find(|tanım| tanım.kimlik == ana_kart)
            .unwrap_or_else(|| std::process::abort())
    }

    fn slug(self) -> &'static str {
        self.tanımlayıcı().slug
    }

    fn slugdan(slug: &str) -> Option<Self> {
        KATALOG_KARTLARI
            .iter()
            .find_map(|tanım| tanım.slugdan(slug))
    }

    fn etkileşimler(self) -> EtkileşimSeçenekleri {
        if self == Self::CursorBind {
            ortak_kart_etkileşimleri().imleç_bağları(İmleçBağSeçenekleri::cursor_bind())
        } else if matches!(self, Self::StreamData(_)) {
            ortak_kart_etkileşimleri().seçim_yakınlaştır(false)
        } else if self == Self::YScaleDrag {
            ortak_kart_etkileşimleri().eksen_sürükleme(true)
        } else if self == Self::ScrollSync {
            ortak_kart_etkileşimleri()
                .tekerlek_etkileşimi(false)
                .dokunma_etkileşimi(false)
        } else {
            ortak_kart_etkileşimleri()
        }
    }
}

pub struct ChartListesi {
    aktif_kart: KartKimliği,
    /// Çizim tabanının son ölçülen boyutu.
    ///
    /// Yüzey yerleşimi normalde `uyarlanan_alan` ile ölçüm anında çözülür.
    /// Lejant düğmesi gibi `Context` isteyen içerikler kapanışa taşınamadığı
    /// için o kartlar bu ölçümü kullanır; değer yalnız değiştiğinde yazılır.
    çizim_alanı: Size<Pixels>,
    kart_listesi_kaydırma: UniformListScrollHandle,
    latency_heatmap_kaydırma: UniformListScrollHandle,
    kart_listesi_kaydırma_bekliyor: Option<usize>,
    nokta_sayısı: usize,
    grafik: Option<Entity<GpuiGrafik>>,
    hata: Option<String>,
    kart_tanımı_açık: bool,
    kullanım_rehberi_açık: bool,
    tekerlek_etkin: bool,
    tekerlek_anahtarı: Entity<Anahtar>,
    tekerlek_odaksız_etkin: bool,
    tekerlek_odaksız_anahtarı: Entity<Anahtar>,
    içi_boş_noktalar_görünür: bool,
    dolu_noktalar_görünür: bool,
    içi_boş_nokta_anahtarı: Entity<Anahtar>,
    dolu_nokta_anahtarı: Entity<Anahtar>,
    arcsinh_kuvvet: i32,
    autosize_kuvvet: i32,
    axis_autosize_akışı: Option<AxisAutosizeAkışı>,
    latency_kova: u8,
    latency_ofset: u8,
    açıklama_istendi: bool,
    açıklama_odak_bekliyor: bool,
    açıklama_metni: Entity<MetinAlani>,
    cursor_bind_tıklama_sayısı: u32,
    dinamik_seri_sayacı: u32,
    align_data_zamanlayıcısı: Option<Task<()>>,
    align_data_grafikleri: Vec<(AlignDataÖrneği, Entity<GpuiGrafik>)>,
    align_data_kurulum_ms: Option<f64>,
    custom_scales_grafikleri: Vec<(CustomScaleÖrneği, Entity<GpuiGrafik>)>,
    data_smoothing_grafikleri: Vec<(SmoothingÖrneği, Entity<GpuiGrafik>)>,
    data_smoothing_ölçümleri_ms: Vec<(SmoothingÖrneği, f64)>,
    focus_cursor_grafikleri: Vec<(FocusÖrneği, Entity<GpuiGrafik>)>,
    gradients_grafikleri: Vec<(GradientÖrneği, Entity<GpuiGrafik>)>,
    high_low_bands_grafikleri: Vec<(HighLowBandsÖrneği, Entity<GpuiGrafik>)>,
    latency_heatmap_grafikleri: Vec<(LatencyHeatmapÖrneği, Entity<GpuiGrafik>)>,
    line_paths_grafikleri: Vec<(LinePathsÖrneği, Entity<GpuiGrafik>)>,
    line_paths_senkronlanıyor: bool,
    log_scales_grafikleri: Vec<(LogScalesÖrneği, Entity<GpuiGrafik>)>,
    log_scales2_grafikleri: Vec<(LogScales2Örneği, Entity<GpuiGrafik>)>,
    log_scales2_senkronlanıyor: bool,
    pixel_align_akışı: Option<PixelAlignAkışı>,
    pixel_align_son_kare: Option<Instant>,
    sine_akışı: Option<SineAkışı>,
    sine_kare_bekleniyor: bool,
    stream_data_grubu: Option<StreamDataGrubu>,
    soft_minmax_akışı: Option<SoftMinMaxAkışı>,
    boyut_senkron_akışı: Option<BoyutSenkronAkışı>,
    y_shifted_series_akışı: Option<YShiftedSeriesAkışı>,
    soft_minmax_çalışıyor: bool,
    soft_minmax_grafikleri: Vec<(SoftMinMaxÖrneği, Entity<GpuiGrafik>)>,
    sparklines_bars_grafikleri: Vec<(SparklinesBarsÖrneği, Entity<GpuiGrafik>)>,
    sparklines_grafikleri: Vec<(SparklineÖrneği, Entity<GpuiGrafik>)>,
    sparse_grafikleri: Vec<(SparseÖrneği, Entity<GpuiGrafik>)>,
    stacked_series_grafikleri: Vec<(StackedSeriesÖrneği, Entity<GpuiGrafik>)>,
    stream_data_grafikleri: Vec<(StreamDataÖrneği, Entity<GpuiGrafik>)>,
    thin_bars_grafikleri: Vec<(ThinBarsÖrneği, Entity<GpuiGrafik>)>,
    time_periods_grafikleri: Vec<(TimePeriodsÖrneği, Entity<GpuiGrafik>)>,
    timeline_discrete_grafikleri: Vec<(TimelineDiscreteÖrneği, Entity<GpuiGrafik>)>,
    sync_cursor_grafikleri: Vec<(SyncCursorÖrneği, Entity<GpuiGrafik>)>,
    sync_cursor_çekirdek_grupları: Vec<Entity<GpuiGrafikGrubu>>,
    sync_cursor_grubu: SyncCursorGrubu,
    timeseries_discrete_grafikleri: Vec<(TimeseriesDiscreteÖrneği, Entity<GpuiGrafik>)>,
    timeseries_discrete_senkronlanıyor: bool,
    timezones_dst_grafikleri: Vec<(TimezonesDstÖrneği, Entity<GpuiGrafik>)>,
    timezones_dst_senkronlanıyor: bool,
    nearest_non_null_grafikleri: Vec<(NearestNonNullÖrneği, Entity<GpuiGrafik>)>,
    missing_data_grafikleri: Vec<(MissingDataÖrneği, Entity<GpuiGrafik>)>,
    months_grafikleri: Vec<Entity<GpuiGrafik>>,
    path_gap_clip_grafikleri: Vec<(PathGapClipÖrneği, Entity<GpuiGrafik>)>,
    pixel_align_grafikleri: Vec<(PixelAlignÖrneği, Entity<GpuiGrafik>)>,
    points_grafikleri: Vec<(PointsÖrneği, Entity<GpuiGrafik>)>,
    scales_dir_ori_grafikleri: Vec<(ScalesDirOriÖrneği, Entity<GpuiGrafik>)>,
    scatter_grafikleri: Vec<(ScatterÖrneği, Entity<GpuiGrafik>)>,
    bars_grouped_stacked_grafikleri: Vec<(ÇubukÖrneği, Entity<GpuiGrafik>)>,
    bars_values_autosize_grafikleri: Vec<(ÇubukYönü, Entity<GpuiGrafik>)>,
    box_whisker_grafikleri: Vec<(&'static str, Entity<GpuiGrafik>)>,
    scales_dir_ori_senkronlanıyor: bool,
    scales_dir_ori_kilitli: bool,
    no_data_örneği: NoDataÖrneği,
    svg_kayıt_baytı: Option<usize>,
    kare_ölçer: KareÖlçer,
    performans_kare_bekleniyor: bool,
    lejant: Entity<KatalogLejantı>,
    /// Denetim çubuğundaki lejant konumu düğmesinin seçimi. `None` iken
    /// kartın kendi `GrafikSeçenekleri::lejant_konumu` değeri geçerlidir;
    /// kart değişiminde sıfırlanır.
    lejant_konumu_seçimi: Option<LejantKonumu>,
    /// Lejantın değerlerini gösterdiği yüzey. İmleç bir yüzeye girdiğinde
    /// oraya taşınır ve fare ayrıldıktan sonra da korunur; `None` iken kartın
    /// ilk yüzeyi gösterilir. Kart değişiminde sıfırlanır.
    lejant_yüzeyi: Option<::gpui::EntityId>,
    /// Sanallaştırılmış çok yüzeyli kartların kaydırma/ölçüm durumu. Kart
    /// değişiminde sıfırlanır, aksi hâlde eski öğe sayısıyla ölçüm yapar.
    thin_bars_liste_durumu: Option<ListState>,
    timezones_dst_liste_durumu: Option<ListState>,
}

impl ChartListesi {
    /// Etkin kartın bütün yüzeylerini tekilleştirerek gezer.
    ///
    /// Tek gerçek yüzey listesi budur; hem sahiplenen `Vec` hem de tahsissiz
    /// tarayanlar buradan beslenir, böylece yeni bir kart ailesi eklenirken
    /// iki ayrı listeyi eşlemek gerekmez.
    fn etkin_grafik_yüzeylerini_gez(&self, mut ziyaret: impl FnMut(&Entity<GpuiGrafik>)) {
        let mut görülenler = HashSet::new();
        let mut ekle = |grafik: &Entity<GpuiGrafik>| {
            if görülenler.insert(grafik.entity_id()) {
                ziyaret(grafik);
            }
        };
        macro_rules! yüzeyleri_ekle {
            ($alan:expr) => {
                for (_, grafik) in $alan.iter() {
                    ekle(grafik);
                }
            };
        }
        for grafik in self.grafik.iter() {
            ekle(grafik);
        }
        yüzeyleri_ekle!(self.align_data_grafikleri);
        yüzeyleri_ekle!(self.custom_scales_grafikleri);
        yüzeyleri_ekle!(self.data_smoothing_grafikleri);
        yüzeyleri_ekle!(self.focus_cursor_grafikleri);
        yüzeyleri_ekle!(self.gradients_grafikleri);
        yüzeyleri_ekle!(self.high_low_bands_grafikleri);
        yüzeyleri_ekle!(self.latency_heatmap_grafikleri);
        yüzeyleri_ekle!(self.line_paths_grafikleri);
        yüzeyleri_ekle!(self.log_scales_grafikleri);
        yüzeyleri_ekle!(self.log_scales2_grafikleri);
        yüzeyleri_ekle!(self.soft_minmax_grafikleri);
        yüzeyleri_ekle!(self.sparklines_bars_grafikleri);
        yüzeyleri_ekle!(self.sparklines_grafikleri);
        yüzeyleri_ekle!(self.sparse_grafikleri);
        yüzeyleri_ekle!(self.stacked_series_grafikleri);
        yüzeyleri_ekle!(self.stream_data_grafikleri);
        yüzeyleri_ekle!(self.thin_bars_grafikleri);
        yüzeyleri_ekle!(self.time_periods_grafikleri);
        yüzeyleri_ekle!(self.timeline_discrete_grafikleri);
        yüzeyleri_ekle!(self.sync_cursor_grafikleri);
        yüzeyleri_ekle!(self.timeseries_discrete_grafikleri);
        yüzeyleri_ekle!(self.timezones_dst_grafikleri);
        yüzeyleri_ekle!(self.nearest_non_null_grafikleri);
        yüzeyleri_ekle!(self.missing_data_grafikleri);
        for grafik in self.months_grafikleri.iter() {
            ekle(grafik);
        }
        yüzeyleri_ekle!(self.path_gap_clip_grafikleri);
        yüzeyleri_ekle!(self.pixel_align_grafikleri);
        yüzeyleri_ekle!(self.points_grafikleri);
        yüzeyleri_ekle!(self.scales_dir_ori_grafikleri);
        yüzeyleri_ekle!(self.scatter_grafikleri);
        yüzeyleri_ekle!(self.bars_grouped_stacked_grafikleri);
        yüzeyleri_ekle!(self.bars_values_autosize_grafikleri);
        yüzeyleri_ekle!(self.box_whisker_grafikleri);
    }

    /// Yüzeyleri sahiplenen listeye toplar.
    ///
    /// `update` çağıran döngüler `cx`'i ödünç aldığından `self`'i aynı anda
    /// ödünç alamaz; bu yollar klonlanmış listeye ihtiyaç duyar.
    fn etkin_grafik_yüzeyleri(&self) -> Vec<Entity<GpuiGrafik>> {
        let mut grafikler = Vec::new();
        self.etkin_grafik_yüzeylerini_gez(|grafik| grafikler.push(grafik.clone()));
        grafikler
    }

    /// Etkin yüzeylerin geri/yakınlaştırma durumunu tahsis etmeden toplar.
    ///
    /// Kök `render` bu iki bayrağı ve yüzey sayısını her çağrıda istiyordu;
    /// bunun için bütün yüzeylerin `Entity` klonlarından bir `Vec` kurulup
    /// hemen atılıyordu. Sonuç yalnız iki bool ve bir sayaç olduğundan
    /// listeyi maddileştirmeye gerek yok.
    fn etkin_görünüm_durumu(&self, cx: &App) -> (bool, bool, usize) {
        let mut geri_var = false;
        let mut yakınlaştırılmış = false;
        let mut sayı = 0_usize;
        self.etkin_grafik_yüzeylerini_gez(|grafik| {
            sayı = sayı.saturating_add(1);
            let grafik = grafik.read(cx);
            geri_var |= grafik.grafik().geri_var();
            yakınlaştırılmış |= grafik.grafik().yakınlaştırılmış();
        });
        (geri_var, yakınlaştırılmış, sayı)
    }

    fn nokta_gösterimlerini_uygula(
        &mut self,
        içi_boş_görünür: bool,
        dolu_görünür: bool,
        cx: &mut Context<Self>,
    ) {
        let arayüz_değişti = self.içi_boş_noktalar_görünür != içi_boş_görünür
            || self.dolu_noktalar_görünür != dolu_görünür;
        self.içi_boş_noktalar_görünür = içi_boş_görünür;
        self.dolu_noktalar_görünür = dolu_görünür;
        let mut grafik_değişti = false;
        for grafik in self.etkin_grafik_yüzeyleri() {
            grafik_değişti |= grafik.update(cx, |grafik, cx| {
                let içi_boş_değişti = grafik.kırılım_noktalarını_göster(içi_boş_görünür, cx);
                let dolu_değişti = grafik.imleç_noktalarını_göster(dolu_görünür, cx);
                içi_boş_değişti || dolu_değişti
            });
        }
        if arayüz_değişti || grafik_değişti {
            cx.notify();
        }
    }

    fn tekerlek_odaksız_etkileşimi_uygula(&mut self, etkin: bool, cx: &mut Context<Self>) {
        let arayüz_değişti = self.tekerlek_odaksız_etkin != etkin;
        self.tekerlek_odaksız_etkin = etkin;
        let mut grafik_değişti = false;
        for grafik in self.etkin_grafik_yüzeyleri() {
            grafik_değişti |= grafik.update(cx, |grafik, cx| {
                grafik.tekerlek_odaksız_etkileşimi_ayarla(etkin, cx)
            });
        }
        if arayüz_değişti || grafik_değişti {
            cx.notify();
        }
    }

    fn kare_ölçümünü_başlat(&mut self, cx: &mut Context<Self>) {
        if self.kare_ölçer.çalışıyor {
            return;
        }
        self.kare_ölçer.başlat();
        self.performans_kare_bekleniyor = false;
        cx.notify();
    }

    fn svg_kaydını_dışa_aktar(&mut self, cx: &mut Context<Self>) {
        let Some(grafik) = self.grafik.as_ref() else {
            self.hata = Some("SVG kaydı için etkin GPUI yüzeyi bulunamadı".to_string());
            cx.notify();
            return;
        };
        let ayarlar = match GpuiSvgKayıtAyarları::yeni(800, 400) {
            Ok(ayarlar) => ayarlar,
            Err(hata) => {
                self.hata = Some(format!("SVG kayıt ayarları oluşturulamadı: {hata}"));
                cx.notify();
                return;
            }
        };
        let kayıt = grafik.read(cx).svg_kaydı(ayarlar);
        let svg = kayıt.stringe_dönüştür();
        self.svg_kayıt_baytı = Some(svg.len());
        #[cfg(target_family = "wasm")]
        if let Err(hata) = web_köprüsü::svg_indir(
            &svg,
            &format!("uplot-rs-{}.svg", self.aktif_kart.tanımlayıcı().slug),
        ) {
            self.hata = Some(hata);
            cx.notify();
            return;
        }
        #[cfg(not(target_family = "wasm"))]
        cx.write_to_clipboard(ClipboardItem::new_string(svg));
        self.hata = None;
        cx.notify();
    }

    fn açıklama_istemini_aç(&mut self, cx: &mut Context<Self>) {
        self.açıklama_metni
            .update(cx, |alan, cx| alan.metni_ayarla("", cx));
        self.açıklama_istendi = true;
        self.açıklama_odak_bekliyor = true;
    }

    fn açıklama_istemini_kapat(&mut self, cx: &mut Context<Self>) {
        self.açıklama_istendi = false;
        self.açıklama_odak_bekliyor = false;
        self.açıklama_metni
            .update(cx, |alan, cx| alan.metni_ayarla("", cx));
        cx.notify();
    }

    fn standart_grafik_olayını_işle(&mut self, olay: &GpuiGrafikOlayı, cx: &mut Context<Self>) {
        let arayüz_değişti = match olay {
            GpuiGrafikOlayı::Açıklamaİstendi => {
                self.açıklama_istemini_aç(cx);
                true
            }
            GpuiGrafikOlayı::FareBırakıldı if self.aktif_kart == KartKimliği::CursorBind => {
                self.cursor_bind_tıklama_sayısı = self.cursor_bind_tıklama_sayısı.saturating_add(1);
                true
            }
            // İmleç olayı yalnız lejant satırını değiştirir. GPUI `notify`
            // ataları da kirlettiğinden ayrı varlık kökü tek başına izole
            // etmez; kazanç metin gerçekten değişmediğinde hiç `notify`
            // etmemekten gelir. Lejant üç ondalıkla biçimlendiğinden yoğun
            // serilerde ardışık örnekler çoğu zaman aynı satırı üretir.
            // Durum olayı görünür seri düğmelerini de değiştirdiğinden kökü
            // yeniler. Görünüm olayı grafik alt yüzeylerinde zaten işlenir.
            GpuiGrafikOlayı::İmleçDeğişti => {
                self.lejantı_yenile(cx);
                false
            }
            GpuiGrafikOlayı::DurumDeğişti => {
                self.lejantı_yenile(cx);
                true
            }
            // Zoom imlecin veri X'ini değiştirmese de lejant değerini
            // taşıyabilir; metin gerçekten değişirse `lejantı_yenile` zaten
            // kendi `notify`'ını yapar.
            GpuiGrafikOlayı::GörünümDeğişti { .. } => {
                self.lejantı_yenile(cx);
                false
            }
            GpuiGrafikOlayı::İmleçKonumuDeğişti | GpuiGrafikOlayı::FareBırakıldı => false,
        };
        if arayüz_değişti {
            cx.notify();
        }
    }

    /// Lejantı besleyen yüzeyleri girdilerin üretileceği sırayla döndürür.
    ///
    /// Timeseries Discrete iki yüzeyin serilerini tek listede birleştirir;
    /// kalan kartlarda lejant tek yüzey gösterir. Çok yüzeyli kartlarda bu
    /// yüzey imlecin en son girdiğidir — `lejant_yüzeyi` — çünkü kart
    /// tanımının ilk yüzeyini göstermek, imleç 16 yüzeyli Stacked Series'in
    /// herhangi birinde gezerken hep aynı değerleri listeliyordu.
    fn lejant_yüzeyleri(&self) -> Vec<Entity<GpuiGrafik>> {
        if self.aktif_kart == KartKimliği::TimeseriesDiscrete {
            return self
                .timeseries_discrete_grafikleri
                .iter()
                .map(|(_, grafik)| grafik.clone())
                .collect();
        }
        let seçili_kimlik = self.lejant_yüzeyi;
        let mut seçili = None;
        let mut ilk = None;
        self.etkin_grafik_yüzeylerini_gez(|yüzey| {
            if ilk.is_none() {
                ilk = Some(yüzey.clone());
            }
            if seçili.is_none() && seçili_kimlik == Some(yüzey.entity_id()) {
                seçili = Some(yüzey.clone());
            }
        });
        seçili.or(ilk).into_iter().collect()
    }

    /// Yüzeyi kurar ve standart grafik olaylarını köke bağlar.
    ///
    /// Abonelik olmadan imleç ve durum olayları köke hiç ulaşmaz: lejant o
    /// yüzeyin değerlerini göremez, `sparklines` tablosunda fare hangi
    /// hücrede olursa olsun satır `x: -- Hacim: --` kalıyordu. Çok yüzeyli
    /// kartların hepsi bu yoldan geçmeli.
    fn bağlı_yüzey(grafik: Grafik, cx: &mut Context<Self>) -> Entity<GpuiGrafik> {
        let yüzey = cx.new(|_| GpuiGrafik::yeni(grafik));
        cx.subscribe(&yüzey, |bu, _, olay: &GpuiGrafikOlayı, cx| {
            bu.standart_grafik_olayını_işle(olay, cx);
        })
        .detach();
        yüzey
    }

    /// Lejantın izlediği yüzeyi imlecin bulunduğu yüzeye taşır.
    ///
    /// Fare yüzeyden ayrılınca seçim korunur: aksi hâlde lejant girdisine
    /// tıklamak için fareyi yüzeyden çekmek gösterilen seriyi değiştirir ve
    /// tıklama yanlış yüzeye giderdi.
    fn lejant_yüzeyini_güncelle(&mut self, cx: &App) {
        let mut imleçli = None;
        self.etkin_grafik_yüzeylerini_gez(|yüzey| {
            if imleçli.is_none() && yüzey.read(cx).imleç_etkin_mi() {
                imleçli = Some(yüzey.entity_id());
            }
        });
        if imleçli.is_some() {
            self.lejant_yüzeyi = imleçli;
        }
    }

    /// Lejantın X başlığını ve seri girdilerini üretir.
    ///
    /// Gizli seriler de listelenir. Değerler seri sırasıyla eşleştiğinden
    /// listeden düşürülselerdi gizli serinin ardındaki her girdi bir kayarak
    /// komşusunun değerini gösterirdi; uPlot da girdiyi kaldırmak yerine
    /// soluklaştırır.
    fn lejant_içeriği(&self, cx: &App) -> (SharedString, Vec<LejantGirdisi>) {
        let tam_sayı_değerler = self.aktif_kart == KartKimliği::TimeseriesDiscrete;
        let mut girdiler = Vec::new();
        let mut ortak_x = None;
        for yüzey in self.lejant_yüzeyleri() {
            let yüzey = yüzey.read(cx);
            let (x, seri_değerleri, canlı) = match yüzey.lejant_değerleri() {
                Some((x, değerler)) => {
                    ortak_x = ortak_x.or(x);
                    (x, değerler, true)
                }
                None => (None, Vec::new(), false),
            };
            // uPlot `series.value` imleç dışındayken son örneği gösterebilir;
            // o değerin imleçle okunan değerle karıştırılmaması gerekir.
            let boşta = canlı && x.is_none();
            lejant_girdilerini_ekle(
                &mut girdiler,
                yüzey.grafik().seri_seçenekleri(),
                &seri_değerleri,
                boşta,
                tam_sayı_değerler,
            );
        }
        let x_metni = ortak_x.map_or_else(|| "x: --".to_string(), |x| format!("x: {x:.3}"));
        (SharedString::from(x_metni), girdiler)
    }

    /// Denetim düğmesiyle lejantı bir sonraki konuma taşır.
    ///
    /// Kart tanımlarının hepsi kaynak sayfalardaki `Alt` yerleşimini
    /// kullandığından dört konumun da canlı doğrulanabilmesi için tek yol
    /// budur.
    fn lejant_konumunu_ilerlet(&mut self, cx: &mut Context<Self>) {
        let sonraki = match self.lejant_konumu(cx) {
            LejantKonumu::Alt => LejantKonumu::Sol,
            LejantKonumu::Sol => LejantKonumu::Üst,
            LejantKonumu::Üst => LejantKonumu::Sağ,
            LejantKonumu::Sağ => LejantKonumu::Alt,
        };
        self.lejant_konumu_seçimi = Some(sonraki);
        izleme::olay(
            "PANEL",
            &format!(
                "lejant konumu {} · {}",
                lejant_konumu_başlığı(sonraki),
                self.aktif_kart.slug()
            ),
        );
        self.lejantı_yenile(cx);
        cx.notify();
    }

    /// Etkin kartın lejant konumu; kullanıcı denetimi seçeneği ezmediyse
    /// kaynak sayfayla aynı `Alt` yerleşimi gelir.
    fn lejant_konumu(&self, cx: &App) -> LejantKonumu {
        self.lejant_konumu_seçimi.unwrap_or_else(|| {
            self.lejant_yüzeyleri()
                .first()
                .map_or(LejantKonumu::Alt, |yüzey| {
                    yüzey.read(cx).grafik().lejant_konumu()
                })
        })
    }

    fn lejantı_yenile(&mut self, cx: &mut Context<Self>) {
        self.lejant_yüzeyini_güncelle(cx);
        let (x_metni, girdiler) = self.lejant_içeriği(cx);
        let konum = self.lejant_konumu(cx);
        self.lejant.update(cx, |lejant, cx| {
            lejant.içeriği_ayarla(x_metni, girdiler, konum, cx);
        });
    }

    /// Lejant satırına gelindiğinde ilgili seriyi odaklar; ayrılınca bırakır.
    ///
    /// uPlot `setSeries(i, {focus: true})` karşılığıdır. Odak yalnız
    /// `cursor.focus` kurulmuş kartlarda boyanır; kurulu olmayan kartlarda
    /// çağrı sahneye dokunmaz. Birleşik lejantta hedef dışındaki yüzeylerin
    /// odağı da bırakılır, yoksa önceki yüzeyde soluk seriler asılı kalırdı.
    fn lejant_serisini_odakla(&mut self, birleşik_indeks: Option<usize>, cx: &mut Context<Self>) {
        let yüzeyler = self.lejant_yüzeyleri();
        let seri_sayıları = yüzeyler
            .iter()
            .map(|yüzey| yüzey.read(cx).grafik().seri_seçenekleri().len())
            .collect::<Vec<_>>();
        let hedef =
            birleşik_indeks.and_then(|indeks| lejant_hedefini_çöz(&seri_sayıları, indeks));
        for (sıra, yüzey) in yüzeyler.into_iter().enumerate() {
            let seri = hedef
                .filter(|(yüzey_sırası, _)| *yüzey_sırası == sıra)
                .map(|(_, seri)| seri);
            yüzey.update(cx, |grafik, cx| grafik.odak_serisini_ayarla(seri, cx));
        }
    }

    /// Lejant girdisine tıklandığında ilgili serinin görünürlüğünü çevirir.
    ///
    /// Birleşik indeks `lejant_içeriği` ile aynı sırayı izler: yüzeyler
    /// sırayla gezilir ve her yüzeyde seri sayısı kadar ilerlenir.
    fn lejant_serisini_değiştir(&mut self, birleşik_indeks: usize, cx: &mut Context<Self>) {
        let yüzeyler = self.lejant_yüzeyleri();
        let seri_sayıları = yüzeyler
            .iter()
            .map(|yüzey| yüzey.read(cx).grafik().seri_seçenekleri().len())
            .collect::<Vec<_>>();
        let Some((yüzey_sırası, seri)) = lejant_hedefini_çöz(&seri_sayıları, birleşik_indeks)
        else {
            return;
        };
        let Some(yüzey) = yüzeyler.into_iter().nth(yüzey_sırası) else {
            return;
        };
        let görünür = yüzey.read(cx).grafik().seri_görünür_mü(seri);
        match yüzey.update(cx, |grafik, cx| {
            grafik.seri_görünürlüğünü_ayarla(seri, !görünür, cx)
        }) {
            Ok(_) => self.hata = None,
            Err(hata) => self.hata = Some(format!("Lejant serisi değiştirilemedi: {hata}")),
        }
        self.lejantı_yenile(cx);
        cx.notify();
    }

    pub fn yeni(cx: &mut Context<Self>) -> Self {
        let başlangıç_kartı = web_köprüsü::başlangıç_kartı().unwrap_or(KartKimliği::Resize);
        let etkileşimler = ortak_kart_etkileşimleri();
        let açıklama_metni = cx.new(|cx| MetinAlani::yeni("Annotation Text", cx));
        cx.subscribe(&açıklama_metni, |bu, _, olay: &MetinAlaniOlayi, cx| {
            if *olay == MetinAlaniOlayi::Onaylandi {
                bu.açıklama_istemini_kapat(cx);
            }
        })
        .detach();
        let tekerlek_anahtarı = cx.new(|cx| {
            Anahtar::yeni(
                "Tekerlek eklentisi · Otomatik",
                etkileşimler.tekerlek_etkileşimi,
                cx,
            )
        });
        cx.subscribe(&tekerlek_anahtarı, |bu, _, olay: &AnahtarOlayi, cx| {
            let AnahtarOlayi::Degisti(etkin) = *olay;
            let arayüz_değişti = bu.tekerlek_etkin != etkin;
            bu.tekerlek_etkin = etkin;
            let mut grafik_değişti = false;
            for grafik in bu.etkin_grafik_yüzeyleri() {
                grafik_değişti |= grafik.update(cx, |grafik, cx| {
                    grafik.tekerlek_etkileşimi_ayarla(etkin, cx)
                });
            }
            if arayüz_değişti || grafik_değişti {
                cx.notify();
            }
        })
        .detach();

        let tekerlek_odaksız_anahtarı = cx.new(|cx| {
            Anahtar::yeni(
                "Odaksız tekerlek",
                etkileşimler.tekerlek_odaksız_etkileşim,
                cx,
            )
        });
        cx.subscribe(
            &tekerlek_odaksız_anahtarı,
            |bu, _, olay: &AnahtarOlayi, cx| {
                let AnahtarOlayi::Degisti(etkin) = *olay;
                bu.tekerlek_odaksız_etkileşimi_uygula(etkin, cx);
            },
        )
        .detach();

        let içi_boş_nokta_anahtarı = cx.new(|cx| Anahtar::yeni("İçi boş noktalar", true, cx));
        let dolu_nokta_anahtarı = cx.new(|cx| Anahtar::yeni("Dolu noktalar", true, cx));
        cx.subscribe(
            &içi_boş_nokta_anahtarı,
            |bu, _, olay: &AnahtarOlayi, cx| {
                let AnahtarOlayi::Degisti(içi_boş) = *olay;
                let dolu = bu.dolu_nokta_anahtarı.read(cx).acik();
                bu.nokta_gösterimlerini_uygula(içi_boş, dolu, cx);
            },
        )
        .detach();
        cx.subscribe(&dolu_nokta_anahtarı, |bu, _, olay: &AnahtarOlayi, cx| {
            let AnahtarOlayi::Degisti(dolu) = *olay;
            let içi_boş = bu.içi_boş_nokta_anahtarı.read(cx).acik();
            bu.nokta_gösterimlerini_uygula(içi_boş, dolu, cx);
        })
        .detach();

        let (grafik, hata) = grafik_oluştur(
            KartKimliği::Resize,
            NoDataÖrneği::BOŞ_ÖZEL_ARALIK,
            100,
            0,
            5,
            0,
            140,
        )
        .map_or_else(
            |hata| (None, Some(format!("Grafik oluşturulamadı: {hata}"))),
            |grafik| (Some(cx.new(|_| GpuiGrafik::yeni(grafik))), None),
        );
        if let Some(grafik) = &grafik {
            cx.subscribe(grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
        }
        let mut bu = Self {
            aktif_kart: KartKimliği::Resize,
            çizim_alanı: Size {
                width: px(0.0),
                height: px(0.0),
            },
            kart_listesi_kaydırma: UniformListScrollHandle::new(),
            latency_heatmap_kaydırma: UniformListScrollHandle::new(),
            kart_listesi_kaydırma_bekliyor: None,
            nokta_sayısı: 100,
            grafik,
            hata,
            kart_tanımı_açık: false,
            kullanım_rehberi_açık: false,
            tekerlek_etkin: etkileşimler.tekerlek_etkileşimi,
            tekerlek_anahtarı,
            tekerlek_odaksız_etkin: etkileşimler.tekerlek_odaksız_etkileşim,
            tekerlek_odaksız_anahtarı,
            içi_boş_noktalar_görünür: true,
            dolu_noktalar_görünür: true,
            içi_boş_nokta_anahtarı,
            dolu_nokta_anahtarı,
            arcsinh_kuvvet: 0,
            autosize_kuvvet: 0,
            axis_autosize_akışı: None,
            latency_kova: 5,
            latency_ofset: 0,
            açıklama_istendi: false,
            açıklama_odak_bekliyor: false,
            açıklama_metni,
            cursor_bind_tıklama_sayısı: 0,
            dinamik_seri_sayacı: 0,
            align_data_zamanlayıcısı: None,
            align_data_grafikleri: Vec::new(),
            align_data_kurulum_ms: None,
            custom_scales_grafikleri: Vec::new(),
            data_smoothing_grafikleri: Vec::new(),
            data_smoothing_ölçümleri_ms: Vec::new(),
            focus_cursor_grafikleri: Vec::new(),
            gradients_grafikleri: Vec::new(),
            high_low_bands_grafikleri: Vec::new(),
            latency_heatmap_grafikleri: Vec::new(),
            line_paths_grafikleri: Vec::new(),
            line_paths_senkronlanıyor: false,
            log_scales_grafikleri: Vec::new(),
            log_scales2_grafikleri: Vec::new(),
            log_scales2_senkronlanıyor: false,
            pixel_align_akışı: None,
            pixel_align_son_kare: None,
            sine_akışı: None,
            sine_kare_bekleniyor: false,
            stream_data_grubu: None,
            soft_minmax_akışı: None,
            boyut_senkron_akışı: None,
            y_shifted_series_akışı: None,
            soft_minmax_çalışıyor: false,
            soft_minmax_grafikleri: Vec::new(),
            sparklines_bars_grafikleri: Vec::new(),
            sparklines_grafikleri: Vec::new(),
            sparse_grafikleri: Vec::new(),
            stacked_series_grafikleri: Vec::new(),
            stream_data_grafikleri: Vec::new(),
            thin_bars_grafikleri: Vec::new(),
            time_periods_grafikleri: Vec::new(),
            timeline_discrete_grafikleri: Vec::new(),
            sync_cursor_grafikleri: Vec::new(),
            sync_cursor_çekirdek_grupları: Vec::new(),
            sync_cursor_grubu: SyncCursorGrubu::yeni(),
            timeseries_discrete_grafikleri: Vec::new(),
            timeseries_discrete_senkronlanıyor: false,
            timezones_dst_grafikleri: Vec::new(),
            timezones_dst_senkronlanıyor: false,
            nearest_non_null_grafikleri: Vec::new(),
            missing_data_grafikleri: Vec::new(),
            months_grafikleri: Vec::new(),
            path_gap_clip_grafikleri: Vec::new(),
            pixel_align_grafikleri: Vec::new(),
            points_grafikleri: Vec::new(),
            scales_dir_ori_grafikleri: Vec::new(),
            scatter_grafikleri: Vec::new(),
            bars_grouped_stacked_grafikleri: Vec::new(),
            bars_values_autosize_grafikleri: Vec::new(),
            box_whisker_grafikleri: Vec::new(),
            scales_dir_ori_senkronlanıyor: false,
            scales_dir_ori_kilitli: false,
            no_data_örneği: NoDataÖrneği::BOŞ_ÖZEL_ARALIK,
            svg_kayıt_baytı: None,
            kare_ölçer: KareÖlçer::default(),
            performans_kare_bekleniyor: false,
            lejant: {
                let sahip = cx.weak_entity();
                cx.new(|_| KatalogLejantı::yeni(sahip))
            },
            lejant_konumu_seçimi: None,
            lejant_yüzeyi: None,
            thin_bars_liste_durumu: None,
            timezones_dst_liste_durumu: None,
        };
        if başlangıç_kartı != KartKimliği::Resize {
            bu.kartı_seç(başlangıç_kartı, cx);
        }
        bu.tekerlek_odaksız_etkileşimi_uygula(bu.tekerlek_odaksız_etkin, cx);
        bu.nokta_gösterimlerini_uygula(true, true, cx);
        bu.lejantı_yenile(cx);
        bu
    }

    fn timeseries_discrete_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let kartlar = match timeseries_discrete_kartları() {
            Ok(kartlar) => kartlar,
            Err(hata) => {
                self.hata = Some(format!(
                    "TimeSeries + Discrete grubu oluşturulamadı: {hata}"
                ));
                self.grafik = None;
                self.timeseries_discrete_grafikleri.clear();
                cx.notify();
                return;
            }
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        let mut hata = None;
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(oluşturma_hatası) => {
                    hata = Some(format!(
                        "{} yüzeyi oluşturulamadı: {oluşturma_hatası}",
                        örnek.başlık()
                    ));
                    break;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, move |bu, _, olay: &GpuiGrafikOlayı, cx| {
                match olay {
                    GpuiGrafikOlayı::Açıklamaİstendi => bu.açıklama_istendi = true,
                    GpuiGrafikOlayı::İmleçKonumuDeğişti => {
                        let yayın = bu
                            .timeseries_discrete_grafikleri
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .and_then(|(_, grafik)| grafik.read(cx).senkron_yayını());
                        let yüzeyler = bu.timeseries_discrete_grafikleri.clone();
                        for (hedef, hedef_grafik) in yüzeyler {
                            if hedef == örnek {
                                continue;
                            }
                            if let Some((x, _, _)) = yayın {
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.senkron_imleci_ayarla(x, None, None, cx);
                                });
                            } else {
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.senkron_imleci_temizle(cx);
                                });
                            }
                        }
                    }
                    GpuiGrafikOlayı::İmleçDeğişti
                    | GpuiGrafikOlayı::FareBırakıldı
                    | GpuiGrafikOlayı::DurumDeğişti => {}
                    GpuiGrafikOlayı::GörünümDeğişti { .. } => {
                        if bu.timeseries_discrete_senkronlanıyor {
                            return;
                        }
                        let x = bu
                            .timeseries_discrete_grafikleri
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .map(|(_, grafik)| grafik.read(cx).grafik().görünür_x_aralığı());
                        if let Some(x) = x {
                            bu.timeseries_discrete_senkronlanıyor = true;
                            for (hedef, hedef_grafik) in bu.timeseries_discrete_grafikleri.clone() {
                                if hedef == örnek {
                                    continue;
                                }
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.görünür_x_aralığını_sessiz_ayarla(x, true, cx);
                                });
                            }
                            bu.timeseries_discrete_senkronlanıyor = false;
                        }
                    }
                }
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        if let Some(hata) = hata {
            self.hata = Some(hata);
            self.grafik = None;
            self.timeseries_discrete_grafikleri.clear();
        } else {
            self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
            self.timeseries_discrete_grafikleri = yüzeyler;
            self.hata = None;
        }
        cx.notify();
    }

    fn timezones_dst_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let kartlar = match timezones_dst_kartları() {
            Ok(kartlar) => kartlar,
            Err(hata) => {
                self.hata = Some(format!("Timezones & DST grubu oluşturulamadı: {hata}"));
                self.grafik = None;
                self.timezones_dst_grafikleri.clear();
                cx.notify();
                return;
            }
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        let mut hata = None;
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(oluşturma_hatası) => {
                    hata = Some(format!(
                        "{} yüzeyi oluşturulamadı: {oluşturma_hatası}",
                        örnek.başlık()
                    ));
                    break;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, move |bu, _, olay: &GpuiGrafikOlayı, cx| {
                let Some(grup) = örnek.senkron_grubu() else {
                    bu.standart_grafik_olayını_işle(olay, cx);
                    return;
                };
                if bu.timezones_dst_senkronlanıyor {
                    return;
                }
                let yüzeyler = bu.timezones_dst_grafikleri.clone();
                let kaynak = yüzeyler
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone());
                match olay {
                    GpuiGrafikOlayı::Açıklamaİstendi => bu.açıklama_istendi = true,
                    GpuiGrafikOlayı::İmleçKonumuDeğişti => {
                        let yayın = kaynak
                            .as_ref()
                            .and_then(|grafik| grafik.read(cx).senkron_yayını());
                        bu.timezones_dst_senkronlanıyor = true;
                        for (hedef, hedef_grafik) in yüzeyler {
                            if hedef == örnek || hedef.senkron_grubu() != Some(grup) {
                                continue;
                            }
                            hedef_grafik.update(cx, |grafik, cx| {
                                if let Some((x, y, seri)) = yayın {
                                    grafik.senkron_imleci_ayarla(x, Some(y), seri, cx);
                                } else {
                                    grafik.senkron_imleci_temizle(cx);
                                }
                            });
                        }
                        bu.timezones_dst_senkronlanıyor = false;
                    }
                    GpuiGrafikOlayı::GörünümDeğişti { .. } => {
                        let x = kaynak
                            .as_ref()
                            .map(|grafik| grafik.read(cx).grafik().görünür_x_aralığı());
                        if let Some(x) = x {
                            bu.timezones_dst_senkronlanıyor = true;
                            for (hedef, hedef_grafik) in yüzeyler {
                                if hedef == örnek || hedef.senkron_grubu() != Some(grup) {
                                    continue;
                                }
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.görünür_x_aralığını_sessiz_ayarla(x, true, cx);
                                });
                            }
                            bu.timezones_dst_senkronlanıyor = false;
                        }
                    }
                    GpuiGrafikOlayı::DurumDeğişti => {
                        let görünür = kaynak
                            .as_ref()
                            .is_some_and(|grafik| grafik.read(cx).grafik().seri_görünür_mü(0));
                        bu.timezones_dst_senkronlanıyor = true;
                        for (hedef, hedef_grafik) in yüzeyler {
                            if hedef == örnek || hedef.senkron_grubu() != Some(grup) {
                                continue;
                            }
                            let _ = hedef_grafik.update(cx, |grafik, cx| {
                                grafik.seri_görünürlüğünü_ayarla(0, görünür, cx)
                            });
                        }
                        bu.timezones_dst_senkronlanıyor = false;
                    }
                    GpuiGrafikOlayı::İmleçDeğişti | GpuiGrafikOlayı::FareBırakıldı => {}
                }
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        if let Some(hata) = hata {
            self.hata = Some(hata);
            self.grafik = None;
            self.timezones_dst_grafikleri.clear();
        } else {
            self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
            self.timezones_dst_grafikleri = yüzeyler;
            self.hata = None;
        }
        cx.notify();
    }

    fn nearest_non_null_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let mut yüzeyler = Vec::with_capacity(NearestNonNullÖrneği::TÜMÜ.len());
        let mut hata = None;
        for örnek in NearestNonNullÖrneği::TÜMÜ {
            let sonuç = nearest_non_null_kartı(örnek)
                .and_then(|(seçenekler, veri)| Grafik::yeni(seçenekler, veri));
            let mut grafik = match sonuç {
                Ok(grafik) => grafik,
                Err(oluşturma_hatası) => {
                    hata = Some(format!(
                        "{} yüzeyi oluşturulamadı: {oluşturma_hatası}",
                        örnek.başlık()
                    ));
                    break;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        if let Some(hata) = hata {
            self.hata = Some(hata);
            self.grafik = None;
            self.nearest_non_null_grafikleri.clear();
        } else {
            self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
            self.nearest_non_null_grafikleri = yüzeyler;
            self.hata = None;
        }
        cx.notify();
    }

    fn align_data_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let başlangıç = Instant::now();
        let sonuç = align_data_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Align Data yüzeyleri oluşturulamadı: {hata}"));
            self.grafik = None;
            self.align_data_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.align_data_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.align_data_grafikleri = yüzeyler;
        self.align_data_kurulum_ms = Some(başlangıç.elapsed().as_secs_f64() * 1_000.0);
        self.hata = None;
        cx.notify();
    }

    fn custom_scales_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = custom_scales_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Custom Scales yüzeyleri oluşturulamadı: {hata}"));
            self.grafik = None;
            self.custom_scales_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.custom_scales_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.custom_scales_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn data_smoothing_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let mut yüzeyler = Vec::with_capacity(SmoothingÖrneği::TÜMÜ.len());
        let mut ölçümler = Vec::with_capacity(SmoothingÖrneği::TÜMÜ.len());
        for örnek in SmoothingÖrneği::TÜMÜ {
            let başlangıç = Instant::now();
            let sonuç = data_smoothing_kartı(örnek);
            let süre_ms = başlangıç.elapsed().as_secs_f64() * 1_000.0;
            let (seçenekler, veri) = match sonuç {
                Ok(kart) => kart,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.data_smoothing_grafikleri.clear();
                    self.data_smoothing_ölçümleri_ms.clear();
                    cx.notify();
                    return;
                }
            };
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.data_smoothing_grafikleri.clear();
                    self.data_smoothing_ölçümleri_ms.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
            ölçümler.push((örnek, süre_ms));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.data_smoothing_grafikleri = yüzeyler;
        self.data_smoothing_ölçümleri_ms = ölçümler;
        self.hata = None;
        cx.notify();
    }

    fn focus_cursor_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = focus_cursor_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Focus Cursor ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.focus_cursor_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.focus_cursor_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.focus_cursor_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn gradients_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = gradients_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Gradients ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.gradients_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.gradients_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.gradients_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn high_low_bands_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = high_low_bands_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("High/Low Bands ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.high_low_bands_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.high_low_bands_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.high_low_bands_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn latency_heatmap_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç =
            latency_heatmap_kartları(f64::from(self.latency_kova), f64::from(self.latency_ofset));
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Latency Heatmap ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.latency_heatmap_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.latency_heatmap_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.latency_heatmap_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn line_paths_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = line_paths_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Line Paths ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.line_paths_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.line_paths_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, move |bu, _, olay: &GpuiGrafikOlayı, cx| {
                if bu.line_paths_senkronlanıyor {
                    return;
                }
                match olay {
                    GpuiGrafikOlayı::İmleçKonumuDeğişti => {
                        let yayın = bu
                            .line_paths_grafikleri
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .and_then(|(_, grafik)| grafik.read(cx).senkron_veri_yayını());
                        let yüzeyler = bu.line_paths_grafikleri.clone();
                        bu.line_paths_senkronlanıyor = true;
                        for (hedef, hedef_grafik) in yüzeyler {
                            if hedef == örnek {
                                continue;
                            }
                            if let Some((x, y, seri)) = yayın {
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.senkron_veri_imleci_ayarla(x, y, seri, cx);
                                });
                            } else {
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.senkron_imleci_temizle(cx);
                                });
                            }
                        }
                        bu.line_paths_senkronlanıyor = false;
                    }
                    GpuiGrafikOlayı::Açıklamaİstendi => bu.açıklama_istendi = true,
                    GpuiGrafikOlayı::DurumDeğişti
                    | GpuiGrafikOlayı::GörünümDeğişti { .. }
                    | GpuiGrafikOlayı::İmleçDeğişti
                    | GpuiGrafikOlayı::FareBırakıldı => {}
                }
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.line_paths_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn log_scales_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = log_scales_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Log Scales ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.log_scales_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.log_scales_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.log_scales_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn log_scales2_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = log_scales2_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Log Scales 2 ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.log_scales2_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.log_scales2_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, move |bu, _, olay: &GpuiGrafikOlayı, cx| {
                let ters_çift = matches!(
                    örnek,
                    LogScales2Örneği::TersGiriş | LogScales2Örneği::TersÇıkış
                );
                if bu.log_scales2_senkronlanıyor {
                    return;
                }
                match olay {
                    GpuiGrafikOlayı::İmleçKonumuDeğişti if ters_çift => {
                        let yayın = bu
                            .log_scales2_grafikleri
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .and_then(|(_, grafik)| grafik.read(cx).senkron_veri_yayını());
                        let hedefler = bu.log_scales2_grafikleri.clone();
                        bu.log_scales2_senkronlanıyor = true;
                        for (hedef, hedef_grafik) in hedefler {
                            if hedef == örnek
                                || !matches!(
                                    hedef,
                                    LogScales2Örneği::TersGiriş | LogScales2Örneği::TersÇıkış
                                )
                            {
                                continue;
                            }
                            if let Some((x, _, seri)) = yayın {
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.senkron_veri_x_imleci_ayarla(x, seri, cx);
                                });
                            } else {
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.senkron_imleci_temizle(cx);
                                });
                            }
                        }
                        bu.log_scales2_senkronlanıyor = false;
                    }
                    GpuiGrafikOlayı::GörünümDeğişti { .. } if ters_çift => {
                        let x = bu
                            .log_scales2_grafikleri
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .map(|(_, grafik)| grafik.read(cx).grafik().görünür_x_aralığı());
                        let hedefler = bu.log_scales2_grafikleri.clone();
                        if let Some(x) = x {
                            bu.log_scales2_senkronlanıyor = true;
                            for (hedef, hedef_grafik) in hedefler {
                                if hedef == örnek
                                    || !matches!(
                                        hedef,
                                        LogScales2Örneği::TersGiriş
                                            | LogScales2Örneği::TersÇıkış
                                    )
                                {
                                    continue;
                                }
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.görünür_x_aralığını_sessiz_ayarla(x, true, cx);
                                });
                            }
                            bu.log_scales2_senkronlanıyor = false;
                        }
                    }
                    GpuiGrafikOlayı::Açıklamaİstendi => bu.açıklama_istendi = true,
                    GpuiGrafikOlayı::DurumDeğişti
                    | GpuiGrafikOlayı::GörünümDeğişti { .. }
                    | GpuiGrafikOlayı::İmleçDeğişti
                    | GpuiGrafikOlayı::İmleçKonumuDeğişti
                    | GpuiGrafikOlayı::FareBırakıldı => {}
                }
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.log_scales2_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn months_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = months_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Months ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.months_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("Months yüzeyi oluşturulamadı: {hata}"));
                    self.grafik = None;
                    self.months_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            yüzeyler.push(Self::bağlı_yüzey(grafik, cx));
        }
        self.grafik = yüzeyler.first().cloned();
        self.months_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn missing_data_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = missing_data_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Missing Data ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.missing_data_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("Missing Data yüzeyi oluşturulamadı: {hata}"));
                    self.grafik = None;
                    self.missing_data_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            yüzeyler.push((örnek, Self::bağlı_yüzey(grafik, cx)));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.missing_data_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn missing_data_serisini_değiştir(
        &mut self,
        örnek: MissingDataÖrneği,
        seri: usize,
        cx: &mut Context<Self>,
    ) {
        let Some((_, grafik)) = self
            .missing_data_grafikleri
            .iter()
            .find(|(kimlik, _)| *kimlik == örnek)
        else {
            return;
        };
        let görünür = grafik.read(cx).grafik().seri_görünür_mü(seri);
        if let Err(hata) = grafik.update(cx, |grafik, cx| {
            grafik.seri_görünürlüğünü_ayarla(seri, !görünür, cx)
        }) {
            self.hata = Some(format!("Missing Data serisi değiştirilemedi: {hata}"));
        }
        cx.notify();
    }

    fn path_gap_clip_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = path_gap_clip_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Path & Gap Clipping ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.path_gap_clip_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.path_gap_clip_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.path_gap_clip_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn pixel_align_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let canlı_akış = PixelAlignAkışı::canlı(0);
        let Ok(canlı_akış) = canlı_akış else {
            self.hata = canlı_akış
                .err()
                .map(|hata| format!("Pixel Align canlı saati kurulamadı: {hata}"));
            self.grafik = None;
            self.pixel_align_grafikleri.clear();
            cx.notify();
            return;
        };
        let canlı_aralık = canlı_akış.görünür_x_aralığı();
        let Ok(canlı_aralık) = canlı_aralık else {
            self.hata = canlı_aralık
                .err()
                .map(|hata| format!("Pixel Align canlı aralığı kurulamadı: {hata}"));
            self.grafik = None;
            self.pixel_align_grafikleri.clear();
            cx.notify();
            return;
        };
        let sonuç = pixel_align_kartları(0);
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Pixel Align ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.pixel_align_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.pixel_align_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.canlı_x_aralığını_ayarla(canlı_aralık);
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.pixel_align_akışı = Some(canlı_akış);
        self.pixel_align_son_kare = Some(Instant::now());
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.pixel_align_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn points_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = points_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Points ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.points_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.points_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            yüzeyler.push((örnek, Self::bağlı_yüzey(grafik, cx)));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.points_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn scales_dir_ori_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = scales_dir_ori_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç.err().map(|hata| {
                format!("Scales Direction & Orientation ailesi oluşturulamadı: {hata}")
            });
            self.grafik = None;
            self.scales_dir_ori_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.scales_dir_ori_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, move |bu, _, olay: &GpuiGrafikOlayı, cx| {
                if bu.scales_dir_ori_senkronlanıyor {
                    return;
                }
                match olay {
                    GpuiGrafikOlayı::İmleçKonumuDeğişti => {
                        let yayın = bu
                            .scales_dir_ori_grafikleri
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .and_then(|(_, grafik)| grafik.read(cx).senkron_veri_yayını());
                        let yüzeyler = bu.scales_dir_ori_grafikleri.clone();
                        bu.scales_dir_ori_senkronlanıyor = true;
                        for (hedef, hedef_grafik) in yüzeyler {
                            if hedef == örnek {
                                continue;
                            }
                            if let Some((x, y, seri)) = yayın {
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.senkron_veri_imleci_ayarla(x, y, seri, cx);
                                });
                            } else {
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.senkron_imleci_temizle(cx);
                                });
                            }
                        }
                        bu.scales_dir_ori_senkronlanıyor = false;
                    }
                    GpuiGrafikOlayı::FareBırakıldı => {
                        bu.scales_dir_ori_kilitli = !bu.scales_dir_ori_kilitli;
                        let kilitli = bu.scales_dir_ori_kilitli;
                        let yüzeyler = bu.scales_dir_ori_grafikleri.clone();
                        bu.scales_dir_ori_senkronlanıyor = true;
                        for (_, hedef) in yüzeyler {
                            hedef.update(cx, |grafik, cx| {
                                grafik.senkron_kilidi_ayarla(kilitli, cx);
                            });
                        }
                        bu.scales_dir_ori_senkronlanıyor = false;
                    }
                    GpuiGrafikOlayı::DurumDeğişti | GpuiGrafikOlayı::GörünümDeğişti { .. } =>
                    {
                        let aralıklar = bu
                            .scales_dir_ori_grafikleri
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .map(|(_, grafik)| {
                                let grafik = grafik.read(cx);
                                (
                                    grafik.grafik().görünür_x_aralığı(),
                                    grafik.grafik().görünür_y_aralığı(),
                                )
                            });
                        let yüzeyler = bu.scales_dir_ori_grafikleri.clone();
                        if let Some((x, y)) = aralıklar {
                            bu.scales_dir_ori_senkronlanıyor = true;
                            for (hedef, hedef_grafik) in yüzeyler {
                                if hedef != örnek {
                                    hedef_grafik.update(cx, |grafik, cx| {
                                        grafik
                                            .görünür_aralıkları_sessiz_ayarla(x, y, true, cx);
                                    });
                                }
                            }
                            bu.scales_dir_ori_senkronlanıyor = false;
                        }
                    }
                    GpuiGrafikOlayı::Açıklamaİstendi => {
                        bu.açıklama_istendi = true;
                    }
                    GpuiGrafikOlayı::İmleçDeğişti => {}
                }
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.scales_dir_ori_grafikleri = yüzeyler;
        self.scales_dir_ori_senkronlanıyor = false;
        self.scales_dir_ori_kilitli = false;
        self.hata = None;
        cx.notify();
    }

    fn scatter_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let mut yüzeyler = Vec::with_capacity(ScatterÖrneği::TÜMÜ.len());
        for örnek in ScatterÖrneği::TÜMÜ {
            let sonuç =
                scatter_kartı(örnek).and_then(|(seçenekler, veri)| Grafik::yeni(seçenekler, veri));
            let mut grafik = match sonuç {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.scatter_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.scatter_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn bars_grouped_stacked_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = bars_grouped_stacked_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Bars Grouped / Stacked ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.bars_grouped_stacked_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.bars_grouped_stacked_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.bars_grouped_stacked_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn bars_serisini_değiştir(
        &mut self,
        örnek: ÇubukÖrneği,
        seri: usize,
        cx: &mut Context<Self>,
    ) {
        let Some((_, grafik)) = self
            .bars_grouped_stacked_grafikleri
            .iter()
            .find(|(kimlik, _)| *kimlik == örnek)
        else {
            return;
        };
        let görünür = grafik.read(cx).grafik().seri_görünür_mü(seri);
        let sonuç = grafik.update(cx, |grafik, cx| {
            grafik.seri_görünürlüğünü_ayarla(seri, !görünür, cx)
        });
        if let Err(hata) = sonuç {
            self.hata = Some(format!("{} setSeries uygulanamadı: {hata}", örnek.başlık()));
        }
        cx.notify();
    }

    fn bars_values_autosize_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = bars_values_autosize_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Bars Values AutoSize ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.bars_values_autosize_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (yön, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!(
                        "{} Bars Values yüzeyi oluşturulamadı: {hata}",
                        if yön == ÇubukYönü::Dikey {
                            "Dikey"
                        } else {
                            "Yatay"
                        }
                    ));
                    self.grafik = None;
                    self.bars_values_autosize_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((yön, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.bars_values_autosize_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn bars_values_serisini_değiştir(&mut self, yön: ÇubukYönü, cx: &mut Context<Self>) {
        let Some((_, grafik)) = self
            .bars_values_autosize_grafikleri
            .iter()
            .find(|(kimlik, _)| *kimlik == yön)
        else {
            return;
        };
        let görünür = grafik.read(cx).grafik().seri_görünür_mü(0);
        if let Err(hata) = grafik.update(cx, |grafik, cx| {
            grafik.seri_görünürlüğünü_ayarla(0, !görünür, cx)
        }) {
            self.hata = Some(format!("Bars Values setSeries uygulanamadı: {hata}"));
        }
        cx.notify();
    }

    fn box_whisker_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = box_whisker_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Box & Whisker ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.box_whisker_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (benchmark, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{benchmark} oluşturulamadı: {hata}"));
                    self.grafik = None;
                    self.box_whisker_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((benchmark, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.box_whisker_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn soft_minmax_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = soft_minmax_kartları(12.0);
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Soft Min/Max ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.soft_minmax_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.soft_minmax_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.soft_minmax_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn sparklines_bars_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = sparklines_bars_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Sparklines Bars ailesi oluşturulamadı: {hata}"));
            self.grafik = None;
            self.sparklines_bars_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.sparklines_bars_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            yüzeyler.push((örnek, Self::bağlı_yüzey(grafik, cx)));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.sparklines_bars_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn sparklines_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = sparklines_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Sparklines tablosu oluşturulamadı: {hata}"));
            self.grafik = None;
            self.sparklines_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.sparklines_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            yüzeyler.push((örnek, Self::bağlı_yüzey(grafik, cx)));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.sparklines_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn sparse_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = sparse_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Sparse grubu oluşturulamadı: {hata}"));
            self.grafik = None;
            self.sparse_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.sparse_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, move |bu, _, olay: &GpuiGrafikOlayı, cx| {
                if bu.scales_dir_ori_senkronlanıyor {
                    return;
                }
                match olay {
                    GpuiGrafikOlayı::İmleçKonumuDeğişti => {
                        let yayın = bu
                            .sparse_grafikleri
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .and_then(|(_, grafik)| grafik.read(cx).senkron_yayını());
                        let yüzeyler = bu.sparse_grafikleri.clone();
                        bu.scales_dir_ori_senkronlanıyor = true;
                        for (hedef, hedef_grafik) in yüzeyler {
                            if hedef == örnek {
                                continue;
                            }
                            if let Some((x, y, seri)) = yayın {
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.senkron_veri_imleci_ayarla(x, y, seri, cx);
                                });
                            } else {
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.senkron_imleci_temizle(cx);
                                });
                            }
                        }
                        bu.scales_dir_ori_senkronlanıyor = false;
                    }
                    GpuiGrafikOlayı::DurumDeğişti | GpuiGrafikOlayı::GörünümDeğişti { .. } =>
                    {
                        let aralıklar = bu
                            .sparse_grafikleri
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .map(|(_, grafik)| {
                                let grafik = grafik.read(cx);
                                (
                                    grafik.grafik().görünür_x_aralığı(),
                                    grafik.grafik().görünür_y_aralığı(),
                                )
                            });
                        if let Some((x, y)) = aralıklar {
                            let yüzeyler = bu.sparse_grafikleri.clone();
                            bu.scales_dir_ori_senkronlanıyor = true;
                            for (hedef, hedef_grafik) in yüzeyler {
                                if hedef != örnek {
                                    hedef_grafik.update(cx, |grafik, cx| {
                                        grafik
                                            .görünür_aralıkları_sessiz_ayarla(x, y, true, cx);
                                    });
                                }
                            }
                            bu.scales_dir_ori_senkronlanıyor = false;
                        }
                    }
                    GpuiGrafikOlayı::Açıklamaİstendi => bu.açıklama_istendi = true,
                    GpuiGrafikOlayı::İmleçDeğişti | GpuiGrafikOlayı::FareBırakıldı => {}
                }
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.sparse_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn stacked_series_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let sonuç = stacked_series_kartları();
        let Ok(kartlar) = sonuç else {
            self.hata = sonuç
                .err()
                .map(|hata| format!("Stacked Series grubu oluşturulamadı: {hata}"));
            self.grafik = None;
            self.stacked_series_grafikleri.clear();
            cx.notify();
            return;
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.stacked_series_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.stacked_series_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn stream_data_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let grup = match StreamDataGrubu::yeni() {
            Ok(grup) => grup,
            Err(hata) => {
                self.hata = Some(format!("Data Stream grubu oluşturulamadı: {hata}"));
                self.grafik = None;
                self.stream_data_grubu = None;
                self.stream_data_grafikleri.clear();
                cx.notify();
                return;
            }
        };
        let kartlar = match grup.kartları() {
            Ok(kartlar) => kartlar,
            Err(hata) => {
                self.hata = Some(format!("Data Stream yüzeyleri üretilemedi: {hata}"));
                self.grafik = None;
                self.stream_data_grubu = None;
                self.stream_data_grafikleri.clear();
                cx.notify();
                return;
            }
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.hata = Some(format!("{} yüzeyi oluşturulamadı: {hata}", örnek.başlık()));
                    self.grafik = None;
                    self.stream_data_grubu = None;
                    self.stream_data_grafikleri.clear();
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.stream_data_grubu = Some(grup);
        self.stream_data_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn thin_bars_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let kartlar = match thin_bars_stroke_fill_kartları() {
            Ok(kartlar) => kartlar,
            Err(hata) => {
                self.grafik = None;
                self.thin_bars_grafikleri.clear();
                self.hata = Some(format!("Thin Bars grubu oluşturulamadı: {hata}"));
                cx.notify();
                return;
            }
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.grafik = None;
                    self.thin_bars_grafikleri.clear();
                    self.hata = Some(format!(
                        "{} Thin Bars yüzeyi oluşturulamadı: {hata}",
                        örnek.başlık()
                    ));
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.thin_bars_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn time_periods_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let kartlar = match time_periods_kartları() {
            Ok(kartlar) => kartlar,
            Err(hata) => {
                self.grafik = None;
                self.time_periods_grafikleri.clear();
                self.hata = Some(format!("Time Periods grubu oluşturulamadı: {hata}"));
                cx.notify();
                return;
            }
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.grafik = None;
                    self.time_periods_grafikleri.clear();
                    self.hata = Some(format!(
                        "{} Time Periods yüzeyi oluşturulamadı: {hata}",
                        örnek.başlık()
                    ));
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.time_periods_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn timeline_discrete_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let kartlar = match timeline_discrete_kartları() {
            Ok(kartlar) => kartlar,
            Err(hata) => {
                self.grafik = None;
                self.timeline_discrete_grafikleri.clear();
                self.hata = Some(format!("Timeline / Discrete grubu oluşturulamadı: {hata}"));
                cx.notify();
                return;
            }
        };
        let mut yüzeyler = Vec::with_capacity(kartlar.len());
        for (örnek, seçenekler, veri) in kartlar {
            let mut grafik = match Grafik::yeni(seçenekler, veri) {
                Ok(grafik) => grafik,
                Err(hata) => {
                    self.grafik = None;
                    self.timeline_discrete_grafikleri.clear();
                    self.hata = Some(format!(
                        "{} Timeline / Discrete yüzeyi oluşturulamadı: {hata}",
                        örnek.başlık()
                    ));
                    cx.notify();
                    return;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                bu.standart_grafik_olayını_işle(olay, cx);
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
        self.timeline_discrete_grafikleri = yüzeyler;
        self.hata = None;
        cx.notify();
    }

    fn grafiği_yenile(&mut self, nokta_sayısı: usize, cx: &mut Context<Self>) {
        self.nokta_sayısı = nokta_sayısı;
        let sonuç = if self.aktif_kart == KartKimliği::SineStream {
            self.sine_akışı
                .as_ref()
                .ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
                    varlık: "SineAkışı",
                    açıklama: "ilk Grafik için akış durumu bulunamadı".to_string(),
                })
                .and_then(SineAkışı::kartı)
                .and_then(|(seçenekler, veri)| Grafik::yeni(seçenekler, veri))
        } else {
            grafik_oluştur(
                self.aktif_kart,
                self.no_data_örneği,
                nokta_sayısı,
                self.autosize_kuvvet,
                self.latency_kova,
                self.latency_ofset,
                140,
            )
        };
        match sonuç {
            Ok(mut yeni) => {
                yeni.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
                yeni.tekerlek_odaksız_etkileşimi_ayarla(self.tekerlek_odaksız_etkin);
                yeni.kırılım_noktalarını_göster(self.içi_boş_noktalar_görünür);
                yeni.imleç_noktalarını_göster(self.dolu_noktalar_görünür);
                if let Some(grafik) = &self.grafik {
                    grafik.update(cx, |grafik, cx| grafik.grafiği_ayarla(yeni, cx));
                } else {
                    let grafik = cx.new(|_| GpuiGrafik::yeni(yeni));
                    cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                        bu.standart_grafik_olayını_işle(olay, cx);
                    })
                    .detach();
                    self.grafik = Some(grafik);
                }
                self.hata = None;
            }
            Err(hata) => {
                self.grafik = None;
                self.hata = Some(format!("Grafik oluşturulamadı: {hata}"));
            }
        }
        cx.notify();
    }

    fn no_data_örneğini_seç(&mut self, örnek: NoDataÖrneği, cx: &mut Context<Self>) {
        if self.no_data_örneği == örnek {
            return;
        }
        self.no_data_örneği = örnek;
        if self.aktif_kart == KartKimliği::NoData {
            self.grafiği_yenile(self.nokta_sayısı, cx);
        } else {
            cx.notify();
        }
    }

    fn sync_cursor_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let mut yüzeyler = Vec::with_capacity(SyncCursorÖrneği::TÜMÜ.len());
        let mut hata = None;
        for örnek in SyncCursorÖrneği::TÜMÜ {
            let sonuç = sync_cursor_kartı(örnek)
                .and_then(|(seçenekler, veri)| Grafik::yeni(seçenekler, veri));
            let mut grafik = match sonuç {
                Ok(grafik) => grafik,
                Err(oluşturma_hatası) => {
                    hata = Some(format!(
                        "{} Sync Cursor yüzeyi oluşturulamadı: {oluşturma_hatası}",
                        örnek.başlık()
                    ));
                    break;
                }
            };
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                    cx.notify();
                }
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        if let Some(hata) = hata {
            self.hata = Some(hata);
            self.grafik = None;
            self.sync_cursor_grafikleri.clear();
            self.sync_cursor_çekirdek_grupları.clear();
        } else {
            let ana_grup = cx.new(|_| {
                GpuiGrafikGrubu::yeni(
                    GpuiGrafikGrupAyarları::default()
                        .seçim_görünümü(self.sync_cursor_grubu.fare_basma_bırakma_senkron()),
                )
            });
            let uyumsuz_grup = cx.new(|_| {
                GpuiGrafikGrubu::yeni(
                    GpuiGrafikGrupAyarları::default()
                        .seri_eşleme(GpuiSeriEşleme::Etiket)
                        .imleç_kilidi(false),
                )
            });
            for (örnek, grafik) in &yüzeyler {
                let grup = if matches!(
                    örnek,
                    SyncCursorÖrneği::Cpu | SyncCursorÖrneği::Ram | SyncCursorÖrneği::Tcp
                ) {
                    &ana_grup
                } else {
                    &uyumsuz_grup
                };
                grup.update(cx, |grup, cx| {
                    grup.grafik_ekle(örnek.kimlik(), grafik.clone(), cx);
                });
            }
            ana_grup.update(cx, |grup, _| {
                grup.etkinliği_ayarla(self.sync_cursor_grubu.senkron());
            });
            self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
            self.sync_cursor_grafikleri = yüzeyler;
            self.sync_cursor_çekirdek_grupları = vec![ana_grup, uyumsuz_grup];
            self.hata = None;
        }
        cx.notify();
    }

    fn sync_cursor_senkronunu_değiştir(&mut self, cx: &mut Context<Self>) {
        let etkin = !self.sync_cursor_grubu.senkron();
        self.sync_cursor_grubu.senkronu_ayarla(etkin);
        if let Some(grup) = self.sync_cursor_çekirdek_grupları.first() {
            grup.update(cx, |grup, _| {
                grup.etkinliği_ayarla(etkin);
            });
        }
        cx.notify();
    }

    fn sync_cursor_serisini_değiştir(
        &mut self,
        örnek: SyncCursorÖrneği,
        seri: usize,
        cx: &mut Context<Self>,
    ) {
        let Some((_, kaynak)) = self
            .sync_cursor_grafikleri
            .iter()
            .find(|(kimlik, _)| *kimlik == örnek)
        else {
            return;
        };
        let görünür = !kaynak
            .read(cx)
            .grafik()
            .seri_seçenekleri()
            .get(seri)
            .is_some_and(|seçenek| seçenek.göster);
        if let Err(hata) = kaynak.update(cx, |grafik, cx| {
            grafik.seri_görünürlüğünü_ayarla(seri, görünür, cx)
        }) {
            self.hata = Some(format!(
                "Sync Cursor seri görünürlüğü değiştirilemedi: {hata}"
            ));
            cx.notify();
            return;
        }
        cx.notify();
    }

    fn sync_cursor_fare_filtresini_değiştir(&mut self, cx: &mut Context<Self>) {
        let etkin = !self.sync_cursor_grubu.fare_basma_bırakma_senkron();
        self.sync_cursor_grubu
            .fare_basma_bırakma_senkronunu_ayarla(etkin);
        if let Some(grup) = self.sync_cursor_çekirdek_grupları.first() {
            grup.update(cx, |grup, _| {
                grup.seçim_görünümünü_ayarla(etkin);
            });
        }
        cx.notify();
    }

    fn kartı_seç(&mut self, kart: KartKimliği, cx: &mut Context<Self>) {
        debug_assert!(
            KATALOG_KARTLARI
                .iter()
                .any(|tanım| tanım.kimlik == kart.ana_kart()),
            "kayıt defterinde bulunmayan kart seçildi"
        );
        debug_assert_eq!(
            KartKimliği::slugdan(kart.slug()),
            Some(kart.ana_kart()),
            "canonical slug kayıt defterindeki ana karta dönmelidir"
        );
        if self.aktif_kart == kart {
            return;
        }
        if let Err(hata) = web_köprüsü::kart_url_adresini_güncelle(kart) {
            self.hata = Some(hata);
        }
        izleme::kart_değişti(self.aktif_kart.slug(), kart.slug());
        self.aktif_kart = kart;
        if let Some(indeks) = KATALOG_KARTLARI
            .iter()
            .position(|tanım| tanım.kimlik == kart.ana_kart())
        {
            self.kart_listesi_kaydırma_bekliyor = Some(indeks);
        }
        self.svg_kayıt_baytı = None;
        self.thin_bars_liste_durumu = None;
        self.timezones_dst_liste_durumu = None;
        self.kart_tanımı_açık = false;
        self.kullanım_rehberi_açık = false;
        self.arcsinh_kuvvet = 0;
        self.autosize_kuvvet = 0;
        self.axis_autosize_akışı =
            (kart == KartKimliği::AxisAutosize).then(AxisAutosizeAkışı::yeni);
        self.latency_kova = 5;
        self.latency_ofset = 0;
        self.açıklama_istendi = false;
        self.açıklama_odak_bekliyor = false;
        self.açıklama_metni
            .update(cx, |alan, cx| alan.metni_ayarla("", cx));
        self.cursor_bind_tıklama_sayısı = 0;
        self.dinamik_seri_sayacı = 0;
        self.align_data_zamanlayıcısı = None;
        self.pixel_align_akışı = None;
        self.pixel_align_son_kare = None;
        self.sine_kare_bekleniyor = false;
        self.sine_akışı = if kart == KartKimliği::SineStream {
            match SineAkışı::yeni() {
                Ok(akış) => Some(akış),
                Err(hata) => {
                    self.hata = Some(format!("Sine Stream başlatılamadı: {hata}"));
                    None
                }
            }
        } else {
            None
        };
        self.stream_data_grubu = None;
        self.soft_minmax_akışı =
            matches!(kart, KartKimliği::SoftMinMax(_)).then(SoftMinMaxAkışı::yeni);
        self.boyut_senkron_akışı =
            (kart == KartKimliği::UpdateCursorSelectResize).then(BoyutSenkronAkışı::yeni);
        self.y_shifted_series_akışı = if kart == KartKimliği::YShiftedSeries {
            match YShiftedSeriesAkışı::yeni() {
                Ok(akış) => Some(akış),
                Err(hata) => {
                    self.hata = Some(format!("Y-shifted Series başlatılamadı: {hata}"));
                    None
                }
            }
        } else {
            None
        };
        self.soft_minmax_çalışıyor = false;
        let etkileşimler = kart.etkileşimler();
        self.tekerlek_etkin = etkileşimler.tekerlek_etkileşimi;
        self.tekerlek_odaksız_etkin = etkileşimler.tekerlek_odaksız_etkileşim;
        self.tekerlek_anahtarı.update(cx, |anahtar, cx| {
            anahtar.ayarla(etkileşimler.tekerlek_etkileşimi, cx);
            anahtar.devre_disi_ayarla(false, cx);
        });
        self.tekerlek_odaksız_anahtarı.update(cx, |anahtar, cx| {
            anahtar.ayarla(etkileşimler.tekerlek_odaksız_etkileşim, cx);
            anahtar.devre_disi_ayarla(false, cx);
        });
        self.align_data_grafikleri.clear();
        self.align_data_kurulum_ms = None;
        self.custom_scales_grafikleri.clear();
        self.data_smoothing_grafikleri.clear();
        self.data_smoothing_ölçümleri_ms.clear();
        self.focus_cursor_grafikleri.clear();
        self.gradients_grafikleri.clear();
        self.high_low_bands_grafikleri.clear();
        self.latency_heatmap_grafikleri.clear();
        self.line_paths_grafikleri.clear();
        self.line_paths_senkronlanıyor = false;
        self.log_scales_grafikleri.clear();
        self.log_scales2_grafikleri.clear();
        self.log_scales2_senkronlanıyor = false;
        self.scales_dir_ori_grafikleri.clear();
        self.scatter_grafikleri.clear();
        self.bars_grouped_stacked_grafikleri.clear();
        self.bars_values_autosize_grafikleri.clear();
        self.box_whisker_grafikleri.clear();
        self.soft_minmax_grafikleri.clear();
        self.sparklines_bars_grafikleri.clear();
        self.sparklines_grafikleri.clear();
        self.sparse_grafikleri.clear();
        self.stacked_series_grafikleri.clear();
        self.stream_data_grafikleri.clear();
        self.thin_bars_grafikleri.clear();
        self.time_periods_grafikleri.clear();
        self.timeline_discrete_grafikleri.clear();
        self.timezones_dst_grafikleri.clear();
        self.missing_data_grafikleri.clear();
        if kart == KartKimliği::AlignDataCost {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.align_data_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::CustomScales {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.custom_scales_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::DataSmoothing {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.data_smoothing_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::FocusCursor {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.focus_cursor_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::Gradients {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.gradients_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::HighLowBands {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.high_low_bands_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::LatencyHeatmap {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.latency_heatmap_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::LinePaths {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.line_paths_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::LogScales {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.log_scales_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::LogScales2 {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.log_scales2_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::SyncCursor {
            self.sync_cursor_grubu = SyncCursorGrubu::yeni();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.sync_cursor_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::TimeseriesDiscrete {
            self.sync_cursor_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.timeseries_discrete_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::TimezonesDst {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.timezones_dst_senkronlanıyor = false;
            self.timezones_dst_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::NearestNonNull {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.nearest_non_null_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::MissingData {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.missing_data_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::Months {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.months_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::PathGapClip {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.path_gap_clip_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::PixelAlign {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.points_grafikleri.clear();
            self.pixel_align_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::Points {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::ScalesDirOri {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.scales_dir_ori_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::Scatter {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.scatter_yüzeylerini_oluştur(cx);
        } else if matches!(kart, KartKimliği::Bars(_)) {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.bars_grouped_stacked_yüzeylerini_oluştur(cx);
        } else if matches!(kart, KartKimliği::BarsValuesAutosize(_)) {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.bars_values_autosize_yüzeylerini_oluştur(cx);
        } else if matches!(kart, KartKimliği::BoxWhisker(_)) {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.box_whisker_yüzeylerini_oluştur(cx);
        } else if matches!(kart, KartKimliği::SoftMinMax(_)) {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.soft_minmax_yüzeylerini_oluştur(cx);
        } else if matches!(kart, KartKimliği::SparklinesBars(_)) {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.sparklines_bars_yüzeylerini_oluştur(cx);
        } else if matches!(kart, KartKimliği::Sparklines(_)) {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.sparklines_yüzeylerini_oluştur(cx);
        } else if matches!(kart, KartKimliği::Sparse(_)) {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.sparse_yüzeylerini_oluştur(cx);
        } else if matches!(kart, KartKimliği::StackedSeries(_)) {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.stacked_series_yüzeylerini_oluştur(cx);
        } else if matches!(kart, KartKimliği::StreamData(_)) {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.stream_data_yüzeylerini_oluştur(cx);
        } else if matches!(kart, KartKimliği::ThinBars(_)) {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.thin_bars_yüzeylerini_oluştur(cx);
        } else if matches!(kart, KartKimliği::TimePeriods(_)) {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.time_periods_yüzeylerini_oluştur(cx);
        } else if matches!(kart, KartKimliği::TimelineDiscrete(_)) {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.timeline_discrete_yüzeylerini_oluştur(cx);
        } else {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.nearest_non_null_grafikleri.clear();
            self.months_grafikleri.clear();
            self.path_gap_clip_grafikleri.clear();
            self.pixel_align_grafikleri.clear();
            self.points_grafikleri.clear();
            self.grafiği_yenile(self.nokta_sayısı, cx);
        }
        if kart == KartKimliği::AxisAutosize {
            self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
                loop {
                    cx.background_executor()
                        .timer(Duration::from_millis(AXIS_AUTOSIZE_ARALIK_MS))
                        .await;
                    let devam = bu
                        .update(cx, |bu, cx| {
                            if bu.aktif_kart != KartKimliği::AxisAutosize {
                                return false;
                            }
                            let sonraki = bu
                                .axis_autosize_akışı
                                .as_mut()
                                .ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
                                    varlık: "AxisAutosizeAkışı",
                                    açıklama: "masaüstü akış durumu bulunamadı".to_string(),
                                })
                                .and_then(AxisAutosizeAkışı::ilerlet);
                            let Some((çarpan, veri)) = (match sonraki {
                                Ok(sonraki) => sonraki,
                                Err(hata) => {
                                    bu.hata = Some(format!("Axis AutoSize güncellenemedi: {hata}"));
                                    cx.notify();
                                    return false;
                                }
                            }) else {
                                return false;
                            };
                            let Some(grafik) = &bu.grafik else {
                                bu.hata =
                                    Some("Axis AutoSize grafik yüzeyi bulunamadı".to_string());
                                cx.notify();
                                return false;
                            };
                            if let Err(hata) = grafik.update(cx, |grafik, cx| {
                                grafik.canlı_veriyi_x_etiket_çarpanında_ayarla(veri, çarpan, cx)
                            }) {
                                bu.hata = Some(format!("Axis AutoSize güncellenemedi: {hata}"));
                                cx.notify();
                                return false;
                            }
                            bu.autosize_kuvvet = çarpan.log10().round() as i32;
                            bu.hata = None;
                            cx.notify();
                            true
                        })
                        .unwrap_or(false);
                    if !devam {
                        break;
                    }
                }
            }));
        } else if kart == KartKimliği::AlignDataCost {
            self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
                let mut etkin = false;
                loop {
                    cx.background_executor().timer(Duration::from_secs(1)).await;
                    etkin = !etkin;
                    let devam = bu
                        .update(cx, |bu, cx| {
                            if bu.aktif_kart != kart {
                                return false;
                            }
                            if let Some((AlignDataÖrneği::HizalamaMaliyeti, grafik)) =
                                bu.align_data_grafikleri.first()
                            {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.boşlukları_birleştir_ayarla(etkin, cx);
                                });
                            }
                            true
                        })
                        .unwrap_or(false);
                    if !devam {
                        break;
                    }
                }
            }));
        } else if kart == KartKimliği::PathGapClip {
            self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
                let mut etkin = false;
                loop {
                    cx.background_executor().timer(Duration::from_secs(1)).await;
                    etkin = !etkin;
                    let devam = bu
                        .update(cx, |bu, cx| {
                            if bu.aktif_kart != KartKimliği::PathGapClip {
                                return false;
                            }
                            for (örnek, grafik) in &bu.path_gap_clip_grafikleri {
                                if örnek.boşluk_animasyonunda_mı() {
                                    grafik.update(cx, |grafik, cx| {
                                        grafik.boşlukları_birleştir_ayarla(etkin, cx);
                                    });
                                }
                            }
                            true
                        })
                        .unwrap_or(false);
                    if !devam {
                        break;
                    }
                }
            }));
        } else if kart == KartKimliği::PixelAlign {
            self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
                loop {
                    cx.background_executor()
                        .timer(Duration::from_millis(16))
                        .await;
                    let devam = bu
                        .update(cx, |bu, cx| {
                            if bu.aktif_kart != KartKimliği::PixelAlign {
                                return false;
                            }
                            let şimdi = Instant::now();
                            let geçen_ms = bu.pixel_align_son_kare.map_or(16.0, |önceki| {
                                şimdi.duration_since(önceki).as_secs_f64() * 1_000.0
                            });
                            bu.pixel_align_son_kare = Some(şimdi);
                            let Some(akış) = bu.pixel_align_akışı.as_mut() else {
                                return false;
                            };
                            let veri_değişti = akış.kareyi_ilerlet(geçen_ms);
                            let Ok(aralık) = akış.görünür_x_aralığı() else {
                                return false;
                            };
                            let veri = veri_değişti.then(|| akış.veri()).transpose();
                            let Ok(veri) = veri else {
                                return false;
                            };
                            for (_, grafik) in &bu.pixel_align_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    if let Some(veri) = &veri {
                                        let _ = grafik.canlı_veriyi_x_aralığında_ayarla(
                                            veri.clone(),
                                            aralık,
                                            cx,
                                        );
                                    } else {
                                        grafik.canlı_x_aralığını_ayarla(aralık, cx);
                                    }
                                });
                            }
                            true
                        })
                        .unwrap_or(false);
                    if !devam {
                        break;
                    }
                }
            }));
        } else if matches!(kart, KartKimliği::StreamData(_)) {
            self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
                loop {
                    cx.background_executor()
                        .timer(Duration::from_millis(STREAM_DATA_ARALIK_MS))
                        .await;
                    let devam = bu
                        .update(cx, |bu, cx| {
                            if bu.aktif_kart != kart {
                                return false;
                            }
                            let sonuç = bu.stream_data_grubu.as_mut().map_or_else(
                                || {
                                    Err(UplotHatası::GeçersizKaynakVeri {
                                        varlık: "StreamDataGrubu",
                                        açıklama: "masaüstü akış grubu bulunamadı".to_string(),
                                    })
                                },
                                |grup| {
                                    if !grup.ilerlet() {
                                        return Ok(None);
                                    }
                                    grup.kartları().map(Some)
                                },
                            );
                            match sonuç {
                                Ok(Some(kartlar)) => {
                                    for (örnek, _, veri) in kartlar {
                                        let Some((_, grafik)) = bu
                                            .stream_data_grafikleri
                                            .iter()
                                            .find(|(kimlik, _)| *kimlik == örnek)
                                        else {
                                            bu.hata = Some(format!(
                                                "{} Data Stream yüzeyi bulunamadı",
                                                örnek.başlık()
                                            ));
                                            return false;
                                        };
                                        let güncellendi = grafik.update(cx, |grafik, cx| {
                                            grafik.canlı_veriyi_ayarla(veri, cx)
                                        });
                                        if let Err(hata) = güncellendi {
                                            bu.hata = Some(format!(
                                                "{} Data Stream yüzeyi güncellenemedi: {hata}",
                                                örnek.başlık()
                                            ));
                                            return false;
                                        }
                                    }
                                    bu.hata = None;
                                    true
                                }
                                Ok(None) => false,
                                Err(hata) => {
                                    bu.hata =
                                        Some(format!("Data Stream verisi üretilemedi: {hata}"));
                                    false
                                }
                            }
                        })
                        .unwrap_or(false);
                    if !devam {
                        break;
                    }
                }
            }));
        } else if kart == KartKimliği::Tooltips {
            let yeniden_kurma_ms = self.grafik.as_ref().and_then(|grafik| {
                grafik
                    .read(cx)
                    .grafik()
                    .tooltip_düzeni()
                    .and_then(|düzen| düzen.yeniden_kurma_ms)
            });
            if let Some(yeniden_kurma_ms) = yeniden_kurma_ms {
                self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
                    loop {
                        cx.background_executor()
                            .timer(Duration::from_millis(yeniden_kurma_ms))
                            .await;
                        let devam = bu
                            .update(cx, |bu, cx| {
                                if bu.aktif_kart != KartKimliği::Tooltips {
                                    return false;
                                }
                                bu.grafiği_yenile(bu.nokta_sayısı, cx);
                                true
                            })
                            .unwrap_or(false);
                        if !devam {
                            break;
                        }
                    }
                }));
            }
        } else if kart == KartKimliği::UpdateCursorSelectResize {
            self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
                loop {
                    cx.background_executor()
                        .timer(Duration::from_millis(UPDATE_CURSOR_SELECT_RESIZE_ARALIK_MS))
                        .await;
                    let devam = bu
                        .update(cx, |bu, cx| {
                            if bu.aktif_kart != KartKimliği::UpdateCursorSelectResize {
                                return false;
                            }
                            let Some(akış) = bu.boyut_senkron_akışı.as_mut() else {
                                bu.hata = Some("Boyut senkron akışı bulunamadı".to_string());
                                return false;
                            };
                            let Some(boyut) = akış.ilerlet() else {
                                return true;
                            };
                            let Some(grafik) = &bu.grafik else {
                                return false;
                            };
                            match grafik
                                .update(cx, |grafik, cx| grafik.boyutu_ayarla(boyut, boyut, cx))
                            {
                                Ok(_) => true,
                                Err(hata) => {
                                    bu.hata =
                                        Some(format!("Boyut senkron akışı güncellenemedi: {hata}"));
                                    false
                                }
                            }
                        })
                        .unwrap_or(false);
                    if !devam {
                        break;
                    }
                }
            }));
        } else if kart == KartKimliği::YShiftedSeries {
            self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
                loop {
                    cx.background_executor()
                        .timer(Duration::from_millis(Y_SHIFTED_SERIES_ARALIK_MS))
                        .await;
                    let devam = bu
                        .update(cx, |bu, cx| {
                            if bu.aktif_kart != KartKimliği::YShiftedSeries {
                                return false;
                            }
                            let sonuç = bu.y_shifted_series_akışı.as_mut().map_or_else(
                                || {
                                    Err(UplotHatası::GeçersizKaynakVeri {
                                        varlık: "YShiftedSeriesAkışı",
                                        açıklama: "masaüstü akış durumu bulunamadı".to_string(),
                                    })
                                },
                                YShiftedSeriesAkışı::ilerlet_güncellemesi,
                            );
                            let güncelleme = match sonuç {
                                Ok(güncelleme) => güncelleme,
                                Err(hata) => {
                                    bu.hata = Some(format!("Y-shifted Series üretilemedi: {hata}"));
                                    return false;
                                }
                            };
                            let Some(grafik) = &bu.grafik else {
                                return false;
                            };
                            let sonuç = grafik.update(cx, |grafik, cx| {
                                grafik.veriyi_y_sunumunda_ayarla(
                                    güncelleme.veri,
                                    güncelleme.y_aralığı,
                                    güncelleme.y_özel_etiketler,
                                    güncelleme.dolgu_tabanları,
                                    cx,
                                )
                            });
                            if let Err(hata) = sonuç {
                                bu.hata = Some(format!("Y-shifted Series güncellenemedi: {hata}"));
                                return false;
                            }
                            bu.hata = None;
                            true
                        })
                        .unwrap_or(false);
                    if !devam {
                        break;
                    }
                }
            }));
        } else if matches!(kart, KartKimliği::SyncYZero(_)) {
            self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
                for aşama in [SyncYZeroAşaması::Simetrik, SyncYZeroAşaması::SıfırHizalı] {
                    cx.background_executor().timer(Duration::from_secs(3)).await;
                    let devam = bu
                        .update(cx, |bu, cx| {
                            if !matches!(bu.aktif_kart, KartKimliği::SyncYZero(_)) {
                                return false;
                            }
                            let aralıklar = match sync_y_zero_aralıkları(aşama) {
                                Ok([y, y2, y3]) => [("y", y), ("y2", y2), ("y3", y3)],
                                Err(hata) => {
                                    bu.hata =
                                        Some(format!("Sync Y Zero aralığı üretilemedi: {hata}"));
                                    return false;
                                }
                            };
                            let Some(grafik) = &bu.grafik else {
                                return false;
                            };
                            if let Err(hata) = grafik.update(cx, |grafik, cx| {
                                grafik.y_ölçek_aralıklarını_ayarla(&aralıklar, cx)
                            }) {
                                bu.hata = Some(format!("Sync Y Zero aşaması uygulanamadı: {hata}"));
                                return false;
                            }
                            bu.aktif_kart = KartKimliği::SyncYZero(aşama);
                            bu.hata = None;
                            cx.notify();
                            true
                        })
                        .unwrap_or(false);
                    if !devam {
                        break;
                    }
                }
            }));
        }
        self.nokta_gösterimlerini_uygula(
            self.içi_boş_noktalar_görünür,
            self.dolu_noktalar_görünür,
            cx,
        );
        self.tekerlek_odaksız_etkileşimi_uygula(self.tekerlek_odaksız_etkin, cx);
        // Yeni kartın serileri ve değerleri farklı; lejant artık kökün
        // `render`'ında hesaplanmadığından burada bir kez tazelenmeli. Konum
        // seçimi kartın kendi tanımına bırakılır, önceki kartın denetimi
        // taşınmaz.
        self.lejant_konumu_seçimi = None;
        self.lejant_yüzeyi = None;
        self.lejantı_yenile(cx);
    }

    fn soft_minmax_başlat(&mut self, cx: &mut Context<Self>) {
        let KartKimliği::SoftMinMax(_) = self.aktif_kart else {
            return;
        };
        if self.soft_minmax_çalışıyor {
            return;
        }
        self.soft_minmax_çalışıyor = true;
        let kart = self.aktif_kart;
        self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
                let devam = bu
                    .update(cx, |bu, cx| {
                        if bu.aktif_kart != kart {
                            return false;
                        }
                        let sonuç = bu.soft_minmax_akışı.as_mut().map_or_else(
                            || {
                                Err(UplotHatası::GeçersizKaynakVeri {
                                    varlık: "SoftMinMaxAkışı",
                                    açıklama: "masaüstü akış durumu bulunamadı".to_string(),
                                })
                            },
                            |akış| {
                                akış.ortak_ilerlet();
                                SoftMinMaxÖrneği::TÜMÜ
                                    .into_iter()
                                    .filter(|örnek| örnek.canlı_mı())
                                    .map(|örnek| Ok((örnek, akış.veri(örnek)?)))
                                    .collect::<Result<Vec<_>, UplotHatası>>()
                            },
                        );
                        match sonuç {
                            Ok(veriler) => {
                                for (örnek, veri) in veriler {
                                    let Some((_, grafik)) = bu
                                        .soft_minmax_grafikleri
                                        .iter()
                                        .find(|(kimlik, _)| *kimlik == örnek)
                                    else {
                                        return false;
                                    };
                                    if let Err(hata) = grafik
                                        .update(cx, |grafik, cx| grafik.veriyi_ayarla(veri, cx))
                                    {
                                        bu.hata =
                                            Some(format!("Soft Min/Max güncellenemedi: {hata}"));
                                        return false;
                                    }
                                }
                                true
                            }
                            Err(hata) => {
                                bu.hata = Some(format!("Soft Min/Max verisi üretilemedi: {hata}"));
                                false
                            }
                        }
                    })
                    .unwrap_or(false);
                if !devam {
                    break;
                }
            }
        }));
        cx.notify();
    }

    fn arcsinh_kuvvetini_ayarla(&mut self, kuvvet: i32, cx: &mut Context<Self>) {
        let kuvvet = kuvvet.clamp(-3, 3);
        let eşik = 10_f64.powi(kuvvet);
        let Some(grafik) = &self.grafik else {
            return;
        };
        grafik.update(cx, |grafik, cx| {
            grafik.y_arcsinh_eşiği_ayarla("y", eşik, cx);
        });
        self.arcsinh_kuvvet = kuvvet;
        cx.notify();
    }

    fn autosize_kuvvetini_ayarla(&mut self, kuvvet: i32, cx: &mut Context<Self>) {
        let kuvvet = kuvvet.clamp(0, 9);
        let çarpan = 10_f64.powi(kuvvet);
        let sonuç = self
            .axis_autosize_akışı
            .get_or_insert_with(AxisAutosizeAkışı::yeni)
            .çarpanı_ayarla(çarpan)
            .and_then(|(çarpan, veri)| {
                let grafik =
                    self.grafik
                        .as_ref()
                        .ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
                            varlık: "AxisAutosizeAkışı",
                            açıklama: "masaüstü grafik yüzeyi bulunamadı".to_string(),
                        })?;
                grafik.update(cx, |grafik, cx| {
                    grafik.canlı_veriyi_x_etiket_çarpanında_ayarla(veri, çarpan, cx)
                })
            });
        match sonuç {
            Ok(()) => {
                self.autosize_kuvvet = kuvvet;
                self.hata = None;
            }
            Err(hata) => {
                self.hata = Some(format!("Axis AutoSize güncellenemedi: {hata}"));
            }
        }
        cx.notify();
    }

    fn latency_histogramını_ayarla(&mut self, kova: u8, ofset: u8, cx: &mut Context<Self>) {
        self.latency_kova = kova.clamp(1, 25);
        self.latency_ofset = ofset.min(25);
        if self.aktif_kart != KartKimliği::LatencyHeatmap {
            return;
        }
        let Some((_, collapsed)) = self
            .latency_heatmap_grafikleri
            .iter()
            .find(|(örnek, _)| *örnek == LatencyHeatmapÖrneği::HistogramBirleşik)
            .cloned()
        else {
            self.hata = Some("Collapsed histogram yüzeyi bulunamadı".to_string());
            cx.notify();
            return;
        };
        match latency_heatmap_kartı(
            LatencyHeatmapÖrneği::HistogramBirleşik,
            f64::from(self.latency_kova),
            f64::from(self.latency_ofset),
        ) {
            Ok((_, veri)) => {
                match collapsed.update(cx, |grafik, cx| grafik.veriyi_ayarla(veri, cx)) {
                    Ok(()) => self.hata = None,
                    Err(hata) => {
                        self.hata = Some(format!("Histogram setData uygulanamadı: {hata}"))
                    }
                }
            }
            Err(hata) => self.hata = Some(format!("Histogram kovalanamadı: {hata}")),
        }
        cx.notify();
    }

    fn dinamik_seri_ekle(&mut self, cx: &mut Context<Self>) {
        let Some(grafik) = &self.grafik else {
            self.hata = Some("Dinamik seri eklemek için grafik bulunamadı".to_string());
            cx.notify();
            return;
        };
        let değerler = add_del_series_ek_verisi(self.dinamik_seri_sayacı);
        let seçenek = add_del_series_ek_seçeneği(self.dinamik_seri_sayacı);
        let sonuç = grafik.update(cx, |grafik, cx| grafik.seri_ekle(1, seçenek, değerler, cx));
        match sonuç {
            Ok(()) => {
                self.dinamik_seri_sayacı = self.dinamik_seri_sayacı.wrapping_add(1);
                self.hata = None;
            }
            Err(hata) => self.hata = Some(format!("Seri eklenemedi: {hata}")),
        }
        cx.notify();
    }

    fn dinamik_seri_sil(&mut self, cx: &mut Context<Self>) {
        let Some(grafik) = &self.grafik else {
            self.hata = Some("Dinamik seri silmek için grafik bulunamadı".to_string());
            cx.notify();
            return;
        };
        match grafik.update(cx, |grafik, cx| grafik.seri_sil(1, cx)) {
            Ok(()) => self.hata = None,
            Err(hata) => self.hata = Some(format!("Seri silinemedi: {hata}")),
        }
        cx.notify();
    }

    fn stacked_seriyi_değiştir(
        &mut self,
        örnek: StackedSeriesÖrneği,
        seri_indeksi: usize,
        cx: &mut Context<Self>,
    ) {
        if !matches!(self.aktif_kart, KartKimliği::StackedSeries(_)) {
            return;
        }
        let Some((_, yüzey)) = self
            .stacked_series_grafikleri
            .iter()
            .find(|(kimlik, _)| *kimlik == örnek)
            .cloned()
        else {
            return;
        };
        let görünürlük = yüzey
            .read(cx)
            .grafik()
            .seri_seçenekleri()
            .iter()
            .map(|seri| seri.göster)
            .collect::<Vec<_>>();
        if seri_indeksi >= görünürlük.len() {
            return;
        }
        let mut yeni_görünürlük = görünürlük;
        if let Some(hedef) = yeni_görünürlük.get_mut(seri_indeksi) {
            *hedef = !*hedef;
        }
        let Some(&seri_görünür) = yeni_görünürlük.get(seri_indeksi) else {
            return;
        };
        let sonuç = if örnek.yeniden_yığılan_mı() {
            stacked_series_kartı_görünür(örnek, &yeni_görünürlük).and_then(|(seçenekler, veri)| {
                yüzey.update(cx, |grafik, cx| {
                    for (indeks, seri) in seçenekler.seriler.iter().enumerate() {
                        grafik.seri_görünürlüğünü_ayarla(indeks, seri.göster, cx)?;
                    }
                    grafik.bantları_ayarla(seçenekler.bantlar, cx);
                    if let Some(y_aralığı) = seçenekler.y_aralığı {
                        grafik.veriyi_y_aralığında_ayarla(veri, y_aralığı, cx)
                    } else {
                        grafik.veriyi_ayarla(veri, cx)
                    }
                })
            })
        } else {
            stacked_series_kartı_görünür(örnek, &yeni_görünürlük).and_then(|(seçenekler, _)| {
                yüzey.update(cx, |grafik, cx| {
                    grafik.seri_görünürlüğünü_ayarla(seri_indeksi, seri_görünür, cx)?;
                    if let Some(y_aralığı) = seçenekler.y_aralığı {
                        grafik.canlı_y_aralığını_ayarla(y_aralığı, cx);
                    }
                    Ok(())
                })
            })
        };
        match sonuç {
            Ok(()) => self.hata = None,
            Err(hata) => {
                self.hata = Some(format!("Seri görünürlüğü değiştirilemedi: {hata}"));
            }
        }
        cx.notify();
    }

    fn timeline_serisini_değiştir(
        &mut self,
        örnek: TimelineDiscreteÖrneği,
        seri_indeksi: usize,
        cx: &mut Context<Self>,
    ) {
        if !matches!(self.aktif_kart, KartKimliği::TimelineDiscrete(_)) {
            return;
        }
        let Some((_, yüzey)) = self
            .timeline_discrete_grafikleri
            .iter()
            .find(|(kimlik, _)| *kimlik == örnek)
            .cloned()
        else {
            return;
        };
        let görünür = yüzey.read(cx).grafik().seri_görünür_mü(seri_indeksi);
        match yüzey.update(cx, |grafik, cx| {
            grafik.seri_görünürlüğünü_ayarla(seri_indeksi, !görünür, cx)
        }) {
            Ok(_) => self.hata = None,
            Err(hata) => {
                self.hata = Some(format!("Timeline seri görünürlüğü değiştirilemedi: {hata}"));
            }
        }
        cx.notify();
    }
}

fn grafik_oluştur(
    kart: KartKimliği,
    no_data_örneği: NoDataÖrneği,
    nokta_sayısı: usize,
    autosize_kuvvet: i32,
    latency_kova: u8,
    latency_ofset: u8,
    pixel_align_adımı: usize,
) -> Result<Grafik, UplotHatası> {
    let girdi = KatalogFabrikaGirdisi {
        kart,
        no_data_örneği,
        nokta_sayısı,
        autosize_kuvvet,
        latency_kova,
        latency_ofset,
        pixel_align_adımı,
    };
    let (seçenekler, veri) = kart.tanımlayıcı().grafiği_oluştur(girdi)?;
    Grafik::yeni(seçenekler, veri)
}

impl Render for ChartListesi {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _kare_ölçümü = izleme::Ölçüm::başlat(izleme::Yuva::KökRender);
        let görüntü_alanı = window.viewport_size();
        izleme::pencere_boyutu(
            f32::from(görüntü_alanı.width),
            f32::from(görüntü_alanı.height),
        );
        if let Some(indeks) = self.kart_listesi_kaydırma_bekliyor.take() {
            cx.on_next_frame(window, move |bu, _, cx| {
                bu.kart_listesi_kaydırma
                    .scroll_to_item(indeks, ScrollStrategy::Nearest);
                cx.notify();
            });
        }
        if self.açıklama_istendi && self.açıklama_odak_bekliyor {
            self.açıklama_odak_bekliyor = false;
            self.açıklama_metni
                .read(cx)
                .focus_handle(cx)
                .focus(window, cx);
        }
        if self.aktif_kart == KartKimliği::SineStream && !self.sine_kare_bekleniyor {
            self.sine_kare_bekleniyor = true;
            cx.on_next_frame(window, |bu, _window, cx| {
                bu.sine_kare_bekleniyor = false;
                if bu.aktif_kart != KartKimliği::SineStream {
                    return;
                }
                let sonuç = bu.sine_akışı.as_mut().map_or_else(
                    || {
                        Err(UplotHatası::GeçersizKaynakVeri {
                            varlık: "SineAkışı",
                            açıklama: "masaüstü akış durumu bulunamadı".to_string(),
                        })
                    },
                    SineAkışı::ilerlet,
                );
                match sonuç {
                    Ok(veri) => {
                        if let Some(grafik) = &bu.grafik
                            && let Err(hata) =
                                grafik.update(cx, |grafik, cx| grafik.veriyi_ayarla(veri, cx))
                        {
                            bu.hata = Some(format!("Sine Stream güncellenemedi: {hata}"));
                        }
                    }
                    Err(hata) => {
                        bu.hata = Some(format!("Sine Stream verisi üretilemedi: {hata}"));
                    }
                }
                cx.notify();
            });
        }
        if self.kare_ölçer.çalışıyor && !self.performans_kare_bekleniyor {
            self.performans_kare_bekleniyor = true;
            cx.on_next_frame(window, |bu, _window, cx| {
                bu.performans_kare_bekleniyor = false;
                bu.kare_ölçer.kareyi_kaydet(Instant::now());
                cx.notify();
            });
        }
        let panel = rgb(0xffffff);
        let zemin = rgb(0xf3f4f6);
        let metin = rgb(0x111827);
        let soluk = rgb(0x6b7280);
        let vurgu = rgb(0xdc2626);
        let aktif_kart = self.aktif_kart;
        let aktif_kart_tanımı = aktif_kart.tanımlayıcı();
        let kare_ölçüm_yazısı = if self.kare_ölçer.çalışıyor {
            format!(
                "Kare ölçümü: {}/{}",
                self.kare_ölçer.ilerleme(),
                PERFORMANS_KARE_SAYISI
            )
        } else if let Some(sonuç) = self.kare_ölçer.sonuç {
            format!(
                "Kare ms p50 {:.2} · p95 {:.2} · p99 {:.2} · max {:.2} · {}",
                sonuç.p50_ms,
                sonuç.p95_ms,
                sonuç.p99_ms,
                sonuç.azami_ms,
                if sonuç.p95_ms < KARE_P95_BÜTÇESİ_MS {
                    "bütçe geçti"
                } else {
                    "bütçe aşıldı"
                }
            )
        } else {
            "Kare ölçümü: istek bekliyor".to_string()
        };
        let gpu = window.gpu_specs();
        let yazılım_gpu = gpu
            .as_ref()
            .is_some_and(|özellikler| özellikler.is_software_emulated);
        let gpu_yazısı = gpu.map_or_else(
            || "GPU: platform bilgisi sunulmadı".to_string(),
            |özellikler| {
                // Upstream `GpuSpecs` backend adını taşımaz; sürücü bilgisi
                // pratikte aynı ayrımı (Vulkan/GL, Mesa sürümü) veriyor.
                format!(
                    "GPU: {} · {} · {}{}",
                    özellikler.device_name,
                    özellikler.driver_name,
                    özellikler.driver_info,
                    if özellikler.is_software_emulated {
                        " · YAZILIM FALLBACK"
                    } else {
                        ""
                    }
                )
            },
        );
        izleme::gpu_bilgisi(&gpu_yazısı);
        let soft_minmax_canlı = matches!(aktif_kart, KartKimliği::SoftMinMax(_));
        let soft_minmax_çalışıyor = self.soft_minmax_çalışıyor;
        let sync_cursor_etkin = self.sync_cursor_grubu.senkron();
        let sync_cursor_fare_etkin = self.sync_cursor_grubu.fare_basma_bırakma_senkron();
        let mevcut_seri_sayısı = self.grafik.as_ref().map_or(0, |grafik| {
            grafik.read(cx).grafik().seri_seçenekleri().len()
        });
        let nokta_yazısı = SharedString::from(match aktif_kart {
            KartKimliği::AddDelSeries => {
                format!("30 nokta × {mevcut_seri_sayısı} dinamik seri")
            }
            KartKimliği::AlignDataCost => {
                format!(
                    "2 bağımsız yüzey · 6 warmup + 1 join · {:.2} ms oturum",
                    self.align_data_kurulum_ms.unwrap_or_default()
                )
            }
            KartKimliği::Resize => format!("{} nokta", self.nokta_sayısı),
            KartKimliği::Annotations => "30 nokta × 2 seri · 2 X annotation".to_string(),
            KartKimliği::AreaFill => "30 sabit nokta × 3 seri".to_string(),
            KartKimliği::ScalePadding => "10 nokta × 13 düz seri".to_string(),
            KartKimliği::Months => {
                "2 kaynak yüzeyi · 72 aylık nokta · UTC ve artık yıl".to_string()
            }
            KartKimliği::MonthsRussian => {
                "tek 1920×600 yüzey · 36 UTC ayı · Rusça yerel adlar".to_string()
            }
            KartKimliği::NiceScale => {
                "6 nokta × 3 seri · boyuta duyarlı güzel Y ölçeği".to_string()
            }
            KartKimliği::NoData => {
                let nokta = self.no_data_örneği.nokta_sayısı();
                format!(
                    "{} · {nokta} nokta × 1 seri · {}",
                    self.no_data_örneği.başlık(),
                    if nokta == 0 {
                        "boş ölçek durumu"
                    } else {
                        "rangeNum kenar durumu"
                    }
                )
            }
            KartKimliği::PathGapClip => {
                "15 ilişkili yüzey · 4 ortak spanGaps animasyonu · 5 karşılaştırma grubu"
                    .to_string()
            }
            KartKimliği::PixelAlign => {
                let örnek = self
                    .pixel_align_akışı
                    .as_ref()
                    .map_or(0, PixelAlignAkışı::örnek_sayısı);
                format!(
                    "2 eşzamanlı yüzey · {örnek} ortak örnek × 3 seri · 60 FPS kayan 120 sn pencere"
                )
            }
            KartKimliği::Points => {
                "4 eşzamanlı yüzey · 2 yüzey ortak 180 nokta · piksel-gap filtresi".to_string()
            }
            KartKimliği::ScalesDirOri => {
                "16 eşzamanlı yüzey · aynı 10 nokta × 2 seri · direction/orientation matrisi"
                    .to_string()
            }
            KartKimliği::Scatter => {
                "2 bağımsız yüzey · 40.000 sabit nokta + 200 alan ölçekli balon".to_string()
            }
            KartKimliği::ScrollSync => "30 nokta × 3 seri · kaydırmada syncRect".to_string(),
            KartKimliği::SineStream => "600 nokta × 6 seri · 60 FPS setData".to_string(),
            KartKimliği::SoftMinMax(_) => {
                "5 ilişkili yüzey · 4 ortak canlı kip + düz sıfır".to_string()
            }
            KartKimliği::SparklinesBars(_) => {
                "2 ilişkili yüzey · aynı 16 nokta · dinamik gradyan / açık renk".to_string()
            }
            KartKimliği::Sparklines(_) => {
                "10 hisse × 2 ölçüm · 20 eşzamanlı 150×30 yüzey · 440 nokta".to_string()
            }
            KartKimliği::Sparse(_) => "13.608 X · 4.608 dolu Y · 622 dolu parça".to_string(),
            KartKimliği::StackedSeries(_) => {
                "16 bağımsız yüzey · 2×800×400 + 2×1600×400 + 12×400×300".to_string()
            }
            KartKimliği::StreamData(_) => {
                let durum = |örnek| {
                    self.stream_data_grubu
                        .as_ref()
                        .and_then(|grup| grup.akış(örnek))
                        .map_or((0, 0), |akış| (akış.başlangıç(), akış.uzunluk()))
                };
                let (kayan_başlangıç, kayan_uzunluk) = durum(StreamDataÖrneği::SabitUzunluk);
                let (_, artan_uzunluk) = durum(StreamDataÖrneği::ArtanUzunluk);
                let (_, sabit_x_uzunluk) = durum(StreamDataÖrneği::SabitXArtanUzunluk);
                format!(
                    "3 bağımsız yüzey · kayan {kayan_başlangıç}+{kayan_uzunluk} · \
                     artan {artan_uzunluk} · sabit X {sabit_x_uzunluk} · 100 ms/10 satır setData"
                )
            }
            KartKimliği::GpuiSvgExport => self.svg_kayıt_baytı.map_or_else(
                || "3 nokta × 1 seri · kayıt yalnız düğmeye basıldığında çalışır".to_string(),
                |bayt| {
                    format!(
                        "800×400 gerçek vektör SVG · {bayt} bayt {}",
                        if cfg!(target_family = "wasm") {
                            "indirildi"
                        } else {
                            "panoya kopyalandı"
                        }
                    )
                },
            ),
            KartKimliği::SyncCursor => {
                "5 yüzey · 3.004 nokta · iki cursor eşleme grubu".to_string()
            }
            KartKimliği::CursorBind => "30 nokta × 3 seri · Ctrl açıklama bağı".to_string(),
            KartKimliği::CursorSnap => "30 nokta × 3 seri".to_string(),
            KartKimliği::CursorTooltip => "7 nokta × 1 seri · canlı bilgi kutusu".to_string(),
            KartKimliği::CustomScales => {
                "3 bağımsız 800×800 yüzey · aynı 199×3 veri + 20 draw noktası".to_string()
            }
            KartKimliği::DataSmoothing => {
                let süre = |örnek| {
                    self.data_smoothing_ölçümleri_ms
                        .iter()
                        .find_map(|(kimlik, ms)| (*kimlik == örnek).then_some(*ms))
                        .unwrap_or(0.0)
                };
                format!(
                    "4 bağımsız 1920×300 yüzey · raw 3600 · SGG 3600/{:.2} ms · \
                     ASAP 137/{:.2} ms · Moving Avg 3600/{:.2} ms",
                    süre(SmoothingÖrneği::SavitzkyGolay),
                    süre(SmoothingÖrneği::Asap),
                    süre(SmoothingÖrneği::HareketliOrtalama),
                )
            }
            KartKimliği::DrawHooks => {
                "9 nokta × 3 seri · gradyan + medyan + yıldız + istatistik".to_string()
            }
            KartKimliği::FocusCursor => {
                "4 bağımsız yüzey · ilk iki yüzeyde ortak 130K aligned veri · retained odak stili"
                    .to_string()
            }
            KartKimliği::Gradients => {
                "5 bağımsız yüzey · ortak data2/data4 · stroke/fill/cursor gradyanları".to_string()
            }
            KartKimliği::GridOverSeries => {
                "30 nokta × 3 dolgulu seri · ızgara üst katmanda".to_string()
            }
            KartKimliği::HighLowBands => {
                "12 bağımsız yüzey · 3 ortak immutable veri çifti · line/step/spline/bar"
                    .to_string()
            }
            KartKimliği::LatencyHeatmap => {
                "5 bağımsız yüzey · ortak ham veri · raw/aggregate/mode2 + iki histogram"
                    .to_string()
            }
            KartKimliği::LinePaths => {
                "8 yüzey · ortak 101 nokta · 4 null boşluğu · senkron cursor".to_string()
            }
            KartKimliği::LogScales => {
                "2 bağımsız yüzey · ortak Arc veri · 1.440 zaman × 12 sunucu".to_string()
            }
            KartKimliği::LogScales2 => {
                "12 yüzey · ilk 3 ortak veri · In/Out senkron çift · log10/log2".to_string()
            }
            KartKimliği::MassSpectrum => {
                "41.986 kaynak CSV noktası · m/z / relative abundance (%)".to_string()
            }
            KartKimliği::MeasureDatums => "5 nokta · 1/2 datum · Esc temizle".to_string(),
            KartKimliği::MultiBars(örnek) => format!("multi-bars · {}", örnek.başlık()),
            KartKimliği::NearestNonNull => {
                "5 bağımsız yüzey · null / proximity / dataIdx / cursor.move".to_string()
            }
            KartKimliği::MissingData => {
                "2 bağımsız yüzey · 200×3 null telemetri + 8×1 komşu X boşluğu".to_string()
            }
            KartKimliği::DependentScale => "7 nokta × °F veri · türetilmiş °C ekseni".to_string(),
            KartKimliği::ArcSinhScales => "111 nokta · −1000…1000 ArcSinh".to_string(),
            KartKimliği::AxisControl => {
                "500.001 nokta · min/max piksel seyrekleştirme".to_string()
            }
            KartKimliği::AxisAutosize => {
                format!("501 nokta · çarpan 10^{}", self.autosize_kuvvet)
            }
            KartKimliği::AxisIndicators => "30 nokta × 3 bağımsız Y ekseni".to_string(),
            KartKimliği::Bars(_) => {
                "10 bağımsız kaynak yüzeyi · grouped/stacked · setSeries".to_string()
            }
            KartKimliği::BarsValuesAutosize(_) => {
                "2 bağımsız yüzey × 12 kanıt değeri · 10…25 px etiket".to_string()
            }
            KartKimliği::BoxWhisker(_) => {
                "17 bağımsız yüzey × ilk 30 keyed framework · stats.js + rangeNum".to_string()
            }
            KartKimliği::Candlestick => "218 gün · Gold OHLC + kanıt hacmi".to_string(),
            KartKimliği::SyncYZero(aşama) => {
                format!("3 nokta × 3 Y ölçeği · {}", aşama.açıklama())
            }
            KartKimliği::ThinBars(_) => {
                "55 bağımsız yüzey · 1.422 çubuk · 270 otomatik nokta".to_string()
            }
            KartKimliği::TimePeriods(_) => {
                "3 bağımsız 1920×200 yüzey · tek traffic.json kaynağı".to_string()
            }
            KartKimliği::TimelineDiscrete(_) => "4 bağımsız 1920×300 yüzey · 3 şerit".to_string(),
            KartKimliği::TimeseriesDiscrete => {
                "50 ortak zaman noktası · 1 float + 3 ayrık seri".to_string()
            }
            KartKimliği::TimezonesDst => "11 bölüm · 51 yüzey · ilk dört üçlü senkron".to_string(),
            KartKimliği::TooltipsClosest => {
                "234 commit × 4 Opt serisi · 100 interpolasyon işareti".to_string()
            }
            KartKimliği::Tooltips => {
                "7 nokta × 2 seri · Two gizli · 2 sn yeniden kurulum".to_string()
            }
            KartKimliği::Trendlines => {
                "100 nokta × 2 seri · görünür uçlar arasında 5/5 kesik trend".to_string()
            }
            KartKimliği::UpdateCursorSelectResize => {
                let boyut = self
                    .boyut_senkron_akışı
                    .map_or(800, BoyutSenkronAkışı::boyut);
                format!("{boyut}×{boyut} px · imleç/seçim/hover oranları korunuyor")
            }
            KartKimliği::WindDirection => {
                "143 saat × sıcaklık, hız ve yön · 139 yön vektörü".to_string()
            }
            KartKimliği::YScaleDrag => {
                "5 nokta × 2 bağımsız Y ölçeği · eksenleri sürükleyin".to_string()
            }
            KartKimliği::YShiftedSeries => {
                "30 nokta × 3 seri · 2 sn normal / +0,+10,+20 kaydırılmış kip".to_string()
            }
        });
        let kart_tanımı_açık = self.kart_tanımı_açık;
        let kart_tanımı_etiketi = SharedString::from(format!(
            "{} Kart tanımı · {}",
            if kart_tanımı_açık { "−" } else { "+" },
            aktif_kart_tanımı.tanım_yolu
        ));
        let tekerlek_anahtarı = self.tekerlek_anahtarı.clone();
        let tekerlek_odaksız_anahtarı = self.tekerlek_odaksız_anahtarı.clone();
        let içi_boş_nokta_anahtarı = self.içi_boş_nokta_anahtarı.clone();
        let dolu_nokta_anahtarı = self.dolu_nokta_anahtarı.clone();
        let (mut geri_var, mut yakınlaştırılmış, etkileşimler, bileşen_hatası) =
            self.grafik.as_ref().map_or_else(
                || (false, false, aktif_kart.etkileşimler(), None),
                |grafik| {
                    let grafik = grafik.read(cx);
                    (
                        grafik.grafik().geri_var(),
                        grafik.grafik().yakınlaştırılmış(),
                        grafik.grafik().etkileşim_seçenekleri(),
                        grafik.hata().map(str::to_string),
                    )
                },
            );
        let (grup_geri_var, grup_yakınlaştırılmış, yüzey_sayısı) = self.etkin_görünüm_durumu(cx);
        if yüzey_sayısı > 1 {
            geri_var = grup_geri_var;
            yakınlaştırılmış = grup_yakınlaştırılmış;
        }
        let çizim_hatası = self.hata.clone().or(bileşen_hatası);
        let lejant = self.lejant.clone();
        let lejant_konumu = self.lejant_konumu(cx);
        // Çubuk aileleri kendi yüzey başına düğmelerini kuruyor, mum kartında
        // kaynak setSeries sunmuyor; kalan kartlarda seri görünürlüğü tek
        // yoldan, lejant girdisine tıklanarak değişir.
        let lejant_görünür = !matches!(
            aktif_kart,
            KartKimliği::Bars(_)
                | KartKimliği::BarsValuesAutosize(_)
                | KartKimliği::BoxWhisker(_)
                | KartKimliği::Candlestick
        );

        let liste = div()
            .id("kart-listesi")
            .w(px(280.0))
            .h_full()
            .min_h_0()
            .flex_none()
            .flex()
            .flex_col()
            .p_4()
            .bg(panel)
            .border_r_1()
            .border_color(rgb(0xe5e7eb))
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(metin)
                    .child("uPlot.rs Grafik Kataloğu"),
            )
            .child(
                div()
                    .mt_1()
                    .mb_4()
                    .text_sm()
                    .text_color(rgb(LİSTE_İKİNCİL_RENGİ))
                    .child("Canlı masaüstü doğrulaması"),
            )
            .child(
                uniform_list(
                    "kart-listesi-ogeleri",
                    KATALOG_KARTLARI.len(),
                    cx.processor(move |bu, aralık: Range<usize>, _pencere, cx| {
                        aralık
                            .filter_map(|indeks| {
                                KATALOG_KARTLARI
                                    .get(indeks)
                                    .copied()
                                    .map(|tanım| (indeks, tanım))
                            })
                            .map(|(indeks, tanım)| {
                                let kart = tanım.kimlik;
                                let aktif = bu.aktif_kart.ana_kart() == tanım.kimlik;
                                let erişilebilir_tetikleyici = cx.weak_entity();
                                let grup_etiketi = match (tanım.grup, tanım.varyant_grubu) {
                                    (KatalogKartGrubu::Tek, _) => SharedString::from(tanım.slug),
                                    (KatalogKartGrubu::İlişkiliYüzeyler, Some(varyant)) => {
                                        SharedString::from(format!(
                                            "{} · varyant grubu: {varyant}",
                                            tanım.slug
                                        ))
                                    }
                                    (KatalogKartGrubu::İlişkiliYüzeyler, None) => {
                                        SharedString::from(format!(
                                            "{} · ilişkili yüzeyler",
                                            tanım.slug
                                        ))
                                    }
                                };
                                katalog_kartı(
                                    tanım.slug,
                                    tanım.başlık,
                                    grup_etiketi,
                                    aktif,
                                    tanım.kaynak,
                                    panel,
                                    vurgu,
                                )
                                .tab_index(0)
                                .key_context("uplot_katalog_kartı")
                                .role(Role::Button)
                                .aria_label(format!("{}. Kaynak: {}", tanım.başlık, tanım.kaynak))
                                .aria_selected(aktif)
                                .aria_position_in_set(indeks + 1)
                                .aria_size_of_set(KATALOG_KARTLARI.len())
                                .aria_keyshortcuts("Enter Space")
                                .focus_visible(|stil| stil.border_color(vurgu))
                                .on_action(cx.listener(move |bu, _: &KartıEtkinleştir, _, cx| {
                                    bu.kartı_seç(kart, cx);
                                }))
                                .on_a11y_action(AccessibleAction::Click, move |_, _, cx| {
                                    erişilebilir_tetikleyici
                                        .update(cx, |bu, cx| bu.kartı_seç(kart, cx))
                                        .ok();
                                })
                                .on_click(cx.listener(
                                    move |bu, _: &ClickEvent, _, cx| {
                                        bu.kartı_seç(kart, cx);
                                    },
                                ))
                            })
                            .collect()
                    }),
                )
                .track_scroll(&self.kart_listesi_kaydırma)
                .flex_1()
                .min_h_0(),
            );

        let araçlar = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap_2()
            .mb_3()
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(soluk)
                    .child(nokta_yazısı),
            )
            .child(tekerlek_anahtarı)
            .child(tekerlek_odaksız_anahtarı)
            .child(içi_boş_nokta_anahtarı)
            .child(dolu_nokta_anahtarı)
            .when(lejant_görünür, |öğe| {
                öğe.child(
                    Dugme::yeni(
                        "lejant-konumu",
                        SharedString::from(format!(
                            "Lejant · {}",
                            lejant_konumu_başlığı(lejant_konumu)
                        )),
                    )
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(DugmeTuru::Ikincil)
                    .tiklaninca(cx.listener(|bu, _, _, cx| {
                        bu.lejant_konumunu_ilerlet(cx);
                    })),
                )
            })
            .when(matches!(aktif_kart, KartKimliği::MultiBars(_)), |öğe| {
                öğe.children(MultiBarsÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    let seçili = aktif_kart == KartKimliği::MultiBars(örnek);
                    Dugme::yeni(
                        SharedString::from(format!("multi-bars-varyant-{}", örnek.kimlik())),
                        örnek.başlık(),
                    )
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(if seçili {
                        DugmeTuru::Birincil
                    } else {
                        DugmeTuru::Ikincil
                    })
                    .tiklaninca(cx.listener(move |bu, _, _, cx| {
                        bu.kartı_seç(KartKimliği::MultiBars(örnek), cx);
                    }))
                }))
            })
            .child(
                Dugme::yeni(
                    "performans-kare-olc",
                    if self.kare_ölçer.çalışıyor {
                        "Kare ölçülüyor…"
                    } else {
                        "180 kare ölç"
                    },
                )
                .boyutu(DugmeBoyutu::Kucuk)
                .turu(DugmeTuru::Ikincil)
                .devre_disi(self.kare_ölçer.çalışıyor)
                .tiklaninca(cx.listener(|bu, _, _, cx| {
                    bu.kare_ölçümünü_başlat(cx);
                })),
            )
            .when(aktif_kart == KartKimliği::SyncCursor, |öğe| {
                öğe
                    .child(
                        Dugme::yeni(
                            "sync-cursor-toggle",
                            if sync_cursor_etkin {
                                "✓ Cursor sync"
                            } else {
                                "○ Cursor sync"
                            },
                        )
                        .boyutu(DugmeBoyutu::Kucuk)
                        .turu(DugmeTuru::Ikincil)
                        .tiklaninca(cx.listener(|bu, _, _, cx| {
                            bu.sync_cursor_senkronunu_değiştir(cx);
                        })),
                    )
                    .child(
                        Dugme::yeni(
                            "sync-cursor-mouse-toggle",
                            if sync_cursor_fare_etkin {
                                "✓ mousedown/up sync"
                            } else {
                                "○ mousedown/up sync"
                            },
                        )
                        .boyutu(DugmeBoyutu::Kucuk)
                        .turu(DugmeTuru::Ikincil)
                        .tiklaninca(cx.listener(|bu, _, _, cx| {
                            bu.sync_cursor_fare_filtresini_değiştir(cx);
                        })),
                    )
            })
            .when(soft_minmax_canlı, |öğe| {
                öğe.child(
                    Dugme::yeni("soft-minmax-baslat", "→ dataMax++")
                        .boyutu(DugmeBoyutu::Kucuk)
                        .turu(DugmeTuru::Ikincil)
                        .devre_disi(soft_minmax_çalışıyor)
                        .tiklaninca(cx.listener(|bu, _, _, cx| bu.soft_minmax_başlat(cx))),
                )
            })
            .when(aktif_kart == KartKimliği::AddDelSeries, |öğe| {
                öğe
                    .child(
                        Dugme::yeni("dinamik-seri-ekle", "Add Series")
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .tiklaninca(cx.listener(|bu, _, _, cx| bu.dinamik_seri_ekle(cx))),
                    )
                    .child(
                        Dugme::yeni("dinamik-seri-sil", "Del Series")
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .tiklaninca(cx.listener(|bu, _, _, cx| bu.dinamik_seri_sil(cx))),
                    )
            })
            .when(aktif_kart == KartKimliği::GpuiSvgExport, |öğe| {
                öğe.child(
                    Dugme::yeni(
                        "gpui-svg-kaydet",
                        if cfg!(target_family = "wasm") {
                            "SVG’yi indir"
                        } else {
                            "SVG’yi panoya kopyala"
                        },
                    )
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(DugmeTuru::Ikincil)
                    .tiklaninca(cx.listener(|bu, _, _, cx| {
                        bu.svg_kaydını_dışa_aktar(cx);
                    })),
                )
            })
            .when(aktif_kart == KartKimliği::ArcSinhScales, |öğe| {
                öğe
                    .child(
                        Dugme::yeni("arcsinh-azalt", "− Eşik")
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .devre_disi(self.arcsinh_kuvvet <= -3)
                            .tiklaninca(cx.listener(|bu, _, _, cx| {
                                bu.arcsinh_kuvvetini_ayarla(bu.arcsinh_kuvvet - 1, cx);
                            })),
                    )
                    .child(div().text_xs().text_color(soluk).child(format!(
                        "Doğrusal eşik: {}",
                        10_f64.powi(self.arcsinh_kuvvet)
                    )))
                    .child(
                        Dugme::yeni("arcsinh-artir", "+ Eşik")
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .devre_disi(self.arcsinh_kuvvet >= 3)
                            .tiklaninca(cx.listener(|bu, _, _, cx| {
                                bu.arcsinh_kuvvetini_ayarla(bu.arcsinh_kuvvet + 1, cx);
                            })),
                    )
            })
            .when(aktif_kart == KartKimliği::AxisAutosize, |öğe| {
                öğe
                    .child(
                        Dugme::yeni("autosize-azalt", "− 10×")
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .devre_disi(self.autosize_kuvvet <= 0)
                            .tiklaninca(cx.listener(|bu, _, _, cx| {
                                bu.autosize_kuvvetini_ayarla(bu.autosize_kuvvet - 1, cx);
                            })),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(soluk)
                            .child(format!("Çarpan: {}", 10_f64.powi(self.autosize_kuvvet))),
                    )
                    .child(
                        Dugme::yeni("autosize-artir", "+ 10×")
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .devre_disi(self.autosize_kuvvet >= 9)
                            .tiklaninca(cx.listener(|bu, _, _, cx| {
                                bu.autosize_kuvvetini_ayarla(bu.autosize_kuvvet + 1, cx);
                            })),
                    )
            })
            .when(aktif_kart == KartKimliği::LatencyHeatmap, |öğe| {
                öğe
                    .child(
                        Dugme::yeni("latency-kova-azalt", "− Kova")
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .devre_disi(self.latency_kova <= 1)
                            .tiklaninca(cx.listener(|bu, _, _, cx| {
                                bu.latency_histogramını_ayarla(
                                    bu.latency_kova.saturating_sub(1),
                                    bu.latency_ofset,
                                    cx,
                                );
                            })),
                    )
                    .child(
                        Dugme::yeni("latency-kova-artir", "+ Kova")
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .devre_disi(self.latency_kova >= 25)
                            .tiklaninca(cx.listener(|bu, _, _, cx| {
                                bu.latency_histogramını_ayarla(
                                    bu.latency_kova.saturating_add(1),
                                    bu.latency_ofset,
                                    cx,
                                );
                            })),
                    )
                    .child(
                        Dugme::yeni("latency-ofset-azalt", "− Ofset")
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .devre_disi(self.latency_ofset == 0)
                            .tiklaninca(cx.listener(|bu, _, _, cx| {
                                bu.latency_histogramını_ayarla(
                                    bu.latency_kova,
                                    bu.latency_ofset.saturating_sub(1),
                                    cx,
                                );
                            })),
                    )
                    .child(div().text_xs().text_color(soluk).child(format!(
                        "{} ms · ofset {}",
                        self.latency_kova, self.latency_ofset
                    )))
                    .child(
                        Dugme::yeni("latency-ofset-artir", "+ Ofset")
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .devre_disi(self.latency_ofset >= 25)
                            .tiklaninca(cx.listener(|bu, _, _, cx| {
                                bu.latency_histogramını_ayarla(
                                    bu.latency_kova,
                                    bu.latency_ofset.saturating_add(1),
                                    cx,
                                );
                            })),
                    )
            })
            .child(
                Dugme::yeni("nokta-azalt", "− Nokta")
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(DugmeTuru::Ikincil)
                    .devre_disi(aktif_kart != KartKimliği::Resize)
                    .tiklaninca(cx.listener(|bu, _, _, cx| {
                        bu.grafiği_yenile(bu.nokta_sayısı.saturating_sub(10).max(10), cx);
                    })),
            )
            .child(
                Dugme::yeni("nokta-artir", "+ Nokta")
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(DugmeTuru::Ikincil)
                    .devre_disi(aktif_kart != KartKimliği::Resize)
                    .tiklaninca(cx.listener(|bu, _, _, cx| {
                        bu.grafiği_yenile(bu.nokta_sayısı.saturating_add(10).min(10_000), cx);
                    })),
            )
            .child(
                Dugme::yeni("gorunum-geri", "↶ Geri")
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(DugmeTuru::Hayalet)
                    .devre_disi(!geri_var || !etkileşimler.görünüm_geçmişi)
                    .tiklaninca(cx.listener(|bu, _, _, cx| {
                        if bu.aktif_kart == KartKimliği::AlignDataCost {
                            for (_, grafik) in &bu.align_data_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::CustomScales {
                            for (_, grafik) in &bu.custom_scales_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::DataSmoothing {
                            for (_, grafik) in &bu.data_smoothing_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::FocusCursor {
                            for (_, grafik) in &bu.focus_cursor_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::Gradients {
                            for (_, grafik) in &bu.gradients_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::HighLowBands {
                            for (_, grafik) in &bu.high_low_bands_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::LatencyHeatmap {
                            for (_, grafik) in &bu.latency_heatmap_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::LinePaths {
                            for (_, grafik) in &bu.line_paths_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::LogScales {
                            for (_, grafik) in &bu.log_scales_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::LogScales2 {
                            for (_, grafik) in &bu.log_scales2_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::SyncCursor {
                            for (_, grafik) in &bu.sync_cursor_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::TimezonesDst {
                            for (_, grafik) in &bu.timezones_dst_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::NearestNonNull {
                            for (_, grafik) in &bu.nearest_non_null_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::MissingData {
                            for (_, grafik) in &bu.missing_data_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::Months {
                            for grafik in &bu.months_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::PathGapClip {
                            for (_, grafik) in &bu.path_gap_clip_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::PixelAlign {
                            for (_, grafik) in &bu.pixel_align_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::Points {
                            for (_, grafik) in &bu.points_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::ScalesDirOri {
                            bu.scales_dir_ori_senkronlanıyor = true;
                            for (_, grafik) in &bu.scales_dir_ori_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
                            }
                            bu.scales_dir_ori_senkronlanıyor = false;
                        } else if bu.aktif_kart == KartKimliği::Scatter {
                            for (_, grafik) in &bu.scatter_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::Bars(_)) {
                            for (_, grafik) in &bu.bars_grouped_stacked_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::BarsValuesAutosize(_)) {
                            for (_, grafik) in &bu.bars_values_autosize_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::BoxWhisker(_)) {
                            for (_, grafik) in &bu.box_whisker_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::SoftMinMax(_)) {
                            for (_, grafik) in &bu.soft_minmax_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::SparklinesBars(_)) {
                            for (_, grafik) in &bu.sparklines_bars_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::Sparklines(_)) {
                            for (_, grafik) in &bu.sparklines_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::Sparse(_)) {
                            for (_, grafik) in &bu.sparse_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::StackedSeries(_)) {
                            for (_, grafik) in &bu.stacked_series_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::StreamData(_)) {
                            for (_, grafik) in &bu.stream_data_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::ThinBars(_)) {
                            for (_, grafik) in &bu.thin_bars_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::TimePeriods(_)) {
                            for (_, grafik) in &bu.time_periods_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::TimelineDiscrete(_)) {
                            for (_, grafik) in &bu.timeline_discrete_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.önceki_görünüm(cx));
                            }
                        } else if let Some(grafik) = &bu.grafik {
                            grafik.update(cx, |grafik, cx| {
                                grafik.önceki_görünüm(cx);
                            });
                        }
                        cx.notify();
                    })),
            )
            .child(
                Dugme::yeni("tam-gorunum", "Tam görünüm")
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(DugmeTuru::Hayalet)
                    .devre_disi(!yakınlaştırılmış || !etkileşimler.çift_tıkla_tam_görünüm)
                    .tiklaninca(cx.listener(|bu, _, _, cx| {
                        if bu.aktif_kart == KartKimliği::AlignDataCost {
                            for (_, grafik) in &bu.align_data_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::CustomScales {
                            for (_, grafik) in &bu.custom_scales_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::DataSmoothing {
                            for (_, grafik) in &bu.data_smoothing_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::FocusCursor {
                            for (_, grafik) in &bu.focus_cursor_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::Gradients {
                            for (_, grafik) in &bu.gradients_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::HighLowBands {
                            for (_, grafik) in &bu.high_low_bands_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::LatencyHeatmap {
                            for (_, grafik) in &bu.latency_heatmap_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::LinePaths {
                            for (_, grafik) in &bu.line_paths_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::LogScales {
                            for (_, grafik) in &bu.log_scales_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::LogScales2 {
                            for (_, grafik) in &bu.log_scales2_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::SyncCursor {
                            for (_, grafik) in &bu.sync_cursor_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::TimezonesDst {
                            for (_, grafik) in &bu.timezones_dst_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::NearestNonNull {
                            for (_, grafik) in &bu.nearest_non_null_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::MissingData {
                            for (_, grafik) in &bu.missing_data_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if bu.aktif_kart == KartKimliği::Months {
                            for grafik in &bu.months_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::PathGapClip {
                            for (_, grafik) in &bu.path_gap_clip_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::PixelAlign {
                            for (_, grafik) in &bu.pixel_align_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::Points {
                            for (_, grafik) in &bu.points_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
                            }
                        } else if bu.aktif_kart == KartKimliği::ScalesDirOri {
                            bu.scales_dir_ori_senkronlanıyor = true;
                            for (_, grafik) in &bu.scales_dir_ori_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
                            }
                            bu.scales_dir_ori_senkronlanıyor = false;
                        } else if bu.aktif_kart == KartKimliği::Scatter {
                            for (_, grafik) in &bu.scatter_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::Bars(_)) {
                            for (_, grafik) in &bu.bars_grouped_stacked_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::BarsValuesAutosize(_)) {
                            for (_, grafik) in &bu.bars_values_autosize_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::BoxWhisker(_)) {
                            for (_, grafik) in &bu.box_whisker_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::SoftMinMax(_)) {
                            for (_, grafik) in &bu.soft_minmax_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::SparklinesBars(_)) {
                            for (_, grafik) in &bu.sparklines_bars_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::Sparklines(_)) {
                            for (_, grafik) in &bu.sparklines_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::Sparse(_)) {
                            for (_, grafik) in &bu.sparse_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::StackedSeries(_)) {
                            for (_, grafik) in &bu.stacked_series_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::StreamData(_)) {
                            for (_, grafik) in &bu.stream_data_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::ThinBars(_)) {
                            for (_, grafik) in &bu.thin_bars_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::TimePeriods(_)) {
                            for (_, grafik) in &bu.time_periods_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if matches!(bu.aktif_kart, KartKimliği::TimelineDiscrete(_)) {
                            for (_, grafik) in &bu.timeline_discrete_grafikleri {
                                grafik.update(cx, |grafik, cx| grafik.tam_görünüm(cx));
                            }
                        } else if let Some(grafik) = &bu.grafik {
                            grafik.update(cx, |grafik, cx| {
                                grafik.tam_görünüm(cx);
                            });
                        }
                        cx.notify();
                    })),
            )
            .child(
                Dugme::yeni("grafik-sifirla", "Sıfırla")
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(DugmeTuru::Hayalet)
                    .tiklaninca(cx.listener(|bu, _, _, cx| {
                        if bu.aktif_kart == KartKimliği::AlignDataCost {
                            bu.align_data_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::CustomScales {
                            bu.custom_scales_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::DataSmoothing {
                            bu.data_smoothing_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::FocusCursor {
                            bu.focus_cursor_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::Gradients {
                            bu.gradients_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::HighLowBands {
                            bu.high_low_bands_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::LatencyHeatmap {
                            bu.latency_heatmap_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::LinePaths {
                            bu.line_paths_senkronlanıyor = false;
                            bu.line_paths_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::LogScales {
                            bu.log_scales_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::LogScales2 {
                            bu.log_scales2_senkronlanıyor = false;
                            bu.log_scales2_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::SyncCursor {
                            bu.sync_cursor_grubu = SyncCursorGrubu::yeni();
                            bu.sync_cursor_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::TimezonesDst {
                            bu.timezones_dst_senkronlanıyor = false;
                            bu.timezones_dst_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::NearestNonNull {
                            bu.nearest_non_null_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::MissingData {
                            bu.missing_data_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::Months {
                            bu.months_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::PathGapClip {
                            bu.path_gap_clip_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::PixelAlign {
                            bu.pixel_align_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::Points {
                            bu.points_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::ScalesDirOri {
                            bu.scales_dir_ori_yüzeylerini_oluştur(cx);
                        } else if bu.aktif_kart == KartKimliği::Scatter {
                            bu.scatter_yüzeylerini_oluştur(cx);
                        } else if matches!(bu.aktif_kart, KartKimliği::Bars(_)) {
                            bu.bars_grouped_stacked_yüzeylerini_oluştur(cx);
                        } else if matches!(bu.aktif_kart, KartKimliği::BarsValuesAutosize(_)) {
                            bu.bars_values_autosize_yüzeylerini_oluştur(cx);
                        } else if matches!(bu.aktif_kart, KartKimliği::BoxWhisker(_)) {
                            bu.box_whisker_yüzeylerini_oluştur(cx);
                        } else if matches!(bu.aktif_kart, KartKimliği::StreamData(_)) {
                            bu.stream_data_yüzeylerini_oluştur(cx);
                        } else if matches!(bu.aktif_kart, KartKimliği::ThinBars(_)) {
                            bu.thin_bars_yüzeylerini_oluştur(cx);
                        } else if matches!(bu.aktif_kart, KartKimliği::TimePeriods(_)) {
                            bu.time_periods_yüzeylerini_oluştur(cx);
                        } else if matches!(bu.aktif_kart, KartKimliği::TimelineDiscrete(_)) {
                            bu.timeline_discrete_yüzeylerini_oluştur(cx);
                        } else {
                            bu.grafiği_yenile(100, cx);
                        }
                    })),
            );

        // `Context` isteyen kartların kullandığı, bir önceki karede ölçülmüş alan.
        let görünür_alan = GörünürAlan::yeni(self.çizim_alanı, OTOMATİK_UYARLA);
        let alan_ölçeri = canvas(
            {
                let bu = cx.entity();
                move |sınır: Bounds<Pixels>, _pencere: &mut Window, cx: &mut App| {
                    // Ölçüm çizim sırasında alınır; varlık güncellemesi aynı
                    // karede yapılamayacağından bir sonraki tura ertelenir.
                    cx.defer(move |cx| {
                        bu.update(cx, |bu, cx| {
                            if bu.çizim_alanı != sınır.size {
                                bu.çizim_alanı = sınır.size;
                                cx.notify();
                            }
                        });
                    });
                }
            },
            |_sınır, _durum, _pencere, _cx| {},
        )
        .absolute()
        .size_full();
        let çizim_tabanı = div()
            .id("canli-chart")
            .relative()
            .child(alan_ölçeri)
            .flex_1()
            // Kalan dikey alan 320 px'in altına düştüğünde sabit alt sınır
            // yüzeyi görünür alandan taşırıyordu; yüzey artık kalan alana
            // sığacak şekilde ölçeklendiği için taban serbest bırakıldı.
            .min_h_0()
            .rounded_lg()
            .border_1()
            .border_color(rgb(0xe5e7eb))
            .bg(panel);
        let sync_yüzeyi = |örnek| {
            self.sync_cursor_grafikleri
                .iter()
                .find(|(kimlik, _)| *kimlik == örnek)
                .map(|(_, grafik)| grafik.clone())
        };
        let çizim = if aktif_kart == KartKimliği::AlignDataCost {
            let yüzey = |örnek| {
                self.align_data_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            let yüzeyler = AlignDataÖrneği::TÜMÜ
                .into_iter()
                .map(|örnek| (örnek, yüzey(örnek)))
                .collect::<Vec<_>>();
            çizim_tabanı
                .overflow_hidden()
                .child(uyarlanan_alan(OTOMATİK_UYARLA, move |alan| {
                    div()
                        .id("align-data-kaydirma")
                        .size_full()
                        .overflow_y_scroll()
                        .p_2()
                        .child(
                            div()
                                .p_2()
                                .rounded_md()
                                .bg(rgb(0xf8fafc))
                                .text_xs()
                                .text_color(soluk)
                                .child("Resmî align-data.html aynı sayfada iki bağımsız uPlot örneği kurar. İlk panel 5×5×1000 tabloyu altı ısınma turundan sonra NULL_EXPAND ile birleştirir ve yalnız onun spanGaps değeri saniyede bir değişir. İkinci panel farklı X dizilerindeki yoğun çizgi ile dört seyrek barı gösterir; zoom ve imleç durumları paylaşılmaz."),
                        )
                        .children(yüzeyler.into_iter().map(|(örnek, grafik)| {
                            let (doğal_genişlik, doğal_yükseklik, açıklama) = match örnek {
                                AlignDataÖrneği::HizalamaMaliyeti => (
                                    2_560.0,
                                    600.0,
                                    "25 birleşik seri; yalnız kırmızı, yeşil ve mavi çizilir · 6 warmup + 1 sonuç join",
                                ),
                                AlignDataÖrneği::ÇizgiVeÇubuk => (
                                    1_920.0,
                                    600.0,
                                    "38 noktalı kırmızı çizgi + dört mavi bar · bağımsız görünür aralık",
                                ),
                            };
                            let (genişlik, yükseklik) =
                                alan.yüzey(doğal_genişlik, doğal_yükseklik);
                            div()
                                .mt_3()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(metin)
                                        .child(örnek.başlık()),
                                )
                                .child(div().text_xs().text_color(soluk).child(açıklama))
                                .child(
                                    div()
                                        .id(SharedString::from(format!(
                                            "align-data-{}-kaydirma",
                                            örnek.kimlik()
                                        )))
                                        .w_full()
                                        .h(px(yükseklik))
                                        .overflow_x_scroll()
                                        .yalnız_tekerlek_ekseninde_kaydır()
                                        .child(
                                            div()
                                                .w(px(genişlik))
                                                .h(px(yükseklik))
                                                .when_some(grafik, |öğe, grafik| {
                                                    öğe.child(önbellekli_grafik(grafik))
                                                }),
                                        ),
                                )
                        }))
                }))
        } else if aktif_kart == KartKimliği::CustomScales {
            let yüzey = |örnek| {
                self.custom_scales_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            let yüzeyler = CustomScaleÖrneği::TÜMÜ
                .into_iter()
                .map(|örnek| (örnek, yüzey(örnek)))
                .collect::<Vec<_>>();
            çizim_tabanı
                .overflow_hidden()
                .child(uyarlanan_alan(OTOMATİK_UYARLA, move |alan| {
                    let (genişlik, yükseklik) = alan.yüzey(800.0, 800.0);
                    div()
                        .id("custom-scales-kaydirma")
                        .size_full()
                        .overflow_y_scroll()
                        .p_2()
                        .child(div().flex().flex_wrap().items_start().gap_3().children(
                            yüzeyler.into_iter().map(|(örnek, grafik)| {
                                div()
                                    .id(SharedString::from(format!(
                                        "custom-scales-{}-surface",
                                        örnek.kimlik()
                                    )))
                                    .flex_none()
                                    .w(px(genişlik))
                                    .h(px(yükseklik))
                                    .when_some(grafik, |öğe, grafik| {
                                        öğe.child(önbellekli_grafik(grafik))
                                    })
                            }),
                        ))
                }))
        } else if aktif_kart == KartKimliği::DataSmoothing {
            let yüzey = |örnek| {
                self.data_smoothing_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            let yüzeyler = SmoothingÖrneği::TÜMÜ
                .into_iter()
                .map(|örnek| (örnek, yüzey(örnek)))
                .collect::<Vec<_>>();
            çizim_tabanı
                .overflow_hidden()
                .child(uyarlanan_alan(OTOMATİK_UYARLA, move |alan| {
                    let (genişlik, yükseklik) = alan.yüzey(1_920.0, 300.0);
                    div()
                        .id("data-smoothing-kaydirma")
                        .size_full()
                        .overflow_y_scroll()
                        .p_2()
                        .children(yüzeyler.into_iter().map(|(örnek, grafik)| {
                            div()
                                .id(SharedString::from(format!(
                                    "data-smoothing-{}-surface",
                                    örnek.kimlik()
                                )))
                                .w_full()
                                .h(px(yükseklik))
                                .mb(px(50.0))
                                .overflow_x_scroll()
                                .yalnız_tekerlek_ekseninde_kaydır()
                                .child(
                                    div()
                                        .w(px(genişlik))
                                        .h(px(yükseklik))
                                        .when_some(grafik, |öğe, grafik| {
                                            öğe.child(önbellekli_grafik(grafik))
                                        }),
                                )
                        }))
                }))
        } else if aktif_kart == KartKimliği::FocusCursor {
            let yüzey = |örnek| {
                self.focus_cursor_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            let yüzeyler = FocusÖrneği::TÜMÜ
                .into_iter()
                .map(|örnek| (örnek, yüzey(örnek)))
                .collect::<Vec<_>>();
            çizim_tabanı
                .overflow_hidden()
                .child(uyarlanan_alan(OTOMATİK_UYARLA, move |alan| {
                    let (genişlik, yükseklik) = alan.yüzey(1_920.0, 600.0);
                    div()
                        .id("focus-cursor-kaydirma")
                        .size_full()
                        .overflow_y_scroll()
                        .p_2()
                        .children(yüzeyler.into_iter().map(|(örnek, grafik)| {
                            div()
                                .id(SharedString::from(format!(
                                    "focus-cursor-{}-surface",
                                    örnek.kimlik()
                                )))
                                .w_full()
                                .h(px(yükseklik))
                                .mb(px(50.0))
                                .overflow_x_scroll()
                                .yalnız_tekerlek_ekseninde_kaydır()
                                .child(
                                    div()
                                        .w(px(genişlik))
                                        .h(px(yükseklik))
                                        .when_some(grafik, |öğe, grafik| {
                                            öğe.child(önbellekli_grafik(grafik))
                                        }),
                                )
                        }))
                }))
        } else if aktif_kart == KartKimliği::Gradients {
            let yüzey = |örnek| {
                self.gradients_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            let yüzeyler = GradientÖrneği::TÜMÜ
                .into_iter()
                .map(|örnek| (örnek, yüzey(örnek)))
                .collect::<Vec<_>>();
            çizim_tabanı
                .overflow_hidden()
                .child(uyarlanan_alan(OTOMATİK_UYARLA, move |alan| {
                    let (genişlik, yükseklik) = alan.yüzey(800.0, 600.0);
                    div()
                        .id("gradients-kaydirma")
                        .size_full()
                        .overflow_y_scroll()
                        .p_2()
                        .child(div().flex().flex_wrap().items_start().gap_3().children(
                            yüzeyler.into_iter().map(|(örnek, grafik)| {
                                div()
                                    .id(SharedString::from(format!(
                                        "gradients-{}-surface",
                                        örnek.kimlik()
                                    )))
                                    .flex_none()
                                    .w(px(genişlik))
                                    .h(px(yükseklik))
                                    .when_some(grafik, |öğe, grafik| {
                                        öğe.child(önbellekli_grafik(grafik))
                                    })
                            }),
                        ))
                }))
        } else if aktif_kart == KartKimliği::HighLowBands {
            let yüzey = |örnek| {
                self.high_low_bands_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            let yüzeyler = HighLowBandsÖrneği::TÜMÜ
                .into_iter()
                .map(|örnek| (örnek, yüzey(örnek)))
                .collect::<Vec<_>>();
            çizim_tabanı
                .overflow_hidden()
                .child(uyarlanan_alan(OTOMATİK_UYARLA, move |alan| {
                    // Her yüzeyin başlığı 30 px yer kaplar; sığdırma payı geri kalanıdır.
                    let çizim_alanı = alan.pay_düş(30.0);
                    div()
                        .id("high-low-bands-kaydirma")
                        .size_full()
                        .overflow_y_scroll()
                        .p_2()
                        .children(yüzeyler.into_iter().map(|(örnek, grafik)| {
                            let (doğal_genişlik, doğal_yükseklik) = match örnek {
                                HighLowBandsÖrneği::HizalanmamışÇubuklar
                                | HighLowBandsÖrneği::HizalanmamışÇubukVuruşu => {
                                    (400.0, 300.0)
                                }
                                HighLowBandsÖrneği::ÇokİnceÇubuklar => (800.0, 300.0),
                                _ => (1_920.0, 600.0),
                            };
                            let (genişlik, yükseklik) =
                                çizim_alanı.yüzey(doğal_genişlik, doğal_yükseklik);
                            div()
                                .id(SharedString::from(format!(
                                    "high-low-bands-{}-surface",
                                    örnek.kimlik()
                                )))
                                .flex_none()
                                .w_full()
                                .h(px(yükseklik + 30.0))
                                .mb_3()
                                .overflow_x_scroll()
                                .yalnız_tekerlek_ekseninde_kaydır()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child(örnek.başlık()),
                                )
                                .child(
                                    div()
                                        .w(px(genişlik))
                                        .h(px(yükseklik))
                                        .when_some(grafik, |öğe, grafik| {
                                            öğe.child(önbellekli_grafik(grafik))
                                        }),
                                )
                        }))
                }))
        } else if aktif_kart == KartKimliği::LatencyHeatmap {
            çizim_tabanı.min_h_0().p_2().child(
                uniform_list(
                    "latency-heatmap-yuzey-listesi",
                    LatencyHeatmapÖrneği::TÜMÜ.len(),
                    cx.processor(|bu, aralık: Range<usize>, _pencere, _cx| {
                        // Yüzey başlığı 30 px; kalan alan sığdırmaya girer.
                        let (genişlik, yükseklik) =
                            GörünürAlan::yeni(bu.çizim_alanı, OTOMATİK_UYARLA)
                                .pay_düş(30.0)
                                .yüzey(1_800.0, 600.0);
                        aralık
                            .filter_map(|indeks| {
                                let örnek = LatencyHeatmapÖrneği::TÜMÜ.get(indeks).copied()?;
                                let grafik = bu
                                    .latency_heatmap_grafikleri
                                    .iter()
                                    .find(|(kimlik, _)| *kimlik == örnek)
                                    .map(|(_, grafik)| grafik.clone())?;
                                Some(
                                    div()
                                        .id(SharedString::from(format!(
                                            "latency-heatmap-{}-surface",
                                            örnek.kimlik()
                                        )))
                                        .flex_none()
                                        .w_full()
                                        .h(px(yükseklik + 30.0))
                                        .overflow_x_scroll()
                                        .yalnız_tekerlek_ekseninde_kaydır()
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child(örnek.başlık()),
                                        )
                                        .child(
                                            div()
                                                .w(px(genişlik))
                                                .h(px(yükseklik))
                                                .child(önbellekli_grafik(grafik)),
                                        ),
                                )
                            })
                            .collect::<Vec<_>>()
                    }),
                )
                .track_scroll(&self.latency_heatmap_kaydırma)
                .h_full(),
            )
        } else if aktif_kart == KartKimliği::LinePaths {
            let yüzey = |örnek| {
                self.line_paths_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            let yüzeyler = LinePathsÖrneği::TÜMÜ
                .into_iter()
                .map(|örnek| (örnek, yüzey(örnek)))
                .collect::<Vec<_>>();
            çizim_tabanı
                .overflow_hidden()
                .child(uyarlanan_alan(OTOMATİK_UYARLA, move |alan| {
                    let çizim_alanı = alan.pay_düş(30.0);
                    let (genişlik, yükseklik) = çizim_alanı.yüzey(2_400.0, 600.0);
                    div()
                        .id("line-paths-kaydirma")
                        .size_full()
                        .overflow_y_scroll()
                        .p_2()
                        .children(yüzeyler.into_iter().map(|(örnek, grafik)| {
                            div()
                                .id(SharedString::from(format!("{}-surface", örnek.kimlik())))
                                .flex_none()
                                .w_full()
                                .h(px(yükseklik + 30.0))
                                .mb_3()
                                .overflow_x_scroll()
                                .yalnız_tekerlek_ekseninde_kaydır()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child(örnek.başlık()),
                                )
                                .child(
                                    div()
                                        .w(px(genişlik))
                                        .h(px(yükseklik))
                                        .when_some(grafik, |öğe, grafik| {
                                            öğe.child(önbellekli_grafik(grafik))
                                        }),
                                )
                        }))
                }))
        } else if aktif_kart == KartKimliği::LogScales {
            let yüzey = |örnek| {
                self.log_scales_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            let yüzeyler = LogScalesÖrneği::TÜMÜ
                .into_iter()
                .map(|örnek| (örnek, yüzey(örnek)))
                .collect::<Vec<_>>();
            çizim_tabanı
                .overflow_hidden()
                .child(uyarlanan_alan(OTOMATİK_UYARLA, move |alan| {
                    let çizim_alanı = alan.pay_düş(30.0);
                    let (genişlik, yükseklik) = çizim_alanı.yüzey(1_600.0, 600.0);
                    div()
                        .id("log-scales-kaydirma")
                        .size_full()
                        .overflow_y_scroll()
                        .p_2()
                        .children(yüzeyler.into_iter().map(|(örnek, grafik)| {
                            div()
                                .id(SharedString::from(format!("{}-surface", örnek.kimlik())))
                                .flex_none()
                                .w_full()
                                .h(px(yükseklik + 30.0))
                                .mb_3()
                                .overflow_x_scroll()
                                .yalnız_tekerlek_ekseninde_kaydır()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child(örnek.başlık()),
                                )
                                .child(
                                    div()
                                        .w(px(genişlik))
                                        .h(px(yükseklik))
                                        .when_some(grafik, |öğe, grafik| {
                                            öğe.child(önbellekli_grafik(grafik))
                                        }),
                                )
                        }))
                }))
        } else if aktif_kart == KartKimliği::LogScales2 {
            let yüzey = |örnek| {
                self.log_scales2_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            let yüzeyler = LogScales2Örneği::TÜMÜ
                .into_iter()
                .map(|örnek| (örnek, yüzey(örnek)))
                .collect::<Vec<_>>();
            çizim_tabanı
                .overflow_hidden()
                .child(uyarlanan_alan(OTOMATİK_UYARLA, move |alan| {
                    div()
                        .id("log-scales2-kaydirma")
                        .size_full()
                        .overflow_y_scroll()
                        .p_2()
                        .children(yüzeyler.into_iter().map(|(örnek, grafik)| {
                            let (ham_genişlik, ham_yükseklik) = match örnek {
                                LogScales2Örneği::GenişDoğrusal
                                | LogScales2Örneği::GenişLog10
                                | LogScales2Örneği::GenişLog2
                                | LogScales2Örneği::PozitifFiltreli => (1_600.0, 600.0),
                                LogScales2Örneği::TersGiriş | LogScales2Örneği::TersÇıkış => {
                                    (1_600.0, 300.0)
                                }
                                LogScales2Örneği::TümüNull => (800.0, 400.0),
                                LogScales2Örneği::ÇokKüçük => (800.0, 600.0),
                                LogScales2Örneği::SeyrekLog10 | LogScales2Örneği::SeyrekLog2 => {
                                    (800.0, 300.0)
                                }
                                LogScales2Örneği::KısmiBüyük | LogScales2Örneği::KısmiKüçük => {
                                    (600.0, 300.0)
                                }
                            };
                            let ters_çift = matches!(
                                örnek,
                                LogScales2Örneği::TersGiriş | LogScales2Örneği::TersÇıkış
                            );
                            let başlık_payı = if ters_çift { 24.0 } else { 30.0 };
                            let (genişlik, yükseklik) =
                                alan.pay_düş(başlık_payı).yüzey(ham_genişlik, ham_yükseklik);
                            div()
                                .id(SharedString::from(format!("{}-surface", örnek.kimlik())))
                                .flex_none()
                                .w_full()
                                .h(px(yükseklik + başlık_payı))
                                .mb(if örnek == LogScales2Örneği::TersGiriş {
                                    px(0.0)
                                } else {
                                    px(12.0)
                                })
                                .overflow_x_scroll()
                                .yalnız_tekerlek_ekseninde_kaydır()
                                .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).child(
                                    if örnek == LogScales2Örneği::TersÇıkış {
                                        "Out · cursor.sync.key=\"moo\" · birleşik In/Out lejant"
                                    } else {
                                        örnek.başlık()
                                    },
                                ))
                                .child(
                                    div()
                                        .w(px(genişlik))
                                        .h(px(yükseklik))
                                        .when_some(grafik, |öğe, grafik| {
                                            öğe.child(önbellekli_grafik(grafik))
                                        }),
                                )
                        }))
                }))
        } else if aktif_kart == KartKimliği::SyncCursor {
            let cpu = sync_yüzeyi(SyncCursorÖrneği::Cpu);
            let ram = sync_yüzeyi(SyncCursorÖrneği::Ram);
            let tcp = sync_yüzeyi(SyncCursorÖrneği::Tcp);
            let kırmızı_mavi = sync_yüzeyi(SyncCursorÖrneği::UyumsuzKırmızıMavi);
            let yeşil_kırmızı = sync_yüzeyi(SyncCursorÖrneği::UyumsuzYeşilKırmızı);
            // Lejant düğmeleri `cx` ister; panel gövdesi ise yalnız ölçülmüş
            // yüksekliği. Düğmeler burada kurulur, gövde görünür alan
            // ölçüldükten sonra kapanışla üretilir.
            let sync_paneli = |örnek: SyncCursorÖrneği, grafik: Option<Entity<GpuiGrafik>>| {
                let seriler = grafik.as_ref().map_or_else(Vec::new, |grafik| {
                    grafik
                        .read(cx)
                        .grafik()
                        .seri_seçenekleri()
                        .iter()
                        .enumerate()
                        .map(|(indeks, seri)| {
                            (indeks, seri.etiket.clone(), seri.renk.clone(), seri.göster)
                        })
                        .collect::<Vec<_>>()
                });
                let düğmeler = seriler
                    .into_iter()
                    .map(|(indeks, etiket, _, görünür)| {
                        let yazı = SharedString::from(format!(
                            "● {etiket}{}",
                            if görünür { "" } else { " · gizli" }
                        ));
                        Dugme::yeni(
                            SharedString::from(format!("sync-{}-{indeks}", örnek.kimlik())),
                            yazı,
                        )
                        .boyutu(DugmeBoyutu::Kucuk)
                        .turu(if görünür {
                            DugmeTuru::Hayalet
                        } else {
                            DugmeTuru::Ikincil
                        })
                        .tiklaninca(cx.listener(move |bu, _, _, cx| {
                            bu.sync_cursor_serisini_değiştir(örnek, indeks, cx);
                        }))
                    })
                    .collect::<Vec<_>>();
                move |yükseklik: f32| {
                    div()
                        .flex_1()
                        .min_w_0()
                        .h(px(yükseklik))
                        .child(
                            div()
                                .w_full()
                                .h(px((yükseklik - SYNC_LEJANT_PAYI).max(1.0)))
                                .when_some(grafik, |öğe, grafik| {
                                    öğe.child(önbellekli_grafik(grafik))
                                }),
                        )
                        .child(div().flex().items_center().gap_1().children(düğmeler))
                        .into_any_element()
                }
            };
            let cpu_paneli = sync_paneli(SyncCursorÖrneği::Cpu, cpu);
            let ram_paneli = sync_paneli(SyncCursorÖrneği::Ram, ram);
            let tcp_paneli = sync_paneli(SyncCursorÖrneği::Tcp, tcp);
            let kırmızı_mavi_paneli =
                sync_paneli(SyncCursorÖrneği::UyumsuzKırmızıMavi, kırmızı_mavi);
            let yeşil_kırmızı_paneli =
                sync_paneli(SyncCursorÖrneği::UyumsuzYeşilKırmızı, yeşil_kırmızı);
            çizim_tabanı
                .overflow_hidden()
                .child(uyarlanan_alan(OTOMATİK_UYARLA, move |alan| {
                    // Grup üyeleri imleç ve zoom'u paylaşır; birlikte
                    // görünmeleri gerektiğinden üç satırın toplamı tek ölçekle
                    // sığdırılır, panel başına ayrı ölçek uygulanmaz.
                    let (_, toplam) = alan.yüzey(SYNC_TOPLAM_YÜKSEKLİK, SYNC_TOPLAM_YÜKSEKLİK);
                    let panel = SYNC_PANEL_YÜKSEKLİĞİ * toplam / SYNC_TOPLAM_YÜKSEKLİK;
                    div()
                        .id("sync-cursor-kaydirma")
                        .size_full()
                        .overflow_y_scroll()
                        .p_2()
                        .child(cpu_paneli(panel))
                        .child(
                            div()
                                .mt_2()
                                .flex()
                                .gap_2()
                                .children([ram_paneli(panel), tcp_paneli(panel)]),
                        )
                        .child(
                            div().mt_2().flex().gap_2().children([
                                kırmızı_mavi_paneli(panel),
                                yeşil_kırmızı_paneli(panel),
                            ]),
                        )
                }))
        } else if aktif_kart == KartKimliği::TimeseriesDiscrete {
            let üst = self
                .timeseries_discrete_grafikleri
                .iter()
                .find(|(örnek, _)| *örnek == TimeseriesDiscreteÖrneği::ZamanSerisi)
                .map(|(_, grafik)| grafik.clone());
            let alt = self
                .timeseries_discrete_grafikleri
                .iter()
                .find(|(örnek, _)| *örnek == TimeseriesDiscreteÖrneği::AyrıkDurumlar)
                .map(|(_, grafik)| grafik.clone());
            çizim_tabanı
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .w(px(görünür_alan
                            .pay_düş(AÇIKLAMA_PAYI)
                            .yüzey(1920.0, 600.0)
                            .0))
                        .h(px(görünür_alan
                            .pay_düş(AÇIKLAMA_PAYI)
                            .yüzey(1920.0, 600.0)
                            .1))
                        .when_some(üst, |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                )
                .child(
                    div()
                        .mt_2()
                        .w(px(görünür_alan
                            .pay_düş(AÇIKLAMA_PAYI)
                            .yüzey(1920.0, 200.0)
                            .0))
                        .h(px(görünür_alan
                            .pay_düş(AÇIKLAMA_PAYI)
                            .yüzey(1920.0, 200.0)
                            .1))
                        .when_some(alt, |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                )
        } else if aktif_kart == KartKimliği::TimezonesDst {
            // 51 yüzey sarmalı flex içinde her render'da kuruluyordu. Kaynak
            // sayfanın üç sütunlu görünümü korunmak için bölümler üçerli
            // satırlara toplanır ve satırlar sanallaştırılır; bölüm
            // yükseklikleri (3–6 yüzey) farklı olduğundan `uniform_list`
            // değil `list` kullanılır.
            const SÜTUN: usize = 3;
            let yüzeyler = self.timezones_dst_grafikleri.clone();
            let bölüm_sayısı = 11_usize;
            let satır_sayısı = bölüm_sayısı.div_ceil(SÜTUN);
            let durum = self
                .timezones_dst_liste_durumu
                .get_or_insert_with(|| ListState::new(satır_sayısı, ListAlignment::Top, px(600.0)));
            çizim_tabanı.min_h_0().p_2().child(
                list(durum.clone(), move |satır, _pencere, _cx| {
                    let ilk_bölüm = satır * SÜTUN;
                    div()
                        .flex()
                        .items_start()
                        .gap_4()
                        .mb_4()
                        .children(
                            (ilk_bölüm..(ilk_bölüm + SÜTUN).min(bölüm_sayısı)).map(|bölüm| {
                                let bölümde =
                                    |örnek: &TimezonesDstÖrneği| örnek.bölüm_indeksi() == bölüm;
                                let başlık = yüzeyler
                                    .iter()
                                    .find(|(örnek, _)| bölümde(örnek))
                                    .map_or("Timezones & DST", |(örnek, _)| örnek.bölüm());
                                div()
                                    .w(px(600.0))
                                    .flex_none()
                                    .overflow_hidden()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(rgb(0xd1d5db))
                                    .bg(rgb(0xffffff))
                                    .child(
                                        div()
                                            .w_full()
                                            .px_3()
                                            .py_2()
                                            .text_center()
                                            .text_sm()
                                            .bg(rgb(0xe1f5fe))
                                            .child(başlık),
                                    )
                                    .children(
                                        yüzeyler.iter().filter(|(örnek, _)| bölümde(örnek)).map(
                                            |(_, grafik)| {
                                                div()
                                                    .w(px(600.0))
                                                    .h(px(200.0))
                                                    .child(önbellekli_grafik(grafik.clone()))
                                            },
                                        ),
                                    )
                            }),
                        )
                        .into_any_element()
                })
                .h_full(),
            )
        } else if aktif_kart == KartKimliği::NearestNonNull {
            let yüzey = |örnek| {
                self.nearest_non_null_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            let tam = [
                NearestNonNullÖrneği::XDeğerineGöre,
                NearestNonNullÖrneği::OtuzPikselYakınlık,
                NearestNonNullÖrneği::NullİseOnBeşPiksel,
            ];
            let küçük = [
                NearestNonNullÖrneği::ÖncekiSeri,
                NearestNonNullÖrneği::ÖncekiİmleçVeSeri,
            ];
            let tam_yüzeyler = tam
                .into_iter()
                .map(|örnek| (örnek, yüzey(örnek)))
                .collect::<Vec<_>>();
            let küçük_yüzeyler = küçük
                .into_iter()
                .map(|örnek| (örnek, yüzey(örnek)))
                .collect::<Vec<_>>();
            çizim_tabanı
                .overflow_hidden()
                .child(uyarlanan_alan(OTOMATİK_UYARLA, move |alan| {
                    // Yüzeyler `w_full`; en boy oranını kapsayıcı belirlediği
                    // için yalnız yükseklik görünür alana çekilir. 20 px pay
                    // yüzey üstündeki kısa açıklama satırınındır.
                    let tam_yükseklik = alan.pay_düş(20.0).dikey(300.0);
                    let küçük_yükseklik = alan.pay_düş(20.0).dikey(250.0);
                    div()
                        .id("nearest-non-null-kaydirma")
                        .size_full()
                        .overflow_y_scroll()
                        .p_2()
                        .child(
                            div()
                                .p_2()
                                .rounded_md()
                                .bg(rgb(0xf8fafc))
                                .text_xs()
                                .text_color(soluk)
                                .child("Amaç: eksik telemetride en yakın X, piksel proximity ve önceki örnek politikalarını karşılaştırır. ")
                                .child("API: null_imleç_düzeni seri başına gerçek index/x/value üretir; null ile join hizalama eksiği ayrıdır. ")
                                .child("İzleme: legend gerçek örnek zamanını göstermeli ve eski değeri güncelmiş gibi sunmamak için stale eşiği koymalıdır. ")
                                .child("Maliyet: sıralı X araması O(log N), null koşusu O(K); hover yalnız hafif cursor katmanını boyar."),
                        )
                        .children(tam_yüzeyler.into_iter().map(|(örnek, grafik)| {
                            div()
                                .mt_2()
                                .w_full()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(soluk)
                                        .child(örnek.kısa_açıklama()),
                                )
                                .child(
                                    div()
                                        .w_full()
                                        .h(px(tam_yükseklik))
                                        .when_some(grafik, |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                                )
                        }))
                        .child(
                            div()
                                .mt_2()
                                .flex()
                                .gap_2()
                                .children(küçük_yüzeyler.into_iter().map(|(örnek, grafik)| {
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(soluk)
                                                .child(örnek.kısa_açıklama()),
                                        )
                                        .child(
                                            div()
                                                .w_full()
                                                .h(px(küçük_yükseklik))
                                                .when_some(grafik, |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                                        )
                                })),
                        )
                }))
        } else if aktif_kart == KartKimliği::MissingData {
            çizim_tabanı
                .overflow_y_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Aynı missing-data.html sayfasındaki iki bağımsız yüzey birlikte gösterilir; cursor, seçim ve görünüm durumları senkronlanmaz."),
                )
                .children(MissingDataÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    let grafik = self
                        .missing_data_grafikleri
                        .iter()
                        .find(|(kimlik, _)| *kimlik == örnek)
                        .map(|(_, grafik)| grafik.clone());
                    let seriler = grafik.as_ref().map_or_else(Vec::new, |grafik| {
                        grafik
                            .read(cx)
                            .grafik()
                            .seri_seçenekleri()
                            .iter()
                            .enumerate()
                            .map(|(indeks, seri)| (indeks, seri.etiket.clone(), seri.göster))
                            .collect::<Vec<_>>()
                    });
                    div()
                        .mt_3()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .child(örnek.başlık()),
                        )
                        .child(div().text_xs().text_color(soluk).child(örnek.açıklama()))
                        .child(
                            div()
                                .w_full()
                                .h(px(görünür_alan.pay_düş(AÇIKLAMA_PAYI).dikey(520.0)))
                                .when_some(grafik, |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                        )
                        .child(div().flex().flex_wrap().gap_1().children(
                            seriler.into_iter().map(|(indeks, etiket, görünür)| {
                                Dugme::yeni(
                                    format!("missing-data-{}-{indeks}", örnek.kimlik()),
                                    format!(
                                        "{} {}",
                                        if görünür { "✓" } else { "○" },
                                        if etiket.is_empty() { "Value" } else { &etiket }
                                    ),
                                )
                                .boyutu(DugmeBoyutu::Kucuk)
                                .turu(DugmeTuru::Ikincil)
                                .tiklaninca(cx.listener(move |bu, _, _, cx| {
                                    bu.missing_data_serisini_değiştir(örnek, indeks, cx);
                                }))
                            }),
                        ))
                }))
        } else if aktif_kart == KartKimliği::Months {
            let yüzey = |indeks: usize| self.months_grafikleri.get(indeks).cloned();
            çizim_tabanı
                .overflow_y_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("İlk iki yüzey aynı months.html sayfasının bağımsız normal/artık yıl karşılaştırmasıdır; yakınlaştırmaları birbirine bağlı değildir. ")
                        .child("Her X UTC'de ayın ilk günüdür. Kaynak 28 günlük piksel-space kuralı gerçek takvim ayı bölmelerini korur; 2024 Şubat 29 gündür."),
                )
                .children(
                    [
                        (
                            0,
                            "2017–2019 · artık yıl yok",
                            "Normal Gregoryen ay uzunlukları; kaynak monthly tick politikası.",
                            200.0,
                        ),
                        (
                            1,
                            "2024–2026 · artık yıl var",
                            "Şubat 2024 → Mart 2024 aralığı 29 gün; yüzey bağımsızdır.",
                            200.0,
                        ),
                    ]
                    .into_iter()
                    .map(|(indeks, başlık, açıklama, yükseklik)| {
                        div()
                            .mt_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(metin)
                                    .child(başlık),
                            )
                            .child(div().text_xs().text_color(soluk).child(açıklama))
                            .child(
                                div()
                                    .id(SharedString::from(format!("months-kaydirma-{indeks}")))
                                    .w_full()
                                    .h(px(yükseklik))
                                    .overflow_x_scroll()
                                    .yalnız_tekerlek_ekseninde_kaydır()
                                    .child(
                                        div()
                                            .w(px(1920.0))
                                            .h(px(yükseklik))
                                            .when_some(yüzey(indeks), |öğe, grafik| {
                                                öğe.child(önbellekli_grafik(grafik))
                                            }),
                                    ),
                            )
                    }),
                )
        } else if aktif_kart == KartKimliği::PathGapClip {
            let gruplar: [(&str, &str, &[PathGapClipÖrneği]); 5] = [
                (
                    "Band ve canlı spanGaps",
                    "İki band yüzeyi, ayrı join grubundaki iki canlı yüzeyle aynı bir saniyelik bridge fazını paylaşır.",
                    &[
                        PathGapClipÖrneği::VeriDışınaTaşanÖlçek,
                        PathGapClipÖrneği::BantBoşlukları,
                    ],
                ),
                (
                    "Stepped yol karşılaştırması",
                    "Doğrudan ve join edilmiş veride before/after boşluk sınırları yan yana incelenir.",
                    &[
                        PathGapClipÖrneği::BasamakSonra,
                        PathGapClipÖrneği::BasamakÖnce,
                        PathGapClipÖrneği::BirleşikBasamakSonra,
                        PathGapClipÖrneği::BirleşikBasamakÖnce,
                    ],
                ),
                (
                    "Join ve null türleri",
                    "NULL_EXPAND ile sayısal join gerçek null'u hizalama eksiğinden ayırır; ikisi de ortak bridge durumuna katılır.",
                    &[
                        PathGapClipÖrneği::GenişletilmişHizalama,
                        PathGapClipÖrneği::SayısalHizalama,
                    ],
                ),
                (
                    "Piksel sınırı regresyonları",
                    "Ters/normal yoğun seri ile dört küçük alt-piksel varyasyonu kaynak boyutunda çizilir.",
                    &[
                        PathGapClipÖrneği::TekBoşlukÇıkışı,
                        PathGapClipÖrneği::TekBoşlukGirişi,
                        PathGapClipÖrneği::TekBoşluk3001,
                        PathGapClipÖrneği::TekBoşluk4999,
                        PathGapClipÖrneği::TekBoşluk5001,
                        PathGapClipÖrneği::ÇiftBoşluk,
                    ],
                ),
                (
                    "Null ve undefined",
                    "Gerçek null görünür bir gap açar; undefined yalnız hizalama eksiği olarak atlanır.",
                    &[PathGapClipÖrneği::Tanımsız],
                ),
            ];
            çizim_tabanı
                .overflow_y_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Kaynak sayfadaki 15 yüzey tek ailede tutulur. Her yüzeyin imleç, seçim ve wheel görünümü bağımsızdır; yalnız scale-range, band-gaps, expand ve numeric yüzeyleri spanGaps durumunu aynı saniyede değiştirir."),
                )
                .children(gruplar.into_iter().map(|(grup, açıklama, örnekler)| {
                    div()
                        .mt_4()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(metin)
                                .child(grup),
                        )
                        .child(div().text_xs().text_color(soluk).child(açıklama))
                        .children(örnekler.iter().filter_map(|örnek| {
                            let grafik = self
                                .path_gap_clip_grafikleri
                                .iter()
                                .find(|(kimlik, _)| kimlik == örnek)
                                .map(|(_, grafik)| grafik.clone())?;
                            let (ham_genişlik, ham_yükseklik) = örnek.kaynak_boyutu();
                            let (genişlik, yükseklik) = görünür_alan
                                .pay_düş(AÇIKLAMA_PAYI)
                                .yüzey(ham_genişlik as f32, ham_yükseklik as f32);
                            Some(
                                div()
                                    .mt_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(metin)
                                            .child(örnek.başlık()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(soluk)
                                            .child(örnek.kısa_açıklama()),
                                    )
                                    .child(
                                        div()
                                            .id(SharedString::from(format!(
                                                "{}-kaydirma",
                                                örnek.kimlik()
                                            )))
                                            .w_full()
                                            .h(px(yükseklik))
                                            .overflow_x_scroll()
                                            .yalnız_tekerlek_ekseninde_kaydır()
                                            .child(
                                                div()
                                                    .w(px(genişlik))
                                                    .h(px(yükseklik))
                                                    .child(önbellekli_grafik(grafik)),
                                            ),
                                    ),
                            )
                        }))
                }))
        } else if aktif_kart == KartKimliği::PixelAlign {
            let yüzey = |örnek| {
                self.pixel_align_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .overflow_y_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Kaynak gibi boş başlayan iki panel aynı halka verisini ve animation-frame saatini paylaşır; ilk örnek 1 saniyede gelir. Üst panel tam pikselde keskin fakat basamaklı “tırtıl”, alt panel alt-pikselde yumuşak hareket üretir. Zoom canlı takibi duraklatır; Tam görünüm takibi sürdürür."),
                )
                .children([
                    (
                        PixelAlignÖrneği::Varsayılan,
                        "Tam piksel · pxAlign 1",
                        "Path, point, axis ve grid koordinatları en yakın piksele yuvarlanır.",
                    ),
                    (
                        PixelAlignÖrneği::Kapalı,
                        "Alt piksel · pxAlign 0",
                        "Koordinatlar yuvarlanmaz; kayan 120 saniyelik pencere frame düzeyinde ilerler.",
                    ),
                ].into_iter().map(|(örnek, başlık, açıklama)| {
                    div()
                        .mt_2()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(metin)
                                .child(başlık),
                        )
                        .child(div().text_xs().text_color(soluk).child(açıklama))
                        .child(
                            div()
                                .w_full()
                                .h(px(görünür_alan.pay_düş(AÇIKLAMA_PAYI).dikey(360.0)))
                                .when_some(yüzey(örnek), |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                        )
                }))
        } else if aktif_kart == KartKimliği::Points {
            let yüzey = |örnek| {
                self.points_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .overflow_y_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Dört panel resmî points.html sayfasının tek anlatımıdır. Ortadaki Points ve Too dense test aynı 180 noktalı veriyi paylaşır; yalnız X aralığı değişerek varsayılan nokta yoğunluğu eşiğini karşılaştırır."),
                )
                .children(PointsÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    let (ham_genişlik, ham_yükseklik) = örnek.kaynak_boyutu();
                            let (genişlik, yükseklik) = görünür_alan
                                .pay_düş(AÇIKLAMA_PAYI)
                                .yüzey(ham_genişlik as f32, ham_yükseklik as f32);
                    div()
                        .mt_2()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(metin)
                                .child(örnek.başlık()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(soluk)
                                .child(örnek.kısa_açıklama()),
                        )
                        .child(
                            div()
                                .id(SharedString::from(format!(
                                    "{}-kaydirma",
                                    örnek.kimlik()
                                )))
                                .w_full()
                                .h(px(yükseklik))
                                .overflow_x_scroll()
                                .yalnız_tekerlek_ekseninde_kaydır()
                                .child(
                                    div()
                                        .w(px(genişlik))
                                        .h(px(yükseklik))
                                        .when_some(yüzey(örnek), |öğe, grafik| {
                                            öğe.child(önbellekli_grafik(grafik))
                                        }),
                                ),
                        )
                }))
        } else if aktif_kart == KartKimliği::ScalesDirOri {
            let yüzey = |örnek| {
                self.scales_dir_ori_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            let grup = |başlık: &'static str, dikey: bool| {
                div()
                    .mt_3()
                    .child(
                        div()
                            .mb_2()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(metin)
                            .child(başlık),
                    )
                    .child(
                        div().flex().flex_wrap().gap_2().items_start().children(
                            ScalesDirOriÖrneği::TÜMÜ
                                .into_iter()
                                .filter(move |örnek| örnek.x_dikey() == dikey)
                                .map(|örnek| {
                                    let (ham_genişlik, ham_yükseklik) = örnek.boyut();
                                    let (genişlik, yükseklik) = görünür_alan
                                        .pay_düş(AÇIKLAMA_PAYI)
                                        .yüzey(ham_genişlik as f32, ham_yükseklik as f32);
                                    div()
                                        .flex_none()
                                        .w(px(genişlik))
                                        .child(
                                            div()
                                                .mb_1()
                                                .text_xs()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(soluk)
                                                .child(örnek.başlık()),
                                        )
                                        .child(
                                            div()
                                                .w(px(genişlik))
                                                .h(px(yükseklik))
                                                .when_some(yüzey(örnek), |öğe, grafik| {
                                                    öğe.child(önbellekli_grafik(grafik))
                                                }),
                                        )
                                }),
                        ),
                    )
            };
            çizim_tabanı
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("On altı panel aynı 10 X değeri ile kırmızı ve mavi seri anlık görüntüsünü paylaşır. İlk grup scale.dir yön terslemelerini; ikinci grup scale.ori ile X/Y eksenlerinin yer değiştirmesini karşılaştırır."),
                )
                .child(grup("Direction Inversion", false))
                .child(grup("Orientation Inversion", true))
        } else if aktif_kart == KartKimliği::Scatter {
            let yüzey = |örnek| {
                self.scatter_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .overflow_y_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Resmî scatter.html iki eşzamanlı fakat bağımsız mode:2 grafik kurar. İlk yüzey 40.000 sabit noktayı seri başına toplu çizer; ikinci yüzey alan ölçekli 200 balonu ayrı veri, y/y2 ölçekleri ve uzamsal hover diziniyle gösterir."),
                )
                .children(ScatterÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    let açıklama = match örnek {
                        ScatterÖrneği::Scatter => {
                            "4 facet × 10.000 sabit 5 px nokta · live legend kapalı"
                        }
                        ScatterÖrneği::Bubble => {
                            "4 facet × 50 balon · Country / Population / GDP / Income hover"
                        }
                    };
                    div()
                        .mt_2()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(metin)
                                .child(örnek.başlık()),
                        )
                        .child(div().text_xs().text_color(soluk).child(açıklama))
                        .child(
                            div()
                                .id(SharedString::from(format!(
                                    "{}-kaydirma",
                                    örnek.kimlik()
                                )))
                                .w_full()
                                .h(px(600.0))
                                .overflow_x_scroll()
                                .yalnız_tekerlek_ekseninde_kaydır()
                                .child(
                                    div()
                                        .w(px(görünür_alan.pay_düş(AÇIKLAMA_PAYI).yüzey(1920.0, 600.0).0))
                                .h(px(görünür_alan.pay_düş(AÇIKLAMA_PAYI).yüzey(1920.0, 600.0).1))
                                        .when_some(yüzey(örnek), |öğe, grafik| {
                                            öğe.child(önbellekli_grafik(grafik))
                                        }),
                                ),
                        )
                }))
        } else if matches!(aktif_kart, KartKimliği::Bars(_)) {
            let yüzey = |örnek| {
                self.bars_grouped_stacked_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Resmî sayfadaki on uPlot yüzeyi kaynak sırasında ve bağımsız görünüm geçmişleriyle birlikte gösterilir. Lejant düğmeleri setSeries davranışıdır: gizlenen grouped seri kendi yuvasını, stacked seri kümülatif boşluğunu korur."),
                )
                .children(ÇubukÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    let grafik = yüzey(örnek);
                    let seriler = grafik.as_ref().map_or_else(Vec::new, |grafik| {
                        grafik
                            .read(cx)
                            .grafik()
                            .seri_seçenekleri()
                            .iter()
                            .enumerate()
                            .map(|(indeks, seri)| {
                                (indeks, seri.etiket.clone(), seri.renk.clone(), seri.göster)
                            })
                            .collect::<Vec<_>>()
                    });
                    let (genişlik, yükseklik) = if örnek.yatay() {
                        (400.0, 800.0)
                    } else {
                        (800.0, 400.0)
                    };
                    let lejantlar = seriler
                        .into_iter()
                        .map(|(indeks, etiket, renk, görünür)| {
                            Dugme::yeni(
                                SharedString::from(format!("{}-seri-{indeks}", örnek.kimlik())),
                                SharedString::from(format!(
                                    "● {etiket} · {renk}{}",
                                    if görünür { "" } else { " · gizli" }
                                )),
                            )
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(if görünür {
                                DugmeTuru::Hayalet
                            } else {
                                DugmeTuru::Ikincil
                            })
                            .tiklaninca(cx.listener(move |bu, _, _, cx| {
                                bu.bars_serisini_değiştir(örnek, indeks, cx);
                            }))
                        })
                        .collect::<Vec<_>>();
                    div()
                        .mt_4()
                        .w(px(genişlik))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(metin)
                                .child(örnek.başlık()),
                        )
                        .child(
                            div()
                                .mb_2()
                                .text_xs()
                                .text_color(soluk)
                                .child(örnek.açıklama()),
                        )
                        .child(div().mb_2().flex().flex_wrap().gap_2().children(lejantlar))
                        .child(
                            div()
                                .w(px(genişlik))
                                .h(px(yükseklik))
                                .when_some(grafik, |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                        )
                }))
        } else if matches!(aktif_kart, KartKimliği::BarsValuesAutosize(_)) {
            let yüzey = |yön| {
                self.bars_values_autosize_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == yön)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Resmî sayfadaki dikey ve yatay 1275×600 uPlot yüzeyleri aynı veri dizisini paylaşır. Yeşil kanıt alanı etiket için kullanılabilir boşluğu, 10–25 px ortak font kararı ise en dar değer/kenar koşulunu gösterir."),
                )
                .children([ÇubukYönü::Dikey, ÇubukYönü::Yatay].into_iter().map(|yön| {
                    let grafik = yüzey(yön);
                    let görünür = grafik
                        .as_ref()
                        .is_some_and(|grafik| grafik.read(cx).grafik().seri_görünür_mü(0));
                    let başlık = if yön == ÇubukYönü::Dikey {
                        "Vertical bars · width + edge-space fit"
                    } else {
                        "Horizontal bars · height fit"
                    };
                    let açıklama = if yön == ÇubukYönü::Dikey {
                        "Tüm kompakt değerlerin en büyük metin ölçüsü, en dar bar genişliği ve pozitif/negatif kenar boşluğuyla tek font boyutu üretir."
                    } else {
                        "En dar bar yüksekliğinin %80'i tek font boyutunu üretir; değer bar ucunun dışına, işaretine göre yerleşir."
                    };
                    div()
                        .mt_4()
                        .w(px(1275.0))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(metin)
                                .child(başlık),
                        )
                        .child(div().mb_2().text_xs().text_color(soluk).child(açıklama))
                        .child(
                            div().mb_2().child(
                                Dugme::yeni(
                                    SharedString::from(format!(
                                        "bars-values-{}-seri",
                                        if yön == ÇubukYönü::Dikey {
                                            "vertical"
                                        } else {
                                            "horizontal"
                                        }
                                    )),
                                    SharedString::from(format!(
                                        "● Value · #00000033{}",
                                        if görünür { "" } else { " · gizli" }
                                    )),
                                )
                                .boyutu(DugmeBoyutu::Kucuk)
                                .turu(if görünür {
                                    DugmeTuru::Hayalet
                                } else {
                                    DugmeTuru::Ikincil
                                })
                                .tiklaninca(cx.listener(move |bu, _, _, cx| {
                                    bu.bars_values_serisini_değiştir(yön, cx);
                                })),
                            ),
                        )
                        .child(
                            div()
                                .w(px(görünür_alan.pay_düş(AÇIKLAMA_PAYI).yüzey(1275.0, 600.0).0))
                                .h(px(görünür_alan.pay_düş(AÇIKLAMA_PAYI).yüzey(1275.0, 600.0).1))
                                .when_some(grafik, |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                        )
                }))
        } else if matches!(aktif_kart, KartKimliği::BoxWhisker(_)) {
            çizim_tabanı
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Resmî sayfanın 17 bağımsız 800×400 benchmark yüzeyi aynı alandadır. Sütun hover'ı ana geometriyi yeniden çizmeden mavi vurgu ve Lib/Median/q1/q3/min/max bilgi kutusunu hafif etkileşim katmanında günceller."),
                )
                .child(
                    div()
                        .w(px(1680.0))
                        .flex()
                        .flex_wrap()
                        .gap_3()
                        .items_start()
                        .children(self.box_whisker_grafikleri.iter().map(
                            |(benchmark, grafik)| {
                                div()
                                    .flex_none()
                                    .w(px(820.0))
                                    .p_2()
                                    .rounded_md()
                                    .bg(rgb(0xffffff))
                                    .border_1()
                                    .border_color(rgb(0xd1d5db))
                                    .child(
                                        div()
                                            .mb_2()
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(metin)
                                            .child(*benchmark),
                                    )
                                    .child(div().w(px(800.0)).h(px(400.0)).child(grafik.clone()))
                            },
                        )),
                )
        } else if matches!(aktif_kart, KartKimliği::SoftMinMax(_)) {
            let yüzey = |örnek| {
                self.soft_minmax_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Resmî soft-minmax.html tek veri nesnesini paylaşan dört canlı rangeNum karşılaştırmasını ve bağımsız düz-sıfır yüzeyini birlikte gösterir. → dataMax++ tek ortak değeri her 50 ms’de dört canlı yüzeye aynı adımda uygular."),
                )
                .child(
                    div().flex().flex_wrap().gap_3().items_start().children(
                        SoftMinMaxÖrneği::TÜMÜ.into_iter().map(|örnek| {
                            div()
                                .flex_none()
                                .w(px(400.0))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(metin)
                                        .child(örnek.başlık()),
                                )
                                .child(
                                    div()
                                        .h(px(58.0))
                                        .text_xs()
                                        .text_color(soluk)
                                        .child(örnek.açıklama()),
                                )
                                .child(
                                    div()
                                        .w(px(görünür_alan.pay_düş(AÇIKLAMA_PAYI).yüzey(400.0, 400.0).0))
                                .h(px(görünür_alan.pay_düş(AÇIKLAMA_PAYI).yüzey(400.0, 400.0).1))
                                        .when_some(yüzey(örnek), |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                                )
                        }),
                    ),
                )
        } else if matches!(aktif_kart, KartKimliği::SparklinesBars(_)) {
            let yüzey = |örnek| {
                self.sparklines_bars_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Resmî iki 800×400 yüzey aynı 16 değerli veri, −25…15 Y ölçeği, sparkline ve yüzen bar geometrisini paylaşır. Yalnız ilk yüzeyin ölçeğe bağlı ayrık gradyanı, ikinci yüzeyin nokta başına kırmızı/yeşil dolgularıyla değiştirilir."),
                )
                .children(SparklinesBarsÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    let açıklama = match örnek {
                        SparklinesBarsÖrneği::GradyanÇubuklar => {
                            "Çizgi alanı ve yüzen çubuklar görünür Y ölçeğinin uçlarına bağlı kırmızı/beyaz/yeşil gradyan kullanır."
                        }
                        SparklinesBarsÖrneği::AyrıkRenkliÇubuklar => {
                            "Her çubuk low veya high ucu negatifse kırmızı, ikisi de negatif değilse yeşildir."
                        }
                    };
                    div()
                        .mt_2()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(metin)
                                .child(örnek.başlık()),
                        )
                        .child(div().text_xs().text_color(soluk).child(açıklama))
                        .child(
                            div()
                                .w(px(görünür_alan.pay_düş(AÇIKLAMA_PAYI).yüzey(800.0, 400.0).0))
                                .h(px(görünür_alan.pay_düş(AÇIKLAMA_PAYI).yüzey(800.0, 400.0).1))
                                .when_some(yüzey(örnek), |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                        )
                }))
        } else if matches!(aktif_kart, KartKimliği::Sparklines(_)) {
            let yüzey = |örnek| {
                self.sparklines_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .w(px(384.0))
                        .flex()
                        .child(
                            div()
                                .w(px(84.0))
                                .h(px(30.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .font_weight(FontWeight::BOLD)
                                .child("Simge"),
                        )
                        .child(
                            div()
                                .w(px(150.0))
                                .h(px(30.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .font_weight(FontWeight::BOLD)
                                .child("Volume"),
                        )
                        .child(
                            div()
                                .w(px(150.0))
                                .h(px(30.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .font_weight(FontWeight::BOLD)
                                .child("Close"),
                        ),
                )
                .children(
                    SparklineÖrneği::SATIRLAR
                        .into_iter()
                        .map(|(hacim, kapanış)| {
                            div()
                                .w(px(384.0))
                                .h(px(30.0))
                                .flex()
                                .child(
                                    div()
                                        .w(px(84.0))
                                        .h(px(30.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .font_weight(FontWeight::BOLD)
                                        .child(hacim.simge()),
                                )
                                .child(
                                    div()
                                        .w(px(150.0))
                                        .h(px(30.0))
                                        .bg(rgb(0xffc0cb))
                                        .when_some(yüzey(hacim), |öğe, grafik| {
                                            öğe.child(önbellekli_grafik(grafik))
                                        }),
                                )
                                .child(
                                    div()
                                        .w(px(150.0))
                                        .h(px(30.0))
                                        .bg(rgb(0xffc0cb))
                                        .when_some(yüzey(kapanış), |öğe, grafik| {
                                            öğe.child(önbellekli_grafik(grafik))
                                        }),
                                )
                        }),
                )
        } else if matches!(aktif_kart, KartKimliği::Sparse(_)) {
            let yüzey = |örnek| {
                self.sparse_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .overflow_scroll()
                .p_2()
                .children(SparseÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    div()
                        .mb_2()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .child(örnek.başlık()),
                        )
                        .child(
                            div()
                                .w(px(800.0))
                                .h(px(200.0))
                                .border_1()
                                .border_color(rgb(0xc0c0c0))
                                .when_some(yüzey(örnek), |öğe, grafik| {
                                    öğe.child(önbellekli_grafik(grafik))
                                }),
                        )
                }))
        } else if matches!(aktif_kart, KartKimliği::StackedSeries(_)) {
            çizim_tabanı.overflow_scroll().p_2().children(
                StackedSeriesÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    let grafik = self
                        .stacked_series_grafikleri
                        .iter()
                        .find(|(kimlik, _)| *kimlik == örnek)
                        .map(|(_, grafik)| grafik.clone());
                    let seriler = grafik.as_ref().map_or_else(Vec::new, |grafik| {
                        grafik
                            .read(cx)
                            .grafik()
                            .seri_seçenekleri()
                            .iter()
                            .enumerate()
                            .map(|(indeks, seri)| (indeks, seri.etiket.clone(), seri.göster))
                            .collect::<Vec<_>>()
                    });
                    let (ham_genişlik, ham_yükseklik) = örnek.boyut();
                    let (genişlik, yükseklik) = görünür_alan
                        .pay_düş(AÇIKLAMA_PAYI)
                        .yüzey(ham_genişlik as f32, ham_yükseklik as f32);
                    div()
                        .mb_4()
                        .child(
                            div()
                                .w(px(genişlik))
                                .h(px(yükseklik))
                                .border_1()
                                .border_color(rgb(0xc0c0c0))
                                .when_some(grafik, |öğe, grafik| {
                                    öğe.child(önbellekli_grafik(grafik))
                                }),
                        )
                        .child(div().mt_1().flex().flex_wrap().gap_1().children(
                            seriler.into_iter().map(|(indeks, etiket, görünür)| {
                                let ad = if etiket.is_empty() {
                                    format!("Seri {}", indeks + 1)
                                } else {
                                    etiket
                                };
                                Dugme::yeni(
                                    format!("stacked-{}-{indeks}", örnek.kimlik()),
                                    format!("{} {ad}", if görünür { "✓" } else { "○" }),
                                )
                                .boyutu(DugmeBoyutu::Kucuk)
                                .turu(DugmeTuru::Ikincil)
                                .tiklaninca(cx.listener(
                                    move |bu, _, _, cx| {
                                        bu.stacked_seriyi_değiştir(örnek, indeks, cx);
                                    },
                                ))
                            }),
                        ))
                }),
            )
        } else if matches!(aktif_kart, KartKimliği::StreamData(_)) {
            çizim_tabanı
                .overflow_scroll()
                .p_2()
                .children(StreamDataÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    let grafik = self
                        .stream_data_grafikleri
                        .iter()
                        .find(|(kimlik, _)| *kimlik == örnek)
                        .map(|(_, grafik)| grafik.clone());
                    div()
                        .mb_4()
                        .w(px(1_600.0))
                        .h(px(600.0))
                        .border_1()
                        .border_color(rgb(0xc0c0c0))
                        .when_some(grafik, |öğe, grafik| öğe.child(önbellekli_grafik(grafik)))
                }))
        } else if matches!(aktif_kart, KartKimliği::ThinBars(_)) {
            // 55 yüzeyin tamamı her render'da eleman ağacına giriyordu; yüzey
            // başına ~13,7 µs ile kart tek başına kareyi ikiye katlıyordu.
            // Dikey sıra korunarak sanallaştırılır: 0 = açıklama, 1 = yoğunluk
            // bloğu, sonrası dörderli geometri satırları. `uniform_list` eşit
            // yükseklik ister, buradaki öğeler farklı yükseklikte; `list` bunu
            // ölçerek yönetir.
            let yüzeyler = self.thin_bars_grafikleri.clone();
            let örnekler = ThinBarsÖrneği::tümü();
            let yoğunluk_sayısı = örnekler.len().min(7);
            let satır_sayısı = örnekler.len().saturating_sub(yoğunluk_sayısı).div_ceil(4);
            let durum = self.thin_bars_liste_durumu.get_or_insert_with(|| {
                ListState::new(satır_sayısı + 2, ListAlignment::Top, px(600.0))
            });
            çizim_tabanı.min_h_0().p_2().child(
                list(durum.clone(), move |indeks, _pencere, _cx| {
                    let yüzey = |örnek: ThinBarsÖrneği| {
                        yüzeyler
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .map(|(_, grafik)| grafik.clone())
                    };
                    match indeks {
                        0 => div()
                            .w(px(1600.0))
                            .p_2()
                            .mb_2()
                            .rounded_md()
                            .bg(rgb(0xf8fafc))
                            .text_xs()
                            .text_color(rgb(0x6b7280))
                            .child("Resmî thin-bars-stroke-fill.html sayfası 7 yoğunluk yüzeyini ve 12 align/width/gap grubundaki 48 geometri yüzeyini birlikte gösterir. Yüzeyler veri veya cursor paylaşmaz; her biri bağımsız zoom geçmişi tutar. Noktalar görünür X aralığındaki piksel açıklığı yeterli olduğunda otomatik açılır.")
                            .into_any_element(),
                        1 => div()
                            .w(px(1600.0))
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .children(örnekler.iter().copied().take(yoğunluk_sayısı).map(
                                |örnek| {
                                    let (ham_genişlik, ham_yükseklik) = örnek.boyut();
                            let (genişlik, yükseklik) = görünür_alan
                                .pay_düş(AÇIKLAMA_PAYI)
                                .yüzey(ham_genişlik as f32, ham_yükseklik as f32);
                                    div()
                                        .flex_none()
                                        .w(px(genişlik))
                                        .h(px(yükseklik))
                                        .border_1()
                                        .border_color(rgb(0xe5e7eb))
                                        .when_some(yüzey(örnek), |öğe, grafik| {
                                            öğe.child(önbellekli_grafik(grafik))
                                        })
                                },
                            ))
                            .into_any_element(),
                        _ => {
                            let başlangıç = yoğunluk_sayısı + (indeks - 2) * 4;
                            let grup = örnekler
                                .get(başlangıç..(başlangıç + 4).min(örnekler.len()))
                                .unwrap_or(&[]);
                            div()
                                .w(px(1600.0))
                                .flex()
                                .border_t_1()
                                .border_color(rgb(0xd1d5db))
                                .pt_2()
                                .children(grup.iter().copied().map(|örnek| {
                                    div()
                                        .flex_none()
                                        .w(px(400.0))
                                        .h(px(200.0))
                                        .border_1()
                                        .border_color(rgb(0xe5e7eb))
                                        .when_some(yüzey(örnek), |öğe, grafik| {
                                            öğe.child(önbellekli_grafik(grafik))
                                        })
                                }))
                                .into_any_element()
                        }
                    }
                })
                .h_full(),
            )
        } else if matches!(aktif_kart, KartKimliği::TimePeriods(_)) {
            çizim_tabanı
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .w(px(1920.0))
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Resmî time-periods.html sayfasındaki üç Grafik aynı traffic.json verisinden türetilir fakat cursor, seçim, zoom ve görünüm geçmişi paylaşmaz. Hourly seri bazlı geçmiş-yıl lejant tarihleri, Feb vs Jan türetilmiş ikinci X ekseni, Daily ortak UTC günü kullanır."),
                )
                .children(TimePeriodsÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    let grafik = self
                        .time_periods_grafikleri
                        .iter()
                        .find(|(kimlik, _)| *kimlik == örnek)
                        .map(|(_, grafik)| grafik.clone());
                    div()
                        .mb_4()
                        .w(px(1920.0))
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(örnek.başlık()),
                        )
                        .child(
                            div()
                                .w(px(1920.0))
                                .h(px(200.0))
                                .border_1()
                                .border_color(rgb(0xe5e7eb))
                                .when_some(grafik, |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                        )
                }))
        } else if matches!(aktif_kart, KartKimliği::TimelineDiscrete(_)) {
            çizim_tabanı
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .w(px(1920.0))
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Resmî timeline-discrete.html sayfasındaki dört Grafik ayrı plugin/hover ve görünüm durumları taşır. İlk yüzey semantic süreleri, ikincisi sabit örnek matrisini, son ikisi yinelenen ve birleştirilmiş durumları karşılaştırır."),
                )
                .children(TimelineDiscreteÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    let grafik = self
                        .timeline_discrete_grafikleri
                        .iter()
                        .find(|(kimlik, _)| *kimlik == örnek)
                        .map(|(_, grafik)| grafik.clone());
                    let görünürlük = grafik
                        .as_ref()
                        .map(|grafik| {
                            (0..3)
                                .map(|indeks| grafik.read(cx).grafik().seri_görünür_mü(indeks))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    div()
                        .mb_4()
                        .w(px(1920.0))
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(örnek.başlık()),
                        )
                        .child(
                            div()
                                .w(px(1920.0))
                                .h(px(300.0))
                                .border_1()
                                .border_color(rgb(0xe5e7eb))
                                .when_some(grafik, |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                        )
                        .child(
                            div()
                                .flex()
                                .gap_2()
                                .px_2()
                                .py_1()
                                .children(
                                    ["Device A", "Device B", "Device C"]
                                        .into_iter()
                                        .enumerate()
                                        .map(|(seri_indeksi, etiket)| {
                                            let görünür = görünürlük
                                                .get(seri_indeksi)
                                                .copied()
                                                .unwrap_or(false);
                                            div()
                                                .id(SharedString::from(format!(
                                                    "timeline-toggle-{}-{seri_indeksi}",
                                                    örnek.kimlik()
                                                )))
                                                .px_2()
                                                .py_1()
                                                .rounded_sm()
                                                .cursor_pointer()
                                                .text_xs()
                                                .bg(if görünür {
                                                    rgb(0xe2e8f0)
                                                } else {
                                                    rgb(0xf8fafc)
                                                })
                                                .text_color(if görünür {
                                                    rgb(0x111827)
                                                } else {
                                                    rgb(0x94a3b8)
                                                })
                                                .child(etiket)
                                                .on_click(cx.listener(
                                                    move |bu, _: &ClickEvent, _, cx| {
                                                        bu.timeline_serisini_değiştir(
                                                            örnek,
                                                            seri_indeksi,
                                                            cx,
                                                        );
                                                    },
                                                ))
                                        }),
                                ),
                        )
                }))
        } else if aktif_kart == KartKimliği::UpdateCursorSelectResize {
            let ham_boyut = self
                .boyut_senkron_akışı
                .map_or(800, BoyutSenkronAkışı::boyut);
            // Kaynak yüzey karedir; setSize canlı olarak boyutu değiştirdiğinden
            // sığdırma her karede güncel ham değerden hesaplanır.
            let (genişlik, yükseklik) = görünür_alan.yüzey(ham_boyut as f32, ham_boyut as f32);
            çizim_tabanı.overflow_scroll().child(
                div()
                    .w(px(genişlik))
                    .h(px(yükseklik))
                    .when_some(self.grafik.clone(), |öğe, grafik| {
                        öğe.child(önbellekli_grafik(grafik))
                    }),
            )
        } else if aktif_kart == KartKimliği::ScrollSync {
            çizim_tabanı
                .overflow_y_scroll()
                .child(
                div()
                    .w(px(400.0))
                    .p_3()
                    .text_sm()
                    .text_color(soluk)
                    .child("Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock traced consectetur through the classical passages in sections 1.10.32 and 1.10.33 of de Finibus Bonorum et Malorum by Cicero.")
                    .child(div().mt_3().child("The source page deliberately places the chart after two long paragraphs. Scrolling this 400×400 container changes the 400×200 chart's window rectangle before pointer interaction. GPUI refreshes layout bounds while the shared Rust core performs the client-to-scene conversion."))
                    .child(
                        div()
                            .my_3()
                            .w(px(görünür_alan.pay_düş(AÇIKLAMA_PAYI).yüzey(400.0, 200.0).0))
                                .h(px(görünür_alan.pay_düş(AÇIKLAMA_PAYI).yüzey(400.0, 200.0).1))
                            .when_some(self.grafik.clone(), |öğe, grafik| öğe.child(önbellekli_grafik(grafik))),
                    )
                    .child("Grafiği kaydırdıktan sonra imleç ve seçim aynı görsel noktada kalır. Kaynak parity için doğal kapsayıcı kaydırması varsayılandır; wheel/touch yakınlaştırma ortak API'den isteğe bağlı açılır."),
                )
        } else if let KartKimliği::MultiBars(örnek) = aktif_kart {
            let (genişlik, yükseklik) = match örnek {
                MultiBarsÖrneği::KitaplıklarDikey => (2_300.0, 800.0),
                MultiBarsÖrneği::KitaplıklarYatay => (800.0, 2_300.0),
                MultiBarsÖrneği::DeğişkenRenkler | MultiBarsÖrneği::HizasızÇubuklar => {
                    (800.0, 400.0)
                }
            };
            let grafik = self.grafik.clone();
            çizim_tabanı
                .overflow_hidden()
                .child(uyarlanan_alan(OTOMATİK_UYARLA, move |alan| {
                    let (genişlik, yükseklik) = alan.yüzey(genişlik, yükseklik);
                    div()
                        .id("multi-bars-kaydirma")
                        .size_full()
                        .overflow_scroll()
                        .child(
                            div()
                                .w(px(genişlik))
                                .h(px(yükseklik))
                                .when_some(grafik, |öğe, grafik| {
                                    öğe.child(önbellekli_grafik(grafik))
                                }),
                        )
                }))
        } else {
            çizim_tabanı
                .overflow_hidden()
                .when_some(self.grafik.clone(), |öğe, grafik| {
                    öğe.child(önbellekli_grafik(grafik))
                })
        };

        let yardım = match aktif_kart {
            KartKimliği::AddDelSeries => {
                "Add Series: sidx=2'ye yeni renkli seri · Del Series: aynı indeksi kaldır · yüzey kimliği korunur"
            }
            KartKimliği::CursorBind => {
                "Tıkla: click! iletimi · sürükle: yakınlaştır · Ctrl+sürükle: kenarlıksız sarı seçim + Annotation Text"
            }
            KartKimliği::ScrollSync => {
                "Kutuyu kaydır · grafik üzerinde imleç ve seçim konumu kaydırmadan sonra doğru kalır"
            }
            KartKimliği::SyncCursor => {
                "İmleci beş yüzeyde gezdir · tıkla: cursor kilidi · anahtarlar ilk grubun pub/sub ve mousedown/up filtresini değiştirir"
            }
            KartKimliği::TimeseriesDiscrete => {
                "İki yüzey aynı X imlecini paylaşır · üst float seri ve alttaki DEV1/DEV2/DEV3 değerleri tek lejantta birleşir"
            }
            KartKimliği::PathGapClip => {
                "15 yüzey bağımsız yakınlaşır · dört canlı yüzey aynı saniyede gerçek null boşluklarını bağlar/ayırır"
            }
            KartKimliği::PixelAlign => {
                "İki yüzey aynı veri/saati paylaşır · tam piksel tırtıl hareketini alt-piksel akışla karşılaştır"
            }
            KartKimliği::Points => {
                "Dört yüzey bağımsız etkileşir · ortadaki A/B çifti aynı veride X yoğunluk eşiğini karşılaştırır"
            }
            KartKimliği::ScalesDirOri => {
                "İki kaynak grubunda 16 yüzeyi birlikte karşılaştır · yön, yönelim ve eksen taraflarını aynı veride izle"
            }
            KartKimliği::LinePaths => {
                "Sekiz yüzey aynı veri zamanını ve cursor'ı paylaşır · seçim, wheel ve görünüm geçmişi etkin yüzeyde bağımsızdır"
            }
            KartKimliği::LogScales => {
                "Log10 ve linear yüzey aynı veriyi paylaşır · cursor, seçim, wheel ve görünüm geçmişi yüzey başına bağımsızdır"
            }
            KartKimliği::LogScales2 => {
                "12 yüzey kaynak sırasındadır · In/Out cursor ve X görünümü birlikte; diğer yüzeyler bağımsızdır"
            }
            _ => {
                "Sürükle: seç · boşluk + sürükle: taşı · kıstır: X/Y yakınlaştır · çift tıkla: tam görünüm"
            }
        };
        let açıklama_istendi = self.açıklama_istendi;
        let açıklama_metni = self.açıklama_metni.clone();
        let cursor_bind_tıklama_sayısı = self.cursor_bind_tıklama_sayısı;
        let no_data_seçenekleri = div()
            .id("no-data-secenekleri")
            .mb_3()
            .max_h(px(150.0))
            .overflow_y_scroll()
            .flex()
            .flex_wrap()
            .gap_1()
            .children(NoDataÖrneği::TÜMÜ.into_iter().map(|örnek| {
                Dugme::yeni(
                    SharedString::from(format!("no-data-secenek-{}", örnek.kimlik())),
                    örnek.başlık(),
                )
                .boyutu(DugmeBoyutu::Kucuk)
                .turu(if self.no_data_örneği == örnek {
                    DugmeTuru::Birincil
                } else {
                    DugmeTuru::Hayalet
                })
                .tiklaninca(cx.listener(move |bu, _, _, cx| {
                    bu.no_data_örneğini_seç(örnek, cx);
                }))
            }));
        // Lejant yüzeyle aynı kapsayıcıda durur: alt/üst konumda yüzeyin
        // altına veya üstüne, sol/sağ konumda yanına geçer. Kapsayıcı kalan
        // dikey alanı aldığından yüzey ölçeri lejantın payını düşülmüş
        // alanda ölçer, ayrı bir pay hesabı gerekmez.
        let lejant_bloğu = div()
            .flex_none()
            .when(lejant_konumu.dikey_mi(), |öğe| {
                öğe.w(px(LEJANT_YAN_GENİŞLİĞİ)).overflow_hidden()
            })
            .map(|öğe| match lejant_konumu {
                LejantKonumu::Alt => öğe.mt_2(),
                LejantKonumu::Üst => öğe.mb_2(),
                LejantKonumu::Sol => öğe.mr_3(),
                LejantKonumu::Sağ => öğe.ml_3(),
            })
            .child(lejant);
        let yüzey_bloğu = div().flex().flex_1().min_h_0().min_w_0().map(|öğe| {
            if lejant_konumu.dikey_mi() {
                öğe.flex_row()
            } else {
                öğe.flex_col()
            }
        });
        let yüzey_bloğu = if !lejant_görünür {
            yüzey_bloğu.child(çizim)
        } else if lejant_konumu.yüzeyden_önce_mi() {
            yüzey_bloğu.child(lejant_bloğu).child(çizim)
        } else {
            yüzey_bloğu.child(çizim).child(lejant_bloğu)
        };
        let kullanım_rehberi = aktif_kart_tanımı.açıklama;
        let kullanım_rehberi_açık = self.kullanım_rehberi_açık;
        let ayrıntı = div()
            .id("kart-ayrinti-kaydirma")
            .flex_1()
            .min_w_0()
            .min_h_0()
            .h_full()
            .overflow_scroll()
            .when(
                aktif_kart == KartKimliği::LatencyHeatmap
                    || matches!(aktif_kart, KartKimliği::MultiBars(_)),
                |öğe| öğe.overflow_hidden(),
            )
            .p_4()
            .flex()
            .flex_col()
            .child(
                div()
                    .mb_3()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .text_color(metin)
                            .child(aktif_kart_tanımı.başlık),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(soluk)
                            .child(aktif_kart_tanımı.kaynak),
                    ),
            )
            .child(araçlar)
            .when(aktif_kart == KartKimliği::NoData, |öğe| {
                öğe
                    .child(
                        div()
                            .mb_1()
                            .text_xs()
                            .text_color(soluk)
                            .child("No Data kaynağı · 33 erişilebilir seçenek"),
                    )
                    .child(no_data_seçenekleri)
            })
            .child(div().mb_2().text_xs().text_color(soluk).child(yardım))
            .when_some(kullanım_rehberi, |öğe, rehber| {
                öğe.child(
                    div()
                        .mb_2()
                        .rounded_lg()
                        .border_1()
                        .border_color(rgb(0xd1d5db))
                        .bg(rgb(0xf8fafc))
                        .child(
                            Dugme::yeni(
                                "kullanim-rehberi-toggle",
                                if kullanım_rehberi_açık {
                                    "− Açıklama · kullanım ve kaynak maliyeti"
                                } else {
                                    "+ Açıklama · kullanım ve kaynak maliyeti"
                                },
                            )
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Hayalet)
                            .tiklaninca(cx.listener(|bu, _, _, cx| {
                                bu.kullanım_rehberi_açık = !bu.kullanım_rehberi_açık;
                                izleme::olay(
                                    "PANEL",
                                    &format!(
                                        "kullanım rehberi {} · {}",
                                        if bu.kullanım_rehberi_açık {
                                            "açıldı"
                                        } else {
                                            "kapandı"
                                        },
                                        bu.aktif_kart.slug()
                                    ),
                                );
                                cx.notify();
                            })),
                        )
                        .when(kullanım_rehberi_açık, |öğe| {
                            öğe.child(
                                div()
                                    .px_2()
                                    .pb_2()
                                    .text_xs()
                                    .text_color(soluk)
                                    .child(rehber),
                            )
                        }),
                )
            })
            .when(aktif_kart == KartKimliği::CursorBind, |öğe| {
                öğe.child(
                    div()
                        .mb_2()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_sm()
                        .text_color(rgb(0x475569))
                        .child(format!(
                            "cursor.bind click! iletimi · {cursor_bind_tıklama_sayısı} tıklama"
                        )),
                )
            })
            .when_some(çizim_hatası, |öğe, hata| {
                öğe.child(
                    div()
                        .mb_2()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xfef2f2))
                        .text_sm()
                        .text_color(rgb(0xb91c1c))
                        .child(hata),
                )
            })
            .child(yüzey_bloğu)
            .child(
                div()
                    .flex_none()
                    .mt_3()
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(0xd1d5db))
                    .bg(rgb(0x111827))
                    .child(
                        Dugme::yeni("kart-tanimi-toggle", kart_tanımı_etiketi)
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Hayalet)
                            .tiklaninca(cx.listener(|bu, _, _, cx| {
                                bu.kart_tanımı_açık = !bu.kart_tanımı_açık;
                                izleme::olay(
                                    "PANEL",
                                    &format!(
                                        "kart tanımı {} · {}",
                                        if bu.kart_tanımı_açık {
                                            "açıldı"
                                        } else {
                                            "kapandı"
                                        },
                                        bu.aktif_kart.slug()
                                    ),
                                );
                                cx.notify();
                            })),
                    )
                    .when(kart_tanımı_açık, |öğe| {
                        öğe.child(
                            div()
                                .px_3()
                                .pb_3()
                                .text_xs()
                                .font_family("SF Mono")
                                .text_color(rgb(0xe5e7eb))
                                .child(aktif_kart_tanımı.tanım),
                        )
                    }),
            );

        let içerik = div()
            .size_full()
            .relative()
            .flex()
            .flex_row()
            .bg(zemin)
            // Kök dinleyiciler kabarma evresinde çalışır ve yayılımı kesmez;
            // iç kaplar olayları normal şekilde görmeye devam eder.
            .on_scroll_wheel(cx.listener(
                |bu, olay: &ScrollWheelEvent, window: &mut Window, _cx| {
                    let delta = olay.delta.pixel_delta(window.line_height());
                    izleme::kaydırma(
                        f32::from(delta.y),
                        f32::from(delta.x),
                        f32::from(olay.position.x),
                        bu.aktif_kart.slug(),
                    );
                },
            ))
            .on_mouse_move(cx.listener(|bu, olay: &MouseMoveEvent, _, _| {
                izleme::fare_hareketi(
                    f32::from(olay.position.x),
                    f32::from(olay.position.y),
                    bu.aktif_kart.slug(),
                );
            }))
            .on_any_mouse_down(cx.listener(|bu, olay: &MouseDownEvent, _, _| {
                izleme::fare_düğmesi(
                    true,
                    &format!("{:?}", olay.button),
                    f32::from(olay.position.x),
                    bu.aktif_kart.slug(),
                );
            }))
            .on_mouse_up(
                gpui::MouseButton::Left,
                cx.listener(|bu, olay: &MouseUpEvent, _, _| {
                    izleme::fare_düğmesi(
                        false,
                        &format!("{:?}", olay.button),
                        f32::from(olay.position.x),
                        bu.aktif_kart.slug(),
                    );
                }),
            )
            .child(liste)
            .child(ayrıntı)
            .when(açıklama_istendi, |kök| {
                kök.child(
                    div()
                        .id("cursor-bind-annotation-overlay")
                        .absolute()
                        .inset_0()
                        .occlude()
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(rgba(0x00000080))
                        .child(
                            div()
                                .id("cursor-bind-annotation-dialog")
                                .w(px(380.0))
                                .p_4()
                                .rounded_lg()
                                .border_1()
                                .border_color(rgb(0xd1d5db))
                                .shadow_lg()
                                .bg(rgb(0xffffff))
                                .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                                    cx.stop_propagation();
                                })
                                .child(
                                    div()
                                        .mb_3()
                                        .text_lg()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(0x111827))
                                        .child("Annotation Text"),
                                )
                                .child(div().mb_3().child(açıklama_metni.clone()))
                                .child(
                                    div()
                                        .flex()
                                        .justify_end()
                                        .gap_2()
                                        .child(
                                            Dugme::yeni("cursor-bind-annotation-cancel", "İptal")
                                                .boyutu(DugmeBoyutu::Kucuk)
                                                .turu(DugmeTuru::Ikincil)
                                                .tiklaninca(cx.listener(|bu, _, _, cx| {
                                                    bu.açıklama_istemini_kapat(cx);
                                                })),
                                        )
                                        .child(
                                            Dugme::yeni("cursor-bind-annotation-ok", "Tamam")
                                                .boyutu(DugmeBoyutu::Kucuk)
                                                .turu(DugmeTuru::Birincil)
                                                .tiklaninca(cx.listener(|bu, _, _, cx| {
                                                    bu.açıklama_istemini_kapat(cx);
                                                })),
                                        ),
                                ),
                        ),
                )
            });

        PlatformPencere::yeni("uplot-rs-pencere", "uPlot.rs Grafik Kataloğu", içerik)
            .ayarlar(CubukAyarlari::default().kompakt(true))
            .sag(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .text_xs()
                    .child(
                        div()
                            .text_color(if yazılım_gpu { vurgu } else { soluk })
                            .child(gpu_yazısı),
                    )
                    .child(div().text_color(soluk).child(kare_ölçüm_yazısı))
                    .child(div().text_color(soluk).child("Rust 2024 · MSRV 1.95")),
            )
    }
}

fn katalog_kartı(
    kimlik: impl Into<SharedString>,
    başlık: impl Into<SharedString>,
    alt_kimlik: impl Into<SharedString>,
    aktif: bool,
    durum: impl Into<SharedString>,
    panel: gpui::Rgba,
    vurgu: gpui::Rgba,
) -> gpui::Stateful<gpui::Div> {
    let kimlik = kimlik.into();
    let başlık = başlık.into();
    let alt_kimlik = alt_kimlik.into();
    let durum = durum.into();
    div()
        .id(kimlik)
        .cursor_pointer()
        // `uniform_list` bütün satırlara tek yükseklik uygular. 96 px yalnız
        // tek satırlık başlığa yetiyor, iki satıra saran başlıklarda kaynak
        // satırı alttan kırpılıyordu; yükseklik iki satırlık başlığa göre.
        .h(px(118.0))
        .mb_2()
        .overflow_hidden()
        .p_3()
        .rounded_lg()
        .border_1()
        .border_color(if aktif { vurgu } else { rgb(0xd1d5db) })
        .bg(if aktif { rgb(0xfef2f2) } else { panel })
        .child(
            div()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(0x111827))
                .child(başlık),
        )
        .child(
            div()
                .mt_1()
                .text_xs()
                .text_color(rgb(LİSTE_İKİNCİL_RENGİ))
                .child(alt_kimlik),
        )
        .child(
            div()
                .mt_2()
                .text_xs()
                .text_color(rgb(LİSTE_KAYNAK_RENGİ))
                .child(durum),
        )
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use uplot_rs::diagnostics::Komut;
    use uplot_rs::{Aralık, Nokta, TekerlekEkseni, XÖlçekDağılımı, YÖlçekDağılımı};

    fn varsayılan_fabrika_girdisi(kart: KartKimliği) -> KatalogFabrikaGirdisi {
        KatalogFabrikaGirdisi {
            kart,
            no_data_örneği: NoDataÖrneği::BOŞ_ÖZEL_ARALIK,
            nokta_sayısı: 100,
            autosize_kuvvet: 0,
            latency_kova: 5,
            latency_ofset: 0,
            pixel_align_adımı: 140,
        }
    }

    fn aralık_geçerli(aralık: Aralık) -> bool {
        aralık.en_az.is_finite() && aralık.en_çok.is_finite() && aralık.en_az < aralık.en_çok
    }

    fn komutlar_sonlu(komutlar: &[Komut]) -> bool {
        fn nokta_sonlu(nokta: &Nokta) -> bool {
            nokta.x.is_finite() && nokta.y.is_finite()
        }

        fn noktalar_sonlu(parçalar: &[Vec<Nokta>]) -> bool {
            parçalar.iter().flatten().all(nokta_sonlu)
        }

        komutlar.iter().all(|komut| match komut {
            Komut::ArkaPlan { .. } => true,
            Komut::Çizgi {
                başlangıç,
                bitiş,
                kalınlık,
                ..
            } => nokta_sonlu(başlangıç) && nokta_sonlu(bitiş) && kalınlık.is_finite(),
            Komut::KesikliÇizgi {
                başlangıç,
                bitiş,
                kalınlık,
                kesik,
                ..
            } => {
                nokta_sonlu(başlangıç)
                    && nokta_sonlu(bitiş)
                    && kalınlık.is_finite()
                    && kesik.is_finite()
            }
            Komut::Yol {
                parçalar, kalınlık,
            ..
            }
            | Komut::GradyanYol {
                parçalar, kalınlık,
            ..
            } => noktalar_sonlu(parçalar) && kalınlık.is_finite(),
            Komut::KesikliYol {
                parçalar,
                kalınlık,
                çizgi,
                boşluk,
                ..
            } => {
                noktalar_sonlu(parçalar)
                    && kalınlık.is_finite()
                    && çizgi.is_finite()
                    && boşluk.is_finite()
            }
            Komut::Alan { çokgenler, .. } | Komut::GradyanAlan { çokgenler, .. } => {
                noktalar_sonlu(çokgenler)
            }
            Komut::Daire {
                merkez,
                yarıçap,
                kalınlık,
                ..
            } => nokta_sonlu(merkez) && yarıçap.is_finite() && kalınlık.is_finite(),
            Komut::Daireler {
                merkezler,
                yarıçap,
                kalınlık,
                kesme_sınırları,
                ..
            } => {
                merkezler.iter().all(nokta_sonlu)
                    && yarıçap.is_finite()
                    && kalınlık.is_finite()
                    && kesme_sınırları.as_ref().is_none_or(|(başlangıç, bitiş)| {
                        nokta_sonlu(başlangıç) && nokta_sonlu(bitiş)
                    })
            }
            Komut::DeğişkenDaireler {
                daireler,
                kalınlık,
                kesme_sınırları,
                ..
            } => {
                daireler
                    .iter()
                    .all(|(nokta, yarıçap)| nokta_sonlu(nokta) && yarıçap.is_finite())
                    && kalınlık.is_finite()
                    && kesme_sınırları.as_ref().is_none_or(|(başlangıç, bitiş)| {
                        nokta_sonlu(başlangıç) && nokta_sonlu(bitiş)
                    })
            }
            Komut::Dikdörtgen {
                konum,
                genişlik,
                yükseklik,
                kalınlık,
                ..
            } => {
                nokta_sonlu(konum)
                    && genişlik.is_finite()
                    && yükseklik.is_finite()
                    && kalınlık.is_finite()
            }
            Komut::YuvarlatılmışDikdörtgen {
                konum,
                genişlik,
                yükseklik,
                yarıçaplar,
                kalınlık,
                ..
            } => {
                nokta_sonlu(konum)
                    && genişlik.is_finite()
                    && yükseklik.is_finite()
                    && [
                        yarıçaplar.üst_sol,
                        yarıçaplar.üst_sağ,
                        yarıçaplar.alt_sağ,
                        yarıçaplar.alt_sol,
                        *kalınlık,
                    ]
                    .into_iter()
                    .all(f32::is_finite)
            }
            Komut::Metin { konum, boyut, .. } | Komut::DöndürülmüşMetin { konum, boyut, .. } => {
                nokta_sonlu(konum) && boyut.is_finite()
            }
        })
    }

    fn sahne_sonlu(grafik: &Grafik) -> bool {
        komutlar_sonlu(grafik.çiz().komutlar())
    }

    fn x_dönüştür(değer: f64, dağılım: XÖlçekDağılımı) -> Option<f64> {
        match dağılım {
            XÖlçekDağılımı::Doğrusal => Some(değer),
            XÖlçekDağılımı::Logaritmik { taban } if değer > 0.0 && taban > 1.0 => {
                Some(değer.log(taban))
            }
            _ => None,
        }
    }

    fn y_dönüştür(değer: f64, dağılım: YÖlçekDağılımı) -> Option<f64> {
        match dağılım {
            YÖlçekDağılımı::Doğrusal => Some(değer),
            YÖlçekDağılımı::Logaritmik { taban } if değer > 0.0 && taban > 1.0 => {
                Some(değer.log(taban))
            }
            YÖlçekDağılımı::Weibull if değer > 0.0 && değer < 1.0 => {
                Some((-(-değer).ln_1p()).ln())
            }
            YÖlçekDağılımı::Özel(dönüşüm) => (dönüşüm.ileri)(değer),
            YÖlçekDağılımı::ArcSinh { eşik } if eşik > 0.0 => Some((değer / eşik).asinh()),
            _ => None,
        }
    }

    fn yakınlaştırma_oranı(
        önce: Aralık,
        sonra: Aralık,
        dönüştür: impl Fn(f64) -> Option<f64>,
    ) -> Option<f64> {
        let önce_alt = dönüştür(önce.en_az)?;
        let önce_üst = dönüştür(önce.en_çok)?;
        let sonra_alt = dönüştür(sonra.en_az)?;
        let sonra_üst = dönüştür(sonra.en_çok)?;
        Some((sonra_üst - sonra_alt) / (önce_üst - önce_alt))
    }

    fn kart_davranışını_doğrula(
        ad: &str,
        seçenekler: uplot_rs::GrafikSeçenekleri,
        veri: uplot_rs::HizalıVeri,
    ) -> Result<(), UplotHatası> {
        let x_dağılımı = seçenekler.x_dağılımı;
        let y_dağılımı = seçenekler
            .y_ölçekleri
            .iter()
            .find(|ölçek| ölçek.anahtar == seçenekler.birincil_y_ölçeği)
            .map_or(YÖlçekDağılımı::Doğrusal, |ölçek| ölçek.dağılım);
        let mut grafik = Grafik::yeni(seçenekler.clone(), veri.clone())?;
        grafik.tekerlek_etkileşimi_ayarla(true);

        let ilk_x = grafik.görünür_x_aralığı();
        let ilk_y = grafik.görünür_y_aralığı();
        assert!(aralık_geçerli(ilk_x), "{ad} geçersiz ilk X: {ilk_x:?}");
        assert!(aralık_geçerli(ilk_y), "{ad} geçersiz ilk Y: {ilk_y:?}");
        assert!(sahne_sonlu(&grafik), "{ad} ilk sahnesi sonlu değil");
        assert!(
            komutlar_sonlu(grafik.çiz_görünür_boyutta(480, 320).komutlar()),
            "{ad} küçük resize sahnesi sonlu değil"
        );
        assert!(
            komutlar_sonlu(grafik.çiz_görünür_boyutta(1_280, 720).komutlar()),
            "{ad} büyük resize sahnesi sonlu değil"
        );

        if let Some(çözüm) = grafik.imleç_çözümü(0.5, 800.0) {
            assert!(çözüm.ortak_x.is_finite(), "{ad} cursor X sonlu değil");
            assert!(
                çözüm
                    .seriler
                    .iter()
                    .flatten()
                    .all(|örnek| { örnek.x.is_finite() && örnek.değer.is_finite() }),
                "{ad} cursor seri çözümü sonlu değil"
            );
        }

        if !grafik.seri_seçenekleri().is_empty() {
            let görünür = grafik.seri_görünür_mü(0);
            grafik.seri_görünürlüğünü_ayarla(0, !görünür)?;
            assert!(sahne_sonlu(&grafik), "{ad} setSeries sonrası sonlu değil");
            grafik.seri_görünürlüğünü_ayarla(0, görünür)?;
        }

        for eksen in [TekerlekEkseni::X, TekerlekEkseni::Y, TekerlekEkseni::İkisi] {
            let mut zoom = Grafik::yeni(seçenekler.clone(), veri.clone())?;
            zoom.tekerlek_etkileşimi_ayarla(true);
            let önce_x = zoom.görünür_x_aralığı();
            let önce_y = zoom.görünür_y_aralığı();
            let değişti = zoom.tekerlek_eksende(0.5, 0.5, 10.0, true, eksen)?;
            let sonra_x = zoom.görünür_x_aralığı();
            let sonra_y = zoom.görünür_y_aralığı();

            assert!(
                aralık_geçerli(sonra_x),
                "{ad} {eksen:?} sonrası geçersiz X: {sonra_x:?}"
            );
            assert!(
                aralık_geçerli(sonra_y),
                "{ad} {eksen:?} sonrası geçersiz Y: {sonra_y:?}"
            );
            assert!(
                sahne_sonlu(&zoom),
                "{ad} {eksen:?} sonrası sahne sonlu değil"
            );

            if değişti && matches!(eksen, TekerlekEkseni::X | TekerlekEkseni::İkisi) {
                let oran = yakınlaştırma_oranı(önce_x, sonra_x, |değer| {
                    x_dönüştür(değer, x_dağılımı)
                })
                .unwrap_or(f64::NAN);
                assert!(
                    (0.90..1.0).contains(&oran),
                    "{ad} {eksen:?} X zoom oranı doğal değil: {oran}"
                );
            }
            if değişti && matches!(eksen, TekerlekEkseni::Y | TekerlekEkseni::İkisi) {
                let oran = yakınlaştırma_oranı(önce_y, sonra_y, |değer| {
                    y_dönüştür(değer, y_dağılımı)
                })
                .unwrap_or(f64::NAN);
                assert!(
                    (0.90..1.0).contains(&oran),
                    "{ad} {eksen:?} Y zoom oranı doğal değil: {oran}"
                );
            }

            let _ = zoom.tekerlek_eksende(0.5, 0.5, -10.0, true, eksen)?;
            assert!(
                aralık_geçerli(zoom.görünür_x_aralığı())
                    && aralık_geçerli(zoom.görünür_y_aralığı())
                    && sahne_sonlu(&zoom),
                "{ad} {eksen:?} ters zoom sonrası geçersiz"
            );
        }

        let mut seçim = Grafik::yeni(seçenekler.clone(), veri.clone())?;
        let _ = seçim.fiziksel_seçim_yakınlaştır_eksenlerde(0.2, 0.2, 0.8, 0.8, true, true)?;
        assert!(sahne_sonlu(&seçim), "{ad} seçim sonrası sonlu değil");
        if seçim.taşımayı_başlat() {
            let _ = seçim.taşı(0.05, -0.05)?;
            seçim.taşımayı_bitir();
            assert!(sahne_sonlu(&seçim), "{ad} taşıma sonrası sonlu değil");
        }

        let mut dokunma = Grafik::yeni(seçenekler, veri)?;
        if dokunma.dokunmayı_başlat() {
            let _ = dokunma.dokunma_yakınlaştır(0.5, 0.5, 1.05)?;
            dokunma.dokunmayı_bitir();
            assert!(
                aralık_geçerli(dokunma.görünür_x_aralığı())
                    && aralık_geçerli(dokunma.görünür_y_aralığı())
                    && sahne_sonlu(&dokunma),
                "{ad} dokunma zoomu sonrası geçersiz"
            );
        }
        Ok(())
    }

    #[test]
    fn yan_menü_ana_kart_sayısı_sabittir() {
        assert_eq!(KATALOG_KARTLARI.len(), 66);
    }

    fn lejant_serisi(etiket: &str, göster: bool) -> uplot_rs::SeriSeçenekleri {
        let mut seri = uplot_rs::SeriSeçenekleri::yeni(etiket);
        seri.göster = göster;
        seri
    }

    #[test]
    fn gizli_seri_lejantta_kalır_ve_değerleri_kaydırmaz() {
        let seriler = [
            lejant_serisi("A", true),
            lejant_serisi("B", false),
            lejant_serisi("C", true),
        ];
        let değerler = [Some(1.0), Some(2.0), Some(3.0)];
        let mut girdiler = Vec::new();
        lejant_girdilerini_ekle(&mut girdiler, &seriler, &değerler, false, false);

        assert_eq!(girdiler.len(), 3, "gizli seri listeden düşürülmemeli");
        let okunan = girdiler
            .iter()
            .map(|girdi| {
                (
                    girdi.indeks,
                    girdi.etiket.as_ref(),
                    girdi.değer.as_ref(),
                    girdi.görünür,
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            okunan,
            vec![
                (0, "A", "1.000", true),
                (1, "B", "2.000", false),
                (2, "C", "3.000", true),
            ]
        );
    }

    #[test]
    fn etiketsiz_seri_lejantta_sıra_numarası_alır() {
        let seriler = [
            lejant_serisi("", true),
            lejant_serisi("Value", true),
            lejant_serisi("  ", true),
        ];
        let mut girdiler = Vec::new();
        lejant_girdilerini_ekle(&mut girdiler, &seriler, &[None, None, None], false, false);

        let etiketler = girdiler
            .iter()
            .map(|girdi| girdi.etiket.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(etiketler, vec!["Seri 1", "Value", "Seri 3"]);
    }

    #[test]
    fn lejant_değeri_boşta_son_örneği_işaretler() {
        let seriler = [lejant_serisi("Value", true), lejant_serisi("DEV1", true)];
        let mut girdiler = Vec::new();
        lejant_girdilerini_ekle(&mut girdiler, &seriler, &[Some(4.5), None], true, true);

        assert_eq!(
            girdiler.first().map(|girdi| girdi.değer.as_ref()),
            Some("4.500 (last)")
        );
        // Değeri olmayan seri "(last)" almaz; okuyan onu son örnek sanmamalı.
        assert_eq!(
            girdiler.get(1).map(|girdi| girdi.değer.as_ref()),
            Some("--")
        );

        let mut tam_sayı = Vec::new();
        lejant_girdilerini_ekle(
            &mut tam_sayı,
            &seriler,
            &[Some(4.5), Some(2.0)],
            false,
            true,
        );
        assert_eq!(tam_sayı.get(1).map(|girdi| girdi.değer.as_ref()), Some("2"));
    }

    #[test]
    fn birleşik_lejant_indeksi_yüzey_sınırını_geçer() {
        let sayılar = [1_usize, 3];
        assert_eq!(lejant_hedefini_çöz(&sayılar, 0), Some((0, 0)));
        assert_eq!(lejant_hedefini_çöz(&sayılar, 1), Some((1, 0)));
        assert_eq!(lejant_hedefini_çöz(&sayılar, 3), Some((1, 2)));
        assert_eq!(lejant_hedefini_çöz(&sayılar, 4), None);
        assert_eq!(lejant_hedefini_çöz(&[], 0), None);
        // Boş yüzey atlanmalı, kendi numarasını tüketmemeli.
        assert_eq!(lejant_hedefini_çöz(&[0, 2], 0), Some((1, 0)));
    }

    #[test]
    fn ana_kart_slugları_benzersiz_ve_metadata_tamdır() {
        let mut sluglar = HashSet::with_capacity(KATALOG_KARTLARI.len());
        for tanım in KATALOG_KARTLARI {
            assert_eq!(tanım.kimlik.tanımlayıcı().slug, tanım.slug);
            assert!(!tanım.slug.is_empty());
            assert!(!tanım.başlık.is_empty());
            assert!(!tanım.kaynak.is_empty());
            assert!(tanım.açıklama.is_some_and(|açıklama| !açıklama.is_empty()));
            assert!(!tanım.tanım.is_empty());
            assert!(!tanım.tanım_yolu.is_empty());
            assert_eq!(KartKimliği::slugdan(tanım.slug), Some(tanım.kimlik));
            assert!(
                sluglar.insert(tanım.slug),
                "yinelenen ana kart slugı: {}",
                tanım.slug
            );
        }
    }

    #[test]
    fn tüm_ana_kartlar_sonlu_çizilir_ve_uyarlanabilir_zoom_oranını_korur() -> Result<(), UplotHatası>
    {
        for tanım in KATALOG_KARTLARI {
            let (seçenekler, veri) =
                tanım.grafiği_oluştur(varsayılan_fabrika_girdisi(tanım.kimlik))?;
            kart_davranışını_doğrula(tanım.slug, seçenekler, veri)?;
        }
        Ok(())
    }

    #[test]
    fn tüm_ilişkili_yüzeyler_ortak_davranış_sözleşmesini_korur() -> Result<(), UplotHatası> {
        let mut doğrulanan = 0_usize;
        macro_rules! grubu_doğrula {
            ($ad:literal, $kartlar:expr) => {
                for (indeks, (_, seçenekler, veri)) in $kartlar?.into_iter().enumerate() {
                    kart_davranışını_doğrula(&format!("{}/{indeks}", $ad), seçenekler, veri)?;
                    doğrulanan += 1;
                }
            };
        }

        grubu_doğrula!("align-data", align_data_kartları());
        grubu_doğrula!("custom-scales", custom_scales_kartları());
        grubu_doğrula!(
            "data-smoothing",
            uplot_rs_gpui_ornekler::data_smoothing_kartları()
        );
        grubu_doğrula!("focus-cursor", focus_cursor_kartları());
        grubu_doğrula!("gradients", gradients_kartları());
        grubu_doğrula!("high-low-bands", high_low_bands_kartları());
        grubu_doğrula!("latency-heatmap", latency_heatmap_kartları(5.0, 0.0));
        grubu_doğrula!("line-paths", line_paths_kartları());
        grubu_doğrula!("log-scales", log_scales_kartları());
        grubu_doğrula!("log-scales2", log_scales2_kartları());
        grubu_doğrula!("missing-data", missing_data_kartları());
        grubu_doğrula!("path-gap-clip", path_gap_clip_kartları());
        grubu_doğrula!("pixel-align", pixel_align_kartları(140));
        grubu_doğrula!("points", points_kartları());
        grubu_doğrula!("scales-dir-ori", scales_dir_ori_kartları());
        grubu_doğrula!("bars", bars_grouped_stacked_kartları());
        grubu_doğrula!("bars-values-autosize", bars_values_autosize_kartları());
        grubu_doğrula!("box-whisker", box_whisker_kartları());
        grubu_doğrula!("soft-minmax", soft_minmax_kartları(12.0));
        grubu_doğrula!("sparklines-bars", sparklines_bars_kartları());
        grubu_doğrula!("sparklines", sparklines_kartları());
        grubu_doğrula!("sparse", sparse_kartları());
        grubu_doğrula!("stacked-series", stacked_series_kartları());
        grubu_doğrula!("thin-bars", thin_bars_stroke_fill_kartları());
        grubu_doğrula!("time-periods", time_periods_kartları());
        grubu_doğrula!("timeline-discrete", timeline_discrete_kartları());
        grubu_doğrula!("timeseries-discrete", timeseries_discrete_kartları());
        grubu_doğrula!("timezones-dst", timezones_dst_kartları());
        grubu_doğrula!("stream-data", StreamDataGrubu::yeni()?.kartları());

        for (indeks, (seçenekler, veri)) in months_kartları()?.into_iter().enumerate() {
            kart_davranışını_doğrula(&format!("months/{indeks}"), seçenekler, veri)?;
            doğrulanan += 1;
        }
        for (ad, örnek) in NoDataÖrneği::TÜMÜ.into_iter().enumerate() {
            let (seçenekler, veri) = no_data_kartı(örnek)?;
            kart_davranışını_doğrula(&format!("no-data/{ad}"), seçenekler, veri)?;
            doğrulanan += 1;
        }
        for (indeks, örnek) in NearestNonNullÖrneği::TÜMÜ.into_iter().enumerate() {
            let (seçenekler, veri) = nearest_non_null_kartı(örnek)?;
            kart_davranışını_doğrula(&format!("nearest-non-null/{indeks}"), seçenekler, veri)?;
            doğrulanan += 1;
        }
        for (indeks, örnek) in MultiBarsÖrneği::TÜMÜ.into_iter().enumerate() {
            let (seçenekler, veri) = multi_bars_kartı(örnek)?;
            kart_davranışını_doğrula(&format!("multi-bars/{indeks}"), seçenekler, veri)?;
            doğrulanan += 1;
        }
        for (indeks, örnek) in ScatterÖrneği::TÜMÜ.into_iter().enumerate() {
            let (seçenekler, veri) = scatter_kartı(örnek)?;
            kart_davranışını_doğrula(&format!("scatter/{indeks}"), seçenekler, veri)?;
            doğrulanan += 1;
        }
        for (indeks, örnek) in SyncCursorÖrneği::TÜMÜ.into_iter().enumerate() {
            let (seçenekler, veri) = sync_cursor_kartı(örnek)?;
            kart_davranışını_doğrula(&format!("sync-cursor/{indeks}"), seçenekler, veri)?;
            doğrulanan += 1;
        }
        for (indeks, aşama) in SyncYZeroAşaması::TÜMÜ.into_iter().enumerate() {
            let (seçenekler, veri) = sync_y_zero_kartı(aşama)?;
            kart_davranışını_doğrula(&format!("sync-y-zero/{indeks}"), seçenekler, veri)?;
            doğrulanan += 1;
        }
        assert_eq!(doğrulanan, 343);
        Ok(())
    }

    #[test]
    fn multi_bars_tek_ana_kart_ve_dört_sayfa_içi_varyanttır() {
        let multi_bars_kayıtları = KATALOG_KARTLARI
            .iter()
            .filter(|tanım| matches!(tanım.kimlik, KartKimliği::MultiBars(_)))
            .count();
        assert_eq!(multi_bars_kayıtları, 1);
        assert_eq!(MultiBarsÖrneği::TÜMÜ.len(), 4);

        let tanım = KartKimliği::MultiBars(MultiBarsÖrneği::KitaplıklarDikey).tanımlayıcı();
        assert_eq!(tanım.grup, KatalogKartGrubu::İlişkiliYüzeyler);
        assert_eq!(tanım.varyant_grubu, Some("multi-bars"));
        for örnek in MultiBarsÖrneği::TÜMÜ {
            let kart = KartKimliği::MultiBars(örnek);
            assert_eq!(kart.slug(), "multi-bars");
            assert_eq!(
                kart.tanımlayıcı().kimlik,
                KartKimliği::MultiBars(MultiBarsÖrneği::KitaplıklarDikey)
            );
        }
    }

    #[test]
    fn eski_multi_bars_derin_bağlantıları_varyantı_korur() {
        for örnek in MultiBarsÖrneği::TÜMÜ {
            assert_eq!(
                KartKimliği::slugdan(örnek.kimlik()),
                Some(KartKimliği::MultiBars(örnek))
            );
        }
        assert_eq!(
            KartKimliği::slugdan("multi-bars"),
            Some(KartKimliği::MultiBars(MultiBarsÖrneği::KitaplıklarDikey))
        );
    }

    #[test]
    fn eski_canonical_aliaslar_aynı_registry_kaydına_döner() {
        for (slug, beklenen) in [
            ("align-data-cost", KartKimliği::AlignDataCost),
            ("line-resize", KartKimliği::Resize),
        ] {
            assert_eq!(KartKimliği::slugdan(slug), Some(beklenen));
            assert_eq!(beklenen.tanımlayıcı().kimlik, beklenen);
        }
    }

    #[test]
    fn multi_bars_varyantları_tek_registry_fabrikasını_kullanır() -> Result<(), UplotHatası> {
        let tanım = KartKimliği::MultiBars(MultiBarsÖrneği::KitaplıklarDikey).tanımlayıcı();
        for örnek in MultiBarsÖrneği::TÜMÜ {
            let girdi = KatalogFabrikaGirdisi {
                kart: KartKimliği::MultiBars(örnek),
                no_data_örneği: NoDataÖrneği::BOŞ_ÖZEL_ARALIK,
                nokta_sayısı: 100,
                autosize_kuvvet: 0,
                latency_kova: 5,
                latency_ofset: 0,
                pixel_align_adımı: 0,
            };
            let (registry_seçenekleri, registry_verisi) = tanım.grafiği_oluştur(girdi)?;
            let (doğrudan_seçenekler, doğrudan_veri) = multi_bars_kartı(örnek)?;
            assert_eq!(
                (
                    registry_seçenekleri.genişlik,
                    registry_seçenekleri.yükseklik
                ),
                (doğrudan_seçenekler.genişlik, doğrudan_seçenekler.yükseklik)
            );
            assert_eq!(registry_verisi.x(), doğrudan_veri.x());
            assert_eq!(registry_verisi.seriler(), doğrudan_veri.seriler());
        }
        Ok(())
    }

    /// Ctrl basılıyken imleç iki eksende de en yakın örneğe oturmalı.
    ///
    /// Yalnız X yapıştığında imleç noktası veri noktasının hizasına gelmiyor,
    /// yanından geçen bir kesişim gösteriyordu. Ctrl bırakıldığında çizgiler
    /// yeniden fareyi izler.
    #[::gpui::test]
    async fn ctrl_yapışması_imleci_örneğin_üstüne_oturtur(cx: &mut ::gpui::TestAppContext) {
        cx.update(|cx| {
            let _ = ortak_bilesenler::baslat(ortak_bileşen_ayarları(), cx);
            başlat(cx);
        });
        let (liste, cx) = cx.add_window_view(|_, cx| ChartListesi::yeni(cx));
        liste.update(cx, |bu, cx| bu.kartı_seç(KartKimliği::Resize, cx));
        cx.run_until_parked();

        let alan = liste.read_with(cx, |bu, cx| {
            bu.grafik
                .as_ref()
                .and_then(|yüzey| yüzey.read(cx).ölçülen_alan())
        });
        assert!(alan.is_some(), "yüzey ölçüm vermedi");
        let Some(alan) = alan else { return };
        let konum = liste.read_with(cx, |bu, cx| {
            bu.grafik
                .as_ref()
                .and_then(|yüzey| yüzey.read(cx).imleç_konumu())
        });
        assert!(konum.is_none(), "başlangıçta imleç olmamalı");

        // Sinüs eğrisinin dik indiği bölge: yapışma ile ham fare arasındaki
        // dikey fark burada ölçülebilir büyüklükte.
        let hedef = ::gpui::point(
            alan.left() + alan.size.width * 0.45,
            alan.top() + alan.size.height * 0.62,
        );
        let ölç = |cx: &mut ::gpui::VisualTestContext| {
            liste.read_with(cx, |bu, cx| {
                bu.grafik
                    .as_ref()
                    .and_then(|yüzey| yüzey.read(cx).imleç_konumu())
            })
        };

        cx.simulate_mouse_move(hedef, None, ::gpui::Modifiers::default());
        cx.run_until_parked();
        let serbest = ölç(cx);
        assert!(serbest.is_some(), "imleç kurulmalı");

        cx.simulate_mouse_move(
            hedef,
            None,
            ::gpui::Modifiers {
                control: true,
                ..::gpui::Modifiers::default()
            },
        );
        cx.run_until_parked();
        let yapışık = ölç(cx);
        assert!(yapışık.is_some(), "yapışık imleç kurulmalı");

        let (Some(serbest), Some(yapışık)) = (serbest, yapışık) else {
            return;
        };
        // Yapışma iki ekseni de örneğe taşır; serbest imleç ham fare
        // konumunda kalır, yani ikisi ayrışmalıdır.
        assert!(
            (serbest.y - yapışık.y).abs() > 1.0,
            "Ctrl ikinci ekseni de örneğe oturtmalı: serbest {serbest:?}, yapışık {yapışık:?}"
        );

        // Ctrl bırakılınca çizgiler yeniden fareyi izler.
        cx.simulate_mouse_move(hedef, None, ::gpui::Modifiers::default());
        cx.run_until_parked();
        assert_eq!(
            ölç(cx).map(|nokta| nokta.y),
            Some(serbest.y),
            "Ctrl bırakılınca imleç ham fare konumuna dönmeli"
        );
    }

    /// Lejant satırına gelmek uPlot gibi seriyi odaklamalı, ayrılmak bırakmalı.
    ///
    /// Odak yalnız `cursor.focus` kurulmuş kartlarda boyanır; `focus-cursor`
    /// ailesi bu davranışın kaynağıdır. Hedef dışındaki yüzeylerin odağı da
    /// bırakılır, yoksa önceki yüzeyde soluk seriler asılı kalırdı.
    #[::gpui::test]
    async fn lejant_satırına_gelmek_seriyi_odaklar(cx: &mut ::gpui::TestAppContext) {
        cx.update(|cx| {
            let _ = ortak_bilesenler::baslat(ortak_bileşen_ayarları(), cx);
            başlat(cx);
        });
        let (liste, cx) = cx.add_window_view(|_, cx| ChartListesi::yeni(cx));
        liste.update(cx, |bu, cx| bu.kartı_seç(KartKimliği::FocusCursor, cx));
        cx.run_until_parked();

        let odaklar = |cx: &mut ::gpui::VisualTestContext| {
            liste.read_with(cx, |bu, cx| {
                bu.lejant_yüzeyleri()
                    .iter()
                    .map(|yüzey| yüzey.read(cx).grafik().odak_serisi())
                    .collect::<Vec<_>>()
            })
        };
        let seri_sayısı = liste.read_with(cx, |bu, cx| {
            bu.lejant_yüzeyleri()
                .first()
                .map_or(0, |yüzey| yüzey.read(cx).grafik().seri_seçenekleri().len())
        });
        assert!(seri_sayısı >= 2, "odak kartı en az iki seri taşımalı");
        assert_eq!(odaklar(cx), vec![None], "başlangıçta odak olmamalı");

        liste.update(cx, |bu, cx| bu.lejant_serisini_odakla(Some(1), cx));
        cx.run_until_parked();
        assert_eq!(
            odaklar(cx),
            vec![Some(1)],
            "lejant satırı seriyi odaklamalı"
        );

        liste.update(cx, |bu, cx| bu.lejant_serisini_odakla(None, cx));
        cx.run_until_parked();
        assert_eq!(odaklar(cx), vec![None], "satırdan ayrılmak odağı bırakmalı");

        // Seri sınırının dışı sessizce yok sayılır; odak değişmez.
        liste.update(cx, |bu, cx| {
            bu.lejant_serisini_odakla(Some(seri_sayısı + 5), cx);
        });
        cx.run_until_parked();
        assert_eq!(odaklar(cx), vec![None], "geçersiz indeks odak kurmamalı");
    }

    /// Çok yüzeyli kartlarda lejant imlecin girdiği yüzeyi göstermeli.
    ///
    /// Lejant kartın ilk yüzeyini okuyordu; Stacked Series 16 yüzeyli ve
    /// imleç hangisinde gezerse gezsin hep aynı değerler listeleniyordu.
    /// Seçim fare ayrıldıktan sonra da korunur — aksi hâlde lejant girdisine
    /// tıklamak için fareyi yüzeyden çekmek gösterilen seriyi değiştirir ve
    /// tıklama yanlış yüzeye giderdi.
    #[::gpui::test]
    async fn lejant_imlecin_girdiği_yüzeyi_gösterir(cx: &mut ::gpui::TestAppContext) {
        cx.update(|cx| {
            let _ = ortak_bilesenler::baslat(ortak_bileşen_ayarları(), cx);
            başlat(cx);
        });
        let (liste, cx) = cx.add_window_view(|_, cx| ChartListesi::yeni(cx));
        liste.update(cx, |bu, cx| {
            bu.kartı_seç(KartKimliği::Sparklines(SparklineÖrneği::İLK), cx);
        });
        cx.run_until_parked();

        // Ölçüm veren ilk iki yüzeyi al; tabloda farklı hisselere aitler.
        let alanlar = liste.read_with(cx, |bu, cx| {
            let mut alanlar = Vec::new();
            bu.etkin_grafik_yüzeylerini_gez(|yüzey| {
                if let Some(alan) = yüzey.read(cx).ölçülen_alan() {
                    alanlar.push((yüzey.entity_id(), alan));
                }
            });
            alanlar
        });
        assert!(alanlar.len() >= 2, "tablo en az iki yüzey ölçmeli");
        let (Some(ilk), Some(ikinci)) = (alanlar.first().copied(), alanlar.get(1).copied()) else {
            return;
        };

        let izlenen =
            |cx: &mut ::gpui::VisualTestContext| liste.read_with(cx, |bu, _| bu.lejant_yüzeyi);
        cx.simulate_mouse_move(ilk.1.center(), None, ::gpui::Modifiers::default());
        cx.run_until_parked();
        assert_eq!(
            izlenen(cx),
            Some(ilk.0),
            "lejant imlecin girdiği yüzeye geçmeli"
        );

        cx.simulate_mouse_move(ikinci.1.center(), None, ::gpui::Modifiers::default());
        cx.run_until_parked();
        assert_eq!(
            izlenen(cx),
            Some(ikinci.0),
            "lejant ikinci yüzeye taşınmalı"
        );

        // Fare tablodan çıkınca seçim korunur, ilk yüzeye geri dönmez.
        cx.simulate_mouse_move(
            ::gpui::point(ikinci.1.left(), ikinci.1.bottom() + ::gpui::px(400.0)),
            None,
            ::gpui::Modifiers::default(),
        );
        cx.run_until_parked();
        assert_eq!(
            izlenen(cx),
            Some(ikinci.0),
            "fare ayrılınca lejant son gezilen yüzeyde kalmalı"
        );
    }

    /// Fare bir yüzeyi terk ettiğinde o yüzeyin imleci sönmeli.
    ///
    /// `sparklines` tablosunda 20 küçük yüzey var ve fare sürekli birinden
    /// çıkıp diğerine giriyor; terk edilen yüzeylerde kesik imleç çizgisi
    /// kalıyordu. GPUI'de `on_mouse_exit` yalnız fare **pencereyi** terk
    /// ettiğinde üretilir, `on_mouse_move` de hitbox ile filtrelidir — yüzey
    /// sınırından çıkışı ikisi de bildirmez.
    #[::gpui::test]
    async fn yüzeyi_terk_eden_fare_imleci_söndürür(cx: &mut ::gpui::TestAppContext) {
        cx.update(|cx| {
            let _ = ortak_bilesenler::baslat(ortak_bileşen_ayarları(), cx);
            başlat(cx);
        });
        let (liste, cx) = cx.add_window_view(|_, cx| ChartListesi::yeni(cx));
        liste.update(cx, |bu, cx| {
            bu.kartı_seç(KartKimliği::Sparklines(SparklineÖrneği::İLK), cx);
        });
        cx.run_until_parked();

        // İlk yüzeyin ölçülen alanının ortasına gir.
        let alan = liste.read_with(cx, |bu, cx| {
            let mut ilk = None;
            bu.etkin_grafik_yüzeylerini_gez(|yüzey| {
                if ilk.is_none() {
                    ilk = yüzey.read(cx).ölçülen_alan();
                }
            });
            ilk
        });
        assert!(alan.is_some(), "yüzey ölçüm vermedi");
        let Some(alan) = alan else { return };
        cx.simulate_mouse_move(alan.center(), None, ::gpui::Modifiers::default());
        cx.run_until_parked();

        let imleçler = |cx: &mut ::gpui::VisualTestContext| {
            liste.read_with(cx, |bu, cx| {
                let mut etkin = 0_usize;
                bu.etkin_grafik_yüzeylerini_gez(|yüzey| {
                    if yüzey.read(cx).imleç_etkin_mi() {
                        etkin += 1;
                    }
                });
                etkin
            })
        };
        assert_eq!(
            imleçler(cx),
            1,
            "fare yüzeyin üstündeyken imleç etkin olmalı"
        );

        // Kartın dışına, kaydırma alanının boş bölgesine çık.
        cx.simulate_mouse_move(
            ::gpui::point(alan.left(), alan.bottom() + ::gpui::px(400.0)),
            None,
            ::gpui::Modifiers::default(),
        );
        cx.run_until_parked();
        assert_eq!(
            imleçler(cx),
            0,
            "fare yüzeyi terk ettiğinde imleç sönmeli; kesik çizgi kalıyor"
        );
    }

    /// Aynı kartın iki grafik yüzeyi birbirinin üstüne yerleşmemeli.
    ///
    /// Bu oturumda bulunan kusurların hiçbirini sahne komutu testleri
    /// yakalamadı: komutlar doğruydu, yüzey yanlış boyutta yerleşiyordu.
    /// `sparklines` 10×2 tablosunda grafik kökünün sabit 120 px taban
    /// yüksekliği her 150×30 hücreyi 150×120 yerleştiriyordu; yüzeyler
    /// sonraki üç satırın üstüne yazıyor ve tabloda yalnız son satır
    /// görünüyordu. Çakışma, hücre boyutlarını bilmeye gerek kalmadan bu
    /// sınıf hatanın tamamını yakalar.
    ///
    /// Sanallaştırılmış kartlarda görünür alana girmemiş yüzeyler ölçüm
    /// vermez ve doğal olarak atlanır.
    #[::gpui::test]
    async fn kart_yüzeyleri_üst_üste_yerleşmez(cx: &mut ::gpui::TestAppContext) {
        /// Komşu yüzeyler kenarlarını paylaşabilir; ölçüm cihaz pikseline
        /// yuvarlandığından bir piksellik örtüşme yerleşim hatası değildir.
        const TOLERANS: f32 = 1.0;

        cx.update(|cx| {
            let _ = ortak_bilesenler::baslat(ortak_bileşen_ayarları(), cx);
            başlat(cx);
        });
        let (liste, cx) = cx.add_window_view(|_, cx| ChartListesi::yeni(cx));

        let mut ihlaller = Vec::new();
        let mut ölçülen_kart_sayısı = 0_usize;
        for tanım in KATALOG_KARTLARI {
            liste.update(cx, |bu, cx| bu.kartı_seç(tanım.kimlik, cx));
            cx.run_until_parked();
            let alanlar = liste.read_with(cx, |bu, cx| {
                let mut alanlar = Vec::new();
                bu.etkin_grafik_yüzeylerini_gez(|yüzey| {
                    if let Some(alan) = yüzey.read(cx).ölçülen_alan() {
                        alanlar.push(alan);
                    }
                });
                alanlar
            });
            if !alanlar.is_empty() {
                ölçülen_kart_sayısı += 1;
            }
            for (sıra, alan) in alanlar.iter().enumerate() {
                for diğer in alanlar.iter().skip(sıra + 1) {
                    let yatay = f32::from(alan.right().min(diğer.right()))
                        - f32::from(alan.left().max(diğer.left()));
                    let dikey = f32::from(alan.bottom().min(diğer.bottom()))
                        - f32::from(alan.top().max(diğer.top()));
                    if yatay > TOLERANS && dikey > TOLERANS {
                        ihlaller.push(format!(
                            "{}: {:?} ile {:?} {yatay}×{dikey} px örtüşüyor",
                            tanım.slug, alan, diğer
                        ));
                    }
                }
            }
        }

        // Ölçüm hiç gelmediyse test sessizce yeşile döner; o durumda
        // invaryant değil, ölçüm yolu bozuktur.
        assert!(
            ölçülen_kart_sayısı > KATALOG_KARTLARI.len() / 2,
            "kartların yarısından azı ölçüm verdi ({ölçülen_kart_sayısı}/{}); ölçüm yolu bozuk olabilir",
            KATALOG_KARTLARI.len()
        );
        assert!(
            ihlaller.is_empty(),
            "grafik yüzeyleri üst üste yerleşti:\n{}",
            ihlaller.join("\n")
        );
    }

    /// Kök render, tek yüzeyli kartlarda yaklaşık sabit bir taban maliyet
    /// (yan menü, araç çubuğu, kart tanımı) ödüyor. Çok yüzeyli kartlar bunun
    /// üstüne yüzey başına eleman kurulumu ekliyordu; ThinBars ve TimezonesDst
    /// bu yüzden sanallaştırıldı. Bu test o kazancın kaybolmasını görünür
    /// kılar: dağılımı yazdırır ve kare bütçesini aşarsa düşer.
    ///
    /// Yalnız release'de anlamlıdır; debug ölçümü bütçeyle karşılaştırılamaz.
    #[::gpui::test]
    async fn kok_render_kare_butcesi(cx: &mut ::gpui::TestAppContext) {
        use std::time::{Duration, Instant};

        if cfg!(debug_assertions) {
            return;
        }

        const ISINMA_TURU: usize = 10;
        const ÖLÇÜM_TURU: u32 = 50;
        const KARE_BÜTÇESİ: Duration = Duration::from_micros(16_700);

        cx.update(|cx| {
            let _ = ortak_bilesenler::baslat(ortak_bileşen_ayarları(), cx);
            başlat(cx);
        });
        let (liste, cx) = cx.add_window_view(|_, cx| ChartListesi::yeni(cx));

        let ölç = |ad: &str, kart: KartKimliği, cx: &mut ::gpui::VisualTestContext| {
            liste.update(cx, |bu, cx| bu.kartı_seç(kart, cx));
            cx.run_until_parked();
            let yüzey_sayısı = liste.read_with(cx, |bu, _| bu.etkin_grafik_yüzeyleri().len());

            for _ in 0..ISINMA_TURU {
                liste.update(cx, |_, cx| cx.notify());
                cx.run_until_parked();
            }

            let mut süreler = Vec::with_capacity(ÖLÇÜM_TURU as usize);
            for _ in 0..ÖLÇÜM_TURU {
                let başlangıç = Instant::now();
                liste.update(cx, |_, cx| cx.notify());
                cx.run_until_parked();
                süreler.push(başlangıç.elapsed());
            }
            süreler.sort_unstable();
            let yüzdelik = |yüzde: usize| {
                let son = süreler.len().saturating_sub(1);
                süreler
                    .get(son.saturating_mul(yüzde).div_ceil(100))
                    .copied()
                    .unwrap_or_default()
            };
            let (p50, p95) = (yüzdelik(50), yüzdelik(95));
            eprintln!("{ad}: {yüzey_sayısı} yüzey · p50 {p50:?} · p95 {p95:?}");
            assert!(
                p50 <= KARE_BÜTÇESİ / 2,
                "{ad} kök render p50 {p50:?}, bütçe {:?}",
                KARE_BÜTÇESİ / 2
            );
            assert!(
                p95 <= KARE_BÜTÇESİ,
                "{ad} kök render p95 {p95:?}, bütçe {KARE_BÜTÇESİ:?}"
            );
        };

        ölç("Resize (tek yüzey)", KartKimliği::Resize, cx);
        if let Some(örnek) = ThinBarsÖrneği::tümü().first().copied() {
            ölç("ThinBars (55 yüzey)", KartKimliği::ThinBars(örnek), cx);
        }
        ölç("TimezonesDst (51 yüzey)", KartKimliği::TimezonesDst, cx);
        ölç("LatencyHeatmap", KartKimliği::LatencyHeatmap, cx);
        ölç("MassSpectrum", KartKimliği::MassSpectrum, cx);
        // 40.000 nokta CPU raster yoluyla tek sprite'a iniyor; ölçüm o yolun
        // gerçek payını görünür kılar.
        ölç("Scatter (raster yolu)", KartKimliği::Scatter, cx);
    }
}
