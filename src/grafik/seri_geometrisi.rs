use crate::Nokta;

pub(super) fn seri_yol_noktaları(
    noktalar: Vec<Nokta>, tür: crate::SeriÇizimTürü
) -> Vec<Nokta> {
    if noktalar.len() < 2
        || matches!(
            tür,
            crate::SeriÇizimTürü::Çizgi
                | crate::SeriÇizimTürü::Noktalar
                | crate::SeriÇizimTürü::Çubuk
        )
    {
        return noktalar;
    }
    if tür == crate::SeriÇizimTürü::Eğri {
        return monoton_eğri_noktaları(&noktalar);
    }
    if tür == crate::SeriÇizimTürü::CatmullRom {
        return catmull_rom_noktaları(&noktalar);
    }
    let mut sonuç = Vec::with_capacity(noktalar.len().saturating_mul(8));
    if let Some(ilk) = noktalar.first().copied() {
        sonuç.push(ilk);
    }
    for indeks in 0..noktalar.len().saturating_sub(1) {
        let Some(p0) = noktalar.get(indeks).copied() else {
            continue;
        };
        let Some(p1) = noktalar.get(indeks + 1).copied() else {
            continue;
        };
        match tür {
            crate::SeriÇizimTürü::BasamakÖnce => {
                sonuç.push(Nokta::yeni(p0.x, p1.y));
                sonuç.push(p1);
            }
            crate::SeriÇizimTürü::BasamakSonra => {
                sonuç.push(Nokta::yeni(p1.x, p0.y));
                sonuç.push(p1);
            }
            crate::SeriÇizimTürü::Eğri | crate::SeriÇizimTürü::CatmullRom => sonuç.push(p1),
            crate::SeriÇizimTürü::Çizgi
            | crate::SeriÇizimTürü::Noktalar
            | crate::SeriÇizimTürü::Çubuk => sonuç.push(p1),
        }
    }
    sonuç
}

pub(super) fn seri_ara_değeri(
    değerler: &[Option<f64>],
    indeks: usize,
    t: f64,
    tür: crate::SeriÇizimTürü,
) -> Option<f64> {
    let başlangıç = değerler.get(indeks).copied().flatten()?;
    let bitiş = değerler.get(indeks + 1).copied().flatten()?;
    match tür {
        crate::SeriÇizimTürü::Çizgi
        | crate::SeriÇizimTürü::Noktalar
        | crate::SeriÇizimTürü::Çubuk => Some(başlangıç + (bitiş - başlangıç) * t),
        crate::SeriÇizimTürü::BasamakÖnce => Some(if t <= f64::EPSILON {
            başlangıç
        } else {
            bitiş
        }),
        crate::SeriÇizimTürü::BasamakSonra => Some(if t >= 1.0 - f64::EPSILON {
            bitiş
        } else {
            başlangıç
        }),
        crate::SeriÇizimTürü::Eğri | crate::SeriÇizimTürü::CatmullRom => {
            let m0 = monoton_eğim_f64(değerler, indeks)?;
            let m1 = monoton_eğim_f64(değerler, indeks + 1)?;
            let t2 = t * t;
            let t3 = t2 * t;
            Some(
                (2.0 * t3 - 3.0 * t2 + 1.0) * başlangıç
                    + (t3 - 2.0 * t2 + t) * m0
                    + (-2.0 * t3 + 3.0 * t2) * bitiş
                    + (t3 - t2) * m1,
            )
        }
    }
}

fn catmull_rom_noktaları(noktalar: &[Nokta]) -> Vec<Nokta> {
    if noktalar.len() < 3 {
        return noktalar.to_vec();
    }
    let mut sonuç = Vec::with_capacity(noktalar.len().saturating_mul(8));
    if let Some(ilk) = noktalar.first().copied() {
        sonuç.push(ilk);
    }
    for indeks in 0..noktalar.len().saturating_sub(1) {
        let Some(p1) = noktalar.get(indeks).copied() else {
            continue;
        };
        let Some(p2) = noktalar.get(indeks + 1).copied() else {
            continue;
        };
        let p0 = indeks
            .checked_sub(1)
            .and_then(|önceki| noktalar.get(önceki))
            .copied()
            .unwrap_or(p1);
        let p3 = noktalar.get(indeks + 2).copied().unwrap_or(p2);
        let (kontrol1, kontrol2) = catmull_kontrol_noktaları(p0, p1, p2, p3);
        for adım in 1..=8 {
            let t = adım as f32 / 8.0;
            let ters = 1.0 - t;
            let x = ters.powi(3) * p1.x
                + 3.0 * ters.powi(2) * t * kontrol1.x
                + 3.0 * ters * t.powi(2) * kontrol2.x
                + t.powi(3) * p2.x;
            let y = ters.powi(3) * p1.y
                + 3.0 * ters.powi(2) * t * kontrol1.y
                + 3.0 * ters * t.powi(2) * kontrol2.y
                + t.powi(3) * p2.y;
            sonuç.push(Nokta::yeni(x, y));
        }
    }
    sonuç
}

