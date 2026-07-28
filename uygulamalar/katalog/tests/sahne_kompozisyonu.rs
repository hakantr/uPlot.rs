//! Ağır yüzeylerin komut kompozisyonunu raporlar.
//!
//! Raster katman politikası "köşe bütçesini aşan yüzeyi bir kez rasterleştir"
//! diyor; hangi komut sınıflarını rasterleştirebilmemiz gerektiği bu dökümden
//! çıkar.

use uplot_rs::diagnostics::Komut;
use uplot_rs::{Grafik, UplotHatası};
use uplot_rs_gpui_ornekler::{LatencyHeatmapÖrneği, latency_heatmap_kartı};

fn komut_adı(komut: &Komut) -> &'static str {
    match komut {
        Komut::ArkaPlan { .. } => "ArkaPlan",
        Komut::Çizgi { .. } => "Çizgi",
        Komut::KesikliÇizgi { .. } => "KesikliÇizgi",
        Komut::Yol { .. } => "Yol",
        Komut::GradyanYol { .. } => "GradyanYol",
        Komut::KesikliYol { .. } => "KesikliYol",
        Komut::Alan { .. } => "Alan",
        Komut::GradyanAlan { .. } => "GradyanAlan",
        Komut::Daire { .. } => "Daire",
        Komut::Daireler { .. } => "Daireler",
        Komut::DeğişkenDaireler { .. } => "DeğişkenDaireler",
        Komut::Dikdörtgen { .. } => "Dikdörtgen",
        Komut::YuvarlatılmışDikdörtgen { .. } => "YuvarlatılmışDikdörtgen",
        Komut::Metin { .. } => "Metin",
        Komut::DöndürülmüşMetin { .. } => "DöndürülmüşMetin",
    }
}

/// Çokgen eksen hizalı dikdörtgen mi?
fn eksen_hizalı_dikdörtgen(çokgen: &[uplot_rs::Nokta]) -> bool {
    let [a, b, c, d] = çokgen else {
        return false;
    };
    (a.y - b.y).abs() < f32::EPSILON
        && (b.x - c.x).abs() < f32::EPSILON
        && (c.y - d.y).abs() < f32::EPSILON
        && (d.x - a.x).abs() < f32::EPSILON
}

#[test]
fn latency_heatmap_yuzey_kompozisyonu() -> Result<(), UplotHatası> {
    for örnek in LatencyHeatmapÖrneği::TÜMÜ {
        let (seçenekler, veri) = latency_heatmap_kartı(örnek, 5.0, 0.0)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let sahne = grafik.çiz();

        let mut sayım = std::collections::BTreeMap::<&str, (usize, usize)>::new();
        let mut dikdörtgen_çokgen = 0usize;
        let mut diğer_çokgen = 0usize;
        for komut in sahne.komutlar() {
            let nokta = match komut {
                Komut::Alan { çokgenler, .. } | Komut::GradyanAlan { çokgenler, .. } => {
                    for çokgen in çokgenler {
                        if eksen_hizalı_dikdörtgen(çokgen) {
                            dikdörtgen_çokgen += 1;
                        } else {
                            diğer_çokgen += 1;
                        }
                    }
                    çokgenler.iter().map(Vec::len).sum()
                }
                Komut::Yol { parçalar, .. }
                | Komut::GradyanYol { parçalar, .. }
                | Komut::KesikliYol { parçalar, .. } => parçalar.iter().map(Vec::len).sum(),
                Komut::Daireler { merkezler, .. } => merkezler.len(),
                Komut::DeğişkenDaireler { daireler, .. } => daireler.len(),
                _ => 1,
            };
            let girdi = sayım.entry(komut_adı(komut)).or_default();
            girdi.0 += 1;
            girdi.1 += nokta;
        }

        eprintln!("--- {} ---", örnek.kimlik());
        for (ad, (komut_sayısı, nokta_sayısı)) in &sayım {
            eprintln!("  {ad:22} {komut_sayısı:>5} komut · {nokta_sayısı:>8} nokta");
        }
        eprintln!("  çokgenler: {dikdörtgen_çokgen} eksen hizalı dikdörtgen, {diğer_çokgen} diğer");
        // Raster katmanı yalnız eksen hizalı dikdörtgenleri kayıpsız
        // çizebiliyor. Isı haritası hücreleri bu sınıftan çıkarsa yüzey
        // sessizce vektör yoluna düşer ve kare maliyeti geri gelir.
        assert_eq!(
            diğer_çokgen,
            0,
            "{}: ısı haritası hücreleri eksen hizalı dikdörtgen kalmalı",
            örnek.kimlik()
        );
    }
    Ok(())
}
