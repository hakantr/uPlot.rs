//! GPUI masaüstü chart kataloğu; dağıtılan bileşeni kullanan örnek uygulama.

use gpui::{
    ClickEvent, Context, Entity, Focusable, FontWeight, IntoElement, Render, SharedString, Task,
    Window, div, prelude::*, px, rgb, rgba,
};
use ortak_bilesenler::{
    Anahtar, AnahtarOlayi, CubukAyarlari, Dugme, DugmeBoyutu, DugmeTuru, MetinAlani,
    MetinAlaniOlayi, PlatformPencere,
};
use std::time::{Duration, Instant};
use uplot_rs::gpui::{GpuiGrafik, GpuiGrafikOlayı};
use uplot_rs::{
    ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ, ALIGN_DATA_KART_TANIM_ÖRNEĞİ, ANNOTATIONS_KART_TANIM_ÖRNEĞİ,
    ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ, AREA_FILL_KART_TANIM_ÖRNEĞİ, AXIS_AUTOSIZE_ARALIK_MS,
    AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ, AXIS_CONTROL_KART_TANIM_ÖRNEĞİ,
    AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ, AlignDataÖrneği, AxisAutosizeAkışı,
    BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ, BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    BOX_WHISKER_KART_TANIM_ÖRNEĞİ, BoyutSenkronAkışı, CANDLESTICK_KART_TANIM_ÖRNEĞİ,
    CURSOR_BIND_KART_TANIM_ÖRNEĞİ, CURSOR_SNAP_KART_TANIM_ÖRNEĞİ, CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ,
    CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ, CustomScaleÖrneği, DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ,
    DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ, DRAW_HOOKS_KART_TANIM_ÖRNEĞİ, EtkileşimSeçenekleri,
    FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ, FocusÖrneği, GRADIENTS_KART_TANIM_ÖRNEĞİ,
    GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ, GradientÖrneği, Grafik, HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ,
    HighLowBandsÖrneği, LATENCY_HEATMAP_KART_TANIM_ÖRNEĞİ, LINE_PATHS_KART_TANIM_ÖRNEĞİ,
    LOG_SCALES_KART_TANIM_ÖRNEĞİ, LOG_SCALES2_KART_TANIM_ÖRNEĞİ, LatencyHeatmapÖrneği,
    LinePathsÖrneği, LogScales2Örneği, LogScalesÖrneği, MASS_SPECTRUM_KART_TANIM_ÖRNEĞİ,
    MEASURE_DATUMS_KART_TANIM_ÖRNEĞİ, MISSING_DATA_KART_TANIM_ÖRNEĞİ, MONTHS_KART_TANIM_ÖRNEĞİ,
    MULTI_BARS_KART_TANIM_ÖRNEĞİ, MissingDataÖrneği, MultiBarsÖrneği,
    NEAREST_NON_NULL_KART_TANIM_ÖRNEĞİ, NICE_SCALE_KART_TANIM_ÖRNEĞİ, NO_DATA_KART_TANIM_ÖRNEĞİ,
    NearestNonNullÖrneği, NoDataÖrneği, PATH_GAP_CLIP_KART_TANIM_ÖRNEĞİ,
    PIXEL_ALIGN_KART_TANIM_ÖRNEĞİ, POINTS_KART_TANIM_ÖRNEĞİ, PathGapClipÖrneği, PixelAlignAkışı,
    PixelAlignÖrneği, PointsÖrneği, RESIZE_KART_TANIM_ÖRNEĞİ, SCALE_PADDING_KART_TANIM_ÖRNEĞİ,
    SCALES_DIR_ORI_KART_TANIM_ÖRNEĞİ, SCATTER_KART_TANIM_ÖRNEĞİ, SCROLL_SYNC_KART_TANIM_ÖRNEĞİ,
    SINE_STREAM_KART_TANIM_ÖRNEĞİ, SOFT_MINMAX_KART_TANIM_ÖRNEĞİ,
    SPARKLINES_BARS_KART_TANIM_ÖRNEĞİ, SPARKLINES_KART_TANIM_ÖRNEĞİ, SPARSE_KART_TANIM_ÖRNEĞİ,
    STACKED_SERIES_KART_TANIM_ÖRNEĞİ, STREAM_DATA_ARALIK_MS, STREAM_DATA_KART_TANIM_ÖRNEĞİ,
    SVG_IMAGE_KART_TANIM_ÖRNEĞİ, SYNC_CURSOR_KART_TANIM_ÖRNEĞİ, SYNC_Y_ZERO_KART_TANIM_ÖRNEĞİ,
    ScalesDirOriÖrneği, ScatterÖrneği, SineAkışı, SmoothingÖrneği, SoftMinMaxAkışı,
    SoftMinMaxÖrneği, SparklinesBarsÖrneği, SparklineÖrneği, SparseÖrneği, StackedSeriesÖrneği,
    StreamDataGrubu, StreamDataÖrneği, SyncCursorGrubu, SyncCursorÖrneği, SyncYZeroAşaması,
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
    focus_cursor_kartları, focus_cursor_kartı, gradients_kartları, gradients_kartı,
    grid_over_series_kartı, high_low_bands_kartı, latency_heatmap_kartı, line_paths_kartı,
    log_scales_kartı, log_scales2_kartı, mass_spectrum_kartı, measure_datums_kartı,
    missing_data_kartları, missing_data_null_kartı, months_artık_yılsız_kartı, months_kartları,
    multi_bars_kartı, nearest_non_null_kartı, nice_scale_kartı, no_data_kartı,
    ortak_kart_etkileşimleri, path_gap_clip_kartları, path_gap_clip_kartı, pixel_align_kartları,
    pixel_align_kartı, points_kartları, points_kartı, resize_kartı, scale_padding_kartı,
    scales_dir_ori_kartları, scales_dir_ori_kartı, scatter_kartı, scroll_sync_kartı,
    sine_stream_kartı, soft_minmax_kartları, soft_minmax_kartı, sparklines_bars_kartları,
    sparklines_bars_kartı, sparklines_kartları, sparklines_kartı, sparse_kartları, sparse_kartı,
    stacked_series_kartları, stacked_series_kartı, stacked_series_kartı_görünür, stream_data_kartı,
    svg_image_kartı, sync_cursor_kartı, sync_y_zero_aralıkları, sync_y_zero_kartı,
    thin_bars_stroke_fill_kartları, thin_bars_stroke_fill_kartı, time_periods_kartları,
    time_periods_kartı, timeline_discrete_kartları, timeline_discrete_kartı,
    timeseries_discrete_kartları, timeseries_discrete_kartı, timezones_dst_kartları,
    timezones_dst_kartı, tooltips_closest_kartı, tooltips_kartı, trendlines_kartı,
    update_cursor_select_resize_kartı, wind_direction_kartı, y_scale_drag_kartı,
    y_shifted_series_kartı, ÇubukYönü, ÇubukÖrneği, İmleçBağSeçenekleri,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum KartKimliği {
    AddDelSeries,
    AlignDataCost,
    Resize,
    Annotations,
    AreaFill,
    ScalePadding,
    Months,
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
    SvgImage,
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
    HighLowBands(HighLowBandsÖrneği),
    LatencyHeatmap(LatencyHeatmapÖrneği),
    LinePaths(LinePathsÖrneği),
    LogScales(LogScalesÖrneği),
    LogScales2(LogScales2Örneği),
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

impl KartKimliği {
    fn başlık(self) -> &'static str {
        match self {
            Self::AddDelSeries => "Add/Delete Series",
            Self::AlignDataCost => "Align Data · 2 related surfaces",
            Self::Resize => "Resize · sayısal x ölçeği",
            Self::Annotations => "Annotations",
            Self::AreaFill => "Area Fill",
            Self::ScalePadding => "Scale Padding · Flat",
            Self::Months => "Months · takvim ve yerelleştirme",
            Self::NiceScale => "Nice Scale & Ticks",
            Self::NoData => "No Data · 33 seçenek",
            Self::PathGapClip => "Path & Gap Clipping · 15 yüzey",
            Self::PixelAlign => "Pixel Align · canlı A/B",
            Self::Points => "Points · 4 yüzey",
            Self::ScalesDirOri => "Scales Direction & Orientation · 16 yüzey",
            Self::Scatter => "Scatter & Bubble · 2 bağımsız yüzey",
            Self::ScrollSync => "Scroll syncRect()",
            Self::SineStream => "6 series x 600 points @ 60fps",
            Self::SoftMinMax(_) => "Soft Min/Max · 5 ilişkili yüzey",
            Self::SparklinesBars(_) => "Sparkline + Floating Bars · 2 ilişkili yüzey",
            Self::Sparklines(_) => "Sparklines · 10×2 tablo",
            Self::Sparse(_) => "Sparse · 3 pathBuilder",
            Self::StackedSeries(_) => "Stacked Series · 16 yüzey",
            Self::StreamData(_) => "Data Stream · 3 yüzey",
            Self::SvgImage => "uPlot to image PoC",
            Self::SyncCursor => "Sync Cursor",
            Self::SyncYZero(_) => "Sync Y Zero",
            Self::ThinBars(_) => "Thin bar stroke & fill",
            Self::TimePeriods(_) => "Time Periods",
            Self::TimelineDiscrete(_) => "Timeline / Discrete",
            Self::TimeseriesDiscrete => "TimeSeries + Discrete",
            Self::TimezonesDst => "Timezones & DST",
            Self::TooltipsClosest => "Summary-opt",
            Self::Tooltips => "Tooltips",
            Self::Trendlines => "Trendlines",
            Self::UpdateCursorSelectResize => "Maintain loc of cursor/select/hoverPts",
            Self::WindDirection => "Wind Direction",
            Self::YScaleDrag => "Draggable x & y scales",
            Self::YShiftedSeries => "Y-shifted Series",
            Self::CursorBind => "Cursor Bind (try Ctrl + drag)",
            Self::CursorSnap => "Cursor Snap · 10×10 grid",
            Self::CursorTooltip => "Cursor Tooltip w/placement.js",
            Self::CustomScales => "Custom Scales · 3 independent surfaces",
            Self::DataSmoothing => "Data Smoothing · 4 independent surfaces",
            Self::DrawHooks => "Draw Hooks",
            Self::FocusCursor => "Focus Cursor · 4 related surfaces",
            Self::Gradients => "Gradients · 5 related surfaces",
            Self::GridOverSeries => "Grid Over Series",
            Self::HighLowBands(örnek) => örnek.başlık(),
            Self::LatencyHeatmap(örnek) => örnek.başlık(),
            Self::LinePaths(örnek) => örnek.başlık(),
            Self::LogScales(örnek) => örnek.başlık(),
            Self::LogScales2(örnek) => örnek.başlık(),
            Self::MassSpectrum => "Mass Spectrum",
            Self::MeasureDatums => "Measure / Datums",
            Self::MultiBars(örnek) => örnek.başlık(),
            Self::NearestNonNull => "Nearest Non-Null · 5 davranış",
            Self::MissingData => "Missing Data · 2 related surfaces",
            Self::DependentScale => "Derived Scale · °F / °C",
            Self::ArcSinhScales => "ArcSinh Y Scale",
            Self::AxisControl => "Axis Control",
            Self::AxisAutosize => "Axis AutoSize",
            Self::AxisIndicators => "Axis indicators",
            Self::Bars(_) => "Bars · Grouped / Stacked · 10 yüzey",
            Self::BarsValuesAutosize(_) => "Bars Values AutoSize · 2 yüzey",
            Self::BoxWhisker(_) => "Box & Whisker · 17 yüzey",
            Self::Candlestick => "Candlestick Chart · Gold",
        }
    }

    fn kaynak(self) -> &'static str {
        match self {
            Self::AddDelSeries => {
                "add-del-series.html · addSeries/delSeries/setData · kaynak Y indeksi 1"
            }
            Self::AlignDataCost => "align-data.html · NULL_EXPAND maliyeti + aligned line/bars",
            Self::Resize => "resize.html + zoom-wheel.html + zoom-touch.html",
            Self::Annotations => {
                "annotations.html · X çizgisi/aralığı · üst/alt etiket · görünürlük kırpması"
            }
            Self::AreaFill => {
                "area-fill.html · kaynakla aynı veri üreteci · ortak Resize etkileşim profili"
            }
            Self::ScalePadding => {
                "scale-padding.html · 13 düz seri · kaynakla aynı değer düzeyleri"
            }
            Self::Months => {
                "months.html + months-ru.html · 3 ilişkili yüzey · UTC ayları, artık yıl, Rusça fmtDate · sabit kanıt tohumu"
            }
            Self::NiceScale => {
                "nice-scale.html · pencere/panel boyutuna bağlı niceScale/niceNum Y aralığı ve artımı"
            }
            Self::NoData => {
                "no-data.html · tek kartta 33 boş, tek noktalı, düz ve hassas ölçek seçeneği"
            }
            Self::PathGapClip => {
                "path-gap-clip.html · 15 null/undefined, band, stepped ve piksel yüzeyi"
            }
            Self::PixelAlign => {
                "pixel-align.html · 2 eşzamanlı yüzey · ortak 1 Hz halka veri + animation-frame X saati"
            }
            Self::Points => {
                "points.html · 4 eşzamanlı yüzey · randomWalk.js · points.space, paths:null ve points.filter"
            }
            Self::ScalesDirOri => {
                "scales-dir-ori.html · 16 eşzamanlı yüzey · scale.dir, scale.ori ve axis.side"
            }
            Self::Scatter => {
                "scatter.html · 2 bağımsız mode:2 yüzey · toplu scatter yolu ve uzamsal bubble vuruşu"
            }
            Self::ScrollSync => "scroll-sync.html · syncRect() · kaydırmada istemci/sahne eşlemesi",
            Self::SineStream => "sine-stream.html · Box–Muller yürüyüşü · requestAnimationFrame",
            Self::SoftMinMax(_) => {
                "soft-minmax.html · rangeNum soft/hard/pad/mode · kaynak dataMax++"
            }
            Self::SparklinesBars(_) => {
                "sparklines-bars.html · sparkline + yüzen çubuklar + ölçek gradyanı"
            }
            Self::Sparklines(_) => "sparklines.html · kaynak CSV · 150×30 eksensiz kompakt yüzey",
            Self::Sparse(_) => "sparse.html · sparse.json · yerleşik/özel nokta/saf moveTo yolları",
            Self::StackedSeries(_) => {
                "stacked-series.html · stack.js · yığma, yüzde, grup ve karma veri"
            }
            Self::StreamData(_) => "stream-data.html · bench/data.json · setData canlı akışı",
            Self::SvgImage => "svg-image.html · canvas + DOM → bağımsız görüntü PoC",
            Self::SyncCursor => "sync-cursor.html · sync.js · bench/data.json · 5 eşzamanlı yüzey",
            Self::SyncYZero(_) => {
                "sync-y-zero.html · ham → simetrik → ortak sıfır pikseli · 3 sol Y ekseni"
            }
            Self::ThinBars(_) => {
                "thin-bars-stroke-fill.html · paths/bars.js · 55 vuruş/dolgu geometrisi"
            }
            Self::TimePeriods(_) => {
                "time-periods.html · traffic.json · saatlik/aylık/günlük dönem karşılaştırması"
            }
            Self::TimelineDiscrete(_) => {
                "timeline-discrete.html · distr.js · quadtree.js · null/undefined şeritleri"
            }
            Self::TimeseriesDiscrete => {
                "timeseries-discrete.html · iki yüzey · ortak X imleci · birleşik lejant"
            }
            Self::TimezonesDst => {
                "timezones-dst.html · tzDate · 51 etkin UTC/London/Chicago yüzeyi"
            }
            Self::TooltipsClosest => {
                "tooltips-closest.html · rustc-perf.json · en yakın seri ve commit karşılaştırması"
            }
            Self::Tooltips => {
                "tooltips.html · imleç ve görünür seri kutuları · 2 sn imleç durum koruması"
            }
            Self::Trendlines => {
                "trendlines.html · drawSeries uç trendleri · veri değerlerine yapışan X aralığı"
            }
            Self::UpdateCursorSelectResize => {
                "update-cursor-select-resize.html · setSize sırasında seçim, kilitli imleç ve hover noktası oranları"
            }
            Self::WindDirection => {
                "wind-direction.html · 143 saatlik kaynak veri · 15 px özel yön vektörleri"
            }
            Self::YScaleDrag => {
                "y-scale-drag.html · bağımsız X/Y eksen sürükleme · Shift ile büyüt/daralt"
            }
            Self::YShiftedSeries => {
                "y-shifted-series.html · aynı ham veriyle 2 sn normal/kaydırılmış kip"
            }
            Self::CursorBind => {
                "cursor-bind.html · Ctrl+sürükle sarı açıklama seçimi · yakınlaştırma yok"
            }
            Self::CursorSnap => "cursor-snap.html · çekirdek 10×10 piksel imleç ızgarası",
            Self::CursorTooltip => "cursor-tooltip.html · sınırlara duyarlı canlı bilgi kutusu",
            Self::CustomScales => {
                "custom-scales.html · aynı sayfada doğrusal, log-log ve özel Weibull ölçeği"
            }
            Self::DataSmoothing => {
                "data-smoothing.html · taxi-trips + SGG + ASAP FFT + Moving Avg 300"
            }
            Self::DrawHooks => "draw-hooks.html · drawClear/drawSeries/draw plugin hooks",
            Self::FocusCursor => "focus-cursor.html · cursor.focus + setSeries",
            Self::Gradients => "gradients.html · scaleGradient + cursor point colors",
            Self::GridOverSeries => "grid-over-series.html · drawOrder: series, axes",
            Self::HighLowBands(_) => "high-low-bands.html · yönlü line/step/spline/bar bantları",
            Self::LatencyHeatmap(_) => {
                "latency-heatmap.html · rand.js · draw hook, mode-2 ve histogram kovaları"
            }
            Self::LinePaths(_) => {
                "line-paths.html · null/linear/spline/stepped/bars + kaynak spline2"
            }
            Self::LogScales(_) => {
                "log-scales.html · 12 Minecraft sunucusu · log10 ve doğrusal Y ölçeği"
            }
            Self::LogScales2(_) => {
                "log-scales2.html · log2/log10, ters yön, null ve kısmi büyüklükler"
            }
            Self::MassSpectrum => {
                "mass-spectrum.html · 41.986 kaynak CSV noktası · özel düz Y aralığı"
            }
            Self::MeasureDatums => "measure-datums.html · 1/2 datum · Esc temizle",
            Self::MultiBars(_) => {
                "multi-bars.html · benchmark grupları · negatif ve durum renkli çubuklar"
            }
            Self::NearestNonNull => {
                "nearest-non-null.html · 5 bağımsız yüzeyde null/proximity/cursor karşılaştırması"
            }
            Self::MissingData => "missing-data.html · resmî veri ve iki kaynak alt grafiği",
            Self::DependentScale => {
                "dependent-scale.html · Fahrenheit'tan türetilen Celsius ekseni"
            }
            Self::ArcSinhScales => "arcsinh-scales.html · değiştirilebilir doğrusal merkez eşiği",
            Self::AxisControl => "axis-control.html · 500.001 nokta ve sağ Y ekseni",
            Self::AxisAutosize => "axis-autosize.html · 501 nokta ve 1…10⁹ dinamik eksen ölçümü",
            Self::AxisIndicators => "axis-indicators.html · üç renkli eksen ve imleç göstergeleri",
            Self::Bars(_) => {
                "bars-grouped-stacked.html · 10 bağımsız grouped/stacked yüzey ve setSeries"
            }
            Self::BarsValuesAutosize(_) => {
                "bars-values-autosize.html · dikey/yatay otomatik kompakt değer yazısı"
            }
            Self::BoxWhisker(_) => {
                "box-whisker.html · 17 bağımsız yüzey · results.json ve stats.js"
            }
            Self::Candlestick => "candlestick-ohlc.html · Gold OHLC ve hacim",
        }
    }

    fn tanım(self) -> &'static str {
        match self {
            Self::AddDelSeries => ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ,
            Self::AlignDataCost => ALIGN_DATA_KART_TANIM_ÖRNEĞİ,
            Self::Resize => RESIZE_KART_TANIM_ÖRNEĞİ,
            Self::Annotations => ANNOTATIONS_KART_TANIM_ÖRNEĞİ,
            Self::AreaFill => AREA_FILL_KART_TANIM_ÖRNEĞİ,
            Self::ScalePadding => SCALE_PADDING_KART_TANIM_ÖRNEĞİ,
            Self::Months => MONTHS_KART_TANIM_ÖRNEĞİ,
            Self::NiceScale => NICE_SCALE_KART_TANIM_ÖRNEĞİ,
            Self::NoData => NO_DATA_KART_TANIM_ÖRNEĞİ,
            Self::PathGapClip => PATH_GAP_CLIP_KART_TANIM_ÖRNEĞİ,
            Self::PixelAlign => PIXEL_ALIGN_KART_TANIM_ÖRNEĞİ,
            Self::Points => POINTS_KART_TANIM_ÖRNEĞİ,
            Self::ScalesDirOri => SCALES_DIR_ORI_KART_TANIM_ÖRNEĞİ,
            Self::Scatter => SCATTER_KART_TANIM_ÖRNEĞİ,
            Self::ScrollSync => SCROLL_SYNC_KART_TANIM_ÖRNEĞİ,
            Self::SineStream => SINE_STREAM_KART_TANIM_ÖRNEĞİ,
            Self::SoftMinMax(_) => SOFT_MINMAX_KART_TANIM_ÖRNEĞİ,
            Self::SparklinesBars(_) => SPARKLINES_BARS_KART_TANIM_ÖRNEĞİ,
            Self::Sparklines(_) => SPARKLINES_KART_TANIM_ÖRNEĞİ,
            Self::Sparse(_) => SPARSE_KART_TANIM_ÖRNEĞİ,
            Self::StackedSeries(_) => STACKED_SERIES_KART_TANIM_ÖRNEĞİ,
            Self::StreamData(_) => STREAM_DATA_KART_TANIM_ÖRNEĞİ,
            Self::SvgImage => SVG_IMAGE_KART_TANIM_ÖRNEĞİ,
            Self::SyncCursor => SYNC_CURSOR_KART_TANIM_ÖRNEĞİ,
            Self::SyncYZero(_) => SYNC_Y_ZERO_KART_TANIM_ÖRNEĞİ,
            Self::ThinBars(_) => THIN_BARS_STROKE_FILL_KART_TANIM_ÖRNEĞİ,
            Self::TimePeriods(_) => TIME_PERIODS_KART_TANIM_ÖRNEĞİ,
            Self::TimelineDiscrete(_) => TIMELINE_DISCRETE_KART_TANIM_ÖRNEĞİ,
            Self::TimeseriesDiscrete => TIMESERIES_DISCRETE_KART_TANIM_ÖRNEĞİ,
            Self::TimezonesDst => TIMEZONES_DST_KART_TANIM_ÖRNEĞİ,
            Self::TooltipsClosest => TOOLTIPS_CLOSEST_KART_TANIM_ÖRNEĞİ,
            Self::Tooltips => TOOLTIPS_KART_TANIM_ÖRNEĞİ,
            Self::Trendlines => TRENDLINES_KART_TANIM_ÖRNEĞİ,
            Self::UpdateCursorSelectResize => UPDATE_CURSOR_SELECT_RESIZE_KART_TANIM_ÖRNEĞİ,
            Self::WindDirection => WIND_DIRECTION_KART_TANIM_ÖRNEĞİ,
            Self::YScaleDrag => Y_SCALE_DRAG_KART_TANIM_ÖRNEĞİ,
            Self::YShiftedSeries => Y_SHIFTED_SERIES_KART_TANIM_ÖRNEĞİ,
            Self::CursorBind => CURSOR_BIND_KART_TANIM_ÖRNEĞİ,
            Self::CursorSnap => CURSOR_SNAP_KART_TANIM_ÖRNEĞİ,
            Self::CursorTooltip => CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ,
            Self::CustomScales => CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ,
            Self::DataSmoothing => DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ,
            Self::DrawHooks => DRAW_HOOKS_KART_TANIM_ÖRNEĞİ,
            Self::FocusCursor => FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ,
            Self::Gradients => GRADIENTS_KART_TANIM_ÖRNEĞİ,
            Self::GridOverSeries => GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ,
            Self::HighLowBands(_) => HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ,
            Self::LatencyHeatmap(_) => LATENCY_HEATMAP_KART_TANIM_ÖRNEĞİ,
            Self::LinePaths(_) => LINE_PATHS_KART_TANIM_ÖRNEĞİ,
            Self::LogScales(_) => LOG_SCALES_KART_TANIM_ÖRNEĞİ,
            Self::LogScales2(_) => LOG_SCALES2_KART_TANIM_ÖRNEĞİ,
            Self::MassSpectrum => MASS_SPECTRUM_KART_TANIM_ÖRNEĞİ,
            Self::MeasureDatums => MEASURE_DATUMS_KART_TANIM_ÖRNEĞİ,
            Self::MultiBars(_) => MULTI_BARS_KART_TANIM_ÖRNEĞİ,
            Self::NearestNonNull => NEAREST_NON_NULL_KART_TANIM_ÖRNEĞİ,
            Self::MissingData => MISSING_DATA_KART_TANIM_ÖRNEĞİ,
            Self::DependentScale => DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ,
            Self::ArcSinhScales => ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ,
            Self::AxisControl => AXIS_CONTROL_KART_TANIM_ÖRNEĞİ,
            Self::AxisAutosize => AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
            Self::AxisIndicators => AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ,
            Self::Bars(_) => BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ,
            Self::BarsValuesAutosize(_) => BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
            Self::BoxWhisker(_) => BOX_WHISKER_KART_TANIM_ÖRNEĞİ,
            Self::Candlestick => CANDLESTICK_KART_TANIM_ÖRNEĞİ,
        }
    }

    fn tanım_yolu(self) -> &'static str {
        match self {
            Self::AddDelSeries => "src/kart/add_del_series.rs",
            Self::AlignDataCost => "src/kart/align_data.rs",
            Self::Resize => "src/kart/resize.rs",
            Self::Annotations => "src/kart/annotations.rs",
            Self::AreaFill => "src/kart/area_fill.rs",
            Self::ScalePadding => "src/kart/scale_padding.rs",
            Self::Months => "src/kart/months.rs",
            Self::NiceScale => "src/kart/nice_scale.rs",
            Self::NoData => "src/kart/no_data.rs",
            Self::PathGapClip => "src/kart/path_gap_clip.rs",
            Self::PixelAlign => "src/kart/pixel_align.rs",
            Self::Points => "src/kart/points.rs",
            Self::ScalesDirOri => "src/kart/scales_dir_ori.rs",
            Self::Scatter => "src/kart/scatter.rs",
            Self::ScrollSync => "src/kart/scroll_sync.rs",
            Self::SineStream => "src/kart/sine_stream.rs",
            Self::SoftMinMax(_) => "src/kart/soft_minmax.rs",
            Self::SparklinesBars(_) => "src/kart/sparklines_bars.rs",
            Self::Sparklines(_) => "src/kart/sparklines.rs",
            Self::Sparse(_) => "src/kart/sparse.rs",
            Self::StackedSeries(_) => "src/kart/stacked_series.rs",
            Self::StreamData(_) => "src/kart/stream_data.rs",
            Self::SvgImage => "src/kart/svg_image.rs",
            Self::SyncCursor => "src/kart/sync_cursor.rs",
            Self::SyncYZero(_) => "src/kart/sync_y_zero.rs",
            Self::ThinBars(_) => "src/kart/thin_bars_stroke_fill.rs",
            Self::TimePeriods(_) => "src/kart/time_periods.rs",
            Self::TimelineDiscrete(_) => "src/kart/timeline_discrete.rs",
            Self::TimeseriesDiscrete => "src/kart/timeseries_discrete.rs",
            Self::TimezonesDst => "src/kart/timezones_dst.rs",
            Self::TooltipsClosest => "src/kart/tooltips_closest.rs",
            Self::Tooltips => "src/kart/tooltips.rs",
            Self::Trendlines => "src/kart/trendlines.rs",
            Self::UpdateCursorSelectResize => "src/kart/update_cursor_select_resize.rs",
            Self::WindDirection => "src/kart/wind_direction.rs",
            Self::YScaleDrag => "src/kart/y_scale_drag.rs",
            Self::YShiftedSeries => "src/kart/y_shifted_series.rs",
            Self::CursorBind => "src/kart/cursor_bind.rs",
            Self::CursorSnap => "src/kart/cursor_snap.rs",
            Self::CursorTooltip => "src/kart/cursor_tooltip.rs",
            Self::CustomScales => "src/kart/custom_scales.rs",
            Self::DataSmoothing => "src/kart/data_smoothing.rs",
            Self::DrawHooks => "src/kart/draw_hooks.rs",
            Self::FocusCursor => "src/kart/focus_cursor.rs",
            Self::Gradients => "src/kart/gradients.rs",
            Self::GridOverSeries => "src/kart/grid_over_series.rs",
            Self::HighLowBands(_) => "src/kart/high_low_bands.rs",
            Self::LatencyHeatmap(_) => "src/kart/latency_heatmap.rs",
            Self::LinePaths(_) => "src/kart/line_paths.rs",
            Self::LogScales(_) => "src/kart/log_scales.rs",
            Self::LogScales2(_) => "src/kart/log_scales2.rs",
            Self::MassSpectrum => "src/kart/mass_spectrum.rs",
            Self::MeasureDatums => "src/kart/measure_datums.rs",
            Self::MultiBars(_) => "src/kart/multi_bars.rs",
            Self::NearestNonNull => "src/kart/nearest_non_null.rs",
            Self::MissingData => "src/kart/missing_data.rs",
            Self::DependentScale => "src/kart/dependent_scale.rs",
            Self::ArcSinhScales => "src/kart/arcsinh_scales.rs",
            Self::AxisControl => "src/kart/axis_control.rs",
            Self::AxisAutosize => "src/kart/axis_autosize.rs",
            Self::AxisIndicators => "src/kart/axis_indicators.rs",
            Self::Bars(_) => "src/kart/bars_grouped_stacked.rs",
            Self::BarsValuesAutosize(_) => "src/kart/bars_values_autosize.rs",
            Self::BoxWhisker(_) => "src/kart/box_whisker.rs",
            Self::Candlestick => "src/kart/candlestick_ohlc.rs",
        }
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
    nokta_sayısı: usize,
    grafik: Option<Entity<GpuiGrafik>>,
    hata: Option<String>,
    kart_tanımı_açık: bool,
    kullanım_rehberi_açık: bool,
    tekerlek_etkin: bool,
    tekerlek_anahtarı: Entity<Anahtar>,
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
    sync_cursor_grubu: SyncCursorGrubu,
    sync_cursor_senkronlanıyor: bool,
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
}

