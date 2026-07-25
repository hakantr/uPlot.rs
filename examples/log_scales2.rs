use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, log_scales2_kartları};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/log-scales2"));
    std::fs::create_dir_all(&dizin)?;
    for (örnek, seçenekler, veri) in log_scales2_kartları()? {
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!("Log Scales 2 kartları üretildi: {}", dizin.display());
    Ok(())
}
