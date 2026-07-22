#[path = "veri/custom_scales_kaynak.rs"]
mod kaynak;

use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, NoktaKatmanı, SeriBandı, SeriSeçenekleri, UplotHatası,
    YÖlçekSeçenekleri,
};
use kaynak::{
    CUSTOM_LOWER_CI, CUSTOM_POINT_X, CUSTOM_POINT_Y, CUSTOM_UPPER_CI, CUSTOM_WEIBULL, CUSTOM_X,
};

pub const CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = custom_scales_kartı(CustomScaleÖrneği::Weibull)?;
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomScaleÖrneği {
    Doğrusal,
    LogLog,
    Weibull,
}

impl CustomScaleÖrneği {
    pub const TÜMÜ: [Self; 3] = [Self::Doğrusal, Self::LogLog, Self::Weibull];
    pub fn kimlik(self) -> &'static str {
        match self {
            Self::Doğrusal => "custom-scales-linear",
            Self::LogLog => "custom-scales-log-log",
            Self::Weibull => "custom-scales-weibull",
        }
    }
    pub fn başlık(self) -> &'static str {
        match self {
            Self::Doğrusal => "x linear; y linear",
            Self::LogLog => "log(x); log(y)",
            Self::Weibull => "log(x); log(-log(1 - y))",
        }
    }
}

pub fn custom_scales_kartı(
    örnek: CustomScaleÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = HizalıVeri::yeni(
        CUSTOM_X.to_vec(),
        [CUSTOM_UPPER_CI, CUSTOM_LOWER_CI, CUSTOM_WEIBULL]
            .into_iter()
            .map(|seri| seri.into_iter().map(Some).collect())
            .collect(),
    )?;
    let noktalar = CUSTOM_POINT_X
        .into_iter()
        .zip(CUSTOM_POINT_Y)
        .collect::<Vec<_>>();
    let y_aralığı = Aralık::yeni(
        CUSTOM_LOWER_CI.first().copied().unwrap_or(0.000_001),
        CUSTOM_UPPER_CI.last().copied().unwrap_or(0.999_999),
    )?;
    let mut y_ölçeği = YÖlçekSeçenekleri::yeni("y").aralık(y_aralığı);
    let mut seçenekler = GrafikSeçenekleri::yeni(800, 800)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri())
        .bant(SeriBandı::yeni(0, 1, "#ffa50030"))
        .nokta_katmanı(NoktaKatmanı::yeni(noktalar))
        .seri(
            SeriSeçenekleri::yeni("upper_ci")
                .renk("#0000ff")
                .çizgi_kalınlığı(0.0),
        )
        .seri(
            SeriSeçenekleri::yeni("lower_ci")
                .renk("#008000")
                .çizgi_kalınlığı(0.0),
        )
        .seri(
            SeriSeçenekleri::yeni("weibull_fitted")
                .renk("#ffa500")
                .çizgi_kalınlığı(2.0)
                .çizgi_kesik(10.0, 5.0),
        );
    match örnek {
        CustomScaleÖrneği::Doğrusal => {}
        CustomScaleÖrneği::LogLog => {
            seçenekler = seçenekler.x_logaritmik(10.0);
            y_ölçeği = y_ölçeği.logaritmik(10.0);
        }
        CustomScaleÖrneği::Weibull => {
            seçenekler = seçenekler.x_logaritmik(10.0);
            y_ölçeği = y_ölçeği.weibull();
        }
    }
    Ok((seçenekler.y_ölçeği(y_ölçeği), veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};
    #[test]
    fn üç_kaynak_ölçeği_aynı_veriyi_farklı_geometriyle_çizer() -> Result<(), UplotHatası> {
        let mut sahneler = Vec::new();
        for örnek in CustomScaleÖrneği::TÜMÜ {
            let (seçenekler, veri) = custom_scales_kartı(örnek)?;
            assert_eq!(veri.uzunluk(), 199);
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            assert!(
                sahne.komutlar().iter().any(
                    |komut| matches!(komut, Komut::Alan { dolgu, .. } if dolgu == "#ffa50030")
                )
            );
            assert_eq!(sahne.komutlar().iter().filter(|komut| matches!(komut, Komut::Dikdörtgen { dolgu, .. } if dolgu == "#000000")).count(), 20);
            assert!(
                sahne
                    .komutlar()
                    .iter()
                    .any(|komut| matches!(komut, Komut::KesikliYol { .. }))
            );
            sahneler.push(sahne);
        }
        assert_ne!(sahneler.first(), sahneler.get(1));
        assert_ne!(sahneler.get(1), sahneler.get(2));
        Ok(())
    }
}
