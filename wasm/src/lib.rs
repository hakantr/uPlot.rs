//! Tarayıcı chart listesinin WASM köprüsü.

#![allow(confusable_idents)]

use uplot_rs::{
    ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ, ALIGN_DATA_KART_TANIM_ÖRNEĞİ, ANNOTATIONS_KART_TANIM_ÖRNEĞİ,
    ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ, AREA_FILL_KART_TANIM_ÖRNEĞİ, AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    AXIS_CONTROL_KART_TANIM_ÖRNEĞİ, AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ,
    BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ, BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    BOX_WHISKER_KART_TANIM_ÖRNEĞİ, BoyutSenkronAkışı, CANDLESTICK_KART_TANIM_ÖRNEĞİ,
    CURSOR_BIND_KART_TANIM_ÖRNEĞİ, CURSOR_SNAP_KART_TANIM_ÖRNEĞİ, CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ,
    CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ, CustomScaleÖrneği, DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ,
    DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ, DRAW_HOOKS_KART_TANIM_ÖRNEĞİ,
    FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ, FocusÖrneği, GRADIENTS_KART_TANIM_ÖRNEĞİ,
    GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ, GradientÖrneği, Grafik, HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ,
    HighLowBandsÖrneği, LATENCY_HEATMAP_KART_TANIM_ÖRNEĞİ, LINE_PATHS_KART_TANIM_ÖRNEĞİ,
    LOG_SCALES_KART_TANIM_ÖRNEĞİ, LOG_SCALES2_KART_TANIM_ÖRNEĞİ, LatencyHeatmapÖrneği,
    LinePathsÖrneği, LogScales2Örneği, LogScalesÖrneği, MASS_SPECTRUM_KART_TANIM_ÖRNEĞİ,
    MEASURE_DATUMS_KART_TANIM_ÖRNEĞİ, MISSING_DATA_KART_TANIM_ÖRNEĞİ, MONTHS_KART_TANIM_ÖRNEĞİ,
    MULTI_BARS_KART_TANIM_ÖRNEĞİ, MultiBarsÖrneği, NEAREST_NON_NULL_KART_TANIM_ÖRNEĞİ,
    NICE_SCALE_KART_TANIM_ÖRNEĞİ, NO_DATA_KART_TANIM_ÖRNEĞİ, NearestNonNullÖrneği, NoDataÖrneği,
    PATH_GAP_CLIP_KART_TANIM_ÖRNEĞİ, PIXEL_ALIGN_KART_TANIM_ÖRNEĞİ, POINTS_KART_TANIM_ÖRNEĞİ,
    PathGapClipÖrneği, PixelAlignÖrneği, PointsÖrneği, RESIZE_KART_TANIM_ÖRNEĞİ,
    SCALE_PADDING_KART_TANIM_ÖRNEĞİ, SCALES_DIR_ORI_KART_TANIM_ÖRNEĞİ, SCATTER_KART_TANIM_ÖRNEĞİ,
    SCROLL_SYNC_KART_TANIM_ÖRNEĞİ, SINE_STREAM_KART_TANIM_ÖRNEĞİ, SOFT_MINMAX_KART_TANIM_ÖRNEĞİ,
    SPARKLINES_BARS_KART_TANIM_ÖRNEĞİ, SPARKLINES_KART_TANIM_ÖRNEĞİ, SPARSE_KART_TANIM_ÖRNEĞİ,
    STACKED_SERIES_KART_TANIM_ÖRNEĞİ, STREAM_DATA_KART_TANIM_ÖRNEĞİ, SVG_IMAGE_KART_TANIM_ÖRNEĞİ,
    SYNC_CURSOR_KART_TANIM_ÖRNEĞİ, SYNC_Y_ZERO_KART_TANIM_ÖRNEĞİ, ScalesDirOriÖrneği,
    ScatterÖrneği, SeriSeçenekleri, SeçimEylemi, SineAkışı, SmoothingÖrneği, SoftMinMaxAkışı,
    SoftMinMaxÖrneği, SparklinesBarsÖrneği, SparklineÖrneği, SparseÖrneği, StackedSeriesÖrneği,
    StreamDataAkışı, StreamDataÖrneği, SyncCursorGrubu, SyncCursorÖrneği, SyncYZeroAşaması,
    THIN_BARS_STROKE_FILL_KART_TANIM_ÖRNEĞİ, TIME_PERIODS_KART_TANIM_ÖRNEĞİ,
    TIMELINE_DISCRETE_KART_TANIM_ÖRNEĞİ, TIMESERIES_DISCRETE_KART_TANIM_ÖRNEĞİ,
    TIMEZONES_DST_KART_TANIM_ÖRNEĞİ, TOOLTIPS_CLOSEST_KART_TANIM_ÖRNEĞİ,
    TOOLTIPS_KART_TANIM_ÖRNEĞİ, TRENDLINES_KART_TANIM_ÖRNEĞİ, TekerlekEkseni, ThinBarsÖrneği,
    TimePeriodsÖrneği, TimelineDiscreteÖrneği, TimeseriesDiscreteÖrneği, TimezonesDstÖrneği,
    UPDATE_CURSOR_SELECT_RESIZE_KART_TANIM_ÖRNEĞİ, UplotHatası, WIND_DIRECTION_KART_TANIM_ÖRNEĞİ,
    Y_SCALE_DRAG_KART_TANIM_ÖRNEĞİ, Y_SHIFTED_SERIES_KART_TANIM_ÖRNEĞİ, YShiftedSeriesAkışı,
    YüzeyDikdörtgeni, ZOOM_FETCH_KANIT_ÖRNEĞİ, ZOOM_RANGER_GRIPS_KANIT_ÖRNEĞİ,
    ZOOM_RANGER_XY_KANIT_ÖRNEĞİ, ZOOM_TOUCH_KART_TANIM_ÖRNEĞİ, ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ,
    ZoomFetchAkışı, add_del_series_ek_verisi, add_del_series_kartı, align_data_maliyet_kartı,
    align_data_çizgi_çubuk_kartı, annotations_kartı, arcsinh_scales_kartı, area_fill_kartı,
    axis_autosize_kartı, axis_control_kartı, axis_indicators_kartı, bars_grouped_stacked_kartı,
    bars_values_autosize_kartı, box_whisker_kartı, candlestick_ohlc_kartı, cursor_bind_kartı,
    cursor_snap_kartı, cursor_tooltip_kartı, custom_scales_kartı, data_smoothing_kartı,
    dependent_scale_kartı, draw_hooks_kartı, focus_cursor_kartı, gradients_kartı,
    grid_over_series_kartı, high_low_bands_kartı, latency_heatmap_kartı, line_paths_kartı,
    log_scales_kartı, log_scales2_kartı, mass_spectrum_kartı, measure_datums_kartı,
    missing_data_null_kartı, missing_data_x_boşluğu_kartı, months_artık_yıllı_kartı,
    months_artık_yılsız_kartı, months_rusça_kartı, multi_bars_kartı,
    multi_bars_kitaplık_etiketleri, multi_bars_kitaplık_kartı, nearest_non_null_kartı,
    nice_scale_kartı, no_data_kartı, ortak_kart_etkileşimleri, path_gap_clip_kartı,
    pixel_align_kartı, points_kartı, resize_kartı, scale_padding_kartı, scales_dir_ori_kartı,
    scatter_kartı, scroll_sync_kartı, sine_stream_kartı, soft_minmax_kartı, sparklines_bars_kartı,
    sparklines_kartı, sparse_kartı, stacked_series_kartı, stacked_series_kartı_görünür,
    stream_data_kartı, svg_image_kartı, sync_cursor_kartı, sync_y_zero_kartı,
    thin_bars_stroke_fill_kartı, time_periods_kartı, timeline_discrete_kartı,
    timeseries_discrete_kartı, timezones_dst_kartı, tooltips_closest_kartı, tooltips_kartı,
    trendlines_kartı, update_cursor_select_resize_kartı, wind_direction_kartı, y_scale_drag_kartı,
    y_shifted_series_kartı, zoom_touch_kartı, zoom_wheel_kartı, ÇubukYönü, ÇubukÖrneği,
};
use wasm_bindgen::prelude::*;

/// Tarayıcı yüzeyinin yalnız olayları ilettiği, seçilen kartın bütün durumunu
/// çekirdekte tutan ortak oturum.
#[wasm_bindgen]
pub struct KartOturumu {
    grafik: Grafik,
    kart_kimliği: String,
    dinamik_seri_sayacı: u32,
    yüzey: Option<YüzeyDikdörtgeni>,
    sine_akışı: Option<SineAkışı>,
    stream_data_akışı: Option<StreamDataAkışı>,
    soft_minmax_akışı: Option<SoftMinMaxAkışı>,
    boyut_senkron_akışı: Option<BoyutSenkronAkışı>,
    y_shifted_series_akışı: Option<YShiftedSeriesAkışı>,
    multi_bars_kategorileri: Option<Vec<bool>>,
    multi_bars_veri_sürümü: u64,
}

