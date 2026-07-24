use super::ortak_kart_etkileşimleri;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const NEAREST_NON_NULL_KART_TANIM_ÖRNEĞİ: &str = r##"let örnek = NearestNonNullÖrneği::XDeğerineGöre;
let (seçenekler, veri) = nearest_non_null_kartı(örnek)?;
let grafik = Grafik::yeni(seçenekler, veri)?;
let indeks = grafik.en_yakın_null_olmayan_indeks(0.5, 0, NullAtlamaYönü::EnYakın);"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NearestNonNullÖrneği {
    XDeğerineGöre,
    OtuzPikselYakınlık,
    NullİseOnBeşPiksel,
    ÖncekiNullOlmayan,
}

impl NearestNonNullÖrneği {
    pub const TÜMÜ: [Self; 4] = [
        Self::XDeğerineGöre,
        Self::OtuzPikselYakınlık,
        Self::NullİseOnBeşPiksel,
        Self::ÖncekiNullOlmayan,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::XDeğerineGöre => "nearest-non-null-x-value",
            Self::OtuzPikselYakınlık => "nearest-non-null-30px",
            Self::NullİseOnBeşPiksel => "nearest-non-null-null-15px",
            Self::ÖncekiNullOlmayan => "nearest-non-null-previous",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::XDeğerineGöre => "Nearest Non-Null · by nearest x scale value",
            Self::OtuzPikselYakınlık => "Nearest Non-Null · by 30px proximity",
            Self::NullİseOnBeşPiksel => "Nearest Non-Null · 15px only when null",
            Self::ÖncekiNullOlmayan => "Nearest Non-Null · snap to previous",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

pub fn nearest_non_null_kartı(
    örnek: NearestNonNullÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (genişlik, yükseklik, x, seriler, adlar, renkler) = match örnek {
        NearestNonNullÖrneği::XDeğerineGöre => {
            let x = (0..200)
                .map(|i| 1_566_453_600.0 + f64::from(i) * 60.0)
                .collect::<Vec<_>>();
            let cpu = (0..200)
                .map(|i| (!(35..=41).contains(&i)).then_some(6.0 + f64::from(i % 9) / 100.0))
                .collect();
            let ram = (0..200)
                .map(|i| {
                    (![79, 80, 91, 125, 126, 127].contains(&i))
                        .then_some(14.0 + f64::from(i % 5) / 100.0)
                })
                .collect();
            (
                1920,
                600,
                x,
                vec![cpu, ram],
                vec!["CPU", "RAM"],
                vec!["red", "blue"],
            )
        }
        NearestNonNullÖrneği::OtuzPikselYakınlık => {
            let x = (0..33).map(|i| f64::from(i) / 32.0 * 100.0).collect();
            let seyrek = (0..33)
                .map(|i| [0, 7, 13].contains(&i).then_some(30.0))
                .collect();
            (
                1920,
                600,
                x,
                vec![
                    seyrek,
                    vec![Some(5.0); 33],
                    vec![Some(18.0); 33],
                    vec![Some(25.0); 33],
                ],
                vec!["Sparse", "5", "18", "25"],
                vec!["red", "blue", "green", "orange"],
            )
        }
        NearestNonNullÖrneği::NullİseOnBeşPiksel => (
            1920,
            600,
            (0..11)
                .map(|i| 1_626_235_540_319.0 + f64::from(i) * 43_685.0)
                .collect(),
            vec![
                [1., 20., 90., 30., 5., 0., 100., 20., 0., 100., 20.]
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| (i != 8).then_some(v))
                    .collect(),
                [1., 20., 90., 30., 5., 0., 0., 0., 0., 0., 0.]
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| (i != 5).then_some(v))
                    .collect(),
            ],
            vec!["Series 1", "Series 2"],
            vec!["red", "blue"],
        ),
        NearestNonNullÖrneği::ÖncekiNullOlmayan => (
            600,
            300,
            (0..10).map(f64::from).collect(),
            vec![(0..10).map(|i| (i != 5).then_some(5.0)).collect()],
            vec!["foo"],
            vec!["red"],
        ),
    };
    let veri = HizalıVeri::yeni(x, seriler)?;
    let mut seçenekler = GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .başlık(örnek.başlık())
        .x_zaman(matches!(
            örnek,
            NearestNonNullÖrneği::XDeğerineGöre | NearestNonNullÖrneği::NullİseOnBeşPiksel
        ))
        .etkileşimler(ortak_kart_etkileşimleri());
    for (ad, renk) in adlar.into_iter().zip(renkler) {
        seçenekler = seçenekler.seri(
            SeriSeçenekleri::yeni(ad)
                .renk(renk)
                .dolgu(format!("{renk}18"))
                .boşlukları_birleştir(örnek != NearestNonNullÖrneği::XDeğerineGöre),
        );
    }
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, NullAtlamaYönü};

    #[test]
    fn null_koşusunda_x_uzaklığı_ve_eşitlikte_sol_kazanır() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = nearest_non_null_kartı(NearestNonNullÖrneği::ÖncekiNullOlmayan)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(
            grafik.en_yakın_null_olmayan_indeks(5.0 / 9.0, 0, NullAtlamaYönü::EnYakın),
            Some(4)
        );
        assert_eq!(
            grafik.en_yakın_null_olmayan_indeks(5.8 / 9.0, 0, NullAtlamaYönü::EnYakın),
            Some(6)
        );
        assert_eq!(
            grafik.en_yakın_null_olmayan_indeks(5.8 / 9.0, 0, NullAtlamaYönü::Önceki),
            Some(4)
        );
        Ok(())
    }

    #[test]
    fn dört_kaynak_yüzeyi_çizilir() -> Result<(), UplotHatası> {
        for örnek in NearestNonNullÖrneği::TÜMÜ {
            let (seçenekler, veri) = nearest_non_null_kartı(örnek)?;
            assert!(!Grafik::yeni(seçenekler, veri)?.çiz().komutlar().is_empty());
        }
        Ok(())
    }
}
