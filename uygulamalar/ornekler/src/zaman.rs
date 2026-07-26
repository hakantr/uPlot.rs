#[cfg(test)]
use crate::ZamanDilimi;

#[cfg(test)]
pub(crate) fn utc_alanları(zaman_damgası: f64) -> Option<(i64, u32, u32, u32, u32, u32)> {
    if !zaman_damgası.is_finite() {
        return None;
    }
    let saniye = zaman_damgası.floor() as i64;
    let gün = saniye.div_euclid(86_400);
    let gün_saniyesi = saniye.rem_euclid(86_400);
    let (yıl, ay, ayın_günü) = günlerden_tarihe(gün);
    Some((
        yıl,
        ay,
        ayın_günü,
        u32::try_from(gün_saniyesi / 3_600).ok()?,
        u32::try_from((gün_saniyesi % 3_600) / 60).ok()?,
        u32::try_from(gün_saniyesi % 60).ok()?,
    ))
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

#[cfg(test)]
pub(crate) fn zaman_dilimi_ofseti(zaman_dilimi: ZamanDilimi, zaman: f64) -> i32 {
    let Some((yıl, _, _, _, _, _)) = utc_alanları(zaman) else {
        return 0;
    };
    match zaman_dilimi {
        ZamanDilimi::Utc => 0,
        ZamanDilimi::EuropeLondon => {
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
        ZamanDilimi::AmericaChicago => {
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

#[cfg(test)]
fn ayın_ninci_pazarı(yıl: i64, ay: u32, sıra: u32) -> Option<u32> {
    if sıra == 0 {
        return None;
    }
    let ilk = utc_zaman_damgası(yıl, ay, 1)? as i64 / 86_400;
    let ilk_hafta_günü = (ilk + 4).rem_euclid(7) as u32;
    let gün = 1 + (7 - ilk_hafta_günü) % 7 + (sıra - 1) * 7;
    (gün <= aydaki_gün_sayısı(yıl, ay)?).then_some(gün)
}

#[cfg(test)]
fn ayın_son_pazarı(yıl: i64, ay: u32) -> Option<u32> {
    let son_gün = aydaki_gün_sayısı(yıl, ay)?;
    let son = utc_zaman_damgası(yıl, ay, son_gün)? as i64 / 86_400;
    Some(son_gün - (son + 4).rem_euclid(7) as u32)
}

#[cfg(test)]
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

#[cfg(test)]
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
