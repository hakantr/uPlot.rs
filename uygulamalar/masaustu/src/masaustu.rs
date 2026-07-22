//! GPUI masaüstü chart kataloğu; dağıtılan bileşeni kullanan örnek uygulama.

use gpui::{
    Context, Entity, FontWeight, IntoElement, Render, SharedString, Window, div, prelude::*, px,
    rgb,
};
use ortak_bilesenler::{
    Anahtar, AnahtarOlayi, CubukAyarlari, Dugme, DugmeBoyutu, DugmeTuru, PlatformPencere,
};
use uplot_rs::gpui::{GpuiGrafik, GpuiGrafikOlayı};
use uplot_rs::{
    Grafik, UplotHatası, ilk_kart_etkileşimleri, sinüs_kartı, İLK_KART_TANIM_ÖRNEĞİ
};

pub struct ChartListesi {
    nokta_sayısı: usize,
    grafik: Option<Entity<GpuiGrafik>>,
    hata: Option<String>,
    kart_tanımı_açık: bool,
    tekerlek_etkin: bool,
    tekerlek_anahtarı: Entity<Anahtar>,
}

impl ChartListesi {
    pub fn yeni(cx: &mut Context<Self>) -> Self {
        let etkileşimler = ilk_kart_etkileşimleri();
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

        let (grafik, hata) = grafik_oluştur(100).map_or_else(
            |hata| (None, Some(format!("Grafik oluşturulamadı: {hata}"))),
            |grafik| (Some(cx.new(|_| GpuiGrafik::yeni(grafik))), None),
        );
        if let Some(grafik) = &grafik {
            cx.subscribe(grafik, |_, _, _: &GpuiGrafikOlayı, cx| cx.notify())
                .detach();
        }
        Self {
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
        match grafik_oluştur(nokta_sayısı) {
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
}

fn grafik_oluştur(nokta_sayısı: usize) -> Result<Grafik, UplotHatası> {
    let (seçenekler, veri) = sinüs_kartı(nokta_sayısı)?;
    Grafik::yeni(seçenekler, veri)
}

impl Render for ChartListesi {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let panel = rgb(0xffffff);
        let zemin = rgb(0xf3f4f6);
        let metin = rgb(0x111827);
        let soluk = rgb(0x6b7280);
        let vurgu = rgb(0xdc2626);
        let nokta_yazısı = SharedString::from(format!("{} nokta", self.nokta_sayısı));
        let kart_tanımı_açık = self.kart_tanımı_açık;
        let tekerlek_anahtarı = self.tekerlek_anahtarı.clone();
        let (geri_var, yakınlaştırılmış, etkileşimler, lejant, bileşen_hatası) =
            self.grafik.as_ref().map_or_else(
                || (false, false, ilk_kart_etkileşimleri(), None, None),
                |grafik| {
                    let grafik = grafik.read(cx);
                    (
                        grafik.grafik().geri_var(),
                        grafik.grafik().yakınlaştırılmış(),
                        grafik.grafik().etkileşim_seçenekleri(),
                        grafik.lejant(),
                        grafik.hata().map(str::to_string),
                    )
                },
            );
        let çizim_hatası = self.hata.clone().or(bileşen_hatası);
        let lejant = lejant.map_or_else(
            || "x: --    □ sin(x): --".to_string(),
            |(x, y)| format!("x: {x:.3}    □ sin(x): {y:.3}"),
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
                    .p_3()
                    .rounded_lg()
                    .border_1()
                    .border_color(vurgu)
                    .bg(rgb(0xfef2f2))
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
                    .tiklaninca(cx.listener(|bu, _, _, cx| {
                        bu.grafiği_yenile(bu.nokta_sayısı.saturating_sub(10).max(10), cx);
                    })),
            )
            .child(
                Dugme::yeni("nokta-artir", "＋ Nokta")
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(DugmeTuru::Ikincil)
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
                            .child("Resize · sayısal x ölçeği"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(soluk)
                            .child("Hazır GPUI yüzeyi · davranışlar uplot-rs çekirdeğinde"),
                    ),
            )
            .child(araçlar)
            .child(
                div()
                    .mb_2()
                    .text_xs()
                    .text_color(soluk)
                    .child("Sürükle: seç · boşluk + sürükle: taşı · kıstır: X/Y yakınlaştır"),
            )
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
                            if kart_tanımı_açık {
                                "▾ Kart tanımı · src/kart.rs"
                            } else {
                                "▸ Kart tanımı · src/kart.rs"
                            },
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
                                .child(İLK_KART_TANIM_ÖRNEĞİ),
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
