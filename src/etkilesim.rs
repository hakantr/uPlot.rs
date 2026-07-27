use std::collections::VecDeque;
use web_time::{Duration, Instant};

use crate::{
    Aralık, EtkileşimSeçenekleri, TekerlekEkseni, TekerlekKipi, UplotHatası, XÖlçekDağılımı,
    YÖlçekDağılımı,
};

#[derive(Clone, Copy, Default, PartialEq)]
struct Görünüm {
    x: Option<Aralık>,
    y: Option<Aralık>,
}

fn aralık_yaklaşık_eşit(sol: Aralık, sağ: Aralık) -> bool {
    let ölçek = (sol.en_çok - sol.en_az)
        .abs()
        .max((sağ.en_çok - sağ.en_az).abs())
        .max(f64::MIN_POSITIVE);
    let tolerans = ölçek * f64::from(f32::EPSILON) * 4.0;
    (sol.en_az - sağ.en_az).abs() <= tolerans && (sol.en_çok - sağ.en_çok).abs() <= tolerans
}

fn görünüm_yaklaşık_eşit(sol: Görünüm, sağ: Görünüm) -> bool {
    let aralıklar_aynı = |sol: Option<Aralık>, sağ: Option<Aralık>| match (sol, sağ) {
        (Some(sol), Some(sağ)) => aralık_yaklaşık_eşit(sol, sağ),
        (None, None) => true,
        _ => false,
    };
    aralıklar_aynı(sol.x, sağ.x) && aralıklar_aynı(sol.y, sağ.y)
}

#[derive(Clone, Copy)]
struct TaşımaBaşlangıcı {
    görünüm: Görünüm,
    x: Aralık,
    y: Aralık,
    geçmişe_eklendi: bool,
}

/// Bir grafiğin görünümünü ve kartta açılan etkileşimlerin bütün durumunu taşır.
/// Yüzey adaptörleri yalnız normalize koordinat ve ham platform deltasını iletir.
pub(crate) struct EtkileşimDenetleyicisi {
    tam_x: Aralık,
    tam_y: Aralık,
    görünüm: Görünüm,
    ayarlar: EtkileşimSeçenekleri,
    geçmiş: VecDeque<Görünüm>,
    son_tekerlek: Option<Instant>,
    tekerlek_hareketi_kaydedildi: bool,
    birikmiş_hassas_delta: f64,
    taşıma: Option<TaşımaBaşlangıcı>,
    dokunma_sürüyor: bool,
    dokunma_hareketi_kaydedildi: bool,
    dokunma_başlangıç_y: Option<Aralık>,
}

impl EtkileşimDenetleyicisi {
    pub(crate) fn yeni(tam_x: Aralık, tam_y: Aralık, ayarlar: EtkileşimSeçenekleri) -> Self {
        Self {
            tam_x,
            tam_y,
            görünüm: Görünüm::default(),
            ayarlar,
            geçmiş: VecDeque::new(),
            son_tekerlek: None,
            tekerlek_hareketi_kaydedildi: false,
            birikmiş_hassas_delta: 0.0,
            taşıma: None,
            dokunma_sürüyor: false,
            dokunma_hareketi_kaydedildi: false,
            dokunma_başlangıç_y: None,
        }
    }

    pub(crate) fn görünür_x(&self) -> Aralık {
        self.görünüm.x.unwrap_or(self.tam_x)
    }

    pub(crate) fn tam_x(&self) -> Aralık {
        self.tam_x
    }

    pub(crate) fn canlı_tam_x_ayarla(&mut self, aralık: Aralık) -> bool {
        if self.tam_x == aralık {
            return false;
        }
        self.tam_x = aralık;
        self.görünüm.x.is_none()
    }

    pub(crate) fn canlı_tam_y_ayarla(&mut self, aralık: Aralık) -> bool {
        if self.tam_y == aralık {
            return false;
        }
        self.tam_y = aralık;
        self.görünüm.y.is_none()
    }

    pub(crate) fn tam_y(&self) -> Aralık {
        self.tam_y
    }

    pub(crate) fn görünür_x_ayarla(&mut self, aralık: Aralık) -> bool {
        let yeni = (aralık != self.tam_x).then_some(aralık);
        if self.görünüm.x == yeni {
            return false;
        }
        self.görünüm.x = yeni;
        true
    }

    pub(crate) fn görünür_y(&self) -> Option<Aralık> {
        self.görünüm.y
    }

