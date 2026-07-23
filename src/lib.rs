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
pub use grafik::{DağılımVuruşu, Grafik, SeçimEylemi};
pub use hata::UplotHatası;
pub use kart::{
    ADD_DEL_SERIES_KANIT_TOHUMU, ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ, ALIGN_DATA_KANIT_TOHUMU,
    ALIGN_DATA_KART_TANIM_ÖRNEĞİ, ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ, AREA_FILL_KANIT_TOHUMU,
    AREA_FILL_KART_TANIM_ÖRNEĞİ, AXIS_AUTOSIZE_KANIT_TOHUMU, AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    AXIS_CONTROL_KANIT_TOHUMU, AXIS_CONTROL_KART_TANIM_ÖRNEĞİ, AXIS_INDICATORS_KANIT_TOHUMU,
    AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ, BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ,
    BARS_VALUES_AUTOSIZE_KANIT_TOHUMU, BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    BOX_WHISKER_BENCHMARKLERİ, BOX_WHISKER_KART_TANIM_ÖRNEĞİ, CANDLESTICK_KANIT_TOHUMU,
    CANDLESTICK_KART_TANIM_ÖRNEĞİ, CURSOR_BIND_KANIT_TOHUMU, CURSOR_BIND_KART_TANIM_ÖRNEĞİ,
    CURSOR_SNAP_KANIT_TOHUMU, CURSOR_SNAP_KART_TANIM_ÖRNEĞİ, CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ,
    CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ, CustomScaleÖrneği, DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ,
    DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ, DRAW_HOOKS_KART_TANIM_ÖRNEĞİ,
    FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ, FocusÖrneği, GRADIENTS_KART_TANIM_ÖRNEĞİ,
    GRID_OVER_SERIES_KANIT_TOHUMU, GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ, GradientÖrneği,
    HIGH_LOW_BANDS_KANIT_TOHUMU, HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ, HighLowBandsÖrneği,
    LATENCY_HEATMAP_KANIT_TOHUMU, LATENCY_HEATMAP_KART_TANIM_ÖRNEĞİ, LINE_PATHS_KART_TANIM_ÖRNEĞİ,
    LOG_SCALES_KART_TANIM_ÖRNEĞİ, LOG_SCALES2_KANIT_TOHUMU, LOG_SCALES2_KART_TANIM_ÖRNEĞİ,
    LatencyHeatmapÖrneği, LinePathsÖrneği, LogScales2Örneği, LogScalesÖrneği,
    MISSING_DATA_KART_TANIM_ÖRNEĞİ, MONTHS_KANIT_TOHUMU, MONTHS_KART_TANIM_ÖRNEĞİ,
    MONTHS_RU_KANIT_TOHUMU, NICE_SCALE_KART_TANIM_ÖRNEĞİ, NO_DATA_KART_TANIM_ÖRNEĞİ, NoDataÖrneği,
    PATH_GAP_CLIP_KART_TANIM_ÖRNEĞİ, PIXEL_ALIGN_ARALIK_MS, PIXEL_ALIGN_KANIT_TOHUMU,
    PIXEL_ALIGN_KART_TANIM_ÖRNEĞİ, PIXEL_ALIGN_PENCERE_MS, POINTS_KANIT_TOHUMU,
    POINTS_KART_TANIM_ÖRNEĞİ, PathGapClipÖrneği, PixelAlignÖrneği, PointsÖrneği,
    RESIZE_KART_TANIM_ÖRNEĞİ, SCALE_PADDING_KART_TANIM_ÖRNEĞİ, SCALES_DIR_ORI_KART_TANIM_ÖRNEĞİ,
    SCATTER_KANIT_TOHUMU, SCATTER_KART_TANIM_ÖRNEĞİ, SCROLL_SYNC_KANIT_TOHUMU,
    SCROLL_SYNC_KART_TANIM_ÖRNEĞİ, SINE_STREAM_KANIT_TOHUMU, SINE_STREAM_KART_TANIM_ÖRNEĞİ,
    SINE_STREAM_NOKTA_SAYISI, SOFT_MINMAX_KART_TANIM_ÖRNEĞİ, SPARKLINES_BARS_KART_TANIM_ÖRNEĞİ,
    ScalesDirOriÖrneği, ScatterÖrneği, SineAkışı, SmoothingÖrneği, SoftMinMaxAkışı,
    SoftMinMaxÖrneği, SparklinesBarsÖrneği, ZOOM_TOUCH_KART_TANIM_ÖRNEĞİ,
    ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ, add_del_series_ek_verisi, add_del_series_kartı,
    align_data_maliyet_kartı, align_data_çizgi_çubuk_kartı, arcsinh_scales_kartı, area_fill_kartı,
    asap_yumuşat, axis_autosize_kartı, axis_control_kartı, axis_indicators_kartı,
    bars_grouped_stacked_kartı, bars_values_autosize_kartı, box_whisker_kartı,
    candlestick_ohlc_kartı, cursor_bind_kartı, cursor_snap_kartı, cursor_tooltip_kartı,
    custom_scales_kartı, data_smoothing_kartı, dependent_scale_kartı, draw_hooks_kartı,
    focus_cursor_kartı, gradients_kartı, grid_over_series_kartı, hareketli_ortalama,
    high_low_bands_kartı, latency_heatmap_kartı, line_paths_kartı, log_scales_kartı,
    log_scales2_kartı, missing_data_null_kartı, missing_data_x_boşluğu_kartı,
    months_artık_yıllı_kartı, months_artık_yılsız_kartı, months_rusça_kartı, nice_scale_kartı,
    no_data_kartı, ortak_kart_etkileşimleri, path_gap_clip_kartı, pixel_align_kartı, points_kartı,
    resize_kartı, savitzky_golay, scale_padding_kartı, scales_dir_ori_kartı, scatter_kartı,
    scroll_sync_kartı, sine_stream_kartı, soft_minmax_kartı, sparklines_bars_kartı,
    zoom_touch_kartı, zoom_wheel_kartı, ÇubukÖrneği,
};
pub use olcek::{Aralık, SayısalAralıkAyarları, SayısalAralıkParçası, YumuşakSınırKipi};
pub use secenek::{
    BantYönü, DağılımDüzeni, DağılımNoktası, DağılımSerisi, EtkileşimSeçenekleri, GradyanDurağı,
    GradyanEkseni, GradyanKonumu, GrafikSeçenekleri, GüzelÖlçekDüzeni, IsıHaritasıDüzeni,
    IsıHücresi, IsıHücresiBoyutu, KutuBıyıkDüzeni, MumDüzeni, NoktaFiltreKipi, NoktaKatmanı,
    OdakDüzeni, OdakStili, SeriBandı, SeriSeçenekleri, SeriÇizimTürü, TarihAdları,
    TekerlekAyarları, TekerlekKipi, XÖlçekDağılımı, YÖlçekDağılımı, YÖlçekEtiketBiçimi,
    YÖlçekSeçenekleri, ÇizimKancasıDüzeni, ÇizimSırası, ÇubukDüzeni, ÇubukYönü, ÖlçekGradyanı,
};
pub use veri::{BoşlukKipi, HizalıDeğer, HizalıVeri, hizalı_verileri_birleştir};
pub use yuzey::YüzeyDikdörtgeni;
