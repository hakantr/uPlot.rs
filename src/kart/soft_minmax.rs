use super::ortak_kart_etkileşimleri;
use crate::{
    GrafikSeçenekleri, HizalıVeri, SayısalAralıkAyarları, SayısalAralıkParçası, SeriSeçenekleri,
    UplotHatası, YumuşakSınırKipi, YÖlçekSeçenekleri,
};

pub const SOFT_MINMAX_KART_TANIM_ÖRNEĞİ: &str = r##"let mut akış = SoftMinMaxAkışı::yeni();
for örnek in SoftMinMaxÖrneği::TÜMÜ {
    let (seçenekler, veri) = soft_minmax_kartı(örnek, akış.veri_en_çok())?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;

    // Resmî demodaki `dataMax++` düğmesinin her 50 ms adımı:
    if örnek.canlı_mı() {
        grafik.veriyi_ayarla(akış.ilerlet(örnek)?)?;
    }
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoftMinMaxÖrneği {
    MinKip0,
    MinKip1,
    MinKip2,
    MinKip3,
    DüzSıfır,
}

impl SoftMinMaxÖrneği {
    pub const TÜMÜ: [Self; 5] = [
        Self::MinKip0,
        Self::MinKip1,
        Self::MinKip2,
        Self::MinKip3,
        Self::DüzSıfır,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::MinKip0 => "soft-minmax-mode-0",
            Self::MinKip1 => "soft-minmax-mode-1",
            Self::MinKip2 => "soft-minmax-mode-2",
            Self::MinKip3 => "soft-minmax-mode-3",
            Self::DüzSıfır => "soft-minmax-flat-zero",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::MinKip0 => "min: {soft: 0, mode: 0}",
            Self::MinKip1 => "min: {soft: 0, mode: 1}",
            Self::MinKip2 => "min: {soft: 0, mode: 2}",
            Self::MinKip3 => "min: {soft: 0, mode: 3}",
            Self::DüzSıfır => "min: {soft: -1, mode: 2}, max: {soft: 1, mode: 2}",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }

    pub const fn açıklama(self) -> &'static str {
        match self {
            Self::MinKip0 => {
                "With min.mode: 0 or min.soft: null, the scaleMin will always be a constant % [of the full range] below dataMin."
            }
            Self::MinKip1 => {
                "With min.mode: 1, the scaleMin will be min.soft unless dataMin goes below it. This is probably how most would expect a softMin setting to behave."
            }
            Self::MinKip2 => {
                "With min.mode: 2, the scaleMin will be min.soft unless (dataMin - pad) goes below it."
            }
            Self::MinKip3 => {
                "With min.mode: 3, the scaleMin will be a constant % [of the full range] below dataMin until (dataMin - pad) goes below it. This is uPlot's default mode - it provides a conditioned softMin - keeping more vertical resolution when the value range is small and far from softMin."
            }
            Self::DüzSıfır => {
                "Flat zero data keeps the configured soft range at -1..1 without the generic flat-data fallback."
            }
        }
    }

    pub const fn canlı_mı(self) -> bool {
        !matches!(self, Self::DüzSıfır)
    }

    const fn en_az_kipi(self) -> YumuşakSınırKipi {
        match self {
            Self::MinKip0 => YumuşakSınırKipi::SabitPay,
            Self::MinKip1 => YumuşakSınırKipi::VeriAşarsa,
            Self::MinKip2 | Self::DüzSıfır => YumuşakSınırKipi::PayAşarsa,
            Self::MinKip3 => YumuşakSınırKipi::Koşullu,
        }
    }

    const fn en_çok_kipi(self) -> YumuşakSınırKipi {
        match self {
            Self::MinKip3 => YumuşakSınırKipi::Koşullu,
            _ => YumuşakSınırKipi::PayAşarsa,
        }
    }

    const fn aralık_ayarları(self) -> SayısalAralıkAyarları {
        if matches!(self, Self::DüzSıfır) {
            SayısalAralıkAyarları::yeni(
                SayısalAralıkParçası::yeni(0.2, Some(-1.0), self.en_az_kipi()),
                SayısalAralıkParçası::yeni(0.2, Some(1.0), self.en_çok_kipi()),
            )
        } else {
            SayısalAralıkAyarları::yeni(
                SayısalAralıkParçası::yeni(0.2, Some(0.0), self.en_az_kipi()),
                SayısalAralıkParçası::yeni(0.2, Some(0.0), self.en_çok_kipi()),
            )
        }
    }
}

