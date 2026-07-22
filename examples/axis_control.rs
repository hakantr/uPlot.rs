use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, axis_control_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/axis-control.svg"));
    let (seçenekler, veri) = axis_control_kartı()?;
    std::fs::write(&çıktı, Grafik::yeni(seçenekler, veri)?.çiz().svg())?;
    println!("Axis Control kartı üretildi: {}", çıktı.display());
    Ok(())
}
