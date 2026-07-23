use crate::{TekerlekAyarları, TekerlekKipi, hata::UplotHatası};

/// Sonlu ve artan sayısal ölçek aralığı.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aralık {
    pub en_az: f64,
    pub en_çok: f64,
}

impl Aralık {
    /// uPlot `rangeNum(min, max, mult, extra)` sayısal ölçek aralığını üretir.
    ///
    /// Eşit değerler, ham büyüklüğe göre sıfır çevresine açılır; birbirine
    /// aşırı yakın değerler ise JavaScript kaynak kodundaki gibi düz veri
    /// kabul edilir. `sıfırı_yumuşat`, resmî Y ölçeği varsayılanındaki
    /// `extra: true` davranışını etkinleştirir.
    pub fn uplot_sayısal(
        en_az: f64,
        en_çok: f64,
        pay: f64,
        sıfırı_yumuşat: bool,
    ) -> Result<Self, UplotHatası> {
        if !en_az.is_finite() || !en_çok.is_finite() || en_az > en_çok {
            return Err(UplotHatası::GeçersizAralık { en_az, en_çok });
        }
        if !pay.is_finite() || pay < 0.0 {
            return Err(UplotHatası::GeçersizÇarpan { değer: pay });
        }

        let mut delta = en_çok - en_az;
        let delta_büyüklüğü = delta.log10();
        let mut mutlak_en_çok = en_az.abs().max(en_çok.abs());
        let skaler_büyüklüğü = mutlak_en_çok.log10();
        let büyüklük_farkı = (skaler_büyüklüğü - delta_büyüklüğü).abs();

        if delta < 1e-24 || büyüklük_farkı > 10.0 {
            delta = if en_az == 0.0 || en_çok == 0.0 {
                1e-24
            } else {
                0.0
            };
        }

        if mutlak_en_çok == 0.0 {
            mutlak_en_çok = 1_000.0;
        }
        let sıfır_olmayan_delta = if delta != 0.0 { delta } else { mutlak_en_çok };
        let taban = 10_f64.powf(sıfır_olmayan_delta.log10().floor());
        let alt_pay = sıfır_olmayan_delta
            * if delta == 0.0 {
                if en_az == 0.0 { 0.1 } else { 1.0 }
            } else {
                pay
            };
        let üst_pay = sıfır_olmayan_delta
            * if delta == 0.0 {
                if en_çok == 0.0 { 0.1 } else { 1.0 }
            } else {
                pay
            };
        let adım = taban / 10.0;
        let mut yeni_alt = ondalık_yuvarla(artıma_aşağı_yuvarla(en_az - alt_pay, adım), 24);
        let mut yeni_üst = ondalık_yuvarla(artıma_yukarı_yuvarla(en_çok + üst_pay, adım), 24);

        if sıfırı_yumuşat {
            if en_az >= 0.0 && yeni_alt <= 0.0 {
                yeni_alt = 0.0;
            }
            if en_çok <= 0.0 && yeni_üst >= 0.0 {
                yeni_üst = 0.0;
            }
        }

        if yeni_alt == yeni_üst {
            if yeni_alt == 0.0 {
                yeni_üst = 100.0;
            } else if yeni_alt < 0.0 {
                yeni_alt *= 2.0;
                yeni_üst = 0.0;
            } else {
                yeni_alt = 0.0;
                yeni_üst *= 2.0;
            }
        }

        Self::yeni(yeni_alt, yeni_üst)
    }

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

fn artıma_aşağı_yuvarla(sayı: f64, artım: f64) -> f64 {
    kayan_noktayı_düzelt((kayan_noktayı_düzelt(sayı / artım)).floor() * artım)
}

fn artıma_yukarı_yuvarla(sayı: f64, artım: f64) -> f64 {
    kayan_noktayı_düzelt((kayan_noktayı_düzelt(sayı / artım)).ceil() * artım)
}

fn kayan_noktayı_düzelt(değer: f64) -> f64 {
    if !değer.is_finite() || değer.fract() == 0.0 {
        return değer;
    }

    let yazı = değer.to_string();
    let (mantis, üs) = yazı
        .split_once('e')
        .or_else(|| yazı.split_once('E'))
        .map_or((yazı.as_str(), None), |(mantis, üs)| (mantis, Some(üs)));
    let Some(nokta) = mantis.find('.') else {
        return değer;
    };
    let basamaklar = &mantis[nokta.saturating_add(1)..];
    let yuvarlama_basamağı =
        basamaklar
            .as_bytes()
            .iter()
            .enumerate()
            .find_map(|(indeks, basamak)| {
                ((*basamak == b'0' || *basamak == b'9')
                    && basamaklar
                        .as_bytes()
                        .get(indeks..indeks.saturating_add(6))
                        .is_some_and(|dizi| dizi.iter().all(|aday| aday == basamak)))
                .then_some(indeks)
            });
    let Some(basamak) = yuvarlama_basamağı else {
        return değer;
    };

    let Ok(mantis_değeri) = mantis.parse::<f64>() else {
        return değer;
    };
    let düzeltilmiş = ondalık_yuvarla(mantis_değeri, basamak as i32);
    let Some(üs) = üs else {
        return düzeltilmiş;
    };
    let Ok(üs) = üs.parse::<i32>() else {
        return değer;
    };
    düzeltilmiş * 10_f64.powi(üs)
}

fn ondalık_yuvarla(değer: f64, basamak: i32) -> f64 {
    if !değer.is_finite() || değer.fract() == 0.0 {
        return değer;
    }
    let çarpan = 10_f64.powi(basamak);
    let ölçekli = değer * çarpan * (1.0 + f64::EPSILON);
    (ölçekli + 0.5).floor() / çarpan
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn range_num_kaynak_kenar_durumlarını_korur() -> Result<(), UplotHatası> {
        let durumlar: [((f64, f64), (f64, f64)); 9] = [
            ((-1.0, -1.0), (-2.0, 0.0)),
            ((0.0, 0.0), (0.0, 100.0)),
            ((1.0, 1.0), (0.0, 2.0)),
            ((36.0, 51.0), (34.0, 53.0)),
            ((10.0, 10.0), (0.0, 20.0)),
            ((9.999_999_9, 10.000_000_1), (0.0, 20.0)),
            ((9_999_999.999_753, 10_000_000.000_027), (0.0, 20_000_000.0)),
            ((-0.1, -0.1), (-0.2, 0.0)),
            ((0.1, 0.1), (0.0, 0.2)),
        ];
        for ((en_az, en_çok), (beklenen_alt, beklenen_üst)) in durumlar {
            let aralık = Aralık::uplot_sayısal(en_az, en_çok, 0.1, true)?;
            let tolerans = beklenen_alt.abs().max(beklenen_üst.abs()).max(1.0) * 1e-12;
            assert!((aralık.en_az - beklenen_alt).abs() <= tolerans);
            assert!((aralık.en_çok - beklenen_üst).abs() <= tolerans);
        }
        Ok(())
    }

    #[test]
    fn range_num_geçersiz_girdiyi_tipli_hatayla_reddeder() {
        assert!(Aralık::uplot_sayısal(f64::NAN, 1.0, 0.1, true).is_err());
        assert!(Aralık::uplot_sayısal(2.0, 1.0, 0.1, true).is_err());
        assert!(Aralık::uplot_sayısal(0.0, 1.0, -0.1, true).is_err());
    }
}
