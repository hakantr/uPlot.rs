use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, YShiftedSeriesAkışı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/y-shifted-series.svg"));
    let akış = YShiftedSeriesAkışı::yeni()?;
    let (seçenekler, veri) = akış.kartı()?;
    let grafik = Grafik::yeni(seçenekler, veri)?;
    std::fs::write(&çıktı, grafik.çiz().svg())?;
    println!("Y-shifted Series üretildi: {}", çıktı.display());
    Ok(())
}
