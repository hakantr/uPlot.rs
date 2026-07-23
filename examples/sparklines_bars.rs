use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, SparklinesBarsÖrneği, sparklines_bars_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/sparklines-bars"));
    std::fs::create_dir_all(&çıktı)?;
    for örnek in SparklinesBarsÖrneği::TÜMÜ {
        let (seçenekler, veri) = sparklines_bars_kartı(örnek)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(çıktı.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!(
        "Sparklines + Floating Bars kartları üretildi: {}",
        çıktı.canonicalize()?.display()
    );
    Ok(())
}