fn catmull_kontrol_noktaları(p0: Nokta, p1: Nokta, p2: Nokta, p3: Nokta) -> (Nokta, Nokta) {
    let d1 = uzaklık(p0, p1);
    let d2 = uzaklık(p1, p2);
    let d3 = uzaklık(p2, p3);
    let d1a = d1.sqrt();
    let d2a = d2.sqrt();
    let d3a = d3.sqrt();
    let n = 3.0 * d1a * (d1a + d2a);
    let m = 3.0 * d3a * (d3a + d2a);
    let a = 2.0 * d1 + 3.0 * d1a * d2a + d2;
    let b = 2.0 * d3 + 3.0 * d3a * d2a + d2;
    let kontrol1 = if n > f32::EPSILON {
        Nokta::yeni(
            (-d2 * p0.x + a * p1.x + d1 * p2.x) / n,
            (-d2 * p0.y + a * p1.y + d1 * p2.y) / n,
        )
    } else {
        p1
    };
    let kontrol2 = if m > f32::EPSILON {
        Nokta::yeni(
            (d3 * p1.x + b * p2.x - d2 * p3.x) / m,
            (d3 * p1.y + b * p2.y - d2 * p3.y) / m,
        )
    } else {
        p2
    };
    (kontrol1, kontrol2)
}

fn uzaklık(a: Nokta, b: Nokta) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

pub(super) fn bant_yönünde(fark: f64, yön: crate::BantYönü) -> bool {
    match yön {
        crate::BantYönü::EnAza => fark >= 0.0,
        crate::BantYönü::EnÇoğa => fark <= 0.0,
    }
}

pub(super) fn bant_dilim_çokgeni(
    a: (f32, f32, f32, f64),
    b: (f32, f32, f32, f64),
    yön: crate::BantYönü,
) -> Option<Vec<Nokta>> {
    let a_geçerli = bant_yönünde(a.3, yön);
    let b_geçerli = bant_yönünde(b.3, yön);
    if a_geçerli && b_geçerli {
        return Some(vec![
            Nokta::yeni(a.0, a.1),
            Nokta::yeni(b.0, b.1),
            Nokta::yeni(b.0, b.2),
            Nokta::yeni(a.0, a.2),
        ]);
    }
    if a_geçerli == b_geçerli {
        return None;
    }
    let payda = a.3 - b.3;
    if !payda.is_finite() || payda.abs() <= f64::EPSILON {
        return None;
    }
    let oran = (a.3 / payda).clamp(0.0, 1.0) as f32;
    let kesişim = Nokta::yeni(a.0 + (b.0 - a.0) * oran, a.1 + (b.1 - a.1) * oran);
    if a_geçerli {
        Some(vec![Nokta::yeni(a.0, a.1), kesişim, Nokta::yeni(a.0, a.2)])
    } else {
        Some(vec![kesişim, Nokta::yeni(b.0, b.1), Nokta::yeni(b.0, b.2)])
    }
}

fn monoton_eğri_noktaları(noktalar: &[Nokta]) -> Vec<Nokta> {
    if noktalar.len() < 3 {
        return noktalar.to_vec();
    }
    let eğimler = monoton_eğimleri(noktalar);
    let mut sonuç = Vec::with_capacity(noktalar.len().saturating_mul(8));
    if let Some(ilk) = noktalar.first().copied() {
        sonuç.push(ilk);
    }
    for indeks in 0..noktalar.len().saturating_sub(1) {
        let Some(p0) = noktalar.get(indeks).copied() else {
            continue;
        };
        let Some(p1) = noktalar.get(indeks + 1).copied() else {
            continue;
        };
        let Some(m0) = eğimler.get(indeks).copied() else {
            continue;
        };
        let Some(m1) = eğimler.get(indeks + 1).copied() else {
            continue;
        };
        let dx = p1.x - p0.x;
        for adım in 1..=8 {
            let t = adım as f32 / 8.0;
            let t2 = t * t;
            let t3 = t2 * t;
            let y = (2.0 * t3 - 3.0 * t2 + 1.0) * p0.y
                + (t3 - 2.0 * t2 + t) * dx * m0
                + (-2.0 * t3 + 3.0 * t2) * p1.y
                + (t3 - t2) * dx * m1;
            sonuç.push(Nokta::yeni(p0.x + dx * t, y));
        }
    }
    sonuç
}

