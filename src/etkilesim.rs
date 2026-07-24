use std::collections::VecDeque;
use web_time::{Duration, Instant};

use crate::{Aralık, EtkileşimSeçenekleri, TekerlekEkseni, TekerlekKipi, UplotHatası};

#[derive(Clone, Copy, Default, PartialEq)]
struct Görünüm {
    x: Option<Aralık>,
    y: Option<Aralık>,
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

    pub(crate) fn yakınlaştırılmış(&self) -> bool {
        self.görünüm != Görünüm::default()
    }

    pub(crate) fn geri_var(&self) -> bool {
        self.ayarlar.görünüm_geçmişi && !self.geçmiş.is_empty()
    }

    pub(crate) fn ayarlar(&self) -> EtkileşimSeçenekleri {
        self.ayarlar
    }

    pub(crate) fn tekerlek_etkileşimi_ayarla(&mut self, etkin: bool) {
        self.ayarlar.tekerlek_etkileşimi = etkin;
        self.tekerleği_sıfırla();
    }

    pub(crate) fn seçim_yakınlaştır(
        &mut self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
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
        let uzunluk = mevcut.en_çok - mevcut.en_az;
        let yeni = Aralık::yeni(
            mevcut.en_az + en_az_oran * uzunluk,
            mevcut.en_az + en_çok_oran * uzunluk,
        )?;
        Ok(self.uygula(
            Görünüm {
                x: Some(yeni),
                y: self.görünüm.y,
            },
            true,
        ))
    }

    pub(crate) fn tekerlek(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        görünür_y: Aralık,
        ham_delta: f64,
        platform_hassas: bool,
        eksen: TekerlekEkseni,
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
        let şimdi = Instant::now();
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
        let x_odak =
            mevcut_x.en_az + yatay_odak_oranı.clamp(0.0, 1.0) * (mevcut_x.en_çok - mevcut_x.en_az);
        let x = if matches!(eksen, TekerlekEkseni::İkisi | TekerlekEkseni::X) {
            mevcut_x.uyarlanabilir_tekerlek_yakınlaştır(
                self.tam_x, x_odak, delta, hassas, tekerlek,
            )?
        } else {
            mevcut_x
        };
        let mevcut_y = self.görünüm.y.unwrap_or(görünür_y);
        let y_odak = mevcut_y.en_az
            + (1.0 - dikey_odak_oranı.clamp(0.0, 1.0)) * (mevcut_y.en_çok - mevcut_y.en_az);
        let y = if matches!(eksen, TekerlekEkseni::İkisi | TekerlekEkseni::Y) {
            mevcut_y.uyarlanabilir_tekerlek_yakınlaştır(
                self.tam_y, y_odak, delta, hassas, tekerlek,
            )?
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
        let x = kaydır(
            başlangıç.x,
            self.tam_x,
            -yatay_fark_oranı * (başlangıç.x.en_çok - başlangıç.x.en_az),
        )?;
        let y = kaydır(
            başlangıç.y,
            self.tam_y,
            dikey_fark_oranı * (başlangıç.y.en_çok - başlangıç.y.en_az),
        )?;
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
    ) -> Result<bool, UplotHatası> {
        if !self.dokunma_sürüyor
            || !yatay_odak_oranı.is_finite()
            || !dikey_odak_oranı.is_finite()
            || !çarpan.is_finite()
            || çarpan <= 0.0
        {
            return Ok(false);
        }
        let x = odakta_yakınlaştır(self.görünür_x(), self.tam_x, yatay_odak_oranı, çarpan)?;
        let y = odakta_yakınlaştır(
            self.görünüm
                .y
                .or(self.dokunma_başlangıç_y)
                .unwrap_or(self.tam_y),
            self.tam_y,
            1.0 - dikey_odak_oranı,
            çarpan,
        )?;
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

    fn uygula(&mut self, yeni: Görünüm, geçmişe_ekle: bool) -> bool {
        if self.görünüm == yeni {
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
