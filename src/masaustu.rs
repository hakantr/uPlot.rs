//! GPUI masaüstü chart listesi ve sahne adaptörü.

use std::cell::Cell;
use std::collections::VecDeque;
use std::f64::consts::PI;
use std::rc::Rc;

use gpui::{
    App, BorderStyle, Bounds, Context, Entity, FontWeight, Hsla, IntoElement, MouseButton,
    MouseDownEvent, MouseExitEvent, MouseMoveEvent, MouseUpEvent, PathBuilder, Pixels, Render,
    ScrollDelta, ScrollWheelEvent, SharedString, TextAlign, TextRun, Window, canvas, div, point,
    prelude::*, px, quad, rgb, rgba, size,
};
use ortak_bilesenler::{
    Anahtar, AnahtarOlayi, CubukAyarlari, Dugme, DugmeBoyutu, DugmeTuru, PlatformPencere,
};

use crate::{
    Aralık, EtkileşimSeçenekleri, Grafik, Komut, MetinHizası, Nokta, Sahne, UplotHatası,
    ilk_kart_etkileşimleri, sinüs_kartı, İLK_KART_TANIM_ÖRNEĞİ,
};

#[derive(Clone, Copy)]
struct İmleçDurumu {
    fare: Nokta,
    veri_x: f64,
    veri_y: f64,
}

pub struct ChartListesi {
    nokta_sayısı: usize,
    x_aralığı: Option<Aralık>,
    imleç: Option<İmleçDurumu>,
    seçim: Option<(f32, f32)>,
    hata: Option<String>,
    kart_tanımı_açık: bool,
    görünüm_geçmişi: VecDeque<Option<Aralık>>,
    etkileşimler: EtkileşimSeçenekleri,
    tekerlek_anahtarı: Entity<Anahtar>,
    çizim_sınırları: Rc<Cell<Option<Bounds<Pixels>>>>,
}

impl ChartListesi {
    pub fn yeni(cx: &mut Context<Self>) -> Self {
        let etkileşimler = ilk_kart_etkileşimleri();
        let tekerlek_anahtarı =
            cx.new(|cx| Anahtar::yeni("Tekerlek eklentisi", etkileşimler.tekerlek_etkileşimi, cx));
        cx.subscribe(&tekerlek_anahtarı, |bu, _, olay: &AnahtarOlayi, cx| {
            let AnahtarOlayi::Degisti(etkin) = *olay;
            bu.etkileşimler.tekerlek_etkileşimi = etkin;
            cx.notify();
        })
        .detach();
        Self {
            nokta_sayısı: 100,
            x_aralığı: None,
            imleç: None,
            seçim: None,
            hata: None,
            kart_tanımı_açık: false,
            görünüm_geçmişi: VecDeque::new(),
            etkileşimler,
            tekerlek_anahtarı,
            çizim_sınırları: Rc::new(Cell::new(None)),
        }
    }

