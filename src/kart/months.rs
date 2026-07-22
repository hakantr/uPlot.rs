use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const MONTHS_KANIT_TOHUMU: u32 = 0x4D4F_4E54;

pub const MONTHS_KART_TANIM_ÖRNEĞİ: &str = r##"let artık_yılsız = months_artık_yılsız_kartı()?;
let artık_yıllı = months_artık_yıllı_kartı()?;
// İki kaynak grafiği aynı UTC ay ekseni ve ortak çekirdek
// etkileşimleriyle ayrı yüzeylerde gösterilir."##;

pub fn months_artık_yılsız_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    months_kartı("No leap year", &[2017, 2018, 2019], MONTHS_KANIT_TOHUMU)
}

pub fn months_artık_yıllı_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    months_kartı(
        "2024 leap year",
        &[2024, 2025, 2026],
        MONTHS_KANIT_TOHUMU.wrapping_add(1),
    )
}

fn months_kartı(
    başlık: &str,
    yıllar: &[i64],
    tohum: u32,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let mut x = Vec::with_capacity(yıllar.len().saturating_mul(12));
    for yıl in yıllar {
        for ay in 1_u32..=12 {
            let zaman = crate::zaman::utc_zaman_damgası(*yıl, ay, 1).ok_or(
                UplotHatası::GeçersizTarih {
                    yıl: *yıl,
                    ay,
                    gün: 1,
                },
            )?;
            x.push(zaman);
        }
    }
    let mut rastgele = KanıtRastgele::yeni(tohum);
    let y = x
        .iter()
        .enumerate()
        .map(|(indeks, _)| {
            if indeks == 0 {
                Some(5.0)
            } else {
                Some((rastgele.sonraki() * 11.0).floor())
            }
        })
        .collect();
    let seçenekler = GrafikSeçenekleri::yeni(1920, 200)?
        .başlık(başlık)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("Value").renk("#ff0000"));
    Ok((seçenekler, HizalıVeri::yeni(x, vec![y])?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn iki_kaynak_grafiği_utc_aylarını_ve_artık_yılı_korur() -> Result<(), UplotHatası> {
        let (normal_seçenekler, normal_veri) = months_artık_yılsız_kartı()?;
        let (artık_seçenekler, artık_veri) = months_artık_yıllı_kartı()?;
        assert_eq!(normal_veri.uzunluk(), 36);
        assert_eq!(artık_veri.uzunluk(), 36);
        assert_eq!(
            normal_veri
                .seriler()
                .first()
                .and_then(|seri| seri.first())
                .copied(),
            Some(Some(5.0))
        );
        assert_eq!(
            artık_veri
                .seriler()
                .first()
                .and_then(|seri| seri.first())
                .copied(),
            Some(Some(5.0))
        );
        let şubat = artık_veri
            .x()
            .get(1)
            .copied()
            .and_then(crate::zaman::utc_alanları);
        assert_eq!(şubat, Some((2024, 2, 1, 0, 0, 0)));

        let normal_sahne = Grafik::yeni(normal_seçenekler, normal_veri)?.çiz();
        let artık_sahne = Grafik::yeni(artık_seçenekler, artık_veri)?.çiz();
        for sahne in [&normal_sahne, &artık_sahne] {
            assert!(sahne.komutlar().iter().any(
                |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik.starts_with("20"))
            ));
        }
        Ok(())
    }
}
