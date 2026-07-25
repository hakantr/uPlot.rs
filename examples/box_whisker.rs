use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, box_whisker_kartları};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/box-whisker"));
    std::fs::create_dir_all(&dizin)?;
    for (benchmark, seçenekler, veri) in box_whisker_kartları()? {
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(format!("{benchmark}.svg")), svg)?;
    }
    println!("17 kutu-bıyık grafiği üretildi: {}", dizin.display());
    Ok(())
}
