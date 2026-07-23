use std::collections::BTreeMap;

use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{
    GrafikSeçenekleri, HizalıVeri, IsıHaritasıDüzeni, IsıHücresi, IsıHücresiBoyutu,
    SeriSeçenekleri, UplotHatası,
};

pub const LATENCY_HEATMAP_KANIT_TOHUMU: u32 = 0x1A7E_4C7A;
const KANIT_ZAMANI_SANİYE: f64 = 1_642_711_320.0;
const KANIT_ZAMANI_MİLİSANİYE: f64 = 1_642_711_320_000.0;

pub const LATENCY_HEATMAP_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) =
    latency_heatmap_kartı(LatencyHeatmapÖrneği::Kovalanmış, 5.0, 0.0)?;
// Örnek üretimi, histogram kovaları ve ısı hücreleri çekirdekte çözülür.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatencyHeatmapÖrneği {
    Ham,
    Kovalanmış,
    Mode2,
    HistogramBirleşik,
    HistogramBoşluklu,
}

impl LatencyHeatmapÖrneği {
    pub const TÜMÜ: [Self; 5] = [
        Self::Ham,
        Self::Kovalanmış,
        Self::Mode2,
        Self::HistogramBirleşik,
        Self::HistogramBoşluklu,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::Ham => "latency-heatmap-raw",
            Self::Kovalanmış => "latency-heatmap-aggregated",
            Self::Mode2 => "latency-heatmap-mode2",
            Self::HistogramBirleşik => "latency-histogram-collapsed",
            Self::HistogramBoşluklu => "latency-histogram-gapped",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::Ham => "Latency Heatmap (~35k)",
            Self::Kovalanmış => "Latency Heatmap Aggregated 10ms (~20k)",
            Self::Mode2 => "Heatmap / Scatter (mode: 2) / ~45k",
            Self::HistogramBirleşik => {
                "Latency Histogram (align: 1, gap: 0, stroke: 2, border collapse)"
            }
            Self::HistogramBoşluklu => {
                "Latency Histogram (align: 1, gap: 2, stroke: 1, disp-gap-shift)"
            }
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

/// Resmî latency-heatmap.html sayfasındaki beş grafikten birini üretir.
///
/// Kova boyutu ve kova ofseti kaynak sayfadaki histogram kontrolleridir.
pub fn latency_heatmap_kartı(
    örnek: LatencyHeatmapÖrneği,
    kova_boyutu: f64,
    kova_ofseti: f64,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    if !kova_boyutu.is_finite()
        || !(1.0..=25.0).contains(&kova_boyutu)
        || !kova_ofseti.is_finite()
        || !(0.0..=25.0).contains(&kova_ofseti)
    {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/latency-heatmap.html#histogram-controls",
            açıklama: "kova boyutu 1..=25, ofset 0..=25 aralığında olmalıdır".to_string(),
        });
    }
    match örnek {
        LatencyHeatmapÖrneği::Ham => ham_ısı_haritası(),
        LatencyHeatmapÖrneği::Kovalanmış => kovalanmış_ısı_haritası(),
        LatencyHeatmapÖrneği::Mode2 => mode2_ısı_haritası(),
        LatencyHeatmapÖrneği::HistogramBirleşik | LatencyHeatmapÖrneği::HistogramBoşluklu => {
            histogram(örnek, kova_boyutu, kova_ofseti)
        }
    }
}

fn ham_ısı_haritası() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (x, sütunlar) = ham_veri();
    let hücreler = x
        .iter()
        .copied()
        .zip(sütunlar.iter())
        .flat_map(|(x, değerler)| {
            değerler.iter().copied().map(move |y| {
                IsıHücresi::yeni(
                    x,
                    y,
                    IsıHücresiBoyutu::Piksel(10.0),
                    IsıHücresiBoyutu::Piksel(1.0),
                    "#ff000066",
                )
            })
        })
        .collect();
    ısı_haritası_seçenekleri(LatencyHeatmapÖrneği::Ham, x, sütunlar, hücreler, false)
}

