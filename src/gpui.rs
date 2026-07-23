//! GPUI çizim yüzeyi ve etkileşim adaptörü.

use std::cell::Cell;
use std::rc::Rc;

use ::gpui::{
    App, BorderStyle, Bounds, ContentMask, Context, EventEmitter, FocusHandle, Hsla, IntoElement,
    KeyDownEvent, KeyUpEvent, MouseButton, MouseDownEvent, MouseExitEvent, MouseMoveEvent,
    MouseUpEvent, PathBuilder, PinchEvent, Pixels, Render, ScrollDelta, ScrollWheelEvent,
    SharedString, TextAlign, TextRun, TouchPhase, Window, canvas, div, linear_color_stop,
    linear_gradient, point, prelude::*, px, quad, rgb, rgba, size,
};

use crate::{
    DağılımVuruşu, DoğrusalGradyan, Grafik, HizalıVeri, Komut, MetinHizası, Nokta, Sahne,
    SeriSeçenekleri, SeçimEylemi, UplotHatası, YüzeyDikdörtgeni,
};

#[derive(Clone)]
struct İmleçDurumu {
    fare: Nokta,
    veri_x: f64,
    seri_değerleri: Vec<Option<f64>>,
    dağılım: Option<DağılımVuruşu>,
}

#[derive(Clone, Copy, Debug)]
pub enum GpuiGrafikOlayı {
    DurumDeğişti,
    /// `cursor-bind` Ctrl seçimi tamamlandı; üst uygulama metin UI'si açabilir.
    Açıklamaİstendi,
}