    fn sahne(&self) -> Result<Sahne, UplotHatası> {
        let (seçenekler, veri) = sinüs_kartı(self.nokta_sayısı)?;
        let mut sahne = Grafik::yeni(seçenekler, veri)?.çiz_aralıkta(self.x_aralığı);
        let x_aralığı = self.geçerli_x_aralığı();
        let y_aralığı = görünür_y_aralığı(self.nokta_sayısı, x_aralığı);
        if let Some(imleç) = self.imleç {
            let nokta_x = ölçekle(imleç.veri_x, x_aralığı, 64.0, 712.0);
            let nokta_y = 352.0 - ölçekle(imleç.veri_y, y_aralığı, 0.0, 304.0);
            sahne.ekle(Komut::KesikliÇizgi {
                başlangıç: Nokta::yeni(imleç.fare.x, 48.0),
                bitiş: Nokta::yeni(imleç.fare.x, 352.0),
                renk: "#6b7280".to_string(),
                kalınlık: 1.0,
                kesik: 4.0,
            });
            sahne.ekle(Komut::KesikliÇizgi {
                başlangıç: Nokta::yeni(64.0, imleç.fare.y),
                bitiş: Nokta::yeni(776.0, imleç.fare.y),
                renk: "#6b7280".to_string(),
                kalınlık: 1.0,
                kesik: 4.0,
            });
            sahne.ekle(Komut::Daire {
                merkez: Nokta::yeni(nokta_x, nokta_y),
                yarıçap: 2.5,
                dolgu: "#dc2626".to_string(),
                çizgi: "#dc2626".to_string(),
                kalınlık: 0.0,
            });
        }
        if let Some((baş, son)) = self.seçim {
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(baş.min(son), 48.0),
                genişlik: (son - baş).abs(),
                yükseklik: 304.0,
                dolgu: "#3b82f633".to_string(),
                çizgi: "#3b82f6".to_string(),
                kalınlık: 1.0,
            });
        }
        Ok(sahne)
    }

    fn geçerli_x_aralığı(&self) -> Aralık {
        self.x_aralığı
            .unwrap_or_else(|| tam_x_aralığı(self.nokta_sayısı))
    }

    fn sahne_konumu(&self, pencere_konumu: gpui::Point<Pixels>) -> Option<Nokta> {
        let sınırlar = self.çizim_sınırları.get()?;
        let ölçek = (f32::from(sınırlar.size.width) / 800.0)
            .min(f32::from(sınırlar.size.height) / 400.0)
            .max(0.01);
        let köken_x =
            f32::from(sınırlar.origin.x) + (f32::from(sınırlar.size.width) - 800.0 * ölçek) / 2.0;
        let köken_y =
            f32::from(sınırlar.origin.y) + (f32::from(sınırlar.size.height) - 400.0 * ölçek) / 2.0;
        Some(Nokta::yeni(
            (f32::from(pencere_konumu.x) - köken_x) / ölçek,
            (f32::from(pencere_konumu.y) - köken_y) / ölçek,
        ))
    }

    fn imleci_güncelle(&mut self, pencere_konumu: gpui::Point<Pixels>) {
        let Some(fare) = self.sahne_konumu(pencere_konumu) else {
            self.imleç = None;
            return;
        };
        if !(64.0..=776.0).contains(&fare.x) || !(48.0..=352.0).contains(&fare.y) {
            self.imleç = None;
            return;
        }
        let aralık = self.geçerli_x_aralığı();
        let ham_x = ters_ölçekle(fare.x, aralık, 64.0, 712.0);
        let adım = 2.0 * PI / self.nokta_sayısı as f64;
        let indeks = (ham_x / adım)
            .round()
            .clamp(0.0, self.nokta_sayısı.saturating_sub(1) as f64);
        let x_değeri = indeks * adım;
        self.imleç = Some(İmleçDurumu {
            fare,
            veri_x: x_değeri,
            veri_y: x_değeri.sin(),
        });
    }

    fn görünümü_uygula(&mut self, yeni: Option<Aralık>) {
        if self.x_aralığı == yeni {
            return;
        }
        if self.etkileşimler.görünüm_geçmişi {
            if self.görünüm_geçmişi.len() >= 100 {
                self.görünüm_geçmişi.pop_front();
            }
            self.görünüm_geçmişi.push_back(self.x_aralığı);
        }
        self.x_aralığı = yeni;
        self.hata = None;
    }

    fn önceki_görünüme_dön(&mut self) {
        if let Some(önceki) = self.görünüm_geçmişi.pop_back() {
            self.x_aralığı = önceki;
            self.hata = None;
        }
    }

    fn tekerlek_yakınlaştır(&mut self, olay: &ScrollWheelEvent) {
        if !self.etkileşimler.tekerlek_etkileşimi {
            return;
        }
        let Some(fare) = self.sahne_konumu(olay.position) else {
            return;
        };
        if !(64.0..=776.0).contains(&fare.x) || !(48.0..=352.0).contains(&fare.y) {
            return;
        }
        let yön = match olay.delta {
            ScrollDelta::Pixels(delta) => f32::from(delta.y),
            ScrollDelta::Lines(delta) => delta.y,
        };
        if !yön.is_finite() || yön.abs() <= f32::EPSILON {
            return;
        }

        let tam = tam_x_aralığı(self.nokta_sayısı);
        let mevcut = self.geçerli_x_aralığı();
        let odak = ters_ölçekle(fare.x, mevcut, 64.0, 712.0);
        match mevcut.tekerlek_yakınlaştır(tam, odak, yön > 0.0) {
            Ok(aralık) => self.görünümü_uygula((aralık != tam).then_some(aralık)),
            Err(hata) => {
                self.hata = Some(format!("Tekerlek yakınlaştırması uygulanamadı: {hata}"));
            }
        }
    }
}