fn kovalanmış_ısı_haritası() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (x, sütunlar) = ham_veri();
    let mut hücreler = Vec::new();
    for (x, değerler) in x.iter().copied().zip(sütunlar.iter()) {
        let kovalar = histogram_say(değerler, 5.0, 0.0);
        let azami = kovalar.values().copied().max().unwrap_or(1).max(1);
        for (kova, sayı) in kovalar {
            let oran = f64::from(sayı) / f64::from(azami);
            hücreler.push(IsıHücresi::yeni(
                x,
                kova as f64 * 5.0,
                IsıHücresiBoyutu::Piksel(10.0),
                IsıHücresiBoyutu::Veri(5.0),
                hsl_renk(180.0 + oran * 180.0, 0.8, 0.5),
            ));
        }
    }
    ısı_haritası_seçenekleri(
        LatencyHeatmapÖrneği::Kovalanmış,
        x,
        sütunlar,
        hücreler,
        false,
    )
}

fn mode2_ısı_haritası() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    const UZUNLUK: usize = 45_000;
    const X_KOVA: f64 = 15_000.0;
    const Y_KOVA: f64 = 2.0;
    let mut rastgele = KanıtRastgele::yeni(LATENCY_HEATMAP_KANIT_TOHUMU ^ 0x02);
    let mut sayımlar = BTreeMap::<(i64, i64), u32>::new();
    for indeks in 0..UZUNLUK {
        let x = KANIT_ZAMANI_MİLİSANİYE
            - (UZUNLUK.saturating_sub(1).saturating_sub(indeks)) as f64 * 100.0;
        let y = çarpık_normal(&mut rastgele, 30.0, 50.0, 3.0);
        let x_kova = (x / X_KOVA).floor() as i64;
        let y_kova = (y / Y_KOVA).floor() as i64;
        let sayaç = sayımlar.entry((x_kova, y_kova)).or_default();
        *sayaç = sayaç.saturating_add(1);
    }
    let en_az = sayımlar
        .values()
        .copied()
        .filter(|sayı| *sayı > 0)
        .min()
        .unwrap_or(1);
    let en_çok = sayımlar.values().copied().max().unwrap_or(en_az).max(en_az);
    let palet = metal_paleti();
    let aralık = en_çok.saturating_sub(en_az);
    let mut hücreler = Vec::with_capacity(sayımlar.len());
    let mut x_sınırları = BTreeMap::<i64, (f64, f64)>::new();
    for ((x_kova, y_kova), sayı) in sayımlar {
        let palet_indeksi = if aralık == 0 {
            0
        } else {
            ((palet.len() as u64 * u64::from(sayı.saturating_sub(en_az))) / u64::from(aralık))
                .min(palet.len().saturating_sub(1) as u64) as usize
        };
        let x = x_kova as f64 * X_KOVA;
        let y = y_kova as f64 * Y_KOVA;
        let sınırlar = x_sınırları.entry(x_kova).or_insert((y, y));
        sınırlar.0 = sınırlar.0.min(y);
        sınırlar.1 = sınırlar.1.max(y);
        let renk = palet.get(palet_indeksi).copied().unwrap_or("#ffffff");
        hücreler.push(IsıHücresi::yeni(
            x,
            y,
            IsıHücresiBoyutu::Veri(X_KOVA),
            IsıHücresiBoyutu::Veri(Y_KOVA),
            renk,
        ));
    }
    let x = x_sınırları
        .keys()
        .map(|kova| *kova as f64 * X_KOVA)
        .collect::<Vec<_>>();
    let alt = x_sınırları
        .values()
        .map(|(en_az, _)| Some(*en_az))
        .collect();
    let üst = x_sınırları
        .values()
        .map(|(_, en_çok)| Some(*en_çok))
        .collect();
    let veri = HizalıVeri::yeni(x, vec![alt, üst])?;
    let seçenekler = temel_ısı_seçenekleri(LatencyHeatmapÖrneği::Mode2, true)?
        .ısı_haritası_düzeni(IsıHaritasıDüzeni::yeni(hücreler));
    Ok((seçenekler, veri))
}

