use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, wind_direction_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/wind-direction.svg"));
    let (seçenekler, veri) = wind_direction_kartı()?;
    let grafik = Grafik::yeni(seçenekler, veri)?;
    std::fs::write(&çıktı, grafik.çiz().svg())?;
    println!("Wind Direction üretildi: {}", çıktı.display());
    Ok(())
}
