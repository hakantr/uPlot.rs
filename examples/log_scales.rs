use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, LogScalesÖrneği, log_scales_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/log-scales"));
    std::fs::create_dir_all(&dizin)?;
    for örnek in LogScalesÖrneği::TÜMÜ {
        let (seçenekler, veri) = log_scales_kartı(örnek)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!("Log Scales kartları üretildi: {}", dizin.display());
    Ok(())
}
