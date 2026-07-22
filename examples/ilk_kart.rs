use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, ilk_kart};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/ilk-kart.svg"));
    if let Some(üst) = çıktı.parent() {
        std::fs::create_dir_all(üst)?;
    }

    let (seçenekler, veri) = ilk_kart()?;
    let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
    std::fs::write(&çıktı, svg)?;
    println!("İlk kart üretildi: {}", çıktı.display());
    Ok(())
}
