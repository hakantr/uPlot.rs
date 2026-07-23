use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, StackedSeriesÖrneği, stacked_series_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/stacked-series"));
    std::fs::create_dir_all(&çıktı)?;
    for örnek in StackedSeriesÖrneği::TÜMÜ {
        let (seçenekler, veri) = stacked_series_kartı(örnek)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(çıktı.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!(
        "Stacked Series kartları üretildi: {}",
        çıktı.canonicalize()?.display()
    );
    Ok(())
}
