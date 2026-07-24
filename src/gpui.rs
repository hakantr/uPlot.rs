//! GPUI çizim yüzeyi ve etkileşim adaptörü.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use ::gpui::{
    App, BorderStyle, Bounds, ContentMask, Context, Corners, Entity, EventEmitter, FocusHandle,
    Hsla, IntoElement, KeyDownEvent, KeyUpEvent, MouseButton, MouseDownEvent, MouseExitEvent,
    MouseMoveEvent, MouseUpEvent, Path, PathBuilder, PinchEvent, Pixels, Render, ScrollDelta,
    ScrollWheelEvent, SharedString, StyleRefinement, TextAlign, TextRun, TouchPhase, WeakEntity,
    Window, canvas, div, linear_color_stop, linear_gradient, point, prelude::*, px, quad, rgb,
    rgba, size,
};

use crate::{
    Aralık, AçıklamaVuruşu, DağılımVuruşu, DoğrusalGradyan, Grafik, HizalıVeri, Komut, MetinHizası,
    Nokta, Sahne, SeriSeçenekleri, SeçimEylemi, TekerlekEkseni, UplotHatası, YüzeyDikdörtgeni,
};

#[derive(Clone)]
struct İmleçDurumu {
    fare: Nokta,
    veri_x: f64,
    seri_x_değerleri: Vec<Option<f64>>,
    seri_değerleri: Vec<Option<f64>>,
    dağılım: Option<DağılımVuruşu>,
}

#[derive(Clone, Copy, Debug)]
pub enum GpuiGrafikOlayı {
    DurumDeğişti,
    İmleçDeğişti,
    FareBırakıldı,
    /// `cursor-bind` Ctrl seçimi tamamlandı; üst uygulama metin UI'si açabilir.
    Açıklamaİstendi,
}

/// Çekirdek [`Grafik`] durumunu GPUI canvas üzerinde gösteren hazır bileşen.
///
/// Bileşen platform olaylarını çekirdeğe iletir; yakınlaştırma, seçim, geçmiş
/// ve tekerlek normalizasyonunu uygulama kodunun tekrar etmesi gerekmez.
pub struct GpuiGrafik {
    grafik: Grafik,
    ana_sahne: Rc<Sahne>,
    ana_yüzey: Option<Entity<GpuiAnaYüzey>>,
    imleç: Option<İmleçDurumu>,
    seçim: Option<(Nokta, Nokta)>,
    açıklama_seçimi: bool,
    taşıma_başlangıcı: Option<Nokta>,
    dokunma_kaydırma: Option<(f64, f64)>,
    boşluk_basılı: bool,
    hata: Option<String>,
    çizim_sınırları: Rc<Cell<Option<Bounds<Pixels>>>>,
    odak: Option<FocusHandle>,
    imleç_kilitli: bool,
    eksen_üzerinde: bool,
    açıklama_vuruşu: Option<AçıklamaVuruşu>,
}

struct GpuiAnaYüzey {
    sahne: Rc<Sahne>,
    çizim_sınırları: Rc<Cell<Option<Bounds<Pixels>>>>,
    yol_önbelleği: Rc<RefCell<GpuiYolÖnbelleği>>,
    duyarlı_grafik: Option<WeakEntity<GpuiGrafik>>,
}

impl GpuiAnaYüzey {
    fn sahneyi_ayarla(
        &mut self, sahne: Rc<Sahne>, duyarlı_grafik: Option<WeakEntity<GpuiGrafik>>
    ) {
        self.yol_önbelleği
            .borrow_mut()
            .sahneyi_değiştir(&self.sahne, &sahne);
        self.sahne = sahne;
        self.duyarlı_grafik = duyarlı_grafik;
    }
}

#[derive(Default)]
struct GpuiYolÖnbelleği {
    sahne_boyutu: Option<(u32, u32)>,
    sınırlar: Option<Bounds<Pixels>>,
    yollar: Vec<Option<Path<Pixels>>>,
}

impl GpuiYolÖnbelleği {
    fn yüzeyi_hazırla(&mut self, sahne: &Sahne, sınırlar: Bounds<Pixels>) {
        let sahne_boyutu = sahne.boyut();
        if self.sahne_boyutu != Some(sahne_boyutu) || self.sınırlar != Some(sınırlar) {
            self.sahne_boyutu = Some(sahne_boyutu);
            self.sınırlar = Some(sınırlar);
            self.yollar.clear();
        }
        if self.yollar.len() != sahne.komutlar().len() {
            self.yollar.resize_with(sahne.komutlar().len(), || None);
        }
    }

    fn sahneyi_değiştir(&mut self, eski: &Sahne, yeni: &Sahne) -> usize {
        if eski.boyut() != yeni.boyut() {
            self.sahne_boyutu = Some(yeni.boyut());
            self.sınırlar = None;
            self.yollar.clear();
            return 0;
        }

        let mut eski_yollar = std::mem::take(&mut self.yollar);
        let mut yeni_yollar = vec![None; yeni.komutlar().len()];
        let mut korunan = 0;
        for (indeks, (eski_komut, yeni_komut)) in
            eski.komutlar().iter().zip(yeni.komutlar()).enumerate()
        {
            if !aynı_yol_geometrisi(eski_komut, yeni_komut) {
                continue;
            }
            let Some(eski_yol) = eski_yollar.get_mut(indeks).and_then(Option::take) else {
                continue;
            };
            let Some(yeni_yol) = yeni_yollar.get_mut(indeks) else {
                continue;
            };
            *yeni_yol = Some(eski_yol);
            korunan += 1;
        }
        self.yollar = yeni_yollar;
        korunan
    }

    fn yol(
        &mut self,
        indeks: usize,
        oluştur: impl FnOnce() -> Option<Path<Pixels>>,
    ) -> Option<Path<Pixels>> {
        if let Some(yol) = self
            .yollar
            .get(indeks)
            .and_then(|yol| yol.as_ref())
            .cloned()
        {
            return Some(yol);
        }
        let yol = oluştur()?;
        if let Some(hedef) = self.yollar.get_mut(indeks) {
            *hedef = Some(yol.clone());
        }
        Some(yol)
    }
}

fn aynı_yol_geometrisi(eski: &Komut, yeni: &Komut) -> bool {
    match (eski, yeni) {
        (
            Komut::Çizgi {
                başlangıç: eski_başlangıç,
                bitiş: eski_bitiş,
                kalınlık: eski_kalınlık,
                ..
            },
            Komut::Çizgi {
                başlangıç: yeni_başlangıç,
                bitiş: yeni_bitiş,
                kalınlık: yeni_kalınlık,
                ..
            },
        ) => {
            eski_başlangıç == yeni_başlangıç
                && eski_bitiş == yeni_bitiş
                && eski_kalınlık == yeni_kalınlık
        }
        (
            Komut::KesikliÇizgi {
                başlangıç: eski_başlangıç,
                bitiş: eski_bitiş,
                kalınlık: eski_kalınlık,
                kesik: eski_kesik,
                ..
            },
            Komut::KesikliÇizgi {
                başlangıç: yeni_başlangıç,
                bitiş: yeni_bitiş,
                kalınlık: yeni_kalınlık,
                kesik: yeni_kesik,
                ..
            },
        ) => {
            eski_başlangıç == yeni_başlangıç
                && eski_bitiş == yeni_bitiş
                && eski_kalınlık == yeni_kalınlık
                && eski_kesik == yeni_kesik
        }
        (
            Komut::Yol {
                parçalar: eski_parçalar,
                kalınlık: eski_kalınlık,
                ..
            },
            Komut::Yol {
                parçalar: yeni_parçalar,
                kalınlık: yeni_kalınlık,
                ..
            },
        )
        | (
            Komut::GradyanYol {
                parçalar: eski_parçalar,
                kalınlık: eski_kalınlık,
                ..
            },
            Komut::GradyanYol {
                parçalar: yeni_parçalar,
                kalınlık: yeni_kalınlık,
                ..
            },
        ) => eski_parçalar == yeni_parçalar && eski_kalınlık == yeni_kalınlık,
        (
            Komut::KesikliYol {
                parçalar: eski_parçalar,
                kalınlık: eski_kalınlık,
                çizgi: eski_çizgi,
                boşluk: eski_boşluk,
                ..
            },
            Komut::KesikliYol {
                parçalar: yeni_parçalar,
                kalınlık: yeni_kalınlık,
                çizgi: yeni_çizgi,
                boşluk: yeni_boşluk,
                ..
            },
        ) => {
            eski_parçalar == yeni_parçalar
                && eski_kalınlık == yeni_kalınlık
                && eski_çizgi == yeni_çizgi
                && eski_boşluk == yeni_boşluk
        }
        (
            Komut::Alan {
                çokgenler: eski_çokgenler,
                ..
            },
            Komut::Alan {
                çokgenler: yeni_çokgenler,
                ..
            },
        )
        | (
            Komut::GradyanAlan {
                çokgenler: eski_çokgenler,
                ..
            },
            Komut::GradyanAlan {
                çokgenler: yeni_çokgenler,
                ..
            },
        ) => eski_çokgenler == yeni_çokgenler,
        _ => false,
    }
}

