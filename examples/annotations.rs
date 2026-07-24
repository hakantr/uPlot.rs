use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, annotations_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/annotations.svg"));
    let (seçenekler, veri) = annotations_kartı()?;
    let grafik = Grafik::yeni(seçenekler, veri)?;
    std::fs::write(&çıktı, grafik.çiz().svg())?;
    println!("Annotations üretildi: {}", çıktı.display());
    Ok(())
}
