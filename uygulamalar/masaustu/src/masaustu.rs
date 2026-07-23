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
    ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ, ALIGN_DATA_KART_TANIM_ÖRNEĞİ,
    ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ, AREA_FILL_KART_TANIM_ÖRNEĞİ, AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    AXIS_CONTROL_KART_TANIM_ÖRNEĞİ, AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ,
    BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ, BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    BOX_WHISKER_BENCHMARKLERİ, BOX_WHISKER_KART_TANIM_ÖRNEĞİ, CANDLESTICK_KART_TANIM_ÖRNEĞİ,
    CURSOR_BIND_KART_TANIM_ÖRNEĞİ, CURSOR_SNAP_KART_TANIM_ÖRNEĞİ, CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ,
    CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ, CustomScaleÖrneği, DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ,
    DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ, DRAW_HOOKS_KART_TANIM_ÖRNEĞİ, EtkileşimSeçenekleri,
    FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ, FocusÖrneği, GRADIENTS_KART_TANIM_ÖRNEĞİ,
    GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ, GradientÖrneği, Grafik, HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ,
    HighLowBandsÖrneği, LATENCY_HEATMAP_KART_TANIM_ÖRNEĞİ, LINE_PATHS_KART_TANIM_ÖRNEĞİ,
    LOG_SCALES_KART_TANIM_ÖRNEĞİ, LOG_SCALES2_KART_TANIM_ÖRNEĞİ, LatencyHeatmapÖrneği,
    LinePathsÖrneği, LogScales2Örneği, LogScalesÖrneği, MISSING_DATA_KART_TANIM_ÖRNEĞİ,
    MONTHS_KART_TANIM_ÖRNEĞİ, NICE_SCALE_KART_TANIM_ÖRNEĞİ, NO_DATA_KART_TANIM_ÖRNEĞİ,
    NoDataÖrneği, RESIZE_KART_TANIM_ÖRNEĞİ, SCALE_PADDING_KART_TANIM_ÖRNEĞİ, SeriSeçenekleri,
    SmoothingÖrneği, UplotHatası, ZOOM_TOUCH_KART_TANIM_ÖRNEĞİ, ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ,
    add_del_series_ek_verisi, add_del_series_kartı, align_data_maliyet_kartı,
    align_data_çizgi_çubuk_kartı, arcsinh_scales_kartı, area_fill_kartı, axis_autosize_kartı,
    axis_control_kartı, axis_indicators_kartı, bars_grouped_stacked_kartı,
    bars_values_autosize_kartı, box_whisker_kartı, candlestick_ohlc_kartı, cursor_bind_kartı,
    cursor_snap_kartı, cursor_tooltip_kartı, custom_scales_kartı, data_smoothing_kartı,
    dependent_scale_kartı, draw_hooks_kartı, focus_cursor_kartı, gradients_kartı,
    grid_over_series_kartı, high_low_bands_kartı, latency_heatmap_kartı, line_paths_kartı,
    log_scales_kartı, log_scales2_kartı, missing_data_null_kartı, missing_data_x_boşluğu_kartı,
    months_artık_yıllı_kartı, months_artık_yılsız_kartı, months_rusça_kartı, nice_scale_kartı,
    no_data_kartı, ortak_kart_etkileşimleri, resize_kartı, scale_padding_kartı, zoom_touch_kartı,
    zoom_wheel_kartı, ÇubukYönü, ÇubukÖrneği,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum KartKimliği {
    AddDelSeries,
    AlignDataCost,
    AlignDataLineBars,
    Resize,
    AreaFill,
    ScalePadding,
    ZoomWheel,
    ZoomTouch,
    MonthsNoLeap,
    MonthsLeap,
    MonthsRussian,
    NiceScale,
    NoData(NoDataÖrneği),
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
            Self::AreaFill => "Area Fill",
            Self::ScalePadding => "Scale Padding · Flat",
            Self::ZoomWheel => "Wheel Zoom & Drag",
            Self::ZoomTouch => "Pinch Zoom & Pan",
            Self::MonthsNoLeap => "Months · No leap year",
            Self::MonthsLeap => "Months · 2024 leap year",
            Self::MonthsRussian => "Months · Russian",
            Self::NiceScale => "Nice Scale & Ticks",
            Self::NoData(örnek) => örnek.başlık(),
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
            Self::AreaFill => AREA_FILL_KART_TANIM_ÖRNEĞİ,
            Self::ScalePadding => SCALE_PADDING_KART_TANIM_ÖRNEĞİ,
            Self::ZoomWheel => ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ,
            Self::ZoomTouch => ZOOM_TOUCH_KART_TANIM_ÖRNEĞİ,
            Self::MonthsNoLeap | Self::MonthsLeap | Self::MonthsRussian => {
                MONTHS_KART_TANIM_ÖRNEĞİ
            }
            Self::NiceScale => NICE_SCALE_KART_TANIM_ÖRNEĞİ,
            Self::NoData(_) => NO_DATA_KART_TANIM_ÖRNEĞİ,
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
            Self::AreaFill => "src/kart/area_fill.rs",
            Self::ScalePadding => "src/kart/scale_padding.rs",
            Self::ZoomWheel => "src/kart/zoom_wheel.rs",
            Self::ZoomTouch => "src/kart/zoom_touch.rs",
            Self::MonthsNoLeap | Self::MonthsLeap | Self::MonthsRussian => "src/kart/months.rs",
            Self::NiceScale => "src/kart/nice_scale.rs",
            Self::NoData(_) => "src/kart/no_data.rs",
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
            if let Some(grafik) = &bu.grafik {
                grafik.update(cx, |grafik, cx| {
                    grafik.tekerlek_etkileşimi_ayarla(etkin, cx);
                });
            }
            bu.tekerlek_etkin = etkin;
            cx.notify();
        })
        .detach();

        let (grafik, hata) = grafik_oluştur(KartKimliği::Resize, 100, 0, 5, 0).map_or_else(
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
        }
    }

    fn grafiği_yenile(&mut self, nokta_sayısı: usize, cx: &mut Context<Self>) {
        self.nokta_sayısı = nokta_sayısı;
        match grafik_oluştur(
            self.aktif_kart,
            nokta_sayısı,
            self.autosize_kuvvet,
            self.latency_kova,
            self.latency_ofset,
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
        let etkileşimler = kart.etkileşimler();
        self.tekerlek_etkin = etkileşimler.tekerlek_etkileşimi;
        self.tekerlek_anahtarı.update(cx, |anahtar, cx| {
            anahtar.ayarla(etkileşimler.tekerlek_etkileşimi, cx);
            anahtar.devre_disi_ayarla(false, cx);
        });
        self.grafiği_yenile(self.nokta_sayısı, cx);
        if kart == KartKimliği::AlignDataCost {
            self.align_data_zamanlayıcısı = Some(cx.spawn(async move |bu, cx| {
                let mut etkin = false;
                loop {
                    cx.background_executor().timer(Duration::from_secs(1)).await;
                    etkin = !etkin;
                    let devam = bu
                        .update(cx, |bu, cx| {
                            if bu.aktif_kart != KartKimliği::AlignDataCost {
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
        }
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
}

fn grafik_oluştur(
    kart: KartKimliği,
    nokta_sayısı: usize,
    autosize_kuvvet: i32,
    latency_kova: u8,
    latency_ofset: u8,
) -> Result<Grafik, UplotHatası> {
    let (seçenekler, veri) = match kart {
        KartKimliği::AddDelSeries => add_del_series_kartı(),
        KartKimliği::AlignDataCost => align_data_maliyet_kartı(),
        KartKimliği::AlignDataLineBars => align_data_çizgi_çubuk_kartı(),
        KartKimliği::Resize => resize_kartı(nokta_sayısı),
        KartKimliği::AreaFill => area_fill_kartı(),
        KartKimliği::ScalePadding => scale_padding_kartı(),
        KartKimliği::ZoomWheel => zoom_wheel_kartı(),
        KartKimliği::ZoomTouch => zoom_touch_kartı(),
        KartKimliği::MonthsNoLeap => months_artık_yılsız_kartı(),
        KartKimliği::MonthsLeap => months_artık_yıllı_kartı(),
        KartKimliği::MonthsRussian => months_rusça_kartı(),
        KartKimliği::NiceScale => nice_scale_kartı(),
        KartKimliği::NoData(örnek) => no_data_kartı(örnek),
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
        let mevcut_seri_sayısı = self.grafik.as_ref().map_or(0, |grafik| {
            grafik.read(cx).grafik().seri_seçenekleri().len()
        });
        let nokta_yazısı = SharedString::from(match aktif_kart {
            KartKimliği::AddDelSeries => {
                format!("30 nokta × {mevcut_seri_sayısı} dinamik seri")
            }
            KartKimliği::AlignDataCost => {
                "5 tablo × 5 seri × 1000 X · birleşik sıralı X".to_string()
            }
            KartKimliği::AlignDataLineBars => "38 noktalı çizgi + 4 çubuk".to_string(),
            KartKimliği::Resize => format!("{} nokta", self.nokta_sayısı),
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
        });
        let kart_tanımı_açık = self.kart_tanımı_açık;
        let kart_tanımı_etiketi = SharedString::from(format!(
            "{} Kart tanımı · {}",
            if kart_tanımı_açık { "▾" } else { "▸" },
            aktif_kart.tanım_yolu()
        ));
        let tekerlek_anahtarı = self.tekerlek_anahtarı.clone();
        let (geri_var, yakınlaştırılmış, etkileşimler, lejant, bileşen_hatası) =
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
        let çizim_hatası = self.hata.clone().or(bileşen_hatası);
        let seri_adları = self.grafik.as_ref().map_or_else(Vec::new, |grafik| {
            grafik
                .read(cx)
                .grafik()
                .seri_seçenekleri()
                .iter()
                .filter(|seri| seri.göster)
                .map(|seri| seri.etiket.clone())
                .collect::<Vec<_>>()
        });
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
                        if let Some(grafik) = &bu.grafik {
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
                        if let Some(grafik) = &bu.grafik {
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
                    .tiklaninca(cx.listener(|bu, _, _, cx| bu.grafiği_yenile(100, cx))),
            );

        let çizim = div()
            .id("canli-chart")
            .flex_1()
            .min_h(px(320.0))
            .rounded_lg()
            .border_1()
            .border_color(rgb(0xe5e7eb))
            .bg(panel)
            .overflow_hidden()
            .when_some(self.grafik.clone(), |öğe, grafik| öğe.child(grafik));

        let yardım = match aktif_kart {
            KartKimliği::AddDelSeries => {
                "Seri ekle: turuncu seriyi kaynak indeksi 2'ye ekle · Seri sil: aynı indeksi kaldır"
            }
            KartKimliği::CursorBind => {
                "Sürükle: yakınlaştır · Ctrl+sürükle: sarı açıklama seçimi · açıklama seçimi zoom yapmaz"
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
    kimlik: &'static str,
    başlık: &'static str,
    alt_kimlik: &'static str,
    aktif: bool,
    durum: &'static str,
    panel: gpui::Rgba,
    vurgu: gpui::Rgba,
) -> gpui::Stateful<gpui::Div> {
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