impl Render for ChartListesi {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let panel = rgb(0xffffff);
        let zemin = rgb(0xf3f4f6);
        let metin = rgb(0x111827);
        let soluk = rgb(0x6b7280);
        let vurgu = rgb(0xdc2626);
        let (sahne, çizim_hatası) = match self.sahne() {
            Ok(sahne) => (Some(sahne), self.hata.clone()),
            Err(hata) => (None, Some(format!("Grafik çizilemedi: {hata}"))),
        };
        let nokta_yazısı = SharedString::from(format!("{} nokta", self.nokta_sayısı));
        let lejant = self.imleç.map_or_else(
            || "x: --    □ sin(x): --".to_string(),
            |imleç| format!("x: {:.3}    □ sin(x): {:.3}", imleç.veri_x, imleç.veri_y),
        );
        let çizim_sınırları = self.çizim_sınırları.clone();
        let kart_tanımı_açık = self.kart_tanımı_açık;
        let geri_var = !self.görünüm_geçmişi.is_empty();
        let yakınlaştırılmış = self.x_aralığı.is_some();
        let tekerlek_anahtarı = self.tekerlek_anahtarı.clone();

        let çizim = div()
            .id("canli-chart")
            .flex_1()
            .min_h(px(320.0))
            .rounded_lg()
            .border_1()
            .border_color(rgb(0xe5e7eb))
            .bg(panel)
            .overflow_hidden()
            .on_mouse_move(cx.listener(|bu, olay: &MouseMoveEvent, _, cx| {
                bu.imleci_güncelle(olay.position);
                if olay.dragging()
                    && let Some((baş, _)) = bu.seçim
                    && let Some(konum) = bu.sahne_konumu(olay.position)
                {
                    bu.seçim = Some((baş, konum.x.clamp(64.0, 776.0)));
                }
                cx.notify();
            }))
            .on_scroll_wheel(cx.listener(|bu, olay: &ScrollWheelEvent, _, cx| {
                bu.tekerlek_yakınlaştır(olay);
                cx.notify();
            }))
            .on_mouse_exit(cx.listener(|bu, _: &MouseExitEvent, _, cx| {
                if bu.seçim.is_none() {
                    bu.imleç = None;
                    cx.notify();
                }
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|bu, olay: &MouseDownEvent, _, cx| {
                    if olay.click_count >= 2 && bu.etkileşimler.çift_tıkla_tam_görünüm {
                        bu.görünümü_uygula(None);
                        bu.seçim = None;
                    } else if bu.etkileşimler.seçim_yakınlaştır
                        && let Some(konum) = bu.sahne_konumu(olay.position)
                        && (64.0..=776.0).contains(&konum.x)
                        && (48.0..=352.0).contains(&konum.y)
                    {
                        bu.seçim = Some((konum.x, konum.x));
                    }
                    cx.notify();
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|bu, _: &MouseUpEvent, _, cx| {
                    if let Some((baş, son)) = bu.seçim.take()
                        && (son - baş).abs() >= 4.0
                    {
                        let eski = bu.geçerli_x_aralığı();
                        let en_az = ters_ölçekle(baş.min(son), eski, 64.0, 712.0);
                        let en_çok = ters_ölçekle(baş.max(son), eski, 64.0, 712.0);
                        match Aralık::yeni(en_az, en_çok) {
                            Ok(aralık) => {
                                bu.görünümü_uygula(Some(aralık));
                            }
                            Err(hata) => {
                                bu.hata = Some(format!("Seçilen aralık uygulanamadı: {hata}"));
                            }
                        }
                    }
                    cx.notify();
                }),
            )
            .child(
                canvas(
                    move |sınırlar, _, _| çizim_sınırları.set(Some(sınırlar)),
                    move |sınırlar, _, pencere, uygulama| {
                        if let Some(sahne) = &sahne {
                            sahneyi_boya(sahne, sınırlar, pencere, uygulama);
                        }
                    },
                )
                .size_full(),
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
                    .child("uPlot.rs Charts"),
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
                    .cursor_pointer()
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(metin)
                            .child("İlk kart · sin(x)"),
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
                            .child("Fixture bağlı · SVG/WASM hazır"),
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
                        bu.nokta_sayısı = bu.nokta_sayısı.saturating_sub(10).max(10);
                        bu.x_aralığı = None;
                        bu.görünüm_geçmişi.clear();
                        bu.imleç = None;
                        cx.notify();
                    })),
            )
            .child(
                Dugme::yeni("nokta-artir", "＋ Nokta")
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(DugmeTuru::Ikincil)
                    .tiklaninca(cx.listener(|bu, _, _, cx| {
                        bu.nokta_sayısı = bu.nokta_sayısı.saturating_add(10).min(10_000);
                        bu.x_aralığı = None;
                        bu.görünüm_geçmişi.clear();
                        bu.imleç = None;
                        cx.notify();
                    })),
            )
            .child(
                Dugme::yeni("gorunum-geri", "↶ Geri")
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(DugmeTuru::Hayalet)
                    .devre_disi(!geri_var || !self.etkileşimler.görünüm_geçmişi)
                    .tiklaninca(cx.listener(|bu, _, _, cx| {
                        bu.önceki_görünüme_dön();
                        cx.notify();
                    })),
            )
            .child(
                Dugme::yeni("tam-gorunum", "Tam görünüm")
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(DugmeTuru::Hayalet)
                    .devre_disi(!yakınlaştırılmış || !self.etkileşimler.çift_tıkla_tam_görünüm)
                    .tiklaninca(cx.listener(|bu, _, _, cx| {
                        bu.görünümü_uygula(None);
                        cx.notify();
                    })),
            )
            .child(
                Dugme::yeni("grafik-sifirla", "Sıfırla")
                    .boyutu(DugmeBoyutu::Kucuk)
                    .turu(DugmeTuru::Hayalet)
                    .tiklaninca(cx.listener(|bu, _, _, cx| {
                        bu.nokta_sayısı = 100;
                        bu.x_aralığı = None;
                        bu.görünüm_geçmişi.clear();
                        bu.imleç = None;
                        cx.notify();
                    })),
            );

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
                            .child(
                                "Çekirdek: seçim + çift tık · İsteğe bağlı resmi eklenti portu: tekerlek · uPlot.rs: geri geçmişi",
                            ),
                    ),
            )
            .child(araçlar)
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

        PlatformPencere::yeni("uplot-rs-pencere", "uPlot.rs Charts", içerik)
            .ayarlar(CubukAyarlari::default().kompakt(true))
            .sag(
                div()
                    .text_xs()
                    .text_color(soluk)
                    .child("Rust 2024 · MSRV 1.95"),
            )
    }
}

