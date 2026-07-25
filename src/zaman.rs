/// Unix saniyesini UTC takvim alanlarına dönüştürür. Algoritma proleptik
/// Gregoryen takvimde çalışır ve platform saat dilimine bağlı değildir.
pub(crate) fn utc_alanları(zaman_damgası: f64) -> Option<(i64, u32, u32, u32, u32, u32)> {
    if !zaman_damgası.is_finite() {
        return None;
    }
    let saniye = zaman_damgası.floor() as i64;
    let gün = saniye.div_euclid(86_400);
    let gün_saniyesi = saniye.rem_euclid(86_400);
    let (yıl, ay, ayın_günü) = günlerden_tarihe(gün);
    let saat = u32::try_from(gün_saniyesi / 3_600).ok()?;
    let dakika = u32::try_from((gün_saniyesi % 3_600) / 60).ok()?;
    let saniye = u32::try_from(gün_saniyesi % 60).ok()?;
    Some((yıl, ay, ayın_günü, saat, dakika, saniye))
}

pub(crate) fn utc_zaman_damgası(yıl: i64, ay: u32, gün: u32) -> Option<f64> {
    if !(1..=12).contains(&ay) || !(1..=31).contains(&gün) {
        return None;
    }
    let ay = i64::from(ay);
    let yıl_düzeltildi = yıl - i64::from(ay <= 2);
    let çağ = yıl_düzeltildi.div_euclid(400);
    let çağ_yılı = yıl_düzeltildi - çağ * 400;
    let mart_ayı = ay + if ay > 2 { -3 } else { 9 };
    let yıl_günü = (153 * mart_ayı + 2) / 5 + i64::from(gün) - 1;
    let çağ_günü = çağ_yılı * 365 + çağ_yılı / 4 - çağ_yılı / 100 + yıl_günü;
    Some(((çağ * 146_097 + çağ_günü - 719_468) * 86_400) as f64)
}

