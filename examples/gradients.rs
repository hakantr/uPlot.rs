use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{GradientÖrneği, Grafik, gradients_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/gradients"));
    std::fs::create_dir_all(&dizin)?;
    for örnek in GradientÖrneği::TÜMÜ {
        let (seçenekler, veri) = gradients_kartı(örnek)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!("Gradients kartları üretildi: {}", dizin.display());
    Ok(())
}
