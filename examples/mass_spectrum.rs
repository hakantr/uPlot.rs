use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, mass_spectrum_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/mass-spectrum.svg"));
    let (seçenekler, veri) = mass_spectrum_kartı()?;
    let grafik = Grafik::yeni(seçenekler, veri)?;
    std::fs::write(&çıktı, grafik.çiz().svg())?;
    println!("Mass Spectrum üretildi: {}", çıktı.display());
    Ok(())
}
