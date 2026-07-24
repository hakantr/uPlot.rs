//! uPlot'un küçük ve hızlı çizim modelini Rust'a taşıyan çekirdek.
//!
//! Çekirdek; GPUI'den bağımsız veri doğrulama, ölçekleme, etkileşim durumu,
//! çizim komutları ve SVG çıktısı sağlar. GPUI ve WASM doğrulama uygulamaları
//! yalnız platform olaylarını çekirdeğe ileten ayrı yüzey adaptörleridir.

#![cfg_attr(feature = "gpui", allow(confusable_idents))]

pub mod cizim;
mod etkilesim;
#[cfg(feature = "gpui")]
pub mod gpui;
pub mod grafik;
pub mod hata;
pub mod kart;
pub mod olcek;
pub mod secenek;
pub mod veri;
pub mod yuzey;
mod zaman;

#[cfg(feature = "svg")]
pub mod svg;

pub use cizim::{DoğrusalGradyan, GradyanRenkDurağı, Komut, MetinHizası, Nokta, Sahne};
pub use grafik::{
    DağılımVuruşu, EksenHedefi, Grafik, NullAtlamaYönü, SeçimEylemi, TimelineVuruşu,
    ZoomRangerDurumu, ZoomRangerSürüklemeEkseni,
};
pub use hata::UplotHatası;
#[cfg(feature = "svg")]
pub use kart::svg_image_belgesi;
pub use kart::{
    ADD_DEL_SERIES_KANIT_TOHUMU, ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ, ALIGN_DATA_KANIT_TOHUMU,
    ALIGN_DATA_KART_TANIM_ÖRNEĞİ, ANNOTATIONS_KANIT_TOHUMU, ANNOTATIONS_KART_TANIM_ÖRNEĞİ,
    ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ, AREA_FILL_KANIT_TOHUMU, AREA_FILL_KART_TANIM_ÖRNEĞİ,
    AXIS_AUTOSIZE_KANIT_TOHUMU, AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ, AXIS_CONTROL_KANIT_TOHUMU,
    AXIS_CONTROL_KART_TANIM_ÖRNEĞİ, AXIS_INDICATORS_KANIT_TOHUMU,
    AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ, BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ,
    BARS_VALUES_AUTOSIZE_KANIT_TOHUMU, BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    BOX_WHISKER_BENCHMARKLERİ, BOX_WHISKER_KART_TANIM_ÖRNEĞİ, BoyutSenkronAkışı,
    CANDLESTICK_KANIT_TOHUMU, CANDLESTICK_KART_TANIM_ÖRNEĞİ, CURSOR_BIND_KANIT_TOHUMU,
    CURSOR_BIND_KART_TANIM_ÖRNEĞİ, CURSOR_SNAP_KANIT_TOHUMU, CURSOR_SNAP_KART_TANIM_ÖRNEĞİ,
    CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ, CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ, CustomScaleÖrneği,
    DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ, DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ,
    DRAW_HOOKS_KART_TANIM_ÖRNEĞİ, FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ, FocusÖrneği,
    GRADIENTS_KART_TANIM_ÖRNEĞİ, GRID_OVER_SERIES_KANIT_TOHUMU, GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ,
    GradientÖrneği, HIGH_LOW_BANDS_KANIT_TOHUMU, HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ,
    HighLowBandsÖrneği, LATENCY_HEATMAP_KANIT_TOHUMU, LATENCY_HEATMAP_KART_TANIM_ÖRNEĞİ,
    LINE_PATHS_KART_TANIM_ÖRNEĞİ, LOG_SCALES_KART_TANIM_ÖRNEĞİ, LOG_SCALES2_KANIT_TOHUMU,
    LOG_SCALES2_KART_TANIM_ÖRNEĞİ, LatencyHeatmapÖrneği, LinePathsÖrneği, LogScales2Örneği,
    LogScalesÖrneği, MASS_SPECTRUM_KART_TANIM_ÖRNEĞİ, MEASURE_DATUMS_KART_TANIM_ÖRNEĞİ,
    MISSING_DATA_KART_TANIM_ÖRNEĞİ, MONTHS_KANIT_TOHUMU, MONTHS_KART_TANIM_ÖRNEĞİ,
    MONTHS_RU_KANIT_TOHUMU, MULTI_BARS_KART_TANIM_ÖRNEĞİ, MultiBarsÖrneği,
    NEAREST_NON_NULL_KART_TANIM_ÖRNEĞİ, NICE_SCALE_KART_TANIM_ÖRNEĞİ, NO_DATA_KART_TANIM_ÖRNEĞİ,
    NearestNonNullÖrneği, NoDataÖrneği, PATH_GAP_CLIP_KART_TANIM_ÖRNEĞİ, PIXEL_ALIGN_ARALIK_MS,
    PIXEL_ALIGN_KANIT_TOHUMU, PIXEL_ALIGN_KART_TANIM_ÖRNEĞİ, PIXEL_ALIGN_PENCERE_MS,
    POINTS_KANIT_TOHUMU, POINTS_KART_TANIM_ÖRNEĞİ, PathGapClipÖrneği, PixelAlignÖrneği,
    PointsÖrneği, RESIZE_KART_TANIM_ÖRNEĞİ, SCALE_PADDING_KART_TANIM_ÖRNEĞİ,
    SCALES_DIR_ORI_KART_TANIM_ÖRNEĞİ, SCATTER_KANIT_TOHUMU, SCATTER_KART_TANIM_ÖRNEĞİ,
    SCROLL_SYNC_KANIT_TOHUMU, SCROLL_SYNC_KART_TANIM_ÖRNEĞİ, SINE_STREAM_KANIT_TOHUMU,
    SINE_STREAM_KART_TANIM_ÖRNEĞİ, SINE_STREAM_NOKTA_SAYISI, SOFT_MINMAX_KART_TANIM_ÖRNEĞİ,
    SPARKLINES_BARS_KART_TANIM_ÖRNEĞİ, SPARKLINES_KART_TANIM_ÖRNEĞİ, SPARSE_KART_TANIM_ÖRNEĞİ,
    STACKED_SERIES_KANIT_TOHUMU, STACKED_SERIES_KART_TANIM_ÖRNEĞİ, STREAM_DATA_ADIMI,
    STREAM_DATA_ARALIK_MS, STREAM_DATA_KART_TANIM_ÖRNEĞİ, STREAM_DATA_PENCERESİ,
    SVG_IMAGE_KART_TANIM_ÖRNEĞİ, SYNC_CURSOR_KART_TANIM_ÖRNEĞİ, SYNC_Y_ZERO_KART_TANIM_ÖRNEĞİ,
    ScalesDirOriÖrneği, ScatterÖrneği, SineAkışı, SmoothingÖrneği, SoftMinMaxAkışı,
    SoftMinMaxÖrneği, SparklinesBarsÖrneği, SparklineÖrneği, SparseÖrneği, StackedSeriesÖrneği,
    StreamDataAkışı, StreamDataÖrneği, SyncCursorGrubu, SyncCursorÖrneği, SyncYZeroAşaması,
    THIN_BARS_STROKE_FILL_KART_TANIM_ÖRNEĞİ, TIME_PERIODS_KART_TANIM_ÖRNEĞİ,
    TIMELINE_DISCRETE_KANIT_TOHUMU, TIMELINE_DISCRETE_KART_TANIM_ÖRNEĞİ,
    TIMELINE_DISCRETE_ZAMAN_ÇAPASI, TIMESERIES_DISCRETE_KANIT_TOHUMU,
    TIMESERIES_DISCRETE_KART_TANIM_ÖRNEĞİ, TIMEZONES_DST_KART_TANIM_ÖRNEĞİ,
    TOOLTIPS_CLOSEST_KART_TANIM_ÖRNEĞİ, TOOLTIPS_KART_TANIM_ÖRNEĞİ, TRENDLINES_KART_TANIM_ÖRNEĞİ,
    ThinBarsYoğunluk, ThinBarsÖrneği, TimePeriodsÖrneği, TimelineDiscreteÖrneği,
    TimeseriesDiscreteGrubu, TimeseriesDiscreteÖrneği, TimezonesDstÖrneği,
    UPDATE_CURSOR_SELECT_RESIZE_ARALIK_MS, UPDATE_CURSOR_SELECT_RESIZE_KART_TANIM_ÖRNEĞİ,
    WIND_DIRECTION_KART_TANIM_ÖRNEĞİ, Y_SCALE_DRAG_KART_TANIM_ÖRNEĞİ, Y_SHIFTED_SERIES_ARALIK_MS,
    Y_SHIFTED_SERIES_KANIT_TOHUMU, Y_SHIFTED_SERIES_KART_TANIM_ÖRNEĞİ, YShiftedSeriesAkışı,
    YShiftedSeriesKipi, ZOOM_FETCH_KANIT_ÖRNEĞİ, ZOOM_RANGER_GRIPS_KANIT_ÖRNEĞİ,
    ZOOM_RANGER_XY_KANIT_ÖRNEĞİ, ZOOM_TOUCH_KART_TANIM_ÖRNEĞİ, ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ,
    ZoomFetchAkışı, add_del_series_ek_verisi, add_del_series_kartı, align_data_maliyet_kartı,
    align_data_çizgi_çubuk_kartı, annotations_kartı, arcsinh_scales_kartı, area_fill_kartı,
    asap_yumuşat, axis_autosize_kartı, axis_control_kartı, axis_indicators_kartı,
    bars_grouped_stacked_kartı, bars_values_autosize_kartı, box_whisker_kartı,
    candlestick_ohlc_kartı, cursor_bind_kartı, cursor_snap_kartı, cursor_tooltip_kartı,
    custom_scales_kartı, data_smoothing_kartı, dependent_scale_kartı, draw_hooks_kartı,
    focus_cursor_kartı, gradients_kartı, grid_over_series_kartı, hareketli_ortalama,
    high_low_bands_kartı, latency_heatmap_kartı, line_paths_kartı, log_scales_kartı,
    log_scales2_kartı, mass_spectrum_kartı, measure_datums_kartı, missing_data_null_kartı,
    missing_data_x_boşluğu_kartı, months_artık_yıllı_kartı, months_artık_yılsız_kartı,
    months_rusça_kartı, multi_bars_kartı, nearest_non_null_kartı, nice_scale_kartı, no_data_kartı,
    ortak_kart_etkileşimleri, path_gap_clip_kartı, pixel_align_kartı, points_kartı, resize_kartı,
    savitzky_golay, scale_padding_kartı, scales_dir_ori_kartı, scatter_kartı, scroll_sync_kartı,
    sine_stream_kartı, soft_minmax_kartı, sparklines_bars_kartı, sparklines_kartı, sparse_kartı,
    stacked_series_kartı, stacked_series_kartı_görünür, stream_data_kartı, svg_image_kartı,
    sync_cursor_kartı, sync_y_zero_aralıkları, sync_y_zero_kartı, thin_bars_stroke_fill_kartı,
    time_periods_kartı, timeline_discrete_kartı, timeseries_discrete_kartı, timezones_dst_kartı,
    tooltips_closest_kartı, tooltips_kartı, trendlines_kartı, update_cursor_select_resize_kartı,
    wind_direction_kartı, y_scale_drag_kartı, y_shifted_series_kartı, zoom_ranger_xy_grafiği,
    zoom_touch_kartı, zoom_wheel_kartı, ÇubukÖrneği,
};
pub use olcek::{Aralık, SayısalAralıkAyarları, SayısalAralıkParçası, YumuşakSınırKipi};
pub use secenek::{
    AçıklamaDüzeni, AçıklamaHizası, AçıklamaStili, Açıklamaİşareti, BantYönü, BoyutSenkronDüzeni,
    DağılımDüzeni, DağılımNoktası, DağılımSerisi, EnYakınTooltipBilgisi, EnYakınTooltipDüzeni,
    EtkileşimSeçenekleri, GradyanDurağı, GradyanEkseni, GradyanKonumu, GrafikSeçenekleri,
    GüzelÖlçekDüzeni, IsıHaritasıDüzeni, IsıHücresi, IsıHücresiBoyutu, KutuBıyıkDüzeni, MumDüzeni,
    NoktaFiltreKipi, NoktaKatmanı, NoktaŞekli, OdakDüzeni, OdakStili, RüzgarYönüDüzeni, SeriBandı,
    SeriSeçenekleri, SeriÇizimTürü, TarihAdları, TekerlekAyarları, TekerlekKipi, TimelineDüzeni,
    TimelineHücresi, TooltipBilgisi, TooltipDüzeni, XÖlçekDağılımı, YÖlçekDağılımı,
    YÖlçekEtiketBiçimi, YÖlçekSeçenekleri, ZamanDilimi, ZoomRangerSeçenekleri, ZoomSürüklemeKipi,
    ÇizimKancasıDüzeni, ÇizimSırası, ÇubukDüzeni, ÇubukYönü, ÖlçekGradyanı, İkincilXEksen,
};
pub use veri::{BoşlukKipi, HizalıDeğer, HizalıVeri, hizalı_verileri_birleştir};
pub use yuzey::YüzeyDikdörtgeni;
