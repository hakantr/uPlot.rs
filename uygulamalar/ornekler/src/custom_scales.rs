#[path = "veri/custom_scales_kaynak.rs"]
mod kaynak;

use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, NoktaKatmanı, SeriBandı, SeriSeçenekleri, UplotHatası,
    YÖlçekEtiketBiçimi, YÖlçekSeçenekleri,
};
use kaynak::{
    CUSTOM_LOWER_CI, CUSTOM_POINT_X, CUSTOM_POINT_Y, CUSTOM_UPPER_CI, CUSTOM_WEIBULL, CUSTOM_X,
};

pub const CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ: &str = r##"let yüzeyler = custom_scales_kartları()?;
for (örnek, seçenekler, veri) in yüzeyler {
    // Üç yüzey aynı kaynak veriyi farklı, bağımsız ölçeklerle gösterir.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

pub const CUSTOM_WEIBULL_BÖLMELERİ: [f64; 18] = [
    0.00001, 0.0001, 0.001, 0.01, 0.1, 0.2, 0.3, 0.5, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99, 0.999,
    0.9999, 0.99999, 0.999999,
];

/// Resmî özel ölçeğin `fwd: v => log(-log(1-v))` dönüşümü.
pub fn custom_weibull_ileri(değer: f64) -> Option<f64> {
    (değer > 0.0 && değer < 1.0).then(|| (-(-değer).ln_1p()).ln())
}

/// Resmî özel ölçeğin `bwd: v => 1-exp(-exp(v))` dönüşümü.
pub fn custom_weibull_geri(değer: f64) -> Option<f64> {
    değer
        .is_finite()
        .then(|| 1.0 - (-değer.exp()).exp())
        .filter(|çıktı| çıktı.is_finite())
}

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

fn custom_scales_verisi() -> Result<HizalıVeri, UplotHatası> {
    HizalıVeri::yeni(
        CUSTOM_X.to_vec(),
        [CUSTOM_UPPER_CI, CUSTOM_LOWER_CI, CUSTOM_WEIBULL]
            .into_iter()
            .map(|seri| seri.into_iter().map(Some).collect())
            .collect(),
    )
}

fn custom_scales_seçenekleri(
    örnek: CustomScaleÖrneği,
) -> Result<GrafikSeçenekleri, UplotHatası> {
    let noktalar = CUSTOM_POINT_X
        .into_iter()
        .zip(CUSTOM_POINT_Y)
        .collect::<Vec<_>>();
    let y_aralığı = Aralık::yeni(
        CUSTOM_LOWER_CI.first().copied().unwrap_or(0.000_001),
        CUSTOM_UPPER_CI.last().copied().unwrap_or(0.999_999),
    )?;
    let mut y_ölçeği = YÖlçekSeçenekleri::yeni("y");
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
        CustomScaleÖrneği::Doğrusal => {
            y_ölçeği = y_ölçeği.aralık(y_aralığı);
        }
        CustomScaleÖrneği::LogLog => {
            seçenekler = seçenekler.x_logaritmik(10.0);
            y_ölçeği = y_ölçeği.logaritmik(10.0);
        }
        CustomScaleÖrneği::Weibull => {
            seçenekler = seçenekler
                .x_logaritmik(10.0)
                .birincil_y_eksen_genişliği(80.0)
                .y_sabit_bölmeler(CUSTOM_WEIBULL_BÖLMELERİ.to_vec())
                .y_özel_etiketler(
                    CUSTOM_WEIBULL_BÖLMELERİ.map(|değer| (değer, format!("{değer:e}"))),
                );
            y_ölçeği = y_ölçeği
                .aralık(y_aralığı)
                .özel(
                    "custom-scales-weibull",
                    custom_weibull_ileri,
                    custom_weibull_geri,
                )
                .etiket_biçimi(YÖlçekEtiketBiçimi::Bilimsel)
                .eksen_genişliği(80.0);
        }
    }
    Ok(seçenekler.y_ölçeği(y_ölçeği))
}

pub fn custom_scales_kartı(
    örnek: CustomScaleÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    Ok((custom_scales_seçenekleri(örnek)?, custom_scales_verisi()?))
}

