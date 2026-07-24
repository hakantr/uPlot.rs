//! GPUI masaüstü chart kataloğu; dağıtılan bileşeni kullanan örnek uygulama.

use gpui::{
    ClickEvent, Context, Entity, FontWeight, IntoElement, Render, SharedString, Task, Window, div,
    prelude::*, px, rgb,
};
use ortak_bilesenler::{
    Anahtar, AnahtarOlayi, CubukAyarlari, Dugme, DugmeBoyutu, DugmeTuru, PlatformPencere,
};
use std::time::Duration;
use uplot_rs::gpui::{GpuiGrafik, GpuiGrafikOlayı};
use uplot_rs::{
    ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ, ALIGN_DATA_KART_TANIM_ÖRNEĞİ, ANNOTATIONS_KART_TANIM_ÖRNEĞİ,
    ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ, AREA_FILL_KART_TANIM_ÖRNEĞİ, AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    AXIS_CONTROL_KART_TANIM_ÖRNEĞİ, AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ,
    BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ, BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    BOX_WHISKER_BENCHMARKLERİ, BOX_WHISKER_KART_TANIM_ÖRNEĞİ, BoyutSenkronAkışı,
    CANDLESTICK_KART_TANIM_ÖRNEĞİ, CURSOR_BIND_KART_TANIM_ÖRNEĞİ, CURSOR_SNAP_KART_TANIM_ÖRNEĞİ,
    CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ, CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ, CustomScaleÖrneği,
    DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ, DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ,
    DRAW_HOOKS_KART_TANIM_ÖRNEĞİ, EtkileşimSeçenekleri, FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ,
    FocusÖrneği, GRADIENTS_KART_TANIM_ÖRNEĞİ, GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ, GradientÖrneği,
    Grafik, HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ, HighLowBandsÖrneği,
    LATENCY_HEATMAP_KART_TANIM_ÖRNEĞİ, LINE_PATHS_KART_TANIM_ÖRNEĞİ, LOG_SCALES_KART_TANIM_ÖRNEĞİ,
    LOG_SCALES2_KART_TANIM_ÖRNEĞİ, LatencyHeatmapÖrneği, LinePathsÖrneği, LogScales2Örneği,
    LogScalesÖrneği, MASS_SPECTRUM_KART_TANIM_ÖRNEĞİ, MEASURE_DATUMS_KART_TANIM_ÖRNEĞİ,
    MISSING_DATA_KART_TANIM_ÖRNEĞİ, MONTHS_KART_TANIM_ÖRNEĞİ, MULTI_BARS_KART_TANIM_ÖRNEĞİ,
    MultiBarsÖrneği, NICE_SCALE_KART_TANIM_ÖRNEĞİ, NO_DATA_KART_TANIM_ÖRNEĞİ, NoDataÖrneği,
    PATH_GAP_CLIP_KART_TANIM_ÖRNEĞİ, PIXEL_ALIGN_KART_TANIM_ÖRNEĞİ, POINTS_KART_TANIM_ÖRNEĞİ,
    PathGapClipÖrneği, PixelAlignÖrneği, PointsÖrneği, RESIZE_KART_TANIM_ÖRNEĞİ,
    SCALE_PADDING_KART_TANIM_ÖRNEĞİ, SCALES_DIR_ORI_KART_TANIM_ÖRNEĞİ, SCATTER_KART_TANIM_ÖRNEĞİ,
    SCROLL_SYNC_KART_TANIM_ÖRNEĞİ, SINE_STREAM_KART_TANIM_ÖRNEĞİ, SOFT_MINMAX_KART_TANIM_ÖRNEĞİ,
    SPARKLINES_BARS_KART_TANIM_ÖRNEĞİ, SPARKLINES_KART_TANIM_ÖRNEĞİ, SPARSE_KART_TANIM_ÖRNEĞİ,
    STACKED_SERIES_KART_TANIM_ÖRNEĞİ, STREAM_DATA_ARALIK_MS, STREAM_DATA_KART_TANIM_ÖRNEĞİ,
    SVG_IMAGE_KART_TANIM_ÖRNEĞİ, SYNC_CURSOR_KART_TANIM_ÖRNEĞİ, SYNC_Y_ZERO_KART_TANIM_ÖRNEĞİ,
    ScalesDirOriÖrneği, ScatterÖrneği, SeriSeçenekleri, SineAkışı, SmoothingÖrneği,
    SoftMinMaxAkışı, SoftMinMaxÖrneği, SparklinesBarsÖrneği, SparklineÖrneği, SparseÖrneği,
    StackedSeriesÖrneği, StreamDataAkışı, StreamDataÖrneği, SyncCursorGrubu, SyncCursorÖrneği,
    SyncYZeroAşaması, THIN_BARS_STROKE_FILL_KART_TANIM_ÖRNEĞİ, TIME_PERIODS_KART_TANIM_ÖRNEĞİ,
    TIMELINE_DISCRETE_KART_TANIM_ÖRNEĞİ, TIMESERIES_DISCRETE_KART_TANIM_ÖRNEĞİ,
    TIMEZONES_DST_KART_TANIM_ÖRNEĞİ, TOOLTIPS_CLOSEST_KART_TANIM_ÖRNEĞİ,
    TOOLTIPS_KART_TANIM_ÖRNEĞİ, TRENDLINES_KART_TANIM_ÖRNEĞİ, ThinBarsÖrneği, TimePeriodsÖrneği,
    TimelineDiscreteÖrneği, TimeseriesDiscreteÖrneği, TimezonesDstÖrneği,
    UPDATE_CURSOR_SELECT_RESIZE_ARALIK_MS, UPDATE_CURSOR_SELECT_RESIZE_KART_TANIM_ÖRNEĞİ,
    UplotHatası, WIND_DIRECTION_KART_TANIM_ÖRNEĞİ, Y_SCALE_DRAG_KART_TANIM_ÖRNEĞİ,
    Y_SHIFTED_SERIES_ARALIK_MS, Y_SHIFTED_SERIES_KART_TANIM_ÖRNEĞİ, YShiftedSeriesAkışı,
    ZOOM_TOUCH_KART_TANIM_ÖRNEĞİ, ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ, add_del_series_ek_verisi,
    add_del_series_kartı, align_data_maliyet_kartı, align_data_çizgi_çubuk_kartı,
    annotations_kartı, arcsinh_scales_kartı, area_fill_kartı, axis_autosize_kartı,
    axis_control_kartı, axis_indicators_kartı, bars_grouped_stacked_kartı,
    bars_values_autosize_kartı, box_whisker_kartı, candlestick_ohlc_kartı, cursor_bind_kartı,
    cursor_snap_kartı, cursor_tooltip_kartı, custom_scales_kartı, data_smoothing_kartı,
    dependent_scale_kartı, draw_hooks_kartı, focus_cursor_kartı, gradients_kartı,
    grid_over_series_kartı, high_low_bands_kartı, latency_heatmap_kartı, line_paths_kartı,
    log_scales_kartı, log_scales2_kartı, mass_spectrum_kartı, measure_datums_kartı,
    missing_data_null_kartı, missing_data_x_boşluğu_kartı, months_artık_yıllı_kartı,
    months_artık_yılsız_kartı, months_rusça_kartı, multi_bars_kartı, nice_scale_kartı,
    no_data_kartı, ortak_kart_etkileşimleri, path_gap_clip_kartı, pixel_align_kartı, points_kartı,
    resize_kartı, scale_padding_kartı, scales_dir_ori_kartı, scatter_kartı, scroll_sync_kartı,
    sine_stream_kartı, soft_minmax_kartı, sparklines_bars_kartı, sparklines_kartı, sparse_kartı,
    stacked_series_kartı, stacked_series_kartı_görünür, stream_data_kartı, svg_image_kartı,
    sync_cursor_kartı, sync_y_zero_kartı, thin_bars_stroke_fill_kartı, time_periods_kartı,
    timeline_discrete_kartı, timeseries_discrete_kartı, timezones_dst_kartı,
    tooltips_closest_kartı, tooltips_kartı, trendlines_kartı, update_cursor_select_resize_kartı,
    wind_direction_kartı, y_scale_drag_kartı, y_shifted_series_kartı, zoom_touch_kartı,
    zoom_wheel_kartı, ÇubukYönü, ÇubukÖrneği,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum KartKimliği {
    AddDelSeries,
    AlignDataCost,
    AlignDataLineBars,
    Resize,
    Annotations,
    AreaFill,
    ScalePadding,
    ZoomWheel,
    ZoomTouch,
    MonthsNoLeap,
    MonthsLeap,
    MonthsRussian,
    NiceScale,
    NoData(NoDataÖrneği),
    PathGapClip(PathGapClipÖrneği),
    PixelAlign(PixelAlignÖrneği),
    Points(PointsÖrneği),
    ScalesDirOri(ScalesDirOriÖrneği),
    Scatter(ScatterÖrneği),
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
    TimezonesDst(TimezonesDstÖrneği),
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
    CustomScales(CustomScaleÖrneği),
    DataSmoothing(SmoothingÖrneği),
    DrawHooks,
    FocusCursor(FocusÖrneği),
    Gradients(GradientÖrneği),
    GridOverSeries,
    HighLowBands(HighLowBandsÖrneği),
    LatencyHeatmap(LatencyHeatmapÖrneği),
    LinePaths(LinePathsÖrneği),
    LogScales(LogScalesÖrneği),
    LogScales2(LogScales2Örneği),
    MassSpectrum,
    MeasureDatums,
    MultiBars(MultiBarsÖrneği),
    MissingDataNull,
    MissingDataXGap,
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
            Self::AlignDataCost => "Align Data · join cost",
            Self::AlignDataLineBars => "Align Data · line + bars",
            Self::Resize => "Resize · sayısal x ölçeği",
            Self::Annotations => "Annotations",
            Self::AreaFill => "Area Fill",
            Self::ScalePadding => "Scale Padding · Flat",
            Self::ZoomWheel => "Wheel Zoom & Drag",
            Self::ZoomTouch => "Pinch Zoom & Pan",
            Self::MonthsNoLeap => "Months · No leap year",
            Self::MonthsLeap => "Months · 2024 leap year",
            Self::MonthsRussian => "Months · Russian",
            Self::NiceScale => "Nice Scale & Ticks",
            Self::NoData(örnek) => örnek.başlık(),
            Self::PathGapClip(örnek) => örnek.başlık(),
            Self::PixelAlign(örnek) => örnek.başlık(),
            Self::Points(örnek) => örnek.başlık(),
            Self::ScalesDirOri(örnek) => örnek.başlık(),
            Self::Scatter(örnek) => örnek.başlık(),
            Self::ScrollSync => "Scroll syncRect()",
            Self::SineStream => "6 series x 600 points @ 60fps",
            Self::SoftMinMax(örnek) => örnek.başlık(),
            Self::SparklinesBars(örnek) => örnek.başlık(),
            Self::Sparklines(örnek) => örnek.başlık(),
            Self::Sparse(örnek) => örnek.başlık(),
            Self::StackedSeries(örnek) => örnek.başlık(),
            Self::StreamData(örnek) => örnek.başlık(),
            Self::SvgImage => "uPlot to image PoC",
            Self::SyncCursor => "Sync Cursor",
            Self::SyncYZero(_) => "Sync Y Zero",
            Self::ThinBars(_) => "Thin bar stroke & fill",
            Self::TimePeriods(_) => "Time Periods",
            Self::TimelineDiscrete(_) => "Timeline / Discrete",
            Self::TimeseriesDiscrete => "TimeSeries + Discrete",
            Self::TimezonesDst(_) => "Timezones & DST",
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
            Self::CustomScales(örnek) => örnek.başlık(),
            Self::DataSmoothing(örnek) => örnek.başlık(),
            Self::DrawHooks => "Draw Hooks",
            Self::FocusCursor(örnek) => örnek.başlık(),
            Self::Gradients(örnek) => örnek.başlık(),
            Self::GridOverSeries => "Grid Over Series",
            Self::HighLowBands(örnek) => örnek.başlık(),
            Self::LatencyHeatmap(örnek) => örnek.başlık(),
            Self::LinePaths(örnek) => örnek.başlık(),
            Self::LogScales(örnek) => örnek.başlık(),
            Self::LogScales2(örnek) => örnek.başlık(),
            Self::MassSpectrum => "Mass Spectrum",
            Self::MeasureDatums => "Measure / Datums",
            Self::MultiBars(örnek) => örnek.başlık(),
            Self::MissingDataNull => "Missing Data · null values",
            Self::MissingDataXGap => "Missing Data · adjacent X gap",
            Self::DependentScale => "Derived Scale · °F / °C",
            Self::ArcSinhScales => "ArcSinh Y Scale",
            Self::AxisControl => "Axis Control",
            Self::AxisAutosize => "Axis AutoSize",
            Self::AxisIndicators => "Axis indicators",
            Self::Bars(örnek) => örnek.kimlik(),
            Self::BarsValuesAutosize(ÇubukYönü::Dikey) => "bars-values-autosize-vertical",
            Self::BarsValuesAutosize(ÇubukYönü::Yatay) => "bars-values-autosize-horizontal",
            Self::BoxWhisker(benchmark) => benchmark,
            Self::Candlestick => "Candlestick Chart · Gold",
        }
    }

    fn kaynak(self) -> &'static str {
        match self {
            Self::AddDelSeries => {
                "add-del-series.html · addSeries/delSeries/setData · kaynak Y indeksi 1"
            }
            Self::AlignDataCost => "align-data.html · 5×5×1000 tablo · NULL_EXPAND join",
            Self::AlignDataLineBars => "align-data.html · farklı X dizilerinde çizgi + çubuk",
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
            Self::ZoomWheel => "zoom-wheel.html · resmî 0.75 katsayılı tekerlek eklentisi",
            Self::ZoomTouch => "zoom-touch.html · resmî kıstırma ve tek parmak taşıma eklentisi",
            Self::MonthsNoLeap | Self::MonthsLeap => {
                "months.html · UTC ay ekseni · resmî sayfadaki iki alt grafik"
            }
            Self::MonthsRussian => {
                "months-ru.html · UTC ay ekseni · resmî Rusça fmtDate tarih adları"
            }
            Self::NiceScale => {
                "nice-scale.html · boyuta bağlı niceScale/niceNum Y aralığı ve artımı"
            }
            Self::NoData(_) => "no-data.html · 33 boş, tek noktalı, düz ve hassas ölçek yüzeyi",
            Self::PathGapClip(_) => {
                "path-gap-clip.html · 15 null/undefined, band, stepped ve piksel yüzeyi"
            }
            Self::PixelAlign(_) => {
                "pixel-align.html · aynı canlı veriyle tam piksel ve alt piksel karşılaştırması"
            }
            Self::Points(_) => {
                "points.html · randomWalk.js · points.space, paths:null ve points.filter"
            }
            Self::ScalesDirOri(_) => {
                "scales-dir-ori.html · src/uPlot.js · scale.dir, scale.ori ve axis.side"
            }
            Self::Scatter(_) => "scatter.html · quadtree.js · mode:2 facet ve bubble vuruşu",
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
            Self::TimezonesDst(_) => {
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
            Self::CustomScales(_) => {
                "custom-scales.html · doğrusal, log-log ve özel Weibull ölçeği"
            }
            Self::DataSmoothing(_) => {
                "data-smoothing.html · taxi-trips + SGG + ASAP FFT + Moving Avg 300"
            }
            Self::DrawHooks => "draw-hooks.html · drawClear/drawSeries/draw plugin hooks",
            Self::FocusCursor(_) => "focus-cursor.html · cursor.focus + setSeries",
            Self::Gradients(_) => "gradients.html · scaleGradient + cursor point colors",
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
            Self::MissingDataNull | Self::MissingDataXGap => {
                "missing-data.html · resmî veri ve iki kaynak alt grafiği"
            }
            Self::DependentScale => {
                "dependent-scale.html · Fahrenheit'tan türetilen Celsius ekseni"
            }
            Self::ArcSinhScales => "arcsinh-scales.html · değiştirilebilir doğrusal merkez eşiği",
            Self::AxisControl => "axis-control.html · 500.001 nokta ve sağ Y ekseni",
            Self::AxisAutosize => "axis-autosize.html · 501 nokta ve 1…10⁹ dinamik eksen ölçümü",
            Self::AxisIndicators => "axis-indicators.html · üç renkli eksen ve imleç göstergeleri",
            Self::Bars(_) => "bars-grouped-stacked.html · kaynaktaki kategorik çubuk düzeni",
            Self::BarsValuesAutosize(_) => {
                "bars-values-autosize.html · otomatik kompakt değer yazısı"
            }
            Self::BoxWhisker(_) => "box-whisker.html · results.json ve stats.js",
            Self::Candlestick => "candlestick-ohlc.html · Gold OHLC ve hacim",
        }
    }

    fn tanım(self) -> &'static str {
        match self {
            Self::AddDelSeries => ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ,
            Self::AlignDataCost | Self::AlignDataLineBars => ALIGN_DATA_KART_TANIM_ÖRNEĞİ,
            Self::Resize => RESIZE_KART_TANIM_ÖRNEĞİ,
            Self::Annotations => ANNOTATIONS_KART_TANIM_ÖRNEĞİ,
            Self::AreaFill => AREA_FILL_KART_TANIM_ÖRNEĞİ,
            Self::ScalePadding => SCALE_PADDING_KART_TANIM_ÖRNEĞİ,
            Self::ZoomWheel => ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ,
            Self::ZoomTouch => ZOOM_TOUCH_KART_TANIM_ÖRNEĞİ,
            Self::MonthsNoLeap | Self::MonthsLeap | Self::MonthsRussian => {
                MONTHS_KART_TANIM_ÖRNEĞİ
            }
            Self::NiceScale => NICE_SCALE_KART_TANIM_ÖRNEĞİ,
            Self::NoData(_) => NO_DATA_KART_TANIM_ÖRNEĞİ,
            Self::PathGapClip(_) => PATH_GAP_CLIP_KART_TANIM_ÖRNEĞİ,
            Self::PixelAlign(_) => PIXEL_ALIGN_KART_TANIM_ÖRNEĞİ,
            Self::Points(_) => POINTS_KART_TANIM_ÖRNEĞİ,
            Self::ScalesDirOri(_) => SCALES_DIR_ORI_KART_TANIM_ÖRNEĞİ,
            Self::Scatter(_) => SCATTER_KART_TANIM_ÖRNEĞİ,
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
            Self::TimezonesDst(_) => TIMEZONES_DST_KART_TANIM_ÖRNEĞİ,
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
            Self::CustomScales(_) => CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ,
            Self::DataSmoothing(_) => DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ,
            Self::DrawHooks => DRAW_HOOKS_KART_TANIM_ÖRNEĞİ,
            Self::FocusCursor(_) => FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ,
            Self::Gradients(_) => GRADIENTS_KART_TANIM_ÖRNEĞİ,
            Self::GridOverSeries => GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ,
            Self::HighLowBands(_) => HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ,
            Self::LatencyHeatmap(_) => LATENCY_HEATMAP_KART_TANIM_ÖRNEĞİ,
            Self::LinePaths(_) => LINE_PATHS_KART_TANIM_ÖRNEĞİ,
            Self::LogScales(_) => LOG_SCALES_KART_TANIM_ÖRNEĞİ,
            Self::LogScales2(_) => LOG_SCALES2_KART_TANIM_ÖRNEĞİ,
            Self::MassSpectrum => MASS_SPECTRUM_KART_TANIM_ÖRNEĞİ,
            Self::MeasureDatums => MEASURE_DATUMS_KART_TANIM_ÖRNEĞİ,
            Self::MultiBars(_) => MULTI_BARS_KART_TANIM_ÖRNEĞİ,
            Self::MissingDataNull | Self::MissingDataXGap => MISSING_DATA_KART_TANIM_ÖRNEĞİ,
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
            Self::AlignDataCost | Self::AlignDataLineBars => "src/kart/align_data.rs",
            Self::Resize => "src/kart/resize.rs",
            Self::Annotations => "src/kart/annotations.rs",
            Self::AreaFill => "src/kart/area_fill.rs",
            Self::ScalePadding => "src/kart/scale_padding.rs",
            Self::ZoomWheel => "src/kart/zoom_wheel.rs",
            Self::ZoomTouch => "src/kart/zoom_touch.rs",
            Self::MonthsNoLeap | Self::MonthsLeap | Self::MonthsRussian => "src/kart/months.rs",
            Self::NiceScale => "src/kart/nice_scale.rs",
            Self::NoData(_) => "src/kart/no_data.rs",
            Self::PathGapClip(_) => "src/kart/path_gap_clip.rs",
            Self::PixelAlign(_) => "src/kart/pixel_align.rs",
            Self::Points(_) => "src/kart/points.rs",
            Self::ScalesDirOri(_) => "src/kart/scales_dir_ori.rs",
            Self::Scatter(_) => "src/kart/scatter.rs",
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
            Self::TimezonesDst(_) => "src/kart/timezones_dst.rs",
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
            Self::CustomScales(_) => "src/kart/custom_scales.rs",
            Self::DataSmoothing(_) => "src/kart/data_smoothing.rs",
            Self::DrawHooks => "src/kart/draw_hooks.rs",
            Self::FocusCursor(_) => "src/kart/focus_cursor.rs",
            Self::Gradients(_) => "src/kart/gradients.rs",
            Self::GridOverSeries => "src/kart/grid_over_series.rs",
            Self::HighLowBands(_) => "src/kart/high_low_bands.rs",
            Self::LatencyHeatmap(_) => "src/kart/latency_heatmap.rs",
            Self::LinePaths(_) => "src/kart/line_paths.rs",
            Self::LogScales(_) => "src/kart/log_scales.rs",
            Self::LogScales2(_) => "src/kart/log_scales2.rs",
            Self::MassSpectrum => "src/kart/mass_spectrum.rs",
            Self::MeasureDatums => "src/kart/measure_datums.rs",
            Self::MultiBars(_) => "src/kart/multi_bars.rs",
            Self::MissingDataNull | Self::MissingDataXGap => "src/kart/missing_data.rs",
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
        if matches!(self, Self::Bars(_)) {
            EtkileşimSeçenekleri::default()
                .seçim_yakınlaştır(false)
                .çift_tıkla_tam_görünüm(false)
        } else if self == Self::CursorBind {
            ortak_kart_etkileşimleri().ctrl_açıklama(true)
        } else if matches!(self, Self::StreamData(_)) {
            ortak_kart_etkileşimleri().seçim_yakınlaştır(false)
        } else if self == Self::YScaleDrag {
            ortak_kart_etkileşimleri().eksen_sürükleme(true)
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
    tekerlek_etkin: bool,
    tekerlek_anahtarı: Entity<Anahtar>,
    arcsinh_kuvvet: i32,
    autosize_kuvvet: i32,
    latency_kova: u8,
    latency_ofset: u8,
    açıklama_istendi: bool,
    dinamik_seri_sayacı: u32,
    align_data_zamanlayıcısı: Option<Task<()>>,
    pixel_align_adımı: usize,
    sine_akışı: Option<SineAkışı>,
    stream_data_akışı: Option<StreamDataAkışı>,
    soft_minmax_akışı: Option<SoftMinMaxAkışı>,
    boyut_senkron_akışı: Option<BoyutSenkronAkışı>,
    y_shifted_series_akışı: Option<YShiftedSeriesAkışı>,
    soft_minmax_çalışıyor: bool,
    sync_cursor_grafikleri: Vec<(SyncCursorÖrneği, Entity<GpuiGrafik>)>,
    sync_cursor_grubu: SyncCursorGrubu,
    timeseries_discrete_grafikleri: Vec<(TimeseriesDiscreteÖrneği, Entity<GpuiGrafik>)>,
}

impl ChartListesi {
    pub fn yeni(cx: &mut Context<Self>) -> Self {
        let etkileşimler = ortak_kart_etkileşimleri();
        let tekerlek_anahtarı = cx.new(|cx| {
            Anahtar::yeni(
                "Tekerlek eklentisi · Otomatik",
                etkileşimler.tekerlek_etkileşimi,
                cx,
            )
        });
        cx.subscribe(&tekerlek_anahtarı, |bu, _, olay: &AnahtarOlayi, cx| {
            let AnahtarOlayi::Degisti(etkin) = *olay;
            if bu.aktif_kart == KartKimliği::SyncCursor {
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
            } else if let Some(grafik) = &bu.grafik {
                grafik.update(cx, |grafik, cx| {
                    grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                });
            }
            bu.tekerlek_etkin = etkin;
            cx.notify();
        })
        .detach();

        let (grafik, hata) = grafik_oluştur(KartKimliği::Resize, 100, 0, 5, 0, 140).map_or_else(
            |hata| (None, Some(format!("Grafik oluşturulamadı: {hata}"))),
            |grafik| (Some(cx.new(|_| GpuiGrafik::yeni(grafik))), None),
        );
        if let Some(grafik) = &grafik {
            cx.subscribe(grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                    bu.açıklama_istendi = true;
                }
                cx.notify();
            })
            .detach();
        }
        Self {
            aktif_kart: KartKimliği::Resize,
            nokta_sayısı: 100,
            grafik,
            hata,
            kart_tanımı_açık: false,
            tekerlek_etkin: etkileşimler.tekerlek_etkileşimi,
            tekerlek_anahtarı,
            arcsinh_kuvvet: 0,
            autosize_kuvvet: 0,
            latency_kova: 5,
            latency_ofset: 0,
            açıklama_istendi: false,
            dinamik_seri_sayacı: 0,
            align_data_zamanlayıcısı: None,
            pixel_align_adımı: 140,
            sine_akışı: None,
            stream_data_akışı: None,
            soft_minmax_akışı: None,
            boyut_senkron_akışı: None,
            y_shifted_series_akışı: None,
            soft_minmax_çalışıyor: false,
            sync_cursor_grafikleri: Vec::new(),
            sync_cursor_grubu: SyncCursorGrubu::yeni(),
            timeseries_discrete_grafikleri: Vec::new(),
        }
    }

    fn timeseries_discrete_yüzeylerini_oluştur(&mut self, cx: &mut Context<Self>) {
        let mut yüzeyler = Vec::with_capacity(TimeseriesDiscreteÖrneği::TÜMÜ.len());
        let mut hata = None;
        for örnek in TimeseriesDiscreteÖrneği::TÜMÜ {
            let sonuç = timeseries_discrete_kartı(örnek)
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

    fn grafiği_yenile(&mut self, nokta_sayısı: usize, cx: &mut Context<Self>) {
        self.nokta_sayısı = nokta_sayısı;
        match grafik_oluştur(
            self.aktif_kart,
            nokta_sayısı,
            self.autosize_kuvvet,
            self.latency_kova,
            self.latency_ofset,
            self.pixel_align_adımı,
        ) {
            Ok(mut yeni) => {
                yeni.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
                if let Some(grafik) = &self.grafik {
                    grafik.update(cx, |grafik, cx| grafik.grafiği_ayarla(yeni, cx));
                } else {
                    let grafik = cx.new(|_| GpuiGrafik::yeni(yeni));
                    cx.subscribe(&grafik, |bu, _, olay: &GpuiGrafikOlayı, cx| {
                        if matches!(olay, GpuiGrafikOlayı::Açıklamaİstendi) {
                            bu.açıklama_istendi = true;
                        }
                        cx.notify();
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
                match olay {
                    GpuiGrafikOlayı::Açıklamaİstendi => {
                        bu.açıklama_istendi = true;
                    }
                    GpuiGrafikOlayı::İmleçDeğişti => {
                        let yayın = bu
                            .sync_cursor_grafikleri
                            .iter()
                            .find(|(kimlik, _)| *kimlik == örnek)
                            .and_then(|(_, grafik)| grafik.read(cx).senkron_yayını());
                        let hedefler = bu.sync_cursor_grubu.imleç_hedefleri(örnek);
                        let yüzeyler = bu.sync_cursor_grafikleri.clone();
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
                                    grafik.senkron_imleci_ayarla(x, dikey, hedef_serisi, cx);
                                });
                            } else {
                                hedef_grafik.update(cx, |grafik, cx| {
                                    grafik.senkron_imleci_temizle(cx);
                                });
                            }
                        }
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
            self.hata = None;
        }
        cx.notify();
    }

    fn sync_cursor_senkronunu_değiştir(&mut self, cx: &mut Context<Self>) {
        let etkin = !self.sync_cursor_grubu.senkron();
        self.sync_cursor_grubu.senkronu_ayarla(etkin);
        if !etkin {
            for (örnek, grafik) in &self.sync_cursor_grafikleri {
                if matches!(
                    örnek,
                    SyncCursorÖrneği::Cpu | SyncCursorÖrneği::Ram | SyncCursorÖrneği::Tcp
                ) {
                    grafik.update(cx, |grafik, cx| {
                        grafik.senkron_imleci_temizle(cx);
                        grafik.senkron_kilidi_ayarla(false, cx);
                    });
                }
            }
        }
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
        self.arcsinh_kuvvet = 0;
        self.autosize_kuvvet = 0;
        self.latency_kova = 5;
        self.latency_ofset = 0;
        self.açıklama_istendi = false;
        self.dinamik_seri_sayacı = 0;
        self.align_data_zamanlayıcısı = None;
        self.pixel_align_adımı = 140;
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
        self.stream_data_akışı = if let KartKimliği::StreamData(örnek) = kart {
            match StreamDataAkışı::yeni(örnek) {
                Ok(akış) => Some(akış),
                Err(hata) => {
                    self.hata = Some(format!("Data Stream başlatılamadı: {hata}"));
                    None
                }
            }
        } else {
            None
        };
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
        if kart == KartKimliği::SyncCursor {
            self.sync_cursor_grubu = SyncCursorGrubu::yeni();
            self.timeseries_discrete_grafikleri.clear();
            self.sync_cursor_yüzeylerini_oluştur(cx);
        } else if kart == KartKimliği::TimeseriesDiscrete {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_yüzeylerini_oluştur(cx);
        } else {
            self.sync_cursor_grafikleri.clear();
            self.timeseries_discrete_grafikleri.clear();
            self.grafiği_yenile(self.nokta_sayısı, cx);
        }
        if kart == KartKimliği::AlignDataCost
            || matches!(
                kart,
                KartKimliği::PathGapClip(
                    PathGapClipÖrneği::VeriDışınaTaşanÖlçek
                        | PathGapClipÖrneği::BantBoşlukları
                        | PathGapClipÖrneği::GenişletilmişHizalama
                        | PathGapClipÖrneği::SayısalHizalama
                )
            )
        {
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
                            if let Some(grafik) = &bu.grafik {
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
        } else if matches!(kart, KartKimliği::PixelAlign(_)) {
            self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
                loop {
                    cx.background_executor().timer(Duration::from_secs(1)).await;
                    let devam = bu
                        .update(cx, |bu, cx| {
                            if bu.aktif_kart != kart {
                                return false;
                            }
                            bu.pixel_align_adımı =
                                bu.pixel_align_adımı.saturating_add(1).min(10_000);
                            bu.grafiği_yenile(bu.nokta_sayısı, cx);
                            true
                        })
                        .unwrap_or(false);
                    if !devam {
                        break;
                    }
                }
            }));
        } else if kart == KartKimliği::SineStream {
            self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
                loop {
                    cx.background_executor()
                        .timer(Duration::from_millis(16))
                        .await;
                    let devam = bu
                        .update(cx, |bu, cx| {
                            if bu.aktif_kart != KartKimliği::SineStream {
                                return false;
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
                                    if let Some(grafik) = &bu.grafik {
                                        let güncellendi = grafik.update(cx, |grafik, cx| {
                                            grafik.veriyi_ayarla(veri, cx)
                                        });
                                        if let Err(hata) = güncellendi {
                                            bu.hata =
                                                Some(format!("Sine Stream güncellenemedi: {hata}"));
                                            return false;
                                        }
                                    }
                                    true
                                }
                                Err(hata) => {
                                    bu.hata =
                                        Some(format!("Sine Stream verisi üretilemedi: {hata}"));
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
                            let sonuç = bu.stream_data_akışı.as_mut().map_or_else(
                                || {
                                    Err(UplotHatası::GeçersizKaynakVeri {
                                        varlık: "StreamDataAkışı",
                                        açıklama: "masaüstü akış durumu bulunamadı".to_string(),
                                    })
                                },
                                |akış| {
                                    if !akış.ilerlet() {
                                        return Ok(None);
                                    }
                                    akış.kartı().map(|(_, veri)| Some(veri))
                                },
                            );
                            match sonuç {
                                Ok(Some(veri)) => {
                                    if let Some(grafik) = &bu.grafik {
                                        let güncellendi = grafik.update(cx, |grafik, cx| {
                                            grafik.veriyi_ayarla(veri, cx)
                                        });
                                        if let Err(hata) = güncellendi {
                                            bu.hata =
                                                Some(format!("Data Stream güncellenemedi: {hata}"));
                                            return false;
                                        }
                                    }
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
                                YShiftedSeriesAkışı::ilerlet,
                            );
                            let (seçenekler, veri) = match sonuç {
                                Ok(kart) => kart,
                                Err(hata) => {
                                    bu.hata = Some(format!("Y-shifted Series üretilemedi: {hata}"));
                                    return false;
                                }
                            };
                            let yeni_grafik = match Grafik::yeni(seçenekler, veri) {
                                Ok(grafik) => grafik,
                                Err(hata) => {
                                    bu.hata = Some(format!("Y-shifted Series kurulamadı: {hata}"));
                                    return false;
                                }
                            };
                            let Some(grafik) = &bu.grafik else {
                                return false;
                            };
                            grafik.update(cx, |grafik, cx| {
                                grafik.grafiği_ayarla(yeni_grafik, cx);
                            });
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
                            bu.aktif_kart = KartKimliği::SyncYZero(aşama);
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
    }

    fn soft_minmax_başlat(&mut self, cx: &mut Context<Self>) {
        let KartKimliği::SoftMinMax(örnek) = self.aktif_kart else {
            return;
        };
        if !örnek.canlı_mı() || self.soft_minmax_çalışıyor {
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
                            |akış| akış.ilerlet(örnek),
                        );
                        match sonuç {
                            Ok(veri) => {
                                if let Some(grafik) = &bu.grafik {
                                    let güncellendi = grafik
                                        .update(cx, |grafik, cx| grafik.veriyi_ayarla(veri, cx));
                                    if let Err(hata) = güncellendi {
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
        self.autosize_kuvvet = kuvvet.clamp(0, 9);
        self.grafiği_yenile(self.nokta_sayısı, cx);
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
        let sonuç = grafik.update(cx, |grafik, cx| {
            grafik.seri_ekle(
                1,
                SeriSeçenekleri::yeni("Orange")
                    .renk("#ffa500")
                    .dolgu("#ffa5001a"),
                değerler,
                cx,
            )
        });
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

    fn stacked_seriyi_değiştir(&mut self, seri_indeksi: usize, cx: &mut Context<Self>) {
        let KartKimliği::StackedSeries(örnek) = self.aktif_kart else {
            return;
        };
        let görünürlük = self.grafik.as_ref().map_or_else(Vec::new, |grafik| {
            grafik
                .read(cx)
                .grafik()
                .seri_seçenekleri()
                .iter()
                .map(|seri| seri.göster)
                .collect::<Vec<_>>()
        });
        if seri_indeksi >= görünürlük.len() {
            return;
        }
        let mut yeni_görünürlük = görünürlük;
        if let Some(hedef) = yeni_görünürlük.get_mut(seri_indeksi) {
            *hedef = !*hedef;
        }
        let sonuç = stacked_series_kartı_görünür(örnek, &yeni_görünürlük)
            .and_then(|(seçenekler, veri)| Grafik::yeni(seçenekler, veri));
        match sonuç {
            Ok(mut yeni) => {
                yeni.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
                if let Some(grafik) = &self.grafik {
                    grafik.update(cx, |grafik, cx| grafik.grafiği_ayarla(yeni, cx));
                }
                self.hata = None;
            }
            Err(hata) => self.hata = Some(format!("Seri görünürlüğü değiştirilemedi: {hata}")),
        }
        cx.notify();
    }
}

fn grafik_oluştur(
    kart: KartKimliği,
    nokta_sayısı: usize,
    autosize_kuvvet: i32,
    latency_kova: u8,
    latency_ofset: u8,
    pixel_align_adımı: usize,
) -> Result<Grafik, UplotHatası> {
    let (seçenekler, veri) = match kart {
        KartKimliği::AddDelSeries => add_del_series_kartı(),
        KartKimliği::AlignDataCost => align_data_maliyet_kartı(),
        KartKimliği::AlignDataLineBars => align_data_çizgi_çubuk_kartı(),
        KartKimliği::Resize => resize_kartı(nokta_sayısı),
        KartKimliği::Annotations => annotations_kartı(),
        KartKimliği::AreaFill => area_fill_kartı(),
        KartKimliği::ScalePadding => scale_padding_kartı(),
        KartKimliği::ZoomWheel => zoom_wheel_kartı(),
        KartKimliği::ZoomTouch => zoom_touch_kartı(),
        KartKimliği::MonthsNoLeap => months_artık_yılsız_kartı(),
        KartKimliği::MonthsLeap => months_artık_yıllı_kartı(),
        KartKimliği::MonthsRussian => months_rusça_kartı(),
        KartKimliği::NiceScale => nice_scale_kartı(),
        KartKimliği::NoData(örnek) => no_data_kartı(örnek),
        KartKimliği::PathGapClip(örnek) => path_gap_clip_kartı(örnek),
        KartKimliği::PixelAlign(örnek) => pixel_align_kartı(örnek, pixel_align_adımı),
        KartKimliği::Points(örnek) => points_kartı(örnek),
        KartKimliği::ScalesDirOri(örnek) => scales_dir_ori_kartı(örnek),
        KartKimliği::Scatter(örnek) => scatter_kartı(örnek),
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
        KartKimliği::TimezonesDst(örnek) => timezones_dst_kartı(örnek),
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
        KartKimliği::CustomScales(örnek) => custom_scales_kartı(örnek),
        KartKimliği::DataSmoothing(örnek) => data_smoothing_kartı(örnek),
        KartKimliği::DrawHooks => draw_hooks_kartı(),
        KartKimliği::FocusCursor(örnek) => focus_cursor_kartı(örnek),
        KartKimliği::Gradients(örnek) => gradients_kartı(örnek),
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
        KartKimliği::MissingDataNull => missing_data_null_kartı(),
        KartKimliği::MissingDataXGap => missing_data_x_boşluğu_kartı(),
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let panel = rgb(0xffffff);
        let zemin = rgb(0xf3f4f6);
        let metin = rgb(0x111827);
        let soluk = rgb(0x6b7280);
        let vurgu = rgb(0xdc2626);
        let aktif_kart = self.aktif_kart;
        let soft_minmax_canlı = matches!(
            aktif_kart,
            KartKimliği::SoftMinMax(örnek) if örnek.canlı_mı()
        );
        let soft_minmax_çalışıyor = self.soft_minmax_çalışıyor;
        let sync_cursor_etkin = self.sync_cursor_grubu.senkron();
        let sync_cursor_fare_etkin = self.sync_cursor_grubu.fare_basma_bırakma_senkron();
        let mevcut_seri_sayısı = self.grafik.as_ref().map_or(0, |grafik| {
            grafik.read(cx).grafik().seri_seçenekleri().len()
        });
        let stacked_seriler = if matches!(aktif_kart, KartKimliği::StackedSeries(_)) {
            self.grafik.as_ref().map_or_else(Vec::new, |grafik| {
                grafik
                    .read(cx)
                    .grafik()
                    .seri_seçenekleri()
                    .iter()
                    .enumerate()
                    .map(|(indeks, seri)| (indeks, seri.etiket.clone(), seri.göster))
                    .collect::<Vec<_>>()
            })
        } else {
            Vec::new()
        };
        let nokta_yazısı = SharedString::from(match aktif_kart {
            KartKimliği::AddDelSeries => {
                format!("30 nokta × {mevcut_seri_sayısı} dinamik seri")
            }
            KartKimliği::AlignDataCost => {
                "5 tablo × 5 seri × 1000 X · birleşik sıralı X".to_string()
            }
            KartKimliği::AlignDataLineBars => "38 noktalı çizgi + 4 çubuk".to_string(),
            KartKimliği::Resize => format!("{} nokta", self.nokta_sayısı),
            KartKimliği::Annotations => "30 nokta × 2 seri · 2 X annotation".to_string(),
            KartKimliği::AreaFill => "30 sabit nokta × 3 seri".to_string(),
            KartKimliği::ScalePadding => "10 nokta × 13 düz seri".to_string(),
            KartKimliği::ZoomWheel => "7 nokta × 2 seri".to_string(),
            KartKimliği::ZoomTouch => "7 nokta × 2 seri".to_string(),
            KartKimliği::MonthsNoLeap | KartKimliği::MonthsLeap => {
                "36 aylık nokta × 1 seri".to_string()
            }
            KartKimliği::MonthsRussian => {
                "36 aylık nokta × 1 seri · Rusça tarih adları".to_string()
            }
            KartKimliği::NiceScale => {
                "6 nokta × 3 seri · boyuta duyarlı güzel Y ölçeği".to_string()
            }
            KartKimliği::NoData(örnek) => {
                let nokta = örnek.nokta_sayısı();
                format!(
                    "{nokta} nokta × 1 seri · {}",
                    if nokta == 0 {
                        "boş ölçek durumu"
                    } else {
                        "rangeNum kenar durumu"
                    }
                )
            }
            KartKimliği::PathGapClip(örnek) => {
                format!(
                    "{} nokta · null/undefined boşluk ve kırpma yüzeyi",
                    örnek.nokta_sayısı()
                )
            }
            KartKimliği::PixelAlign(_) => {
                format!(
                    "{} canlı örnek × 3 seri · 120 sn görünür pencere",
                    self.pixel_align_adımı.min(1_000)
                )
            }
            KartKimliği::Points(örnek) => {
                format!(
                    "{} kaynak indeksi · koşullu nokta görünürlüğü",
                    örnek.nokta_sayısı()
                )
            }
            KartKimliği::ScalesDirOri(örnek) => {
                let (genişlik, yükseklik) = örnek.boyut();
                format!("10 nokta × 2 seri · {genişlik}×{yükseklik} · scale.dir/ori")
            }
            KartKimliği::Scatter(örnek) => {
                format!("{} nokta × 4 mode-2 facet", örnek.seri_başı_nokta())
            }
            KartKimliği::ScrollSync => "30 nokta × 3 seri · kaydırmada syncRect".to_string(),
            KartKimliği::SineStream => "600 nokta × 6 seri · 60 FPS setData".to_string(),
            KartKimliği::SoftMinMax(örnek) => {
                let davranış = match örnek {
                    SoftMinMaxÖrneği::MinKip0 => "sabit % alt pay",
                    SoftMinMaxÖrneği::MinKip1 => "veri aşarsa softMin",
                    SoftMinMaxÖrneği::MinKip2 => "pay aşarsa softMin",
                    SoftMinMaxÖrneği::MinKip3 => "koşullu softMin",
                    SoftMinMaxÖrneği::DüzSıfır => "soft aralık −1…1",
                };
                format!("2 kaynak noktası · {davranış}")
            }
            KartKimliği::SparklinesBars(_) => "16 nokta · sparkline + 16 yüzen çubuk".to_string(),
            KartKimliği::Sparklines(örnek) => {
                format!("{} · {} · 22 nokta · 150×30", örnek.simge(), örnek.ölçüm())
            }
            KartKimliği::Sparse(_) => "13.608 X · 4.608 dolu Y · 622 dolu parça".to_string(),
            KartKimliği::StackedSeries(örnek) => {
                let (genişlik, yükseklik) = örnek.boyut();
                format!("Kaynak yığma yüzeyi · {genişlik}×{yükseklik}")
            }
            KartKimliği::StreamData(örnek) => {
                let (başlangıç, uzunluk) = self
                    .stream_data_akışı
                    .as_ref()
                    .map_or((0, 0), |akış| (akış.başlangıç(), akış.uzunluk()));
                format!(
                    "{} · satır {başlangıç} · {uzunluk} görünür · 100 ms/10 satır setData",
                    örnek.başlık()
                )
            }
            KartKimliği::SvgImage => {
                "3 nokta × 1 seri · 400×200 bağımsız SVG görüntüsü".to_string()
            }
            KartKimliği::SyncCursor => {
                "5 yüzey · 3.004 nokta · iki cursor eşleme grubu".to_string()
            }
            KartKimliği::CursorBind => "30 nokta × 3 seri · Ctrl açıklama bağı".to_string(),
            KartKimliği::CursorSnap => "30 nokta × 3 seri".to_string(),
            KartKimliği::CursorTooltip => "7 nokta × 1 seri · canlı bilgi kutusu".to_string(),
            KartKimliği::CustomScales(_) => {
                "199 nokta × 3 seri · güven bandı + 20 gözlem".to_string()
            }
            KartKimliği::DataSmoothing(SmoothingÖrneği::Ham) => {
                "3600 resmî Taxi Trips örneği".to_string()
            }
            KartKimliği::DataSmoothing(SmoothingÖrneği::SavitzkyGolay) => {
                "3600 nokta · Savitzky–Golay pencere 101".to_string()
            }
            KartKimliği::DataSmoothing(SmoothingÖrneği::Asap) => {
                "137 nokta · ASAP FFT çözünürlük 150".to_string()
            }
            KartKimliği::DataSmoothing(SmoothingÖrneği::HareketliOrtalama) => {
                "3600 nokta · hareketli ortalama 300".to_string()
            }
            KartKimliği::DrawHooks => {
                "9 nokta × 3 seri · gradyan + medyan + yıldız + istatistik".to_string()
            }
            KartKimliği::FocusCursor(FocusÖrneği::İmleç) => {
                "130.000 nokta × 4 seri · alfa 0,3 · bias 1".to_string()
            }
            KartKimliği::FocusCursor(FocusÖrneği::Dinamik) => {
                "130.000 nokta × 3 seri · 30 px dinamik odak".to_string()
            }
            KartKimliği::FocusCursor(FocusÖrneği::KalınlıkVeRenk) => {
                "10 nokta × 2 seri · macenta odak".to_string()
            }
            KartKimliği::FocusCursor(FocusÖrneği::Performans300) => {
                "10 nokta × 300 seri · alfa 0,1".to_string()
            }
            KartKimliği::Gradients(GradientÖrneği::YatayÇizgi) => {
                "5 nokta · X'e hizalı 3 ayrık renk".to_string()
            }
            KartKimliği::Gradients(GradientÖrneği::DikeyÇizgi) => {
                "5 nokta · Y'ye hizalı mavi/kırmızı".to_string()
            }
            KartKimliği::Gradients(GradientÖrneği::DikeyArcSinh) => {
                "5 nokta · ArcSinh · 3 ayrık renk".to_string()
            }
            KartKimliği::Gradients(GradientÖrneği::ÖlçekDolguları) => {
                "6 nokta × 2 seri · ölçek dolguları".to_string()
            }
            KartKimliği::Gradients(GradientÖrneği::GöreliDolgu) => {
                "6 nokta · görünür min/orta/max dolgusu".to_string()
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
            KartKimliği::MissingDataNull => "200 nokta × 3 seri · % + MB".to_string(),
            KartKimliği::MissingDataXGap => "8 nokta × 1 seri · 2 yol parçası".to_string(),
            KartKimliği::DependentScale => "7 nokta × °F veri · türetilmiş °C ekseni".to_string(),
            KartKimliği::ArcSinhScales => "111 nokta · −1000…1000 ArcSinh".to_string(),
            KartKimliği::AxisControl => {
                "500.001 nokta · min/max piksel seyrekleştirme".to_string()
            }
            KartKimliği::AxisAutosize => {
                format!("501 nokta · çarpan 10^{}", self.autosize_kuvvet)
            }
            KartKimliği::AxisIndicators => "30 nokta × 3 bağımsız Y ekseni".to_string(),
            KartKimliği::Bars(örnek) => {
                format!("Kaynak alt grafik · {} seri", örnek.seri_sayısı())
            }
            KartKimliği::BarsValuesAutosize(_) => {
                "12 kanıt değeri · −100K…100K · otomatik etiket".to_string()
            }
            KartKimliği::BoxWhisker(_) => "İlk 30 keyed framework · medyan ve 1,5×IQR".to_string(),
            KartKimliği::Candlestick => "218 gün · Gold OHLC + kanıt hacmi".to_string(),
            KartKimliği::SyncYZero(aşama) => {
                format!("3 nokta × 3 Y ölçeği · {}", aşama.açıklama())
            }
            KartKimliği::ThinBars(örnek) => {
                let (genişlik, yükseklik) = örnek.boyut();
                format!("{genişlik}×{yükseklik} · {}", örnek.başlık())
            }
            KartKimliği::TimePeriods(örnek) => {
                format!("1920×200 · {}", örnek.başlık())
            }
            KartKimliği::TimelineDiscrete(örnek) => {
                format!("1920×300 · {}", örnek.başlık())
            }
            KartKimliği::TimeseriesDiscrete => {
                "50 ortak zaman noktası · 1 float + 3 ayrık seri".to_string()
            }
            KartKimliği::TimezonesDst(örnek) => {
                format!(
                    "600×200 · {} · {}",
                    örnek.bölüm(),
                    örnek.zaman_dilimi().iana()
                )
            }
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
        if aktif_kart == KartKimliği::SyncCursor {
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
            for (_, grafik) in &self.timeseries_discrete_grafikleri {
                if let Some((x, yüzey_değerleri)) = grafik.read(cx).lejant_değerleri() {
                    ortak_x = ortak_x.or(Some(x));
                    değerler.extend(yüzey_değerleri);
                }
            }
            ortak_x.map(|x| (x, değerler))
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
                        değer.map_or_else(|| format!("□ {ad}: --"), |y| format!("□ {ad}: {y:.3}"))
                    })
                    .collect::<Vec<_>>()
                    .join("    ");
                format!("x: {x:.3}    {seriler}")
            },
        );

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
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
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
            .children(FocusÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::FocusCursor(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "focus-cursor",
                    aktif_kart == kart,
                    "Y mesafesine göre çekirdek seri odağı",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(GradientÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::Gradients(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "gradients",
                    aktif_kart == kart,
                    "Ölçek hizalı çekirdek gradyanı",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
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
            .children(TimezonesDstÖrneği::tümü().map(|örnek| {
                let kart = KartKimliği::TimezonesDst(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "timezones-dst",
                    aktif_kart == kart,
                    format!("{} · {}", örnek.bölüm(), örnek.zaman_dilimi().iana()),
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
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
                    "Align Data · join cost",
                    "align-data",
                    aktif_kart == KartKimliği::AlignDataCost,
                    "5×5×1000 tablo · NULL_EXPAND",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::AlignDataCost, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "align-data-line-bars",
                    "Align Data · line + bars",
                    "align-data",
                    aktif_kart == KartKimliği::AlignDataLineBars,
                    "Farklı X dizilerinde çizgi + çubuk",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::AlignDataLineBars, cx);
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
                div()
                    .id("kart-zoom-wheel")
                    .cursor_pointer()
                    .mt_2()
                    .p_3()
                    .rounded_lg()
                    .border_1()
                    .border_color(if aktif_kart == KartKimliği::ZoomWheel {
                        vurgu
                    } else {
                        rgb(0xd1d5db)
                    })
                    .bg(if aktif_kart == KartKimliği::ZoomWheel {
                        rgb(0xfef2f2)
                    } else {
                        panel
                    })
                    .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                        bu.kartı_seç(KartKimliği::ZoomWheel, cx);
                    }))
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(metin)
                            .child("Wheel Zoom & Drag"),
                    )
                    .child(div().mt_1().text_xs().text_color(soluk).child("zoom-wheel"))
                    .child(
                        div()
                            .mt_2()
                            .text_xs()
                            .text_color(vurgu)
                            .child("Resmî tekerlek eklentisi · 2 seri"),
                    ),
            )
            .child(
                div()
                    .id("kart-zoom-touch")
                    .cursor_pointer()
                    .mt_2()
                    .p_3()
                    .rounded_lg()
                    .border_1()
                    .border_color(if aktif_kart == KartKimliği::ZoomTouch {
                        vurgu
                    } else {
                        rgb(0xd1d5db)
                    })
                    .bg(if aktif_kart == KartKimliği::ZoomTouch {
                        rgb(0xfef2f2)
                    } else {
                        panel
                    })
                    .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                        bu.kartı_seç(KartKimliği::ZoomTouch, cx);
                    }))
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(metin)
                            .child("Pinch Zoom & Pan"),
                    )
                    .child(div().mt_1().text_xs().text_color(soluk).child("zoom-touch"))
                    .child(
                        div()
                            .mt_2()
                            .text_xs()
                            .text_color(vurgu)
                            .child("Resmî touch eklentisi · 2 seri"),
                    ),
            )
            .child(
                katalog_kartı(
                    "kart-months-no-leap",
                    "No leap year",
                    "months-no-leap",
                    aktif_kart == KartKimliği::MonthsNoLeap,
                    "UTC ay ekseni · months.html",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::MonthsNoLeap, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-months-leap",
                    "2024 leap year",
                    "months-leap",
                    aktif_kart == KartKimliği::MonthsLeap,
                    "UTC ay ekseni · months.html",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::MonthsLeap, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-months-russian",
                    "Months · Russian",
                    "months-russian",
                    aktif_kart == KartKimliği::MonthsRussian,
                    "Rusça tarih adları · months-ru.html",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::MonthsRussian, cx);
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
            .children(NoDataÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::NoData(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "no-data",
                    aktif_kart == kart,
                    "Boş/tek/düz veri · kaynak rangeNum",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(PathGapClipÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::PathGapClip(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "path-gap-clip",
                    aktif_kart == kart,
                    "null/undefined · boşluk ve yol kırpması",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(PixelAlignÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::PixelAlign(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "pixel-align",
                    aktif_kart == kart,
                    "Canlı pxAlign 1 / 0 karşılaştırması",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(PointsÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::Points(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "points",
                    aktif_kart == kart,
                    "space · paths:null · tekil boşluk filtresi",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(ScalesDirOriÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::ScalesDirOri(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "scales-dir-ori",
                    aktif_kart == kart,
                    "scale.dir · scale.ori · axis.side",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(ScatterÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::Scatter(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "scatter",
                    aktif_kart == kart,
                    "mode:2 · facet · değişken balon alanı",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
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
            .children(SoftMinMaxÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::SoftMinMax(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "soft-minmax",
                    aktif_kart == kart,
                    "rangeNum · soft/pad/mode",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(SparklinesBarsÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::SparklinesBars(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "sparklines-bars",
                    aktif_kart == kart,
                    "sparkline · floating bars",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(SparklineÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::Sparklines(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "sparklines",
                    aktif_kart == kart,
                    "22 CSV kaydı · 150×30",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(SparseÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::Sparse(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "sparse",
                    aktif_kart == kart,
                    "13.608 X · 4.608 dolu Y",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(StackedSeriesÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::StackedSeries(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "stacked-series",
                    aktif_kart == kart,
                    "Yığma · bant · null/undefined · yüzde/grup",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(StreamDataÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::StreamData(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "stream-data",
                    aktif_kart == kart,
                    "55.550 kaynak satırı · 100 ms/10 satır setData",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .child(
                katalog_kartı(
                    "svg-image",
                    "uPlot to image PoC",
                    "svg-image",
                    aktif_kart == KartKimliği::SvgImage,
                    "400×200 · tek bağımsız SVG belgesi",
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
            .children(ThinBarsÖrneği::tümü().into_iter().map(|örnek| {
                let kart = KartKimliği::ThinBars(örnek);
                let (genişlik, yükseklik) = örnek.boyut();
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "thin-bars-stroke-fill",
                    aktif_kart == kart,
                    format!("{genişlik}×{yükseklik} · paths.bars vuruş/dolgu"),
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(TimePeriodsÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::TimePeriods(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "time-periods",
                    aktif_kart == kart,
                    "1920×200 · traffic.json kaynak verisi",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(TimelineDiscreteÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::TimelineDiscrete(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "timeline-discrete",
                    aktif_kart == kart,
                    "1920×300 · 3 şerit · semantic/discrete hücreler",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
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
            .children(CustomScaleÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::CustomScales(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "custom-scales",
                    aktif_kart == kart,
                    "199 nokta · güven bandı · 20 draw noktası",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(SmoothingÖrneği::TÜMÜ.into_iter().map(|örnek| {
                let kart = KartKimliği::DataSmoothing(örnek);
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.başlık(),
                    "data-smoothing",
                    aktif_kart == kart,
                    "Resmî Taxi Trips · kaynak JS algoritması",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
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
                    "kart-missing-data-null",
                    "Missing Data (null values)",
                    "missing-data-null",
                    aktif_kart == KartKimliği::MissingDataNull,
                    "200 özgün nokta · % ve MB ölçekleri",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::MissingDataNull, cx);
                })),
            )
            .child(
                katalog_kartı(
                    "kart-missing-data-x-gap",
                    "Adjacent X gap",
                    "missing-data-x-gap",
                    aktif_kart == KartKimliği::MissingDataXGap,
                    "X farkı > 1 olduğunda yolu böl",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(|bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::MissingDataXGap, cx);
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
            .children(ÇubukÖrneği::TÜMÜ.into_iter().map(|örnek| {
                katalog_kartı(
                    örnek.kimlik(),
                    örnek.kimlik(),
                    "bars-grouped-stacked",
                    aktif_kart == KartKimliği::Bars(örnek),
                    "Resmî grouped/stacked alt grafik",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(KartKimliği::Bars(örnek), cx);
                }))
            }))
            .children([ÇubukYönü::Dikey, ÇubukYönü::Yatay].into_iter().map(|yön| {
                let kart = KartKimliği::BarsValuesAutosize(yön);
                katalog_kartı(
                    kart.başlık(),
                    kart.başlık(),
                    "bars-values-autosize",
                    aktif_kart == kart,
                    "Resmî otomatik değer yazısı alt grafiği",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
            .children(BOX_WHISKER_BENCHMARKLERİ.into_iter().map(|benchmark| {
                let kart = KartKimliği::BoxWhisker(benchmark);
                katalog_kartı(
                    benchmark,
                    benchmark,
                    "box-whisker",
                    aktif_kart == kart,
                    "İlk 30 keyed framework · ayrık değerler",
                    panel,
                    vurgu,
                )
                .on_click(cx.listener(move |bu, _: &ClickEvent, _, cx| {
                    bu.kartı_seç(kart, cx);
                }))
            }))
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
                        Dugme::yeni("dinamik-seri-ekle", "Seri ekle")
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .tiklaninca(cx.listener(|bu, _, _, cx| bu.dinamik_seri_ekle(cx))),
                    )
                    .child(
                        Dugme::yeni("dinamik-seri-sil", "Seri sil")
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .devre_disi(mevcut_seri_sayısı < 2)
                            .tiklaninca(cx.listener(|bu, _, _, cx| bu.dinamik_seri_sil(cx))),
                    )
            })
            .when(matches!(aktif_kart, KartKimliği::StackedSeries(_)), |öğe| {
                öğe.children(
                    stacked_seriler
                        .into_iter()
                        .map(|(indeks, etiket, görünür)| {
                            let ad = if etiket.is_empty() {
                                format!("Seri {}", indeks + 1)
                            } else {
                                etiket
                            };
                            Dugme::yeni(
                                format!("stacked-seri-{indeks}"),
                                format!("{} {ad}", if görünür { "✓" } else { "○" }),
                            )
                            .boyutu(DugmeBoyutu::Kucuk)
                            .turu(DugmeTuru::Ikincil)
                            .tiklaninca(cx.listener(
                                move |bu, _, _, cx| {
                                    bu.stacked_seriyi_değiştir(indeks, cx);
                                },
                            ))
                        }),
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
                        if bu.aktif_kart == KartKimliği::SyncCursor {
                            for (_, grafik) in &bu.sync_cursor_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.önceki_görünüm(cx);
                                });
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
                        if bu.aktif_kart == KartKimliği::SyncCursor {
                            for (_, grafik) in &bu.sync_cursor_grafikleri {
                                grafik.update(cx, |grafik, cx| {
                                    grafik.tam_görünüm(cx);
                                });
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
                        if bu.aktif_kart == KartKimliği::SyncCursor {
                            bu.sync_cursor_grubu = SyncCursorGrubu::yeni();
                            bu.sync_cursor_yüzeylerini_oluştur(cx);
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
        let çizim = if aktif_kart == KartKimliği::SyncCursor {
            let cpu = sync_yüzeyi(SyncCursorÖrneği::Cpu);
            let ram = sync_yüzeyi(SyncCursorÖrneği::Ram);
            let tcp = sync_yüzeyi(SyncCursorÖrneği::Tcp);
            let kırmızı_mavi = sync_yüzeyi(SyncCursorÖrneği::UyumsuzKırmızıMavi);
            let yeşil_kırmızı = sync_yüzeyi(SyncCursorÖrneği::UyumsuzYeşilKırmızı);
            çizim_tabanı
                .flex_none()
                .h(px(740.0))
                .overflow_y_scroll()
                .p_2()
                .child(
                    div()
                        .w_full()
                        .h(px(220.0))
                        .when_some(cpu, |öğe, grafik| öğe.child(grafik)),
                )
                .child(
                    div()
                        .mt_2()
                        .flex()
                        .gap_2()
                        .children([ram, tcp].into_iter().map(|grafik| {
                            div()
                                .flex_1()
                                .min_w_0()
                                .h(px(220.0))
                                .when_some(grafik, |öğe, grafik| öğe.child(grafik))
                        })),
                )
                .child(div().mt_2().flex().gap_2().children(
                    [kırmızı_mavi, yeşil_kırmızı].into_iter().map(|grafik| {
                        div()
                            .flex_1()
                            .min_w_0()
                            .h(px(220.0))
                            .when_some(grafik, |öğe, grafik| öğe.child(grafik))
                    }),
                ))
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
                .flex_none()
                .h(px(720.0))
                .overflow_y_scroll()
                .p_2()
                .child(
                    div()
                        .w_full()
                        .h(px(500.0))
                        .when_some(üst, |öğe, grafik| öğe.child(grafik)),
                )
                .child(
                    div()
                        .mt_2()
                        .w_full()
                        .h(px(180.0))
                        .when_some(alt, |öğe, grafik| öğe.child(grafik)),
                )
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
                    .child("Contrary to popular belief, Lorem Ipsum is not simply random text. Kaydırılabilir içerik grafiğin pencere konumunu değiştirir.")
                    .child(
                        div()
                            .my_3()
                            .w(px(400.0))
                            .h(px(200.0))
                            .when_some(self.grafik.clone(), |öğe, grafik| öğe.child(grafik)),
                    )
                    .child("Grafiği kaydırdıktan sonra imleç ve seçim aynı görsel noktada kalır. GPUI sınırları her yerleşimde, istemci → sahne dönüşümü ise ortak Rust çekirdeğinde yenilenir.")
                    .child(div().h(px(260.0))),
                )
        } else {
            çizim_tabanı
                .overflow_hidden()
                .when_some(self.grafik.clone(), |öğe, grafik| öğe.child(grafik))
        };

        let yardım = match aktif_kart {
            KartKimliği::AddDelSeries => {
                "Seri ekle: turuncu seriyi kaynak indeksi 2'ye ekle · Seri sil: aynı indeksi kaldır"
            }
            KartKimliği::CursorBind => {
                "Sürükle: yakınlaştır · Ctrl+sürükle: sarı açıklama seçimi · açıklama seçimi zoom yapmaz"
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
            _ => {
                "Sürükle: seç · boşluk + sürükle: taşı · kıstır: X/Y yakınlaştır · çift tıkla: tam görünüm"
            }
        };
        let açıklama_istendi = self.açıklama_istendi;
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
            .child(div().mb_2().text_xs().text_color(soluk).child(yardım))
            .child(div().mb_2().text_xs().text_color(vurgu).child(lejant))
            .when(açıklama_istendi, |öğe| {
                öğe.child(
                    div()
                        .mb_2()
                        .p_2()
                        .rounded_md()
                        .bg(rgb(0xfffbeb))
                        .text_sm()
                        .text_color(rgb(0x92400e))
                        .child("Annotation Text istendi · kaynak demo girilen metni kalıcı çizime eklemez"),
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
                        Dugme::yeni(
                            "kart-tanimi-toggle",
                            kart_tanımı_etiketi,
                        )
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
            .flex()
            .flex_row()
            .bg(zemin)
            .child(liste)
            .child(ayrıntı);

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
