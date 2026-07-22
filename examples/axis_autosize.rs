use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, axis_autosize_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/axis-autosize.svg"));
    let (seçenekler, veri) = axis_autosize_kartı(1e9)?;
    std::fs::write(&çıktı, Grafik::yeni(seçenekler, veri)?.çiz().svg())?;
    println!("Axis AutoSize kartı üretildi: {}", çıktı.display());
    Ok(())
}