/// Resmî sayfadaki üç bağımsız yüzeyi kaynak sırasıyla tek kart grubu yapar.
pub fn custom_scales_kartları()
-> Result<Vec<(CustomScaleÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    let veri = custom_scales_verisi()?;
    CustomScaleÖrneği::TÜMÜ
        .into_iter()
        .map(|örnek| Ok((örnek, custom_scales_seçenekleri(örnek)?, veri.clone())))
        .collect()
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut, YÖlçekDağılımı};

    fn fnv1a_f64(değerler: &[f64]) -> u64 {
        değerler.iter().fold(0xcbf2_9ce4_8422_2325, |özet, değer| {
            değer
                .to_bits()
                .to_le_bytes()
                .into_iter()
                .fold(özet, |özet, bayt| {
                    (özet ^ u64::from(bayt)).wrapping_mul(0x0000_0100_0000_01b3)
                })
        })
    }

    #[test]
    fn kaynak_dizilerinin_tüm_bitleri_aktarim_özetleriyle_eştir() {
        assert_eq!(fnv1a_f64(&CUSTOM_X), 0x3934_8a50_e253_1e6d);
        assert_eq!(fnv1a_f64(&CUSTOM_UPPER_CI), 0x73d8_be55_ab7f_ff19);
        assert_eq!(fnv1a_f64(&CUSTOM_LOWER_CI), 0x8750_caad_656f_1727);
        assert_eq!(fnv1a_f64(&CUSTOM_WEIBULL), 0x488d_50ac_2112_d2d0);
        assert_eq!(fnv1a_f64(&CUSTOM_POINT_X), 0x1be4_d171_fa66_4bff);
        assert_eq!(fnv1a_f64(&CUSTOM_POINT_Y), 0xbf0e_f2e5_e89f_8d9d);
    }

    #[test]
    fn üç_kaynak_ölçeği_aynı_veriyi_farklı_geometriyle_çizer() -> Result<(), UplotHatası> {
        let mut sahneler = Vec::new();
        let kartlar = custom_scales_kartları()?;
        assert_eq!(kartlar.len(), 3);
        for (örnek, seçenekler, veri) in kartlar {
            assert_eq!(veri.uzunluk(), 199);
            assert_eq!(seçenekler.genişlik, 800);
            assert_eq!(seçenekler.yükseklik, 800);
            let y_ölçeği = seçenekler
                .y_ölçekleri
                .first()
                .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
            match örnek {
                CustomScaleÖrneği::Doğrusal => {
                    assert!(y_ölçeği.aralık.is_some());
                }
                CustomScaleÖrneği::LogLog => {
                    assert!(y_ölçeği.aralık.is_none());
                }
                CustomScaleÖrneği::Weibull => {
                    assert_eq!(
                        seçenekler.birincil_y_sabit_bölmeler.as_deref(),
                        Some(CUSTOM_WEIBULL_BÖLMELERİ.as_slice())
                    );
                    assert_eq!(y_ölçeği.eksen_genişliği, 80.0);
                    assert_eq!(seçenekler.birincil_y_eksen_genişliği, Some(80.0));
                    assert!(matches!(
                        y_ölçeği.dağılım,
                        YÖlçekDağılımı::Özel(dönüşüm)
                            if dönüşüm.anahtar == "custom-scales-weibull"
                    ));
                }
            }
            let grafik = Grafik::yeni(seçenekler, veri)?;
            match örnek {
                CustomScaleÖrneği::Doğrusal => {
                    assert_eq!(
                        grafik.görünür_y_aralığı(),
                        Aralık::yeni(CUSTOM_LOWER_CI[0], CUSTOM_UPPER_CI[198])?
                    );
                }
                CustomScaleÖrneği::LogLog => {
                    assert_eq!(grafik.görünür_x_aralığı(), Aralık::yeni(0.1, 100.0)?);
                    assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(0.000_001, 1.0)?);
                }
                CustomScaleÖrneği::Weibull => {
                    assert_eq!(grafik.görünür_x_aralığı(), Aralık::yeni(0.1, 100.0)?);
                    assert_eq!(
                        grafik.görünür_y_aralığı(),
                        Aralık::yeni(CUSTOM_LOWER_CI[0], CUSTOM_UPPER_CI[198])?
                    );
                }
            }
            let sahne = grafik.çiz();
            assert!(
                sahne.komutlar().iter().any(
                    |komut| matches!(komut, Komut::Alan { dolgu, .. } if dolgu == "#ffa50030")
                )
            );
            assert_eq!(sahne.komutlar().iter().filter(|komut| matches!(komut, Komut::Dikdörtgen { dolgu, .. } if dolgu == "#000000")).count(), 20);
            let ilk_siyah_kare = sahne.komutlar().iter().position(|komut| {
                matches!(
                    komut,
                    Komut::Dikdörtgen {
                        genişlik,
                        yükseklik,
                        dolgu,
                        ..
                    } if *genişlik == 5.0 && *yükseklik == 5.0 && dolgu == "#000000"
                )
            });
            let son_veri_yolu = sahne.komutlar().iter().rposition(|komut| {
                matches!(
                    komut,
                    Komut::Yol { .. } | Komut::KesikliYol { .. } | Komut::Alan { .. }
                )
            });
            assert!(
                ilk_siyah_kare
                    .zip(son_veri_yolu)
                    .is_some_and(|(kare, yol)| kare > yol),
                "draw-hook kareleri band ve fitted yolunun üstünde boyanmalıdır"
            );
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

    #[test]
    fn özel_weibull_dönüşümü_round_trip_ve_tanım_kümesini_korur() -> Result<(), UplotHatası> {
        assert_eq!(custom_weibull_ileri(0.0), None);
        assert_eq!(custom_weibull_ileri(1.0), None);
        let değerler = [0.00001, 0.001, 0.1, 0.5, 0.9, 0.999999];
        for değer in değerler {
            let dönüşen =
                custom_weibull_ileri(değer).ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
            let geri =
                custom_weibull_geri(dönüşen).ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
            assert!((geri - değer).abs() <= 1e-12_f64.max(değer.abs() * 1e-12));
        }
        Ok(())
    }
}