fn sahneyi_boya(
    sahne: &Sahne,
    sınırlar: gpui::Bounds<gpui::Pixels>,
    pencere: &mut Window,
    uygulama: &mut App,
) {
    let (kaynak_g, kaynak_y) = sahne.boyut();
    let ölçek = (f32::from(sınırlar.size.width) / kaynak_g as f32)
        .min(f32::from(sınırlar.size.height) / kaynak_y as f32)
        .max(0.01);
    let içerik_g = kaynak_g as f32 * ölçek;
    let içerik_y = kaynak_y as f32 * ölçek;
    let köken_x = f32::from(sınırlar.origin.x) + (f32::from(sınırlar.size.width) - içerik_g) / 2.0;
    let köken_y = f32::from(sınırlar.origin.y) + (f32::from(sınırlar.size.height) - içerik_y) / 2.0;
    let dönüştür =
        |nokta: crate::Nokta| point(px(köken_x + nokta.x * ölçek), px(köken_y + nokta.y * ölçek));

    for komut in sahne.komutlar() {
        match komut {
            Komut::ArkaPlan { .. } => {}
            Komut::Çizgi {
                başlangıç,
                bitiş,
                renk,
                kalınlık,
            } => {
                let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek));
                yol.move_to(dönüştür(*başlangıç));
                yol.line_to(dönüştür(*bitiş));
                if let Ok(yol) = yol.build() {
                    pencere.paint_path(yol, renk_çöz(renk));
                }
            }
            Komut::KesikliÇizgi {
                başlangıç,
                bitiş,
                renk,
                kalınlık,
                kesik,
            } => {
                let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek))
                    .dash_array(&[px(*kesik * ölçek), px(*kesik * ölçek)]);
                yol.move_to(dönüştür(*başlangıç));
                yol.line_to(dönüştür(*bitiş));
                if let Ok(yol) = yol.build() {
                    pencere.paint_path(yol, renk_çöz(renk));
                }
            }
            Komut::Yol {
                parçalar,
                renk,
                kalınlık,
            } => {
                let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek));
                for parça in parçalar {
                    let mut noktalar = parça.iter();
                    if let Some(ilk) = noktalar.next() {
                        yol.move_to(dönüştür(*ilk));
                    }
                    for nokta in noktalar {
                        yol.line_to(dönüştür(*nokta));
                    }
                }
                if let Ok(yol) = yol.build() {
                    pencere.paint_path(yol, renk_çöz(renk));
                }
            }
            Komut::Daire {
                merkez,
                yarıçap,
                dolgu,
                çizgi,
                kalınlık,
            } => {
                let merkez = dönüştür(*merkez);
                let yarıçap = px(*yarıçap * ölçek);
                let sınırlar = Bounds::new(
                    point(merkez.x - yarıçap, merkez.y - yarıçap),
                    size(yarıçap * 2.0, yarıçap * 2.0),
                );
                pencere.paint_quad(quad(
                    sınırlar,
                    yarıçap,
                    renk_çöz(dolgu),
                    px(*kalınlık * ölçek),
                    renk_çöz(çizgi),
                    BorderStyle::default(),
                ));
            }
            Komut::Dikdörtgen {
                konum,
                genişlik,
                yükseklik,
                dolgu,
                çizgi,
                kalınlık,
            } => {
                let konum = dönüştür(*konum);
                pencere.paint_quad(quad(
                    Bounds::new(konum, size(px(*genişlik * ölçek), px(*yükseklik * ölçek))),
                    px(0.0),
                    renk_çöz(dolgu),
                    px(*kalınlık * ölçek),
                    renk_çöz(çizgi),
                    BorderStyle::default(),
                ));
            }
            Komut::Metin {
                konum,
                içerik,
                renk,
                boyut,
                hiza,
            } => {
                let paylaşımlı = SharedString::from(içerik.clone());
                let koşu = TextRun {
                    len: paylaşımlı.len(),
                    font: pencere.text_style().font(),
                    color: renk_çöz(renk),
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };
                let çizgi =
                    pencere
                        .text_system()
                        .shape_line(paylaşımlı, px(*boyut * ölçek), &[koşu], None);
                let genişlik = f32::from(çizgi.width());
                let x = match hiza {
                    MetinHizası::Başlangıç => konum.x * ölçek,
                    MetinHizası::Orta => konum.x * ölçek - genişlik / 2.0,
                    MetinHizası::Bitiş => konum.x * ölçek - genişlik,
                };
                let başlangıç = point(px(köken_x + x), px(köken_y + (konum.y - *boyut) * ölçek));
                let _ = çizgi.paint(
                    başlangıç,
                    px(*boyut * 1.25 * ölçek),
                    TextAlign::Left,
                    None,
                    pencere,
                    uygulama,
                );
            }
        }
    }
}

