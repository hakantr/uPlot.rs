use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, LinePathsÖrneği, line_paths_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/line-paths"));
    std::fs::create_dir_all(&dizin)?;
    for örnek in LinePathsÖrneği::TÜMÜ {
        let (seçenekler, veri) = line_paths_kartı(örnek)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!("Line Paths kartları üretildi: {}", dizin.display());
    Ok(())
}
