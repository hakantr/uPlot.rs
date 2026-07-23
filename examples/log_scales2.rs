use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, LogScales2Örneği, log_scales2_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/log-scales2"));
    std::fs::create_dir_all(&dizin)?;
    for örnek in LogScales2Örneği::TÜMÜ {
        let (seçenekler, veri) = log_scales2_kartı(örnek)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!("Log Scales 2 kartları üretildi: {}", dizin.display());
    Ok(())
}
