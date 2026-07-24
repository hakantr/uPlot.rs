use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, TarihAdları, UplotHatası
};

pub const MONTHS_KANIT_TOHUMU: u32 = 0x4D4F_4E54;
pub const MONTHS_RU_KANIT_TOHUMU: u32 = 0x5255_4D4F;

pub const MONTHS_KART_TANIM_ÖRNEĞİ: &str = r##"let artık_yılsız = months_artık_yılsız_kartı()?;
let artık_yıllı = months_artık_yıllı_kartı()?;
let rusça = months_rusça_kartı()?;
// Üç ilişkili yüzey aynı katalog sayfasında karşılaştırılır. İlk iki
// yüzey months.html içindeki 28 günlük axes.space kuralını, üçüncü
// yüzey months-ru.html içindeki yerelleştirilmiş fmtDate adlarını korur."##;

pub fn months_kartları() -> Result<Vec<(GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    Ok(vec![
        months_artık_yılsız_kartı()?,
        months_artık_yıllı_kartı()?,
        months_rusça_kartı()?,
    ])
}

pub fn months_artık_yılsız_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    months_kartı(
        "No leap year",
        &[2017, 2018, 2019],
        MONTHS_KANIT_TOHUMU,
        200,
        TarihAdları::ingilizce(),
        true,
    )
}

pub fn months_artık_yıllı_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    months_kartı(
        "2024 leap year",
        &[2024, 2025, 2026],
        MONTHS_KANIT_TOHUMU.wrapping_add(1),
        200,
        TarihAdları::ingilizce(),
        true,
    )
}

pub fn months_rusça_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    months_kartı(
        "Months",
        &[2017, 2018, 2019],
        MONTHS_RU_KANIT_TOHUMU,
        600,
        TarihAdları::rusça(),
        false,
    )
}

fn months_kartı(
    başlık: &str,
    yıllar: &[i64],
    tohum: u32,
    yükseklik: u32,
    tarih_adları: TarihAdları,
    kaynak_aylık_bölmeleri: bool,
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
    let mut seçenekler = GrafikSeçenekleri::yeni(1920, yükseklik)?
        .başlık(başlık)
        .x_tarih_adları(tarih_adları)
        .y_aralığı(Aralık::yeni(0.0, 11.0)?)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("Value").renk("#ff0000"));
    if kaynak_aylık_bölmeleri {
        // `months.html` axes[0].space callback'i 28 günlük piksel
        // karşılığını döndürür. uPlot bununla her takvim ayını, yeniden
        // boyutlandırmada da atlamadan bölme olarak seçer.
        seçenekler = seçenekler.x_zaman_sabit_artımı(30.0 * 86_400.0);
    }
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
        let normal_şubat_günleri = normal_veri
            .x()
            .get(1)
            .zip(normal_veri.x().get(2))
            .map(|(şubat, mart)| (mart - şubat) / 86_400.0);
        let artık_şubat_günleri = artık_veri
            .x()
            .get(1)
            .zip(artık_veri.x().get(2))
            .map(|(şubat, mart)| (mart - şubat) / 86_400.0);
        assert_eq!(normal_şubat_günleri, Some(28.0));
        assert_eq!(artık_şubat_günleri, Some(29.0));
        assert_eq!(normal_seçenekler.x_zaman_sabit_artımı, Some(2_592_000.0));
        assert_eq!(artık_seçenekler.x_zaman_sabit_artımı, Some(2_592_000.0));

        let normal_sahne = Grafik::yeni(normal_seçenekler, normal_veri)?.çiz();
        let artık_sahne = Grafik::yeni(artık_seçenekler, artık_veri)?.çiz();
        for sahne in [&normal_sahne, &artık_sahne] {
            assert!(sahne.komutlar().iter().any(
                |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik.contains("20"))
            ));
        }
        Ok(())
    }

    #[test]
    fn kaynak_ailesi_tek_grupta_üç_yüzey_üretir() -> Result<(), UplotHatası> {
        let kartlar = months_kartları()?;
        assert_eq!(kartlar.len(), 3);
        assert_eq!(
            kartlar
                .iter()
                .map(|(seçenekler, _)| seçenekler.başlık.as_str())
                .collect::<Vec<_>>(),
            ["No leap year", "2024 leap year", "Months"]
        );

        let Some((seçenekler, veri)) = kartlar.first() else {
            return Err(UplotHatası::BilinmeyenKart {
                kimlik: "months[0]".to_string(),
            });
        };
        let dar = Grafik::yeni(seçenekler.clone(), veri.clone())?.çiz_görünür_boyutta(600, 200);
        let ay_etiketleri = dar
            .komutlar()
            .iter()
            .filter(|komut| {
                matches!(
                    komut,
                    Komut::Metin { içerik, .. }
                        if ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug",
                            "Sep", "Oct", "Nov", "Dec"]
                            .iter()
                            .any(|ay| içerik.starts_with(ay))
                )
            })
            .count();
        assert_eq!(ay_etiketleri, 36);
        Ok(())
    }

    #[test]
    fn rusça_kart_kaynak_veriyi_ve_yerel_ay_adlarını_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = months_rusça_kartı()?;
        assert_eq!(seçenekler.başlık, "Months");
        assert_eq!(seçenekler.yükseklik, 600);
        assert_eq!(
            seçenekler
                .x_tarih_adları
                .uzun_aylar
                .first()
                .map(String::as_str),
            Some("Январь")
        );
        assert_eq!(
            seçenekler
                .x_tarih_adları
                .kısa_aylar
                .last()
                .map(String::as_str),
            Some("Дек")
        );
        assert_eq!(
            seçenekler
                .x_tarih_adları
                .uzun_hafta_günleri
                .first()
                .map(String::as_str),
            Some("Воскресенье")
        );
        assert_eq!(
            seçenekler
                .x_tarih_adları
                .kısa_hafta_günleri
                .last()
                .map(String::as_str),
            Some("Сбт")
        );
        assert_eq!(veri.uzunluk(), 36);
        assert_eq!(
            veri.seriler()
                .first()
                .and_then(|seri| seri.first())
                .copied(),
            Some(Some(5.0))
        );

        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        assert!(
            sahne.komutlar().iter().any(
                |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik.contains("Янв"))
            )
        );
        assert!(
            sahne.komutlar().iter().any(
                |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik.contains("2017"))
            )
        );
        Ok(())
    }
}
