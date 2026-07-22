//! GPUI çizim yüzeyi ve etkileşim adaptörü.

use std::cell::Cell;
use std::rc::Rc;

use ::gpui::{
    App, BorderStyle, Bounds, Context, EventEmitter, FocusHandle, Hsla, IntoElement, KeyDownEvent,
    KeyUpEvent, MouseButton, MouseDownEvent, MouseExitEvent, MouseMoveEvent, MouseUpEvent,
    PathBuilder, PinchEvent, Pixels, Render, ScrollDelta, ScrollWheelEvent, SharedString,
    TextAlign, TextRun, TouchPhase, Window, canvas, div, point, prelude::*, px, quad, rgb, rgba,
    size,
};

use crate::{Aralık, Grafik, Komut, MetinHizası, Nokta, Sahne};

#[derive(Clone)]
struct İmleçDurumu {
    fare: Nokta,
    veri_x: f64,
    seri_değerleri: Vec<Option<f64>>,
}

#[derive(Clone, Copy, Debug)]
pub enum GpuiGrafikOlayı {
    DurumDeğişti,
}

/// Çekirdek [`Grafik`] durumunu GPUI canvas üzerinde gösteren hazır bileşen.
///
/// Bileşen platform olaylarını çekirdeğe iletir; yakınlaştırma, seçim, geçmiş
/// ve tekerlek normalizasyonunu uygulama kodunun tekrar etmesi gerekmez.
pub struct GpuiGrafik {
    grafik: Grafik,
    imleç: Option<İmleçDurumu>,
    seçim: Option<(f32, f32)>,
    taşıma_başlangıcı: Option<Nokta>,
    dokunma_kaydırma: Option<(f64, f64)>,
    boşluk_basılı: bool,
    hata: Option<String>,
    çizim_sınırları: Rc<Cell<Option<Bounds<Pixels>>>>,
    odak: Option<FocusHandle>,
}

impl GpuiGrafik {
    pub fn yeni(grafik: Grafik) -> Self {
        Self {
            grafik,
            imleç: None,
            seçim: None,
            taşıma_başlangıcı: None,
            dokunma_kaydırma: None,
            boşluk_basılı: false,
            hata: None,
            çizim_sınırları: Rc::new(Cell::new(None)),
            odak: None,
        }
    }

    pub fn grafik(&self) -> &Grafik {
        &self.grafik
    }

    pub fn hata(&self) -> Option<&str> {
        self.hata.as_deref()
    }

    pub fn lejant(&self) -> Option<(f64, f64)> {
        self.imleç.as_ref().and_then(|imleç| {
            imleç
                .seri_değerleri
                .first()
                .copied()
                .flatten()
                .map(|y| (imleç.veri_x, y))
        })
    }

    pub fn lejant_değerleri(&self) -> Option<(f64, Vec<Option<f64>>)> {
        self.imleç
            .as_ref()
            .map(|imleç| (imleç.veri_x, imleç.seri_değerleri.clone()))
    }

    pub fn grafiği_ayarla(&mut self, grafik: Grafik, cx: &mut Context<Self>) {
        self.grafik = grafik;
        self.imleç = None;
        self.seçim = None;
        self.taşıma_başlangıcı = None;
        self.dokunma_kaydırma = None;
        self.boşluk_basılı = false;
        self.hata = None;
        Self::bildir(cx);
    }

    pub fn tekerlek_etkileşimi_ayarla(&mut self, etkin: bool, cx: &mut Context<Self>) {
        self.grafik.tekerlek_etkileşimi_ayarla(etkin);
        Self::bildir(cx);
    }

    pub fn önceki_görünüm(&mut self, cx: &mut Context<Self>) -> bool {
        let değişti = self.grafik.önceki_görünüm();
        if değişti {
            Self::bildir(cx);
        }
        değişti
    }

    pub fn tam_görünüm(&mut self, cx: &mut Context<Self>) -> bool {
        let değişti = self.grafik.tam_görünüm();
        if değişti {
            Self::bildir(cx);
        }
        değişti
    }

