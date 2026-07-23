use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, tooltips_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/tooltips.svg"));
    let (seçenekler, veri) = tooltips_kartı()?;
    let grafik = Grafik::yeni(seçenekler, veri)?;
    std::fs::write(&çıktı, grafik.çiz().svg())?;
    println!("Tooltips üretildi: {}", çıktı.display());
    Ok(())
}