#[wasm_bindgen]
impl KartOturumu {
    #[wasm_bindgen(constructor)]
    pub fn yeni(kart_kimliği: &str, nokta_sayısı: usize) -> Result<KartOturumu, JsValue> {
        let (seçenekler, veri) = match kart_kimliği {
            "add-del-series" => add_del_series_kartı(),
            "align-data-cost" => align_data_maliyet_kartı(),
            "align-data-line-bars" => align_data_çizgi_çubuk_kartı(),
            "resize" => resize_kartı(nokta_sayısı),
            "annotations" => annotations_kartı(),
            "mass-spectrum" => mass_spectrum_kartı(),
            "measure-datums" => measure_datums_kartı(),
            kimlik if kimlik.starts_with("multi-bars-") => MultiBarsÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    multi_bars_kartı,
                ),
            kimlik if kimlik.starts_with("nearest-non-null-") => {
                NearestNonNullÖrneği::kimlikten(kimlik).map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    nearest_non_null_kartı,
                )
            }
            "area-fill" => area_fill_kartı(),
            "scale-padding" => scale_padding_kartı(),
            "zoom-wheel" => zoom_wheel_kartı(),
            "zoom-touch" => zoom_touch_kartı(),
            "months-no-leap" => months_artık_yılsız_kartı(),
            "months-leap" => months_artık_yıllı_kartı(),
            "months-russian" => months_rusça_kartı(),
            "nice-scale" => nice_scale_kartı(),
            "no-data" => no_data_kartı(NoDataÖrneği::BOŞ_ÖZEL_ARALIK),
            kimlik if kimlik.starts_with("no-data-") => NoDataÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    no_data_kartı,
                ),
            kimlik if kimlik.starts_with("path-gap-clip-") => {
                PathGapClipÖrneği::kimlikten(kimlik).map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    path_gap_clip_kartı,
                )
            }
            kimlik if kimlik.starts_with("pixel-align-") => PixelAlignÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    |örnek| pixel_align_kartı(örnek, 140),
                ),
            kimlik if kimlik.starts_with("points-") => PointsÖrneği::kimlikten(kimlik).map_or_else(
                || {
                    Err(UplotHatası::BilinmeyenKart {
                        kimlik: kimlik.to_string(),
                    })
                },
                points_kartı,
            ),
            kimlik if kimlik.starts_with("scales-dir-ori-") => {
                ScalesDirOriÖrneği::kimlikten(kimlik).map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    scales_dir_ori_kartı,
                )
            }
            kimlik if kimlik.starts_with("scatter-") => ScatterÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    scatter_kartı,
                ),
            "scroll-sync" => scroll_sync_kartı(),
            "sine-stream" => sine_stream_kartı(),
            kimlik if kimlik.starts_with("soft-minmax-") => SoftMinMaxÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    |örnek| soft_minmax_kartı(örnek, 12.0),
                ),
            kimlik if kimlik.starts_with("sparklines-bars-") => {
                SparklinesBarsÖrneği::kimlikten(kimlik).map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    sparklines_bars_kartı,
                )
            }
            kimlik if kimlik.starts_with("sparklines-") => SparklineÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    sparklines_kartı,
                ),
            kimlik if kimlik.starts_with("sparse-") => SparseÖrneği::kimlikten(kimlik).map_or_else(
                || {
                    Err(UplotHatası::BilinmeyenKart {
                        kimlik: kimlik.to_string(),
                    })
                },
                sparse_kartı,
            ),
            kimlik if kimlik.starts_with("stacked-series-") => {
                StackedSeriesÖrneği::kimlikten(kimlik).map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    stacked_series_kartı,
                )
            }
            kimlik if kimlik.starts_with("stream-data-") => StreamDataÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    stream_data_kartı,
                ),
            "svg-image" => svg_image_kartı(),
            "sync-cursor" => sync_cursor_kartı(SyncCursorÖrneği::Cpu),
            "sync-y-zero" => sync_y_zero_kartı(SyncYZeroAşaması::Ham),
            kimlik if kimlik.starts_with("thin-bars-") => ThinBarsÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    thin_bars_stroke_fill_kartı,
                ),
            kimlik if kimlik.starts_with("time-periods-") => TimePeriodsÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    time_periods_kartı,
                ),
            kimlik if kimlik.starts_with("timeline-discrete-") => {
                TimelineDiscreteÖrneği::kimlikten(kimlik).map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    timeline_discrete_kartı,
                )
            }
            kimlik if kimlik.starts_with("timeseries-discrete-") => {
                TimeseriesDiscreteÖrneği::kimlikten(kimlik).map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    timeseries_discrete_kartı,
                )
            }
            kimlik if kimlik.starts_with("timezones-dst-") => {
                TimezonesDstÖrneği::kimlikten(kimlik).map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    timezones_dst_kartı,
                )
            }
            "tooltips-closest" => tooltips_closest_kartı(),
            "tooltips" => tooltips_kartı(),
            "trendlines" => trendlines_kartı(),
            "update-cursor-select-resize" => update_cursor_select_resize_kartı(800),
            "wind-direction" => wind_direction_kartı(),
            "y-scale-drag" => y_scale_drag_kartı(),
            "y-shifted-series" => y_shifted_series_kartı(),
            kimlik if kimlik.starts_with("sync-cursor-") => SyncCursorÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    sync_cursor_kartı,
                ),
            "cursor-bind" => cursor_bind_kartı(),
            "cursor-snap" => cursor_snap_kartı(),
            "cursor-tooltip" => cursor_tooltip_kartı(),
            "custom-scales-linear" => custom_scales_kartı(CustomScaleÖrneği::Doğrusal),
            "custom-scales-log-log" => custom_scales_kartı(CustomScaleÖrneği::LogLog),
            "custom-scales-weibull" => custom_scales_kartı(CustomScaleÖrneği::Weibull),
            "data-smoothing-raw" => data_smoothing_kartı(SmoothingÖrneği::Ham),
            "data-smoothing-sgg" => data_smoothing_kartı(SmoothingÖrneği::SavitzkyGolay),
            "data-smoothing-asap" => data_smoothing_kartı(SmoothingÖrneği::Asap),
            "data-smoothing-moving-average" => {
                data_smoothing_kartı(SmoothingÖrneği::HareketliOrtalama)
            }
            "draw-hooks" => draw_hooks_kartı(),
            "focus-cursor" => focus_cursor_kartı(FocusÖrneği::İmleç),
            "focus-cursor-dynamic" => focus_cursor_kartı(FocusÖrneği::Dinamik),
            "focus-cursor-width-stroke" => focus_cursor_kartı(FocusÖrneği::KalınlıkVeRenk),
            "focus-cursor-performance-300" => focus_cursor_kartı(FocusÖrneği::Performans300),
            "gradients-horizontal-stroke" => gradients_kartı(GradientÖrneği::YatayÇizgi),
            "gradients-vertical-stroke" => gradients_kartı(GradientÖrneği::DikeyÇizgi),
            "gradients-vertical-arcsinh" => gradients_kartı(GradientÖrneği::DikeyArcSinh),
            "gradients-scale-fills" => gradients_kartı(GradientÖrneği::ÖlçekDolguları),
            "gradients-relative-fill" => gradients_kartı(GradientÖrneği::GöreliDolgu),
            "grid-over-series" => grid_over_series_kartı(),
            kimlik if kimlik.starts_with("high-low-bands-") => {
                HighLowBandsÖrneği::kimlikten(kimlik).map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    high_low_bands_kartı,
                )
            }
            kimlik if kimlik.starts_with("latency-") => LatencyHeatmapÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    |örnek| latency_heatmap_kartı(örnek, 5.0, 0.0),
                ),
            kimlik if kimlik.starts_with("line-paths-") => LinePathsÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    line_paths_kartı,
                ),
            kimlik if kimlik.starts_with("log-scales-") => LogScalesÖrneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    log_scales_kartı,
                ),
            kimlik if kimlik.starts_with("log-scales2-") => LogScales2Örneği::kimlikten(kimlik)
                .map_or_else(
                    || {
                        Err(UplotHatası::BilinmeyenKart {
                            kimlik: kimlik.to_string(),
                        })
                    },
                    log_scales2_kartı,
                ),
            "missing-data-null" => missing_data_null_kartı(),
            "missing-data-x-gap" => missing_data_x_boşluğu_kartı(),
            "dependent-scale" => dependent_scale_kartı(),
            "arcsinh-scales" => arcsinh_scales_kartı(),
            "axis-control" => axis_control_kartı(),
            "axis-autosize" => axis_autosize_kartı(1.0),
            "axis-indicators" => axis_indicators_kartı(),
            "bars-values-autosize-vertical" => bars_values_autosize_kartı(ÇubukYönü::Dikey),
            "bars-values-autosize-horizontal" => bars_values_autosize_kartı(ÇubukYönü::Yatay),
            "candlestick-ohlc" => candlestick_ohlc_kartı(),
            kimlik if kimlik.starts_with("box-whisker-") => {
                box_whisker_kartı(kimlik.trim_start_matches("box-whisker-"))
            }
            kimlik => ÇubukÖrneği::kimlikten(kimlik).map_or_else(
                || {
                    Err(UplotHatası::BilinmeyenKart {
                        kimlik: kimlik.to_string(),
                    })
                },
                bars_grouped_stacked_kartı,
            ),
        }
        .map_err(js_hatası)?;
        let grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        let sine_akışı = if kart_kimliği == "sine-stream" {
            Some(SineAkışı::yeni().map_err(js_hatası)?)
        } else {
            None
        };
        let soft_minmax_akışı =
            SoftMinMaxÖrneği::kimlikten(kart_kimliği).map(|_| SoftMinMaxAkışı::yeni());
        let stream_data_akışı = StreamDataÖrneği::kimlikten(kart_kimliği)
            .map(StreamDataAkışı::yeni)
            .transpose()
            .map_err(js_hatası)?;
        let boyut_senkron_akışı =
            (kart_kimliği == "update-cursor-select-resize").then(BoyutSenkronAkışı::yeni);
        let y_shifted_series_akışı = if kart_kimliği == "y-shifted-series" {
            Some(YShiftedSeriesAkışı::yeni().map_err(js_hatası)?)
        } else {
            None
        };
        let multi_bars_kategorileri = MultiBarsÖrneği::kimlikten(kart_kimliği)
            .filter(|örnek| {
                matches!(
                    örnek,
                    MultiBarsÖrneği::KitaplıklarDikey | MultiBarsÖrneği::KitaplıklarYatay
                )
            })
            .map(|_| {
                multi_bars_kitaplık_etiketleri()
                    .map(|etiketler| vec![true; etiketler.len()])
                    .map_err(js_hatası)
            })
            .transpose()?;
        Ok(Self {
            grafik,
            kart_kimliği: kart_kimliği.to_string(),
            dinamik_seri_sayacı: 0,
            yüzey: None,
            sine_akışı,
            stream_data_akışı,
            soft_minmax_akışı,
            boyut_senkron_akışı,
            y_shifted_series_akışı,
            multi_bars_kategorileri,
            multi_bars_veri_sürümü: 0,
        })
    }

    pub fn svg(&self, genişlik: u32, yükseklik: u32) -> String {
        self.grafik.çiz_görünür_boyutta(genişlik, yükseklik).svg()
    }

    pub fn yuzeyi_esitle(&mut self, sol: f64, ust: f64, genislik: f64, yukseklik: f64) -> bool {
        let yeni = YüzeyDikdörtgeni::yeni(sol, ust, genislik, yukseklik);
        let geçerli = yeni.is_some();
        self.yüzey = yeni;
        geçerli
    }

    pub fn istemci_konumu(
        &self,
        istemci_x: f64,
        istemci_y: f64,
        sahne_genisligi: u32,
        sahne_yuksekligi: u32,
    ) -> Vec<f64> {
        self.yüzey
            .and_then(|yüzey| {
                yüzey.sahne_konumu(istemci_x, istemci_y, sahne_genisligi, sahne_yuksekligi)
            })
            .map_or_else(Vec::new, |nokta| {
                vec![f64::from(nokta.x), f64::from(nokta.y)]
            })
    }

    pub fn sine_akisini_ilerlet(&mut self) -> Result<bool, JsValue> {
        let Some(akış) = self.sine_akışı.as_mut() else {
            return Ok(false);
        };
        let veri = akış.ilerlet().map_err(js_hatası)?;
        self.grafik.veriyi_ayarla(veri).map_err(js_hatası)?;
        Ok(true)
    }

    pub fn stream_data_ilerlet(&mut self) -> Result<bool, JsValue> {
        let Some(akış) = self.stream_data_akışı.as_mut() else {
            return Ok(false);
        };
        if !akış.ilerlet() {
            return Ok(false);
        }
        let (_, veri) = akış.kartı().map_err(js_hatası)?;
        self.grafik.veriyi_ayarla(veri).map_err(js_hatası)?;
        Ok(true)
    }

    pub fn soft_minmax_ilerlet(&mut self) -> Result<bool, JsValue> {
        let Some(örnek) = SoftMinMaxÖrneği::kimlikten(&self.kart_kimliği) else {
            return Ok(false);
        };
        if !örnek.canlı_mı() {
            return Ok(false);
        }
        let Some(akış) = self.soft_minmax_akışı.as_mut() else {
            return Ok(false);
        };
        let veri = akış.ilerlet(örnek).map_err(js_hatası)?;
        self.grafik.veriyi_ayarla(veri).map_err(js_hatası)?;
        Ok(true)
    }

    pub fn y_shifted_series_ilerlet(&mut self) -> Result<bool, JsValue> {
        let Some(akış) = self.y_shifted_series_akışı.as_mut() else {
            return Ok(false);
        };
        let (seçenekler, veri) = akış.ilerlet().map_err(js_hatası)?;
        self.grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        Ok(true)
    }

    pub fn boyut_senkron_ilerlet(&mut self) -> Result<u32, JsValue> {
        let Some(akış) = self.boyut_senkron_akışı.as_mut() else {
            return Ok(0);
        };
        let boyut = akış.ilerlet();
        self.grafik.boyutu_ayarla(boyut, boyut).map_err(js_hatası)?;
        Ok(boyut)
    }

    pub fn kaynak_boyutu(&self) -> u32 {
        self.boyut_senkron_akışı.map_or(0, BoyutSenkronAkışı::boyut)
    }

    pub fn x_dikey(&self) -> bool {
        self.grafik.x_dikey_mi()
    }

    pub fn tekerlek(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        delta: f64,
        hassas_girdi: bool,
    ) -> Result<bool, JsValue> {
        self.grafik
            .tekerlek(yatay_odak_oranı, dikey_odak_oranı, delta, hassas_girdi)
            .map_err(js_hatası)
    }

    /// `eksen`: 1 = yalnız X (Shift), 2 = yalnız Y (Ctrl), diğer = ikisi.
    pub fn tekerlek_eksende(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        delta: f64,
        hassas_girdi: bool,
        eksen: u8,
    ) -> Result<bool, JsValue> {
        let eksen = match eksen {
            1 => TekerlekEkseni::X,
            2 => TekerlekEkseni::Y,
            _ => TekerlekEkseni::İkisi,
        };
        self.grafik
            .tekerlek_eksende(
                yatay_odak_oranı,
                dikey_odak_oranı,
                delta,
                hassas_girdi,
                eksen,
            )
            .map_err(js_hatası)
    }

    pub fn secim_yakinlastir(
        &mut self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
    ) -> Result<bool, JsValue> {
        self.grafik
            .seçim_yakınlaştır(başlangıç_oranı, bitiş_oranı)
            .map_err(js_hatası)
    }

    /// 0: değişmedi, 1: yakınlaştırıldı, 2: açıklama metni istenmeli.
    pub fn secimi_bitir(
        &mut self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
        açıklama_tuşu: bool,
    ) -> Result<u8, JsValue> {
        self.grafik
            .seçimi_bitir(başlangıç_oranı, bitiş_oranı, açıklama_tuşu)
            .map(|eylem| match eylem {
                SeçimEylemi::Değişmedi => 0,
                SeçimEylemi::Yakınlaştırıldı => 1,
                SeçimEylemi::Açıklamaİstendi => 2,
            })
            .map_err(js_hatası)
    }

    pub fn ctrl_aciklama_etkin(&self) -> bool {
        self.grafik.etkileşim_seçenekleri().ctrl_açıklama
    }

    pub fn add_del_seri_ekle(&mut self) -> Result<bool, JsValue> {
        let değerler = add_del_series_ek_verisi(self.dinamik_seri_sayacı);
        self.grafik
            .seri_ekle(
                1,
                SeriSeçenekleri::yeni("Orange")
                    .renk("#ffa500")
                    .dolgu("#ffa5001a"),
                değerler,
            )
            .map_err(js_hatası)?;
        self.dinamik_seri_sayacı = self.dinamik_seri_sayacı.wrapping_add(1);
        Ok(true)
    }

    pub fn add_del_seri_sil(&mut self) -> Result<bool, JsValue> {
        if self.grafik.seri_seçenekleri().len() < 2 {
            return Ok(false);
        }
        self.grafik.seri_sil(1).map_err(js_hatası)?;
        Ok(true)
    }

    pub fn seri_sayisi(&self) -> usize {
        self.grafik.seri_seçenekleri().len()
    }

    pub fn seri_gorunur(&self, seri_indeksi: usize) -> bool {
        self.grafik
            .seri_seçenekleri()
            .get(seri_indeksi)
            .is_some_and(|seri| seri.göster)
    }

    pub fn seri_gorunurlugu_ayarla(
        &mut self,
        seri_indeksi: usize,
        görünür: bool,
    ) -> Result<bool, JsValue> {
        self.grafik
            .seri_görünürlüğünü_ayarla(seri_indeksi, görünür)
            .map_err(js_hatası)
    }

    pub fn multi_bars_kategori_etiketleri(&self) -> Vec<String> {
        if self.multi_bars_kategorileri.is_none() {
            return Vec::new();
        }
        multi_bars_kitaplık_etiketleri().unwrap_or_default()
    }

    pub fn multi_bars_kategori_durumlari(&self) -> Vec<u8> {
        self.multi_bars_kategorileri
            .as_ref()
            .map(|durumlar| durumlar.iter().map(|etkin| u8::from(*etkin)).collect())
            .unwrap_or_default()
    }

    pub fn multi_bars_kategorisini_ayarla(
        &mut self,
        indeks: usize,
        etkin: bool,
    ) -> Result<bool, JsValue> {
        let Some(örnek) = MultiBarsÖrneği::kimlikten(&self.kart_kimliği).filter(|örnek| {
            matches!(
                örnek,
                MultiBarsÖrneği::KitaplıklarDikey | MultiBarsÖrneği::KitaplıklarYatay
            )
        }) else {
            return Ok(false);
        };
        let Some(durumlar) = self.multi_bars_kategorileri.as_mut() else {
            return Ok(false);
        };
        let kategori_sayısı = durumlar.len();
        let Some(durum) = durumlar.get_mut(indeks) else {
            return Err(js_hatası(UplotHatası::GeçersizSeriİndeksi {
                indeks,
                seri_sayısı: kategori_sayısı,
                ekleme: false,
            }));
        };
        if *durum == etkin {
            return Ok(false);
        }
        *durum = etkin;
        self.multi_bars_veri_sürümü = self.multi_bars_veri_sürümü.wrapping_add(1);
        let stiller = self
            .grafik
            .seri_seçenekleri()
            .iter()
            .map(|seri| {
                (
                    seri.göster,
                    seri.renk.clone(),
                    seri.dolgu.clone(),
                    seri.çubuk_dolguları.clone(),
                    seri.çubuk_çizgileri.clone(),
                )
            })
            .collect::<Vec<_>>();
        let tekerlek = self.grafik.etkileşim_seçenekleri().tekerlek_etkileşimi;
        let (seçenekler, veri) =
            multi_bars_kitaplık_kartı(örnek, durumlar, self.multi_bars_veri_sürümü)
                .map_err(js_hatası)?;
        let mut grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        grafik.tekerlek_etkileşimi_ayarla(tekerlek);
        for (seri, (görünür, çizgi, dolgu, çubuk_dolguları, çubuk_çizgileri)) in
            stiller.into_iter().enumerate()
        {
            grafik
                .seri_görünürlüğünü_ayarla(seri, görünür)
                .map_err(js_hatası)?;
            grafik
                .seri_renklerini_ayarla(seri, çizgi, dolgu)
                .map_err(js_hatası)?;
            grafik
                .seri_çubuk_renklerini_ayarla(seri, çubuk_dolguları, çubuk_çizgileri)
                .map_err(js_hatası)?;
        }
        self.grafik = grafik;
        Ok(true)
    }

    pub fn seri_renklerini_ayarla(
        &mut self,
        seri_indeksi: usize,
        çizgi: String,
        dolgu: Option<String>,
    ) -> Result<bool, JsValue> {
        self.grafik
            .seri_renklerini_ayarla(seri_indeksi, çizgi, dolgu)
            .map_err(js_hatası)
    }

    pub fn stacked_seri_gorunurlugu_ayarla(
        &mut self,
        seri_indeksi: usize,
        görünür: bool,
    ) -> Result<bool, JsValue> {
        let Some(örnek) = StackedSeriesÖrneği::kimlikten(&self.kart_kimliği) else {
            return Ok(false);
        };
        if seri_indeksi >= self.grafik.seri_seçenekleri().len() {
            return Ok(false);
        }
        let mut görünürlük = self
            .grafik
            .seri_seçenekleri()
            .iter()
            .map(|seri| seri.göster)
            .collect::<Vec<_>>();
        if let Some(hedef) = görünürlük.get_mut(seri_indeksi) {
            *hedef = görünür;
        }
        let tekerlek = self.grafik.etkileşim_seçenekleri().tekerlek_etkileşimi;
        let (seçenekler, veri) =
            stacked_series_kartı_görünür(örnek, &görünürlük).map_err(js_hatası)?;
        let mut grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        grafik.tekerlek_etkileşimi_ayarla(tekerlek);
        self.grafik = grafik;
        Ok(true)
    }

    pub fn seri_etiketleri(&self) -> Vec<String> {
        self.grafik
            .seri_seçenekleri()
            .iter()
            .map(|seri| seri.etiket.clone())
            .collect()
    }

    pub fn seri_renkleri(&self) -> Vec<String> {
        self.grafik
            .seri_seçenekleri()
            .iter()
            .map(|seri| seri.renk.clone())
            .collect()
    }

    pub fn tasimayi_baslat(&mut self) -> bool {
        self.grafik.taşımayı_başlat()
    }

    pub fn eksen_vurusu(&self, genişlik: u32, yükseklik: u32, x: f32, y: f32) -> bool {
        self.grafik
            .eksen_vuruşu_boyutta(genişlik, yükseklik, x, y)
            .is_some()
    }

    pub fn eksen_suruklemeyi_baslat(
        &mut self,
        genişlik: u32,
        yükseklik: u32,
        x: f32,
        y: f32,
    ) -> bool {
        self.grafik
            .eksen_sürüklemeyi_başlat(genişlik, yükseklik, x, y)
    }

    pub fn eksen_surukle(&mut self, x: f32, y: f32, shift: bool) -> Result<bool, JsValue> {
        self.grafik.eksen_sürükle(x, y, shift).map_err(js_hatası)
    }

    pub fn eksen_suruklemeyi_bitir(&mut self) {
        self.grafik.eksen_sürüklemeyi_bitir();
    }

    pub fn tasi(
        &mut self, yatay_fark_oranı: f64, dikey_fark_oranı: f64
    ) -> Result<bool, JsValue> {
        self.grafik
            .taşı(yatay_fark_oranı, dikey_fark_oranı)
            .map_err(js_hatası)
    }

    pub fn tasimayi_bitir(&mut self) {
        self.grafik.taşımayı_bitir();
    }

    pub fn dokunmayi_baslat(&mut self) -> bool {
        self.grafik.dokunmayı_başlat()
    }

    pub fn dokunma_yakinlastir(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        çarpan: f64,
    ) -> Result<bool, JsValue> {
        self.grafik
            .dokunma_yakınlaştır(yatay_odak_oranı, dikey_odak_oranı, çarpan)
            .map_err(js_hatası)
    }

    pub fn dokunmayi_bitir(&mut self) {
        self.grafik.dokunmayı_bitir();
    }

    pub fn tam_gorunum(&mut self) -> bool {
        self.grafik.tam_görünüm()
    }

    pub fn onceki_gorunum(&mut self) -> bool {
        self.grafik.önceki_görünüm()
    }

    pub fn tekerlek_etkilesimi_ayarla(&mut self, etkin: bool) {
        self.grafik.tekerlek_etkileşimi_ayarla(etkin);
    }

    pub fn bosluklari_birlestir_ayarla(&mut self, etkin: bool) {
        self.grafik.boşlukları_birleştir_ayarla(etkin);
    }

    pub fn gorunur_x_araligi(&self) -> Vec<f64> {
        let aralık = self.grafik.görünür_x_aralığı();
        vec![aralık.en_az, aralık.en_çok]
    }

    pub fn gorunur_y_araligi(&self) -> Vec<f64> {
        let aralık = self.grafik.görünür_y_aralığı();
        vec![aralık.en_az, aralık.en_çok]
    }

    pub fn olcum_datumu_ayarla(&mut self, datum: usize, yatay_oran: f64, dikey_oran: f64) -> bool {
        self.grafik
            .ölçüm_datumunu_ayarla(datum, yatay_oran, dikey_oran)
    }

    pub fn olcum_datumlarini_temizle(&mut self) -> bool {
        self.grafik.ölçüm_datumlarını_temizle()
    }

    pub fn seri_gorunur_y_araligi(&self, seri_indeksi: usize) -> Vec<f64> {
        self.grafik
            .seri_görünür_y_aralığı(seri_indeksi)
            .map_or_else(Vec::new, |aralık| vec![aralık.en_az, aralık.en_çok])
    }

    pub fn seri_y_konum_orani(&self, seri_indeksi: usize, değer: f64) -> f64 {
        self.grafik
            .seri_y_konum_oranı(seri_indeksi, değer)
            .unwrap_or(f64::NAN)
    }

    pub fn x_konum_orani(&self, değer: f64) -> f64 {
        self.grafik.x_konum_oranı(değer).unwrap_or(f64::NAN)
    }

    pub fn y_arcsinh_esigi_ayarla(&mut self, anahtar: &str, eşik: f64) -> bool {
        self.grafik.y_arcsinh_eşiği_ayarla(anahtar, eşik)
    }

    pub fn axis_autosize_carpani_ayarla(&mut self, çarpan: f64) -> Result<(), JsValue> {
        let (seçenekler, veri) = axis_autosize_kartı(çarpan).map_err(js_hatası)?;
        self.grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        Ok(())
    }

    pub fn sync_y_zero_asamasini_ayarla(&mut self, aşama: &str) -> Result<bool, JsValue> {
        if self.kart_kimliği != "sync-y-zero" {
            return Ok(false);
        }
        let aşama = match aşama {
            "raw" => SyncYZeroAşaması::Ham,
            "symmetric" => SyncYZeroAşaması::Simetrik,
            "zero-aligned" => SyncYZeroAşaması::SıfırHizalı,
            _ => {
                return Err(JsValue::from_str(
                    "Bilinmeyen Sync Y Zero aşaması; raw, symmetric veya zero-aligned bekleniyor",
                ));
            }
        };
        let tekerlek = self.grafik.etkileşim_seçenekleri().tekerlek_etkileşimi;
        let (seçenekler, veri) = sync_y_zero_kartı(aşama).map_err(js_hatası)?;
        let mut grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        grafik.tekerlek_etkileşimi_ayarla(tekerlek);
        self.grafik = grafik;
        Ok(true)
    }

    pub fn pixel_align_adimi_ayarla(&mut self, adım: usize) -> Result<bool, JsValue> {
        let Some(örnek) = PixelAlignÖrneği::kimlikten(&self.kart_kimliği) else {
            return Ok(false);
        };
        let tekerlek = self.grafik.etkileşim_seçenekleri().tekerlek_etkileşimi;
        let (seçenekler, veri) = pixel_align_kartı(örnek, adım).map_err(js_hatası)?;
        let mut grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        grafik.tekerlek_etkileşimi_ayarla(tekerlek);
        self.grafik = grafik;
        Ok(true)
    }

    pub fn eksen_gostergeleri_etkin(&self) -> bool {
        self.grafik.eksen_göstergeleri_etkin()
    }

    pub fn cubuk_vurusu(&self, genişlik: u32, yükseklik: u32, x: f32, y: f32) -> Vec<f64> {
        self.grafik
            .çubuk_vuruşu(genişlik, yükseklik, x, y)
            .map_or_else(
                Vec::new,
                |(seri, indeks, konum, çubuk_g, çubuk_y, değer)| {
                    vec![
                        seri as f64,
                        indeks as f64,
                        f64::from(konum.x),
                        f64::from(konum.y),
                        f64::from(çubuk_g),
                        f64::from(çubuk_y),
                        değer,
                    ]
                },
            )
    }

    pub fn kutu_biyik_vurusu(&self, genişlik: u32, yükseklik: u32, x: f32, y: f32) -> Vec<f64> {
        self.grafik
            .kutu_bıyık_vuruşu(genişlik, yükseklik, x, y)
            .map_or_else(Vec::new, |(indeks, konum, kutu_g, kutu_y, değerler)| {
                let mut sonuç = vec![
                    indeks as f64,
                    f64::from(konum.x),
                    f64::from(konum.y),
                    f64::from(kutu_g),
                    f64::from(kutu_y),
                ];
                sonuç.extend(değerler);
                sonuç
            })
    }

    pub fn dagilim_vurusu(&self, genişlik: u32, yükseklik: u32, x: f32, y: f32) -> Vec<f64> {
        self.grafik
            .dağılım_vuruşu_boyutta(genişlik, yükseklik, x, y)
            .map_or_else(Vec::new, |vuruş| {
                vec![
                    vuruş.seri as f64,
                    vuruş.indeks as f64,
                    f64::from(vuruş.merkez.x),
                    f64::from(vuruş.merkez.y),
                    f64::from(vuruş.boyut),
                    vuruş.x,
                    vuruş.y,
                    vuruş.değer.unwrap_or(f64::NAN),
                ]
            })
    }

    pub fn dagilim_vurus_etiketi(&self, genişlik: u32, yükseklik: u32, x: f32, y: f32) -> String {
        self.grafik
            .dağılım_vuruşu_boyutta(genişlik, yükseklik, x, y)
            .and_then(|vuruş| vuruş.etiket)
            .unwrap_or_default()
    }

    pub fn cizim_alani(&self, genişlik: u32, yükseklik: u32) -> Vec<f64> {
        let (sol, sağ, üst, alt) = self.grafik.çizim_alanı_boyutta(genişlik, yükseklik);
        vec![
            f64::from(sol),
            f64::from(sağ),
            f64::from(üst),
            f64::from(alt),
        ]
    }

    pub fn aciklama_vurgusu(&self, genişlik: u32, yükseklik: u32, x: f32, y: f32) -> String {
        let Some(vuruş) = self
            .grafik
            .açıklama_vuruşu_boyutta(genişlik, yükseklik, x, y)
        else {
            return String::new();
        };
        let svg = self
            .grafik
            .açıklama_vurgu_sahnesi_boyutta(genişlik, yükseklik, &vuruş)
            .svg();
        let (etiket_x, etiket_y) = vuruş
            .etiket_konumu
            .map_or((f32::NAN, f32::NAN), |konum| (konum.x, konum.y));
        serde_json::to_string(&serde_json::json!({
            "indeks": vuruş.indeks,
            "svg": svg,
            "baslangicX": vuruş.başlangıç_x,
            "bitisX": vuruş.bitiş_x,
            "ust": vuruş.üst,
            "alt": vuruş.alt,
            "etiketX": etiket_x,
            "etiketY": etiket_y,
            "etiketGenisligi": vuruş.etiket_genişliği,
            "etiketYuksekligi": vuruş.etiket_yüksekliği,
            "etiketUzerinde": vuruş.etiket_üzerinde,
            "etiket": vuruş.etiket,
            "aciklama": vuruş.açıklama,
            "cizgi": vuruş.çizgi,
            "kalinlik": vuruş.kalınlık,
        }))
        .unwrap_or_default()
    }

    pub fn en_yakin_nokta(&self, yatay_oran: f64) -> Vec<f64> {
        self.grafik
            .en_yakın_nokta(yatay_oran, 0)
            .map_or_else(Vec::new, |(x, y)| vec![x, y])
    }

    pub fn en_yakin_noktalar(&self, yatay_oran: f64) -> Vec<f64> {
        self.grafik
            .en_yakın_noktalar(yatay_oran)
            .map_or_else(Vec::new, |(x, değerler)| {
                let mut sonuç = Vec::with_capacity(değerler.len().saturating_add(1));
                sonuç.push(x);
                sonuç.extend(değerler.into_iter().map(|değer| değer.unwrap_or(f64::NAN)));
                sonuç
            })
    }

    pub fn timeline_vuruslari(&self, yatay_oran: f64) -> Vec<f64> {
        self.grafik
            .timeline_vuruşları(yatay_oran)
            .into_iter()
            .flat_map(|vuruş| {
                [
                    vuruş.seri as f64,
                    vuruş.indeks as f64,
                    vuruş.başlangıç,
                    vuruş.bitiş,
                ]
            })
            .collect()
    }

    pub fn son_degerler(&self) -> Vec<f64> {
        self.grafik
            .son_değerler()
            .map_or_else(Vec::new, |(x, değerler)| {
                std::iter::once(x)
                    .chain(değerler.into_iter().map(|değer| değer.unwrap_or(f64::NAN)))
                    .collect()
            })
    }

    pub fn bosta_lejant_degerleri(&self) -> Vec<f64> {
        self.grafik
            .boşta_lejant_değerleri()
            .unwrap_or_default()
            .into_iter()
            .map(|değer| değer.unwrap_or(f64::NAN))
            .collect()
    }

    pub fn lejant_canli(&self) -> bool {
        self.grafik.lejant_canlı()
    }

    pub fn imlec_seri_renkleri(&self, yatay_oran: f64) -> Vec<String> {
        self.grafik
            .en_yakın_noktalar(yatay_oran)
            .map_or_else(Vec::new, |(x, değerler)| {
                değerler
                    .into_iter()
                    .enumerate()
                    .map(|(indeks, değer)| {
                        değer
                            .and_then(|y| self.grafik.seri_imleç_rengi(indeks, x, y))
                            .or_else(|| {
                                self.grafik
                                    .seri_seçenekleri()
                                    .get(indeks)
                                    .map(|seri| seri.renk.clone())
                            })
                            .unwrap_or_else(|| "#000000".to_string())
                    })
                    .collect()
            })
    }

    pub fn imlec_oranlarini_uyarla(
        &self,
        yatay_oran: f64,
        dikey_oran: f64,
        çizim_genişliği: f64,
        çizim_yüksekliği: f64,
    ) -> Vec<f64> {
        self.grafik
            .imleç_oranlarını_uyarla(yatay_oran, dikey_oran, çizim_genişliği, çizim_yüksekliği)
            .map_or_else(Vec::new, |(x, y)| vec![x, y])
    }

    pub fn imlec_odagini_guncelle(
        &mut self,
        yatay_oran: f64,
        dikey_oran: f64,
        çizim_yüksekliği: f64,
    ) -> bool {
        self.grafik
            .imleç_odağını_güncelle(yatay_oran, dikey_oran, çizim_yüksekliği)
    }

    pub fn imlec_odagini_temizle(&mut self) -> bool {
        self.grafik.imleç_odağını_temizle()
    }

    pub fn imlec_odagini_seriye_ayarla(&mut self, seri_indeksi: i32) -> bool {
        let seri = usize::try_from(seri_indeksi).ok();
        self.grafik.imleç_odağını_seriye_ayarla(seri)
    }

    pub fn odak_serisi(&self) -> i32 {
        self.grafik
            .odak_serisi()
            .and_then(|indeks| i32::try_from(indeks).ok())
            .unwrap_or(-1)
    }

    pub fn en_yakin_tooltip(&self, yatay_oran: f64, seri_indeksi: i32) -> Vec<String> {
        usize::try_from(seri_indeksi)
            .ok()
            .and_then(|seri| self.grafik.en_yakın_tooltip(yatay_oran, seri))
            .map_or_else(Vec::new, |bilgi| {
                vec![
                    bilgi.metin,
                    bilgi.kenarlık_rengi,
                    bilgi.karşılaştırma_url,
                    bilgi.interpolasyon.to_string(),
                ]
            })
    }

    pub fn tooltip_bilgileri(&self, yatay_oran: f64, dikey_oran: f64) -> Vec<String> {
        self.grafik
            .tooltip_bilgileri(yatay_oran, dikey_oran)
            .into_iter()
            .flat_map(|bilgi| {
                [
                    bilgi
                        .seri
                        .and_then(|seri| i32::try_from(seri).ok())
                        .unwrap_or(-1)
                        .to_string(),
                    bilgi.metin,
                    bilgi.yatay_oran.to_string(),
                    bilgi.dikey_oran.to_string(),
                    bilgi.arka_plan_rengi,
                    bilgi.metin_rengi,
                ]
            })
            .collect()
    }

    pub fn tooltip_yeniden_kurma_ms(&self) -> u32 {
        self.grafik
            .tooltip_düzeni()
            .and_then(|düzen| düzen.yeniden_kurma_ms)
            .and_then(|milisaniye| u32::try_from(milisaniye).ok())
            .unwrap_or(0)
    }

    pub fn tooltip_imlec_durumunu_koru(&self) -> bool {
        self.grafik
            .tooltip_düzeni()
            .is_some_and(|düzen| düzen.imleç_durumunu_koru)
    }

    pub fn tooltip_yeniden_kur(&mut self) -> Result<bool, JsValue> {
        if self.kart_kimliği != "tooltips" || self.grafik.tooltip_düzeni().is_none() {
            return Ok(false);
        }
        let tekerlek = self.grafik.etkileşim_seçenekleri().tekerlek_etkileşimi;
        let (seçenekler, veri) = tooltips_kartı().map_err(js_hatası)?;
        let mut grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        grafik.tekerlek_etkileşimi_ayarla(tekerlek);
        self.grafik = grafik;
        Ok(true)
    }

    pub fn yakinlastirilmis(&self) -> bool {
        self.grafik.yakınlaştırılmış()
    }

    pub fn zoom_ranger_oranlari(&self) -> Vec<f64> {
        self.grafik
            .zoom_ranger_durumu()
            .map(|durum| {
                let (sol, sağ) = durum.seçim_oranları();
                let (alt, üst) = durum.y_seçim_oranları();
                vec![sol, sağ, alt, üst]
            })
            .unwrap_or_default()
    }

    pub fn zoom_ranger_tasi(&mut self, oran_farki: f64) -> bool {
        let Ok(mut durum) = self.grafik.zoom_ranger_durumu() else {
            return false;
        };
        let tam = durum.tam_aralık();
        durum.pencereyi_taşı(oran_farki * (tam.en_çok - tam.en_az))
            && self.grafik.zoom_ranger_uygula(durum)
    }

    pub fn zoom_ranger_sol(&mut self, oran: f64) -> bool {
        let Ok(mut durum) = self.grafik.zoom_ranger_durumu() else {
            return false;
        };
        let tam = durum.tam_aralık();
        let değer = tam.en_az + oran.clamp(0.0, 1.0) * (tam.en_çok - tam.en_az);
        durum.sol_tutamağı_ayarla(değer) && self.grafik.zoom_ranger_uygula(durum)
    }

    pub fn zoom_ranger_sag(&mut self, oran: f64) -> bool {
        let Ok(mut durum) = self.grafik.zoom_ranger_durumu() else {
            return false;
        };
        let tam = durum.tam_aralık();
        let değer = tam.en_az + oran.clamp(0.0, 1.0) * (tam.en_çok - tam.en_az);
        durum.sağ_tutamağı_ayarla(değer) && self.grafik.zoom_ranger_uygula(durum)
    }

    pub fn zoom_ranger_alt(&mut self, oran: f64) -> bool {
        let Ok(mut durum) = self.grafik.zoom_ranger_durumu() else {
            return false;
        };
        let tam = durum.y_tam_aralık();
        let değer = tam.en_az + oran.clamp(0.0, 1.0) * (tam.en_çok - tam.en_az);
        durum.alt_tutamağı_ayarla(değer) && self.grafik.zoom_ranger_uygula(durum)
    }

    pub fn zoom_ranger_ust(&mut self, oran: f64) -> bool {
        let Ok(mut durum) = self.grafik.zoom_ranger_durumu() else {
            return false;
        };
        let tam = durum.y_tam_aralık();
        let değer = tam.en_az + oran.clamp(0.0, 1.0) * (tam.en_çok - tam.en_az);
        durum.üst_tutamağı_ayarla(değer) && self.grafik.zoom_ranger_uygula(durum)
    }

    pub fn zoom_ranger_surukleme_ekseni(&self, x_px: f64, y_px: f64) -> u8 {
        use uplot_rs::ZoomRangerSürüklemeEkseni::{X, XY, Y, Yok};
        self.grafik.zoom_ranger_durumu().map_or(0, |durum| {
            match durum.uyarlanabilir_sürükleme_ekseni(x_px, y_px) {
                Yok => 0,
                X => 1,
                Y => 2,
                XY => 3,
            }
        })
    }

    pub fn zoom_varyasyon_ekseni(
        &self,
        kip_indeksi: usize,
        dist_on: bool,
        x_px: f64,
        y_px: f64,
    ) -> u8 {
        use uplot_rs::ZoomRangerSürüklemeEkseni::{X, XY, Y, Yok};
        let Some(kip) = uplot_rs::ZoomSürüklemeKipi::TÜMÜ.get(kip_indeksi).copied() else {
            return 0;
        };
        self.grafik.zoom_ranger_durumu().map_or(0, |mut durum| {
            durum.sürükleme_ayarlarını_ayarla(
                uplot_rs::ZoomRangerSeçenekleri::zoom_varyasyonu(
                    kip,
                    if dist_on { 10.0 } else { 0.0 },
                ),
            );
            match durum.uyarlanabilir_sürükleme_ekseni(x_px, y_px) {
                Yok => 0,
                X => 1,
                Y => 2,
                XY => 3,
            }
        })
    }

    pub fn geri_var(&self) -> bool {
        self.grafik.geri_var()
    }

    pub fn latency_histogram_ayarla(
        &mut self,
        kova_boyutu: f64,
        kova_ofseti: f64,
    ) -> Result<bool, JsValue> {
        let Some(örnek) = LatencyHeatmapÖrneği::kimlikten(&self.kart_kimliği) else {
            return Ok(false);
        };
        if !matches!(
            örnek,
            LatencyHeatmapÖrneği::HistogramBirleşik | LatencyHeatmapÖrneği::HistogramBoşluklu
        ) {
            return Ok(false);
        }
        let tekerlek = self.grafik.etkileşim_seçenekleri().tekerlek_etkileşimi;
        let (seçenekler, veri) =
            latency_heatmap_kartı(örnek, kova_boyutu, kova_ofseti).map_err(js_hatası)?;
        let mut grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        grafik.tekerlek_etkileşimi_ayarla(tekerlek);
        self.grafik = grafik;
        Ok(true)
    }
}

