use std::collections::VecDeque;
use web_time::{Duration, Instant};

use crate::{Aralık, EtkileşimSeçenekleri, TekerlekKipi, UplotHatası};

/// Bir grafiğin görünümünü ve kartta açılan etkileşimlerin bütün durumunu taşır.
/// Yüzey adaptörleri yalnız normalize koordinat ve ham platform deltasını iletir.
pub(crate) struct EtkileşimDenetleyicisi {
    tam: Aralık,
    görünür: Option<Aralık>,
    ayarlar: EtkileşimSeçenekleri,
    geçmiş: VecDeque<Option<Aralık>>,
    son_tekerlek: Option<Instant>,
    tekerlek_hareketi_kaydedildi: bool,
    birikmiş_hassas_delta: f64,
}

impl EtkileşimDenetleyicisi {
    pub(crate) fn yeni(tam: Aralık, ayarlar: EtkileşimSeçenekleri) -> Self {
        Self {
            tam,
            görünür: None,
            ayarlar,
            geçmiş: VecDeque::new(),
            son_tekerlek: None,
            tekerlek_hareketi_kaydedildi: false,
            birikmiş_hassas_delta: 0.0,
        }
    }

    pub(crate) fn görünür(&self) -> Aralık {
        self.görünür.unwrap_or(self.tam)
    }

    pub(crate) fn yakınlaştırılmış(&self) -> bool {
        self.görünür.is_some()
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
        let mevcut = self.görünür();
        let uzunluk = mevcut.en_çok - mevcut.en_az;
        let yeni = Aralık::yeni(
            mevcut.en_az + en_az_oran * uzunluk,
            mevcut.en_az + en_çok_oran * uzunluk,
        )?;
        Ok(self.uygula(Some(yeni), true))
    }

    pub(crate) fn tekerlek(
        &mut self,
        odak_oranı: f64,
        ham_delta: f64,
        platform_hassas: bool,
    ) -> Result<bool, UplotHatası> {
        if !self.ayarlar.tekerlek_etkileşimi {
            return Ok(false);
        }
        if !odak_oranı.is_finite() || !ham_delta.is_finite() || ham_delta.abs() <= f64::EPSILON {
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

        let mevcut = self.görünür();
        let odak = mevcut.en_az + odak_oranı.clamp(0.0, 1.0) * (mevcut.en_çok - mevcut.en_az);
        let aralık = mevcut
            .uyarlanabilir_tekerlek_yakınlaştır(self.tam, odak, delta, hassas, tekerlek)?;
        let yeni = (aralık != self.tam).then_some(aralık);
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
        self.uygula(None, true)
    }

    pub(crate) fn geri(&mut self) -> bool {
        if !self.ayarlar.görünüm_geçmişi {
            return false;
        }
        let Some(önceki) = self.geçmiş.pop_back() else {
            return false;
        };
        self.görünür = önceki;
        self.tekerleği_sıfırla();
        true
    }

    fn uygula(&mut self, yeni: Option<Aralık>, geçmişe_ekle: bool) -> bool {
        if self.görünür == yeni {
            return false;
        }
        if geçmişe_ekle && self.ayarlar.görünüm_geçmişi {
            if self.geçmiş.len() >= 100 {
                self.geçmiş.pop_front();
            }
            self.geçmiş.push_back(self.görünür);
        }
        self.görünür = yeni;
        true
    }

    fn tekerleği_sıfırla(&mut self) {
        self.son_tekerlek = None;
        self.tekerlek_hareketi_kaydedildi = false;
        self.birikmiş_hassas_delta = 0.0;
    }
}
