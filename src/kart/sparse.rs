use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, NoktaŞekli, SeriSeçenekleri, UplotHatası,
    YÖlçekSeçenekleri,
};

#[path = "veri/sparse.rs"]
mod kaynak_veri;

pub const SPARSE_KART_TANIM_ÖRNEĞİ: &str = r##"for (örnek, seçenekler, veri) in sparse_kartları()? {
    // Aynı 13.608 X / 4.608 dolu Y snapshot'ı üzerinde optimize native,
    // tek toplu points Path2D ve saf moveTo/lineTo maliyetleri karşılaştırılır.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SparseÖrneği {
    YerleşikDoğrusal,
    ÖzelNoktalar,
    ÖzelSafDoğrusal,
}

impl SparseÖrneği {
    pub const TÜMÜ: [Self; 3] = [
        Self::YerleşikDoğrusal,
        Self::ÖzelNoktalar,
        Self::ÖzelSafDoğrusal,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::YerleşikDoğrusal => "sparse-native-linear",
            Self::ÖzelNoktalar => "sparse-custom-points",
            Self::ÖzelSafDoğrusal => "sparse-custom-naive-linear",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::YerleşikDoğrusal => "Native linear pathBuilder",
            Self::ÖzelNoktalar => "Custom points pathBuilder",
            Self::ÖzelSafDoğrusal => "Custom linear pathBuilder - naive w/moveTo()",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

pub fn sparse_kartı(
    örnek: SparseÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (x, y) = kaynak_veri::sparse_verisi()?;
    let veri = HizalıVeri::yeni(x, vec![y])?;
    Ok((sparse_seçenekleri(örnek)?, veri))
}

pub fn sparse_kartları() -> Result<Vec<(SparseÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası>
{
    let (x, y) = kaynak_veri::sparse_verisi()?;
    let veri = HizalıVeri::yeni(x, vec![y])?;
    SparseÖrneği::TÜMÜ
        .into_iter()
        .map(|örnek| Ok((örnek, sparse_seçenekleri(örnek)?, veri.clone())))
        .collect()
}

fn sparse_seçenekleri(örnek: SparseÖrneği) -> Result<GrafikSeçenekleri, UplotHatası> {
    let seri = match örnek {
        SparseÖrneği::YerleşikDoğrusal => SeriSeçenekleri::yeni("Sparse")
            .renk("red")
            .boşlukları_birleştir(false)
            .noktaları_göster(false),
        SparseÖrneği::ÖzelNoktalar => SeriSeçenekleri::yeni("Sparse")
            .renk("red")
            .çizgi_kalınlığı(0.0)
            .boşlukları_birleştir(false)
            .yalnız_noktalar()
            .noktaları_göster(true)
            .nokta_stili(2.0, 0.0, Some("red"))
            .nokta_şekli(NoktaŞekli::Kare),
        SparseÖrneği::ÖzelSafDoğrusal => SeriSeçenekleri::yeni("Sparse")
            .renk("red")
            .boşlukları_birleştir(false)
            .noktaları_göster(false)
            .saf_doğrusal_yol(true),
    };
    let seçenekler = GrafikSeçenekleri::yeni(800, 200)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .y_aralığı(Aralık::yeni(100.0, 350.0)?)
        .y_ölçeği(YÖlçekSeçenekleri::yeni("y").aralık(Aralık::yeni(100.0, 350.0)?))
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(seri);
    Ok(seçenekler)
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut, SeriÇizimTürü};

    #[test]
    fn üç_yüzey_aynı_kaynak_sparse_verisini_kullanır() -> Result<(), UplotHatası> {
        let mut ilk = None;
        for örnek in SparseÖrneği::TÜMÜ {
            let (seçenekler, veri) = sparse_kartı(örnek)?;
            assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (800, 200));
            assert_eq!(veri.uzunluk(), 13_608);
            assert_eq!(
                veri.seriler()
                    .first()
                    .map(|seri| seri.iter().flatten().count()),
                Some(4_608)
            );
            if let Some(beklenen) = &ilk {
                assert_eq!(&veri, beklenen);
            } else {
                ilk = Some(veri);
            }
        }
        Ok(())
    }

    #[test]
    fn özel_nokta_ve_saf_yol_çekirdekte_ayrılır() -> Result<(), UplotHatası> {
        let (nokta_seçenekleri, nokta_verisi) = sparse_kartı(SparseÖrneği::ÖzelNoktalar)?;
        let nokta_serisi = nokta_seçenekleri.seriler.first();
        assert_eq!(
            nokta_serisi.map(|seri| seri.çizim_türü),
            Some(SeriÇizimTürü::Noktalar)
        );
        assert_eq!(
            nokta_serisi.map(|seri| seri.nokta_şekli),
            Some(NoktaŞekli::Kare)
        );
        let beklenen_kare_sayısı = nokta_verisi
            .seriler()
            .first()
            .map(|seri| {
                seri.iter()
                    .flatten()
                    .filter(|değer| (100.0..=350.0).contains(*değer))
                    .count()
            })
            .unwrap_or_default();
        let sahne = Grafik::yeni(nokta_seçenekleri, nokta_verisi)?.çiz();
        assert!(sahne.komutlar().iter().any(|komut| match komut {
            Komut::Alan { çokgenler, dolgu } if dolgu == "red" => {
                çokgenler.len() == beklenen_kare_sayısı
                    && çokgenler.iter().all(|kare| match kare.as_slice() {
                        [ilk, sağ_üst, _, sol_alt] => {
                            (sağ_üst.x - ilk.x - 2.0).abs() <= f32::EPSILON
                                && (sol_alt.y - ilk.y - 2.0).abs() <= f32::EPSILON
                        }
                        _ => false,
                    })
            }
            _ => false,
        }));

        let (saf_seçenekleri, _) = sparse_kartı(SparseÖrneği::ÖzelSafDoğrusal)?;
        assert!(
            saf_seçenekleri
                .seriler
                .first()
                .is_some_and(|seri| seri.saf_doğrusal_yol)
        );
        Ok(())
    }

    #[test]
    fn null_koşuları_yerleşik_ve_saf_yolda_ayrı_parçalardır() -> Result<(), UplotHatası> {
        for örnek in [
            SparseÖrneği::YerleşikDoğrusal,
            SparseÖrneği::ÖzelSafDoğrusal,
        ] {
            let (seçenekler, veri) = sparse_kartı(örnek)?;
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            let parça_sayısı = sahne.komutlar().iter().find_map(|komut| match komut {
                Komut::Yol { parçalar, .. } => Some(parçalar.len()),
                _ => None,
            });
            assert!(parça_sayısı.is_some_and(|sayı| sayı > 1));
        }
        Ok(())
    }

    #[test]
    fn native_yol_seyrekleşir_saf_yol_tüm_dolu_noktaları_korur() -> Result<(), UplotHatası> {
        let yol_noktaları = |örnek| -> Result<usize, UplotHatası> {
            let (seçenekler, veri) = sparse_kartı(örnek)?;
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            Ok(sahne
                .komutlar()
                .iter()
                .find_map(|komut| match komut {
                    Komut::Yol { parçalar, .. } => {
                        Some(parçalar.iter().map(Vec::len).sum::<usize>())
                    }
                    _ => None,
                })
                .unwrap_or_default())
        };
        let native = yol_noktaları(SparseÖrneği::YerleşikDoğrusal)?;
        let saf = yol_noktaları(SparseÖrneği::ÖzelSafDoğrusal)?;
        assert!(native < saf, "native={native}, saf={saf}");
        assert_eq!(saf, 4_471);
        Ok(())
    }

    #[test]
    fn üç_kaynak_yüzeyi_tek_grup_api_ile_üretilir() -> Result<(), UplotHatası> {
        let kartlar = sparse_kartları()?;
        assert_eq!(kartlar.len(), 3);
        assert_eq!(
            kartlar.iter().map(|kart| kart.0).collect::<Vec<_>>(),
            SparseÖrneği::TÜMÜ
        );
        Ok(())
    }
}
