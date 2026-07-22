use super::Nokta;

pub(crate) fn nokta_dikdörtgende(nokta: Nokta, sol: f32, sağ: f32, üst: f32, alt: f32) -> bool {
    (sol..=sağ).contains(&nokta.x) && (üst..=alt).contains(&nokta.y)
}

pub(crate) fn yolu_dikdörtgene_kırp(
    parçalar: &[Vec<Nokta>],
    sol: f32,
    sağ: f32,
    üst: f32,
    alt: f32,
) -> Vec<Vec<Nokta>> {
    let mut sonuç = Vec::new();
    for parça in parçalar {
        if parça.len() == 1 {
            if let Some(nokta) = parça.first().copied()
                && nokta_dikdörtgende(nokta, sol, sağ, üst, alt)
            {
                sonuç.push(vec![nokta]);
            }
            continue;
        }
        let mut kırpılmış = Vec::new();
        let mut noktalar = parça.iter().copied();
        let Some(mut önceki) = noktalar.next() else {
            continue;
        };
        for sonraki in noktalar {
            if let Some((başlangıç, bitiş)) =
                doğru_parçasını_kırp(önceki, sonraki, sol, sağ, üst, alt)
            {
                if kırpılmış.last().is_some_and(|son| *son != başlangıç) {
                    if kırpılmış.len() >= 2 {
                        sonuç.push(std::mem::take(&mut kırpılmış));
                    } else {
                        kırpılmış.clear();
                    }
                }
                if kırpılmış.is_empty() {
                    kırpılmış.push(başlangıç);
                }
                if kırpılmış.last().is_none_or(|son| *son != bitiş) {
                    kırpılmış.push(bitiş);
                }
            } else if kırpılmış.len() >= 2 {
                sonuç.push(std::mem::take(&mut kırpılmış));
            } else {
                kırpılmış.clear();
            }
            önceki = sonraki;
        }
        if kırpılmış.len() >= 2 {
            sonuç.push(kırpılmış);
        }
    }
    sonuç
}

/// Kapalı bir dolgu çokgenini eksenlerin içindeki dikdörtgene kırpar.
/// Çizgi kırpmasından farklı olarak alanı parçalara ayırıp her parçayı yeniden
/// tabana kapatmaz; böylece sınır dışındaki tepe noktalarında dikey dolgu
/// şeritleri oluşmaz.
pub(crate) fn çokgeni_dikdörtgene_kırp(
    çokgen: &[Nokta],
    sol: f32,
    sağ: f32,
    üst: f32,
    alt: f32,
) -> Vec<Nokta> {
    let mut çıktı = çokgen.to_vec();
    for kenar in [Kenar::Sol, Kenar::Sağ, Kenar::Üst, Kenar::Alt] {
        çıktı = çokgeni_kenara_kırp(&çıktı, kenar, sol, sağ, üst, alt);
        if çıktı.is_empty() {
            break;
        }
    }
    çıktı
}

#[derive(Clone, Copy)]
enum Kenar {
    Sol,
    Sağ,
    Üst,
    Alt,
}

fn çokgeni_kenara_kırp(
    çokgen: &[Nokta],
    kenar: Kenar,
    sol: f32,
    sağ: f32,
    üst: f32,
    alt: f32,
) -> Vec<Nokta> {
    let Some(mut önceki) = çokgen.last().copied() else {
        return Vec::new();
    };
    let mut önceki_içeride = kenarın_içinde(önceki, kenar, sol, sağ, üst, alt);
    let mut çıktı = Vec::new();
    for güncel in çokgen.iter().copied() {
        let güncel_içeride = kenarın_içinde(güncel, kenar, sol, sağ, üst, alt);
        match (önceki_içeride, güncel_içeride) {
            (true, true) => çıktı.push(güncel),
            (true, false) => {
                if let Some(kesişim) = kenar_kesişimi(önceki, güncel, kenar, sol, sağ, üst, alt)
                {
                    çıktı.push(kesişim);
                }
            }
            (false, true) => {
                if let Some(kesişim) = kenar_kesişimi(önceki, güncel, kenar, sol, sağ, üst, alt)
                {
                    çıktı.push(kesişim);
                }
                çıktı.push(güncel);
            }
            (false, false) => {}
        }
        önceki = güncel;
        önceki_içeride = güncel_içeride;
    }
    çıktı
}

fn kenarın_içinde(nokta: Nokta, kenar: Kenar, sol: f32, sağ: f32, üst: f32, alt: f32) -> bool {
    match kenar {
        Kenar::Sol => nokta.x >= sol,
        Kenar::Sağ => nokta.x <= sağ,
        Kenar::Üst => nokta.y >= üst,
        Kenar::Alt => nokta.y <= alt,
    }
}