fn histogram(
    örnek: LatencyHeatmapÖrneği,
    kova_boyutu: f64,
    kova_ofseti: f64,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (_, sütunlar) = ham_veri();
    let tümü = sütunlar.iter().flatten().copied().collect::<Vec<_>>();
    let sayımlar = histogram_say(&tümü, kova_boyutu, kova_ofseti);
    let mut x = sayımlar
        .keys()
        .map(|kova| *kova as f64 * kova_boyutu + kova_ofseti)
        .collect::<Vec<_>>();
    let mut y = sayımlar
        .values()
        .map(|sayı| Some(f64::from(*sayı)))
        .collect::<Vec<_>>();
    if let Some(son) = x.last().copied() {
        x.push(son + kova_boyutu);
        y.push(None);
    }
    let veri = HizalıVeri::yeni(x, vec![y])?;
    let boşluklu = örnek == LatencyHeatmapÖrneği::HistogramBoşluklu;
    let seri = SeriSeçenekleri::yeni("Latency")
        .renk("#ff0000")
        .dolgu("#ff000066")
        .çizgi_kalınlığı(if boşluklu { 1.0 } else { 2.0 })
        .çubuk(true)
        .çubuk_boyutu(if boşluklu { 0.94 } else { 1.0 }, f32::MAX);
    let seçenekler = GrafikSeçenekleri::yeni(1_800, 600)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .ızgara_rengi(if boşluklu { "#00000000" } else { "#e5e7eb" })
        .seri(seri)
        .etkileşimler(ortak_kart_etkileşimleri());
    Ok((seçenekler, veri))
}

fn ısı_haritası_seçenekleri(
    örnek: LatencyHeatmapÖrneği,
    x: Vec<f64>,
    sütunlar: Vec<Vec<f64>>,
    hücreler: Vec<IsıHücresi>,
    milisaniye: bool,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let alt = sütunlar
        .iter()
        .map(|değerler| değerler.first().copied())
        .collect();
    let üst = sütunlar
        .iter()
        .map(|değerler| değerler.last().copied())
        .collect();
    let veri = HizalıVeri::yeni(x, vec![alt, üst])?;
    let seçenekler = temel_ısı_seçenekleri(örnek, milisaniye)?
        .ısı_haritası_düzeni(IsıHaritasıDüzeni::yeni(hücreler));
    Ok((seçenekler, veri))
}

fn temel_ısı_seçenekleri(
    örnek: LatencyHeatmapÖrneği,
    milisaniye: bool,
) -> Result<GrafikSeçenekleri, UplotHatası> {
    Ok(GrafikSeçenekleri::yeni(1_800, 600)?
        .başlık(örnek.başlık())
        .x_zaman(true)
        .x_zaman_milisaniye(milisaniye)
        .seri(
            SeriSeçenekleri::yeni("Minimum")
                .göster(false)
                .çizgi_kalınlığı(0.0),
        )
        .seri(
            SeriSeçenekleri::yeni("Maximum")
                .göster(false)
                .çizgi_kalınlığı(0.0),
        )
        .etkileşimler(ortak_kart_etkileşimleri()))
}

fn ham_veri() -> (Vec<f64>, Vec<Vec<f64>>) {
    let mut rastgele = KanıtRastgele::yeni(LATENCY_HEATMAP_KANIT_TOHUMU);
    let mut x = Vec::with_capacity(100);
    let mut sütunlar = Vec::with_capacity(100);
    for indeks in 0..100 {
        x.push(KANIT_ZAMANI_SANİYE + indeks as f64);
        let adet = (rastgele.sonraki() * 301.0).floor() as usize + 200;
        let mut değerler = (0..adet)
            .map(|_| çarpık_normal(&mut rastgele, 30.0, 30.0, 3.0).max(5.0))
            .collect::<Vec<_>>();
        değerler.sort_by(f64::total_cmp);
        sütunlar.push(değerler);
    }
    (x, sütunlar)
}

