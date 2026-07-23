use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, PointsÖrneği, points_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı_dizini = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/points"));
    std::fs::create_dir_all(&çıktı_dizini)?;

    for örnek in PointsÖrneği::TÜMÜ {
        let (seçenekler, veri) = points_kartı(örnek)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(çıktı_dizini.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!("Points kartları üretildi: {}", çıktı_dizini.display());
    Ok(())
}