/// Resmî demodaki `data[1][1] += .1` durumunu çekirdekte tutar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SoftMinMaxAkışı {
    veri_en_çok: f64,
}

impl SoftMinMaxAkışı {
    pub const fn yeni() -> Self {
        Self { veri_en_çok: 12.0 }
    }

    pub const fn veri_en_çok(self) -> f64 {
        self.veri_en_çok
    }

    pub fn ilerlet(&mut self, örnek: SoftMinMaxÖrneği) -> Result<HizalıVeri, UplotHatası> {
        if !örnek.canlı_mı() {
            return soft_minmax_verisi(örnek, self.veri_en_çok);
        }
        self.veri_en_çok += 0.1;
        soft_minmax_verisi(örnek, self.veri_en_çok)
    }
}

impl Default for SoftMinMaxAkışı {
    fn default() -> Self {
        Self::yeni()
    }
}

pub fn soft_minmax_kartı(
    örnek: SoftMinMaxÖrneği,
    veri_en_çok: f64,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    if !veri_en_çok.is_finite() {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "SoftMinMaxAkışı",
            açıklama: "dataMax sonlu olmalıdır".to_string(),
        });
    }
    let seçenekler = GrafikSeçenekleri::yeni(400, 400)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("Data")
                .renk("blue")
                .dolgu("#0000ff1a"),
        )
        .y_ölçeği(YÖlçekSeçenekleri::yeni("y").sayısal_aralık(örnek.aralık_ayarları()));
    Ok((seçenekler, soft_minmax_verisi(örnek, veri_en_çok)?))
}

fn soft_minmax_verisi(
    örnek: SoftMinMaxÖrneği,
    veri_en_çok: f64,
) -> Result<HizalıVeri, UplotHatası> {
    if matches!(örnek, SoftMinMaxÖrneği::DüzSıfır) {
        HizalıVeri::yeni(vec![1.0, 2.0], vec![vec![Some(0.0), Some(0.0)]])
    } else {
        HizalıVeri::yeni(vec![0.0, 10.0], vec![vec![Some(5.0), Some(veri_en_çok)]])
    }
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn beş_kaynak_yüzeyi_ve_aralık_kipleri_korunur() -> Result<(), UplotHatası> {
        let beklenenler = [
            (SoftMinMaxÖrneği::MinKip0, (3.6, 13.4)),
            (SoftMinMaxÖrneği::MinKip1, (0.0, 13.4)),
            (SoftMinMaxÖrneği::MinKip2, (0.0, 13.4)),
            (SoftMinMaxÖrneği::MinKip3, (3.6, 13.4)),
            (SoftMinMaxÖrneği::DüzSıfır, (-1.0, 1.0)),
        ];
        for (örnek, beklenen) in beklenenler {
            let (seçenekler, veri) = soft_minmax_kartı(örnek, 12.0)?;
            assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (400, 400));
            assert_eq!(seçenekler.başlık, örnek.başlık());
            let grafik = Grafik::yeni(seçenekler, veri)?;
            let aralık = grafik.görünür_y_aralığı();
            assert!(
                (aralık.en_az - beklenen.0).abs() <= 1e-12,
                "{örnek:?}: {} != {}",
                aralık.en_az,
                beklenen.0
            );
            assert!(
                (aralık.en_çok - beklenen.1).abs() <= 1e-12,
                "{örnek:?}: {} != {}",
                aralık.en_çok,
                beklenen.1
            );
            assert!(
                grafik
                    .çiz()
                    .komutlar()
                    .iter()
                    .any(|komut| matches!(komut, Komut::Alan { .. }))
            );
        }
        Ok(())
    }

    #[test]
    fn data_max_artışı_çekirdekten_set_data_verisi_üretir() -> Result<(), UplotHatası> {
        let örnek = SoftMinMaxÖrneği::MinKip2;
        let mut akış = SoftMinMaxAkışı::yeni();
        let (seçenekler, veri) = soft_minmax_kartı(örnek, akış.veri_en_çok())?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        grafik.veriyi_ayarla(akış.ilerlet(örnek)?)?;
        assert!((akış.veri_en_çok() - 12.1).abs() <= 1e-12);
        let aralık = grafik.görünür_y_aralığı();
        assert_eq!(aralık.en_az, 0.0);
        assert!(aralık.en_çok > 13.4);
        Ok(())
    }
}
