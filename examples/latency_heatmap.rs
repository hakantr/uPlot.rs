use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, LatencyHeatmapÖrneği, latency_heatmap_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/latency-heatmap"));
    std::fs::create_dir_all(&dizin)?;
    for örnek in LatencyHeatmapÖrneği::TÜMÜ {
        let (seçenekler, veri) = latency_heatmap_kartı(örnek, 5.0, 0.0)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!("Latency Heatmap kartları üretildi: {}", dizin.display());
    Ok(())
}
