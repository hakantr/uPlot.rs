use std::{collections::BTreeMap, sync::OnceLock};

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
let grafik = Grafik::yeni(seçenekler, veri)?;

// Kaynak sayfadaki 17 bağımsız benchmark yüzeyini kaynak sırasıyla kurar.
let yüzeyler = box_whisker_kartları()?;"##;

#[derive(Deserialize)]
struct KaynakSonuç {
    framework: String,
    benchmark: String,
    values: Vec<f64>,
}

#[derive(Clone)]
struct Özet {
    framework: String,
    medyan: f64,
    q1: f64,
    q3: f64,
    en_az: Option<f64>,
    en_çok: Option<f64>,
    ayrıklar: Vec<f64>,
}

static KAYNAK_ÖZETLERİ: OnceLock<Result<BTreeMap<String, Vec<Özet>>, UplotHatası>> =
    OnceLock::new();

/// Resmî sayfadaki 17 bağımsız benchmark yüzeyini kaynak sırasıyla döndürür.
pub fn box_whisker_kartları()
-> Result<Vec<(&'static str, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    BOX_WHISKER_BENCHMARKLERİ
        .into_iter()
        .map(|benchmark| {
            box_whisker_kartı(benchmark).map(|(seçenekler, veri)| (benchmark, seçenekler, veri))
        })
        .collect()
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
    let benchmarkler = kaynak_özetleri()?;
    let Some(özetler) = benchmarkler.get(benchmark) else {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/data/results.json",
            açıklama: format!("{benchmark} benchmark'i bulunamadı"),
        });
    };
    let x = (0..özetler.len()).map(|indeks| indeks as f64).collect();
    let seriler = [
        özetler.iter().map(|özet| Some(özet.medyan)).collect(),
        özetler.iter().map(|özet| Some(özet.q1)).collect(),
        özetler.iter().map(|özet| Some(özet.q3)).collect(),
        özetler.iter().map(|özet| özet.en_az).collect(),
        özetler.iter().map(|özet| özet.en_çok).collect(),
    ];
    let veri = HizalıVeri::yeni(x, seriler.into())?;
    let kategoriler = özetler
        .iter()
        .map(|özet| özet.framework.clone())
        .collect::<Vec<_>>();
    let ayrıklar = özetler.iter().map(|özet| özet.ayrıklar.clone()).collect();
    let mut seçenekler = GrafikSeçenekleri::yeni(800, 400)?
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

fn kaynak_özetleri() -> Result<&'static BTreeMap<String, Vec<Özet>>, UplotHatası> {
    KAYNAK_ÖZETLERİ
        .get_or_init(|| {
            let sonuçlar: Vec<KaynakSonuç> =
                serde_json::from_str(RESULTS_JSON).map_err(|hata| {
                    UplotHatası::GeçersizKaynakVeri {
                        varlık: "demos/data/results.json",
                        açıklama: hata.to_string(),
                    }
                })?;
            let mut benchmarkler: BTreeMap<String, Vec<Özet>> = BTreeMap::new();
            for sonuç in sonuçlar {
                if sonuç.framework.contains("non-keyed") {
                    continue;
                }
                let özet = özetle(sonuç.framework, sonuç.values)?;
                benchmarkler.entry(sonuç.benchmark).or_default().push(özet);
            }
            for özetler in benchmarkler.values_mut() {
                özetler.sort_by(|sol, sağ| sol.medyan.total_cmp(&sağ.medyan));
                özetler.truncate(30);
            }
            Ok(benchmarkler)
        })
        .as_ref()
        .map_err(Clone::clone)
}