/// `src/sync.js` pub/sub, seri etiketi eşleme ve cursor kilidi kurallarını
/// tarayıcı adaptörüne taşıyan çekirdek köprü.
#[wasm_bindgen]
pub struct SyncCursorKoprusu {
    grup: SyncCursorGrubu,
}

#[wasm_bindgen]
impl SyncCursorKoprusu {
    #[wasm_bindgen(constructor)]
    pub fn yeni() -> Self {
        Self {
            grup: SyncCursorGrubu::yeni(),
        }
    }

    pub fn senkron(&self) -> bool {
        self.grup.senkron()
    }

    pub fn senkronu_ayarla(&mut self, etkin: bool) -> bool {
        self.grup.senkronu_ayarla(etkin)
    }

    pub fn fare_senkronu(&self) -> bool {
        self.grup.fare_basma_bırakma_senkron()
    }

    pub fn fare_senkronunu_ayarla(&mut self, etkin: bool) -> bool {
        self.grup.fare_basma_bırakma_senkronunu_ayarla(etkin)
    }

    pub fn imlec_hedefleri(&self, kaynak_indeksi: usize) -> Vec<u32> {
        örnek_from_index(kaynak_indeksi).map_or_else(Vec::new, |kaynak| {
            self.grup
                .imleç_hedefleri(kaynak)
                .into_iter()
                .filter_map(|hedef| u32::try_from(örnek_index(hedef)).ok())
                .collect()
        })
    }

