use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, trendlines_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/trendlines.svg"));
    let (seçenekler, veri) = trendlines_kartı()?;
    let grafik = Grafik::yeni(seçenekler, veri)?;
    std::fs::write(&çıktı, grafik.çiz().svg())?;
    println!("Trendlines üretildi: {}", çıktı.display());
    Ok(())
}
