use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, resize_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/resize.svg"));
    if let Some(üst) = çıktı.parent() {
        std::fs::create_dir_all(üst)?;
    }

    let nokta_sayısı = std::env::args()
        .nth(2)
        .and_then(|değer| değer.parse().ok())
        .unwrap_or(100);
    let (seçenekler, veri) = resize_kartı(nokta_sayısı)?;
    let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
    std::fs::write(&çıktı, svg)?;
    println!("Resize kartı üretildi: {}", çıktı.display());
    Ok(())
}