    pub(crate) fn görünür_y_ayarla(&mut self, aralık: Aralık) -> bool {
        let yeni = (aralık != self.tam_y).then_some(aralık);
        if self.görünüm.y == yeni {
            return false;
        }
        self.görünüm.y = yeni;
        true
    }

    pub(crate) fn görünür_aralıkları_ayarla(
        &mut self,
        x: Aralık,
        y: Aralık,
        geçmişe_ekle: bool,
    ) -> bool {
        self.uygula(
            Görünüm {
                x: (x != self.tam_x).then_some(x),
                y: (y != self.tam_y).then_some(y),
            },
            geçmişe_ekle,
        )
    }

    pub(crate) fn görünür_x_aralığını_ayarla(
        &mut self,
        x: Aralık,
        geçmişe_ekle: bool,
    ) -> bool {
        self.uygula(
            Görünüm {
                x: (x != self.tam_x).then_some(x),
                y: self.görünüm.y,
            },
            geçmişe_ekle,
        )
    }

    pub(crate) fn yakınlaştırılmış(&self) -> bool {
        self.görünüm != Görünüm::default()
    }

    pub(crate) fn geri_var(&self) -> bool {
        self.ayarlar.görünüm_geçmişi && !self.geçmiş.is_empty()
    }

    pub(crate) fn ayarlar(&self) -> EtkileşimSeçenekleri {
        self.ayarlar
    }

    pub(crate) fn tekerlek_etkileşimi_ayarla(&mut self, etkin: bool) -> bool {
        if self.ayarlar.tekerlek_etkileşimi == etkin {
            return false;
        }
        self.ayarlar.tekerlek_etkileşimi = etkin;
        self.tekerleği_sıfırla();
        true
    }

    pub(crate) fn tekerlek_odaksız_etkileşimi_ayarla(&mut self, etkin: bool) -> bool {
        if self.ayarlar.tekerlek_odaksız_etkileşim == etkin {
            return false;
        }
        self.ayarlar.tekerlek_odaksız_etkileşim = etkin;
        self.tekerleği_sıfırla();
        true
    }