impl ChartListesi {
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
        match olay {
            GpuiGrafikOlayı::Açıklamaİstendi => self.açıklama_istemini_aç(cx),
            GpuiGrafikOlayı::FareBırakıldı if self.aktif_kart == KartKimliği::CursorBind => {
                self.cursor_bind_tıklama_sayısı = self.cursor_bind_tıklama_sayısı.saturating_add(1);
            }
            _ => {}
        }
        cx.notify();
    }

    pub fn yeni(cx: &mut Context<Self>) -> Self {
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
            if bu.aktif_kart == KartKimliği::AlignDataCost {
                for (_, grafik) in &bu.align_data_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::CustomScales {
                for (_, grafik) in &bu.custom_scales_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::DataSmoothing {
                for (_, grafik) in &bu.data_smoothing_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::FocusCursor {
                for (_, grafik) in &bu.focus_cursor_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::Gradients {
                for (_, grafik) in &bu.gradients_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::SyncCursor {
                for (_, grafik) in &bu.sync_cursor_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::TimeseriesDiscrete {
                for (_, grafik) in &bu.timeseries_discrete_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::TimezonesDst {
                for (_, grafik) in &bu.timezones_dst_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::NearestNonNull {
                for (_, grafik) in &bu.nearest_non_null_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::MissingData {
                for (_, grafik) in &bu.missing_data_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::Months {
                for grafik in &bu.months_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::PathGapClip {
                for (_, grafik) in &bu.path_gap_clip_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::PixelAlign {
                for (_, grafik) in &bu.pixel_align_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::Points {
                for (_, grafik) in &bu.points_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if bu.aktif_kart == KartKimliği::ScalesDirOri {
                bu.scales_dir_ori_senkronlanıyor = true;
                for (_, grafik) in &bu.scales_dir_ori_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
                bu.scales_dir_ori_senkronlanıyor = false;
            } else if bu.aktif_kart == KartKimliği::Scatter {
                for (_, grafik) in &bu.scatter_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if matches!(bu.aktif_kart, KartKimliği::Bars(_)) {
                for (_, grafik) in &bu.bars_grouped_stacked_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if matches!(bu.aktif_kart, KartKimliği::BarsValuesAutosize(_)) {
                for (_, grafik) in &bu.bars_values_autosize_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if matches!(bu.aktif_kart, KartKimliği::BoxWhisker(_)) {
                for (_, grafik) in &bu.box_whisker_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if matches!(bu.aktif_kart, KartKimliği::SoftMinMax(_)) {
                for (_, grafik) in &bu.soft_minmax_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if matches!(bu.aktif_kart, KartKimliği::SparklinesBars(_)) {
                for (_, grafik) in &bu.sparklines_bars_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if matches!(bu.aktif_kart, KartKimliği::Sparklines(_)) {
                for (_, grafik) in &bu.sparklines_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if matches!(bu.aktif_kart, KartKimliği::Sparse(_)) {
                for (_, grafik) in &bu.sparse_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if matches!(bu.aktif_kart, KartKimliği::StackedSeries(_)) {
                for (_, grafik) in &bu.stacked_series_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if matches!(bu.aktif_kart, KartKimliği::StreamData(_)) {
                for (_, grafik) in &bu.stream_data_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if matches!(bu.aktif_kart, KartKimliği::ThinBars(_)) {
                for (_, grafik) in &bu.thin_bars_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if matches!(bu.aktif_kart, KartKimliği::TimePeriods(_)) {
                for (_, grafik) in &bu.time_periods_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if matches!(bu.aktif_kart, KartKimliği::TimelineDiscrete(_)) {
                for (_, grafik) in &bu.timeline_discrete_grafikleri {
                    grafik.update(cx, |grafik, cx| {
                        grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                    });
                }
            } else if let Some(grafik) = &bu.grafik {
                grafik.update(cx, |grafik, cx| {
                    grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                });
            }
            bu.tekerlek_etkin = etkin;
            cx.notify();
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
        Self {
            aktif_kart: KartKimliği::Resize,
            nokta_sayısı: 100,
            grafik,
            hata,
            kart_tanımı_açık: false,
            kullanım_rehberi_açık: false,
            tekerlek_etkin: etkileşimler.tekerlek_etkileşimi,
            tekerlek_anahtarı,
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
            sync_cursor_grubu: SyncCursorGrubu::yeni(),
            sync_cursor_senkronlanıyor: false,
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
        }
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
                    GpuiGrafikOlayı::İmleçDeğişti => {
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
                    GpuiGrafikOlayı::FareBırakıldı | GpuiGrafikOlayı::DurumDeğişti => {}
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
                                    grafik.görünür_x_aralığını_ayarla(x, true, cx);
                                });
                            }
                            bu.timeseries_discrete_senkronlanıyor = false;
                        }
                    }
                }
                cx.notify();
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

    fn timeseries_discrete_serisini_değiştir(
        &mut self,
        birleşik_indeks: usize,
        cx: &mut Context<Self>,
    ) {
        let (örnek, seri_indeksi) = if birleşik_indeks == 0 {
            (TimeseriesDiscreteÖrneği::ZamanSerisi, 0)
        } else {
            (TimeseriesDiscreteÖrneği::AyrıkDurumlar, birleşik_indeks - 1)
        };
        let Some((_, yüzey)) = self
            .timeseries_discrete_grafikleri
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
                self.hata = Some(format!("Birleşik lejant serisi değiştirilemedi: {hata}"));
            }
        }
        cx.notify();
    }

    fn tooltip_serisini_değiştir(&mut self, seri: usize, cx: &mut Context<Self>) {
        let Some(grafik) = self.grafik.clone() else {
            return;
        };
        let görünür = grafik.read(cx).grafik().seri_görünür_mü(seri);
        match grafik.update(cx, |grafik, cx| {
            grafik.seri_görünürlüğünü_ayarla(seri, !görünür, cx)
        }) {
            Ok(_) => self.hata = None,
            Err(hata) => self.hata = Some(format!("Tooltip serisi değiştirilemedi: {hata}")),
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
                    if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                        bu.açıklama_istendi = true;
                    }
                    cx.notify();
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
                    GpuiGrafikOlayı::İmleçDeğişti => {
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
                                    grafik.görünür_x_aralığını_ayarla(x, true, cx);
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
                    GpuiGrafikOlayı::FareBırakıldı => {}
                }
                cx.notify();
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
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
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
            yüzeyler.push(cx.new(|_| GpuiGrafik::yeni(grafik)));
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
            yüzeyler.push((örnek, cx.new(|_| GpuiGrafik::yeni(grafik))));
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
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
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
        let sonuç = pixel_align_kartları(140);
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
            grafik.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
            let grafik = cx.new(|_| GpuiGrafik::yeni(grafik));
            cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        self.pixel_align_akışı = PixelAlignAkışı::yeni(140).ok();
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
            yüzeyler.push((örnek, cx.new(|_| GpuiGrafik::yeni(grafik))));
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
                    GpuiGrafikOlayı::İmleçDeğişti => {
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
                                        grafik.görünür_aralıkları_ayarla(x, y, true, cx);
                                    });
                                }
                            }
                            bu.scales_dir_ori_senkronlanıyor = false;
                        }
                    }
                    GpuiGrafikOlayı::Açıklamaİstendi => {
                        bu.açıklama_istendi = true;
                    }
                }
                cx.notify();
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
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
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
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
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
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
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
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
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
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
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
            yüzeyler.push((örnek, cx.new(|_| GpuiGrafik::yeni(grafik))));
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
            yüzeyler.push((örnek, cx.new(|_| GpuiGrafik::yeni(grafik))));
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
                    GpuiGrafikOlayı::İmleçDeğişti => {
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
                                        grafik.görünür_aralıkları_ayarla(x, y, true, cx);
                                    });
                                }
                            }
                            bu.scales_dir_ori_senkronlanıyor = false;
                        }
                    }
                    GpuiGrafikOlayı::Açıklamaİstendi => bu.açıklama_istendi = true,
                    GpuiGrafikOlayı::FareBırakıldı => {}
                }
                cx.notify();
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
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
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
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
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
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
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
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
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
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
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
        match grafik_oluştur(
            self.aktif_kart,
            self.no_data_örneği,
            nokta_sayısı,
            self.autosize_kuvvet,
            self.latency_kova,
            self.latency_ofset,
            140,
        ) {
            Ok(mut yeni) => {
                yeni.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
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
            cx.subscribe(&grafik, move |bu, _, olay: &GpuiGrafikOlayı, cx| {
                if bu.sync_cursor_senkronlanıyor {
                    return;
                }
                match olay {
                    GpuiGrafikOlayı::Açıklamaİstendi => {
                        bu.açıklama_istendi = true;
                    }
                    GpuiGrafikOlayı::İmleçDeğişti => {
                        let yayın = bu
                            .sync_cursor_grafikleri
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .and_then(|(_, grafik)| grafik.read(cx).senkron_veri_yayını());
                        let hedefler = bu.sync_cursor_grubu.imleç_hedefleri(örnek);
                        let yüzeyler = bu.sync_cursor_grafikleri.clone();
                        bu.sync_cursor_senkronlanıyor = true;
                        for hedef in hedefler {
                            let Some((_, hedef_grafik)) =
                                yüzeyler.iter().find(|(kimlik, _)| *kimlik == hedef)
                            else {
                                continue;
                            };
                            if let Some((x, y, kaynak_serisi)) = yayın {
                                let hedef_serisi = kaynak_serisi.and_then(|indeks| {
                                    bu.sync_cursor_grubu.seri_hedefi(örnek, hedef, indeks)
                                });
                                let dikey = bu
                                    .sync_cursor_grubu
                                    .dikey_imleç_senkron_mu(örnek, hedef)
                                    .then_some(y);
                                hedef_grafik.update(cx, |grafik, cx| {
                                    if let Some(y) = dikey {
                                        grafik.senkron_veri_imleci_ayarla(x, y, hedef_serisi, cx);
                                    } else {
                                        grafik.senkron_veri_x_imleci_ayarla(x, hedef_serisi, cx);
                                    }
                                });
                            } else {
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.senkron_imleci_temizle(cx);
                                });
                            }
                        }
                        bu.sync_cursor_senkronlanıyor = false;
                    }
                    GpuiGrafikOlayı::FareBırakıldı => {
                        let değişenler = bu.sync_cursor_grubu.fare_bırak(örnek);
                        let yüzeyler = bu.sync_cursor_grafikleri.clone();
                        for (kimlik, kilitli) in değişenler {
                            if let Some((_, hedef)) =
                                yüzeyler.iter().find(|(hedef, _)| *hedef == kimlik)
                            {
                                hedef.update(cx, |grafik, cx| {
                                    grafik.senkron_kilidi_ayarla(kilitli, cx);
                                });
                            }
                        }
                    }
                    GpuiGrafikOlayı::GörünümDeğişti {
                        fare_basma_bırakma
                    } => {
                        let x = bu
                            .sync_cursor_grafikleri
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .map(|(_, grafik)| grafik.read(cx).grafik().görünür_x_aralığı());
                        let hedefler = bu
                            .sync_cursor_grubu
                            .görünüm_hedefleri(örnek, *fare_basma_bırakma);
                        let yüzeyler = bu.sync_cursor_grafikleri.clone();
                        if let Some(x) = x {
                            bu.sync_cursor_senkronlanıyor = true;
                            for hedef in hedefler {
                                let Some((_, hedef_grafik)) =
                                    yüzeyler.iter().find(|(kimlik, _)| *kimlik == hedef)
                                else {
                                    continue;
                                };
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.görünür_x_aralığını_ayarla(x, true, cx);
                                });
                            }
                            bu.sync_cursor_senkronlanıyor = false;
                        }
                    }
                    GpuiGrafikOlayı::DurumDeğişti => {}
                }
                cx.notify();
            })
            .detach();
            yüzeyler.push((örnek, grafik));
        }
        if let Some(hata) = hata {
            self.hata = Some(hata);
            self.grafik = None;
            self.sync_cursor_grafikleri.clear();
        } else {
            self.grafik = yüzeyler.first().map(|(_, grafik)| grafik.clone());
            self.sync_cursor_grafikleri = yüzeyler;
            self.sync_cursor_senkronlanıyor = false;
            self.hata = None;
        }
        cx.notify();
    }

    fn sync_cursor_senkronunu_değiştir(&mut self, cx: &mut Context<Self>) {
        let etkin = !self.sync_cursor_grubu.senkron();
        self.sync_cursor_grubu.senkronu_ayarla(etkin);
        cx.notify();
    }

    fn sync_cursor_serisini_değiştir(
        &mut self,
        örnek: SyncCursorÖrneği,
        seri: usize,
        cx: &mut Context<Self>,
    ) {
        let yüzeyler = self.sync_cursor_grafikleri.clone();
        let Some((_, kaynak)) = yüzeyler.iter().find(|(kimlik, _)| *kimlik == örnek) else {
            return;
        };
        let görünür = !kaynak
            .read(cx)
            .grafik()
            .seri_seçenekleri()
            .get(seri)
            .is_some_and(|seçenek| seçenek.göster);
        self.sync_cursor_senkronlanıyor = true;
        if let Err(hata) = kaynak.update(cx, |grafik, cx| {
            grafik.seri_görünürlüğünü_ayarla(seri, görünür, cx)
        }) {
            self.hata = Some(format!(
                "Sync Cursor seri görünürlüğü değiştirilemedi: {hata}"
            ));
            self.sync_cursor_senkronlanıyor = false;
            cx.notify();
            return;
        }
        for hedef in self.sync_cursor_grubu.imleç_hedefleri(örnek) {
            let Some(hedef_seri) = self.sync_cursor_grubu.seri_hedefi(örnek, hedef, seri) else {
                continue;
            };
            let Some((_, hedef_grafik)) = yüzeyler.iter().find(|(kimlik, _)| *kimlik == hedef)
            else {
                continue;
            };
            if let Err(hata) = hedef_grafik.update(cx, |grafik, cx| {
                grafik.seri_görünürlüğünü_ayarla(hedef_seri, görünür, cx)
            }) {
                self.hata = Some(format!(
                    "{} Sync Cursor seri görünürlüğü değiştirilemedi: {hata}",
                    hedef.başlık()
                ));
            }
        }
        self.sync_cursor_senkronlanıyor = false;
        cx.notify();
    }

    fn sync_cursor_fare_filtresini_değiştir(&mut self, cx: &mut Context<Self>) {
        let etkin = !self.sync_cursor_grubu.fare_basma_bırakma_senkron();
        self.sync_cursor_grubu
            .fare_basma_bırakma_senkronunu_ayarla(etkin);
        cx.notify();
    }

    fn kartı_seç(&mut self, kart: KartKimliği, cx: &mut Context<Self>) {
        if self.aktif_kart == kart {
            return;
        }
        self.aktif_kart = kart;
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
        self.tekerlek_anahtarı.update(cx, |anahtar, cx| {
            anahtar.ayarla(etkileşimler.tekerlek_etkileşimi, cx);
            anahtar.devre_disi_ayarla(false, cx);
        });
        self.align_data_grafikleri.clear();
        self.align_data_kurulum_ms = None;
        self.custom_scales_grafikleri.clear();
        self.data_smoothing_grafikleri.clear();
        self.data_smoothing_ölçümleri_ms.clear();
        self.focus_cursor_grafikleri.clear();
        self.gradients_grafikleri.clear();
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
                            let veri_değişti = akış.kareyi_ilerlet(geçen_ms.min(1_000.0));
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
                            let boyut = akış.ilerlet();
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
        self.grafiği_yenile(self.nokta_sayısı, cx);
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
    let (seçenekler, veri) = match kart {
        KartKimliği::AddDelSeries => add_del_series_kartı(),
        KartKimliği::AlignDataCost => align_data_maliyet_kartı(),
        KartKimliği::Resize => resize_kartı(nokta_sayısı),
        KartKimliği::Annotations => annotations_kartı(),
        KartKimliği::AreaFill => area_fill_kartı(),
        KartKimliği::ScalePadding => scale_padding_kartı(),
        KartKimliği::Months => months_artık_yılsız_kartı(),
        KartKimliği::NiceScale => nice_scale_kartı(),
        KartKimliği::NoData => no_data_kartı(no_data_örneği),
        KartKimliği::PathGapClip => path_gap_clip_kartı(PathGapClipÖrneği::VeriDışınaTaşanÖlçek),
        KartKimliği::PixelAlign => {
            pixel_align_kartı(PixelAlignÖrneği::Varsayılan, pixel_align_adımı)
        }
        KartKimliği::Points => points_kartı(PointsÖrneği::Karma),
        KartKimliği::ScalesDirOri => scales_dir_ori_kartı(ScalesDirOriÖrneği::XArtıAltYArtıSol),
        KartKimliği::Scatter => scatter_kartı(ScatterÖrneği::Scatter),
        KartKimliği::ScrollSync => scroll_sync_kartı(),
        KartKimliği::SineStream => sine_stream_kartı(),
        KartKimliği::SoftMinMax(örnek) => soft_minmax_kartı(örnek, 12.0),
        KartKimliği::SparklinesBars(örnek) => sparklines_bars_kartı(örnek),
        KartKimliği::Sparklines(örnek) => sparklines_kartı(örnek),
        KartKimliği::Sparse(örnek) => sparse_kartı(örnek),
        KartKimliği::StackedSeries(örnek) => stacked_series_kartı(örnek),
        KartKimliği::StreamData(örnek) => stream_data_kartı(örnek),
        KartKimliği::SvgImage => svg_image_kartı(),
        KartKimliği::SyncCursor => sync_cursor_kartı(SyncCursorÖrneği::Cpu),
        KartKimliği::SyncYZero(aşama) => sync_y_zero_kartı(aşama),
        KartKimliği::ThinBars(örnek) => thin_bars_stroke_fill_kartı(örnek),
        KartKimliği::TimePeriods(örnek) => time_periods_kartı(örnek),
        KartKimliği::TimelineDiscrete(örnek) => timeline_discrete_kartı(örnek),
        KartKimliği::TimeseriesDiscrete => {
            timeseries_discrete_kartı(TimeseriesDiscreteÖrneği::ZamanSerisi)
        }
        KartKimliği::TimezonesDst => {
            let örnek =
                TimezonesDstÖrneği::yeni(0).ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
            timezones_dst_kartı(örnek)
        }
        KartKimliği::TooltipsClosest => tooltips_closest_kartı(),
        KartKimliği::Tooltips => tooltips_kartı(),
        KartKimliği::Trendlines => trendlines_kartı(),
        KartKimliği::UpdateCursorSelectResize => update_cursor_select_resize_kartı(800),
        KartKimliği::WindDirection => wind_direction_kartı(),
        KartKimliği::YScaleDrag => y_scale_drag_kartı(),
        KartKimliği::YShiftedSeries => y_shifted_series_kartı(),
        KartKimliği::CursorBind => cursor_bind_kartı(),
        KartKimliği::CursorSnap => cursor_snap_kartı(),
        KartKimliği::CursorTooltip => cursor_tooltip_kartı(),
        KartKimliği::CustomScales => custom_scales_kartı(CustomScaleÖrneği::Doğrusal),
        KartKimliği::DataSmoothing => data_smoothing_kartı(SmoothingÖrneği::Ham),
        KartKimliği::DrawHooks => draw_hooks_kartı(),
        KartKimliği::FocusCursor => focus_cursor_kartı(FocusÖrneği::İmleç),
        KartKimliği::Gradients => gradients_kartı(GradientÖrneği::YatayÇizgi),
        KartKimliği::GridOverSeries => grid_over_series_kartı(),
        KartKimliği::HighLowBands(örnek) => high_low_bands_kartı(örnek),
        KartKimliği::LatencyHeatmap(örnek) => {
            latency_heatmap_kartı(örnek, f64::from(latency_kova), f64::from(latency_ofset))
        }
        KartKimliği::LinePaths(örnek) => line_paths_kartı(örnek),
        KartKimliği::LogScales(örnek) => log_scales_kartı(örnek),
        KartKimliği::LogScales2(örnek) => log_scales2_kartı(örnek),
        KartKimliği::MassSpectrum => mass_spectrum_kartı(),
        KartKimliği::MeasureDatums => measure_datums_kartı(),
        KartKimliği::MultiBars(örnek) => multi_bars_kartı(örnek),
        KartKimliği::NearestNonNull => {
            nearest_non_null_kartı(NearestNonNullÖrneği::XDeğerineGöre)
        }
        KartKimliği::MissingData => missing_data_null_kartı(),
        KartKimliği::DependentScale => dependent_scale_kartı(),
        KartKimliği::ArcSinhScales => arcsinh_scales_kartı(),
        KartKimliği::AxisControl => axis_control_kartı(),
        KartKimliği::AxisAutosize => axis_autosize_kartı(10_f64.powi(autosize_kuvvet)),
        KartKimliği::AxisIndicators => axis_indicators_kartı(),
        KartKimliği::Bars(örnek) => bars_grouped_stacked_kartı(örnek),
        KartKimliği::BarsValuesAutosize(yön) => bars_values_autosize_kartı(yön),
        KartKimliği::BoxWhisker(benchmark) => box_whisker_kartı(benchmark),
        KartKimliği::Candlestick => candlestick_ohlc_kartı(),
    }?;
    Grafik::yeni(seçenekler, veri)
}

impl Render for ChartListesi {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
        let panel = rgb(0xffffff);
        let zemin = rgb(0xf3f4f6);
        let metin = rgb(0x111827);
        let soluk = rgb(0x6b7280);
        let vurgu = rgb(0xdc2626);
        let aktif_kart = self.aktif_kart;
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
                "3 ilişkili yüzey · 108 aylık nokta · UTC/artık yıl/yerel adlar".to_string()
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
            KartKimliği::SvgImage => {
                "3 nokta × 1 seri · 400×200 canlı sahne · bağımsız SVG API".to_string()
            }
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
            KartKimliği::HighLowBands(örnek) => {
                let uzunluk = örnek.nokta_sayısı();
                format!("{uzunluk} nokta · yönlü ve boşluğa duyarlı bant")
            }
            KartKimliği::LatencyHeatmap(LatencyHeatmapÖrneği::Ham) => {
                "100 zaman sütunu · yaklaşık 35 bin ham örnek".to_string()
            }
            KartKimliği::LatencyHeatmap(LatencyHeatmapÖrneği::Kovalanmış) => {
                "100 zaman sütunu · 5 ms yoğunluk kovaları".to_string()
            }
            KartKimliği::LatencyHeatmap(LatencyHeatmapÖrneği::Mode2) => {
                "45 bin örnek · 15 sn × 2 ms hücreler".to_string()
            }
            KartKimliği::LatencyHeatmap(
                LatencyHeatmapÖrneği::HistogramBirleşik | LatencyHeatmapÖrneği::HistogramBoşluklu,
            ) => "Tüm örnekler · 5 ms histogram kovaları".to_string(),
            KartKimliği::LinePaths(_) => "101 nokta · 4 null boşluğu · kaynak yol".to_string(),
            KartKimliği::LogScales(_) => {
                "1.440 zaman damgası × 12 kaynak sunucu serisi".to_string()
            }
            KartKimliği::LogScales2(örnek) => match örnek {
                LogScales2Örneği::GenişDoğrusal
                | LogScales2Örneği::GenişLog10
                | LogScales2Örneği::GenişLog2 => {
                    "127 nokta · 10⁻⁶…10⁸ kaynak değerleri".to_string()
                }
                LogScales2Örneği::TersGiriş | LogScales2Örneği::TersÇıkış => {
                    "4 zaman noktası · eşlenmiş ters log10 görünümü".to_string()
                }
                LogScales2Örneği::PozitifFiltreli => {
                    "130 nokta · negatif/sıfır değerleri kırpılan log10".to_string()
                }
                LogScales2Örneği::SeyrekLog10 | LogScales2Örneği::SeyrekLog2 => {
                    "2 nokta · geniş aralıkta seyrek log bölmeleri".to_string()
                }
                LogScales2Örneği::TümüNull => {
                    "3 nokta × 2 seri · ikinci seri tümü null".to_string()
                }
                LogScales2Örneği::ÇokKüçük => "2 nokta · 3,1992e−16…4,9047e−13".to_string(),
                LogScales2Örneği::KısmiBüyük | LogScales2Örneği::KısmiKüçük => {
                    "3 nokta × 2 bağımsız kısmi log10 ölçeği".to_string()
                }
            },
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
            if kart_tanımı_açık { "▾" } else { "▸" },
            aktif_kart.tanım_yolu()
        ));
        let tekerlek_anahtarı = self.tekerlek_anahtarı.clone();
        let (mut geri_var, mut yakınlaştırılmış, etkileşimler, lejant, bileşen_hatası) =
            self.grafik.as_ref().map_or_else(
                || (false, false, aktif_kart.etkileşimler(), None, None),
                |grafik| {
                    let grafik = grafik.read(cx);
                    (
                        grafik.grafik().geri_var(),
                        grafik.grafik().yakınlaştırılmış(),
                        grafik.grafik().etkileşim_seçenekleri(),
                        grafik.lejant_değerleri(),
                        grafik.hata().map(str::to_string),
                    )
                },
            );
        if aktif_kart == KartKimliği::AlignDataCost {
            geri_var = self
                .align_data_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .align_data_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::CustomScales {
            geri_var = self
                .custom_scales_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .custom_scales_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::DataSmoothing {
            geri_var = self
                .data_smoothing_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .data_smoothing_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::FocusCursor {
            geri_var = self
                .focus_cursor_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .focus_cursor_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::Gradients {
            geri_var = self
                .gradients_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .gradients_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::SyncCursor {
            geri_var = self
                .sync_cursor_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .sync_cursor_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::TimeseriesDiscrete {
            geri_var = self
                .timeseries_discrete_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .timeseries_discrete_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::TimezonesDst {
            geri_var = self
                .timezones_dst_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .timezones_dst_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::NearestNonNull {
            geri_var = self
                .nearest_non_null_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .nearest_non_null_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::MissingData {
            geri_var = self
                .missing_data_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .missing_data_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::Months {
            geri_var = self
                .months_grafikleri
                .iter()
                .any(|grafik| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .months_grafikleri
                .iter()
                .any(|grafik| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::PathGapClip {
            geri_var = self
                .path_gap_clip_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .path_gap_clip_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::PixelAlign {
            geri_var = self
                .pixel_align_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .pixel_align_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::Points {
            geri_var = self
                .points_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .points_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::ScalesDirOri {
            geri_var = self
                .scales_dir_ori_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .scales_dir_ori_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if aktif_kart == KartKimliği::Scatter {
            geri_var = self
                .scatter_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .scatter_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if matches!(aktif_kart, KartKimliği::Bars(_)) {
            geri_var = self
                .bars_grouped_stacked_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .bars_grouped_stacked_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if matches!(aktif_kart, KartKimliği::BarsValuesAutosize(_)) {
            geri_var = self
                .bars_values_autosize_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .bars_values_autosize_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if matches!(aktif_kart, KartKimliği::BoxWhisker(_)) {
            geri_var = self
                .box_whisker_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .box_whisker_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if matches!(aktif_kart, KartKimliği::SoftMinMax(_)) {
            geri_var = self
                .soft_minmax_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .soft_minmax_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if matches!(aktif_kart, KartKimliği::SparklinesBars(_)) {
            geri_var = self
                .sparklines_bars_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .sparklines_bars_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if matches!(aktif_kart, KartKimliği::Sparklines(_)) {
            geri_var = self
                .sparklines_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .sparklines_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if matches!(aktif_kart, KartKimliği::Sparse(_)) {
            geri_var = self
                .sparse_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .sparse_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if matches!(aktif_kart, KartKimliği::StackedSeries(_)) {
            geri_var = self
                .stacked_series_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .stacked_series_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if matches!(aktif_kart, KartKimliği::StreamData(_)) {
            geri_var = self
                .stream_data_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .stream_data_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if matches!(aktif_kart, KartKimliği::ThinBars(_)) {
            geri_var = self
                .thin_bars_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .thin_bars_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if matches!(aktif_kart, KartKimliği::TimePeriods(_)) {
            geri_var = self
                .time_periods_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .time_periods_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        } else if matches!(aktif_kart, KartKimliği::TimelineDiscrete(_)) {
            geri_var = self
                .timeline_discrete_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().geri_var());
            yakınlaştırılmış = self
                .timeline_discrete_grafikleri
                .iter()
                .any(|(_, grafik)| grafik.read(cx).grafik().yakınlaştırılmış());
        }
        let çizim_hatası = self.hata.clone().or(bileşen_hatası);
        let seri_adları = if aktif_kart == KartKimliği::TimeseriesDiscrete {
            self.timeseries_discrete_grafikleri
                .iter()
                .flat_map(|(_, grafik)| {
                    grafik
                        .read(cx)
                        .grafik()
                        .seri_seçenekleri()
                        .iter()
                        .filter(|seri| seri.göster)
                        .map(|seri| seri.etiket.clone())
                        .collect::<Vec<_>>()
                })
                .collect()
        } else {
            self.grafik.as_ref().map_or_else(Vec::new, |grafik| {
                grafik
                    .read(cx)
                    .grafik()
                    .seri_seçenekleri()
                    .iter()
                    .filter(|seri| seri.göster)
                    .map(|seri| seri.etiket.clone())
                    .collect::<Vec<_>>()
            })
        };
        let lejant = if aktif_kart == KartKimliği::TimeseriesDiscrete {
            let mut ortak_x = None;
            let mut değerler = Vec::new();
            let mut lejant_var = false;
            for (_, grafik) in &self.timeseries_discrete_grafikleri {
                let grafik = grafik.read(cx);
                if let Some((x, yüzey_değerleri)) = grafik.lejant_değerleri() {
                    lejant_var = true;
                    ortak_x = ortak_x.or(x);
                    değerler.extend(
                        yüzey_değerleri
                            .into_iter()
                            .zip(grafik.grafik().seri_seçenekleri())
                            .filter_map(|(değer, seri)| seri.göster.then_some(değer)),
                    );
                }
            }
            lejant_var.then_some((ortak_x, değerler))
        } else {
            lejant
        };
        let lejant = lejant.map_or_else(
            || {
                let seriler = seri_adları
                    .iter()
                    .map(|ad| format!("□ {ad}: --"))
                    .collect::<Vec<_>>()
                    .join("    ");
                format!("x: --    {seriler}")
            },
            |(x, değerler)| {
                let seriler = seri_adları
                    .iter()
                    .zip(değerler.iter())
                    .map(|(ad, değer)| {
                        değer.map_or_else(
                            || format!("□ {ad}: --"),
                            |y| {
                                let değer = if aktif_kart == KartKimliği::TimeseriesDiscrete
                                    && ad.starts_with("DEV")
                                {
                                    format!("{y:.0}")
                                } else {
                                    format!("{y:.3}")
                                };
                                format!(
                                    "□ {ad}: {değer}{}",
                                    if x.is_none() { " (last)" } else { "" }
                                )
                            },
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("    ");
                let x = x.map_or_else(|| "--".to_string(), |x| format!("{x:.3}"));
                format!("x: {x}    {seriler}")
            },
        );
        let tooltip_serileri = if matches!(
            aktif_kart,
            KartKimliği::TooltipsClosest
                | KartKimliği::Tooltips
                | KartKimliği::CursorSnap
                | KartKimliği::Trendlines
                | KartKimliği::UpdateCursorSelectResize
                | KartKimliği::WindDirection
                | KartKimliği::YScaleDrag
                | KartKimliği::YShiftedSeries
                | KartKimliği::DependentScale
                | KartKimliği::ArcSinhScales
                | KartKimliği::AxisControl
                | KartKimliği::AxisAutosize
                | KartKimliği::AxisIndicators
        ) {
            self.grafik.as_ref().map_or_else(Vec::new, |grafik| {
                grafik
                    .read(cx)
                    .grafik()
                    .seri_seçenekleri()
                    .iter()
                    .enumerate()
                    .map(|(indeks, seri)| (indeks, seri.etiket.clone(), seri.göster))
                    .collect()
            })
        } else {
            Vec::new()
        };

        let liste = div()
            .id("kart-listesi")
            .w(px(280.0))
            .h_full()
            .min_h_0()
            .flex_none()
            .overflow_y_scroll()
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
                    .text_color(soluk)
                    .child("Canlı masaüstü doğrulaması"),
            )
            .child(
                katalog_kartı(
                    "add-del-series",
                    "Add/Delete Series",
                    "add-del-series",
                    aktif_kart == KartKimliği::AddDelSeries,
                    "Dinamik addSeries / delSeries / setData",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::AddDelSeries, cx);
                })),
            )
            .children(HighLowBandsÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::HighLowBands(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "high-low-bands",
                    aktif_kart == kart,
                    "Yönlü bant · boşluk ve yol kırpması",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(LatencyHeatmapÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::LatencyHeatmap(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "latency-heatmap",
                    aktif_kart == kart,
                    "Isı hücresi · kaynak histogram kovaları",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(LinePathsÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::LinePaths(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "line-paths",
                    aktif_kart == kart,
                    "101 nokta · null boşluğu · kaynak yol",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(LogScalesÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::LogScales(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "log-scales",
                    aktif_kart == kart,
                    "1.440 zaman × 12 sunucu · kaynak veri",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(LogScales2Örneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::LogScales2(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "log-scales2",
                    aktif_kart == kart,
                    "Log2/log10 · ters yön · null · kısmi büyüklük",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .child(
                katalog_kartı(
                    "mass-spectrum",
                    "Mass Spectrum",
                    "mass-spectrum",
                    aktif_kart == KartKimliği::MassSpectrum,
                    "41.986 CSV noktası · özel Y aralığı",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::MassSpectrum, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "measure-datums",
                    "Measure / Datums",
                    "measure-datums",
                    aktif_kart == KartKimliği::MeasureDatums,
                    "1/2: datum · Esc: temizle",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::MeasureDatums, cx);
                })),
            )
            .children(MultiBarsÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::MultiBars(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "multi-bars",
                    aktif_kart == kart,
                    "Gruplu benchmark · nokta başına renk",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .child(
                katalog_kartı(
                    "nearest-non-null",
                    "Nearest Non-Null",
                    "nearest-non-null.html",
                    aktif_kart == KartKimliği::NearestNonNull,
                    "5 yüzey · null/proximity/cursor karşılaştırması",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::NearestNonNull, cx);
                })),
            )
            .child({
                let kart = KartKimliği::FocusCursor;
                katalog_kartı(
                    "focus-cursor",
                    "Focus Cursor · 4 related surfaces",
                    "focus-cursor",
                    aktif_kart == kart,
                    "Ortak 130K veri · prox/bias · setSeries · 300 seri",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child({
                let kart = KartKimliği::Gradients;
                katalog_kartı(
                    "gradients",
                    "Gradients · 5 related surfaces",
                    "gradients",
                    aktif_kart == kart,
                    "Ayrık stroke · ArcSinh · ortak data4 dolguları",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child(
                katalog_kartı(
                    "grid-over-series",
                    "Grid Over Series",
                    "grid-over-series",
                    aktif_kart == KartKimliği::GridOverSeries,
                    "Izgara ve eksenler seri katmanının üstünde",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::GridOverSeries, cx);
                })),
            )
            .child({
                let kart = KartKimliği::TimezonesDst;
                katalog_kartı(
                    "timezones-dst",
                    "Timezones & DST",
                    "timezones-dst",
                    aktif_kart == kart,
                    "11 bölüm · 51 adet 600×200 yüzey",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child(
                katalog_kartı(
                    "tooltips-closest",
                    "Summary-opt",
                    "tooltips-closest",
                    aktif_kart == KartKimliği::TooltipsClosest,
                    "234 commit · en yakın seri tooltip'i",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::TooltipsClosest, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "tooltips",
                    "Tooltips",
                    "tooltips",
                    aktif_kart == KartKimliği::Tooltips,
                    "7 nokta · ham imleç + görünür seri kutuları",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::Tooltips, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "trendlines",
                    "Trendlines",
                    "trendlines",
                    aktif_kart == KartKimliği::Trendlines,
                    "100 nokta × 2 seri · görünür uç trendleri",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::Trendlines, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "update-cursor-select-resize",
                    "Maintain loc of cursor/select/hoverPts",
                    "update-cursor-select-resize",
                    aktif_kart == KartKimliği::UpdateCursorSelectResize,
                    "800↔400 px · kalıcı imleç/seçim oranı",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::UpdateCursorSelectResize, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "wind-direction",
                    "Wind Direction",
                    "wind-direction",
                    aktif_kart == KartKimliği::WindDirection,
                    "143 saat · sıcaklık, hız ve yön vektörleri",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::WindDirection, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "y-scale-drag",
                    "Draggable x & y scales",
                    "y-scale-drag",
                    aktif_kart == KartKimliği::YScaleDrag,
                    "X/Y eksen sürükleme · Shift ile ölçekleme",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::YScaleDrag, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "y-shifted-series",
                    "Y-shifted Series",
                    "y-shifted-series",
                    aktif_kart == KartKimliği::YShiftedSeries,
                    "30×3 · her 2 sn normal / kaydırılmış",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::YShiftedSeries, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "align-data-cost",
                    "Align Data · 2 related surfaces",
                    "align-data",
                    aktif_kart == KartKimliği::AlignDataCost,
                    "6 warmup + join · line + bars",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::AlignDataCost, cx);
                })),
            )
            .child(
                div()
                    .id("kart-line-resize")
                    .cursor_pointer()
                    .p_3()
                    .rounded_lg()
                    .border_1()
                    .border_color(if aktif_kart == KartKimliği::Resize {
                        vurgu
                    } else {
                        rgb(0xd1d5db)
                    })
                    .bg(if aktif_kart == KartKimliği::Resize {
                        rgb(0xfef2f2)
                    } else {
                        panel
                    })
                    .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                        bu.kartı_seç(KartKimliği::Resize, cx);
                    }))
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(metin)
                            .child("Resize"),
                    )
                    .child(
                        div()
                            .mt_1()
                            .text_xs()
                            .text_color(soluk)
                            .child("line-resize"),
                    )
                    .child(
                        div()
                            .mt_2()
                            .text_xs()
                            .text_color(vurgu)
                            .child("uplot-rs/gpui feature bileşeni"),
                    ),
            )
            .child(
                katalog_kartı(
                    "annotations",
                    "Annotations",
                    "annotations",
                    aktif_kart == KartKimliği::Annotations,
                    "2 seri · X çizgisi ve aralık işaretleri",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::Annotations, cx);
                })),
            )
            .child(
                div()
                    .id("kart-area-fill")
                    .cursor_pointer()
                    .mt_2()
                    .p_3()
                    .rounded_lg()
                    .border_1()
                    .border_color(if aktif_kart == KartKimliği::AreaFill {
                        vurgu
                    } else {
                        rgb(0xd1d5db)
                    })
                    .bg(if aktif_kart == KartKimliği::AreaFill {
                        rgb(0xfef2f2)
                    } else {
                        panel
                    })
                    .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                        bu.kartı_seç(KartKimliği::AreaFill, cx);
                    }))
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(metin)
                            .child("Area Fill"),
                    )
                    .child(div().mt_1().text_xs().text_color(soluk).child("area-fill"))
                    .child(
                        div()
                            .mt_2()
                            .text_xs()
                            .text_color(vurgu)
                            .child("3 alan serisi · kaynak veri üreteci"),
                    ),
            )
            .child(
                div()
                    .id("kart-scale-padding")
                    .cursor_pointer()
                    .mt_2()
                    .p_3()
                    .rounded_lg()
                    .border_1()
                    .border_color(if aktif_kart == KartKimliği::ScalePadding {
                        vurgu
                    } else {
                        rgb(0xd1d5db)
                    })
                    .bg(if aktif_kart == KartKimliği::ScalePadding {
                        rgb(0xfef2f2)
                    } else {
                        panel
                    })
                    .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                        bu.kartı_seç(KartKimliği::ScalePadding, cx);
                    }))
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(metin)
                            .child("Scale Padding"),
                    )
                    .child(
                        div()
                            .mt_1()
                            .text_xs()
                            .text_color(soluk)
                            .child("scale-padding"),
                    )
                    .child(
                        div()
                            .mt_2()
                            .text_xs()
                            .text_color(vurgu)
                            .child("13 düz seri · otomatik Y payı"),
                    ),
            )
            .child(
                katalog_kartı(
                    "kart-months",
                    "Months · calendar ticks",
                    "months",
                    aktif_kart == KartKimliği::Months,
                    "3 ilişkili yüzey · normal/artık yıl + Rusça",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::Months, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-nice-scale",
                    "Nice Scale & Ticks",
                    "nice-scale",
                    aktif_kart == KartKimliği::NiceScale,
                    "Boyuta bağlı Y aralığı ve ızgara",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::NiceScale, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-no-data",
                    "No Data · 33 seçenek",
                    "no-data",
                    aktif_kart == KartKimliği::NoData,
                    "Tek kart · seçilebilir 33 kaynak rangeNum durumu",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::NoData, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-path-gap-clip",
                    "Path & Gap Clipping",
                    "path-gap-clip",
                    aktif_kart == KartKimliği::PathGapClip,
                    "15 ilişkili yüzey · 4 ortak spanGaps animasyonu",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::PathGapClip, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-pixel-align",
                    "Pixel Align · canlı A/B",
                    "pixel-align",
                    aktif_kart == KartKimliği::PixelAlign,
                    "2 ortak veri yüzeyi · 60 FPS kayan pencere",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::PixelAlign, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-points",
                    "Points · 4 yüzey",
                    "points",
                    aktif_kart == KartKimliği::Points,
                    "Aynı sayfada yoğunluk · paths:null · piksel-gap filtresi",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::Points, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-scales-dir-ori",
                    "Scales Direction & Orientation · 16 yüzey",
                    "scales-dir-ori",
                    aktif_kart == KartKimliği::ScalesDirOri,
                    "Direction Inversion · Orientation Inversion",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::ScalesDirOri, cx);
                })),
            )
            .child({
                let kart = KartKimliği::Scatter;
                katalog_kartı(
                    "kart-scatter",
                    "Scatter & Bubble · 2 yüzey",
                    "scatter",
                    aktif_kart == kart,
                    "Bağımsız mode:2 facet · toplu yol + uzamsal hover",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child(
                katalog_kartı(
                    "kart-scroll-sync",
                    "Scroll syncRect()",
                    "scroll-sync",
                    aktif_kart == KartKimliği::ScrollSync,
                    "kaydırmada istemci → sahne eşlemesi",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::ScrollSync, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-sine-stream",
                    "Sine Stream",
                    "sine-stream",
                    aktif_kart == KartKimliği::SineStream,
                    "600 nokta × 6 seri · 60 FPS",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::SineStream, cx);
                })),
            )
            .child({
                let kart = KartKimliği::SoftMinMax(SoftMinMaxÖrneği::MinKip0);
                katalog_kartı(
                    "soft-minmax",
                    "Soft Min/Max · 5 yüzey",
                    "soft-minmax",
                    matches!(aktif_kart, KartKimliği::SoftMinMax(_)),
                    "4 ortak canlı kip + düz sıfır",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child({
                let kart = KartKimliği::SparklinesBars(SparklinesBarsÖrneği::GradyanÇubuklar);
                katalog_kartı(
                    "sparklines-bars",
                    "Sparkline + Floating Bars · 2 yüzey",
                    "sparklines-bars",
                    matches!(aktif_kart, KartKimliği::SparklinesBars(_)),
                    "aynı veri · dinamik gradyan / açık renk A/B",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child({
                let kart = KartKimliği::Sparklines(SparklineÖrneği::İLK);
                katalog_kartı(
                    "sparklines",
                    "Sparklines · 10×2 tablo",
                    "sparklines",
                    matches!(aktif_kart, KartKimliği::Sparklines(_)),
                    "10 hisse × hacim/kapanış · 20 yüzey",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child({
                let kart = KartKimliği::Sparse(SparseÖrneği::YerleşikDoğrusal);
                katalog_kartı(
                    "sparse",
                    "Sparse · 3 pathBuilder",
                    "sparse",
                    matches!(aktif_kart, KartKimliği::Sparse(_)),
                    "aynı 13.608 X · native / points / naive",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child({
                let kart = KartKimliği::StackedSeries(StackedSeriesÖrneği::Stacked1);
                katalog_kartı(
                    "stacked-series",
                    "Stacked Series · 16 yüzey",
                    "stacked-series",
                    matches!(aktif_kart, KartKimliği::StackedSeries(_)),
                    "aynı kaynak · yığma, null/undefined, yüzde ve grup matrisi",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child({
                let kart = KartKimliği::StreamData(StreamDataÖrneği::SabitUzunluk);
                katalog_kartı(
                    "stream-data",
                    "Data Stream · 3 yüzey",
                    "stream-data",
                    matches!(aktif_kart, KartKimliği::StreamData(_)),
                    "55.550 ortak satır · 100 ms/10 satır",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child(
                katalog_kartı(
                    "svg-image",
                    "uPlot to image PoC",
                    "svg-image",
                    aktif_kart == KartKimliği::SvgImage,
                    "400×200 canlı · tek bağımsız SVG belgesi",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::SvgImage, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "sync-cursor",
                    "Sync Cursor",
                    "sync-cursor",
                    aktif_kart == KartKimliği::SyncCursor,
                    "5 yüzey · cursor.pub/sub · seri etiketi eşleme",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::SyncCursor, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "sync-y-zero",
                    "Sync Y Zero",
                    "sync-y-zero",
                    matches!(aktif_kart, KartKimliği::SyncYZero(_)),
                    "3 aşama · 3 sol Y ekseni · ortak sıfır pikseli",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::SyncYZero(SyncYZeroAşaması::Ham), cx);
                })),
            )
            .child({
                let örnek = ThinBarsÖrneği::Yoğunluk(uplot_rs::ThinBarsYoğunluk::Normal30);
                let kart = KartKimliği::ThinBars(örnek);
                katalog_kartı(
                    "thin-bars-stroke-fill",
                    "Thin bar stroke & fill",
                    "thin-bars-stroke-fill",
                    matches!(aktif_kart, KartKimliği::ThinBars(_)),
                    "55 bağımsız yüzey · 7 yoğunluk + 48 geometri",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child({
                let kart = KartKimliği::TimePeriods(TimePeriodsÖrneği::SaatlikKullanıcılar);
                katalog_kartı(
                    "time-periods",
                    "Time Periods",
                    "time-periods",
                    matches!(aktif_kart, KartKimliği::TimePeriods(_)),
                    "3 bağımsız yüzey · tek traffic.json kaynağı",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child({
                let kart =
                    KartKimliği::TimelineDiscrete(TimelineDiscreteÖrneği::DurumZamanÇizelgesi);
                katalog_kartı(
                    "timeline-discrete",
                    "Timeline / Discrete",
                    "timeline-discrete",
                    matches!(aktif_kart, KartKimliği::TimelineDiscrete(_)),
                    "4 bağımsız yüzey · semantic/matrix karşılaştırması",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child(
                katalog_kartı(
                    "timeseries-discrete",
                    "TimeSeries + Discrete",
                    "timeseries-discrete",
                    aktif_kart == KartKimliği::TimeseriesDiscrete,
                    "2 eşzamanlı yüzey · 50 ortak X · birleşik lejant",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::TimeseriesDiscrete, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-cursor-snap",
                    "Cursor Snap",
                    "cursor-snap",
                    aktif_kart == KartKimliği::CursorSnap,
                    "10×10 piksel çekirdek ızgarası",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::CursorSnap, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-cursor-tooltip",
                    "Cursor Tooltip w/placement.js",
                    "cursor-tooltip",
                    aktif_kart == KartKimliği::CursorTooltip,
                    "Sınırlara duyarlı canlı bilgi kutusu",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::CursorTooltip, cx);
                })),
            )
            .child({
                let kart = KartKimliği::CustomScales;
                katalog_kartı(
                    "kart-custom-scales",
                    "Custom Scales · 3 independent surfaces",
                    "custom-scales",
                    aktif_kart == kart,
                    "3×800×800 · aynı veri, bağımsız linear/log/Weibull ölçekleri",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child({
                let kart = KartKimliği::DataSmoothing;
                katalog_kartı(
                    "kart-data-smoothing",
                    "Data Smoothing · 4 independent surfaces",
                    "data-smoothing",
                    aktif_kart == kart,
                    "4×1920×300 · raw, SGG, ASAP FFT, Moving Avg",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child(
                katalog_kartı(
                    "kart-draw-hooks",
                    "Draw Hooks",
                    "draw-hooks",
                    aktif_kart == KartKimliği::DrawHooks,
                    "Gradyan · seri medyanı · 6 uçlu yıldız",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::DrawHooks, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-missing-data",
                    "Missing Data · 2 surfaces",
                    "missing-data",
                    aktif_kart == KartKimliği::MissingData,
                    "Aynı kaynak sayfası · null ve komşu X gap",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::MissingData, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-dependent-scale",
                    "Derived Scale",
                    "dependent-scale",
                    aktif_kart == KartKimliği::DependentScale,
                    "Fahrenheit → Celsius sağ ekseni",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::DependentScale, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-arcsinh-scales",
                    "ArcSinh Y Scale",
                    "arcsinh-scales",
                    aktif_kart == KartKimliği::ArcSinhScales,
                    "Doğrusal eşik: 10⁻³…10³",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::ArcSinhScales, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-axis-control",
                    "Axis Control",
                    "axis-control",
                    aktif_kart == KartKimliği::AxisControl,
                    "500.001 nokta · sağ Y ekseni",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::AxisControl, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-axis-autosize",
                    "Axis AutoSize",
                    "axis-autosize",
                    aktif_kart == KartKimliği::AxisAutosize,
                    "501 nokta · dinamik eksen ölçümü",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::AxisAutosize, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-axis-indicators",
                    "Axis indicators",
                    "axis-indicators",
                    aktif_kart == KartKimliği::AxisIndicators,
                    "3 renkli eksen · canlı değer rozetleri",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::AxisIndicators, cx);
                })),
            )
            .child({
                let örnek = ÇubukÖrneği::ÇokGrupÇokSeriDikeyGruplu;
                katalog_kartı(
                    "kart-bars-grouped-stacked",
                    "Bars · Grouped / Stacked",
                    "bars-grouped-stacked",
                    matches!(aktif_kart, KartKimliği::Bars(_)),
                    "10 bağımsız yüzey · grouped/stacked",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::Bars(örnek), cx);
                }))
            })
            .child({
                let kart = KartKimliği::BarsValuesAutosize(ÇubukYönü::Dikey);
                katalog_kartı(
                    "kart-bars-values-autosize",
                    "Bars Values AutoSize",
                    "bars-values-autosize",
                    matches!(aktif_kart, KartKimliği::BarsValuesAutosize(_)),
                    "2 bağımsız yüzey · 10…25 px etiket",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child({
                let kart = KartKimliği::BoxWhisker("01_run1k");
                katalog_kartı(
                    "kart-box-whisker",
                    "Box & Whisker",
                    "box-whisker",
                    matches!(aktif_kart, KartKimliği::BoxWhisker(_)),
                    "17 bağımsız yüzey · hareketli tooltip",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            })
            .child(
                katalog_kartı(
                    "candlestick-ohlc",
                    "Candlestick Chart · Gold",
                    "candlestick-ohlc",
                    aktif_kart == KartKimliği::Candlestick,
                    "218 gün · OHLC + hacim",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::Candlestick, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "cursor-bind",
                    "Cursor Bind",
                    "cursor-bind",
                    aktif_kart == KartKimliği::CursorBind,
                    "Ctrl+sürükle · sarı açıklama seçimi",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::CursorBind, cx);
                })),
            );

        let araçlar = div()
            .flex()
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
                    Dugme::yeni("soft-minmax-baslat", "▶ dataMax++")
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
            .when(
                matches!(
                    aktif_kart,
                    KartKimliği::LatencyHeatmap(
                        LatencyHeatmapÖrneği::HistogramBirleşik
                            | LatencyHeatmapÖrneği::HistogramBoşluklu
                    )
                ),
                |öğe| {
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
                },
            )
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
                Dugme::yeni("nokta-artir", "＋ Nokta")
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

        let çizim_tabanı = div()
            .id("canli-chart")
            .flex_1()
            .min_h(px(320.0))
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
            çizim_tabanı
                .flex_none()
                .h(px(1380.0))
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
                .children(AlignDataÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    let (genişlik, yükseklik, açıklama) = match örnek {
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
                                .child(
                                    div()
                                        .w(px(genişlik))
                                        .h(px(yükseklik))
                                        .when_some(yüzey(örnek), |öğe, grafik| {
                                            öğe.child(grafik)
                                        }),
                                ),
                        )
                }))
        } else if aktif_kart == KartKimliği::CustomScales {
            let yüzey = |örnek| {
                self.custom_scales_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .flex_none()
                .h(px(1_700.0))
                .overflow_y_scroll()
                .p_2()
                .child(div().flex().flex_wrap().items_start().gap_3().children(
                    CustomScaleÖrneği::TÜMÜ.into_iter().map(|örnek| {
                        div()
                            .id(SharedString::from(format!(
                                "custom-scales-{}-surface",
                                örnek.kimlik()
                            )))
                            .flex_none()
                            .w(px(800.0))
                            .h(px(800.0))
                            .when_some(yüzey(örnek), |öğe, grafik| öğe.child(grafik))
                    }),
                ))
        } else if aktif_kart == KartKimliği::DataSmoothing {
            let yüzey = |örnek| {
                self.data_smoothing_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .flex_none()
                .h(px(1_450.0))
                .overflow_y_scroll()
                .p_2()
                .children(SmoothingÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    div()
                        .id(SharedString::from(format!(
                            "data-smoothing-{}-surface",
                            örnek.kimlik()
                        )))
                        .w_full()
                        .h(px(300.0))
                        .mb(px(50.0))
                        .overflow_x_scroll()
                        .child(
                            div()
                                .w(px(1_920.0))
                                .h(px(300.0))
                                .when_some(yüzey(örnek), |öğe, grafik| öğe.child(grafik)),
                        )
                }))
        } else if aktif_kart == KartKimliği::FocusCursor {
            let yüzey = |örnek| {
                self.focus_cursor_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .flex_none()
                .h(px(2_700.0))
                .overflow_y_scroll()
                .p_2()
                .children(FocusÖrneği::TÜMÜ.into_iter().map(|örnek| {
                    div()
                        .id(SharedString::from(format!(
                            "focus-cursor-{}-surface",
                            örnek.kimlik()
                        )))
                        .w_full()
                        .h(px(600.0))
                        .mb(px(50.0))
                        .overflow_x_scroll()
                        .child(
                            div()
                                .w(px(1_920.0))
                                .h(px(600.0))
                                .when_some(yüzey(örnek), |öğe, grafik| öğe.child(grafik)),
                        )
                }))
        } else if aktif_kart == KartKimliği::Gradients {
            let yüzey = |örnek| {
                self.gradients_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            çizim_tabanı
                .flex_none()
                .h(px(2_050.0))
                .overflow_y_scroll()
                .p_2()
                .child(div().flex().flex_wrap().items_start().gap_3().children(
                    GradientÖrneği::TÜMÜ.into_iter().map(|örnek| {
                        div()
                            .id(SharedString::from(format!(
                                "gradients-{}-surface",
                                örnek.kimlik()
                            )))
                            .flex_none()
                            .w(px(800.0))
                            .h(px(600.0))
                            .when_some(yüzey(örnek), |öğe, grafik| öğe.child(grafik))
                    }),
                ))
        } else if aktif_kart == KartKimliği::SyncCursor {
            let cpu = sync_yüzeyi(SyncCursorÖrneği::Cpu);
            let ram = sync_yüzeyi(SyncCursorÖrneği::Ram);
            let tcp = sync_yüzeyi(SyncCursorÖrneği::Tcp);
            let kırmızı_mavi = sync_yüzeyi(SyncCursorÖrneği::UyumsuzKırmızıMavi);
            let yeşil_kırmızı = sync_yüzeyi(SyncCursorÖrneği::UyumsuzYeşilKırmızı);
            let sync_paneli =
                |örnek: SyncCursorÖrneği, grafik: Option<Entity<GpuiGrafik>>| {
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
                    div()
                        .flex_1()
                        .min_w_0()
                        .h(px(236.0))
                        .child(
                            div()
                                .w_full()
                                .h(px(206.0))
                                .when_some(grafik, |öğe, grafik| öğe.child(grafik)),
                        )
                        .child(div().flex().items_center().gap_1().children(
                            seriler.into_iter().map(|(indeks, etiket, _, görünür)| {
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
                                .tiklaninca(cx.listener(
                                    move |bu, _, _, cx| {
                                        bu.sync_cursor_serisini_değiştir(örnek, indeks, cx);
                                    },
                                ))
                            }),
                        ))
                };
            çizim_tabanı
                .flex_none()
                .h(px(760.0))
                .overflow_y_scroll()
                .p_2()
                .child(sync_paneli(SyncCursorÖrneği::Cpu, cpu))
                .child(div().mt_2().flex().gap_2().children([
                    sync_paneli(SyncCursorÖrneği::Ram, ram),
                    sync_paneli(SyncCursorÖrneği::Tcp, tcp),
                ]))
                .child(div().mt_2().flex().gap_2().children([
                    sync_paneli(SyncCursorÖrneği::UyumsuzKırmızıMavi, kırmızı_mavi),
                    sync_paneli(SyncCursorÖrneği::UyumsuzYeşilKırmızı, yeşil_kırmızı),
                ]))
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
            let birleşik_görünürlük = [
                (TimeseriesDiscreteÖrneği::ZamanSerisi, 0),
                (TimeseriesDiscreteÖrneği::AyrıkDurumlar, 0),
                (TimeseriesDiscreteÖrneği::AyrıkDurumlar, 1),
                (TimeseriesDiscreteÖrneği::AyrıkDurumlar, 2),
            ]
            .map(|(örnek, seri)| {
                self.timeseries_discrete_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .is_some_and(|(_, grafik)| grafik.read(cx).grafik().seri_görünür_mü(seri))
            });
            çizim_tabanı
                .flex_none()
                .h(px(760.0))
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .w(px(1920.0))
                        .h(px(600.0))
                        .when_some(üst, |öğe, grafik| öğe.child(grafik)),
                )
                .child(
                    div()
                        .mt_2()
                        .w(px(1920.0))
                        .h(px(200.0))
                        .when_some(alt, |öğe, grafik| öğe.child(grafik)),
                )
                .child(
                    div().mt_2().flex().gap_2().children(
                        ["Value", "DEV1", "DEV2", "DEV3"]
                            .into_iter()
                            .enumerate()
                            .map(|(indeks, etiket)| {
                                let görünür =
                                    birleşik_görünürlük.get(indeks).copied().unwrap_or(false);
                                div()
                                    .id(SharedString::from(format!(
                                        "timeseries-discrete-toggle-{indeks}"
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
                                    .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                                        bu.timeseries_discrete_serisini_değiştir(indeks, cx);
                                    }))
                            }),
                    ),
                )
        } else if aktif_kart == KartKimliği::TimezonesDst {
            çizim_tabanı
                .flex_none()
                .h(px(900.0))
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .items_start()
                        .gap_4()
                        .children((0..11).map(|bölüm| {
                            let yüzeyler = self
                                .timezones_dst_grafikleri
                                .iter()
                                .filter(|(örnek, _)| örnek.bölüm_indeksi() == bölüm)
                                .map(|(örnek, grafik)| (*örnek, grafik.clone()))
                                .collect::<Vec<_>>();
                            let başlık = yüzeyler
                                .first()
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
                                .children(yüzeyler.into_iter().map(|(_, grafik)| {
                                    div().w(px(600.0)).h(px(200.0)).child(grafik)
                                }))
                        })),
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
            çizim_tabanı
                .flex_none()
                .h(px(1240.0))
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
                .children(tam.into_iter().map(|örnek| {
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
                                .h(px(300.0))
                                .when_some(yüzey(örnek), |öğe, grafik| öğe.child(grafik)),
                        )
                }))
                .child(
                    div()
                        .mt_2()
                        .flex()
                        .gap_2()
                        .children(küçük.into_iter().map(|örnek| {
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
                                        .h(px(250.0))
                                        .when_some(yüzey(örnek), |öğe, grafik| öğe.child(grafik)),
                                )
                        })),
                )
        } else if aktif_kart == KartKimliği::MissingData {
            çizim_tabanı
                .flex_none()
                .h(px(1280.0))
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
                                .h(px(520.0))
                                .when_some(grafik, |öğe, grafik| öğe.child(grafik)),
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
                .flex_none()
                .h(px(1160.0))
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
                        .child("Her X UTC'de ayın ilk günüdür. Kaynak 28 günlük piksel-space kuralı gerçek takvim ayı bölmelerini korur; 2024 Şubat 29 gündür. ")
                        .child("Üçüncü yüzey months-ru.html fmtDate adlarını gösterir; yerelleştirme timestamp'i değiştirmez."),
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
                        (
                            2,
                            "2017–2019 · Rusça tarih adları",
                            "Aynı UTC takvimi, ruNames ile yerelleştirilmiş ay/hafta adları.",
                            600.0,
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
                                    .child(
                                        div()
                                            .w(px(1920.0))
                                            .h(px(yükseklik))
                                            .when_some(yüzey(indeks), |öğe, grafik| {
                                                öğe.child(grafik)
                                            }),
                                    ),
                            )
                    }),
                )
        } else if aktif_kart == KartKimliği::PathGapClip {
            let gruplar: [(&str, &str, &[PathGapClipÖrneği]); 5] = [
                (
                    "Band ve canlı spanGaps",
                    "İki band yüzeyi ile join yüzeyleri aynı bir saniyelik bridge durumunu paylaşır.",
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
                .flex_none()
                .h(px(1260.0))
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
                            let (genişlik, yükseklik) = örnek.kaynak_boyutu();
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
                                            .h(px(yükseklik as f32))
                                            .overflow_x_scroll()
                                            .child(
                                                div()
                                                    .w(px(genişlik as f32))
                                                    .h(px(yükseklik as f32))
                                                    .child(grafik),
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
                .flex_none()
                .h(px(860.0))
                .overflow_y_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("İki panel aynı halka verisini ve aynı animation-frame saatini paylaşır. Üst panel koordinatları tam piksele yuvarlayarak keskin fakat basamaklı “tırtıl” hareketi; alt panel alt-piksel konumlarını koruyarak daha yumuşak hareket üretir."),
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
                                .h(px(360.0))
                                .when_some(yüzey(örnek), |öğe, grafik| öğe.child(grafik)),
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
                .flex_none()
                .h(px(900.0))
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
                    let (genişlik, yükseklik) = örnek.kaynak_boyutu();
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
                                .h(px(yükseklik as f32))
                                .overflow_x_scroll()
                                .child(
                                    div()
                                        .w(px(genişlik as f32))
                                        .h(px(yükseklik as f32))
                                        .when_some(yüzey(örnek), |öğe, grafik| {
                                            öğe.child(grafik)
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
                                    let (genişlik, yükseklik) = örnek.boyut();
                                    div()
                                        .flex_none()
                                        .w(px(genişlik as f32))
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
                                                .w(px(genişlik as f32))
                                                .h(px(yükseklik as f32))
                                                .when_some(yüzey(örnek), |öğe, grafik| {
                                                    öğe.child(grafik)
                                                }),
                                        )
                                }),
                        ),
                    )
            };
            çizim_tabanı
                .flex_none()
                .h(px(2100.0))
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
                .flex_none()
                .h(px(1320.0))
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
                                .child(
                                    div()
                                        .w(px(1920.0))
                                        .h(px(600.0))
                                        .when_some(yüzey(örnek), |öğe, grafik| {
                                            öğe.child(grafik)
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
                .flex_none()
                .h(px(760.0))
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
                                .when_some(grafik, |öğe, grafik| öğe.child(grafik)),
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
                .flex_none()
                .h(px(760.0))
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
                                .w(px(1275.0))
                                .h(px(600.0))
                                .when_some(grafik, |öğe, grafik| öğe.child(grafik)),
                        )
                }))
        } else if matches!(aktif_kart, KartKimliği::BoxWhisker(_)) {
            çizim_tabanı
                .flex_none()
                .h(px(980.0))
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
                .flex_none()
                .h(px(1160.0))
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Resmî soft-minmax.html tek veri nesnesini paylaşan dört canlı rangeNum karşılaştırmasını ve bağımsız düz-sıfır yüzeyini birlikte gösterir. ▶ dataMax++ tek ortak değeri her 50 ms’de dört canlı yüzeye aynı adımda uygular."),
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
                                        .w(px(400.0))
                                        .h(px(400.0))
                                        .when_some(yüzey(örnek), |öğe, grafik| öğe.child(grafik)),
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
                .flex_none()
                .h(px(940.0))
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
                                .w(px(800.0))
                                .h(px(400.0))
                                .when_some(yüzey(örnek), |öğe, grafik| öğe.child(grafik)),
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
                .flex_none()
                .h(px(430.0))
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
                                        .when_some(yüzey(hacim), |öğe, grafik| öğe.child(grafik)),
                                )
                                .child(
                                    div()
                                        .w(px(150.0))
                                        .h(px(30.0))
                                        .bg(rgb(0xffc0cb))
                                        .when_some(yüzey(kapanış), |öğe, grafik| {
                                            öğe.child(grafik)
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
                .flex_none()
                .h(px(700.0))
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
                                .when_some(yüzey(örnek), |öğe, grafik| öğe.child(grafik)),
                        )
                }))
        } else if matches!(aktif_kart, KartKimliği::StackedSeries(_)) {
            çizim_tabanı
                .flex_none()
                .h(px(760.0))
                .overflow_scroll()
                .p_2()
                .children(StackedSeriesÖrneği::TÜMÜ.into_iter().map(|örnek| {
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
                    let (genişlik, yükseklik) = örnek.boyut();
                    div()
                        .mb_4()
                        .child(
                            div()
                                .w(px(genişlik as f32))
                                .h(px(yükseklik as f32))
                                .border_1()
                                .border_color(rgb(0xc0c0c0))
                                .when_some(grafik, |öğe, grafik| öğe.child(grafik)),
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
                }))
        } else if matches!(aktif_kart, KartKimliği::StreamData(_)) {
            çizim_tabanı
                .flex_none()
                .h(px(760.0))
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
                        .when_some(grafik, |öğe, grafik| öğe.child(grafik))
                }))
        } else if matches!(aktif_kart, KartKimliği::ThinBars(_)) {
            let yüzey = |örnek| {
                self.thin_bars_grafikleri
                    .iter()
                    .find(|(kimlik, _)| *kimlik == örnek)
                    .map(|(_, grafik)| grafik.clone())
            };
            let örnekler = ThinBarsÖrneği::tümü();
            let yoğunluklar = örnekler.iter().take(7).copied().collect::<Vec<_>>();
            let geometri_grupları = örnekler
                .iter()
                .skip(7)
                .copied()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|grup| grup.to_vec())
                .collect::<Vec<_>>();
            çizim_tabanı
                .flex_none()
                .h(px(2100.0))
                .overflow_scroll()
                .p_2()
                .child(
                    div()
                        .w(px(1600.0))
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xf8fafc))
                        .text_xs()
                        .text_color(soluk)
                        .child("Resmî thin-bars-stroke-fill.html sayfası 7 yoğunluk yüzeyini ve 12 align/width/gap grubundaki 48 geometri yüzeyini birlikte gösterir. Yüzeyler veri veya cursor paylaşmaz; her biri bağımsız zoom geçmişi tutar. Noktalar görünür X aralığındaki piksel açıklığı yeterli olduğunda otomatik açılır."),
                )
                .child(
                    div().w(px(1600.0)).flex().flex_wrap().gap_2().children(
                        yoğunluklar.into_iter().map(|örnek| {
                            let (genişlik, yükseklik) = örnek.boyut();
                            div()
                                .flex_none()
                                .w(px(genişlik as f32))
                                .h(px(yükseklik as f32))
                                .border_1()
                                .border_color(rgb(0xe5e7eb))
                                .when_some(yüzey(örnek), |öğe, grafik| öğe.child(grafik))
                        }),
                    ),
                )
                .children(geometri_grupları.into_iter().map(|grup| {
                    div()
                        .w(px(1600.0))
                        .flex()
                        .border_t_1()
                        .border_color(rgb(0xd1d5db))
                        .pt_2()
                        .children(grup.into_iter().map(|örnek| {
                            div()
                                .flex_none()
                                .w(px(400.0))
                                .h(px(200.0))
                                .border_1()
                                .border_color(rgb(0xe5e7eb))
                                .when_some(yüzey(örnek), |öğe, grafik| öğe.child(grafik))
                        }))
                }))
        } else if matches!(aktif_kart, KartKimliği::TimePeriods(_)) {
            çizim_tabanı
                .flex_none()
                .h(px(760.0))
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
                                .when_some(grafik, |öğe, grafik| öğe.child(grafik)),
                        )
                }))
        } else if matches!(aktif_kart, KartKimliği::TimelineDiscrete(_)) {
            çizim_tabanı
                .flex_none()
                .h(px(760.0))
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
                                .when_some(grafik, |öğe, grafik| öğe.child(grafik)),
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
            let boyut = self
                .boyut_senkron_akışı
                .map_or(800, BoyutSenkronAkışı::boyut);
            çizim_tabanı.overflow_hidden().child(
                div()
                    .w(px(boyut as f32))
                    .h(px(boyut as f32))
                    .when_some(self.grafik.clone(), |öğe, grafik| öğe.child(grafik)),
            )
        } else if aktif_kart == KartKimliği::ScrollSync {
            çizim_tabanı
                .flex_none()
                .h(px(400.0))
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
                            .w(px(400.0))
                            .h(px(200.0))
                            .when_some(self.grafik.clone(), |öğe, grafik| öğe.child(grafik)),
                    )
                    .child("Grafiği kaydırdıktan sonra imleç ve seçim aynı görsel noktada kalır. Kaynak parity için doğal kapsayıcı kaydırması varsayılandır; wheel/touch yakınlaştırma ortak API'den isteğe bağlı açılır."),
                )
        } else {
            çizim_tabanı
                .overflow_hidden()
                .when_some(self.grafik.clone(), |öğe, grafik| öğe.child(grafik))
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
        let kullanım_rehberi = match aktif_kart {
            KartKimliği::TooltipsClosest => Some(
                "Amaç: dört rustc-perf çalışma kipinden imlece beş CSS piksel içinde en yakın \
                 görünür seriyi bulur; commit, değer ve başlangıca göre değişimi gerçek veri \
                 noktasına bağlı tek kutuda gösterir. API: OdakDüzeni yakınlık/alfa kararını, \
                 EnYakınTooltipDüzeni commit dizisini, 100 interpolasyon indeksini ve perf URL \
                 istatistiğini çekirdekte tutar; lejant setSeries, alan seçimi X+Y çalışır. \
                 İzleme: derleyici, servis veya sürüm regresyonunda aynı commit anındaki çalışma \
                 kiplerini karşılaştırmak için uygundur; yerinde plot tıklaması karşılaştırma \
                 bağlantısını açar, sürükleme bağlantı açmaz. Maliyet: 234×4 çizgi noktası ve \
                 100 dikey kılavuz vardır; kılavuzlar tek path komutunda boyanır, pointer araması \
                 O(log N + görünür seri sayısı) ve kutu ana yolları yeniden çizmeden taşınır. \
                 Tarih metni platformlar arası belirlenim için UTC'dir; kaynak browser-local \
                 Date kullandığından bu bilinçli, belgeli tek sunum farkıdır.",
            ),
            KartKimliği::Tooltips => Some(
                "Amaç: ham imleç X/Y konumu ile en yakın veri indeksindeki görünür seri \
                 noktalarını ayrı, hafif bilgi kutularında gösterir; kaynak örneğin her iki \
                 saniyede destroy/new uPlot yaşam döngüsünde imleç konumunu koruma sınamasını \
                 sürdürür. API: TooltipDüzeni imleç ve seri kutularını, yeniden_kurma_ms yaşam \
                 döngüsünü ve dış cursor memo kararını tanımlar; lejant setSeries ile One ve \
                 varsayılan gizli Two serisini aynı yüzeyde açıp kapatır. İzleme: cursor \
                 konumu ile örneklenmiş ölçümün farklı olduğunu geliştiriciye açıkça göstermek \
                 ve panel yeniden kurulurken inceleme bağlamının kaybolmamasını sınamak için \
                 uygundur. Maliyet: veri yalnız 7×2'dir; ana yollar yalnız setSeries, ölçek veya \
                 kasıtlı iki saniyelik kaynak yeniden kurulumunda boyanır. Normal pointer \
                 hareketi önbellekli ana yüzeye dokunmaz, yalnız mevcut tooltip katmanlarını \
                 ve cursor çizgilerini taşır.",
            ),
            KartKimliği::Trendlines => Some(
                "Amaç: her görünür serinin ekrandaki ilk ve son gerçek veri indeksini kaynak \
                 drawSeries kancası gibi 5/5 kesik bir uç çizgisiyle bağlar; normal path'in \
                 kırpma için görünüm dışı komşu noktaları kullanabilmesi bu i0/i1 kararını \
                 değiştirmez. API: ÇizimKancasıDüzeni::seri_uç_trendleri kesik aralığını, \
                 x_aralığını_veriye_yapıştır ise seçim ve wheel uçlarının valToIdx eşdeğeriyle \
                 gerçek X değerlerine oturmasını sağlar; lejant setSeries ana yol, dolgu ve \
                 trendi birlikte açıp kapatır. İzleme: seçili zaman penceresindeki genel \
                 başlangıç-son eğilimini ham dalgalanmanın üzerinde okumak için uygundur; \
                 regresyon değildir ve ara noktaları modellemez. Maliyet: iki 100 noktalı yol \
                 ve seri başına tek ek çizgi O(görünür N)'dir. Pointer yalnız cursor/lejant \
                 katmanını taşır; uçlar yalnız ölçek, resize veya setSeries sonrasında yeniden \
                 hesaplanır. Kaynak points.space=10 ve tek-piksel yarım-piksel hizası korunur.",
            ),
            KartKimliği::CursorSnap => Some(
                "Amaç: cursor çizgilerini ve alan seçiminin iki ucunu kaynak cursor.move \
                 callback'i gibi aynı 10×10 CSS piksel ızgarasına oturtur; hover noktaları \
                 dönüştürülmüş X'e en yakın gerçek veri örneğinde kalır. API: \
                 GrafikSeçenekleri::imleç_ızgara_adımı dönüşüm sahipliğini çekirdeğe taşır; \
                 GPUI ve WASM cursor, seçim başlangıcı ve seçim bitişinde aynı sonucu kullanır. \
                 Lejant setSeries ile üç dolu çizgi serisini ayrı açıp kapatır. İzleme: \
                 gürültülü zaman serilerinde tekrarlanabilir piksel adımlarıyla karşılaştırma \
                 ve zoom penceresi seçmek içindir. Maliyet: snap O(1), hizalı en yakın X \
                 araması O(log N)'dir; normal pointer hareketi ana üç yolu yeniden çizmez, \
                 yalnız hafif cursor/hover/lejant katmanını günceller.",
            ),
            KartKimliği::CursorTooltip => Some(
                "Amaç: tek yeşil serideki en yakın X/Y örneğini ve plot alanına göre CSS \
                 piksel cursor konumunu hafif bir bilgi kutusunda gösterir. API: \
                 bilgi_kutusunu_yerleştir kaynak placement.js right/start kuralını gerçek \
                 biçimlendirilmiş metin genişliği, plot sınırı ve 12 piksellik boşlukla \
                 çekirdekte hesaplar; sağ alan yetmezse kutu imlecin soluna döner. İzleme: \
                 bir telemetri örneğinin zaman ve değerini ana çizimi değiştirmeden hızlıca \
                 okumak içindir. Maliyet: en yakın X araması O(log N), yerleşim O(1)'dir; \
                 GPUI ana yol önbelleğini korur ve yalnız etkileşim canvas'ı ile overlay'i \
                 yeniler, WASM pointer olaylarını requestAnimationFrame ile birleştirip aynı \
                 SVG/path düğümlerini yerinde tutar.",
            ),
            KartKimliği::CustomScales => Some(
                "Amaç: aynı 199×4 kaynak veriyi ve 20 siyah draw noktasını resmî sayfadaki \
                 sırayla üç bağımsız 800×800 yüzeyde karşılaştırır: doğrusal, log10/log10 ve \
                 log10 X + özel Weibull Y. API: custom_scales_kartları aynı veri/seri/bant \
                 tanımından üç Grafik üretir; YÖlçekSeçenekleri::özel adlandırılmış fwd/bwd \
                 fonksiyonlarını, y_sabit_bölmeler ile y_özel_etiketler kaynak axis callback \
                 sonuçlarını taşır. İzleme: olasılık ve güven aralığı verisinde ham, log ve \
                 dağılıma özgü görünümün aynı örnekleri nasıl ayırdığını kıyaslamak içindir. \
                 Üç yüzeyin cursor, zoom, pan ve geçmiş durumları paylaşılmaz. Maliyet: ilk \
                 kurulum üç retained sahnede O(3N)'dir; pointer yalnız hafif etkileşim katmanını \
                 günceller, ana band/path yalnız ölçek, resize veya görünürlük değişiminde \
                 yeniden üretilir.",
            ),
            KartKimliği::DataSmoothing => Some(
                "Amaç: resmî Taxi Trips verisinin ham halini Savitzky–Golay, ASAP FFT ve \
                 300 örneklik hareketli ortalama sonuçlarıyla aynı sayfada, kaynak sırasıyla \
                 karşılaştırır. Dört 1920×300 yüzey bağımsız Grafik örnekleridir; cursor, zoom, \
                 pan ve geçmiş durumlarını paylaşmaz. API: data_smoothing_kartları dört yüzeyi \
                 tek grupta döndürür; savitzky_golay, asap_yumuşat ve hareketli_ortalama sabit \
                 demo parametrelerinin hesaplama API'leridir. Y aralıkları kaynak gibi sabit, \
                 sol eksen 60 pikseldir. İzleme: yoğun zaman serisindeki genel eğilimi korurken \
                 gürültünün farklı yöntemlerle ne ölçüde bastırıldığını ve tepe davranışını \
                 kıyaslamak içindir. Maliyet: algoritmalar yalnız grup kurulurken bir kez \
                 çalıştırılır ve süreleri ayrı ölçülür; toplam 10.937 çizgi örneği retained \
                 sahnelere alınır. Pointer en yakın X'i bulup yalnız etkin yüzeyin hafif \
                 cursor/lejant katmanını günceller; yumuşatma ve ana yollar yeniden hesaplanmaz.",
            ),
            KartKimliği::FocusCursor => Some(
                "Amaç: resmî focus-cursor.html sayfasındaki bias, 30 px proximity, dinamik \
                 setSeries stili ve 300 seri performans yüzeylerini aynı sayfada kaynak sırasıyla \
                 karşılaştırır. API: focus_cursor_kartları dört bağımsız Grafik döndürür; ilk iki \
                 yüzey aynı immutable HizalıVeri Arc deposunu paylaşır. seri_odak_sunumu, \
                 odak değiştiğinde yalnız stroke/fill/width boya sonucunu verir. İzleme: yoğun \
                 CPU/RAM zaman serilerinde imlece en yakın seriyi ayrıntılandırıp diğerlerini \
                 soluklaştırmak için uygundur. Maliyet: 130K veri ikinci kez tahsis edilmez; GPUI \
                 retained ana yolları korur, pointer yalnız etkileşim katmanı ile seri boya \
                 durumunu günceller. Ana geometri ancak veri, ölçek, resize veya zoom değişince \
                 yeniden kurulur.",
            ),
            KartKimliği::Gradients => Some(
                "Amaç: resmî gradients.html sayfasındaki yatay/dikey ayrık stroke, ArcSinh \
                 koordinatı, iki basınç dolgusu ve görünür min/orta/max dolgusunu kaynak \
                 sırasıyla tek kartta karşılaştırır. API: gradients_kartları beş bağımsız \
                 Grafik döndürür; dikey çift aynı data2, dolgu çifti aynı data4 HizalıVeri Arc \
                 deposunu paylaşır. ÖlçekGradyanı değer, ±sonsuz ve görünür_veri_oranı \
                 duraklarını; seri_imleç_rengi cursor point callback sonucunu taşır. İzleme: \
                 eşik bölgelerini çizgi/dolgu rengiyle vurgulamak ve görünür pencerenin basınç \
                 dağılımını okumak için uygundur. Maliyet: veri kopyalanmaz; pointer yalnız \
                 etkin yüzeyin cursor/lejant katmanını günceller. Gradyan ve ana geometri yalnız \
                 veri, ölçek, görünürlük, zoom/pan veya boyut değişiminde yeniden çözülür.",
            ),
            KartKimliği::GridOverSeries => Some(
                "Amaç: üç opak dolgulu serinin kesişimlerinde ızgara, çentik ve eksen bilgisini \
                 seri boyasının altında kaybetmeden gösterir. API: \
                 ÇizimSırası::SerilerEksenler kaynak drawOrder dizisini taşır; ızgara, X/Y \
                 çentik ve eksen/etiket renkleri CSS olmadan ayrı ayrı ayarlanabilir. Otomatik \
                 Y aralığı görünür X verisinden yeniden hesaplanır. İzleme: yoğun ve üst üste \
                 binen CPU, bellek veya ağ alanlarında ortak eşik düzlemini her serinin üzerinde \
                 okunabilir tutmak için uygundur. Maliyet: üç 30 noktalı seri tek retained \
                 yüzeyde çizilir. Eksen komutları geçici Vec ayırmadan yerinde rotate ile seri \
                 katmanının arkasından önüne alınır; pointer yalnız hafif cursor/lejant \
                 katmanını günceller.",
            ),
            KartKimliği::DrawHooks => Some(
                "Amaç: uPlot yaşam döngüsünün drawClear, drawSeries, özel points.show ve draw \
                 aşamalarının tek yüzeyde hangi sırayla birleştiğini gösterir. API: \
                 ÇizimKancasıDüzeni çok duraklı sürekli arka plan gradyanı, setData sırasında \
                 önbelleklenen seri medyanları, altı uçlu yıldız geometrisi ve gerçek sahne \
                 kurulum süresi stilini tanımlar. Siyah 10px eksen çentikleri ve kaynak veri \
                 birebir korunur; yorum satırındaki grid blur eklentisi bilinçli olarak etkin \
                 değildir. İzleme: Grafana benzeri zaman serilerinde eşik/medyan vurgusu, özel \
                 veri işareti ve çizim maliyeti telemetrisi eklemek için uygundur. Maliyet: \
                 medyan sıralaması yalnız ilk kurulum ve setData sırasında O(S·N logN) çalışır; \
                 drawSeries önbelleği O(S) tüketir. Pointer ana yolları, yıldızları, gradyanı \
                 veya medyanları yeniden üretmeden yalnız cursor/lejant katmanını taşır.",
            ),
            KartKimliği::MissingData => Some(
                "Amaç: aynı resmî sayfadaki iki bağımsız yüzeyi birlikte karşılaştırır. İlk \
                 yüzey gerçek null CPU/RAM örneklerinin yolu nasıl böldüğünü ve TCP Out'un \
                 bağımsız MB ölçeğini; ikinci yüzey dolu değerlerde komşu X farkı 1'i aşınca \
                 series.gaps ile oluşan boşluğu gösterir. API: missing_data_kartları iki ayrı \
                 Grafik örneğini tek kaynak grubunda döndürür; görünüm ve cursor durumları \
                 bilinçli olarak senkronlanmaz. Seri anahtarları setSeries görünürlüğünü ve \
                 autoscale'ı yüzeyinde günceller. İzleme: veri gerçekten yokken oluşan null \
                 kesintisini, örnekleme zamanındaki büyük aralıktan ayırmak içindir. Maliyet: \
                 yollar yalnız setSeries, ölçek veya resize sırasında O(N) yeniden kurulur; \
                 pointer yalnız hafif cursor/lejant katmanını günceller.",
            ),
            KartKimliği::UpdateCursorSelectResize => Some(
                "Amaç: setCursor, cursor._lock ve setSelect ile kurulmuş kalıcı etkileşim \
                 durumunun setSize sırasında çizim alanı oranlarında kalmasını gösterir. API: \
                 BoyutSenkronDüzeni yalnız başlangıç cursor/select/hover oranlarını taşır; \
                 Grafik::boyutu_ayarla veri ve ölçeği koruyarak ana sahneyi yeniden boyar. GPUI \
                 adaptörü ana veri sahnesinden ayrı etkileşim canvas'ında, WASM adaptörü ise aynı \
                 SVG içinde kimliği değişmeyen overlay düğümlerinde durumu saklar. Lejant \
                 setSeries kırmızı yolu ve hover noktasını birlikte gizler. İzleme: panel veya \
                 pencere boyutu değişirken kullanıcının kilitli inceleme konumunu kaybetmemek \
                 içindir. Maliyet: kaynak gibi setSize ana yolları yeniden çizer; cursor, seçim \
                 ve hover için ikinci bir ana yol üretmez, yalnız hafif katman koordinatları \
                 güncellenir. 100 ms zamanlayıcı karttan çıkıldığında durdurulur.",
            ),
            KartKimliği::WindDirection => Some(
                "Amaç: sıcaklık çizgisini ve sabit 0…30 m/s ölçekli rüzgâr hızını aynı hizalı \
                 zaman dizisinde gösterir; üçüncü seri, hız konumlarından derece yönüne uzanan \
                 15 CSS piksellik vektörleri özel path olarak üretir. API: \
                 RüzgarYönüDüzeni::yeni hız/yön serisini ve ölçeği bağlar; stil ile vektör \
                 uzunluğu, rengi ve kalınlığı CSS olmadan geliştirici tarafından seçilebilir. \
                 Direction serisinin auto=false kararı dereceleri Y aralığından çıkarır; \
                 lejant setSeries ile üç katman bağımsız açılıp kapanır. İzleme: sıcaklık, \
                 rüzgâr hızı ve yönü gibi aynı zamanlı fakat farklı birimli telemetriyi tek \
                 inceleme yüzeyinde ilişkilendirmek içindir. Maliyet: 139 vektör kaynak gibi \
                 tek beginPath/stroke eşdeğeri Yol komutunda toplu boyanır; görünüm sınırındaki \
                 dış komşular getOuterIdxs eşdeğeriyle korunur. Pointer yalnız hafif cursor \
                 katmanını taşır; ana yollar setSeries, ölçek veya resize ile yeniden hesaplanır.",
            ),
            KartKimliği::YScaleDrag => Some(
                "Amaç: sayısal X ile meter ve km/h adlı iki bağımsız Y ölçeğini doğrudan eksen \
                 üzerinden kaydırır; Shift basılıyken iki uç ters yönde hareket ederek aralığı \
                 büyütür veya daraltır. API: eksen_vuruşu_boyutta gerçek çizim payından hedef \
                 ölçeği seçer; eksen_sürüklemeyi_başlat/sürükle/bitir kaynak setScale yaşam \
                 döngüsünü taşır. Otomatik Y ekseni hesabı kaynak callback'indeki \
                 25 + en_uzun_etiket × 6 piksel formülünü her aralıkta yeniden uygular; lejant \
                 setSeries ilgili elle sürüklenmiş ölçeği otomatik aralığa döndürür. İzleme: \
                 farklı birimli metriklerin ayrıntı düzeyini paneli yeniden kurmadan ayrı ayrı \
                 ayarlamak için uygundur. Maliyet: hareketler ekran karesiyle birleştirilir; \
                 setScale eksen, grid ve iki kısa yolu yeniden boyar, cursor katmanı yerinde \
                 kalır. WASM pointer capture, GPUI dışarıda mouse-up temizliğiyle sürüklemeyi \
                 yüzey sınırının dışında da güvenle tamamlar.",
            ),
            KartKimliği::YShiftedSeries => Some(
                "Amaç: aynı 30×3 ham ölçümü iki saniyede bir normal 0…10 düzlemi ile \
                 Core #1/#2/#3 için 0/+10/+20 kaydırılmış şerit düzlemi arasında değiştirir. \
                 Kırmızı ve yeşil alanların fillTo tabanları 0/10, mavi bars Path2D tabanı \
                 20'dir; lejant series.value gibi her zaman ham 0…10 değerini gösterirken \
                 hover noktası gerçek kaydırılmış geometride kalır. API: \
                 YShiftedSeriesAkışı::ilerlet_güncellemesi yalnız yeni veri, range, axis values \
                 ve fillTo tabanlarını üretir; Grafik::veriyi_y_sunumunda_ayarla aynı Grafik \
                 örneğinde atomik setData uygular. Lejant setSeries görünürlüğü kip geçişinde \
                 korunur. İzleme: aynı ölçekli çekirdek, pod veya kuyruk metriklerini üst üste \
                 binmeden ayrı şeritlerde izleyip ham değerlerini karşılaştırmak içindir. \
                 Maliyet: seçenek ağacı, GPUI entity'si, SVG kabuğu ve etkileşim bağları yeniden \
                 kurulmaz; 30 mavi çubuk tek dolgu ve tek stroke yolunda toplanır. Timer karttan \
                 çıkıldığında iptal edilir, cursor hafif katmanda aynı konumdan yeniden çözülür.",
            ),
            KartKimliği::DependentScale => Some(
                "Amaç: tek Fahrenheit veri yolunu iki birimde okumayı sağlar; Celsius ekseni \
                 ikinci bir seri veya ikinci çizim yolu değildir. API: \
                 YÖlçekSeçenekleri::sayısal_aralık resmî rangeNum(40,80,.1,true) sonucunu, \
                 kaynak_dönüşümü z.from=y ilişkisini ve eksen_en_az_etiket_boşluğu sağ \
                 axis.space=20 davranışını taşır. Lejant setSeries ile aynı Grafik örneğindeki \
                 blah serisini açıp kapatır. İzleme: sıcaklık, hız veya kapasite gibi doğrusal \
                 dönüştürülebilen aynı telemetriyi iki birim sisteminde gösterin; X ya da Y \
                 görünümü değiştiğinde türetilmiş eksen kaynak ölçeğin min/max dönüşümünü \
                 korur. Maliyet: yalnız bir 7 noktalı çizgi O(N) üretilir; ikinci eksen \
                 dönüşümü ve bölmeleri O(1) ek maliyettir. Pointer yalnız hafif cursor/lejant \
                 katmanını taşır; ana yol setSeries, görünüm veya boyut değişiminde yenilenir.",
            ),
            KartKimliği::ArcSinhScales => Some(
                "Amaç: sıfır çevresindeki küçük değişimleri doğrusal, büyük pozitif ve negatif \
                 büyüklükleri logaritmik okunabilirlikle aynı eksende gösterir. API: \
                 YÖlçekSeçenekleri::arcsinh doğrusal merkez eşiğini tanımlar; \
                 y_arcsinh_eşiği_ayarla aynı Grafik örneğinde ham aralığı ve görünüm geçmişini \
                 koruyarak geometriyi yeniler. Lejant setSeries ile Value serisini açıp \
                 kapatır. İzleme: artı ve eksi yönde birkaç mertebeye yayılan sapma, gecikme \
                 farkı veya bilanço telemetrisi için uygundur; wheel, seçim, pan ve touch ters \
                 ArcSinh dönüşümünü çekirdekte uygular. Maliyet: 111 noktalı tek yol ve \
                 decade/multiple ızgarası yalnız eşik, veri, görünüm veya boyut değişiminde \
                 O(N + tick) yenilenir; pointer ana yolu yeniden üretmez.",
            ),
            KartKimliği::AxisControl => Some(
                "Amaç: yarım milyon örnekte eksen yerleşimi ve sabit −50…50 Y düzlemini kaynak \
                 sinyal ayrıntısını kaybetmeden doğrular. API: \
                 YÖlçekSeçenekleri::eksen_en_az_etiket_boşluğu axis.space=50'yi; \
                 birincil_y_sağda, eksen rengi/genişliği ve X/Y etiketleri resmî eksen \
                 yapılandırmasını taşır. Lejant setSeries ile sin(x) yolunu açıp kapatır. \
                 İzleme: yoğun ve sabit sınırla karşılaştırılması gereken telemetri içindir; \
                 wheel/seçim görünür X dilimini daralttığında kovalar yalnız o dilimde kurulur. \
                 Maliyet: 500.001 değer bellekte korunur; her piksel kovasında ilk/min/maks/son \
                 adaylarıyla sahne O(plot width) noktaya iner, pointer ana yolu yeniden kurmaz.",
            ),
            KartKimliği::AxisAutosize => Some(
                "Amaç: aynı 501 noktalı sinyal 500 ms aralıklarla 10 kat büyürken X son etiketi \
                 ile Y değerleri için gereken eksen payının kendini yeniden ölçmesini gösterir. \
                 API: AxisAutosizeAkışı kaynak 1…10⁹ yaşam döngüsünü yürütür; \
                 Grafik::canlı_veriyi_x_etiket_çarpanında_ayarla aynı Grafik örneğinde setData \
                 ve X values çarpanını atomik yeniler. Lejant setSeries görünürlüğü tikler \
                 boyunca korunur. İzleme: büyüklüğü çalışma anında birkaç mertebe değişebilen \
                 sayaç, kapasite ve finans telemetrisinde etiket kırpılmasını önlemek içindir; \
                 ortak wheel, seçim, pan ve touch görünümü veri güncellenirken kaybolmaz. \
                 Maliyet: her tikte 501 yeni değer O(N) üretilir; grafik, olay katmanları ve \
                 seçenek ağacı yeniden kurulmaz. Y etiketi genişliği ölçülür, sağ pay son gerçek \
                 X split'inde en fazla üç çevrimde yakınsar; görev 10⁹'da veya karttan çıkışta \
                 bırakılır.",
            ),
            KartKimliği::AxisIndicators => Some(
                "Amaç: aynı X örneğindeki üç bağımsız Y ölçeğini, ana grafik yollarını yeniden \
                 çizmeden renkli eksen rozetleri ve kılavuzlarla birlikte okumayı sağlar. API: \
                 her YÖlçekSeçenekleri kendi 50 px eksen dilimini, rengini ve aralığını taşır; \
                 axisIndicsPlugin karşılığı genel yatay cursor çizgisini kapatır ve yalnız \
                 görünür/dolu serilerin rozetlerini günceller. Lejant setSeries ile seri yolunu, \
                 noktasını ve rozetini birlikte açıp kapatır. İzleme: aynı zaman noktasındaki \
                 farklı birim veya büyüklüklerdeki CPU, bellek ve ağ metriklerini bağımsız \
                 ölçeklerde karşılaştırın; kırmızı serinin null aralığında yalnız kırmızı \
                 gösterge gizlenir. Maliyet: 30×3 ana yol yalnız veri, görünüm, boyut veya \
                 setSeries değişiminde üretilir; pointer dört hafif rozeti ve üç kılavuzu \
                 O(seri) konumlandırır, karta özel zamanlayıcı bırakılmaz.",
            ),
            KartKimliği::TimeseriesDiscrete => Some(
                "Amaç: aynı zaman eksenindeki sürekli telemetriyi ve ayrık cihaz durumlarını \
                 iki yükseklikte fakat tek etkileşim bağlamında karşılaştırır. API: \
                 timeseries_discrete_kartları üst float ve alt stepped yüzeyi birlikte döndürür; \
                 TimeseriesDiscreteGrubu ortak X imlecini, seçim/zoom görünümünü ve birleşik \
                 lejantı koordine eder, setSeries yalnız sahibi olan yüzeyi değiştirir. İzleme: \
                 CPU/yük gibi sürekli ölçümlerle servis, alarm veya cihaz açık-kapalı durumlarını \
                 aynı zaman noktasında okumak için uygundur. Maliyet: iki ana yüzey yalnız veri \
                 ya da ölçek değiştiğinde boyanır; cursor çizgileri ve birleşik lejant hafif \
                 katmanda güncellenir, veri yolları pointer hareketinde yeniden kurulmaz.",
            ),
            KartKimliği::ScalePadding => Some(
                "Amaç: farklı büyüklüklerdeki düz eşik ve taban çizgilerini tek Y ölçeğinde \
                 uçlara değmeden gösterir; kaynak rangeNum hesabı %10 payı dışa doğru uygun \
                 artıma yapıştırarak −13000…13000 üretir. API: YÖlçekSeçenekleri::sayısal_aralık \
                 alt/üst payı ve soft sınır kipini tanımlar; okunabilirliği ayrılması gereken \
                 metrik aileleri adlandırılmış farklı ölçeklere atanabilir. İzleme: alarm ve \
                 kapasite eşikleri için uygundur; ±0.1 ile ±10500 aynı ölçekteyse küçük değerlerin \
                 sıfıra yakın görünmesi doğrudur. Maliyet: 13×10 hizalı nokta O(S×N), imleç \
                 O(log N + S); cursor ve lejant ana yolları yeniden üretmeden güncellenir.",
            ),
            KartKimliği::Months => Some(
                "Amaç: gerçek UTC ay sınırlarını normal ve artık yıllarda karşılaştırır; Rusça \
                 yüzey yalnız sunum adlarını değiştirir. API: x tarih ölçeği, TarihAdları ve \
                 kaynak 28 günlük axes.space karşılığı takvim-ay bölmelerini belirler. İzleme: \
                 aylık faturalama, SLO ve kapasite raporlarında sabit 30 gün yerine gerçek ay \
                 sınırlarını kullanın. Maliyet: üç bağımsız yüzeyde toplam 108 nokta; çizim \
                 O(N+T), imleç O(log N). Resize bölmeleri yeniden hesaplar, veriyi üretmez.",
            ),
            KartKimliği::NiceScale => Some(
                "Amaç: panel yüksekliğine sığan okunabilir Y bölmelerini ve bu bölmelere tam \
                 oturan yuvarlak sınırları otomatik seçer. API: GüzelÖlçekDüzeni::yeni(30.0), \
                 kaynak niceNum eşiklerini (1/2/2,5/5/10), uçlarda %2 payı ve ArtımaGöre \
                 etiket biçimini birleştirir. İzleme: pencere veya panel boyutu değiştiğinde \
                 sabit tick sayısı yerine en az 30 piksel aralık korunur. Maliyet: altı X \
                 noktası ve üç seri değişmeden kalır; yalnız ölçek, ızgara ve yollar \
                 O(S×N+T) maliyetle yeniden boyanır.",
            ),
            KartKimliği::NoData => Some(
                "Amaç: boş veri, tek nokta, neredeyse düz ve tam düz serilerde otomatik \
                 sayısal aralığın kararlı kalmasını karşılaştırır. API: NoDataÖrneği::TÜMÜ \
                 kaynak 33 durumu tipli seçenekler olarak sunar; no_data_kartı seçili durumun \
                 zaman kipini, özel boş aralıklarını ve rangeNum eşdeğerini kurar. İzleme: \
                 veri gelmeden önce anlamlı bir aralık; tek veya sabit değer geldiğinde sıfır \
                 genişlikli olmayan güvenli bir ölçek gösterin. Maliyet: 33 eşzamanlı yüzey \
                 yerine seçili tanım aynı GPUI yüzeyinde değiştirilir ve yalnız eksenlerle en \
                 fazla iki nokta yeniden kurulur.",
            ),
            KartKimliği::PathGapClip => Some(
                "Amaç: gerçek null, join sırasında oluşan undefined/hizalama eksiği, band kırpması, \
                 stepped before/after ve tek-piksel gap sınırlarını kaynak sayfadaki 15 yüzeyle \
                 karşılaştırır. API: HizalıDeğer::{Değer, Boş, Tanımsız}, NULL_RETAIN/NULL_EXPAND \
                 join kipleri, linear/stepped/spline yolları ve spanGaps mutasyonu çekirdekte \
                 tanımlıdır; kaynakta setData/setScale yoktur. İzleme: scrape eksiğini gerçek \
                 ölçüm null'u gibi boyamayın; bridge açıldığında çizginin yalnız görsel olarak \
                 bağlandığını kullanıcıya belirtin. Maliyet: path/gap taraması O(N), sıralı imleç \
                 O(log N); pointer yalnız hafif overlay'i günceller, bir saniyelik animasyon yalnız \
                 dört kaynak yüzeyin ana yollarını yeniden kurar.",
            ),
            KartKimliği::PixelAlign => Some(
                "Amaç: aynı canlı telemetriyi aynı kayan 120 saniyelik pencerede tam piksel ve \
                 alt-piksel rasterizasyonuyla A/B karşılaştırır. API: grafik piksel_hizası eksen \
                 ve grid varsayılanını, seri piksel_hizası path/point override'ını belirler; \
                 PixelAlignAkışı 1 Hz örnek eklerken frame saati yalnız X ölçeğini ilerletir. \
                 İzleme: hizalama veriyi değiştirmez; pxAlign=1 keskin ve hızlı fakat tırtıllı, \
                 pxAlign=0 daha yumuşak fakat 1 px çizgilerde daha bulanık olabilir. Maliyet: \
                 halka ekleme O(1), her frame çizim O(görünür N×S); grafik örnekleri yeniden \
                 kurulmaz, yakınlaştırılmış görünüm canlı tam aralık ilerlerken sabit kalır.",
            ),
            KartKimliği::Points => Some(
                "Amaç: varsayılan nokta yoğunluğu, space=0 zorlaması, yalnız-nokta yolu ve \
                 gerçek boşluklar arasındaki tekil ölçümleri tek kaynak sayfasında karşılaştırır. \
                 API: points.space görünür piksel kapasitesini, paths:null yalnız marker çizimini, \
                 NoktaFiltreKipi::BoşlukArasındakiTekiller ise path gap sınırlarından seçilen \
                 indeksleri tanımlar. İzleme: seyrek olayları çizgiyle birleştirip süreklilik \
                 izlenimi vermeden gösterin; yoğun telemetride marker'ları otomatik gizleyerek \
                 ana eğriyi okunur tutun. Maliyet: dört statik yüzey toplam 3.321 X konumu tarar; \
                 yoğunluk testi O(1), gap filtresi O(N+G×99), çizilen marker sayısı O(k). \
                 Yakınlaştırma ve boyut değişimi filtreyi görünür piksel düzleminde yeniden hesaplar.",
            ),
            KartKimliği::ScalesDirOri => Some(
                "Amaç: aynı iki serinin dört yön kombinasyonunu, karşı eksen taraflarını ve X/Y \
                 yönelim değişimini tek matriste karşılaştırır. API: scale.dir veri yönünü, \
                 scale.ori fiziksel eksen yönelimini, axis.side eksenin top/right/bottom/left \
                 tarafını belirler. Direction Inversion sekiz 600×300; Orientation Inversion \
                 sekiz 320×600 yüzeydir. İzleme: ters akan süreçleri veya dikey zaman eksenini \
                 sunarken veri değerlerini dönüştürmeden fiziksel okumayı değiştirin. Maliyet: \
                 16 statik yüzeyin her biri aynı 10 X konumu ve iki seriyi O(S×N) çizer; timer \
                 yoktur. Cursor yalnız hafif etkileşim katmanlarını taşır; ölçek değişiminde \
                 senkron grubun 16 ana yüzeyi birlikte yeniden boyanır.",
            ),
            KartKimliği::ScrollSync => Some(
                "Amaç: kaydırılabilir panel içinde grafiğin pencere konumu değiştiğinde cursor, \
                 seçim ve zoom koordinatlarının görsel noktadan kopmamasını gösterir. API: \
                 adaptör güncel yüzey sınırını iletir; YüzeyDikdörtgeni istemci koordinatını \
                 aspect-fit sahneye dönüştürür. İzleme: sanallaştırılmış liste, kayan dashboard, \
                 sabit başlık veya yeniden yerleşen widget içindeki grafikler için gereklidir. \
                 Maliyet: sınır yenileme tek yerleşim ölçümü ve O(1) dönüşümdür; kaydırma ana \
                 veri sahnesini yeniden çizmez. Kaynak davranışını korumak için doğal kapsayıcı \
                 kaydırması varsayılandır; wheel/touch eklentileri ortak API'den açılabilir.",
            ),
            KartKimliği::SineStream => Some(
                "Amaç: tek grafik yüzeyinde 600 örnekli altı seriyi ekranın boya ritminde \
                 kaydırarak canlı izleme yükünü gösterir. API: SineAkışı::ilerlet yalnız bir \
                 örnek ilerletir; Grafik::veriyi_ayarla aynı Grafik ve GpuiGrafik örneğinde \
                 uPlot setData ölçek sıfırlamasını uygular. İzleme: telemetri, log oranı ve \
                 kaynak ölçümleri gibi sabit uzunluklu canlı pencereler için uygundur. \
                 Başlıktaki 60 FPS kaynak adıdır; gerçek hız ekran yenileme hızıdır. Maliyet: \
                 VecDeque pencere kaydırması O(1), veri aktarımı ve altı yolun çizimi \
                 O(seri×600); sabit eksen/grid yolları önbellekte, cursor/seçim katmanı \
                 güncellemeler arasında korunur.",
            ),
            KartKimliği::SoftMinMax(_) => Some(
                "Amaç: aynı iki noktalı verinin soft min mode 0/1/2/3 kararlarını yan yana \
                 karşılaştırır; beşinci yüzey düz sıfır veride iki taraflı −1…1 soft sınırını \
                 gösterir. API: soft_minmax_kartları tek kaynak sayfasının beş yüzeyini kaynak \
                 sırasıyla kurar; SayısalAralıkParçası pad, soft ve mode alanlarını tipli \
                 tanımlar. İzleme: sıfır tabanını sabit tutan oranlar ile küçük değişimlerde \
                 dikey çözünürlüğü koruyan telemetri politikalarını seçmek için uygundur. \
                 Maliyet: tek dataMax adımı yalnız ikişer noktalı dört grafiğe atomik setData \
                 uygular; düz-sıfır yüzeyi değişmez. Tekrarlanan başlatmalar engellenir; bu, \
                 kaynak örnekteki üst üste interval açabilme durumuna karşı kasıtlı güvenliktir.",
            ),
            KartKimliği::SparklinesBars(_) => Some(
                "Amaç: aynı sparkline ve yüzen low/high çubuklarını yalnız renk stratejisini \
                 değiştirerek kontrollü A/B karşılaştırır. API: sparklines_bars_kartları iki \
                 yüzeyi birlikte kurar; Floating Bars low değerlerini, \
                 yüzen_çubuk_üst_serisi özel high veri taşıyıcısını kullanır ve bu taşıyıcı \
                 otomatik ölçeğe katılmaz. İzleme: pozitif/negatif bölgeleri kesen sapma ve \
                 bütçe aralıklarında gradyan; kategorik eşiklerde açık nokta renkleri uygundur. \
                 Kaynak cursor/select/legend kapalıdır; ortak wheel/touch/drag yalnız geliştirici \
                 etkinleştirirse adaptör uzantısıdır. Maliyet: her yüzey 16 noktayı O(N) tarar; \
                 gradyan tek toplu alan komutudur, açık renk yolu 16 dikdörtgen üretir.",
            ),
            KartKimliği::Sparklines(_) => Some(
                "Amaç: yoğun bir izleme tablosunda 10 varlığın iki küçük zaman serisini tek \
                 bakışta karşılaştırır; kaynak sayfanın ilişkili 20 yüzeyi ayrı katalog \
                 kartlarına bölünmez. API: sparklines_kartları kaynak satır sırasıyla 20 \
                 (örnek, seçenekler, veri) üçlüsü döndürür; SparklineÖrneği::SATIRLAR \
                 Hacim/Kapanış çiftlerini tanımlar ve her yüzey bağımsız \
                 rangeNum(min,max,.1,true) Y aralığı kullanır. İzleme: hisse yerine servis, \
                 pod veya sensör; sütunlara trafik, hata, gecikme ya da son değer konabilir. \
                 Maliyet: kaynak Promise.all ile 10 CSV ve 20 canvas kurar; port doğrulanmış \
                 440 değeri binary içine gömerek fetch/parser yaşam döngüsünü kaldırır. \
                 Kaynak cursor/select/legend kapalıdır; ortak wheel/touch/drag yalnız \
                 geliştirici etkinleştirirse çekirdek uzantısıdır.",
            ),
            KartKimliği::Sparse(_) => Some(
                "Amaç: aynı seyrek telemetride optimize native linear, tek toplu özel kare \
                 noktalar ve naif moveTo/lineTo yolunun görünüm ve maliyet farkını karşılaştırır. \
                 API: sparse_kartları tek decode sonrası üç yüzeyi kaynak sırasıyla üretir; \
                 saf_doğrusal_yol native piksel kovasını atlar, kare points tek Alan/Path2D \
                 komutunda batch edilir. İzleme: uzun null koşularında native yol genel \
                 seçimdir; olay yoğunluğunda points, algoritma kıyasında naive kullanılır. \
                 Maliyet: native piksel başına giriş/min/max/çıkışı koruyup null koşularını \
                 tek kırılmaya indirir; points 4.430 kareyi tek fill path'te taşır; naive \
                 13.608 girdiyi tarayıp dolu noktalarla sınır kırpma kesişimlerini çizer.",
            ),
            KartKimliği::StackedSeries(_) => Some(
                "Amaç: tek kaynak sayfasındaki 16 bağımsız yüzeyi birlikte göstererek seri \
                 sırasının algıya etkisini, normal/yüzde/gruplu yığmayı ve null/undefined/zero \
                 ayrımını karşılaştırır. API: stacked_series_kartları kaynak DOM sırasıyla 16 \
                 (örnek, seçenekler, veri) üçlüsü döndürür; yalnız ilk dört yüzeyin lejant \
                 görünürlüğü kaynak setSeries hook'u gibi bantları yeniden kurup aynı grafik \
                 örneğinde setData uygular, kalan 12 yüzey yalnız görünürlüğü değiştirir. \
                 İzleme: toplam kapasite bileşenleri, pozitif/negatif bütçeler ve eksik örnek \
                 semantiğinin karşılaştırılması için uygundur; ilişkili varyasyonlar ayrı \
                 katalog kartlarına bölünmez. Maliyet: başlangıç aralıkları kaynak \
                 rangeNum(min,max,.1,true) ile sabittir; lejant güncellemesi yüzeyi yeniden \
                 yaratmaz. Kaynak yüzeyler arasında cursor/ölçek senkronu yoktur ve port da \
                 onları bağımsız tutar. Rastgele çubuk verisi tekrarlanabilir test için \
                 belgelenmiş tohuma bağlanır.",
            ),
            KartKimliği::StreamData(_) => Some(
                "Amaç: sabit uzunlukta kayan pencere, sürekli büyüyen veri ve sabit ölçekli \
                 büyüyen veri akışlarını aynı kaynak bağlamında karşılaştırır. API: \
                 StreamDataGrubu tek decode edilmiş Arc kaynağı paylaşır; kartları() üç \
                 bağımsız Grafik üretir, canlı_veriyi_ayarla seçenek ve yüzey ağacını koruyan \
                 setData karşılığıdır. İzleme: CPU/RAM/ağ yerine servis telemetrisi, log oranı \
                 veya sensör değerleri geçirilebilir. Maliyet: kaynak üç ayrı 100 ms timer \
                 kullanır; port aynı tikte tek scheduler ile üç yüzeyi günceller ve veri \
                 sonunda gereksiz kopya/çizimi durdurur. Cursor ve ölçekler yüzeyler arasında \
                 senkronlanmaz; wheel/touch/drag kaynak dışı isteğe bağlı çekirdek uzantısıdır.",
            ),
            KartKimliği::SvgImage => Some(
                "Amaç: canlı grafik ile rapor veya olay eki olarak saklanabilen bağımsız \
                 görüntü anlık görüntüsünü ayırır. API: svg_image_belgesi() başlık, eksen, pink \
                 arka plan ve mavi seriyi tek belirlenimci SVG belgesinde üretir; CLI örneği \
                 bunu dosyaya yazar. İzleme: dashboard panelini etkileşim durumundan bağımsız \
                 paylaşmak için uygundur. Maliyet: kaynak DOM outerHTML, CSS kuralları, \
                 foreignObject, Blob/Image ve iki DPR raster draw yapar; native port aynı \
                 içeriği tek sahne yürüyüşünde CSS/DOM bağımlılığı olmadan üretir. WASM kaynak \
                 eşliği için başlangıç SVG'sini bir kez DPR canvas'a rasterler; GPUI normal \
                 görünümünde ek string/Blob üretmez.",
            ),
            KartKimliği::SyncCursor => Some(
                "Amaç: ayrı CPU, RAM ve TCP yüzeyleriyle farklı seri sıralı iki karşılaştırma \
                 yüzeyini gerçek pub/sub ilişkileri içinde gösterir. API: SyncCursorGrubu \
                 cursor, etiket bazlı setSeries, mouseup/down filtresi ve görünür X hedeflerini \
                 çözer; CPU/RAM dikey cursor'ı ekran oranıyla değil aynı Y veri değerini hedef \
                 ölçeğe yeniden projekte ederek paylaşır, TCP yalnız X'i izler. İzleme: farklı \
                 birim ve aralıklardaki servis telemetrisinde aynı olay anını birlikte incelemek \
                 için uygundur. Sync kapatmak yerel cursor/kilit durumunu silmez; ikinci grup \
                 kaynak gibi cursor kilidi kullanmaz. Maliyet: beş ana canvas bağımsızdır; \
                 pointer yalnız hafif etkileşim katmanını günceller. Ana yollar setSeries, \
                 seçim, wheel/touch/drag veya boyut değişiminde yenilenir ve görünür X aralığı \
                 yalnız abone yüzeylere taşınır.",
            ),
            KartKimliği::SyncYZero(_) => Some(
                "Amaç: farklı büyüklüklerdeki üç Y ölçeğinin sıfırını ham değer sınırlarını \
                 kaybetmeden aynı fiziksel piksele hizalar. API: sync_y_zero_aralıkları ham, \
                 simetrik ve valToPos/posToVal eşdeğeri final aralıklarını üretir; \
                 Grafik::y_ölçek_aralıklarını_ayarla üç adlandırılmış scale.range sonucunu \
                 atomik uygular. İzleme: pozitif/negatif sapmaları farklı birimlerle tek ortak \
                 X ekseninde karşılaştırmak için uygundur. Kaynak zaman çizelgesi seçimden \
                 3 saniye sonra simetrik, 6 saniye sonra 1/11 ortak sıfır oranına geçer. \
                 Maliyet: her aşama O(3) dönüşüm ve tek sahne boyamasıdır; veri, seçenek ağacı, \
                 Grafik ve GPUI entity yeniden kurulmaz. Cursor, legend, X zoom ve ortak \
                 wheel/touch uzantılarının görünüm durumu korunur.",
            ),
            KartKimliği::ThinBars(_) => Some(
                "Amaç: ince çubuklarda vuruşun ne zaman dolguya düştüğünü ve align, width, \
                 gap, dir, stroke birleşimlerinin geometriyi nasıl değiştirdiğini yan yana \
                 karşılaştırır. API: thin_bars_stroke_fill_kartları kaynak sırasıyla 7 \
                 yoğunluk ve 48 geometri yüzeyini tek grup olarak döndürür; her Grafik kendi \
                 cursor, seçim ve geçmişini bağımsız tutar. İzleme: yoğun histogram veya \
                 sütun telemetrisinde panel genişliğine göre okunabilir vuruş/dolgu seçmek ve \
                 ters X/hizalama kararlarını doğrulamak için uygundur. Maliyet: kaynak gibi \
                 55 yüzey ve toplam 1.422 çubuk kurulur; bar başına element ağı kurulmaz. \
                 Pointer yalnız ilgili GpuiGrafik etkileşim katmanını, zoom yalnız ilgili \
                 sahneyi günceller. Noktalar görünür X piksel açıklığı yeterli olduğunda \
                 otomatik açılır; wheel/touch/drag isteğe bağlı çekirdek uzantısıdır.",
            ),
            KartKimliği::TimePeriods(_) => Some(
                "Amaç: aynı trafik kaynağını saatlik yıllar, iki ay ve günlük toplamlar \
                 biçiminde yan yana karşılaştırır. API: time_periods_kartları üç bağımsız \
                 Grafik döndürür; Hourly seri bazlı geçmiş-yıl lejant tarihleri, Feb–Jan \
                 görünür birincil ölçekten türetilen ikinci X ekseni ve Daily ortak UTC \
                 tarihini kullanır. İzleme: aynı ölçümün dönem ve çözünürlük farklarını \
                 Grafana benzeri panellerde karşılaştırmak için uygundur. Maliyet: traffic.json \
                 bir kez ayrıştırılır; her yüzey kendi cursor, seçim, wheel/touch/drag ve \
                 görünüm geçmişini tutar; etkileşim yalnız ilgili GpuiGrafik sahnesini yeniler.",
            ),
            KartKimliği::TimelineDiscrete(_) => Some(
                "Amaç: gerçek süreli durum geçişlerini, sabit örnek hücrelerini ve yinelenen \
                 değer birleştirmesini aynı kaynak bağlamında karşılaştırır. API: \
                 timeline_discrete_kartları dört bağımsız Grafik döndürür; null/undefined \
                 ayrımı, şerit dağılımı, renk/etiket, sağ kenara uzanan son durum ve 100px \
                 sınırlı matrix vuruşu çekirdektedir. timeline_verisini_ayarla setData ile \
                 hücre dizinini atomik yeniler; setSeries görünürlüğü özel timeline katmanını \
                 değiştirir. İzleme: cihaz duty-cycle ve servis durum geçmişi için uygundur. \
                 Maliyet: hücreler element ağı değil tek sahne boyamasıdır; hover yalnız gerçek \
                 boyalı hücreyi ve hafif vurgu katmanını günceller.",
            ),
            KartKimliği::Scatter => Some(
                "Amaç: sabit boyutlu yoğun scatter ile üçüncü metriği alanla anlatan bubble \
                 yaklaşımını aynı kaynak bağlamında karşılaştırır. API: mode:2 facet serileri \
                 bağımsız X/Y dizileri taşır; bubble size/label facet'leri ve Region A için sağ \
                 y2 ölçeği ekler. İki yüzey veri, cursor ve ölçek bakımından bağımsızdır. İzleme: \
                 korelasyon kümeleri, kapasite/gelir ve nüfus yoğunluğu gibi çok boyutlu \
                 telemetri için uygundur. Maliyet: 40.000 scatter noktası seri başına tek toplu \
                 çizim komutuna iner; bubble hover yalnız ölçek veya boyut değişince yenilenen \
                 uzamsal dizinin aday hücresini sorgular ve ana sahneyi yeniden boyamaz.",
            ),
            KartKimliği::Bars(_) => Some(
                "Amaç: kaynak sayfanın grouped/stacked, dikey/yatay ve tek grup/tek seri sınır \
                 durumlarını on bağımsız yüzeyde birlikte karşılaştırır. API: \
                 bars_grouped_stacked_kartları yüzeyleri kaynak DOM sırasında döndürür; \
                 ÇubukDüzeni yön, yığma ve ters ekseni tanımlar. setSeries grouped serinin \
                 yuvasını, önceden yığılmış serinin kümülatif boşluğunu korur; yeniden yığma \
                 yapmaz. Hover yalnız vurulan barı vurgular ve stacked değerini kümülatif tepe \
                 olarak verir. İzleme: kategorik kapasite, sürüm ya da bölge metriklerini \
                 karşılaştırırken düzen sınırlarını tek sayfada doğrulamak için uygundur. \
                 Maliyet: her yüzey yalnız kendi barlarını tek sahne geçişinde O(grup×seri) \
                 çizer; on Grafik veri ve görünüm geçmişi bakımından bağımsızdır. Kaynak seçim \
                 ve wheel kapalıdır; ortak wheel/touch/drag profili geliştiricinin açabildiği \
                 port uzantısıdır.",
            ),
            KartKimliği::BarsValuesAutosize(_) => Some(
                "Amaç: aynı rastgele değer dizisini dikey ve yatay çubuk yönünde gösterirken \
                 değer yazısının bar ucuyla grafik kenarı arasındaki kullanılabilir alana \
                 otomatik sığmasını karşılaştırır. API: bars_values_autosize_kartları kaynak \
                 sırasıyla iki bağımsız Grafik döndürür; değer_etiketi_otomatik tek çizim \
                 geçişinde kompakt metinleri, bar dikdörtgenlerini ve boşlukları ölçer. Dikey \
                 yüzey metin genişliği, yüksekliği ve bar genişliğinin %80'inden; yatay yüzey \
                 en dar bar yüksekliğinin %80'inden bütün etiketler için ortak 10–25 px boyut \
                 seçer. 10 px altına düşerse etiketlerin tamamı gizlenir. İzleme: dinamik \
                 pozitif/negatif kapasite veya fark metriklerinde etiket taşmasını engellemek \
                 için uygundur. Maliyet: kompakt metin ölçüleri setData'da O(N), kullanılabilir \
                 alan ve çizim O(N) hesaplanır; yüzey yeniden kurulmaz. Kaynakta yorumlu \
                 setData/setSize akışları aynı önbellek ve yeniden ölçüm yaşam döngüsünü \
                 kanıtlar; ortak wheel/touch/drag port uzantısıdır.",
            ),
            KartKimliği::BoxWhisker(_) => Some(
                "Amaç: 17 benchmarkın framework dağılımlarını kaynak sayfadaki aynı bağlamda \
                 karşılaştırır. API: box_whisker_kartları kaynak sırasıyla 17 bağımsız Grafik \
                 döndürür; results.json yalnız bir kez ayrıştırılıp özetlenir. stats.js \
                 medyan/q1/q3 değerlerini iki ondalığa yuvarladıktan sonra 1,5×IQR ile bıyık ve \
                 ayrık değer sınıflaması yapılır; rangeNum bütün ayrık değerlerin global \
                 sınırını kapsar. Tam framework adları -90° eksende korunur. Hover ana sahneyi \
                 yeniden çizmeden mavi sütun vurgusunu ve sarı Lib/Median/q1/q3/min/max bilgi \
                 kutusunu hafif katmanda taşır. İzleme: gecikme, bellek ve başlangıç \
                 ölçümlerinde merkezi eğilim kadar varyansı ve kararsız koşuları görmek için \
                 uygundur. Maliyet: ilk özetleme toplam ölçüm sayısıyla O(N), her yüzey çizimi \
                 en çok 30 kutu ve ayrık değer sayısıyla O(N)'dir; ortak wheel/touch/drag ürün \
                 uzantısıdır.",
            ),
            KartKimliği::Candlestick => Some(
                "Amaç: Gold için tek hizalı tarih sütunundaki Open/High/Low/Close ve hacmi \
                 kaynak demodaki aynı mum + hacim yüzeyinde gösterir. API: MumDüzeni UTC \
                 zamanlarını OHLC sütunlarından ayrı yan veri olarak taşır. Beş seri bağımsız \
                 çizgiler değil tek mum geometrisinin zorunlu alanlarıdır; kaynak özel çizicisi \
                 setSeries/legend toggle sunmaz. Hover ana sahneyi yeniden çizmeden mavi sütun \
                 vurgusunu ve sarı Date/Open/High/Low/Close/Volume bilgi kutusunu hafif katmanda \
                 taşır. Fiyatlar kaynak fmtUSD biçiminde, tarih UTC YYYY-MM-DD olarak gösterilir. \
                 İzleme: piyasa fiyatı veya OHLC pencere özetlerinde yönü, aralığı ve hacmi aynı \
                 zaman sütununda incelemek için uygundur. Maliyet: 218 kaynak satırı gömülüdür; \
                 ana sahne yalnız görünür mum aralığını O(V) çizer, sütun vuruşu sıralı X üzerinde \
                 O(log N)'dir. Ortak wheel/touch/drag davranışları ürün uzantısıdır.",
            ),
            KartKimliği::CursorBind => Some(
                "Amaç: bir grafik olayının varsayılan işleyicisini koruyup çevresine uygulama \
                 politikası eklemeyi gösterir; normal sürükleme zoom, Ctrl sürükleme açıklama \
                 istemidir. API: İmleçBağSeçenekleri birincil tuş filtresi, Ctrl sırasında \
                 setScale durdurma, gerçek Annotation Text istemi ve sürüklemesiz click \
                 iletimini tek deklaratif sözleşmede tanımlar. Kaynaktaki gibi sarı seçim yalnız \
                 dolgu taşır; metin İptal/Tamam/Enter sonrasında kalıcı çizime eklenmez. İzleme: \
                 Grafana benzeri yüzeylerde seçim zoomunu korurken Ctrl ile olay/incident notu \
                 istemek veya normal tıklamayı üst uygulamaya iletmek için uygundur. Maliyet: \
                 30×3 kaynak seri O(N) çizilir; bind kararı ve click iletimi O(1), Ctrl seçiminde \
                 yalnız hafif seçim katmanı ve modal güncellenir.",
            ),
            KartKimliği::AddDelSeries => Some(
                "Amaç: aynı grafik örneğinde çalışma zamanında seri ekleme/silme, hizalı veri \
                 sütunlarını koruma ve setData ölçek sıfırlamasını gösterir. API: Grafik::seri_ekle \
                 ve seri_sil doğrulanmış işlemlerdir; SeriYaşamDöngüsüOlayı X'i sayan resmî \
                 seriesIdx ile addSeries/delSeries olayını setData olayından önce taşır. İlk \
                 ekleme kaynak turuncusudur; sonraki eklemeler geliştiricinin serileri ayırt \
                 edebilmesi için belirlenimci paletten renk alır. İzleme: çalışan bir panele yeni \
                 sensör, CPU veya metrik eklerken grafik ve etkileşim kimliğini korumak için \
                 uygundur. Maliyet: sütun üretimi O(N), hizalı yapı doğrulaması ve yeniden çizim \
                 O(N×S)'dir; GPUI Entity ve yüzey kimliği değişmez.",
            ),
            _ => None,
        };
        let kullanım_rehberi_açık = self.kullanım_rehberi_açık;
        let ayrıntı = div()
            .flex_1()
            .h_full()
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
                            .child(aktif_kart.başlık()),
                    )
                    .child(div().text_sm().text_color(soluk).child(aktif_kart.kaynak())),
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
                                    "▾ Açıklama · kullanım ve kaynak maliyeti"
                                } else {
                                    "▸ Açıklama · kullanım ve kaynak maliyeti"
                                },
                            )
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Hayalet)
                            .tiklaninca(cx.listener(|bu, _, _, cx| {
                                bu.kullanım_rehberi_açık = !bu.kullanım_rehberi_açık;
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
            .when(
                !matches!(
                    aktif_kart,
                    KartKimliği::TooltipsClosest
                        | KartKimliği::Tooltips
                        | KartKimliği::CursorSnap
                        | KartKimliği::Trendlines
                        | KartKimliği::UpdateCursorSelectResize
                        | KartKimliği::WindDirection
                        | KartKimliği::YScaleDrag
                        | KartKimliği::YShiftedSeries
                        | KartKimliği::DependentScale
                        | KartKimliği::ArcSinhScales
                        | KartKimliği::AxisControl
                        | KartKimliği::AxisAutosize
                        | KartKimliği::AxisIndicators
                        | KartKimliği::Bars(_)
                        | KartKimliği::BarsValuesAutosize(_)
                        | KartKimliği::BoxWhisker(_)
                        | KartKimliği::Candlestick
                ),
                |öğe| öğe.child(div().mb_2().text_xs().text_color(vurgu).child(lejant)),
            )
            .when(
                matches!(
                    aktif_kart,
                    KartKimliği::TooltipsClosest
                        | KartKimliği::Tooltips
                        | KartKimliği::CursorSnap
                        | KartKimliği::Trendlines
                        | KartKimliği::UpdateCursorSelectResize
                        | KartKimliği::WindDirection
                        | KartKimliği::YScaleDrag
                        | KartKimliği::YShiftedSeries
                        | KartKimliği::DependentScale
                        | KartKimliği::ArcSinhScales
                        | KartKimliği::AxisControl
                        | KartKimliği::AxisAutosize
                        | KartKimliği::AxisIndicators
                ),
                |öğe| {
                    öğe.child(
                        div().mb_2().flex().flex_wrap().gap_2().children(
                            tooltip_serileri
                                .into_iter()
                                .map(|(indeks, etiket, görünür)| {
                                    Dugme::yeni(
                                        SharedString::from(format!("tooltip-seri-{indeks}")),
                                        SharedString::from(format!(
                                            "● {etiket}{}",
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
                                        bu.tooltip_serisini_değiştir(indeks, cx);
                                    }))
                                }),
                        ),
                    )
                },
            )
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
            .child(çizim)
            .child(
                div()
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
                                .child(aktif_kart.tanım()),
                        )
                    }),
            );

        let içerik = div()
            .size_full()
            .relative()
            .flex()
            .flex_row()
            .bg(zemin)
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
                    .text_xs()
                    .text_color(soluk)
                    .child("Rust 2024 · MSRV 1.95"),
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
        .mt_2()
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
                .text_color(rgb(0x6b7280))
                .child(alt_kimlik),
        )
        .child(div().mt_2().text_xs().text_color(vurgu).child(durum))
}