fn özetle(framework: String, mut değerler: Vec<f64>) -> Result<Özet, UplotHatası> {
    if değerler.is_empty() || değerler.iter().any(|değer| !değer.is_finite()) {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/data/results.json",
            açıklama: format!("{framework} boş veya sonlu olmayan ölçüm içeriyor"),
        });
    }
    değerler.sort_by(f64::total_cmp);
    // `demos/lib/stats.js`, yüzdelikleri iki ondalığa yuvarladıktan sonra
    // box-whisker eklentisinin IQR ve ayrık değer eşiğine verir.
    let medyan = iki_basamak(yüzdebirlik(&değerler, 0.5));
    let q1 = iki_basamak(yüzdebirlik(&değerler, 0.25));
    let q3 = iki_basamak(yüzdebirlik(&değerler, 0.75));
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
    let en_az = iç.first().copied();
    let en_çok = iç.last().copied();
    Ok(Özet {
        framework,
        medyan,
        q1,
        q3,
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
        let kartlar = box_whisker_kartları()?;
        assert_eq!(kartlar.len(), BOX_WHISKER_BENCHMARKLERİ.len());
        for ((benchmark, seçenekler, veri), beklenen) in
            kartlar.into_iter().zip(BOX_WHISKER_BENCHMARKLERİ)
        {
            assert_eq!(benchmark, beklenen);
            assert!(seçenekler.başlık.is_empty());
            assert!(!veri.seriler().is_empty());
            assert_eq!(veri.uzunluk(), 30);
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            assert!(sahne.komutlar().iter().any(|komut| {
                matches!(komut, Komut::Dikdörtgen { dolgu, .. } if dolgu == "#eeeeee")
            }));
            assert!(sahne.komutlar().iter().any(|komut| {
                matches!(komut, Komut::DöndürülmüşMetin { açı, .. } if *açı == -90.0)
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

    #[test]
    fn stats_js_yuvarlanmış_iqr_ve_range_num_köşe_durumlarını_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = box_whisker_kartı("01_run1k")?;
        assert_eq!(
            seçenekler.kategoriler.first().map(String::as_str),
            Some("stage0-v0.0.2-keyed")
        );
        let değer = |seri: usize, indeks: usize| {
            veri.seriler()
                .get(seri)
                .and_then(|değerler| değerler.get(indeks))
                .copied()
                .flatten()
        };
        assert_eq!(
            (
                değer(0, 0),
                değer(1, 0),
                değer(2, 0),
                değer(3, 0),
                değer(4, 0)
            ),
            (
                Some(117.97),
                Some(116.32),
                Some(121.38),
                Some(115.185),
                Some(125.863)
            )
        );
        assert_eq!(
            seçenekler
                .kutu_bıyık_düzeni
                .as_ref()
                .and_then(|düzen| düzen.ayrık_değerler.first())
                .map(Vec::as_slice),
            Some([140.341].as_slice())
        );
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(
            grafik.görünür_y_aralığı(),
            crate::Aralık::yeni(105.0, 185.0)?
        );

        let (seçenekler, veri) = box_whisker_kartı("34_startup-totalbytes")?;
        assert_eq!(
            seçenekler.kategoriler.first().map(String::as_str),
            Some("attodom-v0.12.0-keyed")
        );
        let değer = |seri: usize| {
            veri.seriler()
                .get(seri)
                .and_then(|değerler| değerler.first())
                .copied()
                .flatten()
        };
        assert_eq!(
            (değer(0), değer(1), değer(2), değer(3), değer(4)),
            (Some(143.11), Some(143.11), Some(143.11), None, None)
        );
        assert_eq!(
            seçenekler
                .kutu_bıyık_düzeni
                .as_ref()
                .and_then(|düzen| düzen.ayrık_değerler.first())
                .map(Vec::as_slice),
            Some(
                [
                    143.110_351_562_5,
                    143.110_351_562_5,
                    143.110_351_562_5,
                    143.110_351_562_5,
                ]
                .as_slice()
            )
        );
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(
            grafik.görünür_y_aralığı(),
            crate::Aralık::yeni(142.0, 155.0)?
        );
        Ok(())
    }

    #[test]
    fn hover_framework_ve_beş_istatistiği_aynı_sütunda_döndürür() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = box_whisker_kartı("01_run1k")?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let (sol, _, üst, alt) = grafik.çizim_alanı_boyutta(800, 400);
        let vuruş = grafik.kutu_bıyık_vuruşu(800, 400, sol + 2.0, (üst + alt) / 2.0);
        let Some((indeks, _, genişlik, yükseklik, değerler)) = vuruş else {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "box-whisker-hover",
                açıklama: "ilk kaynak sütunu vurulamadı".to_string(),
            });
        };
        assert_eq!(indeks, 0);
        assert!(genişlik > 0.0);
        assert!((yükseklik - (alt - üst)).abs() <= f32::EPSILON);
        assert_eq!(
            grafik.kutu_bıyık_kategorisi(indeks),
            Some("stage0-v0.0.2-keyed")
        );
        assert_eq!(değerler, [117.97, 116.32, 121.38, 115.185, 125.863]);
        Ok(())
    }

    #[test]
    fn on_yedi_global_outlier_range_num_aralığı_kaynakla_eştir() -> Result<(), UplotHatası> {
        let beklenenler = [
            (105.0, 185.0),
            (108.0, 146.0),
            (123.0, 210.0),
            (11.0, 42.0),
            (42.0, 78.0),
            (40.0, 56.0),
            (990.0, 1390.0),
            (210.0, 410.0),
            (90.0, 168.0),
            (1.055, 1.115),
            (1.4, 2.9),
            (1.7, 3.3),
            (2.0, 3.6),
            (2.24, 2.51),
            (1860.0, 2060.0),
            (7.0, 108.0),
            (142.0, 155.0),
        ];
        for (benchmark, (beklenen_alt, beklenen_üst)) in
            BOX_WHISKER_BENCHMARKLERİ.into_iter().zip(beklenenler)
        {
            let (seçenekler, veri) = box_whisker_kartı(benchmark)?;
            let aralık = Grafik::yeni(seçenekler, veri)?.görünür_y_aralığı();
            assert!(
                (aralık.en_az - beklenen_alt).abs() <= 1e-12,
                "{benchmark}: {} != {beklenen_alt}",
                aralık.en_az
            );
            assert!(
                (aralık.en_çok - beklenen_üst).abs() <= 1e-12,
                "{benchmark}: {} != {beklenen_üst}",
                aralık.en_çok
            );
        }
        Ok(())
    }
}
