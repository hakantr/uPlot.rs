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

pub(crate) fn yerel_eksen_etiketi(
    zaman_damgası: f64,
    artım: f64,
    adlar: &crate::TarihAdları,
    zaman_dilimi: crate::ZamanDilimi,
    önceki_yıl: Option<i64>,
) -> Option<(String, i64)> {
    let yerel_zaman = zaman_damgası + f64::from(zaman_dilimi_ofseti(zaman_dilimi, zaman_damgası));
    let (yıl, ay, gün, saat, dakika, saniye) = utc_alanları(yerel_zaman)?;
    let içerik = if artım >= 28.0 * 86_400.0 {
        let ay_adı = adlar.kısa_ay(ay)?;
        if önceki_yıl == Some(yıl) {
            ay_adı.to_string()
        } else {
            format!("{ay_adı}\n{yıl:04}")
        }
    } else if artım >= 86_400.0 {
        format!("{ay:02}-{gün:02}")
    } else if artım >= 60.0 && saat == 0 && dakika == 0 {
        format!("{ay:02}-{gün:02} {saat:02}:{dakika:02}")
    } else if artım >= 60.0 {
        format!("{saat:02}:{dakika:02}")
    } else {
        format!("{saat:02}:{dakika:02}:{saniye:02}")
    };
    Some((içerik, yıl))
}

pub(crate) fn zaman_dilimi_ofseti(zaman_dilimi: crate::ZamanDilimi, zaman: f64) -> i32 {
    match zaman_dilimi {
        crate::ZamanDilimi::Utc => 0,
        crate::ZamanDilimi::EuropeLondon => {
            if (1_711_846_800.0..1_729_990_800.0).contains(&zaman) {
                3_600
            } else {
                0
            }
        }
        crate::ZamanDilimi::AmericaChicago => {
            if (1_710_057_600.0..1_730_617_200.0).contains(&zaman) {
                -5 * 3_600
            } else {
                -6 * 3_600
            }
        }
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
        assert_eq!(zaman.and_then(utc_alanları), Some((2024, 2, 1, 0, 0, 0)));
        assert_eq!(
            zaman.and_then(|z| {
                yerel_eksen_etiketi(
                    z,
                    31.0 * 86_400.0,
                    &crate::TarihAdları::ingilizce(),
                    crate::ZamanDilimi::Utc,
                    None,
                )
            }),
            Some(("Feb\n2024".to_string(), 2024))
        );
        assert_eq!(
            zaman.and_then(|z| {
                yerel_eksen_etiketi(
                    z,
                    3_600.0,
                    &crate::TarihAdları::ingilizce(),
                    crate::ZamanDilimi::Utc,
                    None,
                )
            }),
            Some(("02-01 00:00".to_string(), 2024))
        );
        assert_eq!(
            zaman.and_then(|z| {
                yerel_eksen_etiketi(
                    z,
                    31.0 * 86_400.0,
                    &crate::TarihAdları::rusça(),
                    crate::ZamanDilimi::Utc,
                    None,
                )
            }),
            Some(("Февр\n2024".to_string(), 2024))
        );
    }
}