impl Render for GpuiAnaYüzey {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let sahne = self.sahne.clone();
        let çizim_sınırları = self.çizim_sınırları.clone();
        let yol_önbelleği = self.yol_önbelleği.clone();
        let duyarlı_grafik = self.duyarlı_grafik.clone();
        canvas(
            move |sınırlar, _, uygulama| {
                çizim_sınırları.set(Some(sınırlar));
                let Some(grafik) = duyarlı_grafik else {
                    return;
                };
                let genişlik = f32::from(sınırlar.size.width).round().max(160.0) as u32;
                let yükseklik = f32::from(sınırlar.size.height).round().max(120.0) as u32;
                uygulama.defer(move |uygulama| {
                    if let Some(grafik) = grafik.upgrade() {
                        grafik.update(uygulama, |grafik, cx| {
                            let _ = grafik.boyutu_ayarla(genişlik, yükseklik, cx);
                        });
                    }
                });
            },
            move |sınırlar, _, pencere, uygulama| {
                sahneyi_önbellekli_boya(
                    &sahne,
                    sınırlar,
                    &mut yol_önbelleği.borrow_mut(),
                    pencere,
                    uygulama,
                );
            },
        )
        .size_full()
    }
}

impl GpuiGrafik {
    pub fn yeni(grafik: Grafik) -> Self {
        let ana_sahne = Rc::new(grafik.çiz());
        Self {
            grafik,
            ana_sahne,
            ana_yüzey: None,
            imleç: None,
            seçim: None,
            açıklama_seçimi: false,
            taşıma_başlangıcı: None,
            dokunma_kaydırma: None,
            boşluk_basılı: false,
            hata: None,
            çizim_sınırları: Rc::new(Cell::new(None)),
            odak: None,
            imleç_kilitli: false,
            eksen_üzerinde: false,
            açıklama_vuruşu: None,
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

    pub fn lejant_değerleri(&self) -> Option<(Option<f64>, Vec<Option<f64>>)> {
        if !self.grafik.lejant_canlı() {
            return None;
        }
        self.imleç
            .as_ref()
            .map(|imleç| (Some(imleç.veri_x), imleç.seri_değerleri.clone()))
            .or_else(|| {
                self.grafik
                    .boşta_lejant_değerleri()
                    .map(|değerler| (None, değerler))
            })
    }

    pub fn senkron_yayını(&self) -> Option<(f64, f64, Option<usize>)> {
        let imleç = self.imleç.as_ref()?;
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        if genişlik <= 0.0 || yükseklik <= 0.0 {
            return None;
        }
        Some((
            f64::from((imleç.fare.x - sol) / genişlik),
            f64::from((imleç.fare.y - üst) / yükseklik),
            self.grafik.odak_serisi(),
        ))
    }

    pub fn senkron_veri_yayını(&self) -> Option<(f64, f64, Option<usize>)> {
        let imleç = self.imleç.as_ref()?;
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let yatay = f64::from((imleç.fare.x - sol) / (sağ - sol));
        let dikey = f64::from((imleç.fare.y - üst) / (alt - üst));
        let (_, y_oranı) = self.grafik.fiziksel_oranları_mantıksala(yatay, dikey);
        let y_aralığı = self.grafik.görünür_y_aralığı();
        let y = y_aralığı.en_çok - y_oranı * (y_aralığı.en_çok - y_aralığı.en_az);
        Some((imleç.veri_x, y, self.grafik.odak_serisi()))
    }

    pub fn senkron_veri_imleci_ayarla(
        &mut self,
        x: f64,
        y: f64,
        odak_serisi: Option<usize>,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.imleç_kilitli || !x.is_finite() || !y.is_finite() {
            return false;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let (Some(x_oranı), Some(y_oranı)) = (
            self.grafik.x_konum_oranı(x),
            self.grafik.seri_y_konum_oranı(0, y),
        ) else {
            return false;
        };
        let x_dikey = self.grafik.x_dikey_mi();
        let fare = if x_dikey {
            Nokta::yeni(
                sol + y_oranı as f32 * (sağ - sol),
                alt - x_oranı as f32 * (alt - üst),
            )
        } else {
            Nokta::yeni(
                sol + x_oranı as f32 * (sağ - sol),
                alt - y_oranı as f32 * (alt - üst),
            )
        };
        let eksen_uzunluğu = if x_dikey { alt - üst } else { sağ - sol };
        let Some(çözüm) = self.grafik.imleç_çözümü(x_oranı, f64::from(eksen_uzunluğu))
        else {
            return false;
        };
        self.imleç = Some(İmleçDurumu {
            fare,
            veri_x: çözüm.ortak_x,
            seri_x_değerleri: çözüm
                .seriler
                .iter()
                .map(|örnek| örnek.map(|örnek| örnek.x))
                .collect(),
            seri_değerleri: çözüm
                .seriler
                .iter()
                .map(|örnek| örnek.map(|örnek| örnek.değer))
                .collect(),
            dağılım: None,
        });
        if self.grafik.imleç_odağını_seriye_ayarla(odak_serisi) {
            self.grafik_bildir(cx);
        } else {
            cx.notify();
        }
        true
    }

    pub fn senkron_imleci_ayarla(
        &mut self,
        yatay_oran: f64,
        dikey_oran: Option<f64>,
        odak_serisi: Option<usize>,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.imleç_kilitli || !yatay_oran.is_finite() {
            return false;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let x_oranı = yatay_oran.clamp(0.0, 1.0);
        let Some(çözüm) = self.grafik.imleç_çözümü(x_oranı, f64::from(sağ - sol)) else {
            return false;
        };
        let seri_x_değerleri = çözüm
            .seriler
            .iter()
            .map(|örnek| örnek.map(|örnek| örnek.x))
            .collect();
        let seri_değerleri = çözüm
            .seriler
            .iter()
            .map(|örnek| örnek.map(|örnek| örnek.değer))
            .collect();
        let y = dikey_oran
            .filter(|oran| oran.is_finite())
            .map_or(-10.0, |oran| {
                üst + oran.clamp(0.0, 1.0) as f32 * (alt - üst)
            });
        self.açıklama_vuruşu = None;
        self.imleç = Some(İmleçDurumu {
            fare: Nokta::yeni(
                sol + self.grafik.x_konum_oranı(çözüm.imleç_x).unwrap_or(x_oranı) as f32
                    * (sağ - sol),
                y,
            ),
            veri_x: çözüm.ortak_x,
            seri_x_değerleri,
            seri_değerleri,
            dağılım: None,
        });
        if self.grafik.imleç_odağını_seriye_ayarla(odak_serisi) {
            self.grafik_bildir(cx);
        } else {
            cx.notify();
        }
        true
    }

    pub fn senkron_imleci_temizle(&mut self, cx: &mut Context<Self>) -> bool {
        if self.imleç_kilitli || self.imleç.is_none() {
            return false;
        }
        self.imleç = None;
        if self.grafik.imleç_odağını_temizle() {
            self.grafik_bildir(cx);
        } else {
            cx.notify();
        }
        true
    }

    pub fn senkron_kilidi_ayarla(&mut self, kilitli: bool, cx: &mut Context<Self>) -> bool {
        let değişti = self.imleç_kilitli != kilitli;
        self.imleç_kilitli = kilitli;
        if değişti {
            cx.notify();
        }
        değişti
    }

    pub fn grafiği_ayarla(&mut self, grafik: Grafik, cx: &mut Context<Self>) {
        let imleci_koru = self
            .grafik
            .tooltip_düzeni()
            .is_some_and(|düzen| düzen.imleç_durumunu_koru)
            && grafik
                .tooltip_düzeni()
                .is_some_and(|düzen| düzen.imleç_durumunu_koru);
        let korunmuş_imleç = imleci_koru.then(|| self.imleç.clone()).flatten();
        let korunmuş_kilit = imleci_koru && self.imleç_kilitli;
        self.grafik = grafik;
        self.imleç = korunmuş_imleç;
        self.seçim = None;
        self.açıklama_seçimi = false;
        self.taşıma_başlangıcı = None;
        self.dokunma_kaydırma = None;
        self.boşluk_basılı = false;
        self.hata = None;
        self.imleç_kilitli = korunmuş_kilit;
        self.açıklama_vuruşu = None;
        self.grafik_bildir(cx);
    }

    pub fn veriyi_ayarla(
        &mut self,
        veri: HizalıVeri,
        cx: &mut Context<Self>,
    ) -> Result<(), UplotHatası> {
        self.grafik.veriyi_ayarla(veri)?;
        self.imleç = None;
        self.açıklama_vuruşu = None;
        self.seçim = None;
        self.grafik_bildir(cx);
        Ok(())
    }

    pub fn canlı_veriyi_x_aralığında_ayarla(
        &mut self,
        veri: HizalıVeri,
        aralık: Aralık,
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let görünür_değişti = self.grafik.canlı_veriyi_x_aralığında_ayarla(veri, aralık)?;
        self.açıklama_vuruşu = None;
        if görünür_değişti {
            self.grafik_bildir(cx);
        }
        Ok(görünür_değişti)
    }

    pub fn canlı_x_aralığını_ayarla(
        &mut self,
        aralık: Aralık,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.canlı_x_aralığını_ayarla(aralık);
        if değişti {
            self.grafik_bildir(cx);
        }
        değişti
    }

    pub fn boyutu_ayarla(
        &mut self,
        genişlik: u32,
        yükseklik: u32,
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let değişti = self.grafik.boyutu_ayarla(genişlik, yükseklik)?;
        if değişti {
            self.grafik_bildir(cx);
        }
        Ok(değişti)
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
        self.açıklama_vuruşu = None;
        self.seçim = None;
        self.açıklama_seçimi = false;
        self.grafik_bildir(cx);
        Ok(())
    }

    pub fn seri_sil(&mut self, indeks: usize, cx: &mut Context<Self>) -> Result<(), UplotHatası> {
        self.grafik.seri_sil(indeks)?;
        self.imleç = None;
        self.açıklama_vuruşu = None;
        self.seçim = None;
        self.açıklama_seçimi = false;
        self.grafik_bildir(cx);
        Ok(())
    }

    /// Web tarafındaki lejant düğmeleriyle aynı görünürlük değişimini GPUI
    /// uygulamalarına sunar ve yalnız gerekli sahne katmanlarını yeniler.
    pub fn seri_görünürlüğünü_ayarla(
        &mut self,
        indeks: usize,
        görünür: bool,
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let değişti = self.grafik.seri_görünürlüğünü_ayarla(indeks, görünür)?;
        if değişti {
            self.imleç = None;
            self.grafik_bildir(cx);
        }
        Ok(değişti)
    }

    /// CSS bulunmayan GPUI yüzeylerinde seri çizgi/dolgu rengini çalışma
    /// anında değiştirir.
    pub fn seri_renklerini_ayarla(
        &mut self,
        indeks: usize,
        çizgi: impl Into<String>,
        dolgu: Option<String>,
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let değişti = self.grafik.seri_renklerini_ayarla(indeks, çizgi, dolgu)?;
        if değişti {
            self.grafik_bildir(cx);
        }
        Ok(değişti)
    }

    /// GPUI çubuk serilerinin nokta başına dinamik dolgu/vuruş paletini
    /// değiştirir.
    pub fn seri_çubuk_renklerini_ayarla(
        &mut self,
        indeks: usize,
        dolgular: Vec<String>,
        çizgiler: Vec<String>,
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let değişti = self
            .grafik
            .seri_çubuk_renklerini_ayarla(indeks, dolgular, çizgiler)?;
        if değişti {
            self.grafik_bildir(cx);
        }
        Ok(değişti)
    }

    pub fn boşlukları_birleştir_ayarla(
        &mut self,
        birleştir: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.boşlukları_birleştir_ayarla(birleştir);
        if değişti {
            self.imleç = None;
            self.grafik_bildir(cx);
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
            self.grafik_bildir(cx);
        }
        değişti
    }

    pub fn önceki_görünüm(&mut self, cx: &mut Context<Self>) -> bool {
        let değişti = self.grafik.önceki_görünüm();
        if değişti {
            self.grafik_bildir(cx);
        }
        değişti
    }

    pub fn tam_görünüm(&mut self, cx: &mut Context<Self>) -> bool {
        let değişti = self.grafik.tam_görünüm();
        if değişti {
            self.grafik_bildir(cx);
        }
        değişti
    }

    pub fn görünür_aralıkları_ayarla(
        &mut self,
        x: Aralık,
        y: Aralık,
        geçmişe_ekle: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.görünür_aralıkları_ayarla(x, y, geçmişe_ekle);
        if değişti {
            self.grafik_bildir(cx);
        }
        değişti
    }

    fn çizim_alanı(&self) -> (f32, f32, f32, f32) {
        let (genişlik, yükseklik) = self.grafik.boyut();
        self.grafik.çizim_alanı_boyutta(genişlik, yükseklik)
    }

    fn etkileşim_sahnesi(&self) -> Sahne {
        let (genişlik, yükseklik) = self.grafik.boyut();
        let mut sahne = Sahne::yeni(genişlik, yükseklik);
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        if let Some(imleç) = self.imleç.as_ref() {
            let timeline_sayısı = self.grafik.timeline_seri_sayısı();
            if timeline_sayısı > 0 {
                let yatay_oran = f64::from(
                    ((imleç.fare.x - sol) / (sağ - sol).max(f32::EPSILON)).clamp(0.0, 1.0),
                );
                let şerit_yüksekliği = (alt - üst) * 0.9 / timeline_sayısı as f32;
                let şerit_boşluğu = if timeline_sayısı > 1 {
                    (alt - üst) * 0.1 / timeline_sayısı.saturating_sub(1) as f32
                } else {
                    0.0
                };
                for vuruş in self.grafik.timeline_vuruşları(yatay_oran) {
                    let x0 = self
                        .grafik
                        .x_konum_oranı(vuruş.başlangıç)
                        .map_or(sol, |oran| sol + oran as f32 * (sağ - sol))
                        .clamp(sol, sağ);
                    let x1 = self
                        .grafik
                        .x_konum_oranı(vuruş.bitiş)
                        .map_or(sağ, |oran| sol + oran as f32 * (sağ - sol))
                        .clamp(sol, sağ);
                    if x1 > x0 {
                        sahne.ekle(Komut::Dikdörtgen {
                            konum: Nokta::yeni(
                                x0,
                                üst + vuruş.seri as f32 * (şerit_yüksekliği + şerit_boşluğu),
                            ),
                            genişlik: x1 - x0,
                            yükseklik: şerit_yüksekliği,
                            dolgu: "#0000004d".to_string(),
                            çizgi: "#00000000".to_string(),
                            kalınlık: 0.0,
                        });
                    }
                }
                return sahne;
            }
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
            let x_dikey = self.grafik.x_dikey_mi();
            let x_konumu = self.grafik.x_konum_oranı(imleç.veri_x).map_or(
                if x_dikey {
                    imleç.fare.y
                } else {
                    imleç.fare.x
                },
                |oran| {
                    if x_dikey {
                        alt - oran as f32 * (alt - üst)
                    } else {
                        sol + oran as f32 * (sağ - sol)
                    }
                },
            );
            sahne.ekle(if x_dikey {
                Komut::KesikliÇizgi {
                    başlangıç: Nokta::yeni(sol, x_konumu),
                    bitiş: Nokta::yeni(sağ, x_konumu),
                    renk: "#6b7280".to_string(),
                    kalınlık: 1.0,
                    kesik: 4.0,
                }
            } else {
                Komut::KesikliÇizgi {
                    başlangıç: Nokta::yeni(x_konumu, üst),
                    bitiş: Nokta::yeni(x_konumu, alt),
                    renk: "#6b7280".to_string(),
                    kalınlık: 1.0,
                    kesik: 4.0,
                }
            });
            if !self.grafik.eksen_göstergeleri_etkin() && self.grafik.imleç_y_görünür() {
                sahne.ekle(if x_dikey {
                    Komut::KesikliÇizgi {
                        başlangıç: Nokta::yeni(imleç.fare.x, üst),
                        bitiş: Nokta::yeni(imleç.fare.x, alt),
                        renk: "#6b7280".to_string(),
                        kalınlık: 1.0,
                        kesik: 4.0,
                    }
                } else {
                    Komut::KesikliÇizgi {
                        başlangıç: Nokta::yeni(sol, imleç.fare.y),
                        bitiş: Nokta::yeni(sağ, imleç.fare.y),
                        renk: "#6b7280".to_string(),
                        kalınlık: 1.0,
                        kesik: 4.0,
                    }
                });
            } else {
                sahne.ekle(Komut::Dikdörtgen {
                    konum: Nokta::yeni(x_konumu - 24.0, alt + 6.0),
                    genişlik: 48.0,
                    yükseklik: 22.0,
                    dolgu: "#111111".to_string(),
                    çizgi: "#111111".to_string(),
                    kalınlık: 0.0,
                });
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(x_konumu, alt + 21.0),
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
                let seri_x = imleç
                    .seri_x_değerleri
                    .get(seri_indeksi)
                    .copied()
                    .flatten()
                    .unwrap_or(imleç.veri_x);
                let seri_rengi = self
                    .grafik
                    .seri_imleç_rengi(seri_indeksi, seri_x, *değer)
                    .unwrap_or_else(|| seri.renk.clone());
                let Some(y_oranı) = self.grafik.seri_y_konum_oranı(seri_indeksi, *değer) else {
                    continue;
                };
                let y_konumu = if x_dikey {
                    sol + y_oranı as f32 * (sağ - sol)
                } else {
                    alt - y_oranı as f32 * (alt - üst)
                };
                let seri_x_konumu = self.grafik.x_konum_oranı(seri_x).map_or(x_konumu, |oran| {
                    if x_dikey {
                        alt - oran as f32 * (alt - üst)
                    } else {
                        sol + oran as f32 * (sağ - sol)
                    }
                });
                let seri_noktası = if x_dikey {
                    Nokta::yeni(y_konumu, seri_x_konumu)
                } else {
                    Nokta::yeni(seri_x_konumu, y_konumu)
                };
                if self.grafik.eksen_göstergeleri_etkin() {
                    sahne.ekle(Komut::KesikliÇizgi {
                        başlangıç: Nokta::yeni(sol, y_konumu),
                        bitiş: Nokta::yeni(sağ, y_konumu),
                        renk: seri_rengi.clone(),
                        kalınlık: 1.0,
                        kesik: 4.0,
                    });
                    let rozet_x = sol - 50.0 - seri_indeksi as f32 * 56.0;
                    sahne.ekle(Komut::Dikdörtgen {
                        konum: Nokta::yeni(rozet_x, y_konumu - 11.0),
                        genişlik: 44.0,
                        yükseklik: 22.0,
                        dolgu: seri_rengi.clone(),
                        çizgi: seri_rengi.clone(),
                        kalınlık: 0.0,
                    });
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(rozet_x + 22.0, y_konumu + 4.0),
                        içerik: format!("{değer:.2}"),
                        renk: "#ffffff".to_string(),
                        boyut: 11.0,
                        hiza: MetinHizası::Orta,
                    });
                }
                sahne.ekle(Komut::Daire {
                    merkez: seri_noktası,
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
            let xy = self.grafik.etkileşim_seçenekleri().seçim_xy_yakınlaştır;
            let x_dikey = self.grafik.x_dikey_mi();
            sahne.ekle(Komut::Dikdörtgen {
                konum: if xy {
                    Nokta::yeni(başlangıç.x.min(bitiş.x), başlangıç.y.min(bitiş.y))
                } else if x_dikey {
                    Nokta::yeni(sol, başlangıç.y.min(bitiş.y))
                } else {
                    Nokta::yeni(başlangıç.x.min(bitiş.x), üst)
                },
                genişlik: if xy {
                    (bitiş.x - başlangıç.x).abs()
                } else if x_dikey {
                    sağ - sol
                } else {
                    (bitiş.x - başlangıç.x).abs()
                },
                yükseklik: if xy || x_dikey {
                    (bitiş.y - başlangıç.y).abs()
                } else {
                    alt - üst
                },
                dolgu: dolgu.to_string(),
                çizgi: çizgi.to_string(),
                kalınlık: 1.0,
            });
        }
        if let Some(vuruş) = self.açıklama_vuruşu.as_ref() {
            for komut in self
                .grafik
                .açıklama_vurgu_sahnesi_boyutta(genişlik, yükseklik, vuruş)
                .komutlar()
            {
                sahne.ekle(komut.clone());
            }
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

    fn imleci_güncelle(&mut self, pencere_konumu: ::gpui::Point<Pixels>) -> bool {
        if self.imleç_kilitli {
            return false;
        }
        let Some(fare) = self.sahne_konumu(pencere_konumu) else {
            self.imleç = None;
            self.açıklama_vuruşu = None;
            return self.grafik.imleç_odağını_temizle();
        };
        if !self.grafik_alanında(fare) {
            self.imleç = None;
            self.açıklama_vuruşu = None;
            return self.grafik.imleç_odağını_temizle();
        }
        self.açıklama_vuruşu = self.grafik.açıklama_vuruşu_boyutta(
            self.grafik.boyut().0,
            self.grafik.boyut().1,
            fare.x,
            fare.y,
        );
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
                seri_x_değerleri: değerler
                    .iter()
                    .map(|değer| değer.map(|_| vuruş.x))
                    .collect(),
                seri_değerleri: değerler,
                dağılım: Some(vuruş),
            });
            return false;
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
            return self.grafik.imleç_odağını_temizle();
        };
        let x_dikey = self.grafik.x_dikey_mi();
        let odak_değişti = self.grafik.imleç_odağını_güncelle(
            yatay,
            dikey,
            if x_dikey {
                f64::from(sağ - sol)
            } else {
                f64::from(alt - üst)
            },
        );
        let x_oranı = if x_dikey { 1.0 - dikey } else { yatay };
        let x_uzunluğu = if x_dikey { alt - üst } else { sağ - sol };
        let Some(çözüm) = self.grafik.imleç_çözümü(x_oranı, f64::from(x_uzunluğu)) else {
            self.imleç = None;
            return self.grafik.imleç_odağını_temizle() || odak_değişti;
        };
        let seri_x_değerleri = çözüm
            .seriler
            .iter()
            .map(|örnek| örnek.map(|örnek| örnek.x))
            .collect();
        let seri_değerleri = çözüm
            .seriler
            .iter()
            .map(|örnek| örnek.map(|örnek| örnek.değer))
            .collect();
        let x_konumu = self.grafik.x_konum_oranı(çözüm.imleç_x).unwrap_or(x_oranı) as f32;
        self.imleç = Some(İmleçDurumu {
            fare: if x_dikey {
                Nokta::yeni(
                    sol + yatay as f32 * (sağ - sol),
                    alt - x_konumu * (alt - üst),
                )
            } else {
                Nokta::yeni(
                    sol + x_konumu * (sağ - sol),
                    üst + dikey as f32 * (alt - üst),
                )
            },
            veri_x: çözüm.ortak_x,
            seri_x_değerleri,
            seri_değerleri,
            dağılım: None,
        });
        odak_değişti
    }

    fn tekerlek_yakınlaştır(&mut self, olay: &ScrollWheelEvent) -> bool {
        let Some(fare) = self.sahne_konumu(olay.position) else {
            return false;
        };
        if !self.grafik_alanında(fare) {
            return false;
        }
        if cfg!(target_os = "windows") && self.grafik.etkileşim_seçenekleri().dokunma_etkileşimi
        {
            match olay.touch_phase {
                TouchPhase::Started => {
                    let _ = self.grafik.taşımayı_başlat();
                    self.dokunma_kaydırma = Some((0.0_f64, 0.0_f64));
                    return false;
                }
                TouchPhase::Ended | TouchPhase::Cancelled if self.dokunma_kaydırma.is_some() => {
                    self.dokunma_kaydırma = None;
                    self.grafik.taşımayı_bitir();
                    return false;
                }
                TouchPhase::Moved => {}
                _ => return false,
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
            return match self.grafik.taşı(*birikmiş_x, *birikmiş_y) {
                Ok(değişti) => {
                    self.hata = None;
                    değişti
                }
                Err(hata) => {
                    self.hata = Some(format!("Dokunma taşıması uygulanamadı: {hata}"));
                    false
                }
            };
        }
        let eksen = match (olay.modifiers.shift, olay.modifiers.control) {
            (true, false) => TekerlekEkseni::X,
            (false, true) => TekerlekEkseni::Y,
            _ => TekerlekEkseni::İkisi,
        };
        let (delta, hassas) = match olay.delta {
            ScrollDelta::Pixels(delta) => {
                let x = f64::from(f32::from(delta.x));
                let y = f64::from(f32::from(delta.y));
                (
                    if eksen == TekerlekEkseni::X && x.abs() > y.abs() {
                        x
                    } else {
                        y
                    },
                    true,
                )
            }
            ScrollDelta::Lines(delta) => {
                let x = f64::from(delta.x);
                let y = f64::from(delta.y);
                (
                    if eksen == TekerlekEkseni::X && x.abs() > y.abs() {
                        x
                    } else {
                        y
                    },
                    false,
                )
            }
        };
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let yatay = f64::from((fare.x - sol) / (sağ - sol));
        let dikey = f64::from((fare.y - üst) / (alt - üst));
        match self
            .grafik
            .tekerlek_eksende(yatay, dikey, delta, hassas, eksen)
        {
            Ok(değişti) => {
                self.hata = None;
                değişti
            }
            Err(hata) => {
                self.hata = Some(format!("Tekerlek yakınlaştırması uygulanamadı: {hata}"));
                false
            }
        }
    }

    fn dokunma_yakınlaştır(&mut self, olay: &PinchEvent) -> bool {
        if matches!(olay.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
            self.grafik.dokunmayı_bitir();
            return false;
        }
        if olay.phase == TouchPhase::Started && !self.grafik.dokunmayı_başlat() {
            return false;
        }
        let Some(fare) = self.sahne_konumu(olay.position) else {
            return false;
        };
        if !self.grafik_alanında(fare) {
            return false;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let yatay = f64::from((fare.x - sol) / (sağ - sol));
        let dikey = f64::from((fare.y - üst) / (alt - üst));
        let çarpan = f64::from((1.0 + olay.delta).max(0.01));
        match self.grafik.dokunma_yakınlaştır(yatay, dikey, çarpan) {
            Ok(değişti) => {
                self.hata = None;
                değişti
            }
            Err(hata) => {
                self.hata = Some(format!("Dokunma yakınlaştırması uygulanamadı: {hata}"));
                false
            }
        }
    }

    fn grafik_bildir(&mut self, cx: &mut Context<Self>) {
        self.açıklama_vuruşu = None;
        self.ana_sahne = Rc::new(self.grafik.çiz());
        let duyarlı_grafik = self.grafik.duyarlı_boyut_mu().then(|| cx.weak_entity());
        if let Some(yüzey) = self.ana_yüzey.as_ref() {
            let sahne = self.ana_sahne.clone();
            yüzey.update(cx, |yüzey, cx| {
                yüzey.sahneyi_ayarla(sahne, duyarlı_grafik);
                cx.notify();
            });
        }
        Self::bildir(cx);
    }

    fn bildir(cx: &mut Context<Self>) {
        cx.emit(GpuiGrafikOlayı::DurumDeğişti);
        cx.notify();
    }

    fn imleç_bildir(cx: &mut Context<Self>) {
        cx.emit(GpuiGrafikOlayı::İmleçDeğişti);
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
        let ana_yüzey = self
            .ana_yüzey
            .get_or_insert_with(|| {
                let sahne = self.ana_sahne.clone();
                let çizim_sınırları = self.çizim_sınırları.clone();
                let duyarlı_grafik = self.grafik.duyarlı_boyut_mu().then(|| cx.weak_entity());
                cx.new(|_| GpuiAnaYüzey {
                    sahne,
                    çizim_sınırları,
                    yol_önbelleği: Rc::new(RefCell::new(GpuiYolÖnbelleği::default())),
                    duyarlı_grafik,
                })
            })
            .clone();
        let etkileşim_sahnesi = Rc::new(self.etkileşim_sahnesi());
        let taşıyor = self.taşıma_başlangıcı.is_some();
        let taşımaya_hazır = self.boşluk_basılı && self.grafik.yakınlaştırılmış();
        let eksen_sürükleniyor = self.grafik.eksen_sürükleniyor();
        let eksen_imleci = self.eksen_üzerinde || eksen_sürükleniyor;
        let standart_bilgi_kutusu = self
            .imleç
            .as_ref()
            .filter(|_| self.grafik.etkileşim_seçenekleri().imleç_bilgi_kutusu)
            .filter(|_| self.grafik.tooltip_düzeni().is_none())
            .and_then(|imleç| {
                let seri_indeksi = self.grafik.odak_serisi().or_else(|| {
                    (!self.grafik.en_yakın_tooltip_etkin())
                        .then(|| {
                            imleç
                                .seri_değerleri
                                .iter()
                                .position(|değer| değer.is_some())
                        })
                        .flatten()
                });
                let y = imleç.dağılım.as_ref().map(|vuruş| vuruş.y).or_else(|| {
                    seri_indeksi
                        .and_then(|indeks| imleç.seri_değerleri.get(indeks))
                        .copied()
                        .flatten()
                })?;
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
                let (çizim_sol, çizim_sağ, _, _) = self.çizim_alanı();
                let yatay_oran = f64::from(
                    ((imleç.fare.x - çizim_sol) / (çizim_sağ - çizim_sol)).clamp(0.0, 1.0),
                );
                let en_yakın =
                    seri_indeksi.and_then(|seri| self.grafik.en_yakın_tooltip(yatay_oran, seri));
                if self.grafik.en_yakın_tooltip_etkin() && en_yakın.is_none() {
                    return None;
                }
                let kenarlık = en_yakın.as_ref().map_or_else(
                    || "#000000".to_string(),
                    |bilgi| bilgi.kenarlık_rengi.clone(),
                );
                let bağlantı = en_yakın
                    .as_ref()
                    .map(|bilgi| bilgi.karşılaştırma_url.clone());
                Some((
                    sol,
                    üst,
                    en_yakın.map_or_else(
                        || {
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
                                        vuruş.değer.map_or_else(
                                            || "--".to_string(),
                                            |değer| değer.to_string()
                                        ),
                                        vuruş.x,
                                        vuruş.y
                                    )
                                },
                            )
                        },
                        |bilgi| bilgi.metin,
                    ),
                    kenarlık,
                    bağlantı,
                ))
            });
        let açıklama_bilgi_kutusu = self
            .açıklama_vuruşu
            .as_ref()
            .filter(|vuruş| vuruş.etiket_üzerinde && !vuruş.açıklama.is_empty())
            .and_then(|vuruş| {
                let imleç = self.imleç.as_ref()?;
                let sınırlar = self.çizim_sınırları.get()?;
                let (kaynak_g, kaynak_y) = self.grafik.boyut();
                let ölçek = (f32::from(sınırlar.size.width) / kaynak_g as f32)
                    .min(f32::from(sınırlar.size.height) / kaynak_y as f32)
                    .max(0.01);
                let yatay_pay = (f32::from(sınırlar.size.width) - kaynak_g as f32 * ölçek) / 2.0;
                let dikey_pay = (f32::from(sınırlar.size.height) - kaynak_y as f32 * ölçek) / 2.0;
                Some((
                    (yatay_pay + imleç.fare.x * ölçek + 12.0)
                        .clamp(4.0, (f32::from(sınırlar.size.width) - 190.0).max(4.0)),
                    (dikey_pay + imleç.fare.y * ölçek + 12.0)
                        .clamp(4.0, (f32::from(sınırlar.size.height) - 42.0).max(4.0)),
                    vuruş.açıklama.clone(),
                    vuruş.çizgi.clone(),
                    None,
                ))
            });
        let bilgi_kutusu = açıklama_bilgi_kutusu.or(standart_bilgi_kutusu);
        let tooltip_kutuları = self
            .imleç
            .as_ref()
            .filter(|_| self.grafik.etkileşim_seçenekleri().imleç_bilgi_kutusu)
            .and_then(|imleç| {
                let sınırlar = self.çizim_sınırları.get()?;
                let (kaynak_g, kaynak_y) = self.grafik.boyut();
                let ölçek = (f32::from(sınırlar.size.width) / kaynak_g as f32)
                    .min(f32::from(sınırlar.size.height) / kaynak_y as f32)
                    .max(0.01);
                let yatay_pay = (f32::from(sınırlar.size.width) - kaynak_g as f32 * ölçek) / 2.0;
                let dikey_pay = (f32::from(sınırlar.size.height) - kaynak_y as f32 * ölçek) / 2.0;
                let (sol, sağ, üst, alt) = self.çizim_alanı();
                let yatay_oran = f64::from(((imleç.fare.x - sol) / (sağ - sol)).clamp(0.0, 1.0));
                let dikey_oran = f64::from(((imleç.fare.y - üst) / (alt - üst)).clamp(0.0, 1.0));
                Some(
                    self.grafik
                        .tooltip_bilgileri(yatay_oran, dikey_oran)
                        .into_iter()
                        .map(|bilgi| {
                            let kaynak_x =
                                sol + (sağ - sol) * bilgi.yatay_oran.clamp(0.0, 1.0) as f32;
                            let kaynak_y =
                                üst + (alt - üst) * bilgi.dikey_oran.clamp(0.0, 1.0) as f32;
                            let kutu_sol = (yatay_pay + kaynak_x * ölçek + 10.0)
                                .clamp(4.0, (f32::from(sınırlar.size.width) - 112.0).max(4.0));
                            let kutu_üst = (dikey_pay + kaynak_y * ölçek + 10.0)
                                .clamp(4.0, (f32::from(sınırlar.size.height) - 32.0).max(4.0));
                            (
                                kutu_sol,
                                kutu_üst,
                                bilgi.metin,
                                bilgi.arka_plan_rengi,
                                bilgi.metin_rengi,
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap_or_default();
        div()
            .id("uplot-rs-gpui-grafik")
            .relative()
            .track_focus(&odak)
            .size_full()
            .min_h(px(120.0))
            .overflow_hidden()
            .when(taşıyor, |yüzey| yüzey.cursor_grabbing())
            .when(!taşıyor && taşımaya_hazır, |yüzey| yüzey.cursor_grab())
            .when(!taşıyor && !taşımaya_hazır && eksen_imleci, |yüzey| {
                yüzey.cursor_move()
            })
            .when(!taşıyor && self.açıklama_vuruşu.is_some(), |yüzey| {
                yüzey.cursor_pointer()
            })
            .on_key_down(cx.listener(|bu, olay: &KeyDownEvent, _, cx| {
                let tuş = olay.keystroke.key.as_str();
                if tuş == "escape" && bu.grafik.ölçüm_datumları_etkin() {
                    bu.grafik.ölçüm_datumlarını_temizle();
                    cx.stop_propagation();
                    bu.grafik_bildir(cx);
                } else if matches!(tuş, "1" | "2")
                    && bu.grafik.ölçüm_datumları_etkin()
                    && let Some(imleç) = bu.imleç.as_ref()
                {
                    let (sol, sağ, üst, alt) = bu.çizim_alanı();
                    let yatay = f64::from((imleç.fare.x - sol) / (sağ - sol));
                    let dikey = f64::from((imleç.fare.y - üst) / (alt - üst));
                    let datum = if tuş == "1" { 1 } else { 2 };
                    bu.grafik.ölçüm_datumunu_ayarla(datum, yatay, dikey);
                    cx.stop_propagation();
                    bu.grafik_bildir(cx);
                } else if tuş == "space" {
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
                let mut ana_sahne_değişti = false;
                if let Some(odak) = bu.odak.as_ref()
                    && !odak.is_focused(window)
                    && bu
                        .sahne_konumu(olay.position)
                        .is_some_and(|konum| bu.grafik_alanında(konum))
                {
                    odak.focus(window, cx);
                }
                if bu.grafik.eksen_sürükleniyor()
                    && let Some(konum) = bu.sahne_konumu(olay.position)
                {
                    match bu
                        .grafik
                        .eksen_sürükle(konum.x, konum.y, olay.modifiers.shift)
                    {
                        Ok(değişti) => {
                            bu.hata = None;
                            ana_sahne_değişti = değişti;
                        }
                        Err(hata) => {
                            bu.hata = Some(format!("Eksen ölçeği sürüklenemedi: {hata}"));
                        }
                    }
                    bu.imleç = None;
                    bu.açıklama_vuruşu = None;
                } else if let Some(başlangıç) = bu.taşıma_başlangıcı
                    && let Some(konum) = bu.sahne_konumu(olay.position)
                {
                    let (sol, sağ, üst, alt) = bu.çizim_alanı();
                    let yatay = f64::from((konum.x - başlangıç.x) / (sağ - sol));
                    let dikey = f64::from((konum.y - başlangıç.y) / (alt - üst));
                    match bu.grafik.taşı(yatay, dikey) {
                        Ok(değişti) => {
                            bu.hata = None;
                            ana_sahne_değişti = değişti;
                        }
                        Err(hata) => {
                            bu.hata = Some(format!("Grafik görünümü taşınamadı: {hata}"));
                        }
                    }
                    bu.imleç = None;
                    bu.açıklama_vuruşu = None;
                } else {
                    ana_sahne_değişti = bu.imleci_güncelle(olay.position);
                }
                if bu.taşıma_başlangıcı.is_none()
                    && !bu.grafik.eksen_sürükleniyor()
                    && olay.dragging()
                    && let Some((başlangıç, _)) = bu.seçim
                    && let Some(konum) = bu.sahne_konumu(olay.position)
                {
                    let (sol, sağ, üst, alt) = bu.çizim_alanı();
                    let xy = bu.grafik.etkileşim_seçenekleri().seçim_xy_yakınlaştır;
                    let bitiş = if xy {
                        Nokta::yeni(konum.x.clamp(sol, sağ), konum.y.clamp(üst, alt))
                    } else if bu.grafik.x_dikey_mi() {
                        Nokta::yeni(başlangıç.x, konum.y.clamp(üst, alt))
                    } else {
                        Nokta::yeni(konum.x.clamp(sol, sağ), başlangıç.y)
                    };
                    bu.seçim = Some((başlangıç, bitiş));
                }
                if !bu.grafik.eksen_sürükleniyor()
                    && let Some(konum) = bu.sahne_konumu(olay.position)
                {
                    let (genişlik, yükseklik) = bu.grafik.boyut();
                    bu.eksen_üzerinde = bu
                        .grafik
                        .eksen_vuruşu_boyutta(genişlik, yükseklik, konum.x, konum.y)
                        .is_some();
                }
                if ana_sahne_değişti {
                    bu.grafik_bildir(cx);
                } else {
                    GpuiGrafik::imleç_bildir(cx);
                }
            }))
            .on_scroll_wheel(cx.listener(|bu, olay: &ScrollWheelEvent, _, cx| {
                let datum_değişti = bu.grafik.ölçüm_datumlarını_temizle();
                let görünüm_değişti = bu.tekerlek_yakınlaştır(olay);
                if datum_değişti || görünüm_değişti {
                    bu.grafik_bildir(cx);
                } else {
                    GpuiGrafik::bildir(cx);
                }
            }))
            .on_pinch(cx.listener(|bu, olay: &PinchEvent, _, cx| {
                let datum_değişti = bu.grafik.ölçüm_datumlarını_temizle();
                let görünüm_değişti = bu.dokunma_yakınlaştır(olay);
                if datum_değişti || görünüm_değişti {
                    bu.grafik_bildir(cx);
                } else {
                    GpuiGrafik::bildir(cx);
                }
            }))
            .on_mouse_exit(cx.listener(|bu, _: &MouseExitEvent, _, cx| {
                if !bu.imleç_kilitli && bu.seçim.is_none() && bu.taşıma_başlangıcı.is_none()
                {
                    bu.imleç = None;
                    bu.açıklama_vuruşu = None;
                    bu.eksen_üzerinde = false;
                    if bu.grafik.imleç_odağını_temizle() {
                        bu.grafik_bildir(cx);
                    } else {
                        GpuiGrafik::imleç_bildir(cx);
                    }
                }
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|bu, olay: &MouseDownEvent, window, cx| {
                    let mut ana_sahne_değişti = false;
                    if let Some(odak) = bu.odak.as_ref() {
                        odak.focus(window, cx);
                    }
                    let ayarlar = bu.grafik.etkileşim_seçenekleri();
                    let eksen_başladı = bu.sahne_konumu(olay.position).is_some_and(|konum| {
                        let (genişlik, yükseklik) = bu.grafik.boyut();
                        bu.grafik
                            .eksen_sürüklemeyi_başlat(genişlik, yükseklik, konum.x, konum.y)
                    });
                    if eksen_başladı {
                        bu.seçim = None;
                        bu.taşıma_başlangıcı = None;
                        bu.açıklama_seçimi = false;
                        bu.imleç = None;
                        bu.açıklama_vuruşu = None;
                    } else if bu.boşluk_basılı
                        && let Some(konum) = bu.sahne_konumu(olay.position)
                        && bu.grafik_alanında(konum)
                        && bu.grafik.taşımayı_başlat()
                    {
                        bu.taşıma_başlangıcı = Some(konum);
                        bu.seçim = None;
                        bu.açıklama_seçimi = false;
                        bu.imleç = None;
                        bu.açıklama_vuruşu = None;
                    } else if olay.click_count >= 2 && ayarlar.çift_tıkla_tam_görünüm {
                        let datum_değişti = bu.grafik.ölçüm_datumlarını_temizle();
                        let görünüm_değişti = bu.grafik.tam_görünüm();
                        ana_sahne_değişti = datum_değişti || görünüm_değişti;
                        bu.seçim = None;
                        bu.açıklama_seçimi = false;
                    } else if ayarlar.seçim_yakınlaştır
                        && let Some(konum) = bu.sahne_konumu(olay.position)
                        && bu.grafik_alanında(konum)
                    {
                        bu.seçim = Some((konum, konum));
                        bu.açıklama_seçimi = ayarlar.ctrl_açıklama && olay.modifiers.control;
                    }
                    if ana_sahne_değişti {
                        bu.grafik_bildir(cx);
                    } else {
                        GpuiGrafik::bildir(cx);
                    }
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|bu, _: &MouseUpEvent, _, cx| {
                    if bu.grafik.eksen_sürükleniyor() {
                        bu.grafik.eksen_sürüklemeyi_bitir();
                        GpuiGrafik::bildir(cx);
                        return;
                    }
                    if bu.taşıma_başlangıcı.take().is_some() {
                        bu.grafik.taşımayı_bitir();
                        GpuiGrafik::bildir(cx);
                        return;
                    }
                    let açıklama_seçimi = std::mem::take(&mut bu.açıklama_seçimi);
                    let mut ana_sahne_değişti = false;
                    if let Some((başlangıç, bitiş)) = bu.seçim.take() {
                        let ayarlar = bu.grafik.etkileşim_seçenekleri();
                        let x_farkı = (bitiş.x - başlangıç.x).abs();
                        let y_farkı = (bitiş.y - başlangıç.y).abs();
                        let yeterli = if ayarlar.seçim_xy_yakınlaştır {
                            x_farkı >= 4.0 && y_farkı >= 4.0
                        } else if bu.grafik.x_dikey_mi() {
                            y_farkı >= 4.0
                        } else {
                            x_farkı >= 4.0
                        };
                        if yeterli {
                            let (sol, sağ, üst, alt) = bu.çizim_alanı();
                            if ayarlar.seçim_xy_yakınlaştır {
                                match bu.grafik.fiziksel_seçim_yakınlaştır(
                                    f64::from((başlangıç.x - sol) / (sağ - sol)),
                                    f64::from((başlangıç.y - üst) / (alt - üst)),
                                    f64::from((bitiş.x - sol) / (sağ - sol)),
                                    f64::from((bitiş.y - üst) / (alt - üst)),
                                ) {
                                    Ok(değişti) => {
                                        bu.hata = None;
                                        ana_sahne_değişti = değişti;
                                    }
                                    Err(hata) => {
                                        bu.hata =
                                            Some(format!("Seçilen aralık uygulanamadı: {hata}"));
                                    }
                                }
                            } else {
                                let (başlangıç_oranı, bitiş_oranı) = if bu.grafik.x_dikey_mi() {
                                    (
                                        f64::from((alt - başlangıç.y) / (alt - üst)),
                                        f64::from((alt - bitiş.y) / (alt - üst)),
                                    )
                                } else {
                                    (
                                        f64::from((başlangıç.x - sol) / (sağ - sol)),
                                        f64::from((bitiş.x - sol) / (sağ - sol)),
                                    )
                                };
                                match bu.grafik.seçimi_bitir(
                                    başlangıç_oranı,
                                    bitiş_oranı,
                                    açıklama_seçimi,
                                ) {
                                    Ok(SeçimEylemi::Açıklamaİstendi) => {
                                        bu.hata = None;
                                        cx.emit(GpuiGrafikOlayı::Açıklamaİstendi);
                                    }
                                    Ok(_) => {
                                        bu.hata = None;
                                        ana_sahne_değişti = true;
                                    }
                                    Err(hata) => {
                                        bu.hata =
                                            Some(format!("Seçilen aralık uygulanamadı: {hata}"));
                                    }
                                }
                            }
                        } else {
                            cx.emit(GpuiGrafikOlayı::FareBırakıldı);
                        }
                    }
                    if ana_sahne_değişti {
                        bu.grafik_bildir(cx);
                    } else {
                        GpuiGrafik::bildir(cx);
                    }
                }),
            )
            .child(ana_yüzey.cached(StyleRefinement::default().size_full()))
            .child(
                canvas(
                    |_, _, _| {},
                    move |sınırlar, _, pencere, uygulama| {
                        sahneyi_boya(&etkileşim_sahnesi, sınırlar, pencere, uygulama);
                    },
                )
                .absolute()
                .size_full(),
            )
            .when_some(
                bilgi_kutusu,
                |yüzey, (sol, üst, metin, kenarlık, bağlantı)| {
                    let tıklama_bağlantısı = bağlantı.clone();
                    yüzey.child(
                        div()
                            .absolute()
                            .left(px(sol))
                            .top(px(üst))
                            .px_2()
                            .py_1()
                            .border_1()
                            .border_color(renk_çöz(&kenarlık))
                            .rounded_sm()
                            .bg(if bağlantı.is_some() {
                                rgb(0xffffff)
                            } else {
                                rgba(0x000000cc)
                            })
                            .text_color(if bağlantı.is_some() {
                                rgb(0x111111)
                            } else {
                                rgb(0xffffff)
                            })
                            .text_xs()
                            .when(bağlantı.is_some(), |kutu| {
                                kutu.cursor_pointer().on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |_, _: &MouseDownEvent, _, cx| {
                                        if let Some(url) = tıklama_bağlantısı.as_deref() {
                                            cx.open_url(url);
                                        }
                                    }),
                                )
                            })
                            .child(metin),
                    )
                },
            )
            .children(tooltip_kutuları.into_iter().map(
                |(sol, üst, metin, arka_plan, metin_rengi)| {
                    div()
                        .absolute()
                        .left(px(sol))
                        .top(px(üst))
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .bg(renk_çöz(&arka_plan))
                        .text_color(renk_çöz(&metin_rengi))
                        .text_xs()
                        .child(metin)
                },
            ))
    }
}

/// Ortak sahne komutlarını GPUI canvas üzerine boyar.
pub fn sahneyi_boya(
    sahne: &Sahne,
    sınırlar: Bounds<Pixels>,
    pencere: &mut Window,
    uygulama: &mut App,
) {
    let mut yol_önbelleği = GpuiYolÖnbelleği::default();
    sahneyi_önbellekli_boya(sahne, sınırlar, &mut yol_önbelleği, pencere, uygulama);
}

fn sahneyi_önbellekli_boya(
    sahne: &Sahne,
    sınırlar: Bounds<Pixels>,
    yol_önbelleği: &mut GpuiYolÖnbelleği,
    pencere: &mut Window,
    uygulama: &mut App,
) {
    yol_önbelleği.yüzeyi_hazırla(sahne, sınırlar);
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

    for (komut_indeksi, komut) in sahne.komutlar().iter().enumerate() {
        match komut {
            Komut::ArkaPlan { .. } => {}
            Komut::Çizgi {
                başlangıç,
                bitiş,
                renk,
                kalınlık,
            } => {
                if let Some(yol) = yol_önbelleği.yol(komut_indeksi, || {
                    let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek));
                    yol.move_to(dönüştür(*başlangıç));
                    yol.line_to(dönüştür(*bitiş));
                    yol.build().ok()
                }) {
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
                if let Some(yol) = yol_önbelleği.yol(komut_indeksi, || {
                    let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek))
                        .dash_array(&[px(*kesik * ölçek), px(*kesik * ölçek)]);
                    yol.move_to(dönüştür(*başlangıç));
                    yol.line_to(dönüştür(*bitiş));
                    yol.build().ok()
                }) {
                    pencere.paint_path(yol, renk_çöz(renk));
                }
            }
            Komut::Yol {
                parçalar,
                renk,
                kalınlık,
            } => {
                if let Some(yol) = yol_önbelleği.yol(komut_indeksi, || {
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
                    yol.build().ok()
                }) {
                    pencere.paint_path(yol, renk_çöz(renk));
                }
            }
            Komut::GradyanYol {
                parçalar,
                gradyan,
                kalınlık,
            } => {
                if let Some(yol) = yol_önbelleği.yol(komut_indeksi, || {
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
                    yol.build().ok()
                }) {
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
                if let Some(yol) = yol_önbelleği.yol(komut_indeksi, || {
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
                    yol.build().ok()
                }) {
                    pencere.paint_path(yol, renk_çöz(renk));
                }
            }
            Komut::Alan { çokgenler, dolgu } => {
                if let Some(yol) = yol_önbelleği.yol(komut_indeksi, || {
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
                    yol.build().ok()
                }) {
                    pencere.paint_path(yol, renk_çöz(dolgu));
                }
            }
            Komut::GradyanAlan {
                çokgenler, gradyan
            } => {
                if let Some(yol) = yol_önbelleği.yol(komut_indeksi, || {
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
                    yol.build().ok()
                }) {
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
            Komut::YuvarlatılmışDikdörtgen {
                konum,
                genişlik,
                yükseklik,
                yarıçaplar,
                dolgu,
                çizgi,
                kalınlık,
            } => {
                let konum = dönüştür(*konum);
                pencere.paint_quad(quad(
                    Bounds::new(konum, size(px(*genişlik * ölçek), px(*yükseklik * ölçek))),
                    Corners {
                        top_left: px(yarıçaplar.üst_sol * ölçek),
                        top_right: px(yarıçaplar.üst_sağ * ölçek),
                        bottom_right: px(yarıçaplar.alt_sağ * ölçek),
                        bottom_left: px(yarıçaplar.alt_sol * ölçek),
                    },
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
                // GPUI `shape_line` çok satırlı metni panic ile reddeder. Sahne
                // kaynağı dış veri/başlık içerebildiğinden adaptör sınırında
                // satır sonlarını güvenli tek satır boşluğuna dönüştürürüz.
                let tek_satır = içerik.replace(['\r', '\n'], " ");
                let paylaşımlı = SharedString::from(tek_satır);
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
    let kod = kod.trim().to_ascii_lowercase();
    if let Some(ham) = kod.strip_prefix('#') {
        return match ham.len() {
            3 => kısa_hex_rengi(ham, false),
            4 => kısa_hex_rengi(ham, true),
            6 => u32::from_str_radix(ham, 16)
                .map_or_else(|_| rgb(0x000000).into(), |sayı| rgb(sayı).into()),
            8 => u32::from_str_radix(ham, 16)
                .map_or_else(|_| rgb(0x000000).into(), |sayı| rgba(sayı).into()),
            _ => rgb(0x000000).into(),
        };
    }
    if let Some(renk) = css_rgb_rengi(&kod) {
        return renk;
    }
    let sayı = match kod.as_str() {
        "transparent" => 0x00000000,
        "white" => 0xffffffff,
        "red" => 0xff0000ff,
        "green" => 0x008000ff,
        "blue" => 0x0000ffff,
        "yellow" => 0xffff00ff,
        "orange" => 0xffa500ff,
        "purple" => 0x800080ff,
        "magenta" | "fuchsia" => 0xff00ffff,
        "cyan" | "aqua" => 0x00ffffff,
        "gray" | "grey" => 0x808080ff,
        "brown" => 0xa52a2aff,
        "teal" => 0x008080ff,
        "pink" => 0xffc0cbff,
        _ => 0x000000ff,
    };
    rgba(sayı).into()
}

fn kısa_hex_rengi(ham: &str, alfa_var: bool) -> Hsla {
    let mut rakamlar = ham.chars().filter_map(|rakam| rakam.to_digit(16));
    let Some(r) = rakamlar.next() else {
        return rgb(0x000000).into();
    };
    let Some(g) = rakamlar.next() else {
        return rgb(0x000000).into();
    };
    let Some(b) = rakamlar.next() else {
        return rgb(0x000000).into();
    };
    let a = if alfa_var {
        let Some(a) = rakamlar.next() else {
            return rgb(0x000000).into();
        };
        a * 17
    } else {
        255
    };
    rgba((r * 17) << 24 | (g * 17) << 16 | (b * 17) << 8 | a).into()
}

fn css_rgb_rengi(kod: &str) -> Option<Hsla> {
    let içerik = kod
        .strip_prefix("rgba(")
        .or_else(|| kod.strip_prefix("rgb("))?
        .strip_suffix(')')?;
    let normal = içerik.replace(',', " ");
    let (kanallar, eğik_alfa) = normal
        .split_once('/')
        .map_or((normal.as_str(), None), |(kanallar, alfa)| {
            (kanallar, Some(alfa.trim()))
        });
    let parçalar = kanallar.split_whitespace().collect::<Vec<_>>();
    let (r, g, b, eski_alfa) = match parçalar.as_slice() {
        [r, g, b] => (*r, *g, *b, None),
        [r, g, b, alfa] => (*r, *g, *b, Some(*alfa)),
        _ => return None,
    };
    let r = css_renk_kanalı(r)?;
    let g = css_renk_kanalı(g)?;
    let b = css_renk_kanalı(b)?;
    let a = eğik_alfa.or(eski_alfa).map_or(Some(255), css_alfa_kanalı)?;
    Some(rgba(u32::from(r) << 24 | u32::from(g) << 16 | u32::from(b) << 8 | u32::from(a)).into())
}

fn css_renk_kanalı(değer: &str) -> Option<u8> {
    if let Some(yüzde) = değer.strip_suffix('%') {
        return yüzde
            .parse::<f32>()
            .ok()
            .map(|oran| (oran.clamp(0.0, 100.0) * 2.55).round() as u8);
    }
    değer
        .parse::<f32>()
        .ok()
        .map(|kanal| kanal.clamp(0.0, 255.0).round() as u8)
}

fn css_alfa_kanalı(değer: &str) -> Option<u8> {
    if let Some(yüzde) = değer.strip_suffix('%') {
        return yüzde
            .parse::<f32>()
            .ok()
            .map(|oran| (oran.clamp(0.0, 100.0) * 2.55).round() as u8);
    }
    değer
        .parse::<f32>()
        .ok()
        .map(|alfa| (alfa.clamp(0.0, 1.0) * 255.0).round() as u8)
}

#[cfg(test)]
mod testler {
    use super::*;

    fn yol_sahnesi(renk: &str, kalınlık: f32, bitiş_x: f32) -> Sahne {
        let mut sahne = Sahne::yeni(320, 180);
        sahne.ekle(Komut::Yol {
            parçalar: vec![vec![Nokta::yeni(10.0, 20.0), Nokta::yeni(bitiş_x, 80.0)]],
            renk: renk.to_owned(),
            kalınlık,
        });
        sahne
    }

    fn önbelleğe_örnek_yol_ekle(önbellek: &mut GpuiYolÖnbelleği) {
        önbellek.yollar = vec![Some(Path::new(point(px(0.0), px(0.0))))];
    }

    #[test]
    fn hover_katmanı_ana_sahne_geometrisini_değiştirmez() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = crate::kart::resize_kartı(100)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let mut bileşen = GpuiGrafik::yeni(grafik);
        let ana_komut_sayısı = bileşen.ana_sahne.komutlar().len();
        assert!(ana_komut_sayısı > 0);
        assert!(bileşen.etkileşim_sahnesi().komutlar().is_empty());

        let en_yakın = bileşen.grafik.en_yakın_noktalar(0.5);
        assert!(en_yakın.is_some());
        let Some((veri_x, seri_değerleri)) = en_yakın else {
            return Ok(());
        };
        let (sol, sağ, üst, alt) = bileşen.çizim_alanı();
        bileşen.imleç = Some(İmleçDurumu {
            fare: Nokta::yeni((sol + sağ) / 2.0, (üst + alt) / 2.0),
            veri_x,
            seri_x_değerleri: vec![Some(veri_x); seri_değerleri.len()],
            seri_değerleri,
            dağılım: None,
        });

        assert!(!bileşen.etkileşim_sahnesi().komutlar().is_empty());
        assert_eq!(bileşen.ana_sahne.komutlar().len(), ana_komut_sayısı);
        Ok(())
    }

    #[test]
    fn dikey_x_yüzeyi_imleci_ve_xy_seçimini_fiziksel_yönelimde_çizer() -> Result<(), UplotHatası> {
        let (seçenekler, veri) =
            crate::kart::scales_dir_ori_kartı(crate::kart::ScalesDirOriÖrneği::XArtıSolYArtıÜst)?;
        assert!(seçenekler.etkileşimler.seçim_xy_yakınlaştır);
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let mut bileşen = GpuiGrafik::yeni(grafik);
        let (sol, sağ, üst, alt) = bileşen.çizim_alanı();
        let veri_x = 1.0;
        let veri_y = 0.0;
        let x_oranı = bileşen.grafik.x_konum_oranı(veri_x).unwrap_or(0.5) as f32;
        let y_oranı = bileşen.grafik.seri_y_konum_oranı(0, veri_y).unwrap_or(0.5) as f32;
        bileşen.imleç = Some(İmleçDurumu {
            fare: Nokta::yeni(sol + y_oranı * (sağ - sol), alt - x_oranı * (alt - üst)),
            veri_x,
            seri_x_değerleri: vec![Some(veri_x), Some(veri_x)],
            seri_değerleri: vec![Some(veri_y), Some(2.0)],
            dağılım: None,
        });
        let imleç_sahnesi = bileşen.etkileşim_sahnesi();
        assert!(imleç_sahnesi.komutlar().iter().any(|komut| matches!(
            komut,
            Komut::KesikliÇizgi { başlangıç, bitiş, .. }
                if (başlangıç.y - bitiş.y).abs() <= f32::EPSILON
        )));
        assert!(imleç_sahnesi.komutlar().iter().any(|komut| matches!(
            komut,
            Komut::KesikliÇizgi { başlangıç, bitiş, .. }
                if (başlangıç.x - bitiş.x).abs() <= f32::EPSILON
        )));

        bileşen.imleç = None;
        bileşen.seçim = Some((Nokta::yeni(90.0, 120.0), Nokta::yeni(190.0, 320.0)));
        let seçim_sahnesi = bileşen.etkileşim_sahnesi();
        assert!(seçim_sahnesi.komutlar().iter().any(|komut| matches!(
            komut,
            Komut::Dikdörtgen { genişlik, yükseklik, .. }
                if (*genişlik - 100.0).abs() <= f32::EPSILON
                    && (*yükseklik - 200.0).abs() <= f32::EPSILON
        )));
        Ok(())
    }

    #[test]
    fn annotation_hover_yalnız_etkileşim_sahnesini_değiştirir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = crate::kart::annotations_kartı()?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let mut bileşen = GpuiGrafik::yeni(grafik);
        let ana_sahne = bileşen.ana_sahne.clone();
        let (sol, sağ, _, alt) = bileşen.çizim_alanı();
        let deprem_x = sol + (sağ - sol) * (3.0 / 29.0);
        bileşen.açıklama_vuruşu = bileşen.grafik.açıklama_vuruşu_boyutta(
            bileşen.grafik.boyut().0,
            bileşen.grafik.boyut().1,
            deprem_x,
            alt - 9.0,
        );

        assert!(bileşen.açıklama_vuruşu.is_some());
        assert!(!bileşen.etkileşim_sahnesi().komutlar().is_empty());
        assert_eq!(bileşen.ana_sahne, ana_sahne);
        Ok(())
    }

    #[test]
    fn gpui_css_adlı_ve_modern_rgb_renklerini_korur() {
        let kırmızı = renk_çöz("red").to_rgb();
        assert!((kırmızı.r - 1.0).abs() < f32::EPSILON);
        assert!(kırmızı.g.abs() < f32::EPSILON);
        assert!(kırmızı.b.abs() < f32::EPSILON);
        assert!((kırmızı.a - 1.0).abs() < f32::EPSILON);

        let annotation = renk_çöz("rgb(255 193 7 / 20%)").to_rgb();
        assert!((annotation.r - 1.0).abs() < f32::EPSILON);
        assert!((annotation.g - 193.0 / 255.0).abs() < 0.0001);
        assert!((annotation.b - 7.0 / 255.0).abs() < 0.0001);
        assert!((annotation.a - 0.2).abs() < 0.0001);

        let eski = renk_çöz("rgba(255,0,0,0.1)").to_rgb();
        assert!((eski.a - 26.0 / 255.0).abs() < 0.0001);
        assert_eq!(renk_çöz("#0f08"), renk_çöz("rgba(0,255,0,0.5333333)"));
    }

    #[test]
    fn gpui_yol_önbelleği_renk_değişiminde_geometriyi_korur() {
        let eski = yol_sahnesi("#ff0000", 2.0, 200.0);
        let yeni = yol_sahnesi("#0000ff", 2.0, 200.0);
        let mut önbellek = GpuiYolÖnbelleği {
            sahne_boyutu: Some(eski.boyut()),
            sınırlar: None,
            yollar: Vec::new(),
        };
        önbelleğe_örnek_yol_ekle(&mut önbellek);

        assert_eq!(önbellek.sahneyi_değiştir(&eski, &yeni), 1);
        assert!(önbellek.yollar.first().is_some_and(Option::is_some));
    }

    #[test]
    fn gpui_yol_önbelleği_geometri_ve_kalınlık_değişiminde_geçersizleşir() {
        let eski = yol_sahnesi("#ff0000", 2.0, 200.0);
        for yeni in [
            yol_sahnesi("#ff0000", 3.0, 200.0),
            yol_sahnesi("#ff0000", 2.0, 240.0),
        ] {
            let mut önbellek = GpuiYolÖnbelleği {
                sahne_boyutu: Some(eski.boyut()),
                sınırlar: None,
                yollar: Vec::new(),
            };
            önbelleğe_örnek_yol_ekle(&mut önbellek);

            assert_eq!(önbellek.sahneyi_değiştir(&eski, &yeni), 0);
            assert!(önbellek.yollar.first().is_some_and(Option::is_none));
        }
    }

    #[test]
    fn gpui_yol_önbelleği_yüzey_değişiminde_geçersizleşir() {
        let sahne = yol_sahnesi("#ff0000", 2.0, 200.0);
        let ilk_sınırlar = Bounds::new(point(px(0.0), px(0.0)), size(px(320.0), px(180.0)));
        let yeni_sınırlar = Bounds::new(point(px(1.0), px(0.0)), size(px(320.0), px(180.0)));
        let mut önbellek = GpuiYolÖnbelleği::default();
        önbellek.yüzeyi_hazırla(&sahne, ilk_sınırlar);
        önbelleğe_örnek_yol_ekle(&mut önbellek);

        önbellek.yüzeyi_hazırla(&sahne, yeni_sınırlar);

        assert!(önbellek.yollar.first().is_some_and(Option::is_none));
    }
}