pub(crate) fn tooltip_tarihi(zaman: f64) -> Option<String> {
    let (yıl, ay, gün, saat24, dakika, saniye) = utc_alanları(zaman)?;
    let dönem = if saat24 < 12 { "AM" } else { "PM" };
    let saat = match saat24 % 12 {
        0 => 12,
        değer => değer,
    };
    Some(format!(
        "{ay}/{gün}/{:02} {saat}:{dakika:02}:{saniye:02} {dönem}",
        yıl.rem_euclid(100)
    ))
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ZamanEtiketDurumu {
    yıl: Option<i64>,
    ay: Option<u32>,
    gün: Option<u32>,
    saat: Option<u32>,
    dakika: Option<u32>,
    saniye: Option<u32>,
}

pub(crate) fn yerel_eksen_etiketi(
    zaman_damgası: f64,
    artım: f64,
    adlar: &crate::TarihAdları,
    zaman_dilimi: crate::ZamanDilimi,
    durum: &mut ZamanEtiketDurumu,
) -> Option<String> {
    let yerel_zaman = zaman_damgası + f64::from(zaman_dilimi_ofseti(zaman_dilimi, zaman_damgası));
    let (yıl, ay, gün, saat, dakika, saniye) = utc_alanları(yerel_zaman)?;
    let dönem = if saat < 12 { "am" } else { "pm" };
    let saat12 = match saat % 12 {
        0 => 12,
        değer => değer,
    };
    let yeni_yıl = durum.yıl != Some(yıl);
    let yeni_gün = durum.gün != Some(gün) || durum.ay != Some(ay) || yeni_yıl;
    let yeni_dakika = durum.dakika != Some(dakika) || durum.saat != Some(saat) || yeni_gün;
    let ay_gün = format!("{ay}/{gün}");
    let kısa_yıl = yıl.rem_euclid(100);
    let saat_dakika = format!("{saat12}:{dakika:02}{dönem}");
    let içerik = if artım >= 28.0 * 86_400.0 {
        let ay_adı = adlar.kısa_ay(ay)?;
        if yeni_yıl {
            format!("{ay_adı}\n{yıl:04}")
        } else {
            ay_adı.to_string()
        }
    } else if artım >= 86_400.0 {
        if yeni_yıl {
            format!("{ay_gün}\n{yıl:04}")
        } else {
            ay_gün
        }
    } else if artım >= 3_600.0 {
        let ana = format!("{saat12}{dönem}");
        if yeni_yıl {
            format!("{ana}\n{ay_gün}/{kısa_yıl:02}")
        } else if yeni_gün {
            format!("{ana}\n{ay_gün}")
        } else {
            ana
        }
    } else if artım >= 60.0 {
        if yeni_yıl {
            format!("{saat_dakika}\n{ay_gün}/{kısa_yıl:02}")
        } else if yeni_gün {
            format!("{saat_dakika}\n{ay_gün}")
        } else {
            saat_dakika
        }
    } else {
        let milisaniye = ((yerel_zaman - yerel_zaman.floor()) * 1_000.0)
            .round()
            .clamp(0.0, 999.0) as u32;
        let ana = if artım >= 1.0 {
            format!(":{saniye:02}")
        } else {
            format!(":{saniye:02}.{milisaniye:03}")
        };
        if yeni_yıl {
            format!("{ana}\n{ay_gün}/{kısa_yıl:02} {saat_dakika}")
        } else if yeni_gün {
            format!("{ana}\n{ay_gün} {saat_dakika}")
        } else if yeni_dakika {
            format!("{ana}\n{saat_dakika}")
        } else {
            ana
        }
    };
    *durum = ZamanEtiketDurumu {
        yıl: Some(yıl),
        ay: Some(ay),
        gün: Some(gün),
        saat: Some(saat),
        dakika: Some(dakika),
        saniye: Some(saniye),
    };
    Some(içerik)
}

pub(crate) fn zaman_dilimi_ofseti(zaman_dilimi: crate::ZamanDilimi, zaman: f64) -> i32 {
    let Some((yıl, _, _, _, _, _)) = utc_alanları(zaman) else {
        return 0;
    };
    match zaman_dilimi {
        crate::ZamanDilimi::Utc => 0,
        crate::ZamanDilimi::EuropeLondon => {
            let başlangıç = ayın_son_pazarı(yıl, 3)
                .and_then(|gün| utc_zaman_damgası(yıl, 3, gün))
                .map(|zaman| zaman + 3_600.0);
            let bitiş = ayın_son_pazarı(yıl, 10)
                .and_then(|gün| utc_zaman_damgası(yıl, 10, gün))
                .map(|zaman| zaman + 3_600.0);
            if başlangıç
                .zip(bitiş)
                .is_some_and(|(başlangıç, bitiş)| (başlangıç..bitiş).contains(&zaman))
            {
                3_600
            } else {
                0
            }
        }
        crate::ZamanDilimi::AmericaChicago => {
            let başlangıç = ayın_ninci_pazarı(yıl, 3, 2)
                .and_then(|gün| utc_zaman_damgası(yıl, 3, gün))
                .map(|zaman| zaman + 8.0 * 3_600.0);
            let bitiş = ayın_ninci_pazarı(yıl, 11, 1)
                .and_then(|gün| utc_zaman_damgası(yıl, 11, gün))
                .map(|zaman| zaman + 7.0 * 3_600.0);
            if başlangıç
                .zip(bitiş)
                .is_some_and(|(başlangıç, bitiş)| (başlangıç..bitiş).contains(&zaman))
            {
                -5 * 3_600
            } else {
                -6 * 3_600
            }
        }
    }
}

fn ayın_ninci_pazarı(yıl: i64, ay: u32, sıra: u32) -> Option<u32> {
    if sıra == 0 {
        return None;
    }
    let ilk = utc_zaman_damgası(yıl, ay, 1)? as i64 / 86_400;
    let ilk_hafta_günü = (ilk + 4).rem_euclid(7) as u32;
    let gün = 1 + (7 - ilk_hafta_günü) % 7 + (sıra - 1) * 7;
    (gün <= aydaki_gün_sayısı(yıl, ay)?).then_some(gün)
}

fn ayın_son_pazarı(yıl: i64, ay: u32) -> Option<u32> {
    let son_gün = aydaki_gün_sayısı(yıl, ay)?;
    let son = utc_zaman_damgası(yıl, ay, son_gün)? as i64 / 86_400;
    let son_hafta_günü = (son + 4).rem_euclid(7) as u32;
    Some(son_gün - son_hafta_günü)
}

fn aydaki_gün_sayısı(yıl: i64, ay: u32) -> Option<u32> {
    match ay {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if yıl % 4 == 0 && (yıl % 100 != 0 || yıl % 400 == 0) {
            29
        } else {
            28
        }),
        _ => None,
    }
}

