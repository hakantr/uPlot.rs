use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, axis_indicators_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/axis-indicators.svg"));
    let (seçenekler, veri) = axis_indicators_kartı()?;
    std::fs::write(&çıktı, Grafik::yeni(seçenekler, veri)?.çiz().svg())?;
    println!("Axis indicators kartı üretildi: {}", çıktı.display());
    Ok(())
}