fn histogram_say(değerler: &[f64], boyut: f64, ofset: f64) -> BTreeMap<i64, u32> {
    let mut sonuç = BTreeMap::new();
    for değer in değerler {
        let kova = ((*değer - ofset) / boyut).floor() as i64;
        let sayaç = sonuç.entry(kova).or_insert(0_u32);
        *sayaç = sayaç.saturating_add(1);
    }
    sonuç
}

fn çarpık_normal(rastgele: &mut KanıtRastgele, konum: f64, ölçek: f64, alfa: f64) -> f64 {
    let (u0, v) = normal_çifti(rastgele);
    let delta = alfa / (1.0 + alfa * alfa).sqrt();
    let u1 = delta * u0 + (1.0 - delta * delta).sqrt() * v;
    let z = if u0 >= 0.0 { u1 } else { -u1 };
    konum + ölçek * z
}

fn normal_çifti(rastgele: &mut KanıtRastgele) -> (f64, f64) {
    let mut u1 = rastgele.sonraki();
    let mut u2 = rastgele.sonraki();
    if u1 <= f64::EPSILON {
        u1 = f64::EPSILON;
    }
    if u2 <= f64::EPSILON {
        u2 = f64::EPSILON;
    }
    let yarıçap = (-2.0 * u1.ln()).sqrt();
    let açı = 2.0 * std::f64::consts::PI * u2;
    (yarıçap * açı.cos(), yarıçap * açı.sin())
}

fn hsl_renk(açı: f64, doygunluk: f64, açıklık: f64) -> String {
    let c = (1.0 - (2.0 * açıklık - 1.0).abs()) * doygunluk;
    let h = (açı.rem_euclid(360.0)) / 60.0;
    let x = c * (1.0 - (h.rem_euclid(2.0) - 1.0).abs());
    let (r, g, b) = match h as u8 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = açıklık - c / 2.0;
    format!(
        "#{:02x}{:02x}{:02x}",
        ((r + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((g + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((b + m) * 255.0).round().clamp(0.0, 255.0) as u8,
    )
}

fn metal_paleti() -> [&'static str; 15] {
    [
        "#fff0de", "#ffe0bc", "#ffd199", "#fec173", "#fcb045", "#ff9a3d", "#ff8335", "#ff6a2d",
        "#ff4c25", "#fd1d1d", "#e43550", "#ca3f6f", "#b24388", "#9a419f", "#833ab4",
    ]
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn beş_kaynak_grafiği_aynı_üreteç_ve_kovaları_korur() -> Result<(), UplotHatası> {
        for örnek in LatencyHeatmapÖrneği::TÜMÜ {
            let (seçenekler, veri) = latency_heatmap_kartı(örnek, 5.0, 0.0)?;
            assert!(!veri.x().is_empty());
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            assert!(
                sahne.komutlar().iter().any(|komut| {
                    matches!(komut, Komut::Alan { .. } | Komut::Dikdörtgen { .. })
                })
            );
        }
        Ok(())
    }

    #[test]
    fn ham_kaynak_otuz_beş_bin_civarında_örnek_üretir() {
        let (_, sütunlar) = ham_veri();
        let adet = sütunlar.iter().map(Vec::len).sum::<usize>();
        assert_eq!(sütunlar.len(), 100);
        assert!((20_000..=50_000).contains(&adet));
        assert!(sütunlar.iter().all(|sütun| {
            sütun.windows(2).all(|çift| {
                çift
                    .first()
                    .zip(çift.get(1))
                    .is_some_and(|(sol, sağ)| sol <= sağ)
            })
        }));
    }

    #[test]
    fn histogram_kontrolleri_veriyi_yeniden_kovalar() -> Result<(), UplotHatası> {
        let (_, beşlik) =
            latency_heatmap_kartı(LatencyHeatmapÖrneği::HistogramBirleşik, 5.0, 0.0)?;
        let (_, onluk) =
            latency_heatmap_kartı(LatencyHeatmapÖrneği::HistogramBirleşik, 10.0, 3.0)?;
        assert!(onluk.uzunluk() < beşlik.uzunluk());
        Ok(())
    }
}
