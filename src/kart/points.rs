use std::sync::OnceLock;

use serde::Deserialize;

use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, NoktaFiltreKipi, SeriSeçenekleri, UplotHatası
};

const KAYNAK_JSON: &str = include_str!("veri/points.json");
pub const POINTS_KANIT_TOHUMU: u32 = 0x504F_494E;

pub const POINTS_KART_TANIM_ÖRNEĞİ: &str = r##"for örnek in PointsÖrneği::TÜMÜ {
    let (seçenekler, veri) = points_kartı(örnek)?;
    // points.space, fill, paths:null ve points.filter davranışları
    // platform yüzeyinden bağımsız olarak çekirdekte çözülür.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointsÖrneği {
    Karma,
    VarsayılanYoğunluk,
    AşırıYoğun,
    Seyrek,
}

impl PointsÖrneği {
    pub const TÜMÜ: [Self; 4] = [
        Self::Karma,
        Self::VarsayılanYoğunluk,
        Self::AşırıYoğun,
        Self::Seyrek,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::Karma => "points-mixed",
            Self::VarsayılanYoğunluk => "points-default-density",
            Self::AşırıYoğun => "points-too-dense",
            Self::Seyrek => "points-sparse",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::Karma | Self::VarsayılanYoğunluk => "Points",
            Self::AşırıYoğun => "Too dense test",
            Self::Seyrek => "Sparse Points",
        }
    }

    pub const fn nokta_sayısı(self) -> usize {
        match self {
            Self::Karma => 200,
            Self::VarsayılanYoğunluk | Self::AşırıYoğun => 180,
            Self::Seyrek => 2_761,
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

#[derive(Debug, Deserialize)]
struct PointsKaynakVerisi {
    ilk: Vec<Vec<Option<f64>>>,
    #[serde(rename = "yoğun180")]
    yoğun_180: Vec<Option<f64>>,
    seyrek: Vec<Option<f64>>,
}

fn kaynak_veri() -> Result<&'static PointsKaynakVerisi, UplotHatası> {
    static KAYNAK: OnceLock<Result<PointsKaynakVerisi, String>> = OnceLock::new();
    match KAYNAK.get_or_init(|| serde_json::from_str(KAYNAK_JSON).map_err(|hata| hata.to_string()))
    {
        Ok(kaynak) => Ok(kaynak),
        Err(açıklama) => Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "src/kart/veri/points.json",
            açıklama: açıklama.clone(),
        }),
    }
}

pub fn points_kartı(
    örnek: PointsÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let kaynak = kaynak_veri()?;
    match örnek {
        PointsÖrneği::Karma => {
            if kaynak.ilk.len() != 4 {
                return Err(UplotHatası::GeçersizKaynakVeri {
                    varlık: "points.json#ilk",
                    açıklama: format!("4 seri bekleniyordu, {} bulundu", kaynak.ilk.len()),
                });
            }
            let seçenekler = temel(1_920, 600, örnek.başlık())?
                .seri(SeriSeçenekleri::yeni("Green").renk("green"))
                .seri(
                    SeriSeçenekleri::yeni("Red")
                        .renk("red")
                        .dolgu("#ff00001a")
                        .nokta_boşluğu(0.0)
                        .nokta_stili(5.0, 1.0, Some("red")),
                )
                .seri(
                    SeriSeçenekleri::yeni("Blue")
                        .renk("blue")
                        .dolgu("#0000ff1a")
                        .yalnız_noktalar()
                        .nokta_boşluğu(0.0),
                )
                .seri(
                    SeriSeçenekleri::yeni("Orange")
                        .renk("orange")
                        .nokta_stili(5.0, 1.0, Some("orange"))
                        .nokta_filtresi(NoktaFiltreKipi::BoşlukArasındakiTekiller),
                );
            Ok((
                seçenekler,
                HizalıVeri::yeni(x_değerleri(200), kaynak.ilk.clone())?,
            ))
        }
        PointsÖrneği::VarsayılanYoğunluk | PointsÖrneği::AşırıYoğun => {
            let mut seçenekler = temel(1_920, 300, örnek.başlık())?
                .seri(SeriSeçenekleri::yeni("Green").renk("green"));
            if örnek == PointsÖrneği::AşırıYoğun {
                seçenekler = seçenekler.x_aralığı(Aralık::yeni(-400.0, 180.0)?);
            }
            Ok((
                seçenekler,
                HizalıVeri::yeni(x_değerleri(180), vec![kaynak.yoğun_180.clone()])?,
            ))
        }
        PointsÖrneği::Seyrek => {
            let seçenekler = temel(1_200, 300, örnek.başlık())?.seri(
                SeriSeçenekleri::yeni("Magenta")
                    .renk("magenta")
                    .nokta_stili(5.0, 1.0, Some("magenta"))
                    .nokta_filtresi(NoktaFiltreKipi::BoşlukArasındakiTekiller),
            );
            Ok((
                seçenekler,
                HizalıVeri::yeni(
                    x_değerleri(kaynak.seyrek.len()),
                    vec![kaynak.seyrek.clone()],
                )?,
            ))
        }
    }
}

