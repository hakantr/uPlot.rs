use crate::{TekerlekAyarları, TekerlekKipi, hata::UplotHatası};

/// Sonlu ve artan sayısal ölçek aralığı.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aralık {
    pub en_az: f64,
    pub en_çok: f64,
}

impl Aralık {
    /// uPlot'un resmi `wheelZoomPlugin` yaklaşımındaki 0.75 katsayısını ve
    /// tam veri aralığına sıkıştırmayı kullanarak X görünümünü değiştirir.
    /// `odak`, resmi eklentideki gibi ekrandaki göreli konumunu korur.
    pub fn tekerlek_yakınlaştır(
        self,
        tam: Self,
        odak: f64,
        yakınlaştır: bool,
    ) -> Result<Self, UplotHatası> {
        self.tekerlek_katsayısıyla(tam, odak, if yakınlaştır { 0.75 } else { 1.0 / 0.75 })
    }

    /// Ayrık tekerlekleri sabit adımla, hassas piksel akışlarını delta
    /// büyüklüğüyle orantılı olarak işler. Pozitif `delta` yakınlaştırır.
    pub fn uyarlanabilir_tekerlek_yakınlaştır(
        self,
        tam: Self,
        odak: f64,
        delta: f64,
        hassas_girdi: bool,
        ayarlar: TekerlekAyarları,
    ) -> Result<Self, UplotHatası> {
        if !delta.is_finite() || delta.abs() <= f64::EPSILON {
            return Ok(self);
        }
        let hassas = match ayarlar.kip {
            TekerlekKipi::Otomatik => hassas_girdi,
            TekerlekKipi::Ayrık => false,
            TekerlekKipi::Hassas => true,
        };
        let adım = if hassas {
            if delta.abs() < ayarlar.hassas_ölü_bölge {
                return Ok(self);
            }
            delta.clamp(-ayarlar.azami_hassas_delta, ayarlar.azami_hassas_delta)
                / ayarlar.hassas_piksel_adımı
        } else {
            delta.signum()
        };
        let katsayı = ayarlar.ayrık_katsayı.powf(adım);
        self.tekerlek_katsayısıyla(tam, odak, katsayı)
    }

    fn tekerlek_katsayısıyla(
        self,
        tam: Self,
        odak: f64,
        katsayı: f64,
    ) -> Result<Self, UplotHatası> {
        if !katsayı.is_finite() || katsayı <= 0.0 {
            return Err(UplotHatası::GeçersizAralık {
                en_az: katsayı,
                en_çok: katsayı,
            });
        }
        let mevcut = Self::yeni(self.en_az, self.en_çok)?;
        let tam = Self::yeni(tam.en_az, tam.en_çok)?;
        if !odak.is_finite() {
            return Err(UplotHatası::GeçersizAralık {
                en_az: odak,
                en_çok: odak,
            });
        }

        let tam_uzunluk = tam.en_çok - tam.en_az;
        let mevcut_uzunluk = mevcut.en_çok - mevcut.en_az;
        let yeni_uzunluk = (mevcut_uzunluk * katsayı).max(tam_uzunluk / 10_000.0);
        if yeni_uzunluk >= tam_uzunluk {
            return Ok(tam);
        }

        let odak = odak.clamp(mevcut.en_az, mevcut.en_çok);
        let odak_oranı = (odak - mevcut.en_az) / mevcut_uzunluk;
        let mut en_az = odak - odak_oranı * yeni_uzunluk;
        let mut en_çok = en_az + yeni_uzunluk;
        if en_az < tam.en_az {
            en_az = tam.en_az;
            en_çok = tam.en_az + yeni_uzunluk;
        } else if en_çok > tam.en_çok {
            en_çok = tam.en_çok;
            en_az = tam.en_çok - yeni_uzunluk;
        }

        Self::yeni(en_az, en_çok)
    }

    pub fn yeni(en_az: f64, en_çok: f64) -> Result<Self, UplotHatası> {
        if !en_az.is_finite() || !en_çok.is_finite() || en_az >= en_çok {
            return Err(UplotHatası::GeçersizAralık { en_az, en_çok });
        }
        Ok(Self { en_az, en_çok })
    }

    pub(crate) fn otomatik<'a>(değerler: impl Iterator<Item = &'a Option<f64>>) -> Self {
        let mut en_az = f64::INFINITY;
        let mut en_çok = f64::NEG_INFINITY;
        for değer in değerler.flatten() {
            en_az = en_az.min(*değer);
            en_çok = en_çok.max(*değer);
        }

        if !en_az.is_finite() || !en_çok.is_finite() {
            return Self {
                en_az: 0.0,
                en_çok: 1.0,
            };
        }
        if en_az == en_çok {
            let pay = en_az.abs().max(1.0) * 0.1;
            return Self {
                en_az: en_az - pay,
                en_çok: en_çok + pay,
            };
        }

        let pay = (en_çok - en_az) * 0.1;
        Self {
            en_az: en_az - pay,
            en_çok: en_çok + pay,
        }
    }

    pub(crate) fn konum(self, değer: f64, başlangıç: f32, uzunluk: f32) -> f32 {
        let oran = (değer - self.en_az) / (self.en_çok - self.en_az);
        başlangıç + (oran as f32 * uzunluk)
    }
}
