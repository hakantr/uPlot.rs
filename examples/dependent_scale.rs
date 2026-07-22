use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, dependent_scale_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/dependent-scale.svg"));
    let (seçenekler, veri) = dependent_scale_kartı()?;
    std::fs::write(&çıktı, Grafik::yeni(seçenekler, veri)?.çiz().svg())?;
    println!("Derived Scale kartı üretildi: {}", çıktı.display());
    Ok(())
}
