//! GPUI masaüstü chart kataloğu; dağıtılan bileşeni kullanan örnek uygulama.

use gpui::{
    ClickEvent, Context, Entity, FontWeight, IntoElement, Render, SharedString, Window, div,
    prelude::*, px, rgb,
};
use ortak_bilesenler::{
    Anahtar, AnahtarOlayi, CubukAyarlari, Dugme, DugmeBoyutu, DugmeTuru, PlatformPencere,
};
use uplot_rs::gpui::{GpuiGrafik, GpuiGrafikOlayı};
use uplot_rs::{
    AREA_FILL_KART_TANIM_ÖRNEĞİ, EtkileşimSeçenekleri, Grafik, RESIZE_KART_TANIM_ÖRNEĞİ,
    SCALE_PADDING_KART_TANIM_ÖRNEĞİ, UplotHatası, area_fill_kartı, ortak_kart_etkileşimleri,
    resize_kartı, scale_padding_kartı,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum KartKimliği {
    Resize,
    AreaFill,
    ScalePadding,
}

impl KartKimliği {
    fn başlık(self) -> &'static str {
        match self {
            Self::Resize => "Resize · sayısal x ölçeği",
            Self::AreaFill => "Area Fill",
            Self::ScalePadding => "Scale Padding · Flat",
        }
    }

    fn kaynak(self) -> &'static str {
        match self {
            Self::Resize => "resize.html + zoom-wheel.html + zoom-touch.html",
            Self::AreaFill => {
                "area-fill.html · kaynakla aynı veri üreteci · ortak Resize etkileşim profili"
            }
            Self::ScalePadding => {
                "scale-padding.html · 13 düz seri · kaynakla aynı değer düzeyleri"
            }
        }
    }

    fn tanım(self) -> &'static str {
        match self {
            Self::Resize => RESIZE_KART_TANIM_ÖRNEĞİ,
            Self::AreaFill => AREA_FILL_KART_TANIM_ÖRNEĞİ,
            Self::ScalePadding => SCALE_PADDING_KART_TANIM_ÖRNEĞİ,
        }
    }

    fn tanım_yolu(self) -> &'static str {
        match self {
            Self::Resize => "src/kart/resize.rs",
            Self::AreaFill => "src/kart/area_fill.rs",
            Self::ScalePadding => "src/kart/scale_padding.rs",
        }
    }

    fn etkileşimler(self) -> EtkileşimSeçenekleri {
        ortak_kart_etkileşimleri()
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

        let (grafik, hata) = grafik_oluştur(KartKimliği::Resize, 100).map_or_else(
            |hata| (None, Some(format!("Grafik oluşturulamadı: {hata}"))),
            |grafik| (Some(cx.new(|_| GpuiGrafik::yeni(grafik))), None),
        );
        if let Some(grafik) = &grafik {
            cx.subscribe(grafik, |_, _, _: &GpuiGrafikOlayı, cx| cx.notify())
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
        }
    }

    fn grafiği_yenile(&mut self, nokta_sayısı: usize, cx: &mut Context<Self>) {
        self.nokta_sayısı = nokta_sayısı;
        match grafik_oluştur(self.aktif_kart, nokta_sayısı) {
            Ok(mut yeni) => {
                yeni.tekerlek_etkileşimi_ayarla(self.tekerlek_etkin);
                if let Some(grafik) = &self.grafik {
                    grafik.update(cx, |grafik, cx| grafik.grafiği_ayarla(yeni, cx));
                } else {
                    let grafik = cx.new(|_| GpuiGrafik::yeni(yeni));
                    cx.subscribe(&grafik, |_, _, _: &GpuiGrafikOlayı, cx| cx.notify())
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
        let etkileşimler = kart.etkileşimler();
        self.tekerlek_etkin = etkileşimler.tekerlek_etkileşimi;
        self.tekerlek_anahtarı.update(cx, |anahtar, cx| {
            anahtar.ayarla(etkileşimler.tekerlek_etkileşimi, cx);
            anahtar.devre_disi_ayarla(false, cx);
        });
        self.grafiği_yenile(self.nokta_sayısı, cx);
    }
}

fn grafik_oluştur(kart: KartKimliği, nokta_sayısı: usize) -> Result<Grafik, UplotHatası> {
    let (seçenekler, veri) = match kart {
        KartKimliği::Resize => resize_kartı(nokta_sayısı),
        KartKimliği::AreaFill => area_fill_kartı(),
        KartKimliği::ScalePadding => scale_padding_kartı(),
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
        let nokta_yazısı = SharedString::from(match aktif_kart {
            KartKimliği::Resize => format!("{} nokta", self.nokta_sayısı),
            KartKimliği::AreaFill => "30 sabit nokta × 3 seri".to_string(),
            KartKimliği::ScalePadding => "10 nokta × 13 düz seri".to_string(),
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
        let seri_adları: &[&str] = match aktif_kart {
            KartKimliği::Resize => &["sin(x)"],
            KartKimliği::AreaFill => &["1", "2", "3"],
            KartKimliği::ScalePadding => &[
                "-10500", "-10000", "-9500", "-0.105", "-0.1", "-0.095", "0", "0.095", "0.1",
                "0.105", "9500", "10000", "10500",
            ],
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
            .w(px(280.0))
            .h_full()
            .flex_none()
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
            .child(div().mb_2().text_xs().text_color(soluk).child(
                "Sürükle: seç · boşluk + sürükle: taşı · kıstır: X/Y yakınlaştır · çift tıkla: tam görünüm",
            ))
            .child(div().mb_2().text_xs().text_color(vurgu).child(lejant))
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
