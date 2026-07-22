use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, area_fill_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/area-fill.svg"));
    if let Some(üst) = çıktı.parent() {
        std::fs::create_dir_all(üst)?;
    }

    let (seçenekler, veri) = area_fill_kartı()?;
    let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
    std::fs::write(&çıktı, svg)?;
    println!("Area Fill kartı üretildi: {}", çıktı.display());
    Ok(())
}