fn renk_çöz(kod: &str) -> Hsla {
    if kod == "#3b82f633" {
        return rgba(0x3b82f633).into();
    }
    let sayı = kod
        .strip_prefix('#')
        .and_then(|ham| u32::from_str_radix(ham, 16).ok())
        .unwrap_or(0x000000);
    rgb(sayı).into()
}

fn tam_x_aralığı(nokta_sayısı: usize) -> Aralık {
    let nokta_sayısı = nokta_sayısı.max(2);
    Aralık {
        en_az: 0.0,
        en_çok: 2.0 * PI * nokta_sayısı.saturating_sub(1) as f64 / nokta_sayısı as f64,
    }
}

fn görünür_y_aralığı(nokta_sayısı: usize, x_aralığı: Aralık) -> Aralık {
    let nokta_sayısı = nokta_sayısı.max(2);
    let adım = 2.0 * PI / nokta_sayısı as f64;
    let mut en_az = f64::INFINITY;
    let mut en_çok = f64::NEG_INFINITY;
    for indeks in 0..nokta_sayısı {
        let x = indeks as f64 * adım;
        if x >= x_aralığı.en_az && x <= x_aralığı.en_çok {
            let y = x.sin();
            en_az = en_az.min(y);
            en_çok = en_çok.max(y);
        }
    }
    if en_az == en_çok {
        let pay = en_az.abs().max(1.0) * 0.1;
        return Aralık {
            en_az: en_az - pay,
            en_çok: en_çok + pay,
        };
    }
    let pay = (en_çok - en_az) * 0.1;
    Aralık {
        en_az: en_az - pay,
        en_çok: en_çok + pay,
    }
}

fn ölçekle(değer: f64, aralık: Aralık, başlangıç: f32, uzunluk: f32) -> f32 {
    başlangıç + ((değer - aralık.en_az) / (aralık.en_çok - aralık.en_az)) as f32 * uzunluk
}

fn ters_ölçekle(konum: f32, aralık: Aralık, başlangıç: f32, uzunluk: f32) -> f64 {
    aralık.en_az + f64::from((konum - başlangıç) / uzunluk) * (aralık.en_çok - aralık.en_az)
}
