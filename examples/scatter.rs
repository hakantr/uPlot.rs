use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, ScatterÖrneği, scatter_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/scatter"));
    std::fs::create_dir_all(&çıktı)?;
    for örnek in ScatterÖrneği::TÜMÜ {
        let (seçenekler, veri) = scatter_kartı(örnek)?;
        std::fs::write(
            çıktı.join(format!("{}.svg", örnek.kimlik())),
            Grafik::yeni(seçenekler, veri)?.çiz().svg(),
        )?;
    }
    println!("Scatter & Bubble kartları üretildi: {}", çıktı.display());
    Ok(())
}