    pub fn seri_hedefi(
        &self,
        kaynak_indeksi: usize,
        hedef_indeksi: usize,
        seri_indeksi: usize,
    ) -> i32 {
        let Some(kaynak) = örnek_from_index(kaynak_indeksi) else {
            return -1;
        };
        let Some(hedef) = örnek_from_index(hedef_indeksi) else {
            return -1;
        };
        self.grup
            .seri_hedefi(kaynak, hedef, seri_indeksi)
            .and_then(|indeks| i32::try_from(indeks).ok())
            .unwrap_or(-1)
    }

    pub fn dikey_imlec_senkron_mu(&self, kaynak_indeksi: usize, hedef_indeksi: usize) -> bool {
        let Some(kaynak) = örnek_from_index(kaynak_indeksi) else {
            return false;
        };
        let Some(hedef) = örnek_from_index(hedef_indeksi) else {
            return false;
        };
        self.grup.dikey_imleç_senkron_mu(kaynak, hedef)
    }

    /// `[yüzey indeksi, kilit 0/1, ...]` biçiminde değişen yüzeyleri döndürür.
    pub fn fare_birak(&mut self, kaynak_indeksi: usize) -> Vec<i32> {
        örnek_from_index(kaynak_indeksi).map_or_else(Vec::new, |kaynak| {
            self.grup
                .fare_bırak(kaynak)
                .into_iter()
                .flat_map(|(örnek, kilitli)| {
                    [
                        i32::try_from(örnek_index(örnek)).unwrap_or(-1),
                        i32::from(kilitli),
                    ]
                })
                .collect()
        })
    }

    pub fn kilitli(&self, yüzey_indeksi: usize) -> bool {
        örnek_from_index(yüzey_indeksi).is_some_and(|örnek| self.grup.kilitli(örnek))
    }
}

impl Default for SyncCursorKoprusu {
    fn default() -> Self {
        Self::yeni()
    }
}

const fn örnek_from_index(indeks: usize) -> Option<SyncCursorÖrneği> {
    match indeks {
        0 => Some(SyncCursorÖrneği::Cpu),
        1 => Some(SyncCursorÖrneği::Ram),
        2 => Some(SyncCursorÖrneği::Tcp),
        3 => Some(SyncCursorÖrneği::UyumsuzKırmızıMavi),
        4 => Some(SyncCursorÖrneği::UyumsuzYeşilKırmızı),
        _ => None,
    }
}

