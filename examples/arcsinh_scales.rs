use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, arcsinh_scales_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/arcsinh-scales.svg"));
    let (seçenekler, veri) = arcsinh_scales_kartı()?;
    std::fs::write(&çıktı, Grafik::yeni(seçenekler, veri)?.çiz().svg())?;
    println!("ArcSinh kartı üretildi: {}", çıktı.display());
    Ok(())
}
