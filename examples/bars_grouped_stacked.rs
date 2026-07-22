use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, bars_grouped_stacked_kartı, ÇubukÖrneği};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/bars-grouped-stacked"));
    std::fs::create_dir_all(&dizin)?;
    for örnek in ÇubukÖrneği::TÜMÜ {
        let (seçenekler, veri) = bars_grouped_stacked_kartı(örnek)?;
        let yol = dizin.join(format!("{}.svg", örnek.kimlik()));
        std::fs::write(yol, Grafik::yeni(seçenekler, veri)?.çiz().svg())?;
    }
    println!("On çubuk alt grafiği üretildi: {}", dizin.display());
    Ok(())
}