    fn çizim_alanı(&self) -> (f32, f32, f32, f32) {
        let (genişlik, yükseklik) = self.grafik.boyut();
        self.grafik.çizim_alanı_boyutta(genişlik, yükseklik)
    }

    fn sahne(&self) -> Sahne {
        let mut sahne = self.grafik.çiz();
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let x_aralığı = self.grafik.görünür_x_aralığı();
        if let Some(imleç) = self.imleç.as_ref() {
            let nokta_x = ölçekle(imleç.veri_x, x_aralığı, sol, sağ - sol);
            sahne.ekle(Komut::KesikliÇizgi {
                başlangıç: Nokta::yeni(imleç.fare.x, üst),
                bitiş: Nokta::yeni(imleç.fare.x, alt),
                renk: "#6b7280".to_string(),
                kalınlık: 1.0,
                kesik: 4.0,
            });
            sahne.ekle(Komut::KesikliÇizgi {
                başlangıç: Nokta::yeni(sol, imleç.fare.y),
                bitiş: Nokta::yeni(sağ, imleç.fare.y),
                renk: "#6b7280".to_string(),
                kalınlık: 1.0,
                kesik: 4.0,
            });
            for (seri_indeksi, değer) in imleç.seri_değerleri.iter().enumerate() {
                let Some(değer) = değer else {
                    continue;
                };
                let Some(seri) = self.grafik.seri_seçenekleri().get(seri_indeksi) else {
                    continue;
                };
                let Some(y_aralığı) = self.grafik.seri_görünür_y_aralığı(seri_indeksi)
                else {
                    continue;
                };
                let nokta_y = alt - ölçekle(*değer, y_aralığı, 0.0, alt - üst);
                sahne.ekle(Komut::Daire {
                    merkez: Nokta::yeni(nokta_x, nokta_y),
                    yarıçap: 2.5,
                    dolgu: seri.renk.clone(),
                    çizgi: seri.renk.clone(),
                    kalınlık: 0.0,
                });
            }
        }
        if let Some((başlangıç, bitiş)) = self.seçim {
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(başlangıç.min(bitiş), üst),
                genişlik: (bitiş - başlangıç).abs(),
                yükseklik: alt - üst,
                dolgu: "#3b82f633".to_string(),
                çizgi: "#3b82f6".to_string(),
                kalınlık: 1.0,
            });
        }
        sahne
    }

    fn sahne_konumu(&self, pencere_konumu: ::gpui::Point<Pixels>) -> Option<Nokta> {
        let sınırlar = self.çizim_sınırları.get()?;
        let (kaynak_g, kaynak_y) = self.grafik.boyut();
        let ölçek = (f32::from(sınırlar.size.width) / kaynak_g as f32)
            .min(f32::from(sınırlar.size.height) / kaynak_y as f32)
            .max(0.01);
        let köken_x = f32::from(sınırlar.origin.x)
            + (f32::from(sınırlar.size.width) - kaynak_g as f32 * ölçek) / 2.0;
        let köken_y = f32::from(sınırlar.origin.y)
            + (f32::from(sınırlar.size.height) - kaynak_y as f32 * ölçek) / 2.0;
        Some(Nokta::yeni(
            (f32::from(pencere_konumu.x) - köken_x) / ölçek,
            (f32::from(pencere_konumu.y) - köken_y) / ölçek,
        ))
    }

    fn grafik_alanında(&self, nokta: Nokta) -> bool {
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        (sol..=sağ).contains(&nokta.x) && (üst..=alt).contains(&nokta.y)
    }

    fn imleci_güncelle(&mut self, pencere_konumu: ::gpui::Point<Pixels>) {
        let Some(fare) = self.sahne_konumu(pencere_konumu) else {
            self.imleç = None;
            return;
        };
        if !self.grafik_alanında(fare) {
            self.imleç = None;
            return;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let yatay = f64::from((fare.x - sol) / (sağ - sol));
        let dikey = f64::from((fare.y - üst) / (alt - üst));
        let Some((yatay, dikey)) = self.grafik.imleç_oranlarını_uyarla(
            yatay,
            dikey,
            f64::from(sağ - sol),
            f64::from(alt - üst),
        ) else {
            self.imleç = None;
            return;
        };
        let Some((veri_x, seri_değerleri)) = self.grafik.en_yakın_noktalar(yatay) else {
            self.imleç = None;
            return;
        };
        self.imleç = Some(İmleçDurumu {
            fare: Nokta::yeni(
                sol + (yatay as f32) * (sağ - sol),
                üst + (dikey as f32) * (alt - üst),
            ),
            veri_x,
            seri_değerleri,
        });
    }

    fn tekerlek_yakınlaştır(&mut self, olay: &ScrollWheelEvent) {
        let Some(fare) = self.sahne_konumu(olay.position) else {
            return;
        };
        if !self.grafik_alanında(fare) {
            return;
        }
        if cfg!(target_os = "windows") && self.grafik.etkileşim_seçenekleri().dokunma_etkileşimi
        {
            match olay.touch_phase {
                TouchPhase::Started => {
                    let _ = self.grafik.taşımayı_başlat();
                    self.dokunma_kaydırma = Some((0.0_f64, 0.0_f64));
                    return;
                }
                TouchPhase::Ended | TouchPhase::Cancelled if self.dokunma_kaydırma.is_some() => {
                    self.dokunma_kaydırma = None;
                    self.grafik.taşımayı_bitir();
                    return;
                }
                TouchPhase::Moved => {}
                _ => return,
            }
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        if let Some((birikmiş_x, birikmiş_y)) = self.dokunma_kaydırma.as_mut() {
            let (x, y) = match olay.delta {
                ScrollDelta::Pixels(delta) => {
                    (f64::from(f32::from(delta.x)), f64::from(f32::from(delta.y)))
                }
                ScrollDelta::Lines(delta) => (f64::from(delta.x * 16.0), f64::from(delta.y * 16.0)),
            };
            *birikmiş_x += x / f64::from(sağ - sol);
            *birikmiş_y += y / f64::from(alt - üst);
            match self.grafik.taşı(*birikmiş_x, *birikmiş_y) {
                Ok(_) => self.hata = None,
                Err(hata) => self.hata = Some(format!("Dokunma taşıması uygulanamadı: {hata}")),
            }
            return;
        }
        let (delta, hassas) = match olay.delta {
            ScrollDelta::Pixels(delta) => (f64::from(f32::from(delta.y)), true),
            ScrollDelta::Lines(delta) => (f64::from(delta.y), false),
        };
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let yatay = f64::from((fare.x - sol) / (sağ - sol));
        let dikey = f64::from((fare.y - üst) / (alt - üst));
        match self.grafik.tekerlek(yatay, dikey, delta, hassas) {
            Ok(_) => self.hata = None,
            Err(hata) => {
                self.hata = Some(format!("Tekerlek yakınlaştırması uygulanamadı: {hata}"));
            }
        }
    }

    fn dokunma_yakınlaştır(&mut self, olay: &PinchEvent) {
        if matches!(olay.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
            self.grafik.dokunmayı_bitir();
            return;
        }
        if olay.phase == TouchPhase::Started && !self.grafik.dokunmayı_başlat() {
            return;
        }
        let Some(fare) = self.sahne_konumu(olay.position) else {
            return;
        };
        if !self.grafik_alanında(fare) {
            return;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let yatay = f64::from((fare.x - sol) / (sağ - sol));
        let dikey = f64::from((fare.y - üst) / (alt - üst));
        let çarpan = f64::from((1.0 + olay.delta).max(0.01));
        match self.grafik.dokunma_yakınlaştır(yatay, dikey, çarpan) {
            Ok(_) => self.hata = None,
            Err(hata) => self.hata = Some(format!("Dokunma yakınlaştırması uygulanamadı: {hata}")),
        }
    }

    fn bildir(cx: &mut Context<Self>) {
        cx.emit(GpuiGrafikOlayı::DurumDeğişti);
        cx.notify();
    }
}

impl EventEmitter<GpuiGrafikOlayı> for GpuiGrafik {}

impl Render for GpuiGrafik {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let odak = self
            .odak
            .get_or_insert_with(|| cx.focus_handle().tab_stop(true))
            .clone();
        let sahne = self.sahne();
        let çizim_sınırları = self.çizim_sınırları.clone();
        let taşıyor = self.taşıma_başlangıcı.is_some();
        let taşımaya_hazır = self.boşluk_basılı && self.grafik.yakınlaştırılmış();
        div()
            .id("uplot-rs-gpui-grafik")
            .track_focus(&odak)
            .size_full()
            .min_h(px(120.0))
            .overflow_hidden()
            .when(taşıyor, |yüzey| yüzey.cursor_grabbing())
            .when(!taşıyor && taşımaya_hazır, |yüzey| yüzey.cursor_grab())
            .on_key_down(cx.listener(|bu, olay: &KeyDownEvent, _, cx| {
                if olay.keystroke.key.as_str() == "space" {
                    bu.boşluk_basılı = true;
                    bu.seçim = None;
                    cx.stop_propagation();
                    GpuiGrafik::bildir(cx);
                }
            }))
            .on_key_up(cx.listener(|bu, olay: &KeyUpEvent, _, cx| {
                if olay.keystroke.key.as_str() == "space" {
                    bu.boşluk_basılı = false;
                    bu.taşıma_başlangıcı = None;
                    bu.grafik.taşımayı_bitir();
                    cx.stop_propagation();
                    GpuiGrafik::bildir(cx);
                }
            }))
            .on_mouse_move(cx.listener(|bu, olay: &MouseMoveEvent, window, cx| {
                if let Some(odak) = bu.odak.as_ref()
                    && !odak.is_focused(window)
                    && bu
                        .sahne_konumu(olay.position)
                        .is_some_and(|konum| bu.grafik_alanında(konum))
                {
                    odak.focus(window, cx);
                }
                if let Some(başlangıç) = bu.taşıma_başlangıcı
                    && let Some(konum) = bu.sahne_konumu(olay.position)
                {
                    let (sol, sağ, üst, alt) = bu.çizim_alanı();
                    let yatay = f64::from((konum.x - başlangıç.x) / (sağ - sol));
                    let dikey = f64::from((konum.y - başlangıç.y) / (alt - üst));
                    match bu.grafik.taşı(yatay, dikey) {
                        Ok(_) => bu.hata = None,
                        Err(hata) => {
                            bu.hata = Some(format!("Grafik görünümü taşınamadı: {hata}"));
                        }
                    }
                    bu.imleç = None;
                } else {
                    bu.imleci_güncelle(olay.position);
                }
                if bu.taşıma_başlangıcı.is_none()
                    && olay.dragging()
                    && let Some((başlangıç, _)) = bu.seçim
                    && let Some(konum) = bu.sahne_konumu(olay.position)
                {
                    let (sol, sağ, _, _) = bu.çizim_alanı();
                    bu.seçim = Some((başlangıç, konum.x.clamp(sol, sağ)));
                }
                GpuiGrafik::bildir(cx);
            }))
            .on_scroll_wheel(cx.listener(|bu, olay: &ScrollWheelEvent, _, cx| {
                bu.tekerlek_yakınlaştır(olay);
                GpuiGrafik::bildir(cx);
            }))
            .on_pinch(cx.listener(|bu, olay: &PinchEvent, _, cx| {
                bu.dokunma_yakınlaştır(olay);
                GpuiGrafik::bildir(cx);
            }))
            .on_mouse_exit(cx.listener(|bu, _: &MouseExitEvent, _, cx| {
                if bu.seçim.is_none() && bu.taşıma_başlangıcı.is_none() {
                    bu.imleç = None;
                    GpuiGrafik::bildir(cx);
                }
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|bu, olay: &MouseDownEvent, window, cx| {
                    if let Some(odak) = bu.odak.as_ref() {
                        odak.focus(window, cx);
                    }
                    let ayarlar = bu.grafik.etkileşim_seçenekleri();
                    if bu.boşluk_basılı
                        && let Some(konum) = bu.sahne_konumu(olay.position)
                        && bu.grafik_alanında(konum)
                        && bu.grafik.taşımayı_başlat()
                    {
                        bu.taşıma_başlangıcı = Some(konum);
                        bu.seçim = None;
                        bu.imleç = None;
                    } else if olay.click_count >= 2 && ayarlar.çift_tıkla_tam_görünüm {
                        bu.grafik.tam_görünüm();
                        bu.seçim = None;
                    } else if ayarlar.seçim_yakınlaştır
                        && let Some(konum) = bu.sahne_konumu(olay.position)
                        && bu.grafik_alanında(konum)
                    {
                        bu.seçim = Some((konum.x, konum.x));
                    }
                    GpuiGrafik::bildir(cx);
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|bu, _: &MouseUpEvent, _, cx| {
                    if bu.taşıma_başlangıcı.take().is_some() {
                        bu.grafik.taşımayı_bitir();
                        GpuiGrafik::bildir(cx);
                        return;
                    }
                    if let Some((başlangıç, bitiş)) = bu.seçim.take()
                        && (bitiş - başlangıç).abs() >= 4.0
                    {
                        let (sol, sağ, _, _) = bu.çizim_alanı();
                        let başlangıç_oranı = f64::from((başlangıç - sol) / (sağ - sol));
                        let bitiş_oranı = f64::from((bitiş - sol) / (sağ - sol));
                        match bu.grafik.seçim_yakınlaştır(başlangıç_oranı, bitiş_oranı) {
                            Ok(_) => bu.hata = None,
                            Err(hata) => {
                                bu.hata = Some(format!("Seçilen aralık uygulanamadı: {hata}"));
                            }
                        }
                    }
                    GpuiGrafik::bildir(cx);
                }),
            )
            .child(
                canvas(
                    move |sınırlar, _, _| çizim_sınırları.set(Some(sınırlar)),
                    move |sınırlar, _, pencere, uygulama| {
                        sahneyi_boya(&sahne, sınırlar, pencere, uygulama);
                    },
                )
                .size_full(),
            )
    }
}

/// Ortak sahne komutlarını GPUI canvas üzerine boyar.
pub fn sahneyi_boya(
    sahne: &Sahne,
    sınırlar: Bounds<Pixels>,
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
        |nokta: Nokta| point(px(köken_x + nokta.x * ölçek), px(köken_y + nokta.y * ölçek));

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
            Komut::Alan { çokgenler, dolgu } => {
                let mut yol = PathBuilder::fill();
                for çokgen in çokgenler {
                    let mut noktalar = çokgen.iter();
                    if let Some(ilk) = noktalar.next() {
                        yol.move_to(dönüştür(*ilk));
                    }
                    for nokta in noktalar {
                        yol.line_to(dönüştür(*nokta));
                    }
                    if çokgen.len() >= 3 {
                        yol.close();
                    }
                }
                if let Ok(yol) = yol.build() {
                    pencere.paint_path(yol, renk_çöz(dolgu));
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
                let daire_sınırları = Bounds::new(
                    point(merkez.x - yarıçap, merkez.y - yarıçap),
                    size(yarıçap * 2.0, yarıçap * 2.0),
                );
                pencere.paint_quad(quad(
                    daire_sınırları,
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
    let Some(ham) = kod.strip_prefix('#') else {
        return rgb(0x000000).into();
    };
    match ham.len() {
        8 => u32::from_str_radix(ham, 16)
            .map_or_else(|_| rgb(0x000000).into(), |sayı| rgba(sayı).into()),
        6 => u32::from_str_radix(ham, 16)
            .map_or_else(|_| rgb(0x000000).into(), |sayı| rgb(sayı).into()),
        _ => rgb(0x000000).into(),
    }
}

fn ölçekle(değer: f64, aralık: Aralık, başlangıç: f32, uzunluk: f32) -> f32 {
    başlangıç + ((değer - aralık.en_az) / (aralık.en_çok - aralık.en_az)) as f32 * uzunluk
}