fn monoton_eğimleri(noktalar: &[Nokta]) -> Vec<f32> {
    let mut farklar = Vec::with_capacity(noktalar.len().saturating_sub(1));
    for çift in noktalar.windows(2) {
        let Some(a) = çift.first() else { continue };
        let Some(b) = çift.get(1) else { continue };
        let dx = b.x - a.x;
        farklar.push(if dx.abs() <= f32::EPSILON {
            0.0
        } else {
            (b.y - a.y) / dx
        });
    }
    let mut eğimler = vec![0.0; noktalar.len()];
    let Some(ilk_fark) = farklar.first().copied() else {
        return eğimler;
    };
    if let Some(ilk) = eğimler.first_mut() {
        *ilk = ilk_fark;
    }
    if let Some(son) = eğimler.last_mut()
        && let Some(son_fark) = farklar.last().copied()
    {
        *son = son_fark;
    }
    for indeks in 1..noktalar.len().saturating_sub(1) {
        let Some(önceki) = farklar.get(indeks - 1).copied() else {
            continue;
        };
        let Some(sonraki) = farklar.get(indeks).copied() else {
            continue;
        };
        let Some(dx_önceki) = noktalar
            .get(indeks)
            .zip(noktalar.get(indeks - 1))
            .map(|(b, a)| b.x - a.x)
        else {
            continue;
        };
        let Some(dx_sonraki) = noktalar
            .get(indeks + 1)
            .zip(noktalar.get(indeks))
            .map(|(b, a)| b.x - a.x)
        else {
            continue;
        };
        if önceki == 0.0
            || sonraki == 0.0
            || önceki.is_sign_positive() != sonraki.is_sign_positive()
        {
            continue;
        }
        let payda =
            (2.0 * dx_sonraki + dx_önceki) / önceki + (dx_sonraki + 2.0 * dx_önceki) / sonraki;
        let eğim = 3.0 * (dx_önceki + dx_sonraki) / payda;
        if eğim.is_finite()
            && let Some(hedef) = eğimler.get_mut(indeks)
        {
            *hedef = eğim;
        }
    }
    eğimler
}

fn monoton_eğim_f64(değerler: &[Option<f64>], indeks: usize) -> Option<f64> {
    let değer = değerler.get(indeks).copied().flatten()?;
    let önceki = indeks
        .checked_sub(1)
        .and_then(|önceki| değerler.get(önceki))
        .copied()
        .flatten();
    let sonraki = değerler.get(indeks + 1).copied().flatten();
    match (önceki, sonraki) {
        (None, Some(sonraki)) => Some(sonraki - değer),
        (Some(önceki), None) => Some(değer - önceki),
        (Some(önceki), Some(sonraki)) => {
            let sol = değer - önceki;
            let sağ = sonraki - değer;
            if sol == 0.0 || sağ == 0.0 || sol.is_sign_positive() != sağ.is_sign_positive() {
                Some(0.0)
            } else {
                let payda = sol + sağ;
                (payda.abs() > f64::EPSILON).then_some(2.0 * sol * sağ / payda)
            }
        }
        (None, None) => Some(0.0),
    }
}

#[cfg(test)]
mod catmull_rom_testleri {
    use super::*;

    #[test]
    fn düz_çizgi_girdi_tahsisatını_yeniden_kullanır() {
        let kaynak = vec![
            Nokta::yeni(0.0, 0.0),
            Nokta::yeni(1.0, 2.0),
            Nokta::yeni(2.0, 1.0),
        ];
        let kaynak_işaretçisi = kaynak.as_ptr();

        let sonuç = seri_yol_noktaları(kaynak, crate::SeriÇizimTürü::Çizgi);

        assert_eq!(sonuç.as_ptr(), kaynak_işaretçisi);
        assert_eq!(sonuç.len(), 3);
    }

    #[test]
    fn merkezcil_catmull_rom_uçları_korur_ve_sonlu_örnekler_üretir() {
        let kaynak = [
            Nokta::yeni(0.0, 0.0),
            Nokta::yeni(1.0, 2.0),
            Nokta::yeni(2.0, 1.0),
            Nokta::yeni(4.0, 3.0),
        ];
        let sonuç = catmull_rom_noktaları(&kaynak);

        assert_eq!(sonuç.len(), 25);
        assert_eq!(sonuç.first(), kaynak.first());
        assert_eq!(sonuç.last(), kaynak.last());
        assert!(
            sonuç
                .iter()
                .all(|nokta| nokta.x.is_finite() && nokta.y.is_finite())
        );
    }
}
