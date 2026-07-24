use std::sync::OnceLock;

use serde::Deserialize;

use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, NoktaFiltreKipi, SeriSeçenekleri, UplotHatası
};

const KAYNAK_JSON: &str = include_str!("veri/points.json");
pub const POINTS_KANIT_TOHUMU: u32 = 0x504F_494E;

pub const POINTS_KART_TANIM_ÖRNEĞİ: &str = r##"for (örnek, seçenekler, veri) in points_kartları()? {
    // points.space, fill, paths:null ve points.filter davranışları
    // tek kaynak sayfasındaki dört eşzamanlı yüzeyde karşılaştırılır.
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

    pub const fn kaynak_boyutu(self) -> (u32, u32) {
        match self {
            Self::Karma => (1_920, 600),
            Self::VarsayılanYoğunluk | Self::AşırıYoğun => (1_920, 300),
            Self::Seyrek => (1_200, 300),
        }
    }

    pub const fn kısa_açıklama(self) -> &'static str {
        match self {
            Self::Karma => "Varsayılan yoğunluk, space:0, paths:null ve boşluk tekili filtresi",
            Self::VarsayılanYoğunluk => {
                "180 nokta, 1920 px genişlikte varsayılan nokta yoğunluğu"
            }
            Self::AşırıYoğun => {
                "Aynı 180 nokta ve veri; −400…180 X aralığında noktalar gizlenir"
            }
            Self::Seyrek => "2761 X konumunda boşluklar arasındaki tekil ölçümleri gösterir",
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
                .seri(SeriSeçenekleri::yeni("Value").renk("green"))
                .seri(
                    SeriSeçenekleri::yeni("Value")
                        .renk("red")
                        .dolgu("rgba(255,0,0,0.1)")
                        .nokta_boşluğu(0.0)
                        .nokta_stili(5.0, 1.0, Some("red")),
                )
                .seri(
                    SeriSeçenekleri::yeni("Value")
                        .renk("blue")
                        .dolgu("rgba(0,0,255,0.1)")
                        .yalnız_noktalar()
                        .nokta_boşluğu(0.0),
                )
                .seri(
                    SeriSeçenekleri::yeni("Value")
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
                .seri(SeriSeçenekleri::yeni("Value").renk("green"));
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
                SeriSeçenekleri::yeni("Value")
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

/// `points.html` içindeki dört uPlot yüzeyini kaynak sırasıyla tek aile
/// olarak üretir. Varsayılan ve aşırı yoğun yüzeyler kaynakta aynı `data`
/// nesnesini kullandığından burada de aynı veri anlık görüntüsünü paylaşır.
pub fn points_kartları() -> Result<Vec<(PointsÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası>
{
    let mut kartlar = Vec::with_capacity(PointsÖrneği::TÜMÜ.len());
    let (karma_seçenekleri, karma_verisi) = points_kartı(PointsÖrneği::Karma)?;
    kartlar.push((PointsÖrneği::Karma, karma_seçenekleri, karma_verisi));

    let (varsayılan_seçenekleri, ortak_yoğun_veri) =
        points_kartı(PointsÖrneği::VarsayılanYoğunluk)?;
    kartlar.push((
        PointsÖrneği::VarsayılanYoğunluk,
        varsayılan_seçenekleri,
        ortak_yoğun_veri.clone(),
    ));
    let (aşırı_seçenekleri, _) = points_kartı(PointsÖrneği::AşırıYoğun)?;
    kartlar.push((
        PointsÖrneği::AşırıYoğun,
        aşırı_seçenekleri,
        ortak_yoğun_veri,
    ));

    let (seyrek_seçenekleri, seyrek_veri) = points_kartı(PointsÖrneği::Seyrek)?;
    kartlar.push((PointsÖrneği::Seyrek, seyrek_seçenekleri, seyrek_veri));
    Ok(kartlar)
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
        let kartlar = points_kartları()?;
        assert_eq!(kartlar.len(), 4);
        for (örnek, seçenekler, veri) in &kartlar {
            assert_eq!(veri.uzunluk(), örnek.nokta_sayısı());
            assert_eq!(seçenekler.başlık, örnek.başlık());
            assert_eq!(
                (seçenekler.genişlik, seçenekler.yükseklik),
                örnek.kaynak_boyutu()
            );
        }
        let varsayılan_veri = kartlar.get(1).map(|kart| &kart.2);
        let aşırı_veri = kartlar.get(2).map(|kart| &kart.2);
        assert_eq!(varsayılan_veri, aşırı_veri);
        let (seçenekler, veri) = points_kartı(PointsÖrneği::Karma)?;
        assert_eq!(veri.seriler().len(), 4);
        assert!(
            seçenekler
                .seriler
                .get(2)
                .is_some_and(|seri| seri.çizim_türü == SeriÇizimTürü::Noktalar)
        );
        assert!(seçenekler.seriler.iter().all(|seri| seri.etiket == "Value"));
        assert_eq!(
            seçenekler
                .seriler
                .get(1)
                .and_then(|seri| seri.dolgu.as_deref()),
            Some("rgba(255,0,0,0.1)")
        );
        assert_eq!(
            seçenekler
                .seriler
                .get(2)
                .and_then(|seri| seri.dolgu.as_deref()),
            Some("rgba(0,0,255,0.1)")
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
    fn seyrek_filtre_kaynak_piksel_boşluklarını_gösterir() -> Result<(), UplotHatası> {
        let kaynak = kaynak_veri()?;
        let dolu_değer = kaynak.seyrek.iter().filter(|değer| değer.is_some()).count();
        assert_eq!(dolu_değer, 96);
        // Kaynak pointsFilter görünür kenarlar ve aynı piksele yuvarlanan
        // ardışık gap sınırları üzerinden 94 işaret seçer.
        assert_eq!(daire_sayısı(PointsÖrneği::Seyrek)?, 94);
        Ok(())
    }

    #[test]
    fn seyrek_filtre_zoomda_görünür_piksel_boşluklarından_yeniden_hesaplanır()
    -> Result<(), UplotHatası> {
        let (seçenekler, veri) = points_kartı(PointsÖrneği::Seyrek)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let yakın = grafik.çiz_aralıkta(Some(Aralık::yeni(2_000.0, 2_500.0)?));
        let yakın_daireler = yakın
            .komutlar()
            .iter()
            .filter(|komut| matches!(komut, Komut::Daire { .. }))
            .count();
        assert!(yakın_daireler > 0);
        assert!(yakın_daireler < 94);
        Ok(())
    }
}