const fn örnek_index(örnek: SyncCursorÖrneği) -> usize {
    match örnek {
        SyncCursorÖrneği::Cpu => 0,
        SyncCursorÖrneği::Ram => 1,
        SyncCursorÖrneği::Tcp => 2,
        SyncCursorÖrneği::UyumsuzKırmızıMavi => 3,
        SyncCursorÖrneği::UyumsuzYeşilKırmızı => 4,
    }
}

fn js_hatası(hata: UplotHatası) -> JsValue {
    JsValue::from_str(&hata.to_string())
}

#[wasm_bindgen]
pub fn kart_sayisi() -> usize {
    365
}

#[wasm_bindgen]
pub fn sync_cursor_kart_tanim_ornegi() -> String {
    SYNC_CURSOR_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn sync_y_zero_kart_tanim_ornegi() -> String {
    SYNC_Y_ZERO_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn thin_bars_stroke_fill_kart_tanim_ornegi() -> String {
    THIN_BARS_STROKE_FILL_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn time_periods_kart_tanim_ornegi() -> String {
    TIME_PERIODS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn timeline_discrete_kart_tanim_ornegi() -> String {
    TIMELINE_DISCRETE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn timeseries_discrete_kart_tanim_ornegi() -> String {
    TIMESERIES_DISCRETE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn timezones_dst_kart_tanim_ornegi() -> String {
    TIMEZONES_DST_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn tooltips_closest_kart_tanim_ornegi() -> String {
    TOOLTIPS_CLOSEST_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn tooltips_kart_tanim_ornegi() -> String {
    TOOLTIPS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn trendlines_kart_tanim_ornegi() -> String {
    TRENDLINES_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn update_cursor_select_resize_kart_tanim_ornegi() -> String {
    UPDATE_CURSOR_SELECT_RESIZE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn wind_direction_kart_tanim_ornegi() -> String {
    WIND_DIRECTION_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn y_scale_drag_kart_tanim_ornegi() -> String {
    Y_SCALE_DRAG_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn y_shifted_series_kart_tanim_ornegi() -> String {
    Y_SHIFTED_SERIES_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn soft_minmax_kart_tanim_ornegi() -> String {
    SOFT_MINMAX_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn sparklines_bars_kart_tanim_ornegi() -> String {
    SPARKLINES_BARS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn sparklines_kart_tanim_ornegi() -> String {
    SPARKLINES_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn sparse_kart_tanim_ornegi() -> String {
    SPARSE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn stacked_series_kart_tanim_ornegi() -> String {
    STACKED_SERIES_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn log_scales_kart_tanim_ornegi() -> String {
    LOG_SCALES_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn log_scales2_kart_tanim_ornegi() -> String {
    LOG_SCALES2_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn line_paths_kart_tanim_ornegi() -> String {
    LINE_PATHS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn latency_heatmap_kart_tanim_ornegi() -> String {
    LATENCY_HEATMAP_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn high_low_bands_kart_tanim_ornegi() -> String {
    HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn grid_over_series_kart_tanim_ornegi() -> String {
    GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn gradients_kart_tanim_ornegi() -> String {
    GRADIENTS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn focus_cursor_kart_tanim_ornegi() -> String {
    FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn draw_hooks_kart_tanim_ornegi() -> String {
    DRAW_HOOKS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn data_smoothing_kart_tanim_ornegi() -> String {
    DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn custom_scales_kart_tanim_ornegi() -> String {
    CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn cursor_tooltip_kart_tanim_ornegi() -> String {
    CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn align_data_kart_tanim_ornegi() -> String {
    ALIGN_DATA_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn resize_kart_tanim_ornegi() -> String {
    RESIZE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn add_del_series_kart_tanim_ornegi() -> String {
    ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn area_fill_kart_tanim_ornegi() -> String {
    AREA_FILL_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn annotations_kart_tanim_ornegi() -> String {
    ANNOTATIONS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn mass_spectrum_kart_tanim_ornegi() -> String {
    MASS_SPECTRUM_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn measure_datums_kart_tanim_ornegi() -> String {
    MEASURE_DATUMS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn multi_bars_kart_tanim_ornegi() -> String {
    MULTI_BARS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn nearest_non_null_kart_tanim_ornegi() -> String {
    NEAREST_NON_NULL_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn scale_padding_kart_tanim_ornegi() -> String {
    SCALE_PADDING_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn zoom_wheel_kart_tanim_ornegi() -> String {
    ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn zoom_fetch_kanit_ornegi() -> String {
    ZOOM_FETCH_KANIT_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn zoom_ranger_grips_kanit_ornegi() -> String {
    ZOOM_RANGER_GRIPS_KANIT_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn zoom_ranger_xy_kanit_ornegi() -> String {
    ZOOM_RANGER_XY_KANIT_ÖRNEĞİ.to_owned()
}

#[wasm_bindgen]
pub fn zoom_fetch_kaniti() -> bool {
    let Ok(mut akış) = ZoomFetchAkışı::yeni() else {
        return false;
    };
    let Ok(aralık) = akış.aralık_isteği(0.25, 0.75) else {
        return false;
    };
    akış.kaynak_yanıtını_uygula(aralık).is_ok() && akış.grafik().görünür_x_aralığı() == aralık
}

#[wasm_bindgen]
pub fn zoom_touch_kart_tanim_ornegi() -> String {
    ZOOM_TOUCH_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn months_kart_tanim_ornegi() -> String {
    MONTHS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn nice_scale_kart_tanim_ornegi() -> String {
    NICE_SCALE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn stream_data_kart_tanim_ornegi() -> String {
    STREAM_DATA_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn svg_image_kart_tanim_ornegi() -> String {
    SVG_IMAGE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn no_data_kart_tanim_ornegi() -> String {
    NO_DATA_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn path_gap_clip_kart_tanim_ornegi() -> String {
    PATH_GAP_CLIP_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn pixel_align_kart_tanim_ornegi() -> String {
    PIXEL_ALIGN_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn points_kart_tanim_ornegi() -> String {
    POINTS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn scales_dir_ori_kart_tanim_ornegi() -> String {
    SCALES_DIR_ORI_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn scatter_kart_tanim_ornegi() -> String {
    SCATTER_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn scroll_sync_kart_tanim_ornegi() -> String {
    SCROLL_SYNC_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn sine_stream_kart_tanim_ornegi() -> String {
    SINE_STREAM_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn cursor_snap_kart_tanim_ornegi() -> String {
    CURSOR_SNAP_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn missing_data_kart_tanim_ornegi() -> String {
    MISSING_DATA_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn dependent_scale_kart_tanim_ornegi() -> String {
    DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn arcsinh_scales_kart_tanim_ornegi() -> String {
    ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn axis_control_kart_tanim_ornegi() -> String {
    AXIS_CONTROL_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn axis_autosize_kart_tanim_ornegi() -> String {
    AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn axis_indicators_kart_tanim_ornegi() -> String {
    AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn bars_grouped_stacked_kart_tanim_ornegi() -> String {
    BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn bars_values_autosize_kart_tanim_ornegi() -> String {
    BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn box_whisker_kart_tanim_ornegi() -> String {
    BOX_WHISKER_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn candlestick_kart_tanim_ornegi() -> String {
    CANDLESTICK_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn cursor_bind_kart_tanim_ornegi() -> String {
    CURSOR_BIND_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn ortak_kart_tekerlek_etkilesimi() -> bool {
    ortak_kart_etkileşimleri().tekerlek_etkileşimi
}

#[wasm_bindgen]
pub fn ortak_kart_secim_yakinlastir() -> bool {
    ortak_kart_etkileşimleri().seçim_yakınlaştır
}

#[wasm_bindgen]
pub fn ortak_kart_cift_tikla_tam_gorunum() -> bool {
    ortak_kart_etkileşimleri().çift_tıkla_tam_görünüm
}

#[wasm_bindgen]
pub fn ortak_kart_gorunum_gecmisi() -> bool {
    ortak_kart_etkileşimleri().görünüm_geçmişi
}

#[wasm_bindgen]
pub fn ortak_kart_dokunma_etkilesimi() -> bool {
    ortak_kart_etkileşimleri().dokunma_etkileşimi
}

#[wasm_bindgen]
pub fn kaynak_commit() -> String {
    "0e5812c504430f5c804e0f993376d8999b26cc34".to_string()
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn wasm_katalogu_pointer_akışını_boya_karesiyle_birleştirir() {
        let katalog = include_str!("../www/index.html");
        assert!(katalog.contains("function bekleyenPointerHareketiniİşle()"));
        assert!(katalog.contains("pointerAnimationFrame = requestAnimationFrame"));
        assert!(katalog.contains("pointerKaresi = requestAnimationFrame"));
    }

    #[test]
    fn wasm_yeniden_çizimde_sabit_arayüz_kabuğunu_korur() {
        let katalog = include_str!("../www/index.html");
        assert!(katalog.contains("const tamKurulum ="));
        assert!(katalog.contains("function svgYüzeyiniYerindeGüncelle"));
        assert!(katalog.contains("mevcutSvg.replaceChildren"));
        assert!(!katalog.contains("mevcutSvg.replaceWith(yeniSvg)"));
        assert!(katalog.contains("if (kart.bilgiKutusu && tamKurulum)"));
        assert!(katalog.contains("resizeAnimationFrame = requestAnimationFrame"));
        assert!(katalog.contains("if (boyutİmzası === sonResizeBoyutu) return"));
    }

    #[test]
    fn multi_bars_genel_seri_noktaları_yerine_tek_çubuk_vurgusu_kullanır() {
        let katalog = include_str!("../www/index.html");
        let başlangıç = katalog.find("[\"multi-bars-libraries-vertical\"");
        assert!(
            başlangıç.is_some(),
            "Multi Bars kart grubu katalogda bulunmalı"
        );
        let Some(başlangıç) = başlangıç else {
            return;
        };
        let bitiş = katalog[başlangıç..]
            .find("}])),")
            .map(|uzaklık| başlangıç + uzaklık);
        assert!(bitiş.is_some(), "Multi Bars kart grubu kapanmalı");
        let Some(bitiş) = bitiş else {
            return;
        };
        let grup = &katalog[başlangıç..bitiş];
        assert!(grup.contains("çubuk: kimlik !== \"multi-bars-non-justified\""));
        assert!(katalog.contains("!kart.çubuk && oturum.seri_gorunur(indeks)"));
        assert!(katalog.contains("oturum?.cubuk_vurusu"));
        assert!(katalog.contains("fill=\"rgba(255,255,255,.3)\""));
        assert!(katalog.contains("öncekiMultiBarsKaydırma.scrollLeft"));
        assert!(katalog.contains("yeniKaydırma.scrollLeft = öncekiMultiBarsKaydırmaKonumu.sol"));
        assert!(katalog.contains("const seriİmzası = kart.seriler.map(seri =>"));
        assert!(
            katalog.contains("öğe.style.opacity = oturum.seri_gorunur(indeks) ? \"1\" : \".4\"")
        );
        assert!(katalog.contains("data-multi-bars-kategori"));
        assert!(katalog.contains("oturum.multi_bars_kategorisini_ayarla(indeks, etkin)"));
    }

    #[test]
    fn resize_kartı_wasm_svg_üretir() {
        let oturum = KartOturumu::yeni("resize", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(800, 400);
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("Resize"));
        assert_eq!(kart_sayisi(), 365);
        assert!(resize_kart_tanim_ornegi().contains("resize_kartı(100)"));

        assert!(oturum.secim_yakinlastir(0.15, 0.35).is_ok());
        let yakın = oturum.svg(800, 400);
        assert!(yakın.contains("<circle"));
        assert!(ortak_kart_dokunma_etkilesimi());
        assert!(oturum.dokunmayi_baslat());
        assert!(oturum.dokunma_yakinlastir(0.5, 0.5, 1.25).is_ok());
        oturum.dokunmayi_bitir();
        assert!(oturum.tasimayi_baslat());
        assert!(oturum.tasi(0.05, 0.05).is_ok());
        oturum.tasimayi_bitir();
    }

    #[test]
    fn area_fill_wasm_üç_dolgu_üretir() {
        let oturum = KartOturumu::yeni("area-fill", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(960, 400);
        assert!(svg.contains("Area Fill"));
        assert_eq!(svg.matches("stroke=\"none\"").count(), 3);
        let boşta = oturum.bosta_lejant_degerleri();
        assert_eq!(boşta.len(), 3);
        assert!(boşta.iter().all(|değer| değer.is_finite()));
        let web = include_str!("../www/index.html");
        assert!(web.contains("oturum.bosta_lejant_degerleri()"));
        assert!(web.contains("data-rehber=\"amaç\""));
        assert!(web.contains("no-op wheel/pinch ana sahneyi yeniden üretmez"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn annotations_wasm_tekil_ve_aralık_işaretlerini_çizer() {
        let oturum = KartOturumu::yeni("annotations", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(1_920, 600);
        assert!(svg.contains("Annotations"));
        assert!(svg.contains("eqk_01"));
        assert!(svg.contains("tor_20"));
        assert!(svg.contains("rgb(76 175 80)"));
        assert!(svg.contains("rgb(255 193 7 / 20%)"));
        let alan = oturum.cizim_alani(1_920, 600);
        assert_eq!(alan.len(), 4);
        let [sol, sağ, _, alt] = alan.as_slice() else {
            return;
        };
        let deprem_x = sol + (sağ - sol) * (3.0 / 29.0);
        let vurgu = oturum.aciklama_vurgusu(1_920, 600, deprem_x as f32, *alt as f32 - 9.0);
        let vurgu = serde_json::from_str::<serde_json::Value>(&vurgu);
        assert!(vurgu.is_ok());
        let Ok(vurgu) = vurgu else {
            return;
        };
        assert_eq!(
            vurgu.get("etiket").and_then(serde_json::Value::as_str),
            Some("eqk_01")
        );
        assert_eq!(
            vurgu.get("aciklama").and_then(serde_json::Value::as_str),
            Some("Earthquake 01!")
        );
        assert_eq!(
            vurgu
                .get("etiketUzerinde")
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert!(
            vurgu
                .get("svg")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|svg| {
                    svg.contains("rgb(76 175 80)") && !svg.contains("stroke-dasharray")
                })
        );
        assert!(oturum.aciklama_vurgusu(1_920, 600, 0.0, 0.0).is_empty());
        assert!(oturum.secim_yakinlastir(0.34, 0.52).is_ok());
        let yakın = oturum.svg(1_920, 600);
        assert!(!yakın.contains("eqk_01"));
        assert!(yakın.contains("rgb(255 193 7 / 20%)"));
        assert!(annotations_kart_tanim_ornegi().contains("annotations_kartı"));
        let web = include_str!("../www/index.html");
        assert!(web.contains("id=\"aciklama-vurgu\""));
        assert!(web.contains("data-annotation-hover"));
        assert!(web.contains("İşaret stilleri veri modelinden"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn mass_spectrum_wasm_kaynak_csv_ve_eksenleri_çizer() {
        let oturum = KartOturumu::yeni("mass-spectrum", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(1_920, 600);
        assert!(svg.contains("Mass spectrum"));
        assert!(svg.contains("m/z"));
        assert!(svg.contains("relative abundance (%)"));
        assert!(svg.contains("#305CDE"));
        assert_eq!(oturum.gorunur_y_araligi(), vec![0.0, 100.0]);
        assert!(mass_spectrum_kart_tanim_ornegi().contains("mass_spectrum_kartı"));
        let web = include_str!("../www/index.html");
        assert!(web.contains("seriler: [{ etiket: \"Value\", renk: \"#305CDE\" }]"));
        assert!(web.contains("hover sıralı X üzerinde O(log N)"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn measure_datums_wasm_datum_ve_delta_çizer_temizler() {
        let Ok(mut oturum) = KartOturumu::yeni("measure-datums", 100) else {
            return;
        };
        assert!(oturum.olcum_datumu_ayarla(1, 0.25, 0.4));
        assert!(oturum.olcum_datumu_ayarla(2, 0.75, 0.6));
        let svg = oturum.svg(800, 400);
        assert!(svg.contains("blue"));
        assert!(svg.contains("orange"));
        assert!(svg.contains("dx:"));
        assert!(oturum.olcum_datumlarini_temizle());
        assert!(!oturum.svg(800, 400).contains("dx:"));
        let web = include_str!("../www/index.html");
        assert!(web.contains("document.activeElement === chart"));
        assert!(web.contains("datumTemizlendi || görünümDeğişti"));
        assert!(web.contains("wheel temizleme ve zoom tek redraw'da birleştirilir"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn multi_bars_wasm_dört_kaynak_yüzeyini_üretir() {
        for örnek in MultiBarsÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            assert!(oturum.svg(800, 400).contains("<rect"));
        }
        let Ok(mut oturum) = KartOturumu::yeni("multi-bars-libraries-vertical", 100) else {
            return;
        };
        assert!(
            oturum
                .seri_gorunurlugu_ayarla(0, false)
                .is_ok_and(|değişti| değişti)
        );
        assert!(!oturum.seri_gorunur(0));
        assert!(
            oturum
                .seri_renklerini_ayarla(1, "#123456".to_string(), Some("#abcdef".to_string()))
                .is_ok_and(|değişti| değişti)
        );
        let etiketler = oturum.multi_bars_kategori_etiketleri();
        assert_eq!(etiketler.len(), 13);
        let ilk_etiket = etiketler.first().cloned();
        assert!(
            oturum
                .multi_bars_kategorisini_ayarla(0, false)
                .is_ok_and(|değişti| değişti)
        );
        assert_eq!(
            oturum.multi_bars_kategori_durumlari().first().copied(),
            Some(0)
        );
        let svg = oturum.svg(800, 400);
        assert!(svg.contains("#123456"));
        assert!(svg.contains("#abcdef"));
        assert!(!oturum.seri_gorunur(0));
        if let Some(ilk_etiket) = ilk_etiket {
            assert!(!svg.contains(&ilk_etiket));
        }
        assert!(multi_bars_kart_tanim_ornegi().contains("multi_bars_kartı"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn nearest_non_null_wasm_dört_yüzeyi_üretir() {
        for örnek in NearestNonNullÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            assert!(oturum.svg(800, 400).starts_with("<svg"));
        }
        assert!(nearest_non_null_kart_tanim_ornegi().contains("en_yakın_null_olmayan_indeks"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn path_gap_clip_wasm_on_bes_kaynak_yuzeyini_uretir() {
        for örnek in PathGapClipÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(1_200, 600);
            assert!(svg.starts_with("<svg"), "{}", örnek.kimlik());
            if let Some(ilk_sözcük) = örnek.başlık().split_whitespace().next() {
                assert!(svg.contains(ilk_sözcük), "{}", örnek.kimlik());
            }
        }
        assert!(path_gap_clip_kart_tanim_ornegi().contains("path_gap_clip_kartı"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn pixel_align_wasm_iki_canlı_yüzeyi_yeniler() {
        for örnek in PixelAlignÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(mut oturum) = oturum else {
                continue;
            };
            assert!(oturum.pixel_align_adimi_ayarla(141).is_ok());
            let svg = oturum.svg(1_200, 400);
            assert!(svg.contains(örnek.başlık()));
        }
        assert!(pixel_align_kart_tanim_ornegi().contains("pixel_align_kartı"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn points_wasm_dört_kaynak_yüzeyini_üretir() {
        for örnek in PointsÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(1_200, 500);
            assert!(svg.contains(örnek.başlık()));
        }
        assert!(points_kart_tanim_ornegi().contains("points_kartı"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn scales_dir_ori_wasm_on_altı_kaynak_yüzeyini_üretir() {
        for örnek in ScalesDirOriÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            let (genişlik, yükseklik) = örnek.boyut();
            let svg = oturum.svg(genişlik, yükseklik);
            assert!(svg.contains(örnek.başlık()));
        }
        assert!(scales_dir_ori_kart_tanim_ornegi().contains("scales_dir_ori_kartı"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn scatter_wasm_iki_mode_iki_yüzeyini_ve_vuruşu_üretir() {
        for örnek in ScatterÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else { continue };
            assert!(oturum.svg(1_920, 600).contains(örnek.başlık()));
        }
        let Ok(oturum) = KartOturumu::yeni("scatter-bubble", 100) else {
            return;
        };
        let vuruş_adayı = oturum
            .grafik
            .çiz()
            .komutlar()
            .iter()
            .find_map(|komut| match komut {
                uplot_rs::Komut::Daire { merkez, .. } => Some(*merkez),
                _ => None,
            });
        let Some(merkez) = vuruş_adayı else { return };
        assert_eq!(
            oturum.dagilim_vurusu(1_920, 600, merkez.x, merkez.y).len(),
            8
        );
        assert!(
            !oturum
                .dagilim_vurus_etiketi(1_920, 600, merkez.x, merkez.y)
                .is_empty()
        );
        assert!(scatter_kart_tanim_ornegi().contains("scatter_kartı"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn scroll_sync_wasm_kaydırma_sonrası_yüzeyi_yeniden_eşler() {
        let oturum = KartOturumu::yeni("scroll-sync", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert!(oturum.svg(400, 200).contains(".syncRect()"));
        assert!(oturum.yuzeyi_esitle(10.0, 410.0, 400.0, 200.0));
        let önce = oturum.istemci_konumu(210.0, 510.0, 400, 200);
        assert!(oturum.yuzeyi_esitle(10.0, 110.0, 400.0, 200.0));
        let sonra = oturum.istemci_konumu(210.0, 210.0, 400, 200);
        assert_eq!(önce, sonra);
        assert!(scroll_sync_kart_tanim_ornegi().contains("scroll_sync_kartı"));
    }

    #[test]
    fn sine_stream_wasm_altı_seriyi_canlı_günceller() {
        let oturum = KartOturumu::yeni("sine-stream", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        let önce = oturum.svg(1_920, 600);
        assert!(önce.contains("6 series x 600 points @ 60fps"));
        assert!(oturum.sine_akisini_ilerlet().is_ok_and(|değişti| değişti));
        assert_ne!(oturum.svg(1_920, 600), önce);
        assert!(sine_stream_kart_tanim_ornegi().contains("SineAkışı"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn soft_minmax_wasm_beş_yüzeyi_ve_canlı_artışı_korur() {
        for örnek in SoftMinMaxÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(mut oturum) = oturum else {
                continue;
            };
            let aralık = oturum.gorunur_y_araligi();
            assert_eq!(aralık.len(), 2);
            if örnek.canlı_mı() {
                let önce = oturum.svg(400, 400);
                assert!(oturum.soft_minmax_ilerlet().is_ok_and(|değişti| değişti));
                assert_ne!(oturum.svg(400, 400), önce);
            } else {
                assert_eq!(aralık, vec![-1.0, 1.0]);
                assert!(matches!(oturum.soft_minmax_ilerlet(), Ok(false)));
            }
        }
        assert!(soft_minmax_kart_tanim_ornegi().contains("SoftMinMaxAkışı"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn sparklines_bars_wasm_iki_kaynak_yüzeyini_korur() {
        for örnek in SparklinesBarsÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            assert_eq!(oturum.gorunur_y_araligi(), vec![-25.0, 20.0]);
            let svg = oturum.svg(800, 400);
            assert!(svg.contains("gray"));
            assert!(svg.contains("stroke-dasharray="));
            assert!(svg.contains("<rect") || svg.contains("<polygon"));
        }
        assert!(sparklines_bars_kart_tanim_ornegi().contains("sparklines_bars_kartı"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn sparklines_wasm_yirmi_kompakt_yüzeyi_korur() {
        for örnek in SparklineÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(150, 30);
            assert!(svg.contains("width=\"150\" height=\"30\""));
            assert!(svg.contains("pink"));
            assert!(svg.contains("#03a9f4"));
            assert!(svg.contains("#b3e5fc"));
        }
        assert!(sparklines_kart_tanim_ornegi().contains("sparklines_kartı"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn sparse_wasm_üç_kaynak_yüzeyini_korur() {
        for örnek in SparseÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            assert_eq!(oturum.gorunur_y_araligi(), vec![100.0, 350.0]);
            let svg = oturum.svg(800, 200);
            assert!(svg.contains(örnek.başlık()));
            assert!(svg.contains("red"));
            if örnek == SparseÖrneği::ÖzelNoktalar {
                assert!(svg.contains("<rect"));
            }
        }
        assert!(sparse_kart_tanim_ornegi().contains("sparse_kartı"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn stacked_series_wasm_on_altı_kaynak_yüzeyini_korur() {
        for örnek in StackedSeriesÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            let (genişlik, yükseklik) = örnek.boyut();
            let svg = oturum.svg(genişlik, yükseklik);
            assert!(svg.contains(örnek.başlık()));
            assert!(svg.contains("<path") || svg.contains("<polygon"));
        }
        assert!(stacked_series_kart_tanim_ornegi().contains("stacked_series_kartı"));
        assert_eq!(kart_sayisi(), 365);

        let Ok(mut oturum) = KartOturumu::yeni("stacked-series-stacked-1", 100) else {
            return;
        };
        let önce = oturum.svg(800, 400);
        assert!(
            oturum
                .stacked_seri_gorunurlugu_ayarla(1, false)
                .is_ok_and(|değişti| değişti)
        );
        assert!(!oturum.seri_gorunur(1));
        assert_ne!(oturum.svg(800, 400), önce);
    }

    #[test]
    fn stream_data_wasm_üç_canlı_yüzeyi_korur() {
        for örnek in StreamDataÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(mut oturum) = oturum else {
                continue;
            };
            let önce = oturum.svg(1_600, 600);
            assert!(önce.contains(örnek.başlık()));
            assert!(önce.contains("red"));
            assert!(önce.contains("blue"));
            assert!(önce.contains("green"));
            assert!(oturum.stream_data_ilerlet().is_ok_and(|değişti| değişti));
            assert_ne!(oturum.svg(1_600, 600), önce);
        }
        assert!(stream_data_kart_tanim_ornegi().contains("StreamDataAkışı"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn svg_image_wasm_bağımsız_kaynak_belgesini_üretir() {
        let oturum = KartOturumu::yeni("svg-image", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(400, 200);
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("width=\"400\" height=\"200\""));
        assert!(svg.contains("test chart"));
        assert!(svg.contains("fill=\"pink\""));
        assert!(svg.contains("stroke=\"blue\""));
        assert!(svg_image_kart_tanim_ornegi().contains("bağımsız_svg"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn sync_cursor_wasm_beş_yüzeyi_ve_pub_sub_kuralını_korur() {
        for örnek in SyncCursorÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            assert!(
                oturum
                    .svg(örnek.boyut().0, örnek.boyut().1)
                    .contains(örnek.başlık())
            );
        }
        let mut köprü = SyncCursorKoprusu::yeni();
        assert_eq!(köprü.imlec_hedefleri(0), vec![1, 2]);
        assert!(köprü.dikey_imlec_senkron_mu(0, 1));
        assert!(!köprü.dikey_imlec_senkron_mu(0, 2));
        assert_eq!(köprü.seri_hedefi(3, 4, 0), 1);
        assert_eq!(köprü.seri_hedefi(3, 4, 1), -1);
        assert_eq!(köprü.fare_birak(0), vec![0, 1, 1, 1, 2, 1]);
        assert!(köprü.kilitli(2));
        assert!(sync_cursor_kart_tanim_ornegi().contains("SyncCursorGrubu"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn sync_y_zero_wasm_üç_kaynak_aşamasını_yeniler() {
        let oturum = KartOturumu::yeni("sync-y-zero", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        let ham = oturum.svg(800, 400);
        assert!(ham.contains("Sync Y Zero"));
        assert!(
            oturum
                .sync_y_zero_asamasini_ayarla("symmetric")
                .is_ok_and(|değişti| değişti)
        );
        let simetrik = oturum.svg(800, 400);
        assert_ne!(simetrik, ham);
        assert!(
            oturum
                .sync_y_zero_asamasini_ayarla("zero-aligned")
                .is_ok_and(|değişti| değişti)
        );
        assert_ne!(oturum.svg(800, 400), simetrik);
        assert!(sync_y_zero_kart_tanim_ornegi().contains("SyncYZeroAşaması"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn thin_bars_wasm_resmi_elli_beş_yüzeyi_çizer() {
        let örnekler = ThinBarsÖrneği::tümü();
        assert_eq!(örnekler.len(), 55);
        for örnek in örnekler {
            let oturum = KartOturumu::yeni(&örnek.kimlik(), 100);
            assert!(oturum.is_ok());
            let Ok(oturum) = oturum else {
                continue;
            };
            let (genişlik, yükseklik) = örnek.boyut();
            let svg = oturum.svg(genişlik, yükseklik);
            let başlık = örnek.başlık();
            let svg_başlığı = başlık.replace('&', "&amp;");
            assert!(svg.contains(&svg_başlığı));
            assert!(svg.contains("<rect"));
        }
        assert!(thin_bars_stroke_fill_kart_tanim_ornegi().contains("ThinBarsÖrneği"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn time_periods_wasm_üç_kaynak_yüzeyini_çizer() {
        for örnek in TimePeriodsÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok());
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(1_920, 400);
            assert!(svg.contains(örnek.başlık()));
            assert!(svg.contains("rgba(5, 141, 199, 1)"));
        }
        assert!(time_periods_kart_tanim_ornegi().contains("TimePeriodsÖrneği"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn timeline_discrete_wasm_dört_kaynak_yüzeyini_çizer() {
        for örnek in TimelineDiscreteÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok());
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(1_920, 300);
            let başlık = örnek.başlık().replace('&', "&amp;");
            assert!(svg.contains(&başlık));
            assert!(svg.contains("Device A"));
            assert!(svg.contains("<rect"));
        }
        assert!(timeline_discrete_kart_tanim_ornegi().contains("TimelineDiscreteÖrneği"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn timeseries_discrete_wasm_ortak_x_ile_iki_yüzeyi_çizer() {
        for örnek in TimeseriesDiscreteÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 50);
            assert!(oturum.is_ok());
            let Ok(oturum) = oturum else {
                continue;
            };
            let (genişlik, yükseklik) = örnek.boyut();
            let svg = oturum.svg(genişlik, yükseklik);
            assert!(svg.contains("<svg"));
            let beklenen = match örnek {
                TimeseriesDiscreteÖrneği::ZamanSerisi => 2,
                TimeseriesDiscreteÖrneği::AyrıkDurumlar => 4,
            };
            assert_eq!(oturum.en_yakin_noktalar(0.5).len(), beklenen);
        }
        assert!(timeseries_discrete_kart_tanim_ornegi().contains("TimeseriesDiscreteÖrneği"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn timezones_dst_wasm_elli_bir_kaynak_yüzeyini_çizer() {
        for örnek in TimezonesDstÖrneği::tümü() {
            let oturum = KartOturumu::yeni(&örnek.kimlik(), 100);
            assert!(oturum.is_ok());
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(600, 200);
            assert!(svg.contains("<svg"));
            assert!(svg.contains("red"));
        }
        assert!(timezones_dst_kart_tanim_ornegi().contains("TimezonesDstÖrneği"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn tooltips_closest_wasm_kaynak_tooltipini_üretir() {
        let oturum = KartOturumu::yeni("tooltips-closest", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        assert_eq!(oturum.seri_sayisi(), 4);
        assert!(!oturum.lejant_canli());
        let tooltip = oturum.en_yakin_tooltip(0.0, 0);
        assert_eq!(tooltip.len(), 4);
        assert!(
            tooltip
                .first()
                .is_some_and(|metin| metin.contains("567ad7455d"))
        );
        assert!(
            tooltip
                .get(2)
                .is_some_and(|url| url.contains("stat=instructions:u"))
        );
        assert!(oturum.svg(960, 400).contains("Summary-opt"));
        assert!(tooltips_closest_kart_tanim_ornegi().contains("en_yakın_tooltip"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn tooltips_wasm_gizli_seriyi_atlar_ve_imleci_yeniden_kurmada_korur() {
        let oturum = KartOturumu::yeni("tooltips", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert_eq!(oturum.seri_sayisi(), 2);
        assert!(oturum.seri_gorunur(0));
        assert!(!oturum.seri_gorunur(1));
        let bilgiler = oturum.tooltip_bilgileri(0.5, 0.5);
        assert_eq!(bilgiler.len(), 12);
        assert_eq!(bilgiler.first().map(String::as_str), Some("-1"));
        assert_eq!(bilgiler.get(6).map(String::as_str), Some("0"));
        assert_eq!(oturum.tooltip_yeniden_kurma_ms(), 2_000);
        assert!(oturum.tooltip_imlec_durumunu_koru());
        assert!(matches!(oturum.tooltip_yeniden_kur(), Ok(true)));
        assert!(oturum.svg(600, 400).contains("Tooltips"));
        assert!(tooltips_kart_tanim_ornegi().contains("tooltip_bilgileri"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn trendlines_wasm_uç_trendlerini_ve_yapışan_aralığı_üretir() {
        let oturum = KartOturumu::yeni("trendlines", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert_eq!(oturum.seri_sayisi(), 2);
        let svg = oturum.svg(800, 600);
        assert!(svg.contains("Trendlines"));
        assert_eq!(svg.matches("stroke-dasharray=\"5.00 5.00\"").count(), 2);
        assert!(matches!(oturum.secim_yakinlastir(0.151, 0.817), Ok(true)));
        assert_eq!(oturum.gorunur_x_araligi(), vec![15.0, 81.0]);
        assert!(trendlines_kart_tanim_ornegi().contains("seçim_yakınlaştır"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn update_cursor_select_resize_wasm_boyutu_ve_kalıcı_katmanları_günceller() {
        let oturum = KartOturumu::yeni("update-cursor-select-resize", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert_eq!(oturum.kaynak_boyutu(), 800);
        let svg = oturum.svg(800, 800);
        assert!(svg.contains("Maintain loc of cursor/select/hoverPts"));
        assert!(svg.contains("#607d8b"));
        assert!(svg.contains("#00000012"));
        assert!(matches!(oturum.boyut_senkron_ilerlet(), Ok(790)));
        assert_eq!(oturum.kaynak_boyutu(), 790);
        assert!(update_cursor_select_resize_kart_tanim_ornegi().contains("grafik.boyutu_ayarla"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn wind_direction_wasm_kaynak_vektörlerini_ve_eksenlerini_üretir() {
        let oturum = KartOturumu::yeni("wind-direction", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        assert_eq!(oturum.seri_sayisi(), 3);
        let svg = oturum.svg(800, 400);
        assert!(svg.contains("Wind Direction"));
        assert!(svg.contains("Temp °C"));
        assert!(svg.contains("Wind m/s"));
        assert_eq!(svg.matches("stroke=\"blue\"").count(), 139);
        assert!(wind_direction_kart_tanim_ornegi().contains("çekirdekte 15 px vektörlere"));
        assert_eq!(kart_sayisi(), 365);
    }

    #[test]
    fn y_scale_drag_wasm_eksenleri_çekirdek_üzerinden_sürükler() {
        let oturum = KartOturumu::yeni("y-scale-drag", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert!(oturum.eksen_vurusu(1_200, 600, 20.0, 300.0));
        assert!(oturum.eksen_suruklemeyi_baslat(1_200, 600, 20.0, 300.0));
        assert!(matches!(oturum.eksen_surukle(20.0, 340.0, false), Ok(true)));
        oturum.eksen_suruklemeyi_bitir();
        assert!(y_scale_drag_kart_tanim_ornegi().contains("Shift"));
    }

    #[test]
    fn y_shifted_series_wasm_aynı_ham_veriyi_iki_kipte_sunar() {
        let oturum = KartOturumu::yeni("y-shifted-series", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        let kaydırılmış = oturum.svg(960, 400);
        assert!(kaydırılmış.contains("Y-shifted Series"));
        assert!(kaydırılmış.contains("Core #4"));
        assert_eq!(oturum.gorunur_y_araligi(), vec![0.0, 30.0]);
        assert!(matches!(oturum.y_shifted_series_ilerlet(), Ok(true)));
        let normal = oturum.svg(960, 400);
        assert!(!normal.contains("Core #4"));
        assert_eq!(oturum.gorunur_y_araligi(), vec![0.0, 10.0]);
        assert!(y_shifted_series_kart_tanim_ornegi().contains("her 2 saniyede"));
    }

    #[test]
    fn add_del_series_wasm_seriyi_atomik_günceller() {
        let oturum = KartOturumu::yeni("add-del-series", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert_eq!(oturum.seri_sayisi(), 3);
        assert!(matches!(oturum.add_del_seri_ekle(), Ok(true)));
        assert_eq!(oturum.seri_sayisi(), 4);
        assert_eq!(
            oturum.seri_etiketleri().get(1).map(String::as_str),
            Some("Orange")
        );
        assert!(oturum.svg(960, 400).contains("#ffa500"));
        assert!(matches!(oturum.add_del_seri_sil(), Ok(true)));
        assert_eq!(oturum.seri_sayisi(), 3);
        assert!(add_del_series_kart_tanim_ornegi().contains("seri_ekle"));
    }

    #[test]
    fn align_data_wasm_join_ve_karma_yolu_üretir() {
        let maliyet = KartOturumu::yeni("align-data-cost", 100);
        assert!(maliyet.is_ok());
        let Ok(mut maliyet) = maliyet else {
            return;
        };
        assert_eq!(maliyet.seri_sayisi(), 25);
        let ayrı = maliyet.svg(960, 400);
        maliyet.bosluklari_birlestir_ayarla(true);
        assert_ne!(maliyet.svg(960, 400), ayrı);

        let karma = KartOturumu::yeni("align-data-line-bars", 100);
        assert!(karma.is_ok());
        let Ok(karma) = karma else {
            return;
        };
        let svg = karma.svg(960, 400);
        assert!(svg.contains("#ff0000"));
        assert_eq!(svg.matches("fill=\"#0000ff1a\"").count(), 4);
        assert!(align_data_kart_tanim_ornegi().contains("align_data_maliyet_kartı"));
    }

    #[test]
    fn scale_padding_wasm_on_üç_seriyi_üretir() {
        let oturum = KartOturumu::yeni("scale-padding", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(960, 400);
        assert!(svg.contains("Flat"));
        assert_eq!(svg.matches("fill=\"none\"").count(), 13);
    }

    #[test]
    fn zoom_wheel_wasm_kaynak_serilerini_üretir() {
        let oturum = KartOturumu::yeni("zoom-wheel", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert!(oturum.svg(600, 400).contains("Wheel Zoom &amp; Drag"));
        assert!(oturum.tekerlek(0.5, 0.5, 1.0, false).is_ok());
        assert!(oturum.yakinlastirilmis());
        let Ok(mut yalnız_x) = KartOturumu::yeni("zoom-wheel", 100) else {
            return;
        };
        let tam_x = yalnız_x.gorunur_x_araligi();
        let tam_y = yalnız_x.gorunur_y_araligi();
        assert!(
            yalnız_x
                .tekerlek_eksende(0.5, 0.5, 1.0, false, 1)
                .is_ok_and(|değişti| değişti)
        );
        assert_ne!(yalnız_x.gorunur_x_araligi(), tam_x);
        assert_eq!(yalnız_x.gorunur_y_araligi(), tam_y);
        assert!(zoom_fetch_kaniti());
        assert!(zoom_fetch_kanit_ornegi().contains("kaynak_yanıtını_uygula"));
        let başlangıç = oturum.zoom_ranger_oranlari();
        assert_eq!(başlangıç.len(), 4);
        assert!(oturum.zoom_ranger_sol(0.3));
        assert!(oturum.zoom_ranger_sag(0.7));
        assert!(oturum.zoom_ranger_tasi(0.1));
        assert!(zoom_ranger_grips_kanit_ornegi().contains("pencereyi_taşı"));
        assert!(oturum.zoom_ranger_alt(0.2));
        assert!(oturum.zoom_ranger_ust(0.8));
        assert_eq!(oturum.zoom_ranger_surukleme_ekseni(18.0, 15.0), 3);
        assert!(zoom_ranger_xy_kanit_ornegi().contains("alt_tutamağı_ayarla"));
        let varyasyonlar: Vec<_> = (0..5)
            .flat_map(|kip| {
                [false, true].map(|dist| {
                    (
                        oturum.zoom_varyasyon_ekseni(kip, dist, 30.0, 20.0),
                        oturum.zoom_varyasyon_ekseni(kip, dist, 3.0, 4.0),
                    )
                })
            })
            .collect();
        assert_eq!(varyasyonlar.len(), 10);
        assert!(varyasyonlar.iter().step_by(2).all(|(_, küçük)| *küçük != 0));
        assert!(
            varyasyonlar
                .iter()
                .skip(1)
                .step_by(2)
                .all(|(_, küçük)| *küçük == 0)
        );
        let web = include_str!("../www/index.html");
        assert!(!web.contains("data-kart=\"zoom-wheel\""));
        assert!(!web.contains("data-kart=\"zoom-touch\""));
        assert!(!web.contains("id=\"zoom-ortak-kaniti\""));
        assert!(!web.contains("id=\"zoom-ranger-proof\""));
        assert!(!web.contains("id=\"zoom-variation\""));
    }

    #[test]
    fn zoom_touch_wasm_kıstırmayı_çekirdekte_uygular() {
        let oturum = KartOturumu::yeni("zoom-touch", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert!(oturum.svg(960, 400).contains("Pinch Zoom &amp; Pan"));
        assert!(oturum.dokunmayi_baslat());
        assert!(oturum.dokunma_yakinlastir(0.5, 0.5, 1.25).is_ok());
        oturum.dokunmayi_bitir();
        assert!(oturum.yakinlastirilmis());
    }

    #[test]
    fn months_wasm_kaynak_grafiklerini_üretir() {
        for kimlik in ["months-no-leap", "months-leap", "months-russian"] {
            let oturum = KartOturumu::yeni(kimlik, 100);
            assert!(oturum.is_ok());
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(960, 240);
            assert!(svg.contains("20"));
            assert!(svg.contains("<path"));
        }
        let rusça = KartOturumu::yeni("months-russian", 100);
        let Ok(rusça) = rusça else {
            return;
        };
        assert!(rusça.svg(960, 600).contains("Янв"));
    }

    #[test]
    fn nice_scale_wasm_boyuta_duyarli_izgarayi_üretir() {
        let oturum = KartOturumu::yeni("nice-scale", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let geniş = oturum.svg(1920, 600);
        let dar = oturum.svg(600, 240);
        assert!(geniş.contains("Nice Scale &amp; Ticks (resize me)"));
        assert!(geniş.contains(">-150<"));
        assert!(geniş.contains(">250<"));
        assert!(dar.contains(">-200<"));
        assert!(dar.contains(">400<"));
    }

    #[test]
    fn no_data_wasm_33_kaynak_yüzeyini_üretir() {
        for örnek in NoDataÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(800, 400);
            assert!(svg.contains(örnek.başlık()));
        }
        let boş = KartOturumu::yeni("no-data-empty", 100);
        let Ok(boş) = boş else {
            return;
        };
        assert_eq!(boş.svg(800, 400).matches("<line").count(), 0);

        let web = include_str!("../www/index.html");
        assert_eq!(
            web.matches("<article class=\"kart\" data-kart=\"no-data\"")
                .count(),
            1
        );
        assert_eq!(web.matches("data-kart=\"no-data-").count(), 0);
        assert!(web.contains("id=\"no-data-variation\""));
        assert_eq!(web.matches("[\"no-data-").count(), 33);
    }

    #[test]
    fn cursor_snap_wasm_ızgara_oranını_çekirdekten_alır() {
        let oturum = KartOturumu::yeni("cursor-snap", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        assert_eq!(
            oturum.imlec_oranlarini_uyarla(0.14, 0.16, 100.0, 100.0),
            vec![0.1, 0.2]
        );
    }

    #[test]
    fn cursor_bind_wasm_ctrl_seçimini_yakınlaştırmadan_ayırır() {
        let oturum = KartOturumu::yeni("cursor-bind", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert!(oturum.svg(1_920, 600).contains("Cursor Bind"));
        assert!(oturum.ctrl_aciklama_etkin());
        assert_eq!(oturum.secimi_bitir(0.2, 0.6, true), Ok(2));
        assert!(!oturum.yakinlastirilmis());
        assert_eq!(oturum.secimi_bitir(0.2, 0.6, false), Ok(1));
        assert!(oturum.yakinlastirilmis());
        assert!(cursor_bind_kart_tanim_ornegi().contains("cursor_bind_kartı"));
    }

    #[test]
    fn cursor_tooltip_wasm_kaynak_verisini_üretir() {
        let oturum = KartOturumu::yeni("cursor-tooltip", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(600, 400);
        assert!(svg.contains("placement.js tooltip"));
        assert!(svg.contains("#008000"));
        assert_eq!(oturum.en_yakin_noktalar(0.5), vec![4.0, 65.0]);
        assert!(cursor_tooltip_kart_tanim_ornegi().contains("cursor_tooltip_kartı"));
    }

    #[test]
    fn custom_scales_wasm_üç_farklı_geometri_üretir() {
        let mut svgler = Vec::new();
        for kimlik in [
            "custom-scales-linear",
            "custom-scales-log-log",
            "custom-scales-weibull",
        ] {
            let oturum = KartOturumu::yeni(kimlik, 100);
            assert!(oturum.is_ok());
            let Ok(oturum) = oturum else { return };
            let svg = oturum.svg(800, 800);
            assert!(svg.contains("#ffa50030"));
            assert_eq!(svg.matches("fill=\"#000000\"").count(), 20);
            assert!(svg.contains("stroke-dasharray=\"10.00 5.00\""));
            svgler.push(svg);
        }
        assert_ne!(svgler.first(), svgler.get(1));
        assert_ne!(svgler.get(1), svgler.get(2));
        assert!(custom_scales_kart_tanim_ornegi().contains("CustomScaleÖrneği"));
    }

    #[test]
    fn data_smoothing_wasm_dört_kaynak_alt_grafiğini_üretir() {
        for (kimlik, başlık) in [
            ("data-smoothing-raw", "Taxi Trips (raw)"),
            ("data-smoothing-sgg", "Savitzky-Golay"),
            ("data-smoothing-asap", "Taxi Trips (ASAP FFT)"),
            (
                "data-smoothing-moving-average",
                "Taxi Trips (Moving Avg 300)",
            ),
        ] {
            let oturum = KartOturumu::yeni(kimlik, 100);
            assert!(oturum.is_ok());
            let Ok(oturum) = oturum else { return };
            let svg = oturum.svg(960, 300);
            assert!(svg.contains(başlık));
            assert!(svg.contains("#ff0000"));
        }
        assert!(data_smoothing_kart_tanim_ornegi().contains("SmoothingÖrneği::Asap"));
    }

    #[test]
    fn draw_hooks_wasm_kaynak_çizim_katmanlarını_üretir() {
        let oturum = KartOturumu::yeni("draw-hooks", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else { return };
        let svg = oturum.svg(600, 400);
        assert!(svg.contains("Draw Hooks"));
        assert!(svg.contains("Time to Draw: 0ms"));
        assert!(svg.contains("#ff333333"));
        assert_eq!(svg.matches("fill=\"#ff3333\"").count(), 9);
        assert!(draw_hooks_kart_tanim_ornegi().contains("draw_hooks_kartı"));
    }

    #[test]
    fn focus_cursor_wasm_dört_alt_grafiği_ve_canlı_odağı_üretir() {
        for kimlik in [
            "focus-cursor",
            "focus-cursor-dynamic",
            "focus-cursor-width-stroke",
            "focus-cursor-performance-300",
        ] {
            assert!(KartOturumu::yeni(kimlik, 100).is_ok());
        }
        let oturum = KartOturumu::yeni("focus-cursor-width-stroke", 100);
        let Ok(mut oturum) = oturum else { return };
        assert!(oturum.imlec_odagini_guncelle(0.5, 2.0 / 3.0, 500.0));
        let svg = oturum.svg(960, 400);
        assert!(svg.contains("#ff00ff"));
        assert!(focus_cursor_kart_tanim_ornegi().contains("FocusÖrneği::Dinamik"));
    }

    #[test]
    fn gradients_wasm_beş_alt_grafiği_ve_canlı_nokta_rengini_üretir() {
        for kimlik in [
            "gradients-horizontal-stroke",
            "gradients-vertical-stroke",
            "gradients-vertical-arcsinh",
            "gradients-scale-fills",
            "gradients-relative-fill",
        ] {
            let oturum = KartOturumu::yeni(kimlik, 100);
            let Ok(oturum) = oturum else { continue };
            assert!(oturum.svg(800, 600).contains("linearGradient"));
        }
        let oturum = KartOturumu::yeni("gradients-horizontal-stroke", 100);
        let Ok(oturum) = oturum else { return };
        assert_eq!(oturum.imlec_seri_renkleri(0.0), ["#ff0000"]);
        assert_eq!(oturum.imlec_seri_renkleri(0.25), ["#ffa500"]);
        assert!(gradients_kart_tanim_ornegi().contains("GradientÖrneği::ÖlçekDolguları"));
    }

    #[test]
    fn grid_over_series_wasm_ızgarayı_serilerin_üstünde_üretir() {
        let oturum = KartOturumu::yeni("grid-over-series", 100);
        let Ok(oturum) = oturum else { return };
        let svg = oturum.svg(960, 400);
        let seri = svg.rfind("fill=\"#42A5F5\"");
        let ızgara = svg.rfind("stroke=\"#00000033\"");
        assert!(matches!((seri, ızgara), (Some(seri), Some(ızgara)) if ızgara > seri));
        assert!(grid_over_series_kart_tanim_ornegi().contains("ÇizimSırası"));
    }

    #[test]
    fn high_low_bands_wasm_on_iki_kaynak_grafiği_üretir() {
        for örnek in HighLowBandsÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            let Ok(oturum) = oturum else { continue };
            assert!(oturum.svg(960, 400).contains(örnek.başlık()));
        }
        assert!(high_low_bands_kart_tanim_ornegi().contains("FarklıYollar"));
    }

    #[test]
    fn latency_heatmap_wasm_beş_kaynak_grafiği_üretir() {
        for örnek in LatencyHeatmapÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            let Ok(oturum) = oturum else { continue };
            assert!(oturum.svg(960, 400).contains(örnek.başlık()));
        }
        assert!(latency_heatmap_kart_tanim_ornegi().contains("Kovalanmış"));
    }

    #[test]
    fn line_paths_wasm_sekiz_kaynak_grafiği_üretir() {
        for örnek in LinePathsÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            let Ok(oturum) = oturum else { continue };
            assert!(oturum.svg(960, 240).contains(örnek.başlık()));
        }
        assert!(line_paths_kart_tanim_ornegi().contains("MonotonKübik"));
    }

    #[test]
    fn log_scales_wasm_iki_kaynak_grafiği_üretir() {
        for örnek in LogScalesÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            let Ok(oturum) = oturum else { continue };
            let svg = oturum.svg(960, 360);
            assert!(svg.contains(örnek.başlık()));
            assert!(svg.contains("#d0b283"));
        }
        assert!(log_scales_kart_tanim_ornegi().contains("Logaritmik"));
    }

    #[test]
    fn log_scales2_wasm_on_iki_kaynak_yüzeyi_üretir() {
        for örnek in LogScales2Örneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            let Ok(oturum) = oturum else { continue };
            let svg = oturum.svg(960, 400);
            let kaçışlı_başlık = örnek.başlık().replace('>', "&gt;");
            assert!(svg.contains(&kaçışlı_başlık), "{}", örnek.kimlik());
        }
        let log2 = KartOturumu::yeni("log-scales2-skip-log2", 100);
        let Ok(log2) = log2 else { return };
        assert!(log2.svg(960, 400).contains("2^20"));
        assert!(log_scales2_kart_tanim_ornegi().contains("GenişLog10"));
    }

    #[test]
    fn missing_data_wasm_iki_kaynak_alt_grafiğini_üretir() {
        let ana = KartOturumu::yeni("missing-data-null", 100);
        assert!(ana.is_ok());
        let Ok(ana) = ana else {
            return;
        };
        let svg = ana.svg(960, 400);
        assert!(svg.contains("Missing Data (null values)"));
        assert!(svg.contains("MB"));

        let boşluk = KartOturumu::yeni("missing-data-x-gap", 100);
        assert!(boşluk.is_ok());
        let Ok(boşluk) = boşluk else {
            return;
        };
        assert!(boşluk.svg(960, 400).contains("adjacent points"));
    }

    #[test]
    fn dependent_scale_wasm_iki_sıcaklık_eksenini_üretir() {
        let oturum = KartOturumu::yeni("dependent-scale", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(600, 400);
        assert!(svg.contains("° F"));
        assert!(svg.contains("° C"));
    }

    #[test]
    fn arcsinh_wasm_eşiği_çekirdekte_değiştirir() {
        let oturum = KartOturumu::yeni("arcsinh-scales", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        let önce = oturum.svg(960, 400);
        assert!(oturum.y_arcsinh_esigi_ayarla("y", 0.001));
        assert_ne!(oturum.svg(960, 400), önce);
    }

    #[test]
    fn axis_control_wasm_seyrek_sahne_ve_eksenleri_üretir() {
        let oturum = KartOturumu::yeni("axis-control", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(1048, 600);
        assert!(svg.contains("X Axis Label"));
        assert!(svg.contains("Y Axis Label"));
        assert!(svg.len() < 500_000);
    }

    #[test]
    fn axis_autosize_wasm_dinamik_çarpanda_eksenleri_yeniden_ölçer() {
        let oturum = KartOturumu::yeni("axis-autosize", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        let önceki = oturum.cizim_alani(1048, 600);
        assert!(oturum.axis_autosize_carpani_ayarla(1e9).is_ok());
        let sonraki = oturum.cizim_alani(1048, 600);
        assert!(
            sonraki
                .first()
                .zip(önceki.first())
                .is_some_and(|(yeni, eski)| yeni > eski)
        );
        assert!(oturum.svg(1048, 600).contains("500000000000.00"));
    }

    #[test]
    fn axis_indicators_wasm_üç_ölçeği_ve_göstergeyi_üretir() {
        let oturum = KartOturumu::yeni("axis-indicators", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        assert!(oturum.eksen_gostergeleri_etkin());
        assert_eq!(oturum.svg(1200, 600).matches("fill=\"none\"").count(), 3);
        assert_eq!(oturum.seri_gorunur_y_araligi(2).len(), 2);
    }

    #[test]
    fn bars_grouped_stacked_wasm_on_alt_grafiği_üretir() {
        for örnek in ÇubukÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(800, 500);
            assert!(svg.contains("Group A"));
            assert!(svg.matches("<rect").count() >= 2);
        }
    }

    #[test]
    fn bars_values_autosize_wasm_iki_yönü_üretir() {
        for kimlik in [
            "bars-values-autosize-vertical",
            "bars-values-autosize-horizontal",
        ] {
            let oturum = KartOturumu::yeni(kimlik, 100);
            assert!(oturum.is_ok(), "{kimlik}");
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(1_275, 600);
            assert!(svg.contains("#00ff0022"));
            assert!(svg.matches("#00000033").count() >= 12);
        }
    }

    #[test]
    fn box_whisker_wasm_kaynak_kutusunu_ve_vurgu_sütununu_üretir() {
        let oturum = KartOturumu::yeni("box-whisker-01_run1k", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(800, 400);
        assert!(svg.contains("stroke-dasharray=\"4.00 4.00\""));
        let vuruş = oturum.kutu_biyik_vurusu(800, 400, 76.0, 120.0);
        assert!(vuruş.is_empty() || vuruş.len() == 10);
    }

    #[test]
    fn candlestick_wasm_ohlc_ve_hacmi_üretir() {
        let oturum = KartOturumu::yeni("candlestick-ohlc", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(1_920, 600);
        assert!(svg.contains("#4ab650"));
        assert!(svg.contains("#e54245"));
        assert_eq!(oturum.kutu_biyik_vurusu(1_920, 600, 76.0, 100.0).len(), 10);
    }
}
