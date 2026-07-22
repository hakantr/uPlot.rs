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
}