    pub(crate) fn seçim_yakınlaştır(
        &mut self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
        x_dağılımı: XÖlçekDağılımı,
    ) -> Result<bool, UplotHatası> {
        if !self.ayarlar.seçim_yakınlaştır {
            return Ok(false);
        }
        if !başlangıç_oranı.is_finite() || !bitiş_oranı.is_finite() {
            return Err(UplotHatası::GeçersizAralık {
                en_az: başlangıç_oranı,
                en_çok: bitiş_oranı,
            });
        }
        let başlangıç = başlangıç_oranı.clamp(0.0, 1.0);
        let bitiş = bitiş_oranı.clamp(0.0, 1.0);
        let (en_az_oran, en_çok_oran) = if başlangıç <= bitiş {
            (başlangıç, bitiş)
        } else {
            (bitiş, başlangıç)
        };
        let mevcut = self.görünür_x();
        let dönüştürülmüş = x_aralığını_dönüştür(mevcut, x_dağılımı).unwrap_or(mevcut);
        let uzunluk = dönüştürülmüş.en_çok - dönüştürülmüş.en_az;
        let yeni = Aralık::yeni(
            dönüştürülmüş.en_az + en_az_oran * uzunluk,
            dönüştürülmüş.en_az + en_çok_oran * uzunluk,
        )?;
        let yeni = x_aralığını_geri_dönüştür(yeni, x_dağılımı).unwrap_or(yeni);
        Ok(self.uygula(
            Görünüm {
                x: Some(yeni),
                y: self.görünüm.y,
            },
            true,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn tekerlek(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        görünür_y: Aralık,
        ham_delta: f64,
        platform_hassas: bool,
        eksen: TekerlekEkseni,
        x_dağılımı: XÖlçekDağılımı,
        y_dağılımı: YÖlçekDağılımı,
        şimdi: Instant,
    ) -> Result<bool, UplotHatası> {
        if !self.ayarlar.tekerlek_etkileşimi {
            return Ok(false);
        }
        if !yatay_odak_oranı.is_finite()
            || !dikey_odak_oranı.is_finite()
            || !ham_delta.is_finite()
            || ham_delta.abs() <= f64::EPSILON
        {
            return Ok(false);
        }

        let tekerlek = self.ayarlar.tekerlek_ayarları;
        let hassas = match tekerlek.kip {
            TekerlekKipi::Otomatik => platform_hassas,
            TekerlekKipi::Ayrık => false,
            TekerlekKipi::Hassas => true,
        };
        let yeni_hareket = self.son_tekerlek.is_none_or(|önceki| {
            şimdi.duration_since(önceki) >= Duration::from_millis(tekerlek.hareket_birleştirme_ms)
        });
        if yeni_hareket {
            self.tekerlek_hareketi_kaydedildi = false;
            self.birikmiş_hassas_delta = 0.0;
        }
        self.son_tekerlek = Some(şimdi);

        let delta = if hassas {
            if self.birikmiş_hassas_delta.signum() != ham_delta.signum() {
                self.birikmiş_hassas_delta = 0.0;
            }
            self.birikmiş_hassas_delta += ham_delta;
            if self.birikmiş_hassas_delta.abs() < tekerlek.hassas_ölü_bölge {
                return Ok(false);
            }
            let birikmiş = self.birikmiş_hassas_delta;
            self.birikmiş_hassas_delta = 0.0;
            birikmiş
        } else {
            self.birikmiş_hassas_delta = 0.0;
            ham_delta
        };

        let mevcut_x = self.görünür_x();
        let x = if matches!(eksen, TekerlekEkseni::İkisi | TekerlekEkseni::X) {
            let dönüştürülmüş = x_aralığını_dönüştür(mevcut_x, x_dağılımı).unwrap_or(mevcut_x);
            let dönüştürülmüş_tam =
                x_aralığını_dönüştür(self.tam_x, x_dağılımı).unwrap_or(self.tam_x);
            let x_odak = dönüştürülmüş.en_az
                + yatay_odak_oranı.clamp(0.0, 1.0) * (dönüştürülmüş.en_çok - dönüştürülmüş.en_az);
            let yeni = dönüştürülmüş.uyarlanabilir_tekerlek_yakınlaştır(
                dönüştürülmüş_tam,
                x_odak,
                delta,
                hassas,
                tekerlek,
            )?;
            x_aralığını_geri_dönüştür(yeni, x_dağılımı).unwrap_or(yeni)
        } else {
            mevcut_x
        };
        let mevcut_y = self.görünüm.y.unwrap_or(görünür_y);
        let y = if matches!(eksen, TekerlekEkseni::İkisi | TekerlekEkseni::Y) {
            let dönüştürülmüş = y_aralığını_dönüştür(mevcut_y, y_dağılımı).unwrap_or(mevcut_y);
            let dönüştürülmüş_tam =
                y_aralığını_dönüştür(self.tam_y, y_dağılımı).unwrap_or(self.tam_y);
            let y_odak = dönüştürülmüş.en_az
                + (1.0 - dikey_odak_oranı.clamp(0.0, 1.0))
                    * (dönüştürülmüş.en_çok - dönüştürülmüş.en_az);
            let yeni = dönüştürülmüş.uyarlanabilir_tekerlek_yakınlaştır(
                dönüştürülmüş_tam,
                y_odak,
                delta,
                hassas,
                tekerlek,
            )?;
            y_aralığını_geri_dönüştür(yeni, y_dağılımı).unwrap_or(yeni)
        } else {
            mevcut_y
        };
        let y_görünümü = if y != self.tam_y || (eksen == TekerlekEkseni::X && x != self.tam_x) {
            Some(y)
        } else {
            None
        };
        let yeni = Görünüm {
            x: (x != self.tam_x).then_some(x),
            y: y_görünümü,
        };
        let değişti = self.uygula(yeni, !self.tekerlek_hareketi_kaydedildi);
        if değişti {
            self.tekerlek_hareketi_kaydedildi = true;
        }
        Ok(değişti)
    }

    pub(crate) fn tam_görünüm(&mut self) -> bool {
        if !self.ayarlar.çift_tıkla_tam_görünüm {
            return false;
        }
        self.tekerleği_sıfırla();
        self.taşıma = None;
        self.dokunmayı_bitir();
        self.uygula(Görünüm::default(), true)
    }

    pub(crate) fn geri(&mut self) -> bool {
        if !self.ayarlar.görünüm_geçmişi {
            return false;
        }
        let Some(önceki) = self.geçmiş.pop_back() else {
            return false;
        };
        self.görünüm = önceki;
        self.taşıma = None;
        self.dokunmayı_bitir();
        self.tekerleği_sıfırla();
        true
    }

    pub(crate) fn taşımayı_başlat(&mut self, görünür_y: Aralık) -> bool {
        if !self.yakınlaştırılmış() {
            return false;
        }
        self.tekerleği_sıfırla();
        self.taşıma = Some(TaşımaBaşlangıcı {
            görünüm: self.görünüm,
            x: self.görünür_x(),
            y: görünür_y,
            geçmişe_eklendi: false,
        });
        true
    }

    pub(crate) fn taşı(
        &mut self,
        yatay_fark_oranı: f64,
        dikey_fark_oranı: f64,
        x_dağılımı: XÖlçekDağılımı,
        y_dağılımı: YÖlçekDağılımı,
    ) -> Result<bool, UplotHatası> {
        if !yatay_fark_oranı.is_finite() || !dikey_fark_oranı.is_finite() {
            return Err(UplotHatası::GeçersizAralık {
                en_az: yatay_fark_oranı,
                en_çok: dikey_fark_oranı,
            });
        }
        let Some(mut başlangıç) = self.taşıma else {
            return Ok(false);
        };
        let dönüştürülmüş_x = x_aralığını_dönüştür(başlangıç.x, x_dağılımı).unwrap_or(başlangıç.x);
        let dönüştürülmüş_tam_x =
            x_aralığını_dönüştür(self.tam_x, x_dağılımı).unwrap_or(self.tam_x);
        let x = kaydır(
            dönüştürülmüş_x,
            dönüştürülmüş_tam_x,
            -yatay_fark_oranı * (dönüştürülmüş_x.en_çok - dönüştürülmüş_x.en_az),
        )?;
        let x = x_aralığını_geri_dönüştür(x, x_dağılımı).unwrap_or(x);
        let dönüştürülmüş = y_aralığını_dönüştür(başlangıç.y, y_dağılımı).unwrap_or(başlangıç.y);
        let dönüştürülmüş_tam = y_aralığını_dönüştür(self.tam_y, y_dağılımı).unwrap_or(self.tam_y);
        let y = kaydır(
            dönüştürülmüş,
            dönüştürülmüş_tam,
            dikey_fark_oranı * (dönüştürülmüş.en_çok - dönüştürülmüş.en_az),
        )?;
        let y = y_aralığını_geri_dönüştür(y, y_dağılımı).unwrap_or(y);
        let yeni = Görünüm {
            x: (x != self.tam_x).then_some(x),
            y: (y != self.tam_y).then_some(y),
        };
        if yeni == self.görünüm {
            return Ok(false);
        }
        if !başlangıç.geçmişe_eklendi && self.ayarlar.görünüm_geçmişi {
            if self.geçmiş.len() >= 100 {
                self.geçmiş.pop_front();
            }
            self.geçmiş.push_back(başlangıç.görünüm);
            başlangıç.geçmişe_eklendi = true;
        }
        self.taşıma = Some(başlangıç);
        self.görünüm = yeni;
        Ok(true)
    }

    pub(crate) fn taşımayı_bitir(&mut self) {
        self.taşıma = None;
    }

    pub(crate) fn dokunmayı_başlat(&mut self, görünür_y: Aralık) -> bool {
        if !self.ayarlar.dokunma_etkileşimi {
            return false;
        }
        self.dokunma_sürüyor = true;
        self.dokunma_hareketi_kaydedildi = false;
        self.dokunma_başlangıç_y = Some(görünür_y);
        self.taşıma = None;
        self.tekerleği_sıfırla();
        true
    }

    pub(crate) fn dokunma_yakınlaştır(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        çarpan: f64,
        x_dağılımı: XÖlçekDağılımı,
        y_dağılımı: YÖlçekDağılımı,
    ) -> Result<bool, UplotHatası> {
        if !self.dokunma_sürüyor
            || !yatay_odak_oranı.is_finite()
            || !dikey_odak_oranı.is_finite()
            || !çarpan.is_finite()
            || çarpan <= 0.0
        {
            return Ok(false);
        }
        let mevcut_x = self.görünür_x();
        let dönüştürülmüş_x = x_aralığını_dönüştür(mevcut_x, x_dağılımı).unwrap_or(mevcut_x);
        let dönüştürülmüş_tam_x =
            x_aralığını_dönüştür(self.tam_x, x_dağılımı).unwrap_or(self.tam_x);
        let x = odakta_yakınlaştır(
            dönüştürülmüş_x,
            dönüştürülmüş_tam_x,
            yatay_odak_oranı,
            çarpan,
        )?;
        let x = x_aralığını_geri_dönüştür(x, x_dağılımı).unwrap_or(x);
        let mevcut_y = self
            .görünüm
            .y
            .or(self.dokunma_başlangıç_y)
            .unwrap_or(self.tam_y);
        let dönüştürülmüş = y_aralığını_dönüştür(mevcut_y, y_dağılımı).unwrap_or(mevcut_y);
        let dönüştürülmüş_tam = y_aralığını_dönüştür(self.tam_y, y_dağılımı).unwrap_or(self.tam_y);
        let y = odakta_yakınlaştır(
            dönüştürülmüş,
            dönüştürülmüş_tam,
            1.0 - dikey_odak_oranı,
            çarpan,
        )?;
        let y = y_aralığını_geri_dönüştür(y, y_dağılımı).unwrap_or(y);
        let yeni = Görünüm {
            x: (x != self.tam_x).then_some(x),
            y: (y != self.tam_y).then_some(y),
        };
        let değişti = self.uygula(yeni, !self.dokunma_hareketi_kaydedildi);
        if değişti {
            self.dokunma_hareketi_kaydedildi = true;
        }
        Ok(değişti)
    }

    pub(crate) fn dokunmayı_bitir(&mut self) {
        self.dokunma_sürüyor = false;
        self.dokunma_hareketi_kaydedildi = false;
        self.dokunma_başlangıç_y = None;
    }

    fn uygula(&mut self, mut yeni: Görünüm, geçmişe_ekle: bool) -> bool {
        if yeni.x.is_some_and(|x| aralık_yaklaşık_eşit(x, self.tam_x)) {
            yeni.x = None;
        }
        if yeni.y.is_some_and(|y| aralık_yaklaşık_eşit(y, self.tam_y)) {
            yeni.y = None;
        }
        // Grup görünümü normalize f32 oranlardan geri açılır. Aynı pencerenin
        // bu gidiş-dönüşte kazandığı alt-piksel yuvarlama farkını yeni bir
        // etkileşim saymak sync grubunda olay yankısı ve sahte geçmiş üretir.
        if görünüm_yaklaşık_eşit(self.görünüm, yeni) {
            return false;
        }
        if geçmişe_ekle && self.ayarlar.görünüm_geçmişi {
            if self.geçmiş.len() >= 100 {
                self.geçmiş.pop_front();
            }
            self.geçmiş.push_back(self.görünüm);
        }
        self.görünüm = yeni;
        true
    }

    fn tekerleği_sıfırla(&mut self) {
        self.son_tekerlek = None;
        self.tekerlek_hareketi_kaydedildi = false;
        self.birikmiş_hassas_delta = 0.0;
    }
}

pub(crate) fn x_aralığını_dönüştür(
    aralık: Aralık,
    dağılım: XÖlçekDağılımı,
) -> Option<Aralık> {
    match dağılım {
        XÖlçekDağılımı::Doğrusal => Some(aralık),
        XÖlçekDağılımı::Logaritmik { taban } if aralık.en_az > 0.0 && taban > 1.0 => {
            Aralık::yeni(aralık.en_az.log(taban), aralık.en_çok.log(taban)).ok()
        }
        _ => None,
    }
}

pub(crate) fn x_aralığını_geri_dönüştür(
    aralık: Aralık,
    dağılım: XÖlçekDağılımı,
) -> Option<Aralık> {
    match dağılım {
        XÖlçekDağılımı::Doğrusal => Some(aralık),
        XÖlçekDağılımı::Logaritmik { taban } if taban > 1.0 => {
            Aralık::yeni(taban.powf(aralık.en_az), taban.powf(aralık.en_çok)).ok()
        }
        _ => None,
    }
}

pub(crate) fn y_aralığını_dönüştür(
    aralık: Aralık,
    dağılım: YÖlçekDağılımı,
) -> Option<Aralık> {
    Aralık::yeni(
        y_değerini_dönüştür(aralık.en_az, dağılım)?,
        y_değerini_dönüştür(aralık.en_çok, dağılım)?,
    )
    .ok()
}

pub(crate) fn y_aralığını_geri_dönüştür(
    aralık: Aralık,
    dağılım: YÖlçekDağılımı,
) -> Option<Aralık> {
    Aralık::yeni(
        y_değerini_geri_dönüştür(aralık.en_az, dağılım)?,
        y_değerini_geri_dönüştür(aralık.en_çok, dağılım)?,
    )
    .ok()
}

pub(crate) fn y_değerini_dönüştür(
    değer: f64, dağılım: YÖlçekDağılımı
) -> Option<f64> {
    match dağılım {
        YÖlçekDağılımı::Doğrusal => Some(değer),
        YÖlçekDağılımı::Logaritmik { taban } if değer > 0.0 && taban > 1.0 => {
            Some(değer.log(taban))
        }
        YÖlçekDağılımı::ArcSinh { eşik } if eşik > 0.0 => Some((değer / eşik).asinh()),
        YÖlçekDağılımı::Weibull if değer > 0.0 && değer < 1.0 => {
            Some((-(-değer).ln_1p()).ln())
        }
        YÖlçekDağılımı::Özel(dönüşüm) => (dönüşüm.ileri)(değer),
        _ => None,
    }
}

pub(crate) fn y_değerini_geri_dönüştür(
    değer: f64, dağılım: YÖlçekDağılımı
) -> Option<f64> {
    match dağılım {
        YÖlçekDağılımı::Doğrusal => Some(değer),
        YÖlçekDağılımı::Logaritmik { taban } if taban > 1.0 => Some(taban.powf(değer)),
        YÖlçekDağılımı::ArcSinh { eşik } if eşik > 0.0 => Some(değer.sinh() * eşik),
        YÖlçekDağılımı::Weibull => Some(1.0 - (-değer.exp()).exp()),
        YÖlçekDağılımı::Özel(dönüşüm) => (dönüşüm.geri)(değer),
        _ => None,
    }
}

fn kaydır(mevcut: Aralık, tam: Aralık, fark: f64) -> Result<Aralık, UplotHatası> {
    let uzunluk = mevcut.en_çok - mevcut.en_az;
    let tam_uzunluk = tam.en_çok - tam.en_az;
    if uzunluk >= tam_uzunluk {
        return Ok(tam);
    }
    let mut en_az = mevcut.en_az + fark;
    let mut en_çok = mevcut.en_çok + fark;
    if en_az < tam.en_az {
        en_az = tam.en_az;
        en_çok = tam.en_az + uzunluk;
    } else if en_çok > tam.en_çok {
        en_çok = tam.en_çok;
        en_az = tam.en_çok - uzunluk;
    }
    Aralık::yeni(en_az, en_çok)
}

fn odakta_yakınlaştır(
    mevcut: Aralık,
    tam: Aralık,
    odak_oranı: f64,
    çarpan: f64,
) -> Result<Aralık, UplotHatası> {
    let mevcut_uzunluk = mevcut.en_çok - mevcut.en_az;
    let tam_uzunluk = tam.en_çok - tam.en_az;
    let yeni_uzunluk = (mevcut_uzunluk / çarpan).clamp(tam_uzunluk / 10_000.0, tam_uzunluk);
    let oran = odak_oranı.clamp(0.0, 1.0);
    let odak = mevcut.en_az + oran * mevcut_uzunluk;
    let aday = Aralık::yeni(
        odak - oran * yeni_uzunluk,
        odak + (1.0 - oran) * yeni_uzunluk,
    )?;
    kaydır(aday, tam, 0.0)
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn f32_sync_gidiş_dönüşü_yeni_görünüm_ve_geçmiş_üretmez() {
        let tam = Aralık {
            en_az: 1_700_000_000.0,
            en_çok: 1_700_086_400.0,
        };
        let mut denetleyici = EtkileşimDenetleyicisi::yeni(
            tam,
            Aralık {
                en_az: 0.0,
                en_çok: 100.0,
            },
            Default::default(),
        );
        let görünür = Aralık {
            en_az: tam.en_az + 12_345.678,
            en_çok: tam.en_çok - 23_456.789,
        };
        assert!(denetleyici.görünür_aralıkları_ayarla(
            görünür,
            Aralık {
                en_az: 10.0,
                en_çok: 90.0
            },
            false,
        ));
        let sol = ((görünür.en_az - tam.en_az) / (tam.en_çok - tam.en_az)) as f32;
        let sağ = ((görünür.en_çok - tam.en_az) / (tam.en_çok - tam.en_az)) as f32;
        let f32_gidiş_dönüşü = Aralık {
            en_az: tam.en_az + f64::from(sol) * (tam.en_çok - tam.en_az),
            en_çok: tam.en_az + f64::from(sağ) * (tam.en_çok - tam.en_az),
        };

        assert!(!denetleyici.görünür_aralıkları_ayarla(
            f32_gidiş_dönüşü,
            Aralık {
                en_az: 10.0,
                en_çok: 90.0
            },
            true,
        ));
        assert!(!denetleyici.geri_var());
    }
}