fn temel(
    genişlik: u32, yükseklik: u32, başlık: &str
) -> Result<GrafikSeçenekleri, UplotHatası> {
    Ok(GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .başlık(başlık)
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri()))
}

fn x_değerleri(uzunluk: usize) -> Vec<f64> {
    (1..=uzunluk).map(|değer| değer as f64).collect()
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut, SeriÇizimTürü};

    fn daire_sayısı(örnek: PointsÖrneği) -> Result<usize, UplotHatası> {
        let (seçenekler, veri) = points_kartı(örnek)?;
        Ok(Grafik::yeni(seçenekler, veri)?
            .çiz()
            .komutlar()
            .iter()
            .filter(|komut| matches!(komut, Komut::Daire { .. }))
            .count())
    }

    #[test]
    fn dört_kaynak_yüzeyi_boyut_ve_veriyi_korur() -> Result<(), UplotHatası> {
        for örnek in PointsÖrneği::TÜMÜ {
            let (seçenekler, veri) = points_kartı(örnek)?;
            assert_eq!(veri.uzunluk(), örnek.nokta_sayısı());
            assert_eq!(seçenekler.başlık, örnek.başlık());
        }
        let (seçenekler, veri) = points_kartı(PointsÖrneği::Karma)?;
        assert_eq!(veri.seriler().len(), 4);
        assert!(
            seçenekler
                .seriler
                .get(2)
                .is_some_and(|seri| seri.çizim_türü == SeriÇizimTürü::Noktalar)
        );
        assert_eq!(
            veri.seriler().get(3).map_or(0, |seri| seri
                .iter()
                .filter(|değer| değer.is_none())
                .count()),
            6
        );
        Ok(())
    }

    #[test]
    fn yoğunluk_space_sıfır_ve_paths_null_davranışları_ayrılır() -> Result<(), UplotHatası> {
        assert_eq!(daire_sayısı(PointsÖrneği::Karma)?, 403);
        assert_eq!(daire_sayısı(PointsÖrneği::VarsayılanYoğunluk)?, 180);
        assert_eq!(daire_sayısı(PointsÖrneği::AşırıYoğun)?, 0);
        Ok(())
    }

    #[test]
    fn seyrek_filtre_yalnız_boşluklar_arasındaki_tekilleri_gösterir() -> Result<(), UplotHatası> {
        let kaynak = kaynak_veri()?;
        let beklenen = kaynak
            .seyrek
            .iter()
            .enumerate()
            .filter(|(indeks, değer)| {
                değer.is_some()
                    && indeks
                        .checked_sub(1)
                        .and_then(|önceki| kaynak.seyrek.get(önceki))
                        .is_none_or(Option::is_none)
                    && kaynak
                        .seyrek
                        .get(indeks.saturating_add(1))
                        .is_none_or(Option::is_none)
            })
            .count();
        assert_eq!(daire_sayısı(PointsÖrneği::Seyrek)?, beklenen);
        assert!(beklenen > 0);
        Ok(())
    }
}
