use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, HighLowBandsÖrneği, high_low_bands_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/high-low-bands"));
    std::fs::create_dir_all(&dizin)?;
    for örnek in HighLowBandsÖrneği::TÜMÜ {
        let (seçenekler, veri) = high_low_bands_kartı(örnek)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!("High/Low Bands kartları üretildi: {}", dizin.display());
    Ok(())
}
