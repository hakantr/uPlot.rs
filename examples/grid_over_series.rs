use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, grid_over_series_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/grid-over-series"));
    std::fs::create_dir_all(&dizin)?;
    let (seçenekler, veri) = grid_over_series_kartı()?;
    let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
    std::fs::write(dizin.join("grid-over-series.svg"), svg)?;
    println!("Grid Over Series kartı üretildi: {}", dizin.display());
    Ok(())
}