fn günlerden_tarihe(gün: i64) -> (i64, u32, u32) {
    let z = gün + 719_468;
    let çağ = z.div_euclid(146_097);
    let çağ_günü = z - çağ * 146_097;
    let çağ_yılı = (çağ_günü - çağ_günü / 1_460 + çağ_günü / 36_524 - çağ_günü / 146_096) / 365;
    let mut yıl = çağ_yılı + çağ * 400;
    let yıl_günü = çağ_günü - (365 * çağ_yılı + çağ_yılı / 4 - çağ_yılı / 100);
    let mart_ayı = (5 * yıl_günü + 2) / 153;
    let gün = yıl_günü - (153 * mart_ayı + 2) / 5 + 1;
    let ay = mart_ayı + if mart_ayı < 10 { 3 } else { -9 };
    yıl += i64::from(ay <= 2);
    (
        yıl,
        u32::try_from(ay).unwrap_or(1),
        u32::try_from(gün).unwrap_or(1),
    )
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn artik_yıl_ve_utc_etiketi_korunur() {
        let zaman = utc_zaman_damgası(2024, 2, 1);
        let mut durum = ZamanEtiketDurumu::default();
        assert_eq!(zaman.and_then(utc_alanları), Some((2024, 2, 1, 0, 0, 0)));
        assert_eq!(
            zaman.and_then(|z| {
                yerel_eksen_etiketi(
                    z,
                    31.0 * 86_400.0,
                    &crate::TarihAdları::ingilizce(),
                    crate::ZamanDilimi::Utc,
                    &mut durum,
                )
            }),
            Some("Feb\n2024".to_string())
        );
        let mut durum = ZamanEtiketDurumu::default();
        assert_eq!(
            zaman.and_then(|z| {
                yerel_eksen_etiketi(
                    z,
                    3_600.0,
                    &crate::TarihAdları::ingilizce(),
                    crate::ZamanDilimi::Utc,
                    &mut durum,
                )
            }),
            Some("12am\n2/1/24".to_string())
        );
        let mut durum = ZamanEtiketDurumu::default();
        assert_eq!(
            zaman.and_then(|z| {
                yerel_eksen_etiketi(
                    z,
                    31.0 * 86_400.0,
                    &crate::TarihAdları::rusça(),
                    crate::ZamanDilimi::Utc,
                    &mut durum,
                )
            }),
            Some("Февр\n2024".to_string())
        );
    }

    #[test]
    fn zaman_etiketleri_kaynak_rollover_durumunu_korur() {
        let mut durum = ZamanEtiketDurumu::default();
        let etiketler = [
            (2024, 12, 31, 23, 0, "11pm\n12/31/24"),
            (2025, 1, 1, 0, 0, "12am\n1/1/25"),
            (2025, 1, 1, 1, 0, "1am"),
        ]
        .map(|(yıl, ay, gün, saat, dakika, beklenen)| {
            let zaman = utc_zaman_damgası(yıl, ay, gün)
                .map(|zaman| zaman + f64::from(saat * 3_600 + dakika * 60));
            let gerçek = zaman.and_then(|zaman| {
                yerel_eksen_etiketi(
                    zaman,
                    3_600.0,
                    &crate::TarihAdları::ingilizce(),
                    crate::ZamanDilimi::Utc,
                    &mut durum,
                )
            });
            (gerçek, beklenen)
        });
        assert!(
            etiketler
                .iter()
                .all(|(gerçek, beklenen)| gerçek.as_deref() == Some(*beklenen))
        );
    }

    #[test]
    fn dst_kuralları_yıldan_bağımsız_hesaplanır() {
        let london_önce = utc_zaman_damgası(2025, 3, 30).map(|zaman| zaman + 3_599.0);
        let london_sonra = utc_zaman_damgası(2025, 3, 30).map(|zaman| zaman + 3_600.0);
        assert_eq!(
            london_önce.map(|zaman| zaman_dilimi_ofseti(crate::ZamanDilimi::EuropeLondon, zaman)),
            Some(0)
        );
        assert_eq!(
            london_sonra.map(|zaman| zaman_dilimi_ofseti(crate::ZamanDilimi::EuropeLondon, zaman)),
            Some(3_600)
        );

        let chicago_önce = utc_zaman_damgası(2025, 3, 9).map(|zaman| zaman + 28_799.0);
        let chicago_sonra = utc_zaman_damgası(2025, 3, 9).map(|zaman| zaman + 28_800.0);
        assert_eq!(
            chicago_önce
                .map(|zaman| zaman_dilimi_ofseti(crate::ZamanDilimi::AmericaChicago, zaman)),
            Some(-6 * 3_600)
        );
        assert_eq!(
            chicago_sonra
                .map(|zaman| zaman_dilimi_ofseti(crate::ZamanDilimi::AmericaChicago, zaman)),
            Some(-5 * 3_600)
        );
    }
}