/// Çekirdek [`Grafik`] durumunu GPUI canvas üzerinde gösteren hazır bileşen.
///
/// Bileşen platform olaylarını çekirdeğe iletir; yakınlaştırma, seçim, geçmiş
/// ve tekerlek normalizasyonunu uygulama kodunun tekrar etmesi gerekmez.
pub struct GpuiGrafik {
    grafik: Grafik,
    imleç: Option<İmleçDurumu>,
    seçim: Option<(f32, f32)>,
    açıklama_seçimi: bool,
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
            açıklama_seçimi: false,
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
        self.açıklama_seçimi = false;
        self.taşıma_başlangıcı = None;
        self.dokunma_kaydırma = None;
        self.boşluk_basılı = false;
        self.hata = None;
        Self::bildir(cx);
    }

    pub fn veriyi_ayarla(
        &mut self,
        veri: HizalıVeri,
        cx: &mut Context<Self>,
    ) -> Result<(), UplotHatası> {
        self.grafik.veriyi_ayarla(veri)?;
        self.imleç = None;
        self.seçim = None;
        Self::bildir(cx);
        Ok(())
    }

    pub fn seri_ekle(
        &mut self,
        indeks: usize,
        seçenek: SeriSeçenekleri,
        değerler: Vec<Option<f64>>,
        cx: &mut Context<Self>,
    ) -> Result<(), UplotHatası> {
        self.grafik.seri_ekle(indeks, seçenek, değerler)?;
        self.imleç = None;
        self.seçim = None;
        self.açıklama_seçimi = false;
        Self::bildir(cx);
        Ok(())
    }

    pub fn seri_sil(&mut self, indeks: usize, cx: &mut Context<Self>) -> Result<(), UplotHatası> {
        self.grafik.seri_sil(indeks)?;
        self.imleç = None;
        self.seçim = None;
        self.açıklama_seçimi = false;
        Self::bildir(cx);
        Ok(())
    }

    pub fn boşlukları_birleştir_ayarla(
        &mut self,
        birleştir: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.boşlukları_birleştir_ayarla(birleştir);
        if değişti {
            self.imleç = None;
            Self::bildir(cx);
        }
        değişti
    }

    pub fn tekerlek_etkileşimi_ayarla(&mut self, etkin: bool, cx: &mut Context<Self>) {
        self.grafik.tekerlek_etkileşimi_ayarla(etkin);
        Self::bildir(cx);
    }

    pub fn y_arcsinh_eşiği_ayarla(
        &mut self,
        anahtar: &str,
        eşik: f64,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.y_arcsinh_eşiği_ayarla(anahtar, eşik);
        if değişti {
            Self::bildir(cx);
        }
        değişti
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
        if let Some(imleç) = self.imleç.as_ref() {
            if let Some(vuruş) = &imleç.dağılım {
                sahne.ekle(Komut::Daire {
                    merkez: vuruş.merkez,
                    yarıçap: vuruş.boyut / 2.0,
                    dolgu: "#ffffff66".to_string(),
                    çizgi: "#111111".to_string(),
                    kalınlık: 2.0,
                });
                return sahne;
            }
            if let Some((_, _, konum, genişlik, yükseklik, _)) = self.grafik.çubuk_vuruşu(
                self.grafik.boyut().0,
                self.grafik.boyut().1,
                imleç.fare.x,
                imleç.fare.y,
            ) {
                sahne.ekle(Komut::Dikdörtgen {
                    konum,
                    genişlik,
                    yükseklik,
                    dolgu: "#ffffff4d".to_string(),
                    çizgi: "#ffffff00".to_string(),
                    kalınlık: 0.0,
                });
                return sahne;
            }
            if self.grafik.çubuk_grafiği() {
                return sahne;
            }
            if let Some((_, konum, genişlik, yükseklik, _)) = self.grafik.kutu_bıyık_vuruşu(
                self.grafik.boyut().0,
                self.grafik.boyut().1,
                imleç.fare.x,
                imleç.fare.y,
            ) {
                sahne.ekle(Komut::Dikdörtgen {
                    konum,
                    genişlik,
                    yükseklik,
                    dolgu: "#33ccff4d".to_string(),
                    çizgi: "#33ccff00".to_string(),
                    kalınlık: 0.0,
                });
                return sahne;
            }
            if self.grafik.kutu_bıyık_grafiği() || self.grafik.mum_grafiği() {
                return sahne;
            }
            let nokta_x = self
                .grafik
                .x_konum_oranı(imleç.veri_x)
                .map_or(imleç.fare.x, |oran| sol + oran as f32 * (sağ - sol));
            sahne.ekle(Komut::KesikliÇizgi {
                başlangıç: Nokta::yeni(imleç.fare.x, üst),
                bitiş: Nokta::yeni(imleç.fare.x, alt),
                renk: "#6b7280".to_string(),
                kalınlık: 1.0,
                kesik: 4.0,
            });
            if !self.grafik.eksen_göstergeleri_etkin() {
                sahne.ekle(Komut::KesikliÇizgi {
                    başlangıç: Nokta::yeni(sol, imleç.fare.y),
                    bitiş: Nokta::yeni(sağ, imleç.fare.y),
                    renk: "#6b7280".to_string(),
                    kalınlık: 1.0,
                    kesik: 4.0,
                });
            } else {
                sahne.ekle(Komut::Dikdörtgen {
                    konum: Nokta::yeni(nokta_x - 24.0, alt + 6.0),
                    genişlik: 48.0,
                    yükseklik: 22.0,
                    dolgu: "#111111".to_string(),
                    çizgi: "#111111".to_string(),
                    kalınlık: 0.0,
                });
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(nokta_x, alt + 21.0),
                    içerik: format!("{:.2}", imleç.veri_x),
                    renk: "#ffffff".to_string(),
                    boyut: 11.0,
                    hiza: MetinHizası::Orta,
                });
            }
            for (seri_indeksi, değer) in imleç.seri_değerleri.iter().enumerate() {
                let Some(değer) = değer else {
                    continue;
                };
                let Some(seri) = self.grafik.seri_seçenekleri().get(seri_indeksi) else {
                    continue;
                };
                let seri_rengi = self
                    .grafik
                    .seri_imleç_rengi(seri_indeksi, imleç.veri_x, *değer)
                    .unwrap_or_else(|| seri.renk.clone());
                let Some(y_oranı) = self.grafik.seri_y_konum_oranı(seri_indeksi, *değer) else {
                    continue;
                };
                let nokta_y = alt - y_oranı as f32 * (alt - üst);
                if self.grafik.eksen_göstergeleri_etkin() {
                    sahne.ekle(Komut::KesikliÇizgi {
                        başlangıç: Nokta::yeni(sol, nokta_y),
                        bitiş: Nokta::yeni(sağ, nokta_y),
                        renk: seri_rengi.clone(),
                        kalınlık: 1.0,
                        kesik: 4.0,
                    });
                    let rozet_x = sol - 50.0 - seri_indeksi as f32 * 56.0;
                    sahne.ekle(Komut::Dikdörtgen {
                        konum: Nokta::yeni(rozet_x, nokta_y - 11.0),
                        genişlik: 44.0,
                        yükseklik: 22.0,
                        dolgu: seri_rengi.clone(),
                        çizgi: seri_rengi.clone(),
                        kalınlık: 0.0,
                    });
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(rozet_x + 22.0, nokta_y + 4.0),
                        içerik: format!("{değer:.2}"),
                        renk: "#ffffff".to_string(),
                        boyut: 11.0,
                        hiza: MetinHizası::Orta,
                    });
                }
                sahne.ekle(Komut::Daire {
                    merkez: Nokta::yeni(nokta_x, nokta_y),
                    yarıçap: 2.5,
                    dolgu: seri_rengi.clone(),
                    çizgi: seri_rengi,
                    kalınlık: 0.0,
                });
            }
        }
        if let Some((başlangıç, bitiş)) = self.seçim {
            let (dolgu, çizgi) = if self.açıklama_seçimi {
                ("#ffff004d", "#d4a800")
            } else {
                ("#3b82f633", "#3b82f6")
            };
            let x_dikey = self.grafik.x_dikey_mi();
            sahne.ekle(Komut::Dikdörtgen {
                konum: if x_dikey {
                    Nokta::yeni(sol, başlangıç.min(bitiş))
                } else {
                    Nokta::yeni(başlangıç.min(bitiş), üst)
                },
                genişlik: if x_dikey {
                    sağ - sol
                } else {
                    (bitiş - başlangıç).abs()
                },
                yükseklik: if x_dikey {
                    (bitiş - başlangıç).abs()
                } else {
                    alt - üst
                },
                dolgu: dolgu.to_string(),
                çizgi: çizgi.to_string(),
                kalınlık: 1.0,
            });
        }
        sahne
    }

    fn sahne_konumu(&self, pencere_konumu: ::gpui::Point<Pixels>) -> Option<Nokta> {
        let sınırlar = self.çizim_sınırları.get()?;
        let (kaynak_g, kaynak_y) = self.grafik.boyut();
        YüzeyDikdörtgeni::yeni(
            f64::from(f32::from(sınırlar.origin.x)),
            f64::from(f32::from(sınırlar.origin.y)),
            f64::from(f32::from(sınırlar.size.width)),
            f64::from(f32::from(sınırlar.size.height)),
        )?
        .sahne_konumu(
            f64::from(f32::from(pencere_konumu.x)),
            f64::from(f32::from(pencere_konumu.y)),
            kaynak_g,
            kaynak_y,
        )
    }

    fn grafik_alanında(&self, nokta: Nokta) -> bool {
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        (sol..=sağ).contains(&nokta.x) && (üst..=alt).contains(&nokta.y)
    }

    fn imleci_güncelle(&mut self, pencere_konumu: ::gpui::Point<Pixels>) {
        let Some(fare) = self.sahne_konumu(pencere_konumu) else {
            self.imleç = None;
            self.grafik.imleç_odağını_temizle();
            return;
        };
        if !self.grafik_alanında(fare) {
            self.imleç = None;
            self.grafik.imleç_odağını_temizle();
            return;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        if let Some(vuruş) = self.grafik.dağılım_vuruşu_boyutta(
            self.grafik.boyut().0,
            self.grafik.boyut().1,
            fare.x,
            fare.y,
        ) {
            let mut değerler = vec![None; self.grafik.seri_seçenekleri().len()];
            if let Some(hedef) = değerler.get_mut(vuruş.seri) {
                *hedef = Some(vuruş.y);
            }
            self.imleç = Some(İmleçDurumu {
                fare,
                veri_x: vuruş.x,
                seri_değerleri: değerler,
                dağılım: Some(vuruş),
            });
            return;
        }
        let yatay = f64::from((fare.x - sol) / (sağ - sol));
        let dikey = f64::from((fare.y - üst) / (alt - üst));
        let Some((yatay, dikey)) = self.grafik.imleç_oranlarını_uyarla(
            yatay,
            dikey,
            f64::from(sağ - sol),
            f64::from(alt - üst),
        ) else {
            self.imleç = None;
            self.grafik.imleç_odağını_temizle();
            return;
        };
        let x_dikey = self.grafik.x_dikey_mi();
        self.grafik.imleç_odağını_güncelle(
            yatay,
            dikey,
            if x_dikey {
                f64::from(sağ - sol)
            } else {
                f64::from(alt - üst)
            },
        );
        let x_oranı = if x_dikey { 1.0 - dikey } else { yatay };
        let Some((veri_x, seri_değerleri)) = self.grafik.en_yakın_noktalar(x_oranı) else {
            self.imleç = None;
            self.grafik.imleç_odağını_temizle();
            return;
        };
        self.imleç = Some(İmleçDurumu {
            fare: Nokta::yeni(
                sol + (yatay as f32) * (sağ - sol),
                üst + (dikey as f32) * (alt - üst),
            ),
            veri_x,
            seri_değerleri,
            dağılım: None,
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
        let bilgi_kutusu = self
            .imleç
            .as_ref()
            .filter(|_| self.grafik.etkileşim_seçenekleri().imleç_bilgi_kutusu)
            .and_then(|imleç| {
                let y = imleç
                    .dağılım
                    .as_ref()
                    .map(|vuruş| vuruş.y)
                    .or_else(|| imleç.seri_değerleri.first().copied().flatten())?;
                let sınırlar = self.çizim_sınırları.get()?;
                let (kaynak_g, kaynak_y) = self.grafik.boyut();
                let ölçek = (f32::from(sınırlar.size.width) / kaynak_g as f32)
                    .min(f32::from(sınırlar.size.height) / kaynak_y as f32)
                    .max(0.01);
                let yatay_pay = (f32::from(sınırlar.size.width) - kaynak_g as f32 * ölçek) / 2.0;
                let dikey_pay = (f32::from(sınırlar.size.height) - kaynak_y as f32 * ölçek) / 2.0;
                let sol = (yatay_pay + imleç.fare.x * ölçek + 12.0)
                    .clamp(4.0, (f32::from(sınırlar.size.width) - 190.0).max(4.0));
                let üst = (dikey_pay + imleç.fare.y * ölçek + 12.0)
                    .clamp(4.0, (f32::from(sınırlar.size.height) - 42.0).max(4.0));
                Some((
                    sol,
                    üst,
                    imleç.dağılım.as_ref().map_or_else(
                        || {
                            format!(
                                "{},{y} at {},{}",
                                imleç.veri_x,
                                imleç.fare.x.round(),
                                imleç.fare.y.round()
                            )
                        },
                        |vuruş| {
                            format!(
                                "Country: {} · Population: {} · GDP: ${} · Income: ${}",
                                vuruş.etiket.as_deref().unwrap_or("--"),
                                vuruş
                                    .değer
                                    .map_or_else(|| "--".to_string(), |değer| değer.to_string()),
                                vuruş.x,
                                vuruş.y
                            )
                        },
                    ),
                ))
            });
        div()
            .id("uplot-rs-gpui-grafik")
            .relative()
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
                    bu.açıklama_seçimi = false;
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
                    let (sol, sağ, üst, alt) = bu.çizim_alanı();
                    let eksen_konumu = if bu.grafik.x_dikey_mi() {
                        konum.y.clamp(üst, alt)
                    } else {
                        konum.x.clamp(sol, sağ)
                    };
                    bu.seçim = Some((başlangıç, eksen_konumu));
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
                        bu.açıklama_seçimi = false;
                        bu.imleç = None;
                    } else if olay.click_count >= 2 && ayarlar.çift_tıkla_tam_görünüm {
                        bu.grafik.tam_görünüm();
                        bu.seçim = None;
                        bu.açıklama_seçimi = false;
                    } else if ayarlar.seçim_yakınlaştır
                        && let Some(konum) = bu.sahne_konumu(olay.position)
                        && bu.grafik_alanında(konum)
                    {
                        let eksen_konumu = if bu.grafik.x_dikey_mi() {
                            konum.y
                        } else {
                            konum.x
                        };
                        bu.seçim = Some((eksen_konumu, eksen_konumu));
                        bu.açıklama_seçimi = ayarlar.ctrl_açıklama && olay.modifiers.control;
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
                    let açıklama_seçimi = std::mem::take(&mut bu.açıklama_seçimi);
                    if let Some((başlangıç, bitiş)) = bu.seçim.take()
                        && (bitiş - başlangıç).abs() >= 4.0
                    {
                        let (sol, sağ, üst, alt) = bu.çizim_alanı();
                        let (başlangıç_oranı, bitiş_oranı) = if bu.grafik.x_dikey_mi() {
                            (
                                f64::from((alt - başlangıç) / (alt - üst)),
                                f64::from((alt - bitiş) / (alt - üst)),
                            )
                        } else {
                            (
                                f64::from((başlangıç - sol) / (sağ - sol)),
                                f64::from((bitiş - sol) / (sağ - sol)),
                            )
                        };
                        match bu
                            .grafik
                            .seçimi_bitir(başlangıç_oranı, bitiş_oranı, açıklama_seçimi)
                        {
                            Ok(SeçimEylemi::Açıklamaİstendi) => {
                                bu.hata = None;
                                cx.emit(GpuiGrafikOlayı::Açıklamaİstendi);
                            }
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
            .when_some(bilgi_kutusu, |yüzey, (sol, üst, metin)| {
                yüzey.child(
                    div()
                        .absolute()
                        .left(px(sol))
                        .top(px(üst))
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .bg(rgba(0x000000cc))
                        .text_color(rgb(0xffffff))
                        .text_xs()
                        .child(metin),
                )
            })
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
            Komut::GradyanYol {
                parçalar,
                gradyan,
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
                    gradyan_yolunu_boya(yol, gradyan, &dönüştür, pencere);
                }
            }
            Komut::KesikliYol {
                parçalar,
                renk,
                kalınlık,
                çizgi,
                boşluk,
            } => {
                let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek))
                    .dash_array(&[px(*çizgi * ölçek), px(*boşluk * ölçek)]);
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
            Komut::GradyanAlan {
                çokgenler, gradyan
            } => {
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
                    gradyan_yolunu_boya(yol, gradyan, &dönüştür, pencere);
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

fn gradyan_yolunu_boya(
    yol: ::gpui::Path<Pixels>,
    gradyan: &DoğrusalGradyan,
    dönüştür: &impl Fn(Nokta) -> ::gpui::Point<Pixels>,
    pencere: &mut Window,
) {
    let Some(ilk) = gradyan.duraklar.first() else {
        return;
    };
    if gradyan.duraklar.len() == 1 {
        pencere.paint_path(yol, renk_çöz(&ilk.renk));
        return;
    }
    let başlangıç = dönüştür(gradyan.başlangıç);
    let bitiş = dönüştür(gradyan.bitiş);
    let dx = f32::from(bitiş.x - başlangıç.x);
    let dy = f32::from(bitiş.y - başlangıç.y);
    let yatay = dx.abs() >= dy.abs();
    let eksen_başlangıcı = if yatay {
        f32::from(başlangıç.x)
    } else {
        f32::from(başlangıç.y)
    };
    let eksen_bitişi = if yatay {
        f32::from(bitiş.x)
    } else {
        f32::from(bitiş.y)
    };
    let eksen_farkı = eksen_bitişi - eksen_başlangıcı;
    if eksen_farkı.abs() <= f32::EPSILON {
        pencere.paint_path(yol, renk_çöz(&ilk.renk));
        return;
    }
    let sınır_başı = if yatay {
        f32::from(yol.bounds.left())
    } else {
        f32::from(yol.bounds.top())
    };
    let sınır_sonu = if yatay {
        f32::from(yol.bounds.right())
    } else {
        f32::from(yol.bounds.bottom())
    };
    let sınır_uzunluğu = (sınır_sonu - sınır_başı).max(f32::EPSILON);
    let açı = if yatay {
        if eksen_farkı >= 0.0 { 90.0 } else { 270.0 }
    } else if eksen_farkı >= 0.0 {
        180.0
    } else {
        0.0
    };

    let ilk_konum = eksen_başlangıcı + ilk.oran.clamp(0.0, 1.0) * eksen_farkı;
    boya_maskeli_aralık(
        &yol,
        yatay,
        if eksen_farkı >= 0.0 {
            sınır_başı
        } else {
            ilk_konum
        },
        if eksen_farkı >= 0.0 {
            ilk_konum
        } else {
            sınır_sonu
        },
        renk_çöz(&ilk.renk),
        pencere,
    );

    for çift in gradyan.duraklar.windows(2) {
        let (Some(sol), Some(sağ)) = (çift.first(), çift.get(1)) else {
            continue;
        };
        let sol_konum = eksen_başlangıcı + sol.oran.clamp(0.0, 1.0) * eksen_farkı;
        let sağ_konum = eksen_başlangıcı + sağ.oran.clamp(0.0, 1.0) * eksen_farkı;
        if (sağ_konum - sol_konum).abs() <= f32::EPSILON {
            continue;
        }
        let sol_yüzde = (sol_konum - sınır_başı) / sınır_uzunluğu;
        let sağ_yüzde = (sağ_konum - sınır_başı) / sınır_uzunluğu;
        let arka_plan = linear_gradient(
            açı,
            linear_color_stop(renk_çöz(&sol.renk), sol_yüzde),
            linear_color_stop(renk_çöz(&sağ.renk), sağ_yüzde),
        );
        boya_maskeli_aralık(
            &yol,
            yatay,
            sol_konum.min(sağ_konum),
            sol_konum.max(sağ_konum),
            arka_plan,
            pencere,
        );
    }

    if let Some(son) = gradyan.duraklar.last() {
        let son_konum = eksen_başlangıcı + son.oran.clamp(0.0, 1.0) * eksen_farkı;
        boya_maskeli_aralık(
            &yol,
            yatay,
            if eksen_farkı >= 0.0 {
                son_konum
            } else {
                sınır_başı
            },
            if eksen_farkı >= 0.0 {
                sınır_sonu
            } else {
                son_konum
            },
            renk_çöz(&son.renk),
            pencere,
        );
    }
}

fn boya_maskeli_aralık(
    yol: &::gpui::Path<Pixels>,
    yatay: bool,
    başlangıç: f32,
    bitiş: f32,
    boya: impl Into<::gpui::Background>,
    pencere: &mut Window,
) {
    let (başlangıç, bitiş) = (başlangıç.min(bitiş), başlangıç.max(bitiş));
    if bitiş - başlangıç <= f32::EPSILON {
        return;
    }
    let sınırlar = if yatay {
        Bounds::new(
            point(px(başlangıç), yol.bounds.top()),
            size(px(bitiş - başlangıç), yol.bounds.size.height),
        )
    } else {
        Bounds::new(
            point(yol.bounds.left(), px(başlangıç)),
            size(yol.bounds.size.width, px(bitiş - başlangıç)),
        )
    };
    let boya = boya.into();
    pencere.with_content_mask(Some(ContentMask { bounds: sınırlar }), |pencere| {
        pencere.paint_path(yol.clone(), boya);
    });
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
