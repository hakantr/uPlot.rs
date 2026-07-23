use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, tooltips_closest_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/tooltips-closest.svg"));
    let (seçenekler, veri) = tooltips_closest_kartı()?;
    let grafik = Grafik::yeni(seçenekler, veri)?;
    std::fs::write(&çıktı, grafik.çiz().svg())?;
    println!("Summary-opt üretildi: {}", çıktı.display());
    Ok(())
}
