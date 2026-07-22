use std::collections::BTreeMap;

use serde::Deserialize;

use super::ortak_kart_etkileşimleri;
use crate::{GrafikSeçenekleri, HizalıVeri, KutuBıyıkDüzeni, SeriSeçenekleri, UplotHatası};

const RESULTS_JSON: &str = include_str!("veri/box_whisker_results.json");

pub const BOX_WHISKER_BENCHMARKLERİ: [&str; 17] = [
    "01_run1k",
    "02_replace1k",
    "03_update10th1k_x16",
    "04_select1k",
    "05_swap1k",
    "06_remove-one-1k",
    "07_create10k",
    "08_create1k-after1k_x2",
    "09_clear1k_x8",
    "21_ready-memory",
    "22_run-memory",
    "23_update5-memory",
    "24_run5-memory",
    "25_run-clear-memory",
    "31_startup-ci",
    "32_startup-bt",
    "34_startup-totalbytes",
];

pub const BOX_WHISKER_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = box_whisker_kartı("01_run1k")?;
// Medyan, çeyrekler, bıyıklar, ayrık değerler ve hover sütunu çekirdekte çözülür.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

#[derive(Deserialize)]
struct KaynakSonuç {
    framework: String,
    benchmark: String,
    values: Vec<f64>,
}

struct Özet {
    framework: String,
    medyan: f64,
    q1: f64,
    q3: f64,
    en_az: f64,
    en_çok: f64,
    ayrıklar: Vec<f64>,
}

/// Resmî sayfadaki 17 benchmark kutu-bıyık grafiğinden birini üretir.
pub fn box_whisker_kartı(
    benchmark: &str,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    if !BOX_WHISKER_BENCHMARKLERİ.contains(&benchmark) {
        return Err(UplotHatası::BilinmeyenKart {
            kimlik: format!("box-whisker-{benchmark}"),
        });
    }
    let sonuçlar: Vec<KaynakSonuç> =
        serde_json::from_str(RESULTS_JSON).map_err(|hata| UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/data/results.json",
            açıklama: hata.to_string(),
        })?;
    let mut benchmarkler: BTreeMap<String, Vec<Özet>> = BTreeMap::new();
    for sonuç in sonuçlar {
        if sonuç.framework.contains("non-keyed") {
            continue;
        }
        let özet = özetle(sonuç.framework, sonuç.values)?;
        benchmarkler.entry(sonuç.benchmark).or_default().push(özet);
    }
    let Some(mut özetler) = benchmarkler.remove(benchmark) else {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/data/results.json",
            açıklama: format!("{benchmark} benchmark'i bulunamadı"),
        });
    };
    özetler.sort_by(|sol, sağ| sol.medyan.total_cmp(&sağ.medyan));
    özetler.truncate(30);
    let x = (0..özetler.len()).map(|indeks| indeks as f64).collect();
    let seriler = [
        özetler.iter().map(|özet| Some(özet.medyan)).collect(),
        özetler.iter().map(|özet| Some(özet.q1)).collect(),
        özetler.iter().map(|özet| Some(özet.q3)).collect(),
        özetler.iter().map(|özet| Some(özet.en_az)).collect(),
        özetler.iter().map(|özet| Some(özet.en_çok)).collect(),
    ];
    let veri = HizalıVeri::yeni(x, seriler.into())?;
    let kategoriler = özetler
        .iter()
        .map(|özet| özet.framework.clone())
        .collect::<Vec<_>>();
    let ayrıklar = özetler.into_iter().map(|özet| özet.ayrıklar).collect();
    let mut seçenekler = GrafikSeçenekleri::yeni(800, 400)?
        .başlık(benchmark)
        .x_zaman(false)
        .kategoriler(kategoriler)
        .kutu_bıyık_düzeni(KutuBıyıkDüzeni::yeni(ayrıklar))
        .etkileşimler(ortak_kart_etkileşimleri());
    for etiket in ["Median", "q1", "q3", "min", "max"] {
        seçenekler = seçenekler.seri(
            SeriSeçenekleri::yeni(etiket)
                .renk("#000000")
                .çizgi_kalınlığı(0.0),
        );
    }
    Ok((seçenekler, veri))
}

fn özetle(framework: String, mut değerler: Vec<f64>) -> Result<Özet, UplotHatası> {
    if değerler.is_empty() || değerler.iter().any(|değer| !değer.is_finite()) {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/data/results.json",
            açıklama: format!("{framework} boş veya sonlu olmayan ölçüm içeriyor"),
        });
    }
    değerler.sort_by(f64::total_cmp);
    let medyan = yüzdebirlik(&değerler, 0.5);
    let q1 = yüzdebirlik(&değerler, 0.25);
    let q3 = yüzdebirlik(&değerler, 0.75);
    let pay = (q3 - q1) * 1.5;
    let mut iç = Vec::new();
    let mut ayrıklar = Vec::new();
    for değer in değerler {
        if değer >= q1 - pay && değer <= q3 + pay {
            iç.push(değer);
        } else {
            ayrıklar.push(değer);
        }
    }
    let Some(en_az) = iç.first().copied() else {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/data/results.json",
            açıklama: format!("{framework} için bıyık aralığı üretilemedi"),
        });
    };
    let en_çok = iç.last().copied().unwrap_or(en_az);
    Ok(Özet {
        framework,
        medyan: iki_basamak(medyan),
        q1: iki_basamak(q1),
        q3: iki_basamak(q3),
        en_az,
        en_çok,
        ayrıklar,
    })
}

fn yüzdebirlik(değerler: &[f64], oran: f64) -> f64 {
    let Some(ilk) = değerler.first().copied() else {
        return 0.0;
    };
    let uzunluk = değerler.len();
    if oran >= 1.0 {
        return değerler.last().copied().unwrap_or(ilk);
    }
    let n = oran * (uzunluk.saturating_sub(1)) as f64 + 1.0;
    let konum = oran * (uzunluk.saturating_add(1)) as f64;
    let (sol, sağ) = if konum >= 1.0 {
        let taban = n.floor() as usize;
        let sol = taban
            .checked_sub(1)
            .and_then(|indeks| değerler.get(indeks))
            .copied()
            .unwrap_or(ilk);
        let sağ = değerler.get(taban).copied().unwrap_or(sol);
        (sol, sağ)
    } else {
        (ilk, değerler.get(1).copied().unwrap_or(ilk))
    };
    if sol == sağ {
        sol
    } else {
        sol + (n - n.floor()) * (sağ - sol)
    }
}

fn iki_basamak(değer: f64) -> f64 {
    (değer * 100.0).round() / 100.0
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn tüm_kaynak_benchmarkleri_otuz_kutuya_kadar_çizilir() -> Result<(), UplotHatası> {
        for benchmark in BOX_WHISKER_BENCHMARKLERİ {
            let (seçenekler, veri) = box_whisker_kartı(benchmark)?;
            assert!(!veri.seriler().is_empty());
            assert!(veri.uzunluk() <= 30);
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            assert!(sahne.komutlar().iter().any(|komut| {
                matches!(komut, Komut::KesikliÇizgi { kesik, .. } if *kesik == 4.0)
            }));
        }
        Ok(())
    }

    #[test]
    fn ilk_kaynak_medyani_stats_js_ile_aynıdır() -> Result<(), UplotHatası> {
        let (_, veri) = box_whisker_kartı("01_run1k")?;
        let ilk = veri
            .seriler()
            .first()
            .and_then(|seri| seri.first())
            .copied()
            .flatten();
        assert!(ilk.is_some());
        assert_eq!(ilk, Some(117.97));
        Ok(())
    }
}
