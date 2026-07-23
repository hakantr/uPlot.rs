use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, nice_scale_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/nice-scale.svg"));
    if let Some(dizin) = çıktı.parent() {
        std::fs::create_dir_all(dizin)?;
    }
    let (seçenekler, veri) = nice_scale_kartı()?;
    let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
    std::fs::write(&çıktı, svg)?;
    println!("Nice Scale kartı üretildi: {}", çıktı.display());
    Ok(())
}