fn kenar_kesişimi(
    başlangıç: Nokta,
    bitiş: Nokta,
    kenar: Kenar,
    sol: f32,
    sağ: f32,
    üst: f32,
    alt: f32,
) -> Option<Nokta> {
    let dx = bitiş.x - başlangıç.x;
    let dy = bitiş.y - başlangıç.y;
    match kenar {
        Kenar::Sol | Kenar::Sağ => {
            if dx.abs() <= f32::EPSILON {
                return None;
            }
            let x = if matches!(kenar, Kenar::Sol) {
                sol
            } else {
                sağ
            };
            let oran = (x - başlangıç.x) / dx;
            Some(Nokta::yeni(x, başlangıç.y + oran * dy))
        }
        Kenar::Üst | Kenar::Alt => {
            if dy.abs() <= f32::EPSILON {
                return None;
            }
            let y = if matches!(kenar, Kenar::Üst) {
                üst
            } else {
                alt
            };
            let oran = (y - başlangıç.y) / dy;
            Some(Nokta::yeni(başlangıç.x + oran * dx, y))
        }
    }
}

fn doğru_parçasını_kırp(
    başlangıç: Nokta,
    bitiş: Nokta,
    sol: f32,
    sağ: f32,
    üst: f32,
    alt: f32,
) -> Option<(Nokta, Nokta)> {
    let dx = bitiş.x - başlangıç.x;
    let dy = bitiş.y - başlangıç.y;
    let mut en_az_t = 0.0_f32;
    let mut en_çok_t = 1.0_f32;
    if !kırpma_parametresini_uygula(-dx, başlangıç.x - sol, &mut en_az_t, &mut en_çok_t)
        || !kırpma_parametresini_uygula(dx, sağ - başlangıç.x, &mut en_az_t, &mut en_çok_t)
        || !kırpma_parametresini_uygula(-dy, başlangıç.y - üst, &mut en_az_t, &mut en_çok_t)
        || !kırpma_parametresini_uygula(dy, alt - başlangıç.y, &mut en_az_t, &mut en_çok_t)
    {
        return None;
    }
    Some((
        Nokta::yeni(başlangıç.x + en_az_t * dx, başlangıç.y + en_az_t * dy),
        Nokta::yeni(başlangıç.x + en_çok_t * dx, başlangıç.y + en_çok_t * dy),
    ))
}

fn kırpma_parametresini_uygula(p: f32, q: f32, en_az_t: &mut f32, en_çok_t: &mut f32) -> bool {
    if p.abs() <= f32::EPSILON {
        return q >= 0.0;
    }
    let oran = q / p;
    if p < 0.0 {
        *en_az_t = en_az_t.max(oran);
    } else {
        *en_çok_t = en_çok_t.min(oran);
    }
    *en_az_t <= *en_çok_t
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn veri_yolu_dikdörtgenin_üst_ve_alt_sınırında_kesilir() {
        let parçalar = vec![vec![
            Nokta::yeni(0.0, 5.0),
            Nokta::yeni(5.0, -10.0),
            Nokta::yeni(10.0, 20.0),
            Nokta::yeni(15.0, 5.0),
        ]];
        let kırpılmış = yolu_dikdörtgene_kırp(&parçalar, 0.0, 15.0, 0.0, 10.0);

        assert!(!kırpılmış.is_empty());
        assert!(
            kırpılmış.iter().flatten().all(|nokta| {
                (0.0..=15.0).contains(&nokta.x) && (0.0..=10.0).contains(&nokta.y)
            })
        );
        assert!(kırpılmış.iter().flatten().any(|nokta| nokta.y == 0.0));
        assert!(kırpılmış.iter().flatten().any(|nokta| nokta.y == 10.0));
    }

    #[test]
    fn dolgu_cokgeni_sinir_disindaki_uclarda_tek_alan_olarak_kalir() {
        let çokgen = vec![
            Nokta::yeni(-5.0, 5.0),
            Nokta::yeni(5.0, -10.0),
            Nokta::yeni(10.0, 20.0),
            Nokta::yeni(15.0, 5.0),
            Nokta::yeni(15.0, 10.0),
            Nokta::yeni(-5.0, 10.0),
        ];
        let kırpılmış = çokgeni_dikdörtgene_kırp(&çokgen, 0.0, 10.0, 0.0, 10.0);

        assert!(kırpılmış.len() >= 4);
        assert!(
            kırpılmış.iter().all(|nokta| {
                (0.0..=10.0).contains(&nokta.x) && (0.0..=10.0).contains(&nokta.y)
            })
        );
        assert!(kırpılmış.iter().any(|nokta| nokta.y == 0.0));
        assert!(kırpılmış.iter().any(|nokta| nokta.x == 10.0));
    }
}
